# review.md · add-tauri-page-host 审阅

> 日期：2026-11-10。审阅方式：**本对话独立审阅**。按用户口径，一个对话审阅即可，不另起 Agent、不换模型、不做多模型交叉。
> 审阅对象：本 change 的 `proposal.md` / `design.md` / `tasks.md` / `specs/desktop-page-host/spec.md` / `evidence-index.md` / `.openspec.yaml`，
> 以及本轮同步修订的接手索引、`openspec/config.yaml`、工作流、中英贡献指南、change 索引与决策库。
> 本文件是 change 的流程工件，不是长期结论沉淀；结论与未执行项在此，过程留在各文档。
> 后续处理：下述问题表和验证表保留首次审阅时的状态；用户要求查看后，P2 的规划补充已落实，当前处置见末节，不能将旧“未处理”当最新状态。
> 最终流程说明：用户已明确此 review 来自其切换的另一 Agent，原编写 Agent 负责读取和修订；首次审阅中“一个对话即可”等措辞描述当时讨论，现行规则按工作流 D26 执行。用户已要求意见处理后本地提交，早先“不提交”记录不再是当前限制。

## 结论

**有条件通过（仅限规划阶段）**：四类规划文档齐备，严格规格校验与全库门禁均实跑通过；未发现把「候选 / 未验证 / 未获批」写成「已支持」的口径越界。
但本 change **尚未实施、未实机验证**，下表 P1 项处理后才谈得上相关本地提交。此结论不是验收通过，也不是提交许可。

## 实跑验证（本对话执行，退出码均为 0）

| 命令 | 结果 |
|---|---|
| `node scripts/check-repo.mjs --all` | 洁净度 0 失败 / 0 警告；32 项自测；62 篇文档链接 0 问题；5 篇对外双语 0 问题 |
| `../tools/openspec/openspec validate add-tauri-page-host --strict --no-interactive --json` | `valid: true`、`issues: []`，1 通过 / 0 失败 |
| `../tools/openspec/openspec status --change add-tauri-page-host --json` | 四类规划文档 `done`；`isPlanningComplete: true`、`isComplete: true`，并给出进入 apply 的 `nextSteps` |
| `git status --short` / `git log --oneline -8` | 13 个未提交改动；HEAD `c8493e8`（`4b64372` 已按用户要求撤回） |
| `git config --get core.hooksPath` / `git remote -v` | `.githooks` 已挂；`origin` 一条 fetch + 两条 push URL，与接手索引一致 |

`isComplete: true` 是**规划产物状态**，不是实施完成：tasks 全部未勾选、E01–E12 全为「未执行」。

## 问题清单

| 级别 | 位置 | 现象 | 建议动作 | 本轮处理 |
|---|---|---|---|---|
| P1 | `evidence-index.md`「已执行的文档校验」 | 原记录只写 `isPlanningComplete: true`，漏记 status 实际返回的 `isComplete: true` 与 `nextSteps`；该字段最易被误读为实施完成 | 如实补记并注明字段含义 | 已补记 |
| P1 | D26 / `tasks.md` R1 / 工作流 / `config.yaml` / 中英贡献指南 / change 索引 / 接手状态卡 | 旧口径「等待用户手写 review.md、Agent 不代写」与用户本轮指令冲突，且散落多处容易分叉 | 统一改为「由 Agent 审阅并产出 review.md（一个对话即可）」；保留处理意见 + 检查通过才提交、不得空占位、不替代人工验收、用户最终定夺 | 已全库同步 |
| P1 | `CONTRIBUTING.md` / `CONTRIBUTING.en.md` | 两份同属登记成对文档，改流程必须同轮同步，否则对外英文停在旧口径 | 同轮改两份并跑双语门禁 | 已同轮修改 |
| P2 | `specs/desktop-page-host/spec.md` ↔ `evidence-index.md` E01–E12 | 部分场景缺独立证据编号：页面请求新窗口 / 下载 / 设备权限的受控处理，以及「未执行项单列」没有对应 E 行 | 实施前补齐 E 编号或明确归并到既有 E 行 | 未处理，登记为实施前待办 |
| P2 | `tasks.md` 与 E 编号 | 任务行未标 E 编号，追溯只能从证据表反查 | 实施时在任务行标注对应 E 编号 | 未处理，登记为实施前待办 |
| P2 | `tasks.md` R1/R2 | 流程门禁与实施任务共用 checkbox，会被计入 apply 进度计数 | 后续可把流程门禁只放工作流，tasks 只留实施任务 | 本轮按用户「别大动干戈」保留结构，仅改口径 |
| P2 | E10 写盘观测 | 「系统目录未产生越界资料」若无法排除无关系统活动，容易变成永不命中的假全绿 | 实施时写明观测方法、前后差异与排除无关活动的依据，并给已知坏样本 | 未处理，实施前待办 |

