# review.md · add-project-device-profiles 审阅

> 日期：2026-11-14。审阅方式：**用户切换后的另一 Agent 在本对话独立审阅**，按 D26 / 工作流 §「Agent 审阅与本地提交」执行；未由编写者自审、未另起子 Agent、未做多模型交叉。
> 审阅对象：`openspec/changes/add-project-device-profiles/` 六个文件（proposal / design / specs / tasks / evidence-index / .openspec.yaml），
> 以及本轮同步修订的 14 个已跟踪文档（接手索引、config、工作流、中英门面、产品 / 阶段入口、活动 change 索引、经验库）。
> 核对基准：主规格 `openspec/specs/desktop-page-host/spec.md`、不变量 I12–I17、决策 D26 / D28 / D29 / D30、`docs/knowledge/debt.md`、
> 以及 `apps/desktop/src-tauri/src/lib.rs`、`capture.rs`、`apps/desktop/src/main.ts`、`scripts/check-build.mjs` 的现有源码。
> 本文件是 change 的流程工件，不是长期结论沉淀；结论与未执行项在此，过程留在各文档。全文路径用反引号，不带 Markdown 目标链接。

## 结论

**有条件通过（仅限规划阶段）**。四类规划文档齐备，OpenSpec strict 校验与全库公共门禁在本对话独立实跑全绿；
范围划分克制（反馈 / 补拍 / CLI / MCP / D30 发行验证未混入），诚实性达标：未把未执行项写成通过、未预建 `review.md` 占位、
未把配置落点正确说成「便携通过」、9 条 requirement 与 21 个 scenario 可数可对照。

但下表 **P1 四项闭环前，不建议做本 change 相关的本地提交**。本结论不是实施授权、不是人工验收通过，也不是提交许可。

审阅判断（先给结论再挑刺）：
- 「首版明确拒绝显式相对数据根、提示改用绝对路径」这个方案本身**可接受**，符合 I13 的解析唯一且可见；问题在归属与同步动作，不在方案。
- 「工具层临时铺满工作区 + 被测层平移」的编辑区方案**不否决**，但它与 D28 静态预览同构、且依赖一个根因未定位的 64px 问题；可行性验证必须从任务 1.1 前移并留退路。
- 「平移 / 裁切 + 明确提示」替代阶段 1 计划表格里的「显示缩放」符合该计划正文的「缩放或平移」二选一，不算擅自缩范围，但须登记为欠账。

## 实跑验证（本对话执行，退出码均为 0）

| 命令 | 结果 |
|---|---|
| `../../tools/openspec/openspec validate add-project-device-profiles --strict --no-interactive --json` | `valid: true`、`issues: []`，1 通过 / 0 失败 |
| `../../tools/openspec/openspec status --change add-project-device-profiles --json` | 四类规划文档 `done`；`isPlanningComplete: true`、`isComplete: true`，`nextSteps` 指向 apply。**这两项是规划产物状态，不是实施完成** |
| `../../tools/openspec/openspec validate --specs --strict --no-interactive --json` | 主规格 `desktop-page-host` 1 通过 / 0 问题 |
| `node scripts/check-repo.mjs --all` | 退出码 0：洁净度 112 文件 0 失败 / 0 警告、34 项洁净度自测、文档自测、67 篇链接 0 问题、5 对双语 0 问题；构建门禁徽标 `v20261114-2`、3 条夹具断言、tsc + vite build、`cargo check`、`cargo test --lib` **13/13**、失败传播自测全通过 |
| `node scripts/check-i18n.mjs --links` / `node scripts/check-i18n.mjs` / `git diff --check` | 67 篇 0 问题 / 5 对 0 问题 / 无空白错误 |
| 一次性只读脚本核对新 change | 5 篇 md 的 10 个相对链接 0 断链；spec 9 requirement / 21 scenario；tasks 15 个 checkbox **全未勾选** |
| `git status --short` / `git log --oneline -6` | 14 个已跟踪文件 M + `openspec/changes/add-project-device-profiles/` 未跟踪；HEAD `808e73c` |
| `git config --get core.hooksPath` / `git remote -v` | `.githooks` 已挂；`origin` 一条 fetch + 两条 push URL（Gitee / GitHub） |
| `git ls-files openspec/changes/add-project-device-profiles/` | **0 个文件**。新目录未入索引，上述公共门禁没有覆盖这 6 个新文件；编写轮用隔离索引补做的记录只能算自述 |

