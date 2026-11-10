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

当前产品依据：[产品方向与使用方式](product/product.md)——本地界面验收助手；2026-11-10 用户确定 Tauri 桌面工作台，保留单窗口体验、共用任务事实与 Agent 非交互调用要求，尚未实现。
正式阶段索引：[开发路线图](product/roadmap.md)——六阶段规划已敲定；[第一阶段计划](product/2026-11-10-stage-one-plan.md)已转 Tauri，其余范围待审阅，近期具体、远期留白，尚未开始实现。
首个活动 change：[add-tauri-page-host](../openspec/changes/add-tauri-page-host/proposal.md)——其目录与 `specs/desktop-page-host/` 分别承接首增量执行文档与待实现规格，归属见[活动 change 索引](../openspec/changes/README.md)；已立项，尚未编码。
调研背景：[受众与常用界面检查流程](research/2026-11-09-audience-and-workflow-research.md)——核查开发者调查、直接使用反馈与已有替代，保留竞品实跑、中文近期需求等未验证项；文中“形态未定”为当时讨论状态。
新阶段目标与候选建议：[面向 Agent 开发流程的界面检查](product/2026-11-09-agent-ui-review-direction.md)——一年内就业作品集与开源影响力目标保持，前一轮切片仅作候选，不按实现方便自动锁定技术与受众。
历史资料：[阶段封存](research/2026-11-09-phase-closure.md)、[原路线图快照](product/2026-11-09-previous-roadmap.md)、[基础需求与 AI Agent 增强调研汇总](research/2026-11-05-user-needs-and-agent-augmentation-research.md)、[社区原始记录](research/2026-11-04-community-evidence-round-one.md)与[VS Code 候选](research/2026-11-04-vscode-candidate-readiness.md)保留供查。旧“下次第一步”、待批状态和停止条件不自动作为新阶段执行依据。

## 双语范围

成对规则、表单类的例外与门禁检查项，全部写在 [`/i18n/README.md`](../i18n/README.md)。
一句话版本：**文档默认中文，仅逐篇登记的对外文件维护中英双版本**；当前为根目录 README、CONTRIBUTING、
SECURITY、CODE_OF_CONDUCT、CHANGELOG。接手索引、经验库、方案、指南与工程说明均默认中文，
以后确有对外阅读需求再逐篇登记进 [`/i18n/docs-manifest.json`](../i18n/docs-manifest.json)，由
`node scripts/check-i18n.mjs` 检查；已停止维护的内部英文版归档在本地 `docs/drafts/`，不入库。

