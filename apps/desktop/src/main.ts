/**
 * 可信工具界面：导航、视口、采集与原地静态预览（add-tauri-page-host 任务 2.x–3.x）。
 * 本层持有 Tauri 能力；被测页面（"page" 标签）无任何命令权限。
 *
 * 3.x 口径：矩形只存原图像素坐标；显示缩放只在渲染时应用；
 * 失败 / 失效不显示旧图为本次成功；返回页面不销毁或重载 WebView。
 */

import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import {
  coordSelfTest,
  cssRectToImage,
  displayScale,
  rectFromDrag,
  toDisplayRect,
  type DisplayMode,
  type ImageRect,
  type Point,
} from "./coords";

interface MeasuredViewport {
  w: number;
  h: number;
  dpr: number;
  scroll_x: number | null;
  scroll_y: number | null;
  session: string | null;
}

interface ViewportData {
  configured: [number, number] | null;
  bounds: [number, number];
  measured: MeasuredViewport | null;
  fully_visible: boolean;
}

interface CaptureMeta {
  schema: string;
  id: string;
  status: string;
  page: { label: string; url: string; generation: number };
  started_at_unix_ms: number;
  finished_at_unix_ms: number;
  viewport: {
    configured: [number, number] | null;
    bounds: [number, number];
    measured: MeasuredViewport | null;
    fully_visible: boolean;
  };
  image: { file: string; format: string; width_px: number; height_px: number; bytes: number };
  conditions: {
    engine: string;
    method: string;
    tool_frame_hidden: boolean;
    scroll: string;
    missing: string[];
  };
  data_root: string;
}

interface CaptureOutcome {
  meta: CaptureMeta;
  png_base64: string;
  data_root: string;
  data_root_source: string;
}

interface PageStateInfo {
  configured: [number, number] | null;
  measured: MeasuredViewport | null;
  url: string | null;
  sampleBase: string | null;
  toolbarH: number;
  pageOpen: boolean;
  dataRoot: string | null;
  dataRootSource: string | null;
  dataRootWritable: boolean;
  dataRootError: string | null;
  inPreview: boolean;
  lastCapture: CaptureMeta | null;
}

interface PageEventPayload {
  kind: string;
  data?: unknown;
}

interface RootInfo {
  path: string;
  source: string;
  writable: boolean;
  error: string | null;
}

/** 样例页里的固定参照块（CSS 像素）：截图上位置已知，用于两种展示比例下核对落点。 */
const ANCHOR_CSS: ImageRect = { x: 24, y: 24, w: 160, h: 60 };

function assertElement<T extends HTMLElement>(id: string): T {
  const el = document.getElementById(id);
  if (!el) {
    throw new Error(`缺少元素 #${id}`);
  }
  return el as T;
}

const urlInput = assertElement<HTMLInputElement>("url-input");
const statusLine = assertElement<HTMLDivElement>("status-line");
const viewportLine = assertElement<HTMLDivElement>("viewport-line");
const requestLine = assertElement<HTMLDivElement>("request-line");
const widthInput = assertElement<HTMLInputElement>("width-input");
const heightInput = assertElement<HTMLInputElement>("height-input");
const bar = assertElement<HTMLDivElement>("bar");
const preview = assertElement<HTMLDivElement>("preview");
const previewStage = assertElement<HTMLDivElement>("preview-stage");
const previewCanvas = assertElement<HTMLDivElement>("preview-canvas");
const previewImg = assertElement<HTMLImageElement>("preview-img");
const rectTemp = assertElement<HTMLDivElement>("rect-temp");
const rectAnchor = assertElement<HTMLDivElement>("rect-anchor");
const previewCheck = assertElement<HTMLSpanElement>("preview-check");
const captureMetaLine = assertElement<HTMLDivElement>("capture-meta");
const coordInfo = assertElement<HTMLDivElement>("coord-info");
const dataRootLine = assertElement<HTMLDivElement>("data-root-line");
const btnModeOriginal = assertElement<HTMLButtonElement>("btn-mode-original");
const btnModeFit = assertElement<HTMLButtonElement>("btn-mode-fit");
// 开发期新增元素一律可选获取：HTML 与 JS 不同步（Vite HMR、旧窗口未重载）时，
// 顶层 assertElement 抛异常会让整个工具界面脚本停摆，按钮和状态行全部失效。
const slowCapture = document.getElementById("slow-capture") as HTMLInputElement | null;
const slowWrap = document.getElementById("slow-wrap") as HTMLSpanElement | null;
const captureToast = document.getElementById("capture-toast") as HTMLDivElement | null;

