# review.md · fix-config-single-writer 审阅

> 日期：2026-11-19。审阅方式：**用户切换后的另一 Agent 在本对话独立审阅**，按 D26 / 工作流 §「Agent 审阅与本地提交」执行；未由编写者自审、未另起子 Agent、未做多模型交叉。
> 审阅对象：`openspec/changes/fix-config-single-writer/` 六个文件（proposal / design / specs / tasks / evidence-index / .openspec.yaml），
> 以及本轮同步修订的 6 个已跟踪文档（`AGENTS.md`、`docs/README.md`、`docs/development/workflow.md`、`docs/knowledge/debt.md`、`openspec/changes/README.md`、`openspec/config.yaml`）。
> 核对基准：主规格 `openspec/specs/project-device-profiles/spec.md`、归档 `openspec/changes/archive/2026-11-18-add-project-device-profiles/evidence-index.md`（第 421 行双实例实机记录）、`docs/knowledge/debt.md` P1 挂账、
> 以及 `apps/desktop/src-tauri/src/config_store.rs`、`apps/desktop/src-tauri/src/lib.rs`、`apps/desktop/src/main.ts` 的现有源码。
> 本文件是 change 的流程工件，不是长期结论沉淀；结论与未执行项在此，过程留在各文档。全文路径用反引号，不带 Markdown 目标链接。

## 结论

**有条件通过（仅限规划阶段）**。四类规划文档齐备，OpenSpec strict 校验在本对话独立实跑全绿；
design §1 对三条故障路径的静态描述**逐条与源码核对属实**（见下表），诊断先行（任务 0.1 / E01 允许「未重现、根因待定」）的姿态正确，
范围克制（原地批注、设置态导航 / 布局欠账、D30 发行观测未混入），诚实性达标：未把存储子进程单测当成真实桌面验证、
GUI 无同步点的重建争用明示「未覆盖」改由机器同步点证明、9 项任务全未勾选、未预建 review 占位。

但下表 **P1 两项闭环前，不建议做本 change 相关的本地提交**。本结论不是实施授权、不是人工验收通过，也不是提交许可。

审阅判断（先给结论再挑刺）：
- 「同一根内 `projects.lock` OS 独占锁 + 持锁凭证覆盖写入与发布」的方案本身**可接受**，与既有存储模块语义一致，不另起第二套锁机制。
- 「只读 + 显式重取 + 重取先验证后开放」的交接方案**可接受**；不做自动接任是对的，避免只读实例在用户不知情时变成可写。
- 「重建连续持锁、同一句柄交接」方向正确；`try_acquire` 先置可写再重读、失败不回退的现有缺陷（`config_store.rs:198-202`）被设计准确点名。
- 问题不在方案方向，在**两处规格 / 设计未闭合的语义**：只读实例的项目归属导航记忆路径（P1-1）、损坏配置实例的持锁语义（P1-2）。两者都有现成源码路径对照，不是假想敌。

### design §1 静态描述核对（全部属实）

| 设计声称 | 源码核对结果 |
|---|---|
| `ConfigStore::open` 以 `File::try_lock` 判定可写 | 属实：`config_store.rs:87-96`，`WouldBlock` ⇒ `writable=false`，句柄留存 |
| 双进程测试走存储模块测试进程，桌面启动与状态发布不在其中 | 属实：`config_store.rs:613-658` spawn 的是当前测试二进制（`--exact 子进程持锁`），不经过 `lib.rs` setup |
| `try_acquire` 先置 `writable=true` 再 `reload`，重读错误缺回退 | 属实：`config_store.rs:198-202`，`self.reload()?` 向上传播错误时 `writable` 已为 true 且锁未释放 |
| `try_acquire` 未接到桌面命令 | 属实：`lib.rs` 全文无调用点，仅存储模块与其单测使用 |
| `rebuild_with_backup` 返回即释放锁，`config_rebuild` 随后再 `open`，两步间有竞争窗口 | 属实：`config_store.rs:266-284` 自开自持锁，`lib.rs:1963→1970` 先重建后重开 |
| 设置面板读配置状态的 `writable` | 属实：`lib.rs:1721` 发布 `store.is_writable()`，`main.ts:317` 据此显示「可写 / 只读」 |

补充两个设计未点名、但实施必碰的源码事实（静态推断，见 P2-1 / P2-2 / P2-3）：
`config_rebuild` 仅在 `config_store` 为 None 时可达（`lib.rs:1936`），而 `open` 读配置失败即丢弃锁句柄（`config_store.rs:98`  Err 返回时 `lock` 被 drop）⇒ **损坏配置实例当前不持锁**；
`startup_restore` 不检查可写性（`lib.rs:2169-2208`），`remember_last_loaded_url` 提交失败分支记录 `UnsavedState` 并显示红条（`lib.rs:1109-1122`）。