与编写轮 `evidence-index.md`「本轮规划校验」的差异，如实登记、不判为造假：编写轮记录沙箱内两个可达性单测绑定临时端口被拦（11/13），获授权沙箱外重跑 13/13；
**本对话在同一沙箱内、未申请沙箱外授权，直接 13/13 通过**。说明该失败与端口 / 环境相关、非稳定现象，处置见 P2-7。
另：`isComplete: true` 与 15 项未勾选任务并存，本审阅按后者判定进度。

## 问题清单

### P1（闭环后才谈本地提交）

| # | 位置 | 现象与后果 | 建议动作 |
|---|---|---|---|
| P1-1 | `proposal.md` §Capabilities / Impact、`spec.md` Req「便携数据根内的版本化持久化」、`design.md` §5/§7 ↔ 主规格 `desktop-page-host`「便携数据根与可写检查」、`docs/knowledge/debt.md` 第 25 行 | proposal 写「Modified Capabilities: 无，不修改已归档的主规格」，但设计明确要改 `capture.rs` 的数据根入口：当前 `resolve_data_root_from` 对 `--data-dir` / `LAYRESS_DATA_DIR` 直接 `PathBuf::from(v)`，相对路径随启动工作目录解析；本 change 要把它改成**明确拒绝**。这是既有 capability 的可观察行为变更，且 `debt.md` 把它登记为「归另一 Agent 审阅方案、编写者修订」的欠账。若按「无修改」实施，归档后主规格仍写旧口径而源码已变，按 config 权威性排序接手人读主规格会漏判，欠账也不会自动销账 | 审阅定夺：**方案接受，归属与同步动作必须补齐**。① proposal 的 Impact / Capabilities 显式声明这是对 `desktop-page-host` 数据根口径的修订，写明归档时同步主规格相应 requirement（或列 MODIFIED Requirements）；② tasks 1.4 / 4.3 写明实现后回填 `debt.md` 该行销账、同步 `apps/desktop/README.md` 与 I13 口径；③ 明确 `--data-dir` 与 `LAYRESS_DATA_DIR` 两种来源都适用，不留一个改一个不改 |
| P1-2 | `design.md` §5/§6、`spec.md` Req 7 Scenario「不可写根或相对根」、`evidence-index.md` E05 ↔ `lib.rs` 1239–1278 / 1281–1322、`main.ts` 453–460、I15 / I13 | 数据根在 setup 早期解析，窗口、工具层与底部状态栏在 1281 行之后才创建；`DataRoot` 事件 1267 行发出时无订阅者，`main.ts` 的 `page_state` 兜底又被 `if (state.dataRoot)` 守住——**解析失败时 root 为 None，兜底被跳过**，用户只看到开场提示。若把相对根拒绝实现成 `resolve_data_root()` 返回 Err，错误码 `DATA_ROOT_RELATIVE_UNSUPPORTED` 只落 stderr 和一个可能丢失的事件，spec 要求的「明确失败并提示可用的绝对路径方式」与 I15「结果类反馈只走底部状态栏」同时落空；E05 反例只写「相对根拒绝」，验不到提示可见 | 明确启动期数据根错误的**用户可见通道**（状态栏创建后按错误码补发红条 / 启动失败页 / 明确的退出码，三选一即可，不预设实现），写进 `design.md` §5、spec scenario 与 E05 / E07 判据；并说明该错误属于「解析层失败」还是「可写检查失败」，因为两条路径的现有可见性不同 |
| P1-3 | `tasks.md` 1.1 / 4.2、`evidence-index.md` E07 与「方法和有效性」、`design.md` §6 ↔ `scripts/check-build.mjs` 75–88、I16 | 徽标断言只比 `index.html` 的 `data-build` 与 `main.ts` 的 `BUILD_TAG` **是否相等**，不检查值是否随源码改动**递增**。本 change 必然改工具层前端（项目入口、设置面板），若只改代码不递增徽标，门禁照样全绿，人工复验会再次跑在改动前的 JS 上——I16 记录的这类事故已命中三次。现在 tasks 1.1 只写「保留状态栏与版本自证」，evidence 只写「复验前核对徽标」，没有递增动作 | tasks 1.1 / 4.2 明确「工具层前端每次改动递增 `BUILD_TAG` 并同步 `index.html` 的 `data-build`，人工复验记录实际徽标值」；E07 加判据「徽标为实施后的新值」；建议把 `check-build.mjs` 断言升级为与工具层源码内容绑定（可作为本 change 的实现任务或明确登记为后续欠账） |
| P1-4 | `design.md` §1、`tasks.md` 1.1、`evidence-index.md` E07 / E09 ↔ D28、`docs/knowledge/debt.md` 工具层 64px 挂账 | 编辑区采用「工具层铺满工作区 + 被测层等尺寸移出可视区」，与 D28 的静态预览**同构**；工具层 CSS 视口 64px 的根因仍未定位，铺满后能否真正容纳项目表单未验证。设计只说「须独立于采集预览状态」，没定义两种模式叠加时的仲裁、关闭编辑后恢复到哪个状态、返回预览是否影响会话；也没有验证失败时的退路。而任务 1.1 把「配置模型 / 错误结构 / 模块接入定稿」与「布局可见性验证」绑在同一个 checkbox，整个 UI 方案压在一个未验证可行性上 | 把布局可行性验证拆成**独立前置任务**（判据含铺满后工具层 CSS 视口实测值、焦点、状态栏层级），给出失败退路（扩大工具条 / 独立承载视图等，不预设结论）；`design.md` §1 与 E07 / E09 补「预览中打开设置」「编辑中触发采集」「关闭编辑后返回预览」的互斥与恢复规则，保住 D28 的「返回后会话不重建」 |

