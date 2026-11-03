# docs/ · 文档地图与索引

<p><a href="README.en.md">English</a> | 简体中文</p>

本目录分两层：**面向使用者的文档**（`guide` / `architecture` / `development`）与
**工程过程资产**（`research` / `product` / `engineering` / `reviews` / `drafts`）。
后者是调研、探讨、审阅、规划与验收记录，不是产品说明。

## 权威性排序

判断项目状态时，冲突以下列顺序为准：

1. `packages/*/src` 与 `apps/*/src` 源码
2. `openspec/changes/archive/` —— 已归档 change 的 tasks / design / evidence-index
3. `openspec/specs/` —— 合入主 spec 的能力口径
4. 本目录
5. [`/AGENTS.md`](../AGENTS.md) —— 接手索引，已知会滞后

过程文档里的日期就是它的时效边界：**带日期的文档描述的是当时**，不代表当前状态。

## 面向使用者

| 目录 | 内容 | 双语 |
|---|---|---|
| [`guide/`](guide/README.md) | 快速开始：安装、四条上手路径、核心概念 | 需要 |
| [`architecture/`](architecture/README.md) | 架构总览：模块划分、数据流、设计原则 | 需要 |
| [`development/`](development/workflow.md) | 开发工作流：门禁链、openspec 流程、文档与提交约定 | `workflow.md` 需要 |

## 经验库

[`knowledge/`](knowledge/README.md) —— 本仓最重要的机制：开发中踩的坑、实机硬事实、不许重开的不变量与关键决策**全部入库**，
任何开发者或任何智能体 clone 后即可接手。条目格式与升级规则见 [`knowledge/README.md`](knowledge/README.md)。

| 文件 | 收什么 |
|---|---|
| [`knowledge/engineering.md`](knowledge/engineering.md) | 构建、测试、门禁、依赖、openspec 用法上的坑 |
| [`knowledge/runtime-probe.md`](knowledge/runtime-probe.md) | 实机探针硬事实：CDP、窗口、DPR、输入法、时序、假绿判据 |
| [`knowledge/invariants.md`](knowledge/invariants.md) | 架构与口径不变量，每条给出不成立时的后果 |
| [`knowledge/decisions.md`](knowledge/decisions.md) | 关键决策速查：决定 / 理由 / 代价 / 推翻条件 |
| [`knowledge/debt.md`](knowledge/debt.md) | 未决口径与欠账，标注归谁定夺与必须定下的时点 |

## 工程过程资产

| 目录 | 内容 | 双语 |
|---|---|---|
| [`research/`](research/README.md) | 市场与竞品调研、相关研究、命名核查 | 不需要 |
| [`product/`](product/README.md) | 方向定稿、路线图、能力边界 | 不需要 |
| [`engineering/`](engineering/README.md) | `plans/` 执行方案 · `reports/` 交付与构建报告 · `notes/` 功能说明与工作流约定 | 不需要 |
| [`reviews/`](reviews/README.md) | 实机验收与审阅证据；**过程截图默认不入库** | 不需要 |
| `drafts/` | 个人工作草稿，**整目录不入库** | — |

## 双语范围

成对规则、表单类的例外与门禁检查项，全部写在 [`/i18n/README.md`](../i18n/README.md)。
一句话版本：**门面 + 接手索引 + 面向使用者文档 + 经验库必须中英成对，过程资产不翻译**；
是否成对以 [`/i18n/docs-manifest.json`](../i18n/docs-manifest.json) 的登记为准，由
`node scripts/check-i18n.mjs` 强制。

## 命名约定

- **日期前缀 `YYYY-MM-DD-`**：会话产物一律带（调研、日志、审阅、报告、回信），便于时间序检索。
- **无日期前缀**：长期有效文档（`workflow.md`、`decisions.md`、`invariants.md`）。
- 新增文档先在上表选目录，再在本文件登记一行；**跨目录链接按当前 Markdown 文件计算相对路径**。
- 术语中英对应一律查 [`/i18n/glossary.md`](../i18n/glossary.md)，代码标识与错误码原值不翻译。
- 一个目录只有 README 占位时，说明其归属与进入门槛，不写内容。

```bash
node scripts/check-i18n.mjs   # 地图里写的链接断了会在这里红
```

## 路径变更记录

- **2026-11-03 建仓骨架**：确立 guide / architecture / development / knowledge / research /
  product / engineering / reviews / drafts 九类；`docs/drafts/`、`docs/reviews/**/*.png`、
  `artifacts/`、`.layress/` 定为不入库；调研原文自仓库根迁入 [`research/`](research/README.md)。
- **口径来源**：前置项目 ReBaseAgent 的 2026-09-24「分层归组」与 2026-10-12「草稿归档」两次整理，
  以及其 `output/` 目录烂掉的教训（见 `/AGENTS.md` §五）。