let currentMeta: CaptureMeta | null = null;
let lastCapture: CaptureMeta | null = null;
let lastRoot: RootInfo | null = null;
let mode: DisplayMode = "fit";
let scale = 1;
let imageW = 0;
let imageH = 0;
let tempRect: ImageRect | null = null;
let anchorRect: ImageRect | null = null;
let measuredSession: string | null = null;
let sessionAtCapture: string | null = null;
let captureSeq = 0;
let dragging = false;
let dragStart: Point | null = null;

function fmtViewport(v: ViewportData): string {
  const cfg = v.configured ? `${v.configured[0]}×${v.configured[1]}` : "跟随窗口";
  const m = v.measured
    ? `页面实测 ${v.measured.w}×${v.measured.h} @DPR ${v.measured.dpr}`
    : "页面实测：待样例上报";
  const visible = v.fully_visible ? "窗口内完整显示" : "⚠ 窗口容纳不下（视口保持不变）";
  return `设定 ${cfg} · 边界 ${Math.round(v.bounds[0])}×${Math.round(v.bounds[1])} · ${m} · ${visible}`;
}

function fmtTime(ms: number): string {
  return new Date(ms).toLocaleString("zh-CN", { hour12: false });
}

function clamp(v: number, min: number, max: number): number {
  return Math.max(min, Math.min(max, v));
}

function eventData<T>(p: PageEventPayload): T | null {
  return p.data === undefined || p.data === null ? null : (p.data as T);
}

type ToastLevel = "info" | "ok" | "error";

/** 采集结果常驻条：只由采集阶段更新，不被导航 / 加载 / 视口事件覆盖，也不被第三行挤没。 */
function showCaptureToast(text: string, level: ToastLevel): void {
  if (!captureToast) return;
  captureToast.textContent = text;
  captureToast.title = text; // 单行省略时悬停可看完整失败原因与数据根路径
  captureToast.className = level === "info" ? "show" : `show ${level}`;
}

// ── 导航与视口（任务 2.x）────────────────────────────────────────

async function navigate(): Promise<void> {
  const raw = urlInput.value;
  statusLine.textContent = `导航请求：${raw} …`;
  try {
    await invoke("page_navigate", { raw });
    // 成功后的状态与地址由 Rust 事件回报，这里不做乐观展示
  } catch (e) {
    statusLine.textContent = `✗ 已拒绝：${String(e)}`;
  }
}

async function refresh(): Promise<void> {
  try {
    await invoke("page_refresh");
    statusLine.textContent = "刷新当前页面 …";
  } catch (e) {
    statusLine.textContent = `✗ 刷新失败：${String(e)}`;
  }
}

async function applySize(): Promise<void> {
  const w = Number(widthInput.value);
  const h = Number(heightInput.value);
  if (!Number.isFinite(w) || !Number.isFinite(h)) {
    statusLine.textContent = "✗ 宽高必须是数字";
    return;
  }
  try {
    await invoke("page_set_size", { width: w, height: h });
    statusLine.textContent = `视口设定请求已提交：${w}×${h}`;
  } catch (e) {
    statusLine.textContent = `✗ ${String(e)}`;
  }
}

async function openSample(): Promise<void> {
  try {
    const base = await invoke<string>("sample_url");
    urlInput.value = `${base}/`;
    await navigate();
  } catch (e) {
    statusLine.textContent = `✗ 样例服务不可用：${String(e)}`;
  }
}

// ── 采集与静态预览（任务 3.x）───────────────────────────────────

function renderDataRoot(): void {
  if (!lastRoot) return;
  const state = lastRoot.writable ? "可写" : `不可写：${lastRoot.error ?? "原因未取得"}`;
  bar.title = `数据根：${lastRoot.path}（来源：${lastRoot.source}，${state}）`;
  if (!currentMeta) {
    dataRootLine.textContent = `数据根：${lastRoot.path}（来源：${lastRoot.source}，${state}）`;
  }
}

function hidePreview(): void {
  preview.classList.remove("show");
  previewImg.removeAttribute("src");
  currentMeta = null;
  tempRect = null;
  anchorRect = null;
  rectTemp.classList.remove("show");
  rectAnchor.classList.remove("show");
}