## 实跑验证（本对话执行）

| 命令 | 结果 |
|---|---|
| `~/Developer/tools/openspec/openspec validate fix-config-single-writer --strict --no-interactive --json` | `valid: true`、`issues: []`，1 通过 / 0 失败，退出码 0 |
| `~/Developer/tools/openspec/openspec validate --specs --strict --no-interactive --json` | `desktop-page-host` 与 `project-device-profiles` 2 通过 / 0 问题，退出码 0 |
| `~/Developer/tools/openspec/openspec status --change fix-config-single-writer --json` | proposal / specs / design / tasks 四类 `done`。**这是规划产物状态，不是实施完成**；进度以 9 项未勾选任务为准 |
| `node scripts/check-repo.mjs --all` | **退出码 1，未跑完**：洁净度 124 文件 0 失败 / 0 警告 + 34 项自测、链接 76 篇 0 问题、双语 5 对 0 问题、徽标 `v20261118-4` 一致 + 工具层源码指纹一致 + 7 条夹具 / 命令不变量断言全过；构建门禁在「工具链缺失：pnpm」处按设计停止，**前端 tsc / vite build 段未执行**（门禁不提供跳过开关）。本审阅 shell 的 PATH 无 pnpm（node v26.13.0、npm 11.20.0、cargo 在位）；不擅自安装工具链 |
| `cargo test --lib`（`apps/desktop/src-tauri/`） | **55 通过 / 0 失败**，退出码 0，含双实例子进程持锁、替换前中断、备份重建 5 项——与归档基线 55/55 一致 |
| 一次性只读脚本核对新 change | 5 篇 md 的 12 个相对链接 0 断链；tasks 9 个 checkbox **全未勾选**；delta 1 requirement / 8 scenario；`.openspec.yaml` 存在（schema: spec-driven，created: 2026-11-19） |
| `git diff --check` / `git ls-files openspec/changes/fix-config-single-writer/` | 无空白错误；新目录 **0 个已跟踪文件**——公共门禁未覆盖这 6 个新文件，编写轮隔离索引记录只能算自述，本审阅以独立脚本补核 |
| `wc -l -c AGENTS.md` / `git log --oneline -1` / `git config --get core.hooksPath` | 202 行 / 17841 字节，在 300 行 / 18 KB 预算内；HEAD `17b18a4`；`.githooks` 已挂 |

与编写轮 `evidence-index.md`「建档校验」的交叉核对：其隔离索引记录「洁净度 130 文件、链接 81 篇、双语 5 篇」，
与本对话实跑「124 文件、76 篇、5 对」+ 新增 6 文件 / 5 篇 md **算术自洽**，不判为造假；但两者口径不同（隔离索引 vs 已跟踪），引用时须注明。
另：编写轮声明的两条 strict 校验本对话独立复跑一致；`cargo test --lib` 55/55 一致；**前端构建段本对话未能复验**（pnpm 缺失），如实单列。

## 问题清单

### P1（闭环后才谈本地提交）

