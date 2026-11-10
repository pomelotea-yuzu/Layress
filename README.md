<div align="center">

# 🧪 Layress

<p><a href="README.en.md">English</a> | 简体中文</p>

**本地界面验收助手 · Local UI Acceptance Assistant**

<p>
  <a href="LICENSE"><img src="https://img.shields.io/badge/License-MIT-blue.svg" alt="License: MIT"/></a>
  <img src="https://img.shields.io/badge/Release-none-lightgrey.svg" alt="尚无发行版"/>
</p>

</div>

> **当前状态：产品方向、Tauri 桌面载体与六阶段路线图已确认，第一阶段计划其余范围待审阅；尚未开始产品实现，没有发行版。**
> 下文是目标体验，不是已交付能力。实际进度见 [`AGENTS.md`](AGENTS.md) §二与 `git log`。

## 🧭 这是什么

Layress 面向使用 AI Agent 开发 Web / Electron 界面的个人开发者与小团队，帮助人和 Agent 共用验收任务、问题现场和复查结果。

人的最短路径是：**看到现场 → 圈出位置 → 写一句想法 → 交给 Agent → 查看修改后的结果**。
例如，圈住保存按钮并写“这里被底栏挡住了，缩小窗口后也要能保存”；工具整理现场、位置和必要上下文，降低描述与定位成本。

首版形态为 **Tauri 桌面工作台 + 共用任务核心 + CLI**。一个主窗口内操作被测页面、选择设备并原地反馈；MCP 按实际需求增加。
Tauri 使用系统 WebView，具体采集与模拟能力按平台实测公布，不默认附带另一套浏览器。
具体被测应用和 Agent 的接入范围需要逐项验证；当前不承诺任意网站、框架或客户端开箱可用。

## 📌 当前状态

- 没有可安装产品、npm 发行包或 tag；没有产品源码或 workspace。首个活动 change [add-tauri-page-host](openspec/changes/add-tauri-page-host/proposal.md)已建立，尚未编码。
- 当前已敲定[产品方向与使用方式](docs/product/product.md)、Tauri 载体和[正式开发路线图](docs/product/roadmap.md)；[第一阶段计划](docs/product/2026-11-10-stage-one-plan.md)其余范围待审阅，尚未开始实现。
- 仓库现有资产包括文档、目录归属、公开调研与工程知识、洁净度 / 链接 / 双语门禁。
- GitHub [`pomelotea-yuzu/Layress`](https://github.com/pomelotea-yuzu/Layress)为主仓库，Gitee [`yuzu-tea-duck/layress`](https://gitee.com/yuzu-tea-duck/layress)为镜像；远程同步状态以实时命令为准，智能体不 push。
- 11-03～11-09 旧阶段资料已[封存](docs/research/2026-11-09-phase-closure.md)，可作历史参考；旧验证闸门、四模块划分和桌面平台承诺不自动恢复。

## 🚀 计划中的能力

| 能力 | 目标体验 |
|---|---|
| 圈画反馈 | 从现场或已采集画面开始，圈选位置并写短描述，不强制填写选择器、JSON 或测试用例 |
| 人与 Agent 协作 | 工作台与 CLI 读写同一份任务和证据，Agent 可取得上下文并回传处理结果 |
| 修复后复查 | 关联原问题和修改后现场，区分已执行、失败与待复核；Agent 声称修好不自动等于验收完成 |
| 可重复验收 | 逐步保存可复用的检查任务与准备步骤，接入成熟自动检查能力 |

每阶段具体设计按需成文，详见[路线图索引](docs/product/roadmap.md)。桌面载体已选定 Tauri；具体 WebView 接入、适配顺序与发布日期尚未锁定。

## ⌨️ 仓库检查

Node ≥ 20；当前只有仓库门禁可运行：

```bash
node scripts/check-repo.mjs --all      # 本地排查与 CI
node scripts/check-repo.mjs --staged   # 按实际提交快照检查，pre-commit 同此
```

每个克隆设置一次 `git config core.hooksPath .githooks`。产品可运行后再补真实安装与上手命令。

## 🧱 项目结构

- [`AGENTS.md`](AGENTS.md)：唯一接手入口、状态与工程纪律。
- [`docs/product/`](docs/product/README.md)：现行产品依据、路线图索引与细分设计。
- [`docs/`](docs/README.md)：指南、架构、工作流、知识库、调研与过程资产。
- [`packages/`](packages/README.md) / [`apps/`](apps/README.md)：代码归属占位，具体包与应用目录待设计，不沿用旧四模块划分。
- `scripts/` / `i18n/` / `openspec/` / `datasets/`：门禁、双语口径、变更流程与案例归属。
- [`artifacts/`](artifacts/README.md)：一次性产物目录，目录内仅占位说明入库，其余内容不入库。

一次性产物只去 `artifacts/`，运行时数据与日志只去 `.layress/`，均不入库。

## 📚 文档

- 当前产品：[产品方向与使用方式](docs/product/product.md)
- 正式阶段索引：[开发路线图](docs/product/roadmap.md)
- 文档地图与经验库：[文档索引](docs/README.md) · [知识库](docs/knowledge/README.md)
- 开发与术语：[工作流](docs/development/workflow.md) · [术语表](i18n/glossary.md)

## 🤝 贡献

请读 [`CONTRIBUTING.md`](CONTRIBUTING.md) 与 [`AGENTS.md`](AGENTS.md)。对外登记文档同轮同步中英，内部文档默认中文。
Tauri 单应用的首个页面承载 change 已建立；第一阶段后续范围与阶段 2 另行讨论，建档不等于已经完成实现。

## 📄 许可与承诺

- 许可证：[MIT](LICENSE)。`Layress` 品牌名保留，与 npm 的 `layrr` / `layerr` 及组件库 `layui` 无关。
- **本地优先**：任务、页面资料与截图默认留本机；交给远程 Agent 或模型时明确数据流与成本，不静默外发。详见 [`SECURITY.md`](SECURITY.md)。
- **便携优先**：数据、日志、缓存与临时文件放产品目录或用户明确指定的目录；不默认写系统用户目录，不提权、不改系统 PATH，不可写时明确报错。开发环境安装与产品写盘承诺分开，详见 [`AGENTS.md`](AGENTS.md) §七。
- 行为准则与版本记录：[`CODE_OF_CONDUCT.md`](CODE_OF_CONDUCT.md) · [`CHANGELOG.md`](CHANGELOG.md)。