function renderRects(): void {
  const pairs: Array<[HTMLDivElement, ImageRect | null]> = [
    [rectAnchor, anchorRect],
    [rectTemp, tempRect],
  ];
  for (const [el, rect] of pairs) {
    if (!rect || rect.w <= 0 || rect.h <= 0) {
      el.classList.remove("show");
      continue;
    }
    const d = toDisplayRect(rect, scale);
    el.style.left = `${d.left}px`;
    el.style.top = `${d.top}px`;
    el.style.width = `${d.width}px`;
    el.style.height = `${d.height}px`;
    el.classList.add("show");
  }
}

function renderCoordInfo(): void {
  const r = tempRect;
  if (r) {
    const d = toDisplayRect(r, scale);
    coordInfo.textContent =
      `展示缩放 ${scale.toFixed(3)} · 原图矩形 (${r.x}, ${r.y}, ${r.w}×${r.h}) · ` +
      `显示位置 (${d.left.toFixed(1)}, ${d.top.toFixed(1)}, ${d.width.toFixed(1)}×${d.height.toFixed(1)}) · ` +
      `切换模式后原图坐标应保持相同`;
  } else {
    coordInfo.textContent = `展示缩放 ${scale.toFixed(3)} · 在画面上拖出一个临时矩形（坐标以原图像素保存）`;
  }
}

function computeScaleAndRender(): void {
  const areaW = previewStage.clientWidth - 24;
  const areaH = previewStage.clientHeight - 24;
  scale = displayScale(mode, imageW, imageH, areaW, areaH);
  previewCanvas.style.width = `${Math.round(imageW * scale)}px`;
  previewCanvas.style.height = `${Math.round(imageH * scale)}px`;
  btnModeOriginal.classList.toggle("active", mode === "original");
  btnModeFit.classList.toggle("active", mode === "fit");
  renderRects();
  renderCoordInfo();
}

function renderMeta(outcome: CaptureOutcome): void {
  const m = outcome.meta;
  const vp = m.viewport.configured
    ? `${m.viewport.configured[0]}×${m.viewport.configured[1]}（设定）`
    : `跟随窗口 ${Math.round(m.viewport.bounds[0])}×${Math.round(m.viewport.bounds[1])}`;
  captureMetaLine.textContent =
    `采集 ${m.id} · ${m.schema} · ${m.page.url} · ` +
    `开始 ${fmtTime(m.started_at_unix_ms)} → 完成 ${fmtTime(m.finished_at_unix_ms)} · ` +
    `视口 ${vp} · 图片 ${m.image.width_px}×${m.image.height_px} px / ${m.image.bytes} B · ${m.conditions.engine}`;
  dataRootLine.textContent =
    `数据根：${outcome.data_root}（来源：${outcome.data_root_source}） · ` +
    `产物 ${m.image.file} 与 ${m.id}.json · 滚动：${m.conditions.scroll} · ` +
    `缺失条件：${m.conditions.missing.join("；")}`;
}

function showPreview(outcome: CaptureOutcome): void {
  currentMeta = outcome.meta;
  lastCapture = outcome.meta;
  imageW = outcome.meta.image.width_px;
  imageH = outcome.meta.image.height_px;
  tempRect = null;
  anchorRect = null;
  sessionAtCapture = measuredSession;
  previewImg.src = `data:image/png;base64,${outcome.png_base64}`;
  preview.classList.add("show");
  renderMeta(outcome);
  computeScaleAndRender();
}

async function capture(): Promise<void> {
  captureSeq += 1;
  const seq = captureSeq;
  hidePreview();
  statusLine.textContent = "采集请求已提交 …";
  showCaptureToast("采集请求已提交 …", "info");
  // 慢采开关只在开发构建生效：留出人工可操作的窗口，验证竞态失效（E06 运行级负例）。
  const delayMs = slowCapture?.checked ? 5000 : 0;
  try {
    const outcome = await invoke<CaptureOutcome>("capture_start", { delayMs });
    if (seq !== captureSeq) {
      // 旧请求被新采集替代：不改写当前画面，但结果必须留痕，不能静默忽略。
      showCaptureToast(
        `旧请求 ${outcome.meta.id} 已完成但被新采集替代，未作为当前结果显示`,
        "info",
      );
      return;
    }
    showPreview(outcome);
    statusLine.textContent = `✓ 采集完成：${outcome.meta.id}`;
    showCaptureToast(
      `✓ 采集完成 ${outcome.meta.id} · ${outcome.meta.image.width_px}×${outcome.meta.image.height_px} px · 产物 ${outcome.meta.image.file}`,
      "ok",
    );
  } catch (e) {
    hidePreview();
    const detail = String(e);
    const old = lastCapture ? `；上次成功 ${lastCapture.id} 仅作旧现场，不代表本次成功` : "";
    const root = lastRoot ? `；可到 ${lastRoot.path}/captures/ 核对本次没有新文件` : "";
    const stale = seq !== captureSeq ? "（旧请求，已被新采集替代）" : "";
    const text = `✗ 本次采集失败${stale}：${detail}${old}${root}`;
    // 常驻条先写，保证即使状态行被后续事件覆盖、或旧请求被忽略，失败仍然可见。
    showCaptureToast(text, "error");
    if (seq !== captureSeq) return;
    statusLine.textContent = text;
  }
}