| # | 位置 | 现象与后果 | 建议动作 |
|---|---|---|---|
| P1-1 | `design.md` §3、`specs/project-device-profiles/spec.md`「第二实例争用配置」场景、`tasks.md` 1.2、`evidence-index.md` E03 ↔ `lib.rs:2169-2208`（startup_restore）、`lib.rs:1072-1124`（remember_last_loaded_url）、主规格「最近地址的事实记忆」requirement 及其「导航完成与重定向」场景 | delta 只规定「**临时浏览**不记忆最近地址、不累积待补写意图」，但只读实例 B 按现有启动流程会恢复快照中的活动项目并打开页面（startup_restore 不检查可写性）；该项目归属导航完成（含重定向）后，remember_last_loaded_url 提交被 `CONFIG_LOCKED` 拒绝，**现有失败分支会记录 UnsavedState 并显示「可重试」红条**——这不是临时浏览，而是「待补写意图」，与 delta 的 THEN 直接冲突；主规格「最近地址的事实记忆」场景又无条件要求导航完成记录最终 URL，归档后两条 requirement 互相矛盾。任务 1.2 的「只读临时浏览不进待补写队列」按字面覆盖不到这条路径，实施时 B 是否恢复、是否亮红条无据可依 | ① design §3 明确只读实例**是否执行启动恢复**（恢复或跳过都可接受，但必须写死一种）；② 若恢复，明确「只读实例的项目归属导航同样不记忆、不进待补写队列、不显示未保存红条」，并给主规格「最近地址的事实记忆」补 MODIFIED delta（或在 delta requirement 文本中写明记忆以持锁为前提），消除归档后的规格矛盾；③ E03 增加夹具「B 恢复活动项目后完成一次产生新地址的导航」，断言无 unsaved、无红条、配置字节 / revision 不变；④ 任务 1.2 点名 startup_restore 与 remember_last_loaded_url 两条具体路径 |
| P1-2 | `design.md` §2「锁争用和配置内容有效性分别表达」/ §4、delta「备份重建与其他实例争用」场景 ↔ `config_store.rs:98`（open 读失败即释放锁）、`lib.rs:1936`（重建仅在 store 为 None 时可达）、主规格「损坏与未知版本的显式恢复」 | 损坏配置实例的持锁语义未定义。现状：open 遇损坏返回 Err 时锁句柄被 drop，**损坏实例不持锁**，两个损坏实例都可进入重建，由 rebuild 内部锁仲裁——E05 按此可实施。但设计「分别表达」一句可读成「损坏也保留锁」：若实施者这样改，第二实例启动将先撞 `CONFIG_LOCKED` 而看不到 `CONFIG_CORRUPT`，与主规格「损坏或高版本文件」场景（启动读取损坏须返回 CONFIG_CORRUPT）冲突，且重建入口的错误码门禁（lib.rs:1947-1957）语义随之漂移。两种读法都「符合」现有设计文字，实施会摇摆 | 在 design §2 / §4 写死一种：a) 维持「损坏不持锁」，写明重建争用由 rebuild 内同一锁仲裁、E05 记录双方 store=None 前提；或 b) 损坏也持锁，则必须定义「锁被占 + 磁盘损坏」并存时第二实例的错误表达（内容错误不得被 CONFIG_LOCKED 吞掉），并同步主规格损坏场景的 delta。建议取 a，改动最小且与现有单测一致 |

### P2（实施前补齐或归档前处理，登记为待办不等于已解决）

| # | 位置 | 现象 | 建议动作 |
|---|---|---|---|
| P2-1 | `design.md` §2、`tasks.md` 1.1 ↔ `lib.rs:112`（`OnceLock<ConfigStore>`）、`config_store.rs:191`（`try_acquire(&mut self)`） | OnceLock 只提供 `&ConfigStore`，而 try_acquire 需要 `&mut self`；「统一持锁状态」必须解决内部可变性（锁句柄 / writable 收进 Mutex 或原子状态），设计「进程内串行的状态转换」未点名这一硬约束 | design §2 或任务 1.1 补一句接线约束，避免实施时把 OnceLock 换成 UnsafeCell 之类的歧路；状态转换的串行裁决点写明确 |
| P2-2 | `tasks.md` 1.2、`evidence-index.md` E03 / E06 ↔ `lib.rs:2117-2121` | config_retry_unsaved 只读分支现返回 `CONFIG_WRITE_FAILED`（「配置存储只读…不能重试保存」），delta 要求重试保存以 `CONFIG_LOCKED` 拒绝——这是「收敛只读判定」的一个现成错误码不一致实例，设计未点名 | 任务 1.2 把该分支列为具体反例；E06 加「只读重试返回 CONFIG_LOCKED 而非 WRITE_FAILED」的可失败断言 |
| P2-3 | `design.md` §2「复用 projects.lock」/ §4「同一持锁句柄交给存储」、`tasks.md` 1.4 ↔ `config_store.rs:266-284` | rebuild_with_backup 现自开第二个锁句柄。flock 按 open file description 持有：**同进程已持锁时再对同一文件开新句柄 try_lock 会被自己的锁拒绝**（静态推断，macOS / Linux flock 语义，未实测）。当前重建仅在 store=None 时可达故不自锁；一旦按设计把重建纳入持锁周期，复用句柄是必要条件而非优化 | 任务 1.4 明确「重建复用已持有句柄，禁止持锁期间对同一锁文件开第二句柄申请」；E05 / E06 补对应反例（持锁实例发起重建不得自拒） |
| P2-4 | `tasks.md` 0.1、`evidence-index.md` E01 | E01 的假设清单（双持锁 / 路径偏差 / 提前释放 / 展示失真）合理，但可补一条已核实的静态事实缩小排查面：现有代码正常运行期**没有** drop / 重开 store 的路径（open 仅 setup 与 config_rebuild 两处），`is_writable` 是 open 时的一次性快照、此后从不与 OS 复核；「UI 可写」与「命令可提交」共用同一 bool，故二者必然同真同假，E01 的三分证据（UI / 命令 / OS 锁）中真正要区分的是「快照为真但 OS 锁已失」与「两根实际不同」 | 0.1 记录该静态结论，优先核对两进程解析后的绝对根与锁文件 inode（含默认根解析来源差异），再查句柄生命周期；未重现时按 E01 保留边界、根因待定，不猜造 |
| P2-5 | `AGENTS.md` 状态卡、`openspec/changes/README.md`、`docs/README.md`、`openspec/config.yaml` | 四处均写「待独立 Agent 审阅」；本文件产出后这些时态陈述过期（沿上一轮 P2-9 同族问题） | 原编写 Agent 处理意见时统一推进为「已审阅、待处理意见」，处理完再推进到实施态；`docs/README.md` 路径变更记录补 review.md 产出事实 |

