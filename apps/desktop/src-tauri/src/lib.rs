//! Layress 桌面工作台核心编排（add-tauri-page-host 任务 2.x–3.x）。
//!
//! 单窗口三块显式布局：可信工具层（"main"，持有能力）在顶部条带，
//! 被测页面（"page"）固定在条带之下且无任何 Tauri 能力。
//! 预览态把工具层铺满窗口、把被测层**只平移出可视区且尺寸不变**：
//! 因此返回操作页面时不销毁、不重载、不改视口（E07 / E08）。
//! 真实截图只取被测 WKWebView 自身（E05），采集身份与失效规则见 `capture.rs`。

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod capture;
// 任务 1.3：配置落盘 / 独占写锁 / 串行写 / 原子替换（纯模块，尚未接入启动与命令）。
mod config_store;
mod fixture;
// 任务 1.2：项目配置模型 / 校验 / 稳定错误码（纯逻辑，尚未接入命令与文件存储）。
mod project_config;

use capture::{
    base64_encode, ensure_writable, persist, png_size, resolve_data_root, snapshot_png, unix_ms,
    CaptureConditions, CaptureImage, CaptureMeta, CapturePage, CaptureState, CapturedViewport,
    DataRoot, MeasuredViewport, SCHEMA,
};
use config_store::ConfigStore;
use project_config::{ConfigError, PresetId, ProjectConfig};
use serde::Serialize;
use std::net::TcpStream;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Mutex, OnceLock};
#[cfg(debug_assertions)]
use std::sync::atomic::AtomicBool;
use std::time::Duration;
use tauri::webview::{DownloadEvent, NewWindowResponse, PageLoadEvent, PermissionResponse};
use tauri::{
    AppHandle, Emitter, LogicalPosition, LogicalSize, Manager, State, Url, WebviewUrl, WindowEvent,
};

const TOOL_LABEL: &str = "main";
const PAGE_LABEL: &str = "page";
const STATUS_LABEL: &str = "status";
/// 工具条高度（逻辑像素）；布局为互不重叠的三块：
/// 工具条 [0, TOOLBAR_H]、被测页面 [TOOLBAR_H, 窗口底 - STATUS_H]、状态栏 [窗口底 - STATUS_H, 窗口底]。
const TOOLBAR_H: f64 = 96.0;
/// 窗口底部独立状态栏高度（逻辑像素）。工具层 WebView 实测 CSS 视口只有 64px，
/// 放不进任何结果条，所以采集反馈走这块独立视图，不再浮在工具栏上。
const STATUS_H: f64 = 42.0;

/// 页面区可用高度（逻辑 px）：窗口去掉顶部工具条与底部状态栏。
fn stage_height(win_h: f64) -> f64 {
    (win_h - TOOLBAR_H - STATUS_H).max(0.0)
}
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
    /// 任务 1.1：启动期打开的配置存储（数据根可写且解析成功时才有）
    config_store: OnceLock<ConfigStore>,
    /// 配置不可用 / 损坏 / 未知版本的结构化原因；不自动重建、不回落
    config_error: Mutex<Option<ConfigError>>,
    /// 项目设置（编辑）模式：工具层铺满、被测层平移出视口；与预览 / 采集互斥
    settings: Mutex<bool>,
    /// 稳定项目 ID 的进程内序号（与毫秒时间戳组合，不作权限来源）
    project_seq: AtomicU64,
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
    let bounds = configured.unwrap_or((win_w, stage_height(win_h)));
    let fully_visible = win_w + 0.5 >= bounds.0 && stage_height(win_h) + 0.5 >= bounds.1;
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
                let b = configured.unwrap_or((w, stage_height(h)));
                (w, h, w + 0.5 >= b.0 && stage_height(h) + 0.5 >= b.1)
            }
            Err(_) => (0.0, 0.0, false),
        },
        None => (0.0, 0.0, false),
    };
    let bounds = configured.unwrap_or((win_w, stage_height(win_h)));
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
        let size = configured.unwrap_or((win_w, stage_height(win_h)));
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
    // 状态栏常驻窗口最底部，预览态也不移走；它后于工具层创建，位于其上层。
    if let Some(status) = app.get_webview(STATUS_LABEL) {
        let _ = status.set_bounds(tauri::Rect {
            position: LogicalPosition::new(0.0, (win_h - STATUS_H).max(0.0)).into(),
            size: LogicalSize::new(win_w, STATUS_H).into(),
        });
    }
}

/// 把底部状态栏提到最上层（仅 macOS 已验证）。
/// 被测页面是用户点「打开样例页」时才 `add_child`，后创建的 WKWebView 天然压在
/// 启动期创建的状态栏之上；设定视口（1280×704）时页面 bounds 会延伸到底部 42px，
/// 窗口缩小后状态栏被页面盖住 ⇒ 红条 / 失败提示全部看不见。
/// wry 在 macOS 的 `reparent` 实现就是 `contentView.addSubview`，等于把视图移到
/// subviews 末尾（最上层）；reparent 到同一窗口不改变归属，只重排层级。
#[cfg(target_os = "macos")]
fn bring_status_to_front(app: &AppHandle<tauri::Wry>) {
    let Some(status) = app.get_webview(STATUS_LABEL) else {
        return;
    };
    let Some(window) = app.get_window("main") else {
        return;
    };
    if let Err(e) = status.reparent(&window) {
        eprintln!("[layress] 状态栏置顶失败：{e}");
    }
}

/// 非 macOS 的 reparent 行为未实测，不擅自调整层级；如实留到对应平台验证。
#[cfg(not(target_os = "macos"))]
fn bring_status_to_front(_app: &AppHandle<tauri::Wry>) {}

/// 任务 0.1 探针期间保持铺满：窗口 Resized 会触发 relayout，若不覆盖状态就会把
/// 工具层复位成 96px（首轮 full3 实测 595×96 / cssH=64 / 控件全被裁）。
/// 产品正式实现由明确的编辑/预览模式状态承担同样的职责；这里只是 debug 探针的等价占位。
#[cfg(debug_assertions)]
static LAYOUT_PROBE_FULL: AtomicBool = AtomicBool::new(false);

