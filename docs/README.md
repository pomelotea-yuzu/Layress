# docs/ · 文档地图与索引

> 本文默认以中文维护。

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
| [`guide/`](guide/README.md) | 快速开始：安装、四条上手路径、核心概念 | 默认中文；对外篇目按需登记 |
| [`architecture/`](architecture/README.md) | 架构总览：模块划分、数据流、设计原则 | 默认中文 |
| [`development/`](development/workflow.md) | 开发工作流：门禁链、openspec 流程、文档与提交约定 | 默认中文 |

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
| `drafts/` | 个人工作草稿与停止维护的内部译本本地归档，**整目录不入库**；归档子目录自带来源索引 | — |

## 双语范围

成对规则、表单类的例外与门禁检查项，全部写在 [`/i18n/README.md`](../i18n/README.md)。
一句话版本：**文档默认中文，仅逐篇登记的对外文件维护中英双版本**；当前为根目录 README、CONTRIBUTING、
SECURITY、CODE_OF_CONDUCT、CHANGELOG。接手索引、经验库、方案、指南与工程说明均默认中文，
以后确有对外阅读需求再逐篇登记进 [`/i18n/docs-manifest.json`](../i18n/docs-manifest.json)，由
`node scripts/check-i18n.mjs` 检查；已停止维护的内部英文版归档在本地 `docs/drafts/`，不入库。

## 命名约定

- **稳定主题名 `<english-kebab-case>.md`**：已批准并作为当前依据的方案、现行规则、已核实且持续引用的事实与机制说明；英文小写单词用 `-` 连接，不加日期，例如 `probe-channel-selection.md`、`agent-interface-contract.md`。单词本身完整时可用 `workflow.md`、`invariants.md`，不为凑短横线增加词语。
- **过程记录 `YYYY-MM-DD-<english-kebab-case>.md`**：日常探讨、调研、待批提案、日志、审阅和当次报告，例如 `2026-11-04-agent-interface-discussion.md`。日期后仍写清主题，不只用日期，避免同一天多个文件无法区分。
- **按用途与状态判断，不按篇幅或重要程度判断**：重要调研仍是过程记录；已批准方案才成为当前依据。稳定文档在正文标注状态、批准或核实日期、适用范围和证据来源；无日期文件名本身不证明已批准或已实现。
- **从过程结论晋升为当前依据**：把持续有效的结论提炼到稳定主题文档或既有经验条目，保留带日期的原始记录并互相链接；若只是同一份提案获批，可直接改为稳定主题名，正文保留批准记录。迁移须同轮修复引用、双语 manifest 与下方路径变更记录，避免两份都被当成现行方案。
- **例外与边界**：目录入口保留 `README.md`，英文配对保留 `*.en.md`；本规则适用于 `docs/`，不改变 OpenSpec 的固定文件名和归档日期，也不改变数据集与证据文件的命名约定。
- 新增文档先在上表选目录，再在本文件登记一行；**跨目录链接按当前 Markdown 文件计算相对路径**。
- 术语中英对应一律查 [`/i18n/glossary.md`](../i18n/glossary.md)，代码标识与错误码原值不翻译。
- 一个目录只有 README 占位时，说明其归属与进入门槛，不写内容。

```bash
node scripts/check-i18n.mjs   # 地图里写的链接断了会在这里红
```

## 路径变更记录

- **2026-11-04 内部译本本地归档**：根目录 `AGENTS.en.md`、本地图与 `docs/knowledge/` 的 7 份英文版、`docs/development/workflow.en.md`、`i18n/` 的 2 份英文版，共 11 份移入 `docs/drafts/2026-11-04-internal-translations/`，原路径索引见归档目录的 `README.md`。该目录不入库；原文另可从 Git 历史查找。现行中英双版本仅保留根目录 5 类对外门面。
- **2026-11-04 命名规则细化**：明确稳定主题名与带日期过程记录的边界及晋升规则；本轮无路径迁移，首个验证方案仍待批准，保留日期前缀。
- **2026-11-03 建仓骨架**：确立 guide / architecture / development / knowledge / research /
  product / engineering / reviews / drafts 九类；`docs/drafts/`、`docs/reviews/**/*.png`、
  `artifacts/`、`.layress/` 定为不入库；调研原文自仓库根迁入 [`research/`](research/README.md)。
- **口径来源**：前置项目 ReBaseAgent 的 2026-09-24「分层归组」与 2026-10-12「草稿归档」两次整理，
  以及其 `output/` 目录烂掉的教训（见 `/AGENTS.md` §五）。