async function cancelCapture(): Promise<void> {
  try {
    const ok = await invoke<boolean>("capture_cancel");
    statusLine.textContent = ok ? "已请求取消当前采集" : "当前没有在途采集";
  } catch (e) {
    statusLine.textContent = `✗ 取消失败：${String(e)}`;
  }
}

async function backToPage(): Promise<void> {
  const before = sessionAtCapture;
  hidePreview();
  try {
    await invoke("preview_exit");
  } catch (e) {
    statusLine.textContent = `✗ 返回页面失败：${String(e)}`;
    return;
  }
  // Rust 会触发一次标题哨兵回报；会话标识只是机器观测，输入 / 弹窗仍需人工复核。
  window.setTimeout(() => {
    if (before && measuredSession && before !== measuredSession) {
      statusLine.textContent = "⚠ 返回后页面会话标识变化：页面可能被重载，请人工核对输入与弹窗";
    } else if (before && measuredSession) {
      statusLine.textContent = "已返回原 WebView（会话标识未变化）；输入与弹窗请人工复核";
    } else {
      statusLine.textContent = "已返回原 WebView；会话标识未取得，输入与弹窗请人工复核";
    }
  }, 400);
}

function alignAnchor(): void {
  if (!currentMeta) return;
  const cssW = currentMeta.viewport.measured?.w;
  if (!cssW) {
    coordInfo.textContent = "⚠ 页面视口未上报，无法把参照块换算到原图坐标（如实标注缺失）";
    return;
  }
  anchorRect = cssRectToImage(ANCHOR_CSS, cssW, imageW);
  tempRect = anchorRect ? { ...anchorRect } : null;
  renderRects();
  renderCoordInfo();
}

function setMode(next: DisplayMode): void {
  mode = next;
  computeScaleAndRender();
}

// ── 初始化与事件 ────────────────────────────────────────────────

async function loadInitialState(): Promise<void> {
  try {
    const state = await invoke<PageStateInfo>("page_state");
    if (state.url) urlInput.value = state.url;
    if (state.configured) {
      widthInput.value = String(state.configured[0]);
      heightInput.value = String(state.configured[1]);
    }
    measuredSession = state.measured?.session ?? null;
    lastCapture = state.lastCapture;
    lastRoot = {
      path: state.dataRoot ?? "未解析",
      source: state.dataRootSource ?? "未解析",
      writable: state.dataRootWritable,
      error: state.dataRootError,
    };
    renderDataRoot();
    // 工具层重载后若 Rust 仍处于预览态，先恢复布局，避免卡在静态画面且没有图片数据。
    if (state.inPreview) {
      await invoke("preview_exit");
      hidePreview();
    }
    statusLine.textContent = state.pageOpen
      ? `已有被测页面会话${lastCapture ? `；旧现场 ${lastCapture.id}（非本次采集结果）` : ""}`
      : "输入地址或点击「打开样例页」开始";
  } catch (e) {
    statusLine.textContent = `✗ 状态读取失败：${String(e)}`;
  }
}

