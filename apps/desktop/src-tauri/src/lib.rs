//! Layress 桌面工作台核心编排（add-tauri-page-host 任务 2.x）。
//!
//! 单窗口双 WebView：工具层（label "main"，本地可信页面，持有能力）铺满窗口作底层；
//! 被测页面（label "page"）为上层覆盖视图，固定在工具条之下，无任何 Tauri 能力。
//! 权限边界、数据根与会话口径见 change design「实施定稿」。

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod fixture;

use serde::{Deserialize, Serialize};
use std::net::TcpStream;
use std::sync::{Mutex, OnceLock};
use std::time::Duration;
use tauri::webview::{
    DownloadEvent, NewWindowResponse, PageLoadEvent, PermissionResponse,
};
use tauri::{
    AppHandle, Emitter, LogicalPosition, LogicalSize, Manager, State, Url, WebviewUrl,
    Window, WindowEvent,
};

const TOOL_LABEL: &str = "main";
const PAGE_LABEL: &str = "page";
/// 工具条高度（逻辑像素），前端 #bar 的 CSS 高度必须与此一致；
/// 布局为互不重叠的两块：工具条 [0, TOOLBAR_H]，被测页面 [TOOLBAR_H, 窗口底]。
const TOOLBAR_H: f64 = 96.0;
/// 视口上报脚本写入 document.title 的哨兵前缀（零宽空格 + 标记）。
/// 被测页面无 IPC 能力，借 title 变化回传真实视口（wry macOS 支持 title 观测）。
const TITLE_PREFIX: &str = "\u{200b}LW ";
const REACHABILITY_TIMEOUT: Duration = Duration::from_secs(3);

const MEASURE_SCRIPT: &str = r#"(function(){
  // 视口边框样式（工具辅助标记，pointer-events:none 不拦截页面交互）；
  // 仅当 Rust 侧判定"设定视口小于舞台"时加 html.__layress_frame_on 类才显示
  var st = document.createElement("style");
  st.id = "__layress_frame_style";
  st.textContent = 'html.__layress_frame_on::after{content:"";position:fixed;inset:0;pointer-events:none;box-sizing:border-box;border:1px solid rgba(96,112,128,.9);z-index:2147483647}';
  (document.documentElement || document).appendChild(st);
  function __layressReport(){
    try {
      document.title = "\u200bLW " + JSON.stringify({
        w: window.innerWidth, h: window.innerHeight, dpr: window.devicePixelRatio
      });
    } catch (e) {}
  }
  __layressReport();
  window.addEventListener("resize", __layressReport);
})();"#;

// ── 状态 ─────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct MeasuredViewport {
    w: f64,
    h: f64,
    dpr: f64,
}

#[derive(Default)]
struct PageHost {
    /// 用户显式设定的页面视口（逻辑 px）；None = 跟随窗口自适应
    configured: Mutex<Option<(f64, f64)>>,
    /// 样例页面上报的真实视口
    measured: Mutex<Option<MeasuredViewport>>,
    /// 本地样例服务基址
    sample_base: OnceLock<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "kind", content = "data")]
enum PageEvent {
    /// 当前页面地址变化
    Url(String),
    /// 加载状态：loading / loaded
    Status { phase: String, url: String },
    /// 视口：设定值、实际边界、页面上报测量值、窗口能否完整容纳
    Viewport {
        configured: Option<(f64, f64)>,
        bounds: (f64, f64),
        measured: Option<MeasuredViewport>,
        fully_visible: bool,
    },
    /// 被测页面的额外入口请求（新窗口 / 下载 / 权限），均为拒绝动作
    Request {
        kind: String,
        url: String,
        action: String,
    },
    /// 工具明确拒绝的操作（无效地址等），供状态条显示
    Denied { reason: String },
}

/// 当前应用句柄。回调闭包拿不到 State/Manager 之外的应用引用，统一经此取。
static APP: OnceLock<AppHandle<tauri::Wry>> = OnceLock::new();

fn emit(app: &AppHandle<tauri::Wry>, event: PageEvent) {
    let _ = app.emit_to(TOOL_LABEL, "page://event", event);
}

