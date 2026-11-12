//! Layress 桌面工作台核心编排（add-tauri-page-host 任务 2.x–3.x）。
//!
//! 单窗口三块显式布局：可信工具层（"main"，持有能力）在顶部条带，
//! 被测页面（"page"）固定在条带之下且无任何 Tauri 能力。
//! 预览态把工具层铺满窗口、把被测层**只平移出可视区且尺寸不变**：
//! 因此返回操作页面时不销毁、不重载、不改视口（E07 / E08）。
//! 真实截图只取被测 WKWebView 自身（E05），采集身份与失效规则见 `capture.rs`。

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod capture;
mod fixture;

use capture::{
    base64_encode, ensure_writable, persist, png_size, resolve_data_root, snapshot_png, unix_ms,
    CaptureConditions, CaptureImage, CaptureMeta, CapturePage, CaptureState, CapturedViewport,
    DataRoot, MeasuredViewport, SCHEMA,
};
use serde::Serialize;
use std::net::TcpStream;
use std::sync::{Mutex, OnceLock};
use std::time::Duration;
use tauri::webview::{DownloadEvent, NewWindowResponse, PageLoadEvent, PermissionResponse};
use tauri::{
    AppHandle, Emitter, LogicalPosition, LogicalSize, Manager, State, Url, WebviewUrl, WindowEvent,
};

const TOOL_LABEL: &str = "main";
const PAGE_LABEL: &str = "page";
/// 工具条高度（逻辑像素），前端 #bar 的 CSS 高度必须与此一致；
/// 布局为互不重叠的两块：工具条 [0, TOOLBAR_H]，被测页面 [TOOLBAR_H, 窗口底]。
const TOOLBAR_H: f64 = 96.0;
/// 预览时被测层平移出可视区的额外间距；尺寸保持不变，不触发页面 resize。
const PREVIEW_OFFSCREEN_GAP: f64 = 2000.0;
/// 视口上报脚本写入 document.title 的哨兵前缀（零宽空格 + 标记）。
/// 被测页面无 IPC 能力，借 title 变化回传真实视口与会话标识。
const TITLE_PREFIX: &str = "\u{200b}LW ";
const REACHABILITY_TIMEOUT: Duration = Duration::from_secs(3);

const MEASURE_SCRIPT: &str = r#"(function(){
  // 视口边框样式（工具辅助标记，pointer-events:none 不拦截页面交互）；
  // 仅当 Rust 侧判定"设定视口小于舞台"时加 html.__layress_frame_on 类才显示。
  // 任务 3.x 采集期间会强制移除，避免工具标记混入真实画面。
  var st = document.createElement("style");
  st.id = "__layress_frame_style";
  st.textContent = 'html.__layress_frame_on::after{content:"";position:fixed;inset:0;pointer-events:none;box-sizing:border-box;border:1px solid rgba(96,112,128,.9);z-index:2147483647}';
  (document.documentElement || document).appendChild(st);
  // 每个 document 生成一次会话标识：页面被重载时会变化，用于核对预览切换是否重载（E08）。
  // 这不是给页面用的原生桥，只是通过标题哨兵向 Rust 单向回报。
  var session = Math.random().toString(36).slice(2) + "-" + Date.now().toString(36);
  window.__layressReport = function(){
    try {
      document.title = "\u200bLW " + JSON.stringify({
        w: window.innerWidth, h: window.innerHeight, dpr: window.devicePixelRatio,
        // 早期 document 里 scrollX/Y 可能为 undefined，显式归一为 0，
        // 不能让「未滚动」被 JSON.stringify 省掉字段后误标成「未取得」。
        scroll_x: Number(window.scrollX || window.pageXOffset || 0),
        scroll_y: Number(window.scrollY || window.pageYOffset || 0),
        session: session
      });
    } catch (e) {}
  };
  window.__layressReport();
  // 初始化脚本早于文档 <title>，页面自身的标题会覆盖哨兵；DOM 就绪后再报一次，
  // 否则首轮不改视口时拿不到实测值（E04 / E07 的页面实测会显示缺失）。
  document.addEventListener("DOMContentLoaded", window.__layressReport);
  window.addEventListener("load", window.__layressReport);
  window.addEventListener("resize", window.__layressReport);
  window.addEventListener("scroll", window.__layressReport);
})();"#;