### P2（实施前补齐或归档前处理，登记为待办不等于已解决）

| # | 位置 | 现象 | 建议动作 |
|---|---|---|---|
| P2-1 | `evidence-index.md` E01–E10、`tasks.md` 4.3 | E 表按 requirement 聚合，21 个 scenario 中若干 THEN 缺独立判据：Req 4「用户可显式打开起始地址」；Req 9「明确区分损坏与版本不支持」的错误码区分；`design.md` §6 的 8 个结构化错误码；「进行中 / 成功 / 拒绝 / 失败」四类状态通道（E07 只写成功 / 失败可见）；Req 3「旋转保留预设身份」；备份重建后原始字节的保留与清理口径。tasks 4.3 也漏标 E 编号 | E 表补逐 scenario 对照，或至少把上述判据写入对应 E 行并给 4.3 补 E10；否则 4.3 的「全部规格场景具备可追溯结果」无依据 |
| P2-2 | `design.md` §2、`spec.md` Req 2、`tasks.md` 2.3 | 设计把主文档完成加载回调的 URL 当作「最终实际 URL」，spec 要求重定向记最终 URL；现有 `lib.rs` 412–428 用 `payload.url()`，WKWebView 在重定向 / SPA 场景下该值是否确为最终地址**未经实测**。设计已承认 history / hash 可能不回调，但未把「先核对 API 实际值」列为前置 | 任务 2.3 加前置判据：可控重定向夹具先核对 `payload.url()`；无法可靠归属时按 `design.md` §4 跳过该次记忆并明确诊断，未覆盖项单列，不猜造成功 |
| P2-3 | `design.md` §4、`tasks.md` 3.2 | 设计说「必要时使用唯一页面标签并更新既有 label 查询」，但现有 `PAGE_LABEL` 被 `get_webview`、`sync_page_frame`、`emit_viewport`、状态栏 reparent 及 `capabilities/` 按 label 授权共同引用，只提「查询」不够 | 实施前列出全部按 label 的点与权限配置；E09 补「新标签被测页零 capability、状态栏仍置顶（I17）」判据 |
| P2-4 | `design.md` §5 ↔ 主规格「生命周期与交付诚实性」 | 「退出等待正在提交的配置写入完成」与「退出时取消未完成采集并释放资源」并发时顺序未定义：采集回调携带地址记忆意图仍在写队列、退出取消采集后该意图是否仍写入，未写 | 写清退出顺序与「待处理意图不得标成已保存」的判据，补进 E08 |
| P2-5 | `design.md` §5、`spec.md` Req 4 / Req 8 | 只读第二实例「所有项目写入禁用」，但 Req 4 的切换第一步是「先持久提交目标活动项目」；只读实例能否切换项目浏览、切换被拒时怎么表达，未定义 | 明确只读模式下切换是否禁用及其提示，写进 spec / E08 |
| P2-6 | `design.md` §7、`tasks.md` 4.1、`evidence-index.md` E08 / E10 ↔ `scripts/check-build.mjs` 68 行 | 公共链只跑 `cargo test --lib`；E08 要求真实子进程 / 双实例 / 中断测试，若写成 integration test 或需要 GUI 的夹具，门禁不会覆盖，会出现「新增测试写了但 CI 没跑」的假绿 | tasks 4.1 明确新测试必须落在该命令覆盖范围，或同轮扩展 `check-build.mjs`（本地与 CI 同链、不加跳过开关）；E10 记录实跑用例数 |
| P2-7 | `docs/knowledge/engineering.md` 新增 P2 条目、`evidence-index.md`「初跑环境失败」 | 条目把「当前沙箱禁止本地监听」写成稳定结论；本对话沙箱内 `cargo test --lib` 13/13 通过（含两个端口测试），说明是一次性 / 端口相关现象。经验库写成硬事实会与后续实跑互相打架 | 改为「偶发 / 特定端口绑定被拦，一次失败不判回归；保留回显、同一条链复跑、不删测试不加跳过」；`evidence-index.md` 补记本轮沙箱内 13/13 的独立回显，保留编写轮的 11/13 + 沙箱外 13/13 两次记录 |
| P2-8 | `docs/knowledge/decisions.md` 表后新增条目 | 决策表格已到 D31；新条目用 `- [2026-11-14][P2] …` 经验条目格式追加在表后，四段（现象⇒根因⇒规避⇒证据）实际写的是「需求背景⇒用户授权⇒建档⇒证据」，与 AGENTS.md §六固定格式语义不符，也把一条用户决定散在表后形成两处维护 | 登记为 D32（决定 / 理由 / 代价 / 推翻条件），或按四段语义重写；`debt.md` 已有对应挂账，不要三处各写一半 |
| P2-9 | `docs/README.md` 路径变更记录、`tasks.md`「审阅与交接」、`AGENTS.md` 状态卡、workflow、`openspec/changes/README.md`、中英 README | 编写轮多处写「未创建 `review.md`」「本轮不预建占位」「待另一 Agent 审阅」「本轮不提交」；本文件产出后，这些时态陈述过期，提交后会被当成事实错误 | 原编写 Agent 处理意见时统一改为「编写轮不预建占位；审阅轮由另一 Agent 产出 `review.md`」，并把状态卡 / 索引推进到「已审阅、待处理意见」 |
| P2-10 | `design.md` §6、`spec.md` Req 9 | 「在 `config/` 内按唯一名称完整备份原始字节」没有保留策略，也没说明备份是否参与后续读取 / 再次重建；多次重建会无限累积运行时文件 | 写明备份命名、上限或清理授权边界、不被当作活动配置读取；E06 补判据 |
| P2-11 | `proposal.md` §Scope、`docs/product/2026-11-10-stage-one-plan.md` 实施顺序 2、`docs/knowledge/debt.md` | 阶段计划表格把「显示缩放」列为第 2 增量交付，本 change 按正文二选一取平移 / 裁切，Scope 已说明「保留现有静态预览缩放」，但没把「真实展示缩放」登记为后续欠账 | 在 Scope 或 `debt.md` 点明这是按阶段计划正文择一，展示缩放是否另立增量由用户定夺，避免下个接手人当漏做 |