/// 计算并上报视口状态：页面实际边界 vs 用户设定 vs 窗口容纳情况。
fn emit_viewport(app: &AppHandle<tauri::Wry>) {
    let state = app.state::<PageHost>();
    let configured = state.configured.lock().ok().and_then(|c| *c);
    let measured = state.measured.lock().ok().and_then(|m| m.clone());
    let Some(window) = app.get_window("main") else {
        return;
    };
    let (win_w, win_h) = match window.inner_size() {
        Ok(inner) => {
            let scale = window.scale_factor().unwrap_or(1.0);
            (inner.width as f64 / scale, inner.height as f64 / scale)
        }
        Err(_) => return,
    };
    let bounds = configured.unwrap_or((win_w, win_h - TOOLBAR_H));
    let fully_visible = win_w + 0.5 >= bounds.0 && win_h - TOOLBAR_H + 0.5 >= bounds.1;
    emit(
        app,
        PageEvent::Viewport {
            configured,
            bounds,
            measured,
            fully_visible,
        },
    );
}

/// 依据窗口尺寸重排两块 WebView：工具条固定在顶部，页面占其余区域。
/// 显式边界、互不重叠，避免任何坐标解释差异导致的层间穿透。
fn relayout(window: &Window<tauri::Wry>, app: &AppHandle<tauri::Wry>) {
    let scale = window.scale_factor().unwrap_or(1.0);
    let Ok(inner) = window.inner_size() else {
        return;
    };
    let win_w = inner.width as f64 / scale;
    let win_h = inner.height as f64 / scale;

    if let Some(tool) = app.get_webview(TOOL_LABEL) {
        let _ = tool.set_bounds(tauri::Rect {
            position: LogicalPosition::new(0.0, 0.0).into(),
            size: LogicalSize::new(win_w, TOOLBAR_H).into(),
        });
    }
    let configured = app
        .state::<PageHost>()
        .configured
        .lock()
        .ok()
        .and_then(|c| *c);
    if let Some(page) = app.get_webview(PAGE_LABEL) {
        let size = configured.unwrap_or((win_w, win_h - TOOLBAR_H));
        let _ = page.set_bounds(tauri::Rect {
            position: LogicalPosition::new(0.0, TOOLBAR_H).into(),
            size: LogicalSize::new(size.0, size.1).into(),
        });
    }
}

/// 页面视口边框：设定视口且小于舞台时开启，让页面边界与白色背景可区分。
/// 边框是注入 DOM 的固定定位描边（pointer-events:none），属工具辅助标记，
/// 任务 3.x 采集真实画面时必须先关闭再截图，不得混入页面现场。
fn sync_page_frame(app: &AppHandle<tauri::Wry>) {
    let state = app.state::<PageHost>();
    let configured = state.configured.lock().ok().and_then(|c| *c);
    let on = match (configured, app.get_window("main")) {
        (Some((w, h)), Some(window)) => {
            window
                .inner_size()
                .ok()
                .map(|inner| {
                    let scale = window.scale_factor().unwrap_or(1.0);
                    let stage_w = inner.width as f64 / scale;
                    let stage_h = inner.height as f64 / scale - TOOLBAR_H;
                    w + 0.5 < stage_w || h + 0.5 < stage_h
                })
                .unwrap_or(false)
        }
        _ => false, // 跟随窗口模式：页面铺满舞台，无需边框
    };
    if let Some(page) = app.get_webview(PAGE_LABEL) {
        let js = if on {
            "document.documentElement.classList.add('__layress_frame_on');"
        } else {
            "document.documentElement.classList.remove('__layress_frame_on');"
        };
        let _ = page.eval(js.to_string());
    }
}

// ── 被测页面 WebView 生命周期 ─────────────────────────────────────