function bindUi(): void {
  assertElement<HTMLButtonElement>("btn-go").addEventListener("click", () => void navigate());
  urlInput.addEventListener("keydown", (ev) => {
    if (ev.key === "Enter") void navigate();
  });
  assertElement<HTMLButtonElement>("btn-refresh").addEventListener("click", () => void refresh());
  assertElement<HTMLButtonElement>("btn-sample").addEventListener("click", () => void openSample());
  assertElement<HTMLButtonElement>("btn-size").addEventListener("click", () => void applySize());
  assertElement<HTMLButtonElement>("btn-capture").addEventListener("click", () => void capture());
  assertElement<HTMLButtonElement>("btn-cancel").addEventListener("click", () => void cancelCapture());
  assertElement<HTMLButtonElement>("btn-recapture").addEventListener("click", () => void capture());
  assertElement<HTMLButtonElement>("btn-preview-back").addEventListener("click", () => void backToPage());
  btnModeOriginal.addEventListener("click", () => setMode("original"));
  btnModeFit.addEventListener("click", () => setMode("fit"));
  assertElement<HTMLButtonElement>("btn-anchor").addEventListener("click", alignAnchor);

  previewCanvas.addEventListener("mousedown", (ev) => {
    if (!currentMeta) return;
    const box = previewCanvas.getBoundingClientRect();
    dragging = true;
    dragStart = { x: clamp(ev.clientX - box.left, 0, box.width), y: clamp(ev.clientY - box.top, 0, box.height) };
    tempRect = null;
    renderRects();
    renderCoordInfo();
  });
  window.addEventListener("mousemove", (ev) => {
    if (!dragging || !dragStart) return;
    const box = previewCanvas.getBoundingClientRect();
    const end: Point = {
      x: clamp(ev.clientX - box.left, 0, box.width),
      y: clamp(ev.clientY - box.top, 0, box.height),
    };
    tempRect = rectFromDrag(dragStart, end, scale);
    renderRects();
    renderCoordInfo();
  });
  window.addEventListener("mouseup", () => {
    dragging = false;
    dragStart = null;
  });
  window.addEventListener("resize", () => {
    if (currentMeta) computeScaleAndRender();
  });
  previewImg.addEventListener("load", () => {
    if (currentMeta) computeScaleAndRender();
  });
}

async function subscribeEvents(): Promise<void> {
  await listen<PageEventPayload>("page://event", (ev) => {
    const p = ev.payload;
    switch (p.kind) {
      case "Url":
        urlInput.value = String(p.data ?? "");
        break;
      case "Status": {
        const s = eventData<{ phase: string; url: string }>(p);
        if (s) {
          statusLine.textContent = s.phase === "loading" ? `加载中：${s.url}` : `✓ 已加载：${s.url}`;
        }
        break;
      }
      case "Viewport": {
        const v = eventData<ViewportData>(p);
        if (v) {
          viewportLine.textContent = fmtViewport(v);
          measuredSession = v.measured?.session ?? null;
        }
        break;
      }
      case "Request": {
        const r = eventData<{ kind: string; url: string }>(p);
        if (r) requestLine.textContent = `已拒绝 ${r.kind}：${r.url || "(无地址)"}`;
        break;
      }
      case "Denied": {
        const d = eventData<{ reason: string }>(p);
        if (d) statusLine.textContent = `✗ ${d.reason}`;
        break;
      }
      case "Capture": {
        const c = eventData<{ phase: string; id: string; message: string }>(p);
        if (!c) break;
        const failed = c.phase === "failed" || c.phase === "invalidated" || c.phase === "cancelled";
        const level: ToastLevel = failed ? "error" : c.phase === "completed" ? "ok" : "info";
        // 每个采集阶段都进常驻条：started 的「慢采已开启 …ms」可直接证明参数生效。
        showCaptureToast(`[${c.phase}] ${c.message || c.id}`, level);
        if (failed) {
          statusLine.textContent = `✗ ${c.message}`;
        } else if (c.phase === "started") {
          statusLine.textContent = `采集 ${c.id}：${c.message}`;
        }
        break;
      }
      case "DataRoot": {
        const r = eventData<RootInfo>(p);
        if (r) {
          lastRoot = r;
          renderDataRoot();
        }
        break;
      }
      case "Preview":
        // 布局由 Rust 管理、界面显隐由命令显式控制；此处不自动切换，避免重新采集时抖动。
        break;
    }
  });
}

async function main(): Promise<void> {
  // 慢采是开发期复验开关，正式构建不显示；Rust 侧也只在 debug 构建接受延迟。
  if (!import.meta.env.DEV && slowWrap) slowWrap.style.display = "none";
  const check = coordSelfTest();
  previewCheck.textContent = check.passed ? `坐标自检 ✓ ${check.detail}` : `坐标自检 ✗ ${check.detail}`;
  previewCheck.classList.toggle("bad", !check.passed);
  bindUi();
  await subscribeEvents();
  await loadInitialState();
  viewportLine.textContent = "视口：尚未打开页面";
  // 旧窗口 + 刚更新的脚本会出现「HTML 还是旧版」：此时慢采开关与结果条都不存在，
  // 与其让用户看到空白状态行，不如直接给出重启指引。
  if (!captureToast || !slowCapture || !slowWrap) {
    statusLine.textContent = "⚠ 工具界面 HTML 与脚本不同步：请 Ctrl+C 后重新运行 pnpm tauri dev（完整重启）";
  }
}

void main();
