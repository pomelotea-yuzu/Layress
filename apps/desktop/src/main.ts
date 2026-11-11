import { getVersion } from "@tauri-apps/api/app";

// 可信工具界面骨架：仅验证承载与构建链路。
// 被测页面 WebView、地址栏与采集入口在 add-tauri-page-host 任务 2.x/3.x 实现，不在此提前宣称。

async function render(): Promise<void> {
  const app = document.querySelector<HTMLDivElement>("#app");
  if (!app) {
    throw new Error("缺少 #app 挂载点");
  }

  let version: string;
  try {
    version = await getVersion();
  } catch {
    version = "未知（getVersion 调用失败）";
  }
  app.textContent = `Layress 工具界面骨架 · v${version}（首增量实施中，尚未包含页面承载能力）`;
}

void render();
