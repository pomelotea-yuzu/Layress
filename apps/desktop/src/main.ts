/**
 * 可信工具界面：地址导航、刷新、视口设定与状态显示（add-tauri-page-host 任务 2.x）。
 * 本层持有 Tauri 能力；被测页面（"page" 标签）无任何命令权限。
 */

import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

/** 与 Rust 侧 TOOLBAR_H 一致（逻辑像素），也等于 #bar 的 CSS 高度 */
const TOOLBAR_H = 96;

interface MeasuredViewport {
  w: number;
  h: number;
  dpr: number;
}

interface PageEventPayload {
  kind: "Url" | "Status" | "Viewport" | "Request" | "Denied";
  data?: unknown;
}

interface ViewportData {
  configured: [number, number] | null;
  bounds: [number, number];
  measured: MeasuredViewport | null;
  fully_visible: boolean;
}

interface RequestData {
  kind: string;
  url: string;
  action: string;
}

interface PageStateInfo {
  configured: [number, number] | null;
  measured: MeasuredViewport | null;
  url: string | null;
  sampleBase: string | null;
  toolbarH: number;
  pageOpen: boolean;
}

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

function fmtViewport(v: ViewportData): string {
  const cfg = v.configured ? `${v.configured[0]}×${v.configured[1]}` : "跟随窗口";
  const m = v.measured
    ? `页面实测 ${v.measured.w}×${v.measured.h} @DPR ${v.measured.dpr}`
    : "页面实测：待样例上报";
  const visible = v.fully_visible ? "窗口内完整显示" : "⚠ 窗口容纳不下（视口保持不变）";
  return `设定 ${cfg} · 边界 ${Math.round(v.bounds[0])}×${Math.round(v.bounds[1])} · ${m} · ${visible}`;
}

async function navigate(): Promise<void> {
  const raw = urlInput.value;
  statusLine.textContent = `导航请求：${raw} …`;
  try {
    await invoke("page_navigate", { raw });
    // 成功后的状态与地址由 Rust 事件回报，这里不做乐观展示
  } catch (e) {
    // 命令失败 = 明确拒绝，不把旧页面说成新地址加载成功（E03）
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

async function loadInitialState(): Promise<void> {
  try {
    const state = await invoke<PageStateInfo>("page_state");
    if (state.url) {
      urlInput.value = state.url;
    }
    if (state.configured) {
      widthInput.value = String(state.configured[0]);
      heightInput.value = String(state.configured[1]);
    }
    statusLine.textContent = state.pageOpen ? "已有被测页面会话" : "输入地址或点击「打开样例页」开始";
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
}

async function subscribeEvents(): Promise<void> {
  await listen<PageEventPayload>("page://event", (ev) => {
    const p = ev.payload;
    switch (p.kind) {
      case "Url":
        urlInput.value = p.data as string;
        break;
      case "Status": {
        const s = p.data as { phase: string; url: string };
        statusLine.textContent =
          s.phase === "loading" ? `加载中：${s.url}` : `✓ 已加载：${s.url}`;
        break;
      }
      case "Viewport":
        viewportLine.textContent = fmtViewport(p.data as ViewportData);
        break;
      case "Request": {
        const r = p.data as RequestData;
        requestLine.textContent = `已拒绝 ${r.kind}：${r.url || "(无地址)"}`;
        break;
      }
      case "Denied":
        statusLine.textContent = `✗ ${String((p.data as { reason: string }).reason)}`;
        break;
    }
  });
}

async function main(): Promise<void> {
  bindUi();
  await subscribeEvents();
  await loadInitialState();
  viewportLine.textContent = "视口：尚未打开页面";
  void TOOLBAR_H; // 布局由 Rust 侧控制；常量仅作口径对齐说明
}

void main();