// ── 状态 ─────────────────────────────────────────────────────────

#[derive(Default)]
struct PageHost {
    /// 用户显式设定的页面视口（逻辑 px）；None = 跟随窗口自适应
    configured: Mutex<Option<(f64, f64)>>,
    /// 样例页面上报的真实视口、滚动与会话标识
    measured: Mutex<Option<MeasuredViewport>>,
    /// 本地样例服务基址
    sample_base: OnceLock<String>,
    /// 采集身份 / 竞态 / 旧现场 / 预览态
    captures: Mutex<CaptureState>,
    /// 启动解析一次的数据根
    data_root: OnceLock<DataRoot>,
    /// 数据根不可解析或不可写时的明确原因；不回落系统用户目录
    data_root_error: Mutex<Option<String>>,
    /// 采集进行中：此时视口边框必须关闭，不得混入截图
    capturing: Mutex<bool>,
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
    /// 采集阶段：started / completed / failed / invalidated / cancelled
    Capture {
        phase: String,
        id: String,
        message: String,
        meta: Option<CaptureMeta>,
    },
    /// 预览切换：active=true 表示静态画面已覆盖工作区
    Preview { active: bool },
    /// 数据根解析与可写性（启动横幅与 UI 可见）
    DataRoot {
        path: String,
        source: String,
        writable: bool,
        error: Option<String>,
    },
}

/// 当前应用句柄。回调闭包拿不到 State/Manager 之外的应用引用，统一经此取。
static APP: OnceLock<AppHandle<tauri::Wry>> = OnceLock::new();

fn emit(app: &AppHandle<tauri::Wry>, event: PageEvent) {
    let _ = app.emit_to(TOOL_LABEL, "page://event", event);
}

fn emit_capture(app: &AppHandle<tauri::Wry>, phase: &str, id: &str, message: impl Into<String>) {
    emit(
        app,
        PageEvent::Capture {
            phase: phase.to_string(),
            id: id.to_string(),
            message: message.into(),
            meta: None,
        },
    );
}

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

fn current_viewport(app: &AppHandle<tauri::Wry>) -> CapturedViewport {
    let state = app.state::<PageHost>();
    let configured = state.configured.lock().ok().and_then(|c| *c);
    let measured = state.measured.lock().ok().and_then(|m| m.clone());
    let (win_w, win_h, fully_visible) = match app.get_window("main") {
        Some(window) => match window.inner_size() {
            Ok(inner) => {
                let scale = window.scale_factor().unwrap_or(1.0);
                let w = inner.width as f64 / scale;
                let h = inner.height as f64 / scale;
                let b = configured.unwrap_or((w, h - TOOLBAR_H));
                (w, h, w + 0.5 >= b.0 && h - TOOLBAR_H + 0.5 >= b.1)
            }
            Err(_) => (0.0, 0.0, false),
        },
        None => (0.0, 0.0, false),
    };
    let bounds = configured.unwrap_or((win_w, win_h - TOOLBAR_H));
    CapturedViewport {
        configured,
        bounds,
        measured,
        fully_visible,
    }
}