## 静态推断与实跑的区分

- **实跑得到**：上表全部命令回显；strict 校验 1 change + 2 specs 全过；cargo test --lib 55/55；公共门禁洁净度 / 链接 / 双语 / 徽标指纹段通过、构建段因 pnpm 缺失停止；12 链接 0 断链、9 任务未勾、1 requirement / 8 scenario、AGENTS.md 202 行 / 17841 字节、新目录 0 个已跟踪文件。
- **读源码 / 文档的静态推断**：design §1 核对表全部六行、P1-1（startup_restore / remember_last_loaded_url 路径）、P1-2（open 失败释放锁）、P2-1 至 P2-4。**这些均未通过运行验证**；其中 P2-3 的 flock 同进程自拒属平台语义推断，实施时须以实测校正。
- **未做的运行验证**：未启动 GUI、未构建 / 运行本 change 的任何代码（本 change 尚无代码）、未做双桌面进程实机复验、未跑前端 tsc / vite build（pnpm 缺失）、未核查远程同步状态。

## 未执行项（不得勾完成）

- E01–E07 全部未执行；9 项实施任务全未勾选。根因核实、锁修复、只读 / 重取 / 重建接线、退出顺序与实机复验均未开始。
- 公共门禁的**前端构建 / 类型段**本对话未复验（pnpm 不在审阅环境 PATH）；实施轮任务 2.1 实跑前须先确认 pnpm 入口可用，不得以本轮部分回显宣称「公共链全绿」。
- GUI 人工复验（只读面板、状态栏拒绝提示、重取交接、重建确认）：未执行。机器 PASS 不替代人工验收（I11 / I16）。
- Windows / Linux、D30 打包产物运行与真实写盘观测：沿既有计划单列，未执行 / 未承诺。
- 本审阅不替代人工验收，用户保留最终决定权。
## 编写者处置（2026-11-19）

| 意见 | 处置 |
|---|---|
| P1-1 | 保留有效只读快照的启动恢复。导航发起时与完成时均校验写入资格，项目导航、重定向与临时浏览都不得生成旧地址补写。增加「最近地址的事实记忆」MODIFIED 增量；任务 1.2 与 E03 补启动恢复、后续导航、迟到回调和无未保存提示的判据。 |
| P1-2 | 采用损坏或未知版本启动时报告内容错误并释放所获锁的方案。两实例完成用户确认后通过同一锁仲裁重建。并列验证锁占用与内容错误，重取失败后的恢复入口也按内容状态裁决。设计 §2 / §4、并发保护规格、任务 1.4 和 E05 已同步。 |
| P2-1 | 设计 §2 与任务 1.1 明确 `OnceLock<ConfigStore>` 共享引用约束，明确状态内部串行点。 |
| P2-2 | 任务 1.2 与 E06 明确只读重试优先返回 `CONFIG_LOCKED`，包含无待保存意图的测试。 |
| P2-3 | 重建复用本次取得的句柄直到状态发布，任务 1.4 与 E05 / E06 覆盖重建自拒和交接空档。平台重锁语义仍由实施测试核实。 |
| P2-4 | 设计 §1、任务 0.1 与 E01 先对比绝对根来源及文件身份，再追踪句柄和 `writable` 快照。用会改变配置的有效命令区分锁门禁与完整提交结果。历史故障根因仍待实机复验。 |
| P2-5 | 提案、接手状态卡、文档地图、活动索引和 OpenSpec context 已同步到意见处理完成，路径变更记录已登记本审阅。 |

本节记录规划修订。9 项实施任务及实机复验仍待执行。