/// 创建或复用被测页面 WebView 并导航到目标地址。
/// 已存在时原地导航（保留会话与页面状态），不销毁重建。
fn ensure_and_navigate(app: &AppHandle<tauri::Wry>, url: Url) -> Result<(), String> {
    let window = app
        .get_window("main")
        .ok_or_else(|| "主窗口未就绪".to_string())?;
    let state = app.state::<PageHost>();
    let configured = state.configured.lock().ok().and_then(|c| *c);

    let (w, h) = {
        let inner = window
            .inner_size()
            .map_err(|e| format!("读取窗口尺寸失败：{e}"))?;
        let scale = window.scale_factor().unwrap_or(1.0);
        let win_w = inner.width as f64 / scale;
        let win_h = inner.height as f64 / scale;
        configured.unwrap_or((win_w, win_h - TOOLBAR_H))
    };

    if let Some(page) = app.get_webview(PAGE_LABEL) {
        page.navigate(url).map_err(|e| format!("导航失败：{e}"))?;
        return Ok(());
    }

    let builder = tauri::webview::WebviewBuilder::new(PAGE_LABEL, WebviewUrl::External(url))
        .incognito(true)
        .initialization_script(MEASURE_SCRIPT)
        .on_navigation(|nav_url| {
            // 导航边界：只放行 HTTP(S)；其余 scheme 明确拒绝（E03）
            let ok = matches!(nav_url.scheme(), "http" | "https");
            if !ok {
                if let Some(app) = APP.get() {
                    emit(
                        app,
                        PageEvent::Denied {
                            reason: format!("已拒绝非 HTTP(S) 导航：{nav_url}"),
                        },
                    );
                }
            }
            ok
        })
        .on_page_load(|webview, payload| {
            let app = webview.app_handle().clone();
            let phase = match payload.event() {
                PageLoadEvent::Started => "loading",
                PageLoadEvent::Finished => "loaded",
            };
            emit(
                &app,
                PageEvent::Status {
                    phase: phase.to_string(),
                    url: payload.url().to_string(),
                },
            );
            if phase == "loaded" {
                // 导航会重置 DOM，加载完成后按当前状态重新应用视口边框
                sync_page_frame(&app);
                emit_viewport(&app);
            }
        })
        .on_document_title_changed(|webview, title| {
            let Some(rest) = title.strip_prefix(TITLE_PREFIX) else {
                return; // 非哨兵标题（页面自身标题），忽略
            };
            if let Ok(m) = serde_json::from_str::<MeasuredViewport>(rest) {
                let app = webview.app_handle().clone();
                if let Ok(mut slot) = app.state::<PageHost>().measured.lock() {
                    *slot = Some(m);
                }
                emit_viewport(&app);
            }
        })
        .on_new_window(|new_url, _features| {
            // 新窗口一律拒绝：不创建额外顶层窗口（E13）
            if let Some(app) = APP.get() {
                emit(
                    app,
                    PageEvent::Request {
                        kind: "new-window".to_string(),
                        url: new_url.to_string(),
                        action: "denied".to_string(),
                    },
                );
            }
            NewWindowResponse::Deny
        })
        .on_download(|_webview, event| {
            // 下载一律拒绝：不写默认下载目录（E13）
            if let DownloadEvent::Requested { url, .. } = event {
                if let Some(app) = APP.get() {
                    emit(
                        app,
                        PageEvent::Request {
                            kind: "download".to_string(),
                            url: url.to_string(),
                            action: "denied".to_string(),
                        },
                    );
                }
            }
            false
        })
        .on_permission_request(|_webview, kind| {
            // 设备权限一律拒绝（E13）
            if let Some(app) = APP.get() {
                emit(
                    app,
                    PageEvent::Request {
                        kind: format!("{kind:?}"),
                        url: String::new(),
                        action: "denied".to_string(),
                    },
                );
            }
            PermissionResponse::Deny
        });

    let _page = window
        .add_child(
            builder,
            LogicalPosition::new(0.0, TOOLBAR_H),
            LogicalSize::new(w, h),
        )
        .map_err(|e| format!("创建被测页面视图失败：{e}"))?;
    emit_viewport(app);
    Ok(())
}

// ── 地址校验与可达性 ─────────────────────────────────────────────

fn validate_url(raw: &str) -> Result<Url, String> {
    let raw = raw.trim();
    if raw.is_empty() {
        return Err("地址为空".to_string());
    }
    let url = Url::parse(raw).map_err(|e| format!("地址无法解析：{e}"))?;
    match url.scheme() {
        "http" | "https" => {}
        other => return Err(format!("不支持的 scheme「{other}」，仅允许 http/https")),
    }
    if url.host_str().map(str::is_empty).unwrap_or(true) {
        return Err("地址缺少主机名".to_string());
    }
    Ok(url)
}

/// 导航前做 TCP 可达性预检：失败明确报错且不触碰当前页面（E03）。
async fn check_reachable(url: &Url) -> Result<(), String> {
    let host = url.host_str().unwrap_or_default().to_string();
    let port = url
        .port_or_known_default()
        .ok_or_else(|| "地址缺少端口且 scheme 无默认端口".to_string())?;
    let target = format!("{host}:{port}");
    let res = tauri::async_runtime::spawn_blocking(move || {
        use std::net::ToSocketAddrs;
        match target.to_socket_addrs() {
            Err(e) => Err(format!("域名解析失败：{e}")),
            Ok(mut addrs) => match addrs.find(|a| a.is_ipv4()).or_else(|| addrs.next()) {
                None => Err("域名未解析出可用地址".to_string()),
                Some(addr) => match TcpStream::connect_timeout(&addr, REACHABILITY_TIMEOUT) {
                    Ok(_) => Ok(()),
                    Err(e) => Err(format!("目标不可达（{addr}）：{e}")),
                },
            },
        }
    })
    .await
    .map_err(|e| format!("可达性检查任务失败：{e}"))?;
    res
}

// ── 命令（工具层 → Rust） ────────────────────────────────────────

