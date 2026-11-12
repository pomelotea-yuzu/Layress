//! 采集模块（add-tauri-page-host 任务 3.x）。
//!
//! 职责边界：
//! - 只截「被测页面」这一个原生 WebView（macOS WKWebView `takeSnapshot`），不整屏截图、不 DOM 重绘；
//! - 采集产物落在产品数据根内，附带版本化元数据（E05）；
//! - 采集身份 / 竞态失效与旧图隔离是纯状态机，可脱离 GUI 单测（E06）。
//!
//! 预览布局与命令编排在 `lib.rs`；本模块不碰窗口和 WebView 生命周期。

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub const SCHEMA: &str = "layress.capture/v1";
pub const SNAPSHOT_TIMEOUT: Duration = Duration::from_secs(15);

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MeasuredViewport {
    pub w: f64,
    pub h: f64,
    pub dpr: f64,
    /// 页面上报的滚动位置；title 哨兵未带该字段时为 null（缺失如实标注）
    #[serde(default)]
    pub scroll_x: Option<f64>,
    #[serde(default)]
    pub scroll_y: Option<f64>,
    /// 每个 document 生成一次；页面被重载时会变化，用于核对预览切换是否重载（E08）
    #[serde(default)]
    pub session: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapturedViewport {
    pub configured: Option<(f64, f64)>,
    pub bounds: (f64, f64),
    pub measured: Option<MeasuredViewport>,
    pub fully_visible: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapturePage {
    /// 被测视图标签；采集只针对该标签
    pub label: String,
    /// 采集开始时读取到的实际地址，不是地址栏输入
    pub url: String,
    /// 导航代号：任何导航 / 刷新都会递增，用于判定在途采集是否失效
    pub generation: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaptureImage {
    /// 相对数据根的路径
    pub file: String,
    pub format: String,
    pub width_px: u32,
    pub height_px: u32,
    pub bytes: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaptureConditions {
    pub engine: String,
    pub method: String,
    /// 采集前已关闭工具注入的视口边框（严格 CSP 页面可能拦截注入，如实记为 false）
    pub tool_frame_hidden: bool,
    pub scroll: String,
    /// 未取得或无法确认的条件，逐条列出，不把缺失写成 0 / 空值
    pub missing: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaptureMeta {
    pub schema: String,
    pub id: String,
    /// 仅成功落盘的采集为 "ok"；失败 / 取消不产生元数据文件
    pub status: String,
    pub page: CapturePage,
    pub started_at_unix_ms: u128,
    pub finished_at_unix_ms: u128,
    pub viewport: CapturedViewport,
    pub image: CaptureImage,
    pub conditions: CaptureConditions,
    /// 数据根解析结果（绝对路径写进元数据，UI 与报告同时显示）
    pub data_root: String,
}

// ── 竞态状态机（E06）─────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActiveCapture {
    pub id: u64,
    pub generation: u64,
}

/// 单个被测页面的采集状态。所有变更都经此结构，便于用单元测试锁住口径：
/// 导航 / 关闭 / 新请求会使在途采集失效；失效结果不得写盘、不得冒充本次成功、不得替换旧图。
#[derive(Debug, Default)]
pub struct CaptureState {
    pub generation: u64,
    pub seq: u64,
    pub active: Option<ActiveCapture>,
    pub last: Option<CaptureMeta>,
    pub preview: bool,
}

impl CaptureState {
    /// 开始一次采集；新请求会自然替换旧 active，使旧结果在 finish 时被判失效。
    pub fn begin(&mut self) -> ActiveCapture {
        self.seq += 1;
        let active = ActiveCapture {
            id: self.seq,
            generation: self.generation,
        };
        self.active = Some(active.clone());
        active
    }

    /// 导航 / 刷新 / 关闭窗口：递增代号并作废在途采集。返回被作废的采集 id（供事件回报）。
    pub fn invalidate(&mut self) -> Option<u64> {
        self.generation += 1;
        self.active.take().map(|a| a.id)
    }

    pub fn cancel(&mut self, id: u64) -> bool {
        match self.active.as_ref() {
            Some(a) if a.id == id => {
                self.active = None;
                true
            }
            _ => false,
        }
    }

    /// 结果回来时是否仍属于当前页面与当前请求。
    pub fn is_current(&self, a: &ActiveCapture) -> bool {
        self.active.as_ref() == Some(a) && a.generation == self.generation
    }

    /// 只有当前请求才能落盘并成为「最近一次成功」；旧图保持不动。
    pub fn finish(&mut self, a: &ActiveCapture, meta: CaptureMeta) -> bool {
        if !self.is_current(a) {
            return false;
        }
        self.active = None;
        self.last = Some(meta);
        self.preview = true;
        true
    }

    /// 明确失败 / 取消：清在途状态，但绝不触碰 last（旧图只作旧现场）。
    pub fn fail(&mut self, a: &ActiveCapture) -> bool {
        if !self.is_current(a) {
            return false;
        }
        self.active = None;
        true
    }

    pub fn exit_preview(&mut self) {
        self.preview = false;
    }
}

// ── 编码与工具 ───────────────────────────────────────────────────

pub fn unix_ms() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis())
        .unwrap_or(0)
}

/// 从 PNG IHDR 读出像素尺寸；比额外引入图像解码依赖更轻，且元数据只信图片字节本身。
pub fn png_size(bytes: &[u8]) -> Option<(u32, u32)> {
    if bytes.len() < 24 || &bytes[0..8] != b"\x89PNG\r\n\x1a\n" {
        return None;
    }
    let w = u32::from_be_bytes(bytes[16..20].try_into().ok()?);
    let h = u32::from_be_bytes(bytes[20..24].try_into().ok()?);
    (w > 0 && h > 0).then_some((w, h))
}

/// 标准 base64（RFC 4648），用于把 PNG 经 IPC 交给可信工具层作静态预览。
pub fn base64_encode(data: &[u8]) -> String {
    const TABLE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::with_capacity(data.len().div_ceil(3) * 4);
    for chunk in data.chunks(3) {
        let b0 = chunk[0] as u32;
        let b1 = chunk.get(1).copied().unwrap_or(0) as u32;
        let b2 = chunk.get(2).copied().unwrap_or(0) as u32;
        let n = (b0 << 16) | (b1 << 8) | b2;
        out.push(TABLE[(n >> 18) as usize & 63] as char);
        out.push(TABLE[(n >> 12) as usize & 63] as char);
        out.push(if chunk.len() > 1 {
            TABLE[(n >> 6) as usize & 63] as char
        } else {
            '='
        });
        out.push(if chunk.len() > 2 {
            TABLE[n as usize & 63] as char
        } else {
            '='
        });
    }
    out
}

// ── 数据根（任务 4.1 的最小先行实现，完整验收仍属 4.x）─────────

#[derive(Debug, Clone)]
pub struct DataRoot {
    pub path: PathBuf,
    pub source: String,
}

/// 优先级：`--data-dir` > `LAYRESS_DATA_DIR` > 可执行文件目录旁 `.layress/`。
/// 不依赖启动工作目录，不回落任何系统用户目录。
pub fn resolve_data_root() -> Result<DataRoot, String> {
    let args: Vec<std::ffi::OsString> = std::env::args_os().collect();
    for (i, arg) in args.iter().enumerate() {
        let s = arg.to_string_lossy();
        if let Some(v) = s.strip_prefix("--data-dir=") {
            if !v.is_empty() {
                return Ok(DataRoot {
                    path: PathBuf::from(v),
                    source: "命令行 --data-dir".to_string(),
                });
            }
        }
        if s == "--data-dir" {
            if let Some(next) = args.get(i + 1) {
                return Ok(DataRoot {
                    path: PathBuf::from(next),
                    source: "命令行 --data-dir".to_string(),
                });
            }
            return Err("--data-dir 缺少路径参数".to_string());
        }
    }
    if let Some(v) = std::env::var_os("LAYRESS_DATA_DIR") {
        if !v.is_empty() {
            return Ok(DataRoot {
                path: PathBuf::from(v),
                source: "环境变量 LAYRESS_DATA_DIR".to_string(),
            });
        }
    }
    let exe = std::env::current_exe().map_err(|e| format!("无法定位可执行文件：{e}"))?;
    let dir = exe
        .parent()
        .ok_or_else(|| "可执行文件没有父目录".to_string())?;
    Ok(DataRoot {
        path: dir.join(".layress"),
        source: "可执行文件目录旁 .layress/".to_string(),
    })
}

/// 任何持久写入之前先做真实可写检查；失败明确返回，不换目录。
pub fn ensure_writable(root: &Path) -> Result<(), String> {
    fs::create_dir_all(root).map_err(|e| format!("数据根不可创建（{}）：{e}", root.display()))?;
    let probe = root.join(".write-probe");
    fs::write(&probe, b"layress").map_err(|e| format!("数据根不可写（{}）：{e}", root.display()))?;
    fs::remove_file(&probe).map_err(|e| format!("数据根探针无法清理：{e}"))?;
    Ok(())
}

/// 写 PNG + 版本化元数据；调用方必须已完成身份校验与可写检查。
pub fn persist(root: &Path, meta: &CaptureMeta, png: &[u8]) -> Result<(), String> {
    let dir = root.join("captures");
    fs::create_dir_all(&dir).map_err(|e| format!("采集目录不可创建：{e}"))?;
    let png_path = dir.join(format!("{}.png", meta.id));
    let json_path = dir.join(format!("{}.json", meta.id));
    fs::write(&png_path, png).map_err(|e| format!("截图写入失败：{e}"))?;
    let json = serde_json::to_vec_pretty(meta).map_err(|e| format!("元数据序列化失败：{e}"))?;
    fs::write(&json_path, json).map_err(|e| format!("元数据写入失败：{e}"))?;
    Ok(())
}

// ── 原生快照（仅 macOS）──────────────────────────────────────────

#[cfg(target_os = "macos")]
pub fn snapshot_png(page: &tauri::Webview<tauri::Wry>) -> Result<Vec<u8>, String> {
    use std::sync::mpsc;

    let (tx, rx) = mpsc::channel::<Result<Vec<u8>, String>>();
    page.with_webview(move |platform| {
        // with_webview 的闭包在主线程执行；WKWebView 的 completionHandler 也在主线程回调，
        // 因此调用方必须在非主线程等待，否则会互相等待。
        unsafe { trigger_snapshot(platform.inner(), tx) };
    })
    .map_err(|e| format!("无法取得被测 WebView 的原生句柄：{e}"))?;

    rx.recv_timeout(SNAPSHOT_TIMEOUT)
        .map_err(|e| format!("原生快照回调超时或中断：{e}"))?
}

#[cfg(target_os = "macos")]
type Id = *mut objc2::runtime::AnyObject;

/// 在被测 WKWebView 上发起 takeSnapshot；配置传 nil ⇒ 取该视图整个 bounds，
/// 天然不含工具层、其他窗口与屏幕背景（E05）。所有失败路径都经通道明确回报。
#[cfg(target_os = "macos")]
unsafe fn trigger_snapshot(inner: *mut std::ffi::c_void, tx: std::sync::mpsc::Sender<Result<Vec<u8>, String>>) {
    use block2::{DynBlock, RcBlock};
    use objc2::runtime::AnyObject;

    if inner.is_null() {
        let _ = tx.send(Err("WKWebView 原生指针为空".to_string()));
        return;
    }
    let view: &AnyObject = unsafe { &*inner.cast::<AnyObject>() };
    let block = RcBlock::new(move |image: Id, error: Id| {
        let _ = tx.send(unsafe { encode_png(image, error) });
    });
    // WKWebView 会 copy completionHandler 再异步持有；Rust 侧 RcBlock 在本次调用结束后释放自己的引用。
    let handler: &DynBlock<dyn Fn(Id, Id)> = &block;
    let config: Id = std::ptr::null_mut();
    let _: () = unsafe {
        objc2::msg_send![view, takeSnapshotWithConfiguration: config, completionHandler: handler]
    };
}

/// NSImage → TIFF → NSBitmapImageRep → PNG bytes。
/// 任一环节失败都返回明确错误，不回退整屏截图、不用空白图冒充成功。
#[cfg(target_os = "macos")]
unsafe fn encode_png(image: Id, error: Id) -> Result<Vec<u8>, String> {
    use objc2::runtime::{AnyClass, AnyObject};
    use std::ffi::{c_char, CStr};

    if !error.is_null() {
        let err: &AnyObject = unsafe { &*error };
        let desc: Id = unsafe { objc2::msg_send![err, localizedDescription] };
        let detail = if desc.is_null() {
            "未提供错误描述".to_string()
        } else {
            let desc_ref: &AnyObject = unsafe { &*desc };
            let ptr: *const c_char = unsafe { objc2::msg_send![desc_ref, UTF8String] };
            if ptr.is_null() {
                "错误描述无法读取".to_string()
            } else {
                unsafe { CStr::from_ptr(ptr) }.to_string_lossy().into_owned()
            }
        };
        return Err(format!("WKWebView 快照失败：{detail}"));
    }
    if image.is_null() {
        return Err("WKWebView 快照返回空图像".to_string());
    }
    let img: &AnyObject = unsafe { &*image };
    let tiff: Id = unsafe { objc2::msg_send![img, TIFFRepresentation] };
    if tiff.is_null() {
        return Err("NSImage 无法生成 TIFF 数据，PNG 编码不可用（未作降级替代）".to_string());
    }
    let cls = AnyClass::get(c"NSBitmapImageRep")
        .ok_or_else(|| "本机缺少 NSBitmapImageRep 类".to_string())?;
    let rep: Id = unsafe { objc2::msg_send![cls, imageRepWithData: tiff] };
    if rep.is_null() {
        return Err("无法从 TIFF 构造 NSBitmapImageRep".to_string());
    }
    // NSBitmapImageFileTypePNG = 4；properties 传 nil，使用默认编码参数。
    let props: Id = std::ptr::null_mut();
    let rep_ref: &AnyObject = unsafe { &*rep };
    let png: Id = unsafe {
        objc2::msg_send![rep_ref, representationUsingType: 4usize, properties: props]
    };
    if png.is_null() {
        return Err("NSBitmapImageRep 无法编码 PNG".to_string());
    }
    let png_ref: &AnyObject = unsafe { &*png };
    let len: usize = unsafe { objc2::msg_send![png_ref, length] };
    let ptr: *const u8 = unsafe { objc2::msg_send![png_ref, bytes] };
    if ptr.is_null() || len == 0 {
        return Err("PNG 数据为空".to_string());
    }
    let bytes = unsafe { std::slice::from_raw_parts(ptr, len) }.to_vec();
    if png_size(&bytes).is_none() {
        return Err("原生快照返回的数据不是有效 PNG".to_string());
    }
    Ok(bytes)
}

#[cfg(not(target_os = "macos"))]
pub fn snapshot_png(_page: &tauri::Webview<tauri::Wry>) -> Result<Vec<u8>, String> {
    Err("当前平台未实现 WKWebView 原生快照；本 change 首轮仅 macOS 验证，不以整屏截图或 DOM 重绘替代".to_string())
}

// ── 纯逻辑测试（无需 GUI）───────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn meta(id: &str) -> CaptureMeta {
        CaptureMeta {
            schema: SCHEMA.to_string(),
            id: id.to_string(),
            status: "ok".to_string(),
            page: CapturePage {
                label: "page".to_string(),
                url: "http://127.0.0.1:1/".to_string(),
                generation: 0,
            },
            started_at_unix_ms: 1,
            finished_at_unix_ms: 2,
            viewport: CapturedViewport {
                configured: None,
                bounds: (800.0, 600.0),
                measured: None,
                fully_visible: true,
            },
            image: CaptureImage {
                file: format!("captures/{id}.png"),
                format: "png".to_string(),
                width_px: 800,
                height_px: 600,
                bytes: 3,
            },
            conditions: CaptureConditions {
                engine: "WKWebView".to_string(),
                method: "takeSnapshot".to_string(),
                tool_frame_hidden: true,
                scroll: "未取得".to_string(),
                missing: vec![],
            },
            data_root: ".layress".to_string(),
        }
    }

    #[test]
    fn 新请求替代旧请求后旧结果不得写盘也不得替换旧图() {
        let mut s = CaptureState::default();
        let old = s.begin();
        let current = s.begin();
        assert!(!s.is_current(&old));
        assert!(!s.finish(&old, meta("old")));
        assert!(s.last.is_none(), "失效结果不得成为最近一次成功");
        assert!(s.finish(&current, meta("new")));
        assert_eq!(s.last.as_ref().map(|m| m.id.as_str()), Some("new"));
    }

    #[test]
    fn 导航使在途采集失效且旧图保留() {
        let mut s = CaptureState::default();
        let first = s.begin();
        assert!(s.finish(&first, meta("first")));
        let in_flight = s.begin();
        assert_eq!(s.invalidate(), Some(in_flight.id));
        assert!(!s.is_current(&in_flight));
        assert!(!s.finish(&in_flight, meta("stale")));
        assert_eq!(s.last.as_ref().map(|m| m.id.as_str()), Some("first"));
    }

    #[test]
    fn 明确失败不替换旧现场() {
        let mut s = CaptureState::default();
        let ok = s.begin();
        assert!(s.finish(&ok, meta("ok")));
        let failing = s.begin();
        assert!(s.fail(&failing));
        assert!(s.active.is_none());
        assert_eq!(s.last.as_ref().map(|m| m.id.as_str()), Some("ok"));
    }

    #[test]
    fn 预览退出不改变采集身份() {
        let mut s = CaptureState::default();
        let ok = s.begin();
        assert!(s.finish(&ok, meta("ok")));
        assert!(s.preview);
        s.exit_preview();
        assert!(!s.preview);
        assert_eq!(s.generation, 0);
        assert_eq!(s.last.as_ref().map(|m| m.id.as_str()), Some("ok"));
    }

    /// 哨兵脚本的键名与 MeasuredViewport 字段漂移时，元数据会静默写成「未取得」。
    /// 这条断言把该类 bug 变成机器可拦的失败，而不是靠人看 JSON 发现。
    #[test]
    fn 标题哨兵键名与结构字段一致() {
        let script = crate::MEASURE_SCRIPT;
        for key in ["w:", "h:", "dpr:", "scroll_x:", "scroll_y:", "session:"] {
            assert!(script.contains(key), "哨兵脚本缺少字段 {key}");
        }
        let raw = r#"{"w":1280,"h":704,"dpr":2,"scroll_x":0,"scroll_y":0,"session":"s1"}"#;
        let m: MeasuredViewport = serde_json::from_str(raw).expect("哨兵 JSON 必须能反序列化");
        assert_eq!(m.scroll_y, Some(0.0));
        assert_eq!(m.session.as_deref(), Some("s1"));
    }

    #[test]
    fn png尺寸与base64编码可校验() {
        let mut png = b"\x89PNG\r\n\x1a\n".to_vec();
        png.extend_from_slice(&[0u8; 8]);
        png.extend_from_slice(&640u32.to_be_bytes());
        png.extend_from_slice(&480u32.to_be_bytes());
        assert_eq!(png_size(&png), Some((640, 480)));
        assert_eq!(png_size(b"not png"), None);
        assert_eq!(base64_encode(b"f"), "Zg==");
        assert_eq!(base64_encode(b"fo"), "Zm8=");
        assert_eq!(base64_encode(b"foo"), "Zm9v");
        assert_eq!(base64_encode(b"foobar"), "Zm9vYmFy");
    }
}