## 静态推断与实跑的区分

- **实跑得到**：上表全部命令回显、9 / 21 / 15 三个计数、10 个链接 0 断链、新目录 0 个已跟踪文件、沙箱内 13/13、中英门面同轮一致、`git diff --check` 通过。
- **读源码 / 文档的静态推断**：P1-1（现有 `PathBuf::from(v)` 行为）、P1-2（setup 顺序与 `page_state` 兜底条件）、P1-3（徽标断言只比相等）、P1-4（D28 布局同构）、P2-2 至 P2-6、P2-8 至 P2-11。**这些均未通过运行验证**，属于设计一致性判断，实现时须以实测校正。
- **未做的运行验证**：未启动 GUI、未构建 / 运行本 change 的任何代码（公共门禁跑的是既有代码基线）、未做实机写盘观测、未核查远程同步状态。

## 未执行项（不得勾完成）

- E01–E10 全部未执行；15 项实施任务全未勾选。本 change 的产品实施、配置模型 / 文件锁 / 原子替换、项目切换、重启恢复、损坏恢复与双实例行为测试均未开始。
- GUI 人工复验（可见性、焦点、状态栏层级、两项目主流程、真实退出重开、离线 / 保存失败 / 损坏配置 / 小窗口）：**未执行**。机器 PASS 不替代人工验收（I11 / I16）。
- D30 打包产物运行与 WebView 全进程真实写盘观测仍归发行轮；Windows / Linux、完整手机模拟、真机、持久登录会话、SPA 非主文档路由全面观测、Gitee CI 修复后首次回显：均未执行 / 未承诺。
- 本审阅不替代人工验收，用户保留最终决定权。