#[tauri::command]
fn sample_url(state: State<'_, PageHost>) -> Result<String, String> {
    state
        .sample_base
        .get()
        .cloned()
        .ok_or_else(|| "样例服务未就绪".to_string())
}

#[tauri::command]
async fn page_navigate(app: AppHandle<tauri::Wry>, raw: String) -> Result<(), String> {
    let url = validate_url(&raw)?;
    check_reachable(&url).await?;
    ensure_and_navigate(&app, url)?;
    emit(&app, PageEvent::Url(raw.trim().to_string()));
    Ok(())
}

#[tauri::command]
fn page_refresh(app: AppHandle<tauri::Wry>) -> Result<(), String> {
    let page = app
        .get_webview(PAGE_LABEL)
        .ok_or_else(|| "尚无被测页面".to_string())?;
    let current = page.url().map_err(|e| format!("读取当前地址失败：{e}"))?;
    page.navigate(current).map_err(|e| format!("刷新失败：{e}"))
}

#[tauri::command]
fn page_set_size(app: AppHandle<tauri::Wry>, width: f64, height: f64) -> Result<(), String> {
    if !(16.0..=4096.0).contains(&width) || !(16.0..=4096.0).contains(&height) {
        return Err("尺寸超出允许范围（16–4096）".to_string());
    }
    {
        let state = app.state::<PageHost>();
        let mut cfg = state
            .configured
            .lock()
            .map_err(|_| "视口状态锁不可用".to_string())?;
        *cfg = Some((width, height));
    }
    if let Some(page) = app.get_webview(PAGE_LABEL) {
        page.set_bounds(tauri::Rect {
            position: LogicalPosition::new(0.0, TOOLBAR_H).into(),
            size: LogicalSize::new(width, height).into(),
        })
        .map_err(|e| format!("调整页面视口失败：{e}"))?;
    }
    sync_page_frame(&app);
    emit_viewport(&app);
    Ok(())
}

/// 工具层初始化时取一次全量状态。
#[tauri::command]
fn page_state(app: AppHandle<tauri::Wry>) -> Result<serde_json::Value, String> {
    let state = app.state::<PageHost>();
    let configured = state.configured.lock().ok().and_then(|c| *c);
    let measured = state.measured.lock().ok().and_then(|m| m.clone());
    let url = app
        .get_webview(PAGE_LABEL)
        .and_then(|p| p.url().ok())
        .map(|u| u.to_string());
    Ok(serde_json::json!({
        "configured": configured,
        "measured": measured,
        "url": url,
        "sampleBase": state.sample_base.get().cloned(),
        "toolbarH": TOOLBAR_H,
        "pageOpen": url.is_some(),
    }))
}

// ── 启动 ─────────────────────────────────────────────────────────

pub fn run() {
    tauri::Builder::default()
        .manage(PageHost::default())
        .invoke_handler(tauri::generate_handler![
            sample_url,
            page_navigate,
            page_refresh,
            page_set_size,
            page_state
        ])
        .setup(|app| {
            // 样例服务：失败则明确记录，工具层显示不可用，不静默降级
            match fixture::start() {
                Ok(base) => {
                    let _ = app.state::<PageHost>().sample_base.set(base);
                }
                Err(e) => {
                    eprintln!("[layress] 样例服务启动失败：{e}");
                }
            }

            // 主窗口：配置不定义窗口，双 WebView 都在此创建（unstable 多 WebView）
            let window = tauri::window::WindowBuilder::new(app, "main")
                .title("Layress")
                .inner_size(1280.0, 800.0)
                .min_inner_size(520.0, 400.0)
                .build()?;

            // 工具层：仅顶部条带（与页面互不重叠）；非持久会话
            let inner = window.inner_size()?;
            let scale = window.scale_factor().unwrap_or(1.0);
            let _tool = window.add_child(
                tauri::webview::WebviewBuilder::new(
                    TOOL_LABEL,
                    WebviewUrl::App("index.html".into()),
                )
                .incognito(true),
                LogicalPosition::new(0.0, 0.0),
                LogicalSize::new(inner.width as f64 / scale, TOOLBAR_H),
            )?;

            let _ = APP.set(app.handle().clone());

            let handle = app.handle().clone();
            let win = window.clone();
            window.on_window_event(move |event| {
                if let WindowEvent::Resized(_) = event {
                    // 显式重排：工具条固定顶部，页面按设定或跟随窗口（E04 不静默改视口）
                    relayout(&win, &handle);
                    sync_page_frame(&handle);
                    emit_viewport(&handle);
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("Layress 桌面应用启动失败");
}