/// 依据窗口尺寸重排三块视图；预览态下工具层铺满窗口，被测层只平移、不改尺寸。
fn apply_layout(app: &AppHandle<tauri::Wry>, preview: bool) {
    let Some(window) = app.get_window("main") else {
        return;
    };
    let scale = window.scale_factor().unwrap_or(1.0);
    let Ok(inner) = window.inner_size() else {
        return;
    };
    let win_w = inner.width as f64 / scale;
    let win_h = inner.height as f64 / scale;

    if let Some(tool) = app.get_webview(TOOL_LABEL) {
        let tool_h = if preview { win_h } else { TOOLBAR_H };
        let _ = tool.set_bounds(tauri::Rect {
            position: LogicalPosition::new(0.0, 0.0).into(),
            size: LogicalSize::new(win_w, tool_h).into(),
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
        let position_x = if preview {
            win_w + PREVIEW_OFFSCREEN_GAP
        } else {
            0.0
        };
        let _ = page.set_bounds(tauri::Rect {
            position: LogicalPosition::new(position_x, TOOLBAR_H).into(),
            size: LogicalSize::new(size.0, size.1).into(),
        });
    }
}

fn relayout(app: &AppHandle<tauri::Wry>) {
    // 显式重排：工具条固定顶部，页面按设定或跟随窗口（E04 不静默改视口）
    let preview = app
        .state::<PageHost>()
        .captures
        .lock()
        .map(|s| s.preview)
        .unwrap_or(false);
    apply_layout(app, preview);
}

/// 页面视口边框：设定视口且小于舞台时开启；采集进行中强制关闭（工具标记不得入画）。
fn sync_page_frame(app: &AppHandle<tauri::Wry>) {
    let state = app.state::<PageHost>();
    let capturing = state.capturing.lock().map(|c| *c).unwrap_or(false);
    let configured = state.configured.lock().ok().and_then(|c| *c);
    let on = !capturing
        && match (configured, app.get_window("main")) {
            (Some((w, h)), Some(window)) => window
                .inner_size()
                .ok()
                .map(|inner| {
                    let scale = window.scale_factor().unwrap_or(1.0);
                    let stage_w = inner.width as f64 / scale;
                    let stage_h = inner.height as f64 / scale - TOOLBAR_H;
                    w + 0.5 < stage_w || h + 0.5 < stage_h
                })
                .unwrap_or(false),
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

/// 任何使当前画面不再可靠归属原页面的事件，都作废在途采集（E06）。
fn invalidate_capture(app: &AppHandle<tauri::Wry>, reason: &str) {
    let cancelled = app
        .state::<PageHost>()
        .captures
        .lock()
        .ok()
        .and_then(|mut s| s.invalidate());
    if let Some(id) = cancelled {
        emit_capture(app, "invalidated", &id.to_string(), reason);
    }
}

/// 采集期标志只影响视口边框开关，独立成函数避免 State 临时借用拖到块尾。
fn set_capturing(app: &AppHandle<tauri::Wry>, value: bool) {
    let state = app.state::<PageHost>();
    if let Ok(mut flag) = state.capturing.lock() {
        *flag = value;
    };
}

fn enter_preview(app: &AppHandle<tauri::Wry>) {
    apply_layout(app, true);
    emit(app, PageEvent::Preview { active: true });
}

fn exit_preview(app: &AppHandle<tauri::Wry>) -> Result<(), String> {
    {
        let state = app.state::<PageHost>();
        let mut s = state
            .captures
            .lock()
            .map_err(|_| "采集状态锁不可用".to_string())?;
        s.exit_preview();
    }
    apply_layout(app, false);
    sync_page_frame(app);
    // 触发一次标题哨兵回报：若会话标识变化，说明页面被重载（E08 人工核对依据）。
    if let Some(page) = app.get_webview(PAGE_LABEL) {
        let _ = page.eval("window.__layressReport && window.__layressReport();".to_string());
    }
    emit_viewport(app);
    emit(app, PageEvent::Preview { active: false });
    Ok(())
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
            // 页面自身发起的导航也会使在途采集失效，旧图不得绑定到新地址（E06）。
            if phase == "loading" {
                invalidate_capture(&app, "采集期间页面发生导航，本次结果已作废");
            }
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

#[derive(Debug, Clone, Serialize)]
struct CaptureOutcome {
    meta: CaptureMeta,
    png_base64: String,
    data_root: String,
    data_root_source: String,
}

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
    invalidate_capture(&app, "导航前已作废在途采集");
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
    invalidate_capture(&app, "刷新前已作废在途采集");
    page.navigate(current).map_err(|e| format!("刷新失败：{e}"))
}

#[tauri::command]
fn page_set_size(app: AppHandle<tauri::Wry>, width: f64, height: f64) -> Result<(), String> {
    if !(16.0..=4096.0).contains(&width) || !(16.0..=4096.0).contains(&height) {
        return Err("尺寸超出允许范围（16–4096）".to_string());
    }
    invalidate_capture(&app, "视口变更已作废在途采集");
    {
        let state = app.state::<PageHost>();
        let mut cfg = state
            .configured
            .lock()
            .map_err(|_| "视口状态锁不可用".to_string())?;
        *cfg = Some((width, height));
    }
    let preview = app
        .state::<PageHost>()
        .captures
        .lock()
        .map(|s| s.preview)
        .unwrap_or(false);
    apply_layout(&app, preview);
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
    let root = state.data_root.get().cloned();
    let root_error = state.data_root_error.lock().ok().and_then(|e| e.clone());
    let (in_preview, last) = state
        .captures
        .lock()
        .map(|s| (s.preview, s.last.clone()))
        .unwrap_or((false, None));
    Ok(serde_json::json!({
        "configured": configured,
        "measured": measured,
        "url": url,
        "sampleBase": state.sample_base.get().cloned(),
        "toolbarH": TOOLBAR_H,
        "pageOpen": url.is_some(),
        "dataRoot": root.as_ref().map(|r| r.path.to_string_lossy().to_string()),
        "dataRootSource": root.as_ref().map(|r| r.source.clone()),
        "dataRootWritable": root.is_some() && root_error.is_none(),
        "dataRootError": root_error,
        "inPreview": in_preview,
        "lastCapture": last,
    }))
}

/// 采集当前被测页面：只对 PAGE_LABEL 的原生 WebView 取快照，绑定页面身份与条件，
/// 落盘成功后进入原地静态预览。任何失效 / 失败都明确返回，旧图不冒充本次成功。
#[tauri::command]
async fn capture_start(
    app: AppHandle<tauri::Wry>,
    delay_ms: Option<u64>,
) -> Result<CaptureOutcome, String> {
    // 预览中重新采集：先恢复原布局，但绝不销毁 / 重载被测视图。
    let in_preview = app
        .state::<PageHost>()
        .captures
        .lock()
        .map(|s| s.preview)
        .unwrap_or(false);
    if in_preview {
        exit_preview(&app)?;
    }

    let page = app
        .get_webview(PAGE_LABEL)
        .ok_or_else(|| "尚无被测页面，无法采集".to_string())?;
    let url = page
        .url()
        .map_err(|e| format!("读取当前地址失败：{e}"))?
        .to_string();
    let active = {
        let state = app.state::<PageHost>();
        let mut s = state
            .captures
            .lock()
            .map_err(|_| "采集状态锁不可用".to_string())?;
        let a = s.begin();
        a
    };
    let active_id = active.id;
    let generation = active.generation;
    let id = format!("cap-{}-{active_id}", unix_ms());
    let delay = capture_delay(delay_ms);
    let start_msg = if delay.is_zero() {
        "正在采集被测 WebView 的真实画面 …".to_string()
    } else {
        format!(
            "慢采已开启：快照完成后再等 {} ms，此期间点「刷新」/「取消」/ 再次「采集」可验证失效（未写盘）",
            delay.as_millis()
        )
    };
    emit_capture(&app, "started", &id, start_msg);

    let viewport = current_viewport(&app);
    let measured = app
        .state::<PageHost>()
        .measured
        .lock()
        .ok()
        .and_then(|m| m.clone());
    let started_at = unix_ms();

    // 采集期间关闭工具注入的视口边框；完成后恢复。
    set_capturing(&app, true);
    sync_page_frame(&app);

    let snapshot_page = page.clone();
    let png_result = tauri::async_runtime::spawn_blocking(move || snapshot_png(&snapshot_page))
        .await
        .map_err(|e| format!("采集任务异常结束：{e}"))?;

    // 开发期慢采：延迟刻意卡在「已拿到 PNG，但还没校验身份、没写盘、没显示」之间，
    // 人工此时导航 / 取消 / 再发一次采集，结果必须被判失效且不落盘（E06 运行级负例）。
    if !delay.is_zero() {
        let _ = tauri::async_runtime::spawn_blocking(move || std::thread::sleep(delay)).await;
    }

    set_capturing(&app, false);
    sync_page_frame(&app);

    // 完成条件核对：页面代号、请求号必须都还成立，否则旧结果一律丢弃。
    let still_current = {
        let state = app.state::<PageHost>();
        let s = state
            .captures
            .lock()
            .map_err(|_| "采集状态锁不可用".to_string())?;
        let ok = s.is_current(&active);
        ok
    };
    if !still_current {
        emit_capture(
            &app,
            "invalidated",
            &id,
            "采集期间页面发生导航、关闭或被新请求替代：本次结果已丢弃，未写盘",
        );
        return Err("本次采集已失效：页面已切换或被新请求替代，结果已丢弃".to_string());
    };

    let png = match png_result {
        Ok(bytes) => bytes,
        Err(e) => {
            let _ = fail_capture(&app, &active);
            emit_capture(&app, "failed", &id, format!("采集失败：{e}"));
            return Err(format!("采集失败：{e}"));
        }
    };
    let (width_px, height_px) = png_size(&png).ok_or_else(|| {
        "采集失败：原生快照未返回有效 PNG".to_string()
    })?;

    let root = app
        .state::<PageHost>()
        .data_root
        .get()
        .cloned()
        .ok_or_else(|| "数据根未解析，拒绝写盘".to_string())?;
    if let Some(err) = app
        .state::<PageHost>()
        .data_root_error
        .lock()
        .ok()
        .and_then(|e| e.clone())
    {
        let _ = fail_capture(&app, &active);
        emit_capture(&app, "failed", &id, format!("数据根不可用：{err}"));
        return Err(format!("数据根不可用，拒绝写盘：{err}"));
    }
    // 可写检查必须先于任何持久写入；失败不换目录、不兜底。
    if let Err(e) = ensure_writable(&root.path) {
        let _ = fail_capture(&app, &active);
        emit_capture(&app, "failed", &id, format!("数据根不可写：{e}"));
        return Err(format!("数据根不可写，拒绝写盘：{e}"));
    }

    let finished_at = unix_ms();
    let missing = build_missing(&viewport, &measured);
    let meta = CaptureMeta {
        schema: SCHEMA.to_string(),
        id: id.clone(),
        status: "ok".to_string(),
        page: CapturePage {
            label: PAGE_LABEL.to_string(),
            url,
            generation,
        },
        started_at_unix_ms: started_at,
        finished_at_unix_ms: finished_at,
        viewport,
        image: CaptureImage {
            file: format!("captures/{id}.png"),
            format: "png".to_string(),
            width_px,
            height_px,
            bytes: png.len(),
        },
        conditions: CaptureConditions {
            engine: "系统 WebView（macOS WKWebView）".to_string(),
            method: "WKWebView takeSnapshotWithConfiguration（仅被测视图 bounds）".to_string(),
            tool_frame_hidden: true,
            scroll: measured
                .as_ref()
                .and_then(|m| m.scroll_y)
                .map(|y| format!("窗口滚动 y={y}（内部容器滚动未取得）"))
                .unwrap_or_else(|| "未取得".to_string()),
            missing,
        },
        data_root: root.path.to_string_lossy().to_string(),
    };

    if let Err(e) = persist(&root.path, &meta, &png) {
        let _ = fail_capture(&app, &active);
        emit_capture(&app, "failed", &id, format!("采集落盘失败：{e}"));
        return Err(format!("采集落盘失败：{e}"));
    }

    let finished = {
        let state = app.state::<PageHost>();
        let mut s = state
            .captures
            .lock()
            .map_err(|_| "采集状态锁不可用".to_string())?;
        s.finish(&active, meta.clone())
    };
    if !finished {
        emit_capture(&app, "invalidated", &id, "落盘后身份校验失败：本次结果不显示");
        return Err("落盘后身份校验失败：本次结果不显示".to_string());
    }
    enter_preview(&app);
    emit(
        &app,
        PageEvent::Capture {
            phase: "completed".to_string(),
            id: id.clone(),
            message: format!(
                "采集完成：{}×{} px，产物位于数据根 captures/",
                width_px, height_px
            ),
            meta: Some(meta.clone()),
        },
    );
    Ok(CaptureOutcome {
        meta,
        png_base64: base64_encode(&png),
        data_root: root.path.to_string_lossy().to_string(),
        data_root_source: root.source,
    })
}

fn fail_capture(app: &AppHandle<tauri::Wry>, active: &capture::ActiveCapture) -> bool {
    app.state::<PageHost>()
        .captures
        .lock()
        .map(|mut s| s.fail(active))
        .unwrap_or(false)
}

/// 缺失条件逐条标注，不把「取不到」写成 0 或空值。
fn build_missing(viewport: &CapturedViewport, measured: &Option<MeasuredViewport>) -> Vec<String> {
    let mut missing = Vec::new();
    match measured {
        None => missing.push("页面视口 / DPR / 会话标识未上报（title 哨兵可能被页面覆盖）".to_string()),
        Some(m) => {
            if m.dpr <= 0.0 {
                missing.push("DPR 未取得".to_string());
            }
            if m.scroll_y.is_none() {
                missing.push("页面滚动位置未取得".to_string());
            }
            if m.session.is_none() {
                missing.push("页面会话标识未取得".to_string());
            }
        }
    }
    if viewport.bounds.0 <= 0.0 || viewport.bounds.1 <= 0.0 {
        missing.push("页面原生边界未取得".to_string());
    }
    missing.push("被测页面内部容器滚动、跨 iframe 合成与动态画面原子性不在本次采集可验证范围".to_string());
    missing.push("视口边框关闭依赖注入脚本；严格 CSP 页面是否拦截注入未实测（拦截时边框本就不显示）".to_string());
    missing
}

/// 取消当前在途采集（同一时刻只允许一个，新请求会自动替代旧请求）。
#[tauri::command]
fn capture_cancel(app: AppHandle<tauri::Wry>) -> Result<bool, String> {
    let (ok, cancelled_id) = {
        let state = app.state::<PageHost>();
        let mut s = state
            .captures
            .lock()
            .map_err(|_| "采集状态锁不可用".to_string())?;
        let id = s.active.as_ref().map(|a| a.id);
        let ok = id.map(|n| s.cancel(n)).unwrap_or(false);
        (ok, id)
    };
    if ok {
        emit_capture(
            &app,
            "cancelled",
            &cancelled_id.unwrap_or(0).to_string(),
            "本次采集已取消",
        );
    }
    Ok(ok)
}

#[tauri::command]
fn preview_exit(app: AppHandle<tauri::Wry>) -> Result<serde_json::Value, String> {
    exit_preview(&app)?;
    let url = app
        .get_webview(PAGE_LABEL)
        .and_then(|p| p.url().ok())
        .map(|u| u.to_string());
    Ok(serde_json::json!({ "url": url }))
}

/// 开发期「慢采」延迟：只在 debug 构建生效，上限 15s，默认 0（产品路径零影响）。
/// 用途：让人工在采集未完成时触发导航 / 取消 / 新请求，验证 E06 的运行级负例；
/// 43ms 的真实采集速度下人工根本来不及操作，没有这个开关就只能靠纯逻辑单测。
#[cfg(debug_assertions)]
fn capture_delay(delay_ms: Option<u64>) -> Duration {
    delay_ms
        .map(|v| Duration::from_millis(v.min(15_000)))
        .unwrap_or(Duration::ZERO)
}

#[cfg(not(debug_assertions))]
fn capture_delay(_delay_ms: Option<u64>) -> Duration {
    Duration::ZERO
}

/// 采集前的最近一次成功现场，供「旧图只作旧现场」显示；失败不覆盖它。
#[tauri::command]
fn capture_latest(app: AppHandle<tauri::Wry>) -> Result<Option<CaptureMeta>, String> {
    Ok(app
        .state::<PageHost>()
        .captures
        .lock()
        .map_err(|_| "采集状态锁不可用".to_string())?
        .last
        .clone())
}

// ── 开发期自测探针 ───────────────────────────────────────────────

/// 仅 debug 构建且显式 `LAYRESS_PROBE_CAPTURE=1` 时执行：自动打开样例页 → 采集 → 打印结果。
/// 目的是无人值守核实「WKWebView 原生快照 → PNG → 数据根」这条链路在真机上可用，
/// 不参与产品默认路径，也不代替 E05–E08 的人工验收。
#[cfg(debug_assertions)]
fn count_capture_pngs(root: &std::path::Path) -> usize {
    std::fs::read_dir(root.join("captures"))
        .map(|entries| {
            entries
                .filter_map(Result::ok)
                .filter(|e| e.path().extension().is_some_and(|x| x == "png"))
                .count()
        })
        .unwrap_or(0)
}

#[cfg(debug_assertions)]
fn maybe_run_capture_probe(app: &AppHandle<tauri::Wry>) {
    let mode = std::env::var("LAYRESS_PROBE_CAPTURE").unwrap_or_default();
    if mode.is_empty() {
        return;
    }
    let handle = app.clone();
    tauri::async_runtime::spawn(async move {
        // 等工具层与样例服务就绪；探针不赶时间，避免与窗口创建竞态。
        std::thread::sleep(Duration::from_secs(2));
        let Some(base) = handle.state::<PageHost>().sample_base.get().cloned() else {
            eprintln!("[layress][probe] 样例服务未就绪，探针失败");
            return;
        };
        if let Err(e) = page_navigate(handle.clone(), format!("{base}/")).await {
            eprintln!("[layress][probe] 导航失败：{e}");
            return;
        }
        std::thread::sleep(Duration::from_secs(2));

        // race 模式：慢采期间刷新页面，核对结果失效且没有新 PNG 落盘（E06 运行级负例）。
        if mode == "race" {
            let root = handle.state::<PageHost>().data_root.get().cloned();
            let before = root.as_ref().map(|r| count_capture_pngs(&r.path)).unwrap_or(0);
            let racer = handle.clone();
            let task = tauri::async_runtime::spawn(async move {
                capture_start(racer, Some(3000)).await
            });
            let _ = tauri::async_runtime::spawn_blocking(|| {
                std::thread::sleep(Duration::from_millis(300));
            })
            .await;
            if let Err(e) = page_refresh(handle.clone()) {
                eprintln!("[layress][probe] race 刷新失败：{e}");
            }
            match task.await {
                Ok(Ok(_)) => eprintln!("[layress][probe] race 失败：采集本应失效却成功了"),
                Ok(Err(e)) => {
                    let after = root
                        .as_ref()
                        .map(|r| count_capture_pngs(&r.path))
                        .unwrap_or(0);
                    let verdict = if after == before { "通过" } else { "失败（有新增 PNG）" };
                    eprintln!(
                        "[layress][probe] race {verdict}：采集已失效（{e}）；captures PNG 数 {before} → {after}"
                    );
                }
                Err(e) => eprintln!("[layress][probe] race 任务异常：{e}"),
            }
            return;
        }

        match capture_start(handle.clone(), None).await {
            Ok(outcome) => eprintln!(
                "[layress][probe] 采集成功 id={} url={} 图片={}x{} 字节={} 数据根={}",
                outcome.meta.id,
                outcome.meta.page.url,
                outcome.meta.image.width_px,
                outcome.meta.image.height_px,
                outcome.meta.image.bytes,
                outcome.data_root
            ),
            Err(e) => eprintln!("[layress][probe] 采集失败：{e}"),
        }
    });
}

#[cfg(not(debug_assertions))]
fn maybe_run_capture_probe(_app: &AppHandle<tauri::Wry>) {}

/// 开发期工具层可见性探针：`LAYRESS_PROBE_TOOL=1` 时把采集结果条点亮并截取工具层 WebView，
/// 用来证明提示确实落在 96px 高的工具层视口内（而不是被视口裁掉）。仅 debug 构建生效。
#[cfg(debug_assertions)]
fn maybe_run_tool_probe(app: &AppHandle<tauri::Wry>) {
    if std::env::var_os("LAYRESS_PROBE_TOOL").is_none() {
        return;
    }
    let handle = app.clone();
    tauri::async_runtime::spawn(async move {
        std::thread::sleep(Duration::from_secs(5));
        let Some(tool) = handle.get_webview(TOOL_LABEL) else {
            eprintln!("[layress][toolprobe] 工具层未就绪");
            return;
        };
        if let Ok(bounds) = tool.bounds() {
            eprintln!("[layress][toolprobe] 工具层 bounds={bounds:?}");
        }
        if let Some(window) = handle.get_window("main") {
            eprintln!(
                "[layress][toolprobe] 窗口 inner={:?} scale={:?}",
                window.inner_size().ok(),
                window.scale_factor().ok()
            );
        }
        // 同时自报 DOM 诊断，避免「看不到」时无法区分是元素缺失、eval 失效还是被裁切。
        let show = "(function(){var t=document.getElementById('capture-toast');\
var s=document.getElementById('status-line');\
var info='toast='+(!!t)+' status='+(!!s)+' innerH='+window.innerHeight+\
' body='+document.body.innerHTML.length+' href='+location.href;\
document.title='[layress-probe] '+info;\
if(s){s.textContent=info;}\
if(t){t.textContent=info;t.title=info;t.className='show';}})();";
        let _ = tool.eval(show.to_string());
        std::thread::sleep(Duration::from_millis(600));
        match snapshot_png(&tool) {
            Ok(png) => {
                if let Some(root) = handle.state::<PageHost>().data_root.get().cloned() {
                    let path = root.path.join("tool-probe.png");
                    match std::fs::write(&path, &png) {
                        Ok(()) => eprintln!(
                            "[layress][toolprobe] 工具层截图：{}（{} 字节）",
                            path.display(),
                            png.len()
                        ),
                        Err(e) => eprintln!("[layress][toolprobe] 截图写入失败：{e}"),
                    }
                }
            }
            Err(e) => eprintln!("[layress][toolprobe] 工具层截图失败：{e}"),
        }
        // 截图后清掉探针文字，交付给用户的窗口保持干净。
        let _ = tool.eval(
            "(function(){var t=document.getElementById('capture-toast');\
if(t){t.textContent='';t.title='';t.className='';}})();"
                .to_string(),
        );
    });
}

#[cfg(not(debug_assertions))]
fn maybe_run_tool_probe(_app: &AppHandle<tauri::Wry>) {}

// ── 启动 ─────────────────────────────────────────────────────────

pub fn run() {
    tauri::Builder::default()
        .manage(PageHost::default())
        .invoke_handler(tauri::generate_handler![
            sample_url,
            page_navigate,
            page_refresh,
            page_set_size,
            page_state,
            capture_start,
            capture_cancel,
            capture_latest,
            preview_exit
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

            // 数据根：启动即解析并做可写检查，结果进日志、事件与 UI；不可写明确失败，不回落。
            let (root, error) = match resolve_data_root() {
                Ok(root) => match ensure_writable(&root.path) {
                    Ok(()) => {
                        eprintln!(
                            "[layress] 数据根：{}（来源：{}，可写：是）",
                            root.path.display(),
                            root.source
                        );
                        (Some(root), None)
                    }
                    Err(e) => {
                        eprintln!("[layress] 数据根不可写，采集将明确失败：{e}");
                        (Some(root), Some(e))
                    }
                },
                Err(e) => {
                    eprintln!("[layress] 数据根解析失败，采集将明确失败：{e}");
                    (None, Some(e))
                }
            };
            if let Some(r) = &root {
                let _ = app.state::<PageHost>().data_root.set(r.clone());
            }
            if let Some(e) = &error {
                if let Ok(mut slot) = app.state::<PageHost>().data_root_error.lock() {
                    *slot = Some(e.clone());
                }
            }
            emit(
                app.handle(),
                PageEvent::DataRoot {
                    path: root
                        .as_ref()
                        .map(|r| r.path.to_string_lossy().to_string())
                        .unwrap_or_default(),
                    source: root.map(|r| r.source).unwrap_or_default(),
                    writable: error.is_none(),
                    error,
                },
            );

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
                .incognito(true)
                // 开发期探针借标题变化把工具层 DOM 诊断回报到 stderr（不影响产品路径）。
                .on_document_title_changed(|_webview, title| {
                    if let Some(info) = title.strip_prefix("[layress-probe] ") {
                        eprintln!("[layress][toolprobe] {info}");
                    }
                }),
                LogicalPosition::new(0.0, 0.0),
                LogicalSize::new(inner.width as f64 / scale, TOOLBAR_H),
            )?;

            let _ = APP.set(app.handle().clone());
            maybe_run_capture_probe(app.handle());
            maybe_run_tool_probe(app.handle());

            let handle = app.handle().clone();
            window.on_window_event(move |event| match event {
                WindowEvent::Resized(_) => {
                    relayout(&handle);
                    sync_page_frame(&handle);
                    emit_viewport(&handle);
                }
                WindowEvent::Destroyed => {
                    // 退出取消待完成采集，不让失效结果冒充成功（任务 3.2 / 4.4）
                    invalidate_capture(&handle, "窗口关闭，在途采集已取消");
                }
                _ => {}
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("Layress 桌面应用启动失败");
}