全库文件链接由 `node scripts/check-i18n.mjs --links` 独立检查，不以双语清单为范围；统一门禁入口是 [`scripts/check-repo.mjs`](../scripts/check-repo.mjs)，由本地提交与两侧 CI 共用。

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
node scripts/check-i18n.mjs --links  # 检查全部已跟踪 Markdown 的本地目标路径，与双语登记无关
```

## 路径变更记录

- **2026-11-10 建立首个 change**：新增 `openspec/changes/add-tauri-page-host/` 及其 `specs/desktop-page-host/`，包含 schema 元数据、提案、设计、规格增量、任务与证据计划；更新活动 / 主规格索引、产品 / 阶段 / 目录入口、中英 README、config、工作流与接手状态。主规格仍未合入，未安装工具或开始编码；OpenSpec CLI 校验未执行。无既有路径迁移。

- **2026-11-10 转用 Tauri**：按用户决定同步[产品依据](product/product.md)、[第一阶段计划](product/2026-11-10-stage-one-plan.md)、路线图、D25、未决项、存储经验、config、接手索引、中英 README / 贡献 / 安全说明及模块、指南、架构和 i18n 入口。计划改为验证系统 WebView、Rust / TypeScript 边界与总体分发成本；历史讨论和旧三端方案只补时效说明，正文保留。无路径迁移，未装依赖、未启动实现。

- **2026-11-10 第一阶段计划提案**：新增[单窗口体验与现场记录计划](product/2026-11-10-stage-one-plan.md)，明确范围、界面、实施增量、验收与阶段边界；讨论稿、产品 / 路线图 / 目录索引、config、未决项与接手状态同轮接续。提案未逐项确认，无既有路径迁移，未启动开发。

- **2026-11-10 单应用与竞争力讨论**：新增[同类产品核查](research/2026-11-10-integrated-review-differentiation.md)，同步产品与阶段 1 讨论稿、研究索引、config、经验条目与接手状态。Electron 为优先评估方向，尚未锁定；记录已有能力重叠和未验证收益，无既有路径迁移，无产品实现。

- **2026-11-10 第一阶段讨论**：新增[体验与接入讨论稿](product/2026-11-10-stage-one-experience-and-access.md)，记录窗口形态、设备配置、模型操作能力影响及待验收项；产品依据补充设备选择与保留上次配置，路线图、产品索引、D24、config 和接手状态同步。具体接入未定，无既有路径迁移，无产品实现。

- **2026-11-09 收工一致性复核**：同步中英 README / 贡献与安全说明、GitHub / Gitee 表单、模块与指南入口、config 和术语，移除现行入口对旧四模块、首验批准与桌面平台的自动承诺；历史不变量与欠账增加时效说明，原条目与旧研究保留。现行依据统一指向产品与正式路线图；无文件迁移，未新增产品实现。

- **2026-11-09 六阶段路线图索引**：将 [`product/roadmap.md`](product/roadmap.md) 改为六阶段目标、完成信号、滚动调整原则与细分设计入口，限制索引体量 ≤ 100 行 / ≤ 8 KB，方案按需下沉；改写前正文原样保存在 [`product/2026-11-09-previous-roadmap.md`](product/2026-11-09-previous-roadmap.md)（仅新增历史快照说明）。现行产品 / 目录索引及接手状态同轮更新，历史研究中的旧链接保留为当时指向。仅文档整理，未启动实现。

- **2026-11-09 产品方向与圈画反馈确认**：新增稳定依据 [`product/product.md`](product/product.md)，记录用户确认的本地界面验收助手、Web 工作台 + CLI 与简单圈画反馈要求，区分已确认目标和实现建议；状态卡、D22、前期讨论时效注记、索引、路线图与 config 同轮接续。无路径迁移，无产品实现。

- **2026-11-09 受众与常用流程续查**：新增 [`research/2026-11-09-audience-and-workflow-research.md`](research/2026-11-09-audience-and-workflow-research.md)，核查公开调查、用户工作流与成熟替代，回应“避免越做越小众”；前一轮建议加时效说明，状态卡、D21、索引、路线图顶部与 config 注记同轮接续。无路径迁移，无产品实现。

- **2026-11-09 新阶段讨论**：新增 [`product/2026-11-09-agent-ui-review-direction.md`](product/2026-11-09-agent-ui-review-direction.md)，记录用户新目标、公开共性问题续查、成熟工具分工、本机 Playwright 条件与首个切片建议。状态卡、config 时效注记、D20、产品 / 调研索引及旧路线图顶部同轮指向新入口；旧封存说明和研究正文保持原文。无路径迁移，无产品实现。

- **2026-11-09 阶段封存**：新增 [`research/2026-11-09-phase-closure.md`](research/2026-11-09-phase-closure.md) 作为 11-03～11-09 阶段的唯一收尾说明；状态卡（[`../AGENTS.md`](../AGENTS.md) §二）、`knowledge/decisions.md` D19 与 `research/README.md` 索引同轮指向它。历史文件不删除、不改写，全部保留供查；全库旧「当前接手 / 下次第一步」字样作废。无既有文件迁移。

- **2026-11-09 最小可用草案**：新增 [`product/2026-11-09-declaration-and-evidence-draft.md`](product/2026-11-09-declaration-and-evidence-draft.md)：开发者声明四项、三态判定、有版本的证据 schema 与用 11-08 B 轮观测填好的实例。按用户「别再论证、先开始做」的原意直接产出，不再新增方向论证文档；[`research/2026-11-09-shrink-direction-comparison.md`](research/2026-11-09-shrink-direction-comparison.md) 降级为过程记录。无既有文件迁移，未立 change。

- **2026-11-09 收口接续与方向对照**：新增 [`research/2026-11-09-shrink-direction-comparison.md`](research/2026-11-09-shrink-direction-comparison.md) 作为收口后接手入口；[`research/2026-11-05-user-needs-and-agent-augmentation-research.md`](research/2026-11-05-user-needs-and-agent-augmentation-research.md) 第十节、[`product/roadmap.md`](product/roadmap.md)「当前接手从哪里开始」、[`knowledge/debt.md`](knowledge/debt.md) 与首验方案状态块同轮标注 D18 收口后的时效；`docs/research/README.md` 索引同步。无既有文件迁移，未立 change。

- **2026-11-08 CHANGELOG 回归版本定位**：`CHANGELOG.md`／`CHANGELOG.en.md`「未发布」下原有 2026-11-03～11-08 的仓库搭建与门禁过程条目，按主题回归既有落点（决策 `knowledge/decisions.md`、坑 `knowledge/engineering.md`、调研与实验 `research/`、状态 `AGENTS.md` §二、逐笔 `git log`），**不另建第三份流水账**；原文可从 `git log -- CHANGELOG.md` 查。文件口径改为「按版本记录面向使用者的变更」。无路径迁移。

- **2026-11-07 Agent 原生证据闸门**：新增 [`engineering/plans/2026-11-07-agent-native-evidence-gate.md`](engineering/plans/2026-11-07-agent-native-evidence-gate.md)，作为首验提案的精简前端（两段闸门取证"要不要开发"），首验提案加衔接注记、路线图与状态卡同步；实验对象具体项目/路径/证据按隐私口径仅留本地草稿，不入库；无既有路径迁移。

- **2026-11-06 三端打包规划**：新增 [`engineering/plans/2026-11-06-tri-platform-packaging-plan.md`](engineering/plans/2026-11-06-tri-platform-packaging-plan.md)，登记 Windows x64 / Linux Ubuntu 24 x64 / macOS arm64 三端打包承诺与分阶段验证；同步 config 平台口径、决策 D15、debt 五项未决与路线图；无既有路径迁移。

- **2026-11-05 自有应用只读核查与收工接续**：在[综合调研](research/2026-11-05-user-needs-and-agent-augmentation-research.md)补充第一手任务、已有自动化与跨客户端结果消费边界，第十节登记停点、下次第一步和未执行项；具体项目与证据仅留本地草稿，未启动或修改被查项目；无新增入库文档或路径迁移。
- **2026-11-05 综合调研入口**：新增[基础需求与 AI Agent 增强调研汇总](research/2026-11-05-user-needs-and-agent-augmentation-research.md)，整合历史证据与本轮讨论，明确应用入口、跨 Agent 调用及当前 Agent 对照的待验证问题，同步索引、路线图和状态卡；旧记录保留，无路径迁移。
- **2026-11-05 基础需求续查**：在既有[社区证据记录](research/2026-11-04-community-evidence-round-one.md)追加第七节，补充日常任务、用户证据、现有解法和待验证的最低完成标准，调整路线图与接续入口为基础需求优先；无新增文档或路径迁移。
- **2026-11-05 候选输入续查**：在既有[准备核查记录](research/2026-11-04-vscode-candidate-readiness.md)追加第七节，补充原 profile 缺口、固定源码核查与同修复的较小触发线索，同步接续入口与状态卡；无新增文档或路径迁移。
- **2026-11-04 优先候选准备核查**：新增 [`research/2026-11-04-vscode-candidate-readiness.md`](research/2026-11-04-vscode-candidate-readiness.md)，记录触发输入、版本可用性、准备成本与下次接续，同步调研索引、前轮提示、路线图和状态卡；无既有路径迁移。
- **2026-11-04 第二轮 GitHub 证据**：新增 [`research/2026-11-04-github-evidence-round-two.md`](research/2026-11-04-github-evidence-round-two.md)，接续授权后的竞品与真实缺陷核查，同步调研索引、路线图、前轮接续提示及状态卡；无既有路径迁移。
- **2026-11-04 首轮社区证据**：新增 [`research/2026-11-04-community-evidence-round-one.md`](research/2026-11-04-community-evidence-round-one.md)，登记来源、反证、真实缺陷适用范围与未执行项，同步调研入口、路线图和状态卡；无既有路径迁移。
- **2026-11-04 简短路线图**：新增 [`product/roadmap.md`](product/roadmap.md)，登记调研优先的阶段顺序与证据门槛，同步产品目录入口、接手状态卡与首验提案状态；无既有路径迁移。
- **2026-11-04 内部译本本地归档**：根目录 `AGENTS.en.md`、本地图与 `docs/knowledge/` 的 7 份英文版、`docs/development/workflow.en.md`、`i18n/` 的 2 份英文版，共 11 份移入 `docs/drafts/2026-11-04-internal-translations/`，原路径索引见归档目录的 `README.md`。该目录不入库；原文另可从 Git 历史查找。现行中英双版本仅保留根目录 5 类对外门面。
- **2026-11-04 命名规则细化**：明确稳定主题名与带日期过程记录的边界及晋升规则；本轮无路径迁移，首个验证方案仍待批准，保留日期前缀。
- **2026-11-03 建仓骨架**：确立 guide / architecture / development / knowledge / research /
  product / engineering / reviews / drafts 九类；`docs/drafts/`、`docs/reviews/**/*.png`、
  `artifacts/`、`.layress/` 定为不入库；调研原文自仓库根迁入 [`research/`](research/README.md)。
- **口径来源**：前置项目 ReBaseAgent 的 2026-09-24「分层归组」与 2026-10-12「草稿归档」两次整理，
  以及其 `output/` 目录烂掉的教训（见 `/AGENTS.md` §五）。
