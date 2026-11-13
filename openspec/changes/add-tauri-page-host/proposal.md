# Tauri 单窗口页面承载与采集基础

> 日期：2026-11-10。状态：**首个活动 change 已建立，尚未开始实现或实机验证**。
> 用户本轮授权建立第一个 change；本轮只准备文档，不安装依赖、不创建产品工程、不运行被测项目。

## Why

Layress 已确定使用 Tauri，一个主窗口承载页面和工具。后续设备配置、圈画反馈与复查都依赖页面能正常操作、实际视口可控、截图干净以及批注模式不丢失会话。
这些能力尚未验证；尤其单窗口多 WebView API 和 macOS WebView 数据路径存在具体限制，应在第一个可体验增量中明确，而不是先铺完反馈管理。

## What Changes

- 建立最小 Tauri / Rust + TypeScript strict 工程和共用检查入口，先在 macOS arm64 运行。
- 一个主窗口内保留可信工具界面和隔离的被测页面，支持地址、导航、刷新、输入、滚动和页面内弹窗。
- 提供基础宽高设置，核对网页实际视口；显示真实引擎与可取得的采集条件，不宣称完整手机模拟。
- 取得仅含被测页面的截图，在同一工作区进入静态预览，以一个临时矩形验证坐标对应；退出后返回原页面会话。
- 验证 Tauri 权限隔离与数据根解析 / 可写检查；失败明确，不静默改用 Electron、外部浏览器或系统用户目录。WebView 真实写盘观测按 D30 移出本 change，归首次打包后的发行轮。

## Capabilities

### New Capabilities

- `desktop-page-host`：单窗口页面承载、基础视口、绑定现场的采集 / 临时标记、错误表达和本次运行资源归属，增量规格见 [spec](specs/desktop-page-host/spec.md)。

### Modified Capabilities

无。主规格目前为空，本 change 完成并归档后才将已验证增量合入主规格。

## Impact

- 实现时预计新增根工程配置和 `apps/desktop/`，具体版本、依赖与路径须先在[设计](design.md)和目录索引登记；本轮不创建这些目录。
- 需要核对 Rust、Tauri、pnpm、系统 WebView 与构建条件，新增前端 / Rust 检查进入现有公共门禁。
- 不默认分发 Chromium 或 Node.js；Tauri 多 WebView 目前涉及 `unstable` 功能，macOS 存储与快照需单独验证。
- 不包含项目管理、设备预设与记忆、持久反馈、箭头 / 描述编辑器、前后对比、Agent CLI / MCP 或跨平台发行；这些不因建立本 change 被视为已实现或全部获批。

## Completion

完成信号：可在一个窗口内打开普通页面，在两个已记录尺寸下操作、采集并验证临时标记，返回原会话；错误、权限和数据根反例均有结果。真实写盘观测不在本 change 完成信号内（D30）。
具体顺序见 [tasks](tasks.md)，证据与未执行范围见 [evidence-index](evidence-index.md)。静态文档检查不能代替运行与人工体验验收。