#[cfg(debug_assertions)]
fn layout_probe_holds_full() -> bool {
    LAYOUT_PROBE_FULL.load(Ordering::Relaxed)
}

#[cfg(not(debug_assertions))]
fn layout_probe_holds_full() -> bool {
    false
}

fn relayout(app: &AppHandle<tauri::Wry>) {
    // 显式重排：工具条固定顶部，页面按设定或跟随窗口（E04 不静默改视口）。
    // 预览 / 项目设置 / 0.1 探针都要求工具层铺满，缩窗触发 relayout 时也必须保持。
    let host = app.state::<PageHost>();
    let settings = host.settings.lock().map(|s| *s).unwrap_or(false);
    let preview = host
        .captures
        .lock()
        .map(|s| s.preview)
        .unwrap_or(false)
        || settings
        || layout_probe_holds_full();
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
                    let stage_h = stage_height(inner.height as f64 / scale);
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
        configured.unwrap_or((win_w, stage_height(win_h)))
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
            // 下载一律拒绝：不写默认下载目录（E13）。打日志便于区分
            // 「系统没发起下载请求」与「发起后已拒绝」——两者都不能写成红条通过。
            match &event {
                DownloadEvent::Requested { url, .. } => {
                    eprintln!("[layress][request] 下载请求已拒绝：{url}");
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
                _ => eprintln!("[layress][request] 收到非 Requested 下载事件（未放行下载）"),
            }
            false
        })
        .on_permission_request(|_webview, kind| {
            eprintln!("[layress][request] 设备权限请求已拒绝：{kind:?}");
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
    // 页面是此时才 add_child 的，会盖住启动期创建的状态栏；立刻把状态栏提回最上层，
    // 否则窗口缩小后底部 42px 的红条会被页面埋掉（用户 2026-11-14 复验发现）。
    bring_status_to_front(app);
    emit_viewport(app);
    Ok(())
}

// ── 地址校验与可达性 ─────────────────────────────────────────────

/// 宿主与配置模型共用的 HTTP(S) 地址校验；`project_config` 也走这里，
/// 避免配置里的 URL 与导航地址出现两套边界。
pub(crate) fn validate_url(raw: &str) -> Result<Url, String> {
    let raw = raw.trim();
    if raw.is_empty() {
        return Err("地址为空".to_string());
    }
    // 「http:///path」会被 url crate 把路径段规范化成主机名（"path"），
    // 只靠后续 host 非空检查会放行；再等 DNS 碰巧失败来兜底不可靠（mDNS 可能解析出主机）。
    // 这层 authority 检查**只对 http(s) 生效**：否则 file:///etc/passwd 会先被判「缺少主机名」，
    // 掩盖真正的拒绝理由「scheme 不受支持」（2026-11-13 用户人工复验指出）。
    if let Some(pos) = raw.find("://") {
        let scheme = &raw[..pos];
        if scheme.eq_ignore_ascii_case("http") || scheme.eq_ignore_ascii_case("https") {
            let authority = &raw[pos + 3..];
            let first = authority.split(['/', '?', '#']).next().unwrap_or("");
            if first.is_empty() {
                return Err("地址缺少主机名".to_string());
            }
        }
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

/// 同步的 TCP 可达性探测：抽成独立函数，便于单测直接断言而不依赖 async runtime。
fn probe_reachable(url: &Url) -> Result<(), String> {
    use std::net::ToSocketAddrs;
    let host = url.host_str().unwrap_or_default().to_string();
    let port = url
        .port_or_known_default()
        .ok_or_else(|| "地址缺少端口且 scheme 无默认端口".to_string())?;
    let target = format!("{host}:{port}");
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
}

/// 导航前做 TCP 可达性预检：失败明确报错且不触碰当前页面（E03）。
/// DNS 与 connect 放在阻塞线程，避免拖住 UI 线程；`connect_timeout` 只约束连接阶段。
async fn check_reachable(url: &Url) -> Result<(), String> {
    let owned = url.clone();
    tauri::async_runtime::spawn_blocking(move || probe_reachable(&owned))
        .await
        .map_err(|e| format!("可达性检查任务失败：{e}"))?
}

/// E03 的纯逻辑部分：地址校验与可达性预检各有一正一反样本。
/// 运行级「预检失败后当前页面不变、不误报加载成功」仍须人工 GUI 复验，机器单测不替代。
#[cfg(test)]
mod tests {
    use super::*;
    use std::net::TcpListener;

    #[test]
    fn validate_url_rejects_invalid_addresses() {
        let cases = [
            "",
            "   ",
            "example.com",
            "javascript:alert(1)",
            "file:///etc/passwd",
            "data:text/html,hi",
            "ftp://example.com/",
            "http://",
            "https://",
            "http:///path",
            "http:////example.com/",
            "http://example.com:99999/",
        ];
        for raw in cases {
            assert!(
                validate_url(raw).is_err(),
                "本应被拒绝的地址却通过校验：{raw:?}"
            );
        }
    }

    #[test]
    fn validate_url_accepts_http_s_and_normalizes_scheme() {
        let u = validate_url("  HTTP://Example.com:8080/path?q=1  ").expect("合法 http 地址");
        assert_eq!(u.scheme(), "http");
        assert_eq!(u.host_str(), Some("example.com"));
        assert_eq!(u.port(), Some(8080));
        assert_eq!(
            validate_url("https://example.com/").unwrap().scheme(),
            "https"
        );
    }

    /// 拒绝理由必须归因准确：危险 scheme 不能被说成「缺少主机名」，
    /// 否则用户会以为补上主机名就能打开 file://（2026-11-13 人工复验反馈）。
    #[test]
    fn reject_reason_keeps_scheme_before_host_check() {
        for raw in ["file:///etc/passwd", "ftp://example.com/", "data:text/html,hi"] {
            let err = validate_url(raw).unwrap_err();
            assert!(err.contains("scheme"), "{raw:?} 的错因被误归因：{err}");
            assert!(!err.contains("主机名"), "{raw:?} 不该报缺少主机名：{err}");
        }
        let err = validate_url("http:///path").unwrap_err();
        assert!(err.contains("主机名"), "http(s) 畸形地址仍应报缺少主机名：{err}");
    }

    /// 已知坏样本：刚释放的本地端口没有监听，预检必须判不可达。
    #[test]
    fn probe_reachable_rejects_closed_port() {
        let listener = TcpListener::bind("127.0.0.1:0").expect("绑定临时端口");
        let port = listener.local_addr().expect("读取临时端口").port();
        drop(listener);
        let url = Url::parse(&format!("http://127.0.0.1:{port}/")).unwrap();
        assert!(probe_reachable(&url).is_err(), "端口已关闭，预检必须失败");
    }

    /// 正例：本地有监听时预检通过，避免把「一律失败」误当成拒绝逻辑正确。
    #[test]
    fn probe_reachable_accepts_listening_port() {
        let listener = TcpListener::bind("127.0.0.1:0").expect("绑定临时端口");
        let port = listener.local_addr().expect("读取临时端口").port();
        let url = Url::parse(&format!("http://127.0.0.1:{port}/")).unwrap();
        assert!(probe_reachable(&url).is_ok(), "本地监听端口应可达");
    }
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
    // 任务 1.1 的最小模式仲裁：项目设置（编辑）中禁止采集。
    // 完整仲裁表（预览→设置→返回、迟到结果隔离等）仍归任务 3.4，不在此假称已实现。
    if app
        .state::<PageHost>()
        .settings
        .lock()
        .map(|s| *s)
        .unwrap_or(false)
    {
        return Err("项目设置编辑中禁止采集；请先关闭设置".to_string());
    }
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

/// 采集结果条：可信工具层调用，Rust 转发到**无能力**的底部状态栏 WebView。
/// 状态栏 HTML 无脚本、不出现在任何 capability 里，只接受这里注入的文本与级别。
#[tauri::command]
fn status_bar_set(app: AppHandle<tauri::Wry>, text: String, level: String) -> Result<(), String> {
    let status = app
        .get_webview(STATUS_LABEL)
        .ok_or_else(|| "底部状态栏未就绪".to_string())?;
    let level = match level.as_str() {
        "ok" | "error" | "pending" => level,
        _ => "info".to_string(),
    };
    let text_js = serde_json::to_string(&text).map_err(|e| format!("状态文本序列化失败：{e}"))?;
    let level_js = serde_json::to_string(&level).map_err(|e| format!("状态级别序列化失败：{e}"))?;
    let js = format!(
        "(function(){{var b=document.getElementById('bar');var m=document.getElementById('msg');\
if(m)m.textContent={text_js};if(b)b.className={level_js};}})();"
    );
    status.eval(js).map_err(|e| format!("状态栏更新失败：{e}"))
}

// ── 任务 1.1：配置模型 / 错误码 / 存储接入 ────────────────────────

/// 配置命令的结构化错误：8 个稳定 code 由 `ConfigError` 带入，前端按 code 映射中文。
#[derive(Debug, Clone, Serialize)]
struct CommandError {
    code: String,
    message: String,
}

impl From<ConfigError> for CommandError {
    fn from(e: ConfigError) -> Self {
        Self {
            code: e.code_str().to_string(),
            message: e.message,
        }
    }
}

fn cmd_err(code: project_config::ConfigErrorCode, message: impl Into<String>) -> CommandError {
    CommandError::from(ConfigError::new(code, message))
}

fn preset_name(preset: PresetId) -> &'static str {
    match preset {
        PresetId::Desktop => "desktop",
        PresetId::Tablet => "tablet",
        PresetId::Phone => "phone",
        PresetId::Custom => "custom",
    }
}

fn parse_preset(raw: &str) -> Result<PresetId, CommandError> {
    match raw {
        "desktop" => Ok(PresetId::Desktop),
        "tablet" => Ok(PresetId::Tablet),
        "phone" => Ok(PresetId::Phone),
        "custom" => Ok(PresetId::Custom),
        other => Err(cmd_err(
            project_config::ConfigErrorCode::InvalidInput,
            format!("未知视口预设「{other}」"),
        )),
    }
}

/// 稳定 ID：毫秒时间戳 + 进程内序号；不含路径分隔符，绝不作为文件路径或权限来源。
fn new_project_id(now_ms: u128, seq: u64) -> String {
    format!("p-{now_ms}-{seq}")
}

/// 模式仲裁的最小判据：设置不与静态预览 / 采集中叠加（完整仲裁表归任务 3.4）。
fn can_enter_settings(preview: bool, capturing: bool) -> bool {
    !preview && !capturing
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct ProjectView {
    id: String,
    name: String,
    start_url: String,
    last_loaded_url: Option<String>,
    preset_id: String,
    width: u32,
    height: u32,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct ConfigStateView {
    revision: u64,
    active_project_id: Option<String>,
    projects: Vec<ProjectView>,
    writable: bool,
    data_root: String,
    error: Option<CommandError>,
}

fn project_view(p: &project_config::Project) -> ProjectView {
    ProjectView {
        id: p.id.clone(),
        name: p.name.clone(),
        start_url: p.start_url.clone(),
        last_loaded_url: p.last_loaded_url.clone(),
        preset_id: preset_name(p.viewport.preset_id).to_string(),
        width: p.viewport.width,
        height: p.viewport.height,
    }
}

fn view_from_config(
    config: &ProjectConfig,
    writable: bool,
    data_root: String,
    error: Option<CommandError>,
) -> ConfigStateView {
    ConfigStateView {
        revision: config.revision,
        active_project_id: config.active_project_id.clone(),
        projects: config.projects.iter().map(project_view).collect(),
        writable,
        data_root,
        error,
    }
}

/// 只有「已保存状态」才发布给前端：读存储里的当前模型，不采用任何乐观展示。
fn current_view(app: &AppHandle<tauri::Wry>) -> ConfigStateView {
    let host = app.state::<PageHost>();
    let error: Option<CommandError> = host
        .config_error
        .lock()
        .ok()
        .and_then(|s| s.clone())
        .map(CommandError::from);
    let data_root = host
        .data_root
        .get()
        .map(|r| r.path.to_string_lossy().to_string())
        .unwrap_or_default();
    match host.config_store.get() {
        Some(store) => view_from_config(&store.config(), store.is_writable(), data_root, error),
        None => view_from_config(
            &ProjectConfig::empty(),
            false,
            data_root,
            error.or_else(|| {
                Some(cmd_err(
                    project_config::ConfigErrorCode::ReadFailed,
                    "配置存储未就绪（数据根不可用）",
                ))
            }),
        ),
    }
}

#[tauri::command]
fn config_state(app: AppHandle<tauri::Wry>) -> ConfigStateView {
    current_view(&app)
}

#[tauri::command]
fn config_create_project(
    app: AppHandle<tauri::Wry>,
    name: String,
    start_url: String,
    preset_id: String,
) -> Result<ConfigStateView, CommandError> {
    let host = app.state::<PageHost>();
    let store = host.config_store.get().ok_or_else(|| {
        cmd_err(
            project_config::ConfigErrorCode::ReadFailed,
            "配置存储未就绪，不能创建项目",
        )
    })?;
    let preset = parse_preset(&preset_id)?;
    let seq = host.project_seq.fetch_add(1, Ordering::Relaxed) + 1;
    let id = new_project_id(capture::unix_ms(), seq);
    let mut config = store.config();
    let expected = config.revision;
    config
        .create_project(&id, &name, &start_url, preset)
        .map_err(CommandError::from)?;
    store.commit(config, expected).map_err(CommandError::from)?;
    Ok(current_view(&app))
}

#[tauri::command]
fn config_rename_project(
    app: AppHandle<tauri::Wry>,
    id: String,
    name: String,
) -> Result<ConfigStateView, CommandError> {
    let host = app.state::<PageHost>();
    let store = host.config_store.get().ok_or_else(|| {
        cmd_err(
            project_config::ConfigErrorCode::ReadFailed,
            "配置存储未就绪，不能重命名项目",
        )
    })?;
    let mut config = store.config();
    let expected = config.revision;
    let project = config
        .projects
        .iter_mut()
        .find(|p| p.id == id)
        .ok_or_else(|| {
            cmd_err(
                project_config::ConfigErrorCode::InvalidInput,
                format!("项目 {id} 不存在"),
            )
        })?;
    project.rename(&name).map_err(CommandError::from)?;
    store.commit(config, expected).map_err(CommandError::from)?;
    Ok(current_view(&app))
}

/// 编辑既有项目的名称与起始地址（任务 2.1）。ID 不变；仅保存字段不导航、
/// 不覆盖最近地址（E04：单独编辑起始地址的判据由编排层「不触发导航」保证）。
/// 无效输入被拒且已保存配置不变；保存成功才发布新状态。
#[tauri::command]
fn config_update_project(
    app: AppHandle<tauri::Wry>,
    id: String,
    name: String,
    start_url: String,
) -> Result<ConfigStateView, CommandError> {
    let host = app.state::<PageHost>();
    let store = host.config_store.get().ok_or_else(|| {
        cmd_err(
            project_config::ConfigErrorCode::ReadFailed,
            "配置存储未就绪，不能编辑项目",
        )
    })?;
    let mut config = store.config();
    let expected = config.revision;
    let project = config
        .projects
        .iter_mut()
        .find(|p| p.id == id)
        .ok_or_else(|| {
            cmd_err(
                project_config::ConfigErrorCode::InvalidInput,
                format!("项目 {id} 不存在"),
            )
        })?;
    project.rename(&name).map_err(CommandError::from)?;
    project.set_start_url(&start_url).map_err(CommandError::from)?;
    store.commit(config, expected).map_err(CommandError::from)?;
    Ok(current_view(&app))
}

/// 用户确认后的备份重建（任务 1.5 / design §6 / E06）。仅当配置存储未就绪且
/// 错误属于「文件不可用」三类（读取失败 / 损坏 / 未知版本）时允许；第二实例
/// （CONFIG_LOCKED）与数据根错误不在内。成功后立即重新打开存储并清除错误状态。
/// 前端必须先做两段确认（会清空项目记忆、保留截图），本命令不做二次弹窗。
#[tauri::command]
fn config_rebuild(app: AppHandle<tauri::Wry>) -> Result<ConfigStateView, CommandError> {
    let host = app.state::<PageHost>();
    if host.config_store.get().is_some() {
        return Err(cmd_err(
            project_config::ConfigErrorCode::InvalidInput,
            "配置存储正常，无需重建",
        ));
    }
    let existing = host.config_error.lock().ok().and_then(|s| s.clone());
    let code = existing
        .as_ref()
        .map(|e| e.code)
        .ok_or_else(|| cmd_err(project_config::ConfigErrorCode::InvalidInput, "没有记录配置错误，无需重建"))?;
    match code {
        project_config::ConfigErrorCode::ReadFailed
        | project_config::ConfigErrorCode::Corrupt
        | project_config::ConfigErrorCode::VersionUnsupported => {}
        _ => {
            return Err(cmd_err(
                code,
                "当前错误类型不支持备份重建（如另一实例占用或数据根未解析），先解除对应问题",
            ));
        }
    }
    let root = host
        .data_root
        .get()
        .cloned()
        .ok_or_else(|| cmd_err(code, "数据根未解析，不能重建"))?;
    let outcome = config_store::rebuild_with_backup(&root.path, config_store::WriteFault::None)
        .map_err(CommandError::from)?;
    eprintln!(
        "[layress] 配置已备份重建：备份 {}，空配置 revision {}",
        outcome.backup_path.display(),
        outcome.revision
    );
    match config_store::ConfigStore::open(&root.path) {
        Ok(store) => {
            let _ = app.state::<PageHost>().config_store.set(store);
            if let Ok(mut slot) = app.state::<PageHost>().config_error.lock() {
                *slot = None;
            }
        }
        Err(e) => {
            // 理论上不应发生：重建刚以同一根成功写入合法空配置；如实记录不冒充成功
            eprintln!("[layress] 重建后重新打开配置存储失败：{e}");
            if let Ok(mut slot) = app.state::<PageHost>().config_error.lock() {
                *slot = Some(e);
            }
        }
    }
    Ok(current_view(&app))
}

/// 进入项目设置：工具层铺满工作区、被测层等尺寸平移（0.1 已实机验证的布局）。
/// 与静态预览 / 采集互斥；进入失败不改变布局，也不写配置。
#[tauri::command]
fn settings_enter(app: AppHandle<tauri::Wry>) -> Result<ConfigStateView, String> {
    let host = app.state::<PageHost>();
    let preview = host.captures.lock().map(|s| s.preview).unwrap_or(false);
    let capturing = host.capturing.lock().map(|c| *c).unwrap_or(false);
    if !can_enter_settings(preview, capturing) {
        return Err("静态预览或采集进行中，不能打开项目设置；请先返回页面 / 完成采集".to_string());
    }
    if let Ok(mut flag) = host.settings.lock() {
        *flag = true;
    }
    apply_layout(&app, true);
    Ok(current_view(&app))
}

#[tauri::command]
fn settings_exit(app: AppHandle<tauri::Wry>) {
    if let Ok(mut flag) = app.state::<PageHost>().settings.lock() {
        *flag = false;
    }
    apply_layout(&app, false);
    sync_page_frame(&app);
}

#[cfg(test)]
mod config_command_tests {
    use super::*;

    #[test]
    fn 视口预设名正反映射() {
        for (raw, expected) in [
            ("desktop", PresetId::Desktop),
            ("tablet", PresetId::Tablet),
            ("phone", PresetId::Phone),
            ("custom", PresetId::Custom),
        ] {
            assert_eq!(parse_preset(raw).unwrap(), expected);
            assert_eq!(preset_name(expected), raw);
        }
        assert_eq!(
            parse_preset("watch").unwrap_err().code,
            "CONFIG_INVALID_INPUT"
        );
    }

    #[test]
    fn 结构化错误保留配置错误码() {
        let e = CommandError::from(ConfigError::new(
            project_config::ConfigErrorCode::VersionUnsupported,
            "未知版本",
        ));
        assert_eq!(e.code, "CONFIG_VERSION_UNSUPPORTED");
        assert_eq!(e.message, "未知版本");
        let json = serde_json::to_value(&e).unwrap();
        assert_eq!(json["code"], "CONFIG_VERSION_UNSUPPORTED");
    }

    #[test]
    fn 状态视图使用驼峰键名() {
        let mut config = ProjectConfig::empty();
        config
            .create_project("p-a", "项目 A", "https://example.com", PresetId::Phone)
            .unwrap();
        let view = view_from_config(&config, true, "/tmp/root".to_string(), None);
        let json = serde_json::to_value(&view).unwrap();
        assert_eq!(json["activeProjectId"], "p-a");
        assert_eq!(json["dataRoot"], "/tmp/root");
        // 模型保存规范化后的 URL（url crate 补尾斜杠），不是地址栏草稿原串。
        assert_eq!(json["projects"][0]["startUrl"], "https://example.com/");
        assert_eq!(json["projects"][0]["presetId"], "phone");
        assert!(json["error"].is_null());
    }

    #[test]
    fn 项目_id_唯一且不含路径分隔符() {
        let a = new_project_id(1_700_000_000_000, 1);
        let b = new_project_id(1_700_000_000_000, 2);
        assert_ne!(a, b);
        assert!(!a.contains('/') && !a.contains('\\') && !a.contains(".."));
        assert!(a.starts_with("p-"));
    }

    #[test]
    fn 设置模式不与预览或采集叠加() {
        assert!(can_enter_settings(false, false));
        assert!(!can_enter_settings(true, false));
        assert!(!can_enter_settings(false, true));
        assert!(!can_enter_settings(true, true));
    }

    /// 任务 2.1 / E01：编辑起始地址与重命名共用一条更新路径——ID 不变、
    /// URL 规范化、不覆盖最近地址；无效输入被拒且已保存字段不变。
    #[test]
    fn 编辑起始地址更新字段且无效输入不改原值() {
        let mut config = ProjectConfig::empty();
        config
            .create_project("p-a", "项目 A", "http://localhost:4173/a", PresetId::Desktop)
            .unwrap();
        config.projects[0]
            .set_last_loaded_url("https://example.com/final")
            .unwrap();
        let revision_before = config.revision;

        // 有效编辑：改名 + 新起始地址；ID / 最近地址 / 预设尺寸不变，URL 规范化
        let project = config.projects.iter_mut().find(|p| p.id == "p-a").unwrap();
        project.rename("改名后的 A").unwrap();
        project.set_start_url("  HTTP://Example.com:8080/new  ").unwrap();
        let stored = project.clone();
        assert_eq!(stored.id, "p-a");
        assert_eq!(stored.name, "改名后的 A");
        assert_eq!(stored.start_url, "http://example.com:8080/new");
        assert_eq!(
            stored.last_loaded_url.as_deref(),
            Some("https://example.com/final"),
            "编辑起始地址不覆盖最近地址"
        );
        assert_eq!((stored.viewport.width, stored.viewport.height), (1280, 800));

        // 无效编辑：坏 scheme 被拒，已保存字段保持上一轮成功值
        let project = config.projects.iter_mut().find(|p| p.id == "p-a").unwrap();
        let err = project.set_start_url("file:///etc/passwd").unwrap_err();
        assert_eq!(err.code_str(), "CONFIG_INVALID_INPUT");
        assert_eq!(config.projects[0].start_url, "http://example.com:8080/new");
        assert_eq!(config.projects[0].name, "改名后的 A");
        assert_eq!(config.revision, revision_before, "失败不推进 revision");

        // 不存在的 ID：命令层查询语义（find 返回 None），与视图一致性由驼峰键名测试覆盖
        assert!(config.projects.iter_mut().find(|p| p.id == "p-missing").is_none());
    }
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

/// 开发期状态栏可见性探针：`LAYRESS_PROBE_TOOL=1` 时点亮底部状态栏并截取该 WebView，
/// 用来证明结果条确实显示在窗口最底部且不与工具栏重叠。仅 debug 构建生效。
#[cfg(debug_assertions)]
fn maybe_run_status_probe(app: &AppHandle<tauri::Wry>) {
    if std::env::var_os("LAYRESS_PROBE_TOOL").is_none() {
        return;
    }
    let handle = app.clone();
    tauri::async_runtime::spawn(async move {
        std::thread::sleep(Duration::from_secs(5));
        let Some(status) = handle.get_webview(STATUS_LABEL) else {
            eprintln!("[layress][statusprobe] 底部状态栏未就绪");
            return;
        };
        if let Ok(bounds) = status.bounds() {
            eprintln!("[layress][statusprobe] 状态栏 bounds={bounds:?}");
        }
        // DOM 自报 + 走真实命令链路（status_bar_set → eval → 状态栏 DOM）点亮状态栏，
        // 这样截图验证的是产品路径，而不是探针自己写 DOM。
        let report = "(function(){var b=document.getElementById('bar');\
var m=document.getElementById('msg');\
var info='bar='+(!!b)+' msg='+(!!m)+' innerH='+window.innerHeight+\
' body='+document.body.innerHTML.length+' href='+location.href;\
document.title='[layress-probe] '+info;})();";
        let _ = status.eval(report.to_string());
        if let Err(e) = status_bar_set(
            handle.clone(),
            "[probe] 底部状态栏可见性自检：看到这条说明结果条在窗口最底部且不遮工具栏".to_string(),
            "ok".to_string(),
        ) {
            eprintln!("[layress][statusprobe] status_bar_set 失败：{e}");
        }
        std::thread::sleep(Duration::from_millis(600));
        match snapshot_png(&status) {
            Ok(png) => {
                if let Some(root) = handle.state::<PageHost>().data_root.get().cloned() {
                    let path = root.path.join("status-probe.png");
                    match std::fs::write(&path, &png) {
                        Ok(()) => eprintln!(
                            "[layress][statusprobe] 状态栏截图：{}（{} 字节）",
                            path.display(),
                            png.len()
                        ),
                        Err(e) => eprintln!("[layress][statusprobe] 截图写入失败：{e}"),
                    }
                }
            }
            Err(e) => eprintln!("[layress][statusprobe] 状态栏截图失败：{e}"),
        }
        // 截图后恢复「就绪」，交付给用户的窗口保持干净。
        let _ = status_bar_set(handle.clone(), "就绪".to_string(), "info".to_string());
    });
}

#[cfg(not(debug_assertions))]
fn maybe_run_status_probe(_app: &AppHandle<tauri::Wry>) {}

// ── 任务 0.1：全工作区编辑布局可行性探针（仅 debug 构建） ──────────

/// `LAYRESS_PROBE_LAYOUT=1` 时自动量「常态 → 工具层铺满 → 恢复」三组数据：
/// 工具层 CSS 视口、#bar 高度、临时诊断表单的控件可见 / 可命中 / 可聚焦数、
/// 三块视图 bounds 与被测页设定尺寸。结果走既有 `[layress-probe]` 标题通道打到 stderr，
/// 不写产品数据、不改前端源码（因此不触发 BUILD_TAG 递增）、不进入采集状态机。
/// 机器只负责记录；控件肉眼可见、状态栏层级、焦点手感仍归人工复核（I11）。
#[cfg(debug_assertions)]
const LAYOUT_PROBE_JS: &str = r##"(function(){
var stage='%STAGE%';
var form=document.getElementById('__layress_probe_form');
if(stage==='restore'&&form){form.remove();form=null;}
if(stage.indexOf('full')===0 && !form){
  form=document.createElement('div');
  form.id='__layress_probe_form';
  form.setAttribute('style','position:fixed;left:16px;top:120px;width:360px;padding:12px;z-index:2147483000;background:#fff;border:1px solid #607d8b;font:13px -apple-system;box-sizing:border-box');
  form.innerHTML='<div style="margin-bottom:6px">0.1 布局诊断表单（探针临时注入）</div>'+
    '<input id="pf-name" placeholder="项目名称" style="display:block;width:100%;margin:6px 0">'+
    '<input id="pf-url" placeholder="起始地址" style="display:block;width:100%;margin:6px 0">'+
    '<input id="pf-w" placeholder="宽" style="display:block;width:100%;margin:6px 0">'+
    '<input id="pf-h" placeholder="高" style="display:block;width:100%;margin:6px 0">'+
    '<button id="pf-save">保存</button> <button id="pf-cancel">取消</button>';
  document.body.appendChild(form);
}
var controls=form?Array.prototype.slice.call(form.querySelectorAll('input,button')):[];
var visible=0,hit=0,focusable=0,clipped=[];
controls.forEach(function(el){
  var r=el.getBoundingClientRect();
  var inView=r.width>0&&r.height>0&&r.top>=0&&r.left>=0&&r.bottom<=window.innerHeight&&r.right<=window.innerWidth;
  if(inView){visible++;}else{clipped.push(el.id+':'+Math.round(r.top)+'-'+Math.round(r.bottom));}
  var top=document.elementFromPoint(r.left+r.width/2,r.top+r.height/2);
  if(top===el||(top&&el.contains(top)))hit++;
  el.focus();
  if(document.activeElement===el)focusable++;
});
var bar=document.getElementById('bar');
var report={
  probe:'r3',stage:stage,cssW:window.innerWidth,cssH:window.innerHeight,dpr:window.devicePixelRatio,
  barH:bar?Math.round(bar.getBoundingClientRect().height):null,
  controls:controls.length,visible:visible,hit:hit,focusable:focusable,clipped:clipped,
  build:(document.getElementById('build-tag')||{}).textContent||null
};
document.title='[layress-probe] LAYOUT '+JSON.stringify(report);
})();"##;

#[cfg(debug_assertions)]
fn probe_layout_dom(app: &AppHandle<tauri::Wry>, stage: &str) {
    if let Some(tool) = app.get_webview(TOOL_LABEL) {
        let _ = tool.eval(LAYOUT_PROBE_JS.replace("%STAGE%", stage));
    }
}

#[cfg(debug_assertions)]
fn probe_layout_bounds(app: &AppHandle<tauri::Wry>, stage: &str) {
    let tool = app.get_webview(TOOL_LABEL).and_then(|w| w.bounds().ok());
    let page = app.get_webview(PAGE_LABEL).and_then(|w| w.bounds().ok());
    let status = app.get_webview(STATUS_LABEL).and_then(|w| w.bounds().ok());
    let configured = app
        .state::<PageHost>()
        .configured
        .lock()
        .ok()
        .and_then(|c| *c);
    eprintln!(
        "[layress][layoutprobe] probe=r3 {stage} tool={tool:?} page={page:?} status={status:?} configured={configured:?}"
    );
    // 触发一次 title 哨兵，让被测页自报实际视口；铺满前后应一致（尺寸不变，只改位置）。
    if let Some(page_view) = app.get_webview(PAGE_LABEL) {
        let _ = page_view.eval("window.__layressReport && window.__layressReport();".to_string());
    }
}

/// WKWebView 的 bounds 必须在主线程设置；用 run_on_main_thread + 回执确保已生效再测。
#[cfg(debug_assertions)]
fn probe_apply_full(app: &AppHandle<tauri::Wry>, preview: bool) {
    let handle = app.clone();
    let (tx, rx) = std::sync::mpsc::channel();
    let dispatcher = handle.clone();
    if let Err(e) = dispatcher.run_on_main_thread(move || {
        apply_layout(&handle, preview);
        let _ = tx.send(());
    }) {
        eprintln!("[layress][layoutprobe] 主线程重排失败：{e}");
        return;
    }
    if rx.recv_timeout(Duration::from_secs(5)).is_err() {
        eprintln!("[layress][layoutprobe] 主线程重排超时，本次测量可能无效");
    }
}

/// r3：探针自己把窗口缩到最小，覆盖 E07 小窗口判据并验证 Resized 时 relayout 仍保持铺满；
/// 恢复阶段再还原到 1280×800。仅 debug 构建、仅探针启用时执行。
#[cfg(debug_assertions)]
fn probe_set_window_size(app: &AppHandle<tauri::Wry>, width: f64, height: f64) {
    let handle = app.clone();
    let (tx, rx) = std::sync::mpsc::channel();
    let dispatcher = handle.clone();
    if let Err(e) = dispatcher.run_on_main_thread(move || {
        if let Some(window) = handle.get_window("main") {
            if let Err(e) = window.set_size(LogicalSize::new(width, height)) {
                eprintln!("[layress][layoutprobe] 调整窗口尺寸失败：{e}");
            }
        }
        let _ = tx.send(());
    }) {
        eprintln!("[layress][layoutprobe] 主线程调整窗口失败：{e}");
        return;
    }
    if rx.recv_timeout(Duration::from_secs(5)).is_err() {
        eprintln!("[layress][layoutprobe] 调整窗口超时，后续小窗口测量可能无效");
    }
}

#[cfg(debug_assertions)]
fn maybe_run_layout_probe(app: &AppHandle<tauri::Wry>) {
    if std::env::var_os("LAYRESS_PROBE_LAYOUT").is_none() {
        return;
    }
    let handle = app.clone();
    tauri::async_runtime::spawn(async move {
        eprintln!(
            "[layress][layoutprobe] 探针 r3 已启动：自动打开样例页、自动缩窗，全程不用点任何按钮；回显里必须有 probe=r3，否则是旧程序"
        );
        std::thread::sleep(Duration::from_secs(3));
        // 0.1 要核对铺满前后被测页尺寸，页面必须一开始就存在；探针自动打开，不必人工点按钮。
        match handle.state::<PageHost>().sample_base.get().cloned() {
            Some(base) => match page_navigate(handle.clone(), format!("{base}/")).await {
                Ok(()) => eprintln!("[layress][layoutprobe] 已自动打开样例页：{base}/"),
                Err(e) => eprintln!(
                    "[layress][layoutprobe] 自动打开样例页失败：{e}（本轮被测页证据缺失）"
                ),
            },
            None => eprintln!("[layress][layoutprobe] 样例服务未就绪（本轮被测页证据缺失）"),
        }
        std::thread::sleep(Duration::from_secs(1));
        probe_layout_dom(&handle, "normal");
        probe_layout_bounds(&handle, "normal");
        std::thread::sleep(Duration::from_millis(500));

        // 置位后 Resized → relayout 也保持铺满，避免首轮 full3 那种「量到复位后的 96px」。
        LAYOUT_PROBE_FULL.store(true, Ordering::Relaxed);
        eprintln!(
            "[layress][layoutprobe] 开始铺满工具层（约 14 秒）：前 3 轮大窗口，之后自动缩到最小；请看白色表单、底部状态栏和输入框"
        );
        for i in 0..6 {
            if i == 3 {
                // 主动触发 Resized：既覆盖小窗口判据，也验证竞态修复后 relayout 仍保持铺满。
                probe_set_window_size(&handle, 520.0, 400.0);
                eprintln!("[layress][layoutprobe] 已自动缩到最小窗口 520×400，后 3 轮是小窗口判据");
                std::thread::sleep(Duration::from_millis(900));
            }
            // 每轮重新铺满：按当前窗口尺寸重排。
            probe_apply_full(&handle, true);
            std::thread::sleep(Duration::from_millis(700));
            let stage = format!("full{i}");
            probe_layout_dom(&handle, &stage);
            probe_layout_bounds(&handle, &stage);
            std::thread::sleep(Duration::from_millis(1300));
        }

        // 先复位标志再还原窗口，保证恢复后任何 Resized 都按真实模式重排。
        LAYOUT_PROBE_FULL.store(false, Ordering::Relaxed);
        probe_set_window_size(&handle, 1280.0, 800.0);
        probe_apply_full(&handle, false);
        std::thread::sleep(Duration::from_millis(700));
        probe_layout_dom(&handle, "restore");
        probe_layout_bounds(&handle, "restore");
        eprintln!("[layress][layoutprobe] 已恢复常态并还原窗口；请把终端里所有 [layress] 开头的行复制给编写者");
    });
}

#[cfg(not(debug_assertions))]
fn maybe_run_layout_probe(_app: &AppHandle<tauri::Wry>) {}

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
            preview_exit,
            status_bar_set,
            config_state,
            config_create_project,
            config_rename_project,
            config_update_project,
            config_rebuild,
            settings_enter,
            settings_exit
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
            // 任务 1.4：显式相对路径在解析层拒绝，错误码直达 config_error / 状态栏红条。
            let (root, root_err): (Option<capture::DataRoot>, Option<(project_config::ConfigErrorCode, String)>) =
                match resolve_data_root() {
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
                            (Some(root), Some((project_config::ConfigErrorCode::WriteFailed, e)))
                        }
                    },
                    Err(e) => {
                        eprintln!("[layress] 数据根解析失败，采集将明确失败：{e}");
                        let code = match &e {
                            capture::DataRootError::RelativeUnsupported { .. } => {
                                project_config::ConfigErrorCode::DataRootRelativeUnsupported
                            }
                            capture::DataRootError::Other(_) => project_config::ConfigErrorCode::WriteFailed,
                        };
                        (None, Some((code, e.to_string())))
                    }
                };
            if let Some(r) = &root {
                let _ = app.state::<PageHost>().data_root.set(r.clone());
            }
            if let Some((_, message)) = &root_err {
                if let Ok(mut slot) = app.state::<PageHost>().data_root_error.lock() {
                    *slot = Some(message.clone());
                }
            }
            // 任务 1.1：只在数据根解析成功且可写时打开配置存储；损坏 / 未知版本 / 锁不可用
            // 都记录结构化原因，界面进入明确的错误状态，不自动重建、不回落到系统用户目录。
            if root_err.is_none() {
                if let Some(r) = &root {
                    match ConfigStore::open(&r.path) {
                        Ok(store) => {
                            eprintln!("[layress] 配置存储已就绪：{}", store.path().display());
                            let _ = app.state::<PageHost>().config_store.set(store);
                        }
                        Err(e) => {
                            eprintln!("[layress] 配置存储不可用：{e}");
                            if let Ok(mut slot) = app.state::<PageHost>().config_error.lock() {
                                *slot = Some(e);
                            }
                        }
                    }
                }
            } else if let Ok(mut slot) = app.state::<PageHost>().config_error.lock() {
                let (code, message) = root_err
                    .clone()
                    .unwrap_or((project_config::ConfigErrorCode::WriteFailed, "数据根不可用".to_string()));
                *slot = Some(ConfigError::new(code, message));
            }
            emit(
                app.handle(),
                PageEvent::DataRoot {
                    path: root
                        .as_ref()
                        .map(|r| r.path.to_string_lossy().to_string())
                        .unwrap_or_default(),
                    source: root.as_ref().map(|r| r.source.clone()).unwrap_or_default(),
                    writable: root_err.is_none(),
                    error: root_err.as_ref().map(|(_, m)| m.clone()),
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

            // 底部状态栏：独立 WebView，窗口最底部一条；不与工具条 / 被测页面重叠。
            // 无 capability、页面无脚本，文本只由 status_bar_set 命令注入。
            let win_h = inner.height as f64 / scale;
            let _status = window.add_child(
                tauri::webview::WebviewBuilder::new(
                    STATUS_LABEL,
                    WebviewUrl::App("status.html".into()),
                )
                .incognito(true)
                .on_document_title_changed(|_webview, title| {
                    if let Some(info) = title.strip_prefix("[layress-probe] ") {
                        eprintln!("[layress][statusprobe] {info}");
                    }
                }),
                LogicalPosition::new(0.0, (win_h - STATUS_H).max(0.0)),
                LogicalSize::new(inner.width as f64 / scale, STATUS_H),
            )?;

            let _ = APP.set(app.handle().clone());
            maybe_run_capture_probe(app.handle());
            maybe_run_status_probe(app.handle());
            maybe_run_layout_probe(app.handle());

            let handle = app.handle().clone();
            window.on_window_event(move |event| match event {
                WindowEvent::Resized(_) => {
                    relayout(&handle);
                    sync_page_frame(&handle);
                    emit_viewport(&handle);
                }
                WindowEvent::Destroyed => {
                    // 退出取消待完成采集，不让失效结果冒充成功（任务 3.2 / 4.4）。
                    invalidate_capture(&handle, "窗口关闭，在途采集已取消");
                    // 4.4：只清理本次启动的可写探针残留；持久截图 / 元数据按记录策略留在数据根。
                    // 明确不删除数据根以外任何文件，重启后仍能读取上次现场。
                    if let Some(root) = handle.state::<PageHost>().data_root.get() {
                        let probe = root.path.join(".write-probe");
                        if probe.exists() {
                            let _ = std::fs::remove_file(&probe);
                        }
                        eprintln!(
                            "[layress] 退出：数据根 {} 保留，未清理数据根外文件",
                            root.path.display()
                        );
                    }
                }
                _ => {}
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("Layress 桌面应用启动失败");
}