## 未执行项（不得勾完成）

- E01–E12 全部未执行：无产品工程、无构建、无截图、无便携写盘观测、无权限负例、无坐标与会话验证。
- Windows / Linux 实机、移动真机、多引擎、复杂登录、持久会话、设备预设恢复、完整批注与反馈、Agent 接入均不在本 change 已验证范围。
- 视觉 / 交互人工复核、macOS 人工体验：**未执行**。机器 PASS 不替代人工验收（I11）。

## 意见处理与提交状态

- 已处理：上述 P1 三项已在本对话修订；修订后重跑 `check-repo.mjs --all`（洁净度 / 链接 / 双语均 0 问题）与 OpenSpec strict validate（`valid: true`、`issues: []`），退出码均为 0。提交时仍以当时的实际回显为准，不认本文件的自述。
- 未处理：P2 四项登记为实施前待办，不因本文件存在视为已解决。
- 本轮**不提交**、不 push、不打 tag；用户保留随时自审与最终决定权。

## 收到 review 后的处置（2026-11-10）

核对工作区，首次审阅记载的 P1 修订已经存在：status 的 `isComplete` / `nextSteps` 已补记，当前审阅规则与中英文贡献指南已同步。本轮保留这些已有改动，不将其冒充为本轮新完成。

| 原问题 | 文档处置 | 产品验证状态 |
|---|---|---|
| P2：规格场景缺证据编号 | evidence-index 新增 E13（新窗口 / 下载 / 设备权限）与 E14（执行覆盖、未执行与不确定状态），其余场景归 E01–E12 | E01–E14 全未执行 |
| P2：任务与证据未映射 | 每条实施任务追加 E 编号，后续依证据逐项判定 | 全部实施 checkbox 仍未勾选 |
| P2：R1 / R2 混入实施计数 | 改为 tasks 顶部的非 checkbox 流程说明，审阅处置留本文件，提交条件仍按工作流 | 不把文档审阅计入产品实现进度 |
| P2：E10 可能误判全绿 | design / spec / evidence / tasks 补齐隔离夹具的已知根外写入、真实进程与时间归因、前后对照、覆盖盲区；权限不足或观测不完整时不确定 / 未执行，不能通过 | 检测链反例、真实 WebView 写盘观测均未执行 |

以上四项已完成的是**规划文档修订**，没有声称对应运行行为通过。首次审阅列出的实施前文档缺口已补齐，Tauri 多 WebView、原生截图、便携存储等可行性仍需真实实现验证。本轮保持未提交，不开始编码。

修订后复核：OpenSpec 1.17.0 strict validate 退出码 0，1 项通过、0 问题；`node scripts/check-repo.mjs --all` 退出码 0，32 项洁净度自测、文档自测、62 篇已跟踪文档链接和 5 对双语全部通过。另核对 18 条实施任务全未勾选且均有 E 编号、E01–E14 均已登记；`git diff --check` 通过。
当前 review.md 尚未加入索引，公共门禁只覆盖已跟踪文档；已单独核对本文件没有 Markdown 目标链接。将来提交前仍须对完整暂存快照运行门禁，不能把本轮工作区检查说成提交快照已通过。


## 用户确认流程与本地提交（2026-11-10）

用户明确：编写 Agent 写完 change 后，切换另一 Agent 审阅并产出 review，再回原编写 Agent 读取、修改，完成后即可本地提交，无需再次确认。本轮已落实全部规划文档意见，并同步 D26、接手入口、工作流、中英文贡献指南、change 索引和 config；这不构成产品实现或人工验收通过。

提交前已将 review.md 与所有相关改动纳入完整暂存快照：`node scripts/check-repo.mjs --staged` 退出码 0，80 个文件洁净度 0 失败 / 0 警告、32 项自测、文档自测、63 篇文档链接与 5 对双语全部通过；OpenSpec strict 校验退出码 0，`valid: true`、`issues: []`。`git diff --cached --check` 通过。本段记录加入提交后，pre-commit 仍会按最终索引重跑门禁。
本轮只提交文档，不 push、不打 tag，不开始编码。