## 意见处理与提交前置（交给原编写 Agent）

1. 按 D26 由原编写 Agent 读取本文件，处理 P1 四项；P2 各项至少逐条给出「已修订 / 登记为实施前待办」的明确处置，不能仅凭本文件存在视为通过。
2. 处理意见时同步 `AGENTS.md` 状态卡、工作流、活动 change 索引、产品 / 阶段入口、`docs/knowledge/` 对应专题与 `debt.md` 欠账行；口径变更必须同轮入库，只写本机记忆等于没写。
3. **提交前**把 `review.md`、6 个新文件与 14 个已跟踪修订一起纳入完整暂存快照，实跑 `node scripts/check-repo.mjs --staged`（此时新文件才进入门禁）、`../../tools/openspec/openspec validate add-project-device-profiles --strict --no-interactive --json` 与 `git diff --cached --check`，以当时的实际回显为准，不认本文件的自述。
4. 本审阅**不提交、不 push、不打 tag**，未修改任何产品代码或 change 文档（只新增本文件）。push 与 tag 归用户；远程同步状态只认实时 `git ls-remote` 回显。

## 原编写 Agent 意见处置（2026-11-14）

以上审阅原文和结论保留，本节由用户返回后的原编写者追加，仅声明规划文档的处置，不冒充另一 Agent 复审或运行验收。所有 P1 已在规划中补齐，P2 均逐项落实到设计、规格、任务或欠账；相应产品代码与实测仍未开始。

