# apps/ · 应用入口归属

> 当前状态：`desktop/` 已有 add-tauri-page-host 首增量的最小 Tauri 骨架（任务 1.3，2026-11-11）；
> 页面承载、采集等能力在任务 2.x+ 实现，见[活动 change](../openspec/changes/add-tauri-page-host/proposal.md)。

## desktop/

Tauri 桌面工作台：Rust 原生层（tauri 2.12.5，`unstable` 多 WebView 功能）+ TypeScript strict 前端
（Vite 7.3.8，暂不引入 UI 框架）。结构与本地命令见 [`desktop/README.md`](desktop/README.md)。

入口与 CLI 共享任务和证据，现场圈画位置绑定采集条件；Agent 回传"已修复"不自动完成用户复核。
产品数据与临时文件遵守[便携存储要求](../AGENTS.md)（§七），界面类结果区分机器检查与人工验收。
MCP 由实际需求决定；被测 Electron 应用的适配与 Layress 自身采用 Tauri 是两件事，选定载体不表示任意被测应用已兼容。
