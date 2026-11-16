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

/**
 * 工具层脚本版本：每次改前端必须与 index.html 的 #build-tag[data-build] 同步递增，
 * `check-build.mjs` 会断言两处一致。用途是在 GUI 上自证「窗口加载的是当前代码」——
 * CSP 挡掉 Vite HMR、旧窗口未重载时，人工复验会跑在旧 DOM 上（本仓已踩三次）。
 */
const BUILD_TAG = "v20261116-8";

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
  restoreFailure: RestoreFailureInfo | null;
}

/** 任务 3.1：页面打开失败的可重试状态（启动恢复 / 项目切换共用）。 */
interface RestoreFailureInfo {
  projectId: string;
  projectName: string;
  targetUrl: string;
  message: string;
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
/** 视口信号去重：状态栏是单条覆盖通道，只有「设定值 / 容纳状态」变化时才占用它。 */
let lastViewSignal = "";
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

type ToastLevel = "info" | "pending" | "ok" | "error";

/** 操作反馈 → 窗口最底部的独立状态栏 WebView（status.html，无能力、无脚本）。
 *  导航 / 加载 / 采集共用这一条常驻通道：工具层第三行在 64px CSS 视口下被裁，
 *  只写 statusLine 会让「已拒绝」在 GUI 上完全不可见（见 runtime-probe）。
 *  状态栏未就绪时静默降级，不改变操作结果。 */
function showStatusBar(text: string, level: ToastLevel): void {
  void invoke("status_bar_set", { text, level }).catch((e) => {
    // 只影响提示呈现，不改变操作结果；但不能静默：工具层第三行已被裁，
    // 至少留一条可抓取的机器痕迹，避免「用户说没出现」时无从分诊。
    console.error("[layress] 状态栏更新失败：", e);
  });
  statusLine.textContent = text; // 工具层状态行留一份兜底（第三行受 64px 视口限制，不依赖它）
}

// ── 任务 1.1：项目配置状态与设置面板 ─────────────────────────────

interface ConfigProjectView {
  id: string;
  name: string;
  startUrl: string;
  lastLoadedUrl: string | null;
  presetId: string;
  width: number;
  height: number;
}

interface ConfigStateView {
  revision: number;
  activeProjectId: string | null;
  projects: ConfigProjectView[];
  writable: boolean;
  dataRoot: string;
  error: { code: string; message: string } | null;
  /** 任务 3.1：活动项目已提交但页面打开失败时非空；可点「重试打开当前项目」。 */
  restoreFailure: RestoreFailureInfo | null;
}

// 新增 DOM 一律可选获取：旧窗口 HTML 与 Vite HMR 不同步时降级，不让整个脚本崩掉。
const settingsPanel = document.getElementById("settings");
const settingsState = document.getElementById("settings-state");
const settingsList = document.getElementById("settings-list");
const btnRebuild = document.getElementById("settings-rebuild") as HTMLButtonElement | null;
const btnRetry = document.getElementById("settings-retry") as HTMLButtonElement | null;
const settingsName = document.getElementById("settings-name") as HTMLInputElement | null;
const settingsUrl = document.getElementById("settings-url") as HTMLInputElement | null;
const settingsPreset = document.getElementById("settings-preset") as HTMLSelectElement | null;
const btnSettings = document.getElementById("btn-settings") as HTMLButtonElement | null;

/** 结构化错误：Rust 返回 { code, message }；普通 String 错误也兼容。 */
function commandErrorText(e: unknown): string {
  if (e && typeof e === "object") {
    const obj = e as { code?: unknown; message?: unknown };
    if (typeof obj.code === "string" && typeof obj.message === "string") {
      return `${obj.code}：${obj.message}`;
    }
    if (typeof obj.message === "string") return obj.message;
  }
  return String(e);
}

/** 只渲染 Rust 返回的「已保存状态」，不做乐观展示。 */
let lastConfigView: ConfigStateView | null = null;
/** 任务 2.1 编辑模式：非 null 时保存走 config_update_project，ID 不变。 */
let editingProjectId: string | null = null;

function shortId(id: string): string {
  const tail = id.split("-").pop() ?? id;
  return `#${tail}`;
}

function resetProjectForm(): void {
  editingProjectId = null;
  if (settingsName) settingsName.value = "";
  if (settingsUrl) settingsUrl.value = "";
  if (settingsPreset) settingsPreset.value = "desktop";
  const save = document.getElementById("settings-save");
  if (save) save.textContent = "新建项目并保存";
  document.getElementById("settings-new")?.setAttribute("hidden", "");
  document.getElementById("settings-viewport")?.setAttribute("hidden", "");
  document.getElementById("settings-last-loaded")?.setAttribute("hidden", "");
}

function enterEditMode(p: ConfigStateView["projects"][number]): void {
  editingProjectId = p.id;
  if (settingsName) settingsName.value = p.name;
  if (settingsUrl) settingsUrl.value = p.startUrl;
  if (settingsPreset) settingsPreset.value = p.presetId;
  const vpW = document.getElementById("settings-vp-w") as HTMLInputElement | null;
  const vpH = document.getElementById("settings-vp-h") as HTMLInputElement | null;
  if (vpW) vpW.value = String(p.width);
  if (vpH) vpH.value = String(p.height);
  const save = document.getElementById("settings-save");
  if (save) save.textContent = `保存修改（${shortId(p.id)}）`;
  document.getElementById("settings-new")?.removeAttribute("hidden");
  document.getElementById("settings-viewport")?.removeAttribute("hidden");
  // 任务 2.3：最近地址事实记忆（只读展示）；显式打开起始地址后由完成加载自动更新
  const lastRow = document.getElementById("settings-last-loaded");
  const lastValue = document.getElementById("settings-last-loaded-value");
  if (lastValue) {
    lastValue.textContent =
      p.lastLoadedUrl ?? "暂无（尚未在该项目下完成过主文档加载；临时浏览不记忆）";
  }
  lastRow?.removeAttribute("hidden");
  document.getElementById("settings-open-start")?.removeAttribute("hidden");
  showStatusBar(
    `正在编辑 ${p.name}（${shortId(p.id)}）；保存才提交，关闭面板即取消且不导航`,
    "info",
  );
}

function renderConfig(view: ConfigStateView): void {
  lastConfigView = view;
  if (btnSettings) {
    const active = view.projects.find((p) => p.id === view.activeProjectId);
    btnSettings.textContent = view.error
      ? "项目：配置错误"
      : `项目：${active ? active.name : "无"}`;
  }
  if (settingsState) {
    settingsState.textContent = view.error
      ? `配置不可用（${view.error.code}）：${view.error.message}`
      : `已保存 ${view.projects.length} 个项目 · revision ${view.revision} · ${view.writable ? "可写" : "只读"}`;
  }
  if (settingsList) {
    if (!view.projects.length) {
      settingsList.textContent =
        "尚无项目记录；临时浏览不会自动记忆。同名项目允许，以稳定 ID 区分（条目内 #序号）。";
    } else {
      settingsList.textContent = "";
      for (const p of view.projects) {
        const item = document.createElement("div");
        item.className = "project-item" + (p.id === view.activeProjectId ? " active" : "");
        item.dataset.projectId = p.id;
        item.title = `完整 ID：${p.id}；点击文字进编辑，点「切换」重新打开该项目`;
        const mark = p.id === view.activeProjectId ? "● " : "○ ";
        const label = document.createElement("span");
        label.textContent =
          `${mark}${p.name}（${shortId(p.id)} · ${p.startUrl} · ${p.presetId} ${p.width}×${p.height}）`;
        item.appendChild(label);
        // 任务 3.1：显式切换动作。两段确认由按钮事件承担；点文字仍是编辑入口
        const switchBtn = document.createElement("button");
        switchBtn.type = "button";
        switchBtn.className = "switch-btn";
        switchBtn.dataset.switchId = p.id;
        switchBtn.textContent = p.id === view.activeProjectId ? "重开" : "切换";
        switchBtn.title =
          p.id === view.activeProjectId
            ? "重开当前项目页面（页面已打开时不重载）"
            : "切换到此项目：先持久提交，再销毁旧页、按该项目视口重新打开（未保存输入不恢复）";
        item.appendChild(switchBtn);
        settingsList.appendChild(item);
      }
    }
  }
  // 任务 3.1：有活动项目且无配置错误时提供「重试打开当前项目」（启动恢复 / 切换失败后）
  if (btnRetry) {
    btnRetry.hidden = view.error != null || view.activeProjectId == null;
  }
  // 任务 1.5：只有「文件不可用」三类错误才提供备份重建；两段确认在按钮事件里
  const rebuildable =
    view.error?.code === "CONFIG_READ_FAILED" ||
    view.error?.code === "CONFIG_CORRUPT" ||
    view.error?.code === "CONFIG_VERSION_UNSUPPORTED";
  if (btnRebuild) {
    btnRebuild.hidden = !rebuildable;
    if (!rebuildable) disarmRebuild();
  }
}

/** 重建按钮的两段确认状态：第一次点击只进入待确认，再点才真正调用。 */
let rebuildArmed = false;
let rebuildDisarmTimer: ReturnType<typeof setTimeout> | null = null;

function disarmRebuild(): void {
  rebuildArmed = false;
  if (rebuildDisarmTimer) {
    clearTimeout(rebuildDisarmTimer);
    rebuildDisarmTimer = null;
  }
  if (btnRebuild) {
    btnRebuild.textContent = "备份并重建空配置";
    btnRebuild.classList.remove("danger");
  }
}

async function rebuildProject(): Promise<void> {
  if (!btnRebuild) return;
  if (!rebuildArmed) {
    rebuildArmed = true;
    btnRebuild.textContent = "⚠ 会清空项目记忆（截图保留）——再点一次确认";
    btnRebuild.classList.add("danger");
    rebuildDisarmTimer = setTimeout(disarmRebuild, 6000);
    showStatusBar("再次点击「备份并重建空配置」确认；取消请等待或关闭面板", "info");
    return;
  }
  disarmRebuild();
  showStatusBar("正在备份并重建项目配置 …", "pending");
  try {
    const view = await invoke<ConfigStateView>("config_rebuild");
    renderConfig(view);
    showStatusBar(`✓ 已备份原配置并重建为空配置（revision ${view.revision}）`, "ok");
  } catch (e) {
    showStatusBar(`✗ 重建失败：${commandErrorText(e)}`, "error");
    await refreshConfig();
  }
}

async function refreshConfig(): Promise<void> {
  try {
    renderConfig(await invoke<ConfigStateView>("config_state"));
  } catch (e) {
    showStatusBar(`✗ 读取项目配置失败：${commandErrorText(e)}`, "error");
  }
}

// ── 任务 3.1：跨项目切换（可取消）与重试打开 ──────────────────────

/** 切换按钮的两段确认状态：第一次点击只武装，再点才真正切换；超时 / 改点其他项自动取消。 */
let switchArmedId: string | null = null;
let switchDisarmTimer: ReturnType<typeof setTimeout> | null = null;

function disarmSwitch(): void {
  switchArmedId = null;
  if (switchDisarmTimer) {
    clearTimeout(switchDisarmTimer);
    switchDisarmTimer = null;
  }
  const btn = document.querySelector<HTMLButtonElement>(".switch-btn.armed");
  if (btn) {
    btn.classList.remove("armed");
    btn.textContent = btn.dataset.switchId === lastConfigView?.activeProjectId ? "重开" : "切换";
  }
}

/** 切换到指定项目 / 重开当前项目。跨项目切换先两段确认（可取消）：
 * 明确提示会话不恢复；确认后先持久提交再重建被测页（顺序在 Rust 命令内）。
 * 重选当前项目：页面已打开 ⇒ 不重载、保持原会话（Rust 端同判 no-op 双保险）；
 * 页面未打开（此前恢复 / 切换失败）⇒ 走命令真打开。 */
async function activateProject(id: string): Promise<void> {
  const view = lastConfigView;
  const p = view?.projects.find((x) => x.id === id);
  if (!view || !p) return;
  const isSame = p.id === view.activeProjectId;
  if (isSame) {
    let pageOpen = false;
    try {
      pageOpen = !!(await invoke<PageStateInfo>("page_state")).pageOpen;
    } catch {
      // 状态取不到时按「未打开」处理，走命令让 Rust 判定（有 no-op 保护）
    }
    if (pageOpen) {
      showStatusBar(`已是当前项目「${p.name}」：页面保持原会话，未重载`, "ok");
      return;
    }
    showStatusBar(`当前项目页面未打开，正在打开「${p.name}」 …`, "pending");
  } else if (switchArmedId !== id) {
    // 第一段：武装确认；改点其他项 / 超时自动取消
    disarmSwitch();
    switchArmedId = id;
    const btn = document.querySelector<HTMLButtonElement>(`[data-switch-id="${id}"]`);
    if (btn) {
      btn.classList.add("armed");
      btn.textContent = "确认切换";
    }
    switchDisarmTimer = setTimeout(disarmSwitch, 6000);
    showStatusBar(
      `⚠ 将重新打开项目「${p.name}」：当前页面未保存的输入和弹窗不会恢复。再点一次「确认切换」执行；不操作即取消`,
      "error",
    );
    return;
  }
  disarmSwitch();
  if (!isSame) {
    showStatusBar("正在切换项目：先持久提交，再重建被测页 …", "pending");
  }
  try {
    const next = await invoke<ConfigStateView>("config_activate_project", { id });
    renderConfig(next);
    if (next.restoreFailure) {
      // 活动项目已提交但页面打开失败：旧页已销毁，不冒充切换成功；保持面板开放重试
      showStatusBar(
        `✗ 已切换活动项目「${p.name}」，但页面打开失败：${next.restoreFailure.message}；可点「重试打开当前项目」`,
        "error",
      );
      return;
    }
    await closeSettings();
    showStatusBar(
      isSame
        ? `✓ 已打开项目「${p.name}」页面`
        : `✓ 已切换到项目「${p.name}」；旧页面会话已销毁，未保存输入不恢复`,
      "ok",
    );
  } catch (e) {
    // 持久提交失败：整体未开始切换，当前项目与页面保留（design §4）
    showStatusBar(`✗ 切换未执行（当前项目与页面保留）：${commandErrorText(e)}`, "error");
    await refreshConfig();
  }
}

/** 重试打开当前活动项目（启动恢复 / 切换后页面打开失败的显式重试）。 */
async function retryRestore(): Promise<void> {
  if (!lastConfigView?.activeProjectId) {
    showStatusBar("当前没有活动项目，无需恢复；先在下方选择或新建项目", "info");
    return;
  }
  showStatusBar("正在重试打开当前项目：先应用视口再导航 …", "pending");
  try {
    await invoke("config_retry_restore");
    await refreshConfig();
    showStatusBar("✓ 当前项目页面已打开（最近地址优先，为空回退起始地址）", "ok");
  } catch (e) {
    showStatusBar(`✗ 重试失败：${commandErrorText(e)}；项目配置保留，可再次重试`, "error");
    await refreshConfig();
  }
}

// ── 任务 2.2 视口编辑与应用 ──────────────────────────────────────

function editingProject(): ConfigStateView["projects"][number] | null {
  return lastConfigView?.projects.find((p) => p.id === editingProjectId) ?? null;
}

/** 配置保存成功后把视口真正套到被测页（复用既有 page_set_size）。 */
async function applyConfiguredSize(width: number, height: number, label: string): Promise<void> {
  try {
    await invoke("page_set_size", { width, height });
    showStatusBar(`✓ 视口已保存并应用：${label} → ${width}×${height}`, "ok");
  } catch (e) {
    showStatusBar(`✗ 视口已保存，但应用到被测页失败：${String(e)}`, "error");
  }
}

/** 表单里的视口相对已保存值是否有改动；有则随「保存修改」一并提交。 */
function viewportDirty(p: ConfigStateView["projects"][number]): boolean {
  const presetSel = settingsPreset?.value ?? "";
  const [width, height] = viewportInputs();
  return presetSel !== p.presetId || width !== p.width || height !== p.height;
}

function viewportInputs(): [number, number] {
  const w = document.getElementById("settings-vp-w") as HTMLInputElement | null;
  const h = document.getElementById("settings-vp-h") as HTMLInputElement | null;
  return [Number(w?.value), Number(h?.value)];
}

/** 提交表单当前视口：预设变化走预设分支，否则按宽高走自定义分支。 */
async function commitViewport(p: ConfigStateView["projects"][number]): Promise<void> {
  const presetSel = settingsPreset?.value ?? "";
  const [width, height] = viewportInputs();
  if (presetSel !== p.presetId) {
    await invoke<ConfigStateView>("config_set_viewport", { id: p.id, presetId: presetSel });
  } else {
    if (!Number.isInteger(width) || !Number.isInteger(height)) {
      throw new Error("视口宽高必须是整数（16–4096）");
    }
    await invoke<ConfigStateView>("config_set_viewport", {
      id: p.id,
      presetId: null,
      width,
      height,
    });
  }
}

/** 任务 2.3 / E04：显式打开起始地址——导航成功完成加载后才更新最近地址；
 * 仅编辑起始地址不会触发（不导航）。 */
async function openStartUrl(): Promise<void> {
  const p = editingProject();
  if (!p) return;
  showStatusBar(`正在打开起始地址：${p.startUrl} …`, "pending");
  try {
    await invoke("page_navigate", { raw: p.startUrl });
    // 成功与完成加载由 PageEvent 回报；最近地址由 loaded 回调自动记忆
  } catch (e) {
    showStatusBar(`✗ 已拒绝：${commandErrorText(e)}`, "error");
  }
}

async function rotateViewport(): Promise<void> {  const p = editingProject();
  if (!p) return;
  showStatusBar("正在旋转视口 …", "pending");
  try {
    const view = await invoke<ConfigStateView>("config_rotate_viewport", { id: p.id });
    renderConfig(view);
    const np = editingProject();
    syncViewportInputs(np);
    if (np) await applyConfiguredSize(np.width, np.height, `旋转（保留 ${np.presetId}）`);
  } catch (e) {
    showStatusBar(`✗ 已拒绝：${commandErrorText(e)}`, "error");
  }
}

function syncViewportInputs(p: ConfigStateView["projects"][number] | null): void {
  const vpW = document.getElementById("settings-vp-w") as HTMLInputElement | null;
  const vpH = document.getElementById("settings-vp-h") as HTMLInputElement | null;
  if (p && vpW && vpH) {
    vpW.value = String(p.width);
    vpH.value = String(p.height);
  }
}

async function openSettings(): Promise<void> {
  try {
    const view = await invoke<ConfigStateView>("settings_enter");
    renderConfig(view);
    settingsPanel?.classList.add("show");
  } catch (e) {
    showStatusBar(`✗ 已拒绝：${commandErrorText(e)}`, "error");
  }
}

async function closeSettings(): Promise<void> {
  disarmRebuild();
  disarmSwitch();
  try {
    await invoke("settings_exit");
    settingsPanel?.classList.remove("show");
    await refreshConfig();
  } catch (e) {
    showStatusBar(`✗ 关闭设置失败：${commandErrorText(e)}`, "error");
  }
}

async function saveProject(): Promise<void> {
  const name = settingsName?.value ?? "";
  const startUrl = settingsUrl?.value ?? "";
  const presetId = settingsPreset?.value ?? "desktop";
  const editing = editingProjectId ? editingProject() : null;
  // 任务 2.2（用户 2026-11-16 定）：视口改动随「保存修改」一并提交，
  // 避免下意识只点保存而漏掉视口；非法宽高在提交前拦下，两个字段都不落盘。
  if (editing && viewportDirty(editing)) {
    const [width, height] = viewportInputs();
    if (!Number.isInteger(width) || !Number.isInteger(height)) {
      showStatusBar("✗ 视口宽高必须是整数（16–4096）；修正后 再保存", "error");
      return;
    }
  }
  showStatusBar("正在保存项目配置 …", "pending");
  try {
    let view: ConfigStateView;
    if (editingProjectId && editing) {
      // 任务 2.1：编辑既有项目——ID 不变，仅名称与起始地址；不导航、不覆盖最近地址
      view = await invoke<ConfigStateView>("config_update_project", {
        id: editingProjectId,
        name,
        startUrl,
      });
      // 视口有改动则随保存一并提交（同一「保存」语义，分命令执行）
      if (viewportDirty(editing)) {
        await commitViewport(editing);
        view = await invoke<ConfigStateView>("config_state");
      }
    } else {
      view = await invoke<ConfigStateView>("config_create_project", {
        name,
        // Tauri 2 把 Rust 命令的 snake_case 参数映射为前端 camelCase；写成 snake_case 键会报 missing required key。
        startUrl,
        presetId,
      });
    }
    renderConfig(view);
    const wasEditing = editing;
    resetProjectForm();
    showStatusBar(`✓ 项目已保存：${name.trim()} · revision ${view.revision}（当前页面不变，不导航）`, "ok");
    // 编辑保存且视口有改动：把新视口套到被测页；应用失败单独红条，不冒充成功
    if (wasEditing) {
      const np = lastConfigView?.projects.find((p) => p.id === wasEditing.id);
      if (np) await applyConfiguredSize(np.width, np.height, np.presetId);
    }
  } catch (e) {
    showStatusBar(`✗ 已拒绝：${commandErrorText(e)}`, "error");
  }
}

// ── 导航与视口（任务 2.x）────────────────────────────────────────

async function navigate(): Promise<void> {
  const raw = urlInput.value;
  showStatusBar(`正在校验地址：${raw} …`, "pending");
  try {
    await invoke("page_navigate", { raw });
    // 成功后的状态与地址由 Rust 事件回报，这里不做乐观展示
  } catch (e) {
    showStatusBar(`✗ 已拒绝：${String(e)}`, "error");
    // 预检失败不导航：地址栏必须恢复成 Rust 侧的真实页面地址，
    // 否则输入框残留无效地址，会被误读成「旧页面已到了新地址」。
    try {
      const state = await invoke<PageStateInfo>("page_state");
      urlInput.value = state.url ?? "";
    } catch {
      urlInput.value = "";
    }
  }
}

async function refresh(): Promise<void> {
  try {
    await invoke("page_refresh");
    showStatusBar("刷新当前页面 …", "pending");
  } catch (e) {
    showStatusBar(`✗ 刷新失败：${String(e)}`, "error");
  }
}

async function applySize(): Promise<void> {
  const w = Number(widthInput.value);
  const h = Number(heightInput.value);
  if (!Number.isFinite(w) || !Number.isFinite(h)) {
    showStatusBar("✗ 宽高必须是数字", "error");
    return;
  }
  try {
    await invoke("page_set_size", { width: w, height: h });
    // 成功反馈由 Viewport 事件统一给：page_set_size 内部先 emit_viewport，
    // 若这里再写一条绿条就会盖掉「窗口容纳不下」红条（单条覆盖通道的先后顺序坑）。
    // 实测值看被测页「实际视口与引擎」区与采集元数据，工具层第三行被裁，不作判据（I15）。
  } catch (e) {
    showStatusBar(`✗ 视口设定失败：${String(e)}`, "error");
  }
}

async function openSample(): Promise<void> {
  try {
    const base = await invoke<string>("sample_url");
    urlInput.value = `${base}/`;
    await navigate();
  } catch (e) {
    showStatusBar(`✗ 样例服务不可用：${String(e)}`, "error");
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
  showStatusBar("采集请求已提交 …", "pending");
  // 慢采开关只在开发构建生效：留出人工可操作的窗口，验证竞态失效（E06 运行级负例）。
  const delayMs = slowCapture?.checked ? 5000 : 0;
  try {
    const outcome = await invoke<CaptureOutcome>("capture_start", { delayMs });
    if (seq !== captureSeq) {
      // 旧请求被新采集替代：不改写当前画面，但结果必须留痕，不能静默忽略。
      showStatusBar(
        `旧请求 ${outcome.meta.id} 已完成但被新采集替代，未作为当前结果显示`,
        "info",
      );
      return;
    }
    showPreview(outcome);
    showStatusBar(
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
    // showStatusBar 内部已把同一文本兜底写进第三行，不在这里重复赋值（I15）。
    showStatusBar(text, "error");
  }
}

async function cancelCapture(): Promise<void> {
  try {
    const ok = await invoke<boolean>("capture_cancel");
    showStatusBar(ok ? "已请求取消当前采集" : "当前没有在途采集", "info");
  } catch (e) {
    showStatusBar(`✗ 取消失败：${String(e)}`, "error");
  }
}

async function backToPage(): Promise<void> {
  const before = sessionAtCapture;
  hidePreview();
  try {
    await invoke("preview_exit");
  } catch (e) {
    showStatusBar(`✗ 返回页面失败：${String(e)}`, "error");
    return;
  }
  // Rust 会触发一次标题哨兵回报；会话标识只是机器观测，输入 / 弹窗仍需人工复核。
  // E08 的「返回后是否重载」判据必须落在底部状态栏，不能只写被裁的第三行（I15）。
  window.setTimeout(() => {
    if (before && measuredSession && before !== measuredSession) {
      showStatusBar("⚠ 返回后页面会话标识变化：页面可能被重载，请人工核对输入与弹窗", "error");
    } else if (before && measuredSession) {
      showStatusBar("已返回原 WebView（会话标识未变化）；输入与弹窗请人工复核", "ok");
    } else {
      showStatusBar("已返回原 WebView；会话标识未取得，输入与弹窗请人工复核", "info");
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
    // 启动提示先写，数据根后写：状态栏是单条通道，最后一条必须是数据根，
    // 否则 4.1 的「启动可见」判据会被开场白冲掉（I13 / I15）。
    showStatusBar(
      state.pageOpen
        ? `已有被测页面会话${lastCapture ? `；旧现场 ${lastCapture.id}（非本次采集结果）` : ""}`
        : "输入地址或点击「打开样例页」开始",
      "info",
    );
    // 启动时事件可能早于订阅到达；这里按 page_state 再点亮一次数据根，保证 I13 的「启动可见」。
    // 任务 1.4：无根（解析失败，如相对路径被拒）也必须亮红条，且排在开场提示之后不被冲掉。
    if (state.dataRoot || state.dataRootError) {
      const detail = state.dataRoot
        ? `数据根：${state.dataRoot}（来源：${state.dataRootSource ?? "未解析"}，` +
          `${state.dataRootWritable ? "可写" : `不可写：${state.dataRootError ?? "原因未取得"}`}）`
        : `数据根不可用：${state.dataRootError ?? "原因未取得"}`;
      showStatusBar(detail, state.dataRootWritable ? "info" : "error");
    }
    // 工具层重载后若 Rust 仍处于预览态，先恢复布局，避免卡在静态画面且没有图片数据。
    if (state.inPreview) {
      await invoke("preview_exit");
      hidePreview();
    }
    // 任务 3.1：启动恢复失败（离线 / 目标不可达）时红条最后写，保证不被前面的提示冲掉；
    // 活动项目与配置已保留，重试入口在项目设置的「重试打开当前项目」。
    if (state.restoreFailure) {
      const f = state.restoreFailure;
      showStatusBar(
        `✗ 启动恢复失败：${f.targetUrl} —— ${f.message}；项目「${f.projectName}」配置已保留，打开项目设置可重试`,
        "error",
      );
    }
  } catch (e) {
    showStatusBar(`✗ 状态读取失败：${String(e)}`, "error");
  }
}

function bindUi(): void {
  assertElement<HTMLButtonElement>("btn-go").addEventListener("click", () => void navigate());
  urlInput.addEventListener("keydown", (ev) => {
    if (ev.key === "Enter") void navigate();
  });
  assertElement<HTMLButtonElement>("btn-refresh").addEventListener("click", () => void refresh());
  assertElement<HTMLButtonElement>("btn-sample").addEventListener("click", () => void openSample());
  btnSettings?.addEventListener("click", () => void openSettings());
  document.getElementById("settings-save")?.addEventListener("click", () => void saveProject());
  document.getElementById("settings-cancel")?.addEventListener("click", () => void closeSettings());
  btnRebuild?.addEventListener("click", () => void rebuildProject());
  btnRetry?.addEventListener("click", () => void retryRestore());
  // 任务 2.1：列表条目点选进编辑模式；「新建」退出编辑回到空表单（取消不提交）
  // 任务 3.1：点「切换 / 重开」按钮走显式切换（两段确认），点文字仍是编辑入口
  settingsList?.addEventListener("click", (ev) => {
    const switchBtn = (ev.target as HTMLElement).closest<HTMLElement>("[data-switch-id]");
    if (switchBtn?.dataset.switchId) {
      void activateProject(switchBtn.dataset.switchId);
      return;
    }
    const target = (ev.target as HTMLElement).closest<HTMLElement>("[data-project-id]");
    const p = lastConfigView?.projects.find((x) => x.id === target?.dataset.projectId);
    if (p) {
      disarmSwitch();
      enterEditMode(p);
    }
  });
  document.getElementById("settings-new")?.addEventListener("click", () => {
    resetProjectForm();
    showStatusBar("已回到新建模式；编辑未保存即取消，已保存配置不变", "info");
  });
  document.getElementById("settings-vp-rotate")?.addEventListener("click", () => void rotateViewport());
  document.getElementById("settings-open-start")?.addEventListener("click", () => void openStartUrl());
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
          const loading = s.phase === "loading";
          // 加载状态也走常驻条：负例时红条不能被「✓ 已加载」冲成成功语义。
          showStatusBar(
            loading ? `加载中：${s.url}` : `✓ 页面加载完成：${s.url}`,
            loading ? "pending" : "ok",
          );
        }
        break;
      }
      case "Viewport": {
        const v = eventData<ViewportData>(p);
        if (v) {
          viewportLine.textContent = fmtViewport(v);
          measuredSession = v.measured?.session ?? null;
          // E04 的「实际视口可核对 / 容纳不下明确提示」判据不能只落在被裁的第三行。
          // 但状态栏是单条覆盖通道：常态回报（页面加载完成、title 哨兵、返回预览）也会发 Viewport，
          // 若每次都写就会冲掉「✓ 页面加载完成」与返回会话提示。只在「设定值 / 容纳状态」变化时占用它。
          const cfg = v.configured ? `${v.configured[0]}×${v.configured[1]}` : "跟随窗口";
          const signal = `${cfg}|${v.fully_visible ? "可见" : "容纳不下"}`;
          if (signal !== lastViewSignal) {
            lastViewSignal = signal;
            if (!v.fully_visible) {
              showStatusBar(
                `⚠ 窗口容纳不下已设定视口 ${Math.round(v.bounds[0])}×${Math.round(v.bounds[1])}：` +
                  "视口保持不变，未静默缩放（请放大窗口）",
                "error",
              );
            } else if (v.configured) {
              showStatusBar(
                `✓ 视口已生效 ${cfg} · 窗口内完整显示（实测值请看被测页「实际视口与引擎」区）`,
                "ok",
              );
            }
          }
        }
        break;
      }
      case "Request": {
        const r = eventData<{ kind: string; url: string }>(p);
        if (r) {
          const text = `✗ 已拒绝 ${r.kind}：${r.url || "(无地址)"}（未开新窗口 / 未写下载 / 未授设备权限）`;
          requestLine.textContent = `已拒绝 ${r.kind}：${r.url || "(无地址)"}`;
          // E13 的「明确处理」判据要能被看见：第三行在 64px 视口下被裁，必须同步到状态栏。
          showStatusBar(text, "error");
        }
        break;
      }
      case "Denied": {
        const d = eventData<{ reason: string }>(p);
        // 页面内部发起的非 HTTP(S) 导航被拒绝，也必须可见，不能只在被裁的第三行留痕。
        if (d) showStatusBar(`✗ ${d.reason}`, "error");
        break;
      }
      case "Capture": {
        const c = eventData<{ phase: string; id: string; message: string }>(p);
        if (!c) break;
        const failed = c.phase === "failed" || c.phase === "invalidated" || c.phase === "cancelled";
        const level: ToastLevel =
          failed ? "error" : c.phase === "completed" ? "ok" : "pending";
        // 每个采集阶段都进常驻条：started 的「慢采已开启 …ms」可直接证明参数生效。
        // showStatusBar 内部已兜底写第三行，不在这里重复赋值（I15）。
        showStatusBar(`[${c.phase}] ${c.message || c.id}`, level);
        break;
      }
      case "DataRoot": {
        const r = eventData<RootInfo>(p);
        if (r) {
          lastRoot = r;
          renderDataRoot();
          // I13 / 4.1：数据根解析结果必须在启动时可见。工具层第三行被 64px 视口裁掉，
          // 因此和所有结果类反馈一样走底部独立状态栏（I15）；不可写直接给红条。
          showStatusBar(
            `数据根：${r.path}（来源：${r.source}，${r.writable ? "可写" : `不可写：${r.error ?? "原因未取得"}`}）`,
            r.writable ? "info" : "error",
          );
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
  // 版本徽标：只有当前 main.ts 真的跑起来，才会把红色「加载中…」写成 v...。
  // 用户启动后一眼可核对窗口是不是新代码，不靠「我重启过」这种口头确认。
  const buildTag = document.getElementById("build-tag");
  if (buildTag) {
    buildTag.textContent = BUILD_TAG;
    buildTag.classList.remove("stale");
  }
  // 慢采是开发期复验开关，正式构建不显示；Rust 侧也只在 debug 构建接受延迟。
  if (!import.meta.env.DEV && slowWrap) slowWrap.style.display = "none";
  const check = coordSelfTest();
  previewCheck.textContent = check.passed ? `坐标自检 ✓ ${check.detail}` : `坐标自检 ✗ ${check.detail}`;
  previewCheck.classList.toggle("bad", !check.passed);
  bindUi();
  await subscribeEvents();
  await loadInitialState();
  await refreshConfig();
  viewportLine.textContent = "视口：尚未打开页面";
  // 旧窗口 + 刚更新的脚本会出现「HTML 还是旧版」：此时版本徽标、慢采开关都不存在。
  // 与其让用户看到空白状态行，不如在状态栏直接给出重启指引（该通道已由 A 组复验证明可用）。
  if (!slowCapture || !slowWrap || !buildTag) {
    showStatusBar(
      "⚠ 工具界面 HTML 与脚本不同步（工具条第二行无版本徽标）：请 Ctrl+C 后重新运行 pnpm tauri dev（完整重启）",
      "error",
    );
  }
}

void main();