| 意见 | 处置与可复查位置 | 实施 / 验证边界 |
|---|---|---|
| P1-1 | 已修订：proposal 明列 Modified Capability，新增 `specs/desktop-page-host/spec.md` 完整 MODIFIED 数据根 requirement；任务 1.4 / 4.3 要求两种来源同改、实现验证后同步 app README / I13、合入主规格并销账 debt | 当前主规格和源码不提前改写；两来源拒绝仍待实施 |
| P1-2 | 已修订：design §5 区分解析失败无根与可写失败有根；独立保存启动错误，`page_state` 不依赖 root 存在，状态栏就绪后补红条且不被普通提示盖掉；两份 delta、E05 / E07 和任务 1.4 同步 | 启动红条真实可见性未验证，不用 stderr 代替 GUI |
| P1-3 | 已修订：design §6、任务 1.1 / 4.2 要求每次前端变更递增并同步徽标、记录窗口新值；任务 4.1 增源码内容指纹及“保留旧值改源码必须失败”的门禁反例，E07 / E10 对应 | 既有门禁仍只有相等断言；指纹机制待实施，不预称已升级 |
| P1-4 | 已修订：新增独立前置任务 0.1，记录工具层 CSS 实测、控件、焦点、被测视口和状态栏；design §1 有同窗口 bounds / CSS 退路和实时 / 预览 / 编辑仲裁表，新增模式 scenario，E07 / E09 验证返回原会话 | 前置可行性未执行；未通过不得扩展完整 UI，无退路可行时如实标阻塞 |
| P2-1 | 已修订：evidence 增 28 个 scenario 的逐条 THEN 映射、8 个错误码与四类状态通道；起始地址显式打开、预设旋转身份、备份字节保留均有独立判据，任务 4.3 标 E10 | 映射是证据计划，28 场景均未执行 |
| P2-2 | 已修订：design §2 / 任务 2.3 要求重定向夹具前置核对 payload / WebView 当前 URL 和已知最终端点，无法可靠归属就不记忆且诊断，E03 对应 | 未假定当前 `payload.url()` 已被证实可靠 |
| P2-3 | 已修订：design §4 / 任务 3.2 列查询、布局、实测、快照、清理、reparent 和 capability 完整影响清单；E09 验新动态标签零 capability 与 I17 | 动态标签和权限回归未执行 |
| P2-4 | 已修订：design §5 定义退出拒新意图→作废 / 取消→仅完成已开始事务→释放资源；新增退出 scenario、E08 和任务 3.4 | 队列取消、revision / 字节和退出后无提交待真实测试 |
| P2-5 | 已修订：design §5 / 第二实例 scenario 明确只读实例禁创建 / 编辑 / 重建 / 切换，命令亦 `CONFIG_LOCKED`；临时浏览不记忆，重取锁后重读 revision | 第二实例交互和真实锁行为待测试 |
| P2-6 | 已修订：design §7 / 任务 4.1 / E10 要求真实子进程 / integration 测试确实接入公共链，超出 `cargo test --lib` 时同轮扩展命令，并记录用例数与失败传播 | 当前公共链仍是已有命令，新业务测试尚未编写 |
| P2-7 | 已修订：engineering 改为特定环境 / 端口偶发观察，evidence 保留编写轮 11/13 + 沙箱外 13/13，并补独立审阅轮沙箱内 13/13，不推断稳定沙箱禁令 | 没有复现本轮旧环境故障，不删测试或加跳过 |
| P2-8 | 已修订：把表后新增授权叙述改为 decisions 表内 D32，记录决定 / 理由 / 代价 / 调整条件，移除原新增经验条目 | D32 只记录用户立项授权，不把本次规划当实施授权 |
| P2-9 | 已修订：状态卡、config、工作流、活动 / 产品 / 阶段索引、中英 README 更新为已审阅并修订；路径记录和任务区分编写轮未预建占位、独立审阅轮产出 review | 历史审阅原文及初稿回显的旧计数 / 时态保留并明确属于当轮 |
| P2-10 | 已修订：design §6 / 备份 scenario / 任务 1.5 / E06 规定唯一命名、原始字节、10 份上限；不自动读取或淘汰，已满拒绝重建并提示用户自行清理，提交失败后的备份也计数 | 保留策略已定义，命名 / 上限 / 失败保护待实施测试 |
| P2-11 | 已修订：proposal Scope、阶段计划顺序 2 和 debt 真实页面展示缩放项同步，明确按正文选择平移，是否另立缩放增量归用户 | 真实页面展示缩放未实现，静态预览缩放与其分开 |

修订后为两份 delta：新能力 9 requirement / 24 scenario，既有宿主数据根 1 MODIFIED requirement / 4 scenario；共 10 / 28，16 项实施任务全未勾选。未修改产品源码、未运行 GUI 或新能力，不宣称人工通过或便携通过。

D26 要求由原编写者修订、检查并本地提交，不要求仅因规划意见修订再切换一次 Agent。push / tag 仍归用户。

### 修订轮实际检查

- `validate add-project-device-profiles --strict --no-interactive --json` 与 `validate --specs --strict --no-interactive --json`：各 1 通过 / 0 issues。
- 实际索引纳入全部 22 个变更文件；`git diff --cached --check` 通过，无产品源码变化。`check-repo.mjs --staged` 在当前沙箱初跑两个临时端口绑定 PermissionDenied（11/13），获授权沙箱外重跑退出码 0：120 文件洁净度、34 项洁净度自测、文档自测、73 篇链接、5 对双语、tsc / vite、Rust check、13/13 lib 单测、失败传播自测全部通过。
- 原独立审阅环境沙箱内 13/13 记录不改写；本次成功仅验证当前暂存文档与既有代码基线。提交 hook 继续检查最终快照，不等于产品实施 / GUI / 便携或跨平台验收。
