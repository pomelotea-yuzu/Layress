# 证据计划：项目与设备配置记忆

> 2026-11-14：编写轮**仅规划；运行证据全部未执行**（历史口径，保留）。
> 2026-11-15 更新：**0.1 布局可行性的运行证据已取得**（r3 实机回显 + 人工三句，见文末「0.1 r3 回显与勾选」）；
> E07 / E09 的布局层判据已通过，其余 E 项与业务仲裁的运行证据仍全部未执行，下表相应行仍为判据与拟用方法，不是已通过记录。
> 文档 / 编译 / 行为测试、GUI 人工复验、未执行项分列。只有实际获得结果后才登记时间、命令、退出码和相对路径。
> 2026-11-16 更新：**任务 1.2 / 1.3 机器层证据齐备并收口勾选**；Gitee / GitHub CI 双绿已由用户确认登记。见文末「1.2 / 1.3 收口」。

## 证据映射

| 编号 | 规格 requirement | 任务 | 正反判据与证据方法 | 当前状态 |
|---|---|---|---|---|
| E01 | 稳定项目身份与基础编辑 | 1.1、1.2、2.1、4.2 | 两项目稳定 ID / 重命名 / 取消 / 临时浏览；无效输入返回 `CONFIG_INVALID_INPUT`，不自动记录随机样例端口；模型测试 + GUI + JSON 对照 | 未执行 |
| E02 | 可核对的视口选择与旋转 | 1.1、2.2、4.2 | 三预设、旋转保留 presetId、手改变 custom、方形无变化；非法值拒绝；实际哨兵与采集条件对照，不只核对标签 | 未执行 |
| E03 | 最近地址的事实记忆；项目归属与迟到结果隔离 | 2.3、3.2、4.2 | 重定向夹具先核对 payload / WebView 实际 URL；最终地址可信才记忆；草稿 / 取消 / 失败 / 同 URL 迟到不改旧值；无法归属诊断且不写 | 未执行 |
| E04 | 按项目切换与重启恢复 | 2.3、3.1、4.2 | A 桌面 / B 手机往返及重启恢复；取消 / 离线不冒用旧页，重选不重载；仅编辑起始地址不导航，显式打开成功后才更新最近地址 | 未执行 |
| E05 | 两份 delta 的数据根要求；提交失败与并发保护 | 1.3、1.4、3.3、4.2 | 默认 / 自定义绝对根文件清单；两个有效来源相对路径分别解析拒绝，无 DataRoot 时状态栏就绪后红条可见；命令行覆盖环境；可写失败与写入 / 替换注入失败旧文件哈希不变、无回落，未保存可重试 | **1.3 / 1.4 机器层 + 红条可见性已过（2026-11-16）**：相对根两来源解析层拒绝 + 优先级单测、写入 / 替换失败与双实例 / 中断子进程测试、三档人工复验通过；其余运行时证据未执行，不是 D30 全进程写盘观测 |
| E06 | 损坏与未知版本的显式恢复 | 1.2、1.5、4.2 | `CONFIG_READ_FAILED` / `CONFIG_CORRUPT` / `CONFIG_VERSION_UNSUPPORTED` 分层；确认 + 原始字节备份 + 空配置提交才启写；取消 / 备份 / 提交失败保原件；唯一命名、10 份上限、备份不自动读取 / 删除 | 未执行 |
| E07 | 工具布局不改变选定视口；项目归属；启动错误 | 0.1、1.1、1.4、3.3、3.4、4.2 | 铺满后 CSS 实测与工作区尺寸、控件 / 焦点 / 状态栏层级；实际视口不变；进行中 / 成功 / 拒绝 / 失败四类反馈全可见；徽标为实施后的新值；解析失败无根红条不被开场提示覆盖；模式仲裁 | **布局可行性通过（r3 实机 + 人工）**；设定固定视口后缩窗反例、启动错误红条、四类状态通道未执行 |
| E08 | 提交失败与并发保护；迟到结果隔离 | 1.3、3.2、3.4、4.1、4.2 | revision 单调，`CONFIG_STALE` 拒旧；真实子进程中断得到完整旧 / 新文件；第二实例切换等命令 `CONFIG_LOCKED`，重取锁须重读；退出拒新意图、取消未启动队列 / 采集，只完成已开始事务，退出后无新提交 | 未执行 |
| E09 | 项目归属；工具布局；旧现场不变 | 0.1、3.2、3.4、4.2 | 完整 label / capability 清单，新动态标签零 capability、状态栏 I17；预览→编辑→返回保留 ID / 标记 / 比例 / 会话，编辑中采集被拒，现场变更不挂回旧预览，旧文件哈希不变 | **布局层通过（r3）**：页面只平移不重建、状态栏未被遮挡、恢复归位；预览→编辑→返回、迟到结果隔离等业务仲裁未执行（归 3.4） |
| E10 | 两份 delta 全部 requirement 收口 | 4.1、4.3 | strict + 公共链实际命令 / 用例数；子进程 / integration 测试确实接入；保留旧徽标改源码必须判红，更新徽标 / 指纹后通过；逐场景结果核对；归档合入新能力和数据根修订，回填 I13 / app README / debt | 实施验证未执行 |

## 逐场景对照

下表覆盖修订后的 10 条 requirement / 28 个 scenario（项目能力 9 / 24，既有宿主修订 1 / 4）。每行都必须在实施轮记录实际证据与结果；0.1 涉及的布局层场景已由 r3 回显记录（见文末），其余场景目前全部未执行。

| capability / 场景 | 证据 | 独立 THEN 核对点 |
|---|---|---|
| project-device-profiles / 创建并重命名项目 | E01 | 两 ID 不同且改名不改 ID，配置仍属原项目 |
| project-device-profiles / 编辑无效或取消 | E01 | 错误明确；取消后文件和当前页面不变 |
| project-device-profiles / 没有项目记录 | E01 | 文件缺失为空，临时浏览不自动记忆 |
| project-device-profiles / 导航完成与重定向 | E03 | 夹具验证最终实际 URL、起始地址独立，无业务通过断言 |
| project-device-profiles / 导航失败或仍是输入草稿 | E03 | 最近地址未改，目标与旧页实际地址分开 |
| project-device-profiles / 选择预设后旋转和自定义 | E02 | 844×390 保留 phone，手改后 custom，确切尺寸保存 |
| project-device-profiles / 无效尺寸与方形旋转 | E02 | 非法值不改运行 / 落盘值，方形保持原值 |
| project-device-profiles / 两项目往返与重新启动 | E04 | 最近项目与各自地址 / 尺寸恢复，重选不重载 |
| project-device-profiles / 目标离线或打开失败 | E04 | 目标失败 / 重试可见，记录不丢，旧页不冒用 |
| project-device-profiles / 取消切换与单独编辑起始地址 | E04 | 取消保持现场；编辑不导航；点打开起始地址后才导航并在成功后更新 |
| project-device-profiles / 旧项目结果在切换后到达 | E03、E08、E09 | 不导航 B、不覆盖 B 条件 / 图片，旧文件不改 |
| project-device-profiles / 同地址重试或过期写请求 | E03、E08 | 操作身份 / revision 拒旧，无法归属不记忆 |
| project-device-profiles / 切换后核对权限与状态栏 | E07、E09 | 动态标签零 capability，原生拒绝，状态栏不被覆盖 |
| project-device-profiles / 小窗口和设置面板 | E07 | 实际视口 / 采集条件不变，控件可用与限制可见 |
| project-device-profiles / 编辑与静态预览仲裁 | E07、E09 | 编辑中采集拒绝，返回原预览及会话；上下文过期回实时页并提示 |
| project-device-profiles / 默认根与自定义绝对根 | E05、E09 | 配置 / 附属文件只在显示根，旧采集元数据不改 |
| project-device-profiles / 不可写根或相对根 | E05、E07 | 有根可写失败 / 无根解析失败分别可见，禁写 / 禁采且无回落 |
| project-device-profiles / 写入或替换失败 | E05 | 原文件逐字节保持；跨项目不切换，当前未保存明确且可重试 |
| project-device-profiles / 配置提交中断后重新启动 | E08 | 各中断点完整旧 / 新 JSON 二选一，无未提交成功提示 |
| project-device-profiles / 第二实例争用配置 | E08 | 切换 / 编辑等 UI 与命令拒写；释放后重取锁 / 重读 revision |
| project-device-profiles / 退出时仍有异步意图和写入 | E08 | 作废 / 取消 / 等待 / 释放顺序有事件证据，退出后无迟到提交 |
| project-device-profiles / 损坏或高版本文件 | E06 | CORRUPT 与 VERSION_UNSUPPORTED 分码，原文件不改，无猜造模型 |
| project-device-profiles / 确认重建及备份失败 | E06 | 取消 / 备份失败 / 提交失败原件不变；确认且两步成功才启写 |
| project-device-profiles / 备份保留与达到上限 | E06 | 唯一备份字节一致，不自动读取 / 删除，第 11 份拒绝且原件不改 |
| desktop-page-host / 数据根可写 | E05 | 显示真实根，采集归该根 |
| desktop-page-host / 数据根不可写 | E05、E07 | 显示已解析根及原因，不回落、不冒充通过 |
| desktop-page-host / 命令行或环境变量给出相对根 | E05、E07 | 分别测两个来源，解析层分码，状态栏就绪后无根也有红条及绝对路径提示 |
| desktop-page-host / 命令行覆盖环境变量 | E05 | 有效绝对 CLI 根覆盖环境相对值，不误拒；来源 / 绝对根可见 |

## 内部错误码与状态通道

| 错误码 / 状态 | 注入条件与证据 |
|---|---|
| `CONFIG_INVALID_INPUT` | 空名称 / 非法 URL / 非法尺寸，E01 / E02 |
| `CONFIG_READ_FAILED` | 配置文件读取 I/O 失败，E06；区别于成功读取但 JSON 坏 |
| `CONFIG_CORRUPT` | 截断 JSON / 重复 ID / 悬空引用等，E06 |
| `CONFIG_VERSION_UNSUPPORTED` | 可读但未知 schema，E06；不自动降级 |
| `CONFIG_WRITE_FAILED` | 写入 / 替换 / 备份失败或备份上限，E05 / E06 |
| `CONFIG_LOCKED` | 第二实例请求写入或切换，E08 |
| `CONFIG_STALE` | 过期 revision，E08 |
| `DATA_ROOT_RELATIVE_UNSUPPORTED` | 有效 CLI / 环境变量分别给相对根，E05 / E07 |
| 进行中 / 成功 / 拒绝 / 失败 | E07 分别触发正常待完成保存、提交成功、编辑中采集拒绝、保存或启动解析失败；四类均经底部状态栏，普通提示不覆盖启动错误 |

## 方法和有效性

- 每次前端改动递增并同步两个徽标字段；GUI 复验记录实施后的新徽标实际值，不接受两个旧值相等作为自证。不符时整轮结果作废。内容指纹门禁在任务 4.1 实现，不能提前称已覆盖。
- A / B 使用同一受控本地服务的不同主文档地址，另加延迟和重定向端点；分别配桌面与手机视口。测试当前能观测的主文档路由，SPA history / hash 单列限制。
- 文件测试使用隔离数据根，记录提交前后内容 / 哈希；坏样本必须得到预期错误代码，不能以“应用没崩”判通过。文件锁 / 中断测试用真实子进程补足内存模拟。
- 写失败注入应覆盖临时写入、同步、替换、备份各阶段；真实不可写用不能作为目录的路径等可重复反例，避免仅 chmod 在高权限进程下失效。
- 过期结果用可控延迟制造，核对操作代号、最终页面身份、实际尺寸和配置内容；旧结果未触发即为未覆盖，不能判隔离有效。
- 运行截图、JSON、日志和中断夹具只留 `.layress/` 或 `artifacts/`；手写结论回填本文件与经验库，不提交被测页面资料或新增垃圾场。公开文档只记录相对路径。

## 编写轮初稿校验（历史）

2026-11-14，以下为实际命令回显；验证的是规划文档与既有代码基线，**不计为本 change 的业务实现 / 人工验收**。

| 检查 | 实际结果与边界 |
|---|---|
| `../../tools/openspec/openspec validate add-project-device-profiles --strict --no-interactive --json` | 1 个 change 通过，0 issues；9 条 requirement、21 个 scenario |
| `../../tools/openspec/openspec status --change add-project-device-profiles --json` | proposal / specs / design / tasks 齐备；`isComplete` 指规划齐备，15 项实施任务仍全部未勾选 |
| 纳入新文件的文档 / 洁净度检查 | 在 `artifacts/` 下建立临时索引和对象目录，复制原索引后仅在隔离索引纳入本轮变更；洁净度检查 118 个文件，失败 0 / 警告 0；全库链接检查 72 篇，问题 0；5 篇对外文档中英配对通过；真实暂存区保持为空 |
| `node scripts/check-repo.mjs --all` | 沿用 app README 显式工具链 PATH；获授权在沙箱外实跑，退出码 0；前端类型 / 构建、Rust 编译、既有 13 项 lib 单测、门禁自测全部通过。普通 `--all` 不含尚未暂存的新文件，新增文件覆盖由上一行隔离索引检查补足 |
| 初跑环境失败 | 默认 PATH 无 pnpm；沿用已有工具链后，当轮环境下两个既有可达性测试的特定临时端口绑定被拦（`PermissionDenied / Operation not permitted`），11 通过 / 2 失败；获授权沙箱外重跑 13/13 通过，不改测试、不增加跳过开关 |

工具链 PATH 口径见 [app README](../../../apps/desktop/README.md)，本轮未安装依赖。GUI、项目持久化和重启恢复均未实现 / 执行，仍按下节单列。

## 审阅与修订轮校验

- 独立审阅轮回显见 [review](review.md)：其环境中未申请沙箱外授权，原有端口测试及完整公共链均 13/13 通过。与编写轮一次 11/13、沙箱外 13/13 记录并存；只支持环境 / 端口条件相关的判断，不能推出所有沙箱稳定禁止监听。
- 本轮规划修订为 10 条 requirement / 28 个 scenario、16 项未勾选任务；全部意见处置见 review 末尾。
- 修订轮已将全部 22 个变更文件（含 review 和两份 delta）纳入真实索引；`node scripts/check-repo.mjs --staged` 获授权在沙箱外实跑退出码 0：120 文件洁净度、34 项洁净度自测、文档自测、73 篇链接、5 对双语、前端类型 / 构建、Rust check、13/13 lib 单测及失败传播自测全部通过。沙箱内同次初跑仍是两个端口绑定 PermissionDenied（11/13），失败记录保留；不据此推翻独立审阅环境曾 13/13 的事实。
- 本轮 OpenSpec change strict 与 `--specs` strict 各 1 通过 / 0 issues；`git diff --cached --check` 通过。静态计数核对 28 场景全部有逐条 THEN 映射、15 条 review 意见全有处置、16 任务均未勾选。提交 hook 仍对最终快照执行同一公共链，不跳过检查；当前没有新能力实现 / GUI 结果。

## 未执行与继承限制

- 本 change 全部产品实施、行为测试、GUI 与真实重启复验：未执行，本轮仅建档。
- 另一 Agent 已产出 review；编写者仅补意见处置，不代写原审阅结论。规划修订和本地提交不等于实施或人工验收。
- D30：打包产物运行与 WebView 全进程真实写盘观测仍归发行轮。本次仅能验证新增配置文件落点，不得宣称便携通过。
- Windows / Linux、完整手机模拟、真机、持久登录会话、SPA 非主文档路由全面观测：本轮不承诺已支持或验证。
- Gitee CI 修复后的首次回显待用户 push；本地运行门禁不代表 Gitee 已通过。本轮不核查远程同步、不 push 或打 tag。

## 2026-11-15 补记：Gitee CI 第二次回显（不属本 change 验收）

- 用户提供 2026-11-14 14:51 Gitee Go 原始日志：Node 20.19.0 与 `npm install -g pnpm@12.18.0` 均成功，`pnpm -v` 报 `Failed to switch pnpm to v12.18.0` / `spawnSync Unknown system error -8`，命令在工具链前置阶段即失败，Rust / WebView / 公共链三步都未跑到；GitHub Actions 同一提交用户回显通过。
- 2026-11-15 已定位为容器预装 pnpm 10 抢在 12.18.0 之前并按 `packageManager` pin 自切换（受管 shim 无法被 spawn），修 `.workflow/ci.yml`：PATH 前置 npm prefix/bin、关闭 `manage-package-manager-versions`、版本自证、rustup 改 USTC 二进制、crates 用 rsproxy sparse。本机 `sh -n` 与 `check-repo.mjs --all`（13/13）通过。
- **云端复验仍待用户 push**；E01–E10 的运行证据仍全部未执行、16 项任务仍全未勾选（模型层单测见下节），本 change 的实施与验收状态不因 CI 配置修订改变。根因与规避见 `docs/knowledge/engineering.md`，欠账行见 `docs/knowledge/debt.md`。
- > **2026-11-16 补记**：上行「待用户 push」为当时口径。本机当日实跑 `git ls-remote` 确认 Gitee 与 GitHub 的 `main` 均为 `951d634`（等于本地 HEAD，无未推提交），且**用户确认 push 后 Gitee 与 GitHub CI 双绿**（原始日志未入库，依据为用户回显）；debt「Gitee Go 容器工具链」欠账据此销账。CI 配置本身的最后修改仍在本仓 `ef87388`，云端跑的是该配置。

## 实施轮启动记录（2026-11-15，仅纯配置模型层）

- 实跑：`cargo test --lib`（`apps/desktop/src-tauri/`，macOS arm64 / cargo 1.99.1）退出码 0，**27 项通过 / 0 失败**（既有 13 + 新增 14）；`node scripts/check-repo.mjs --all` 退出码 0（洁净度 34 项自测、链接 / 双语、tsc + vite、`cargo check`、27 项 lib 单测、失败传播自测全通过）。新增测试落在 `cargo test --lib` 覆盖范围内，符合任务 4.1「不写公共链跑不到的测试」的要求。
- 落地范围：`src/project_config.rs` 的配置模型、`layress.project-config/v1` / revision / 稳定 ID / 名称 / HTTP(S) 地址 / 16–4096 尺寸 / 活动引用校验、三预设与旋转 / 自定义、8 个稳定错误码；`lib.rs` 只加模块声明与把 `validate_url` 提为 `pub(crate)`，既有地址校验行为不变。
- 覆盖的模型层判据（**纯逻辑，不是 GUI / 文件 / 进程结果**）：E01 稳定 ID / 重命名 / 无效输入，E02 三预设 / 旋转保身份 / 手改变 custom / 越界与方形，E06 `CONFIG_CORRUPT` 与 `CONFIG_VERSION_UNSUPPORTED` 分层、坏输入不发布模型，E08 `CONFIG_STALE` 与 revision 单调。
- **未执行、不得勾任务**：1.2 的「默认文件缺失」与 1.3 文件存储 / 独占锁 / 串行写 / 原子替换 / 中断与双实例、1.4 数据根相对路径拒绝与启动红条、0.1 界面可行性、2.x / 3.x 全部 UI 与编排、4.1 徽标内容指纹、4.2 人工复验、4.3 归档合入；E01–E10 的运行时与 GUI 证据仍全部未执行。
- 中间态：模块接线前带 `#![allow(dead_code)]`（注释已写明接线后删除）；8 个错误码中 `CONFIG_READ_FAILED` / `CONFIG_LOCKED` / `DATA_ROOT_RELATIVE_UNSUPPORTED` 尚未由实现路径抛出，`CONFIG_WRITE_FAILED` 目前只用于序列化失败。
- D30 发行轮、Gitee 云端复验（待用户 push）、Windows / Linux 平台仍单列，本机单测全绿不汇总为通过（I11）。

### 任务 0.1 机器侧探针已就绪（2026-11-15，人工复验未执行）

- 新增 debug-only `LAYRESS_PROBE_LAYOUT=1`（`lib.rs`）：复用既有 `[layress-probe]` 标题通道，自动记录常态 / 铺满 6 轮 / 恢复三组数据——工具层 CSS 视口与 `#bar` 高、临时诊断表单 6 个控件的可见 / `elementFromPoint` 命中 / 可聚焦数、三块视图 bounds、`configured` 与被测页 title 哨兵回报；铺满由 `run_on_main_thread` 执行 `apply_layout(app, true)`，每轮重排以适配窗口缩放。
- 不改前端源码 ⇒ 徽标仍 `v20261114-2`，不触发任务 1.1 的递增要求；不进入采集状态机、不写产品数据、release 为空实现。探针注入的诊断表单是临时 DOM，不代表项目 UI 已实现。
- 实跑：`cargo test --lib` 27/27、`check-repo.mjs --all` 与 `--staged` 退出码 0（本机 macOS arm64）。**GUI 启动、小窗口、控件肉眼可见、状态栏层级、焦点手感、预览→设置→返回、编辑中拒绝采集全部未执行**；机器 PASS 不替代人工（I11），release 构建未实跑。
- 操作清单与判据见 [app README「任务 0.1」](../../../apps/desktop/README.md)；通过前 0.1 不勾选、1.1 之后完整 UI 不扩展；失败退路按 design §1 同窗口 bounds / CSS 修正，仍失败如实标阻塞。

### 0.1 首轮回显（2026-11-15，用户实机提供；判定：布局可行 / 任务未通过）

数据源：用户 `LAYRESS_PROBE_LAYOUT=1 pnpm tauri dev` 终端回显（macOS arm64，窗口 1280×800 → 595×521 → 520×400，dpr=2，徽标 `v20261114-2`）。

| 阶段 | tool bounds（逻辑 px） | 工具层 CSS | 控件 可见 / 命中 / 可聚焦 | 状态栏 bounds | 判定 |
|---|---|---|---|---|---|
| normal | 1280×96 | 1280×64 | 0（未注入表单） | (0,758) 1280×42 | 常态基线 |
| full0–2 | 1280×800 | 1280×768 | 6 / 6 / 6，无裁切 | (0,758) 1280×42 | **大窗口铺满可行** |
| full3 | 595×96 | 595×64 | 0 / 0 / 6，6 控件全被裁（top 155–289） | (0,479) 595×42 | **探针竞态，非布局能力失败**：Resized → relayout 把工具层复位成 96px |
| full4–5 | 520×400 | 520×368 | 6 / 6 / 6，无裁切 | (0,358) 520×42 | **最小窗口铺满可行** |
| restore | 520×96 | 520×64 | 6 / 0 / 6（表单清理前瞬时） | (0,358) 520×42 | 工具层恢复 96px；报告顺序缺陷 |

- 硬事实：`cssH = tool bounds 逻辑高 − 32`（96→64、800→768、400→368，9 组全部一致），`cssW` 与 bounds 宽一致；**64px 不是随机缺失，是固定 32px 偏移**，成因（WKWebView inset / 安全区）仍未实测定位，不宣称已修。
- 同轮已修：`relayout` 在探针期间由 debug-only 标志保持铺满，消除 full3 竞态；`restore` 阶段先移除诊断表单再报告（重跑应得 `controls=0`）。`cargo test --lib` 27/27。
- **未执行、不得勾 0.1**：全程 `page=None`（未点「打开样例页」）⇒ 被测页 bounds / 实际视口、预览→设置→返回现场保持、编辑中拒绝采集、动态标签 capability 全部无证据；用户未提供三句肉眼观察（表单可见、状态栏层级 / 红条、焦点手感）；状态栏层级只有 bounds 数值，`bring_status_to_front` 未触发；修复后的探针尚未重跑；release 构建未实跑。
- 「编辑中拒绝采集」属任务 3.4 的业务逻辑、尚未实现，0.1 不得假称通过；`focusable=6` 在全裁切的 full3 仍成立 ⇒ 焦点自测不能单独证明可见性，判据须同时看 `visible` / `hit`。
- 结论：**布局可行性（大窗口 + 最小窗口）通过，0.1 整体未通过、未勾选**，1.1 之后完整 UI 仍不扩展。修复后重跑一轮并补齐被测页与人工观察，再判定。

### 0.1 第二轮回显与 r2 探针（2026-11-15 12:53）

- 用户第二次贴回的回显与首轮**逐字一致**（`Finished dev profile … in 9.15s`、`full3` 595×96 / 控件全裁、`restore` `controls=6` 且 `clipped` 全列、全程 `page=None`、徽标 `v20261114-2`），且不含版本标记 ⇒ 判定为**修复前二进制或同一份旧日志**，不是 r1 修复后的运行结果，不采信、不改变 0.1 状态。决定性反证：r1 已把 `restore` 改为「先移除表单再报告」，新二进制必然输出 `controls=0`。
- 同轮把探针升级 **r2**：JS 报告与 Rust 日志都带 `probe=r2`；探针启动 3 秒后自动调用 `page_navigate` 打开样例页（免去人工点按钮，`page` 不再是 None）；终端打印白话步骤提示（关旧进程 / `Compiling layress-desktop` / 缩窗 / 贴回显）。
- `apps/desktop/README.md` 的 0.1 清单改为四步照做版，明确「没有 `probe=r2` 和 `已自动打开样例页` 就别贴」。
- 实跑：`cargo test --lib` 27/27、`check-repo.mjs --staged` 退出码 0（本机 macOS arm64）。**r2 的 GUI 运行、自动导航、缩窗、三句人工观察全部未执行**；release 构建未实跑。
- 0.1 仍未通过、未勾选；E07 / E09 状态不变。r2 回显到达并补齐人工观察后再判定，通过前不扩展 1.1 之后完整 UI。

### 0.1 r2 回显判定与 r3 探针（2026-11-15 13:03）

- 用户第三轮贴回 `probe=r2` 回显（新程序确认）+ 人工三句「能 / 没挡 / 能」。数据：`已自动打开样例页：http://127.0.0.1:53259/`；`normal` tool 1280×96 / cssH 64、page (0,96) 1280×662、status (0,758) 42；`full0–5` tool 1280×800 / cssH 768、page position x=3280 且 **size 恒为 1280×662**、status 仍在底部、6/6/6/6 无裁切、`build=v20261114-2`；`restore` tool 1280×96、page position 回 0 且 size 不变、`controls=0`、clipped 空。
- 判定：**E07 / E09 的布局层证据在大窗口下齐备**——铺满后 CSS 工作区 768、6 控件可见 / 可命中 / 可聚焦、被测页尺寸未被偷偷改动（只平移出视口）、状态栏未被遮挡（人工）、焦点可用（人工）、恢复后工具层与页面归位、临时表单清零。`cssH = bounds 逻辑高 − 32` 在 r2 再次成立（96→64、800→768）。
- **仍未执行、不得勾 0.1**：6 轮窗口恒为 1280×800，用户未缩窗 ⇒ 小窗口（520×400）在 r2 无数据；因未触发 Resized，「竞态修复后 relayout 保持铺满」也未被验证（r2 的 full3 不再是 595×96 只是因为没缩窗，不能算修复通过）；静态预览→设置→返回的真实 UI 流程与「编辑中拒绝采集」（任务 3.4 业务逻辑）未实现；release 构建未实跑。
- 同轮升级 **r3**：探针在 `i==3` 前自动 `window.set_size(520,400)` 主动触发 Resized，后 3 轮自动覆盖小窗口，结束前还原 1280×800；回显带 `probe=r3`。**操作者不用再手动缩窗**，README 第 3 步已改为「什么都不用点，看 14 秒」。
- 实跑：`cargo test --lib` 27/27、`check-repo.mjs --staged` 退出码 0（本机 macOS arm64）；**r3 的 GUI 运行、自动缩窗、大小窗口人工观察全部未执行**。0.1 仍未通过、未勾选；E07 / E09 状态不变，待 r3 回显再判定。

### 0.1 r3 回显与勾选（2026-11-15 13:16，用户实机 + 人工三句）

> 以上 r1 / r2 / r3 各历史小节均记录当时状态，**最终结论以本节为准**；历史原文保留不改写。

数据源：`probe=r3` 完整回显（macOS arm64，窗口 1280×800 → 自动缩到 520×400 → 还原），徽标 `v20261114-2`，人工三句「和刚刚一样能，没挡，能」。

| 阶段 | 窗口 / tool | CSS | page bounds | status | 控件 可见/命中/聚焦 | 判定 |
|---|---|---|---|---|---|---|
| normal | 1280×800 / 1280×96 | 1280×64 | (0,96) 1280×662 | (0,758) 42 | 0（未注入） | 基线 |
| full0–2 | 1280×800 / 1280×800 | 1280×768 | x=3280，size 1280×662 **不变** | (0,758) 42 | 6/6/6/6 | 大窗口铺满通过 |
| full3–5 | 520×400 / **520×400（未复位成 96）** | 520×368 | x=2520，size 520×262 | (0,358) 42 | 6/6/6/6 | 小窗口通过；Resized 竞态修复验证通过 |
| restore | 1280×800 / 1280×96 | 1280×64 | (0,96) 1280×662 | (0,758) 42 | 0（表单已移除） | 恢复干净 |

- 通过判据：`已自动打开样例页` + 自动缩窗提示齐备；6 轮 tool 高始终等于当前窗口高，缩窗后没有再出现 96px ⇒ `LAYOUT_PROBE_FULL` / `relayout` 修复生效；`cssH = bounds 逻辑高 − 32` 在三种尺寸下继续成立（96→64、800→768、400→368）；6 控件在大 / 小窗口均可见、可命中、可聚焦、无裁切；状态栏 bounds 恒在底部 42，人工确认「没挡」；焦点人工「能」；`restore` 工具层回 96、窗口还原、page 归位、`controls=0`。
- 硬事实与边界：`configured=None`（跟随窗口模式），缩窗时 page bounds 由 1280×662 变为 520×262 属既有 `apply_layout` 行为，**不是铺满布局偷偷改视口**；但「先用 `page_set_size` 设定固定视口、再缩窗核对尺寸不变」这一 E07 反例在 0.1 **未执行**，单列到 3.4 回归（归档 change 5.2 第三轮曾人工复验过缩小窗口后视口未静默改动，属既有证据，不冒充本 change 实测）。被测页 title 哨兵已由探针触发，但回显未留下 CSS 实测值，不写成已核对。
- **未执行、不随 0.1 勾选**：静态预览→设置→返回的真实 UI 流程、编辑中拒绝采集（任务 3.4 业务逻辑）、四类状态通道与启动错误红条（1.4 / 3.3）、迟到结果隔离（3.2）、release 构建、Gitee 云端复验（用户暂不 push）。人工三句按大小窗口均通过采信，但未逐窗口分别记录。
- 结论：**0.1 布局可行性前置通过并勾选**（范围按 design §1 / review P1-4，只含布局层）；E07 标布局可行性通过、E09 标布局层通过，其余运行时证据仍未执行。1.1 之后的完整 UI 现在可以开始，但 3.4 业务仲裁仍属欠账。

### 实施轮：任务 1.3 配置存储模块（2026-11-15，纯 Rust / 未接线）

- 落地 `apps/desktop/src-tauri/src/config_store.rs`：`<data-root>/config/projects.json` + 同目录 `projects.lock` / 临时文件；`std::fs::File::try_lock` 独占锁（第二实例只读、`CONFIG_LOCKED`）、`Mutex` 进程内串行写队列、`create_new` + `write_all` + `sync_all` + `rename` 原子替换、`revision` 过期 `CONFIG_STALE`、读取 / 损坏 / 未知版本 / 写入失败分层；重取锁后 `reload()` 重读最新 revision。`Cargo.toml` MSRV 1.77.2→1.89.0（D33），未引入新依赖。
- 实跑：`cargo test --lib` **36 项通过 / 0 失败**（既有 27 + 新增 9）、`cargo check --release` 退出码 0、`check-repo.mjs --staged` 退出码 0（本机 macOS arm64）。新增 9 项含真实子进程双实例（持锁 → 第二实例只读 → kill → 重取锁）与替换前中断（退出码 9 → 正式文件逐字节仍为完整旧版本、临时文件残留但不参与恢复），全部落在公共链覆盖的 `cargo test --lib` 内，未新增跳过开关。
- 覆盖的模型 / 存储层判据：E05 写入失败旧文件字节不变、E06 损坏与未知版本分码、E08 过期 revision 与双实例 / 中断保护；均为机器证据，不是 GUI 结果。
- **未执行、不勾 1.3**：模块尚未接入 `PageHost` / Tauri 命令与启动流程（归 1.1 / 2.x），产品启动不会创建配置文件；备份重建与 10 份上限（1.5）、只读实例完整交互、断电耐久性与 Windows / Linux 文件锁语义、真实写盘观测（D30）、Gitee 云端复验均未执行。机器 PASS 不替代人工（I11）。

### 实施轮：任务 1.1 配置模型 / 错误码 / 存储接入（2026-11-15，GUI 人工复验未执行）

> 上一节 1.3 记录的「尚未接线」为当时状态，本轮已由 1.1 接入启动与命令；历史原文保留不改写。

- 接入 `lib.rs`：`PageHost` 增 `config_store` / `config_error` / `settings` / `project_seq`；setup 在数据根可写时 `ConfigStore::open`，损坏 / 未知版本 / 不可写记录结构化原因，不自动重建、不回落。
- 新增命令：`config_state`（只发布已保存状态）、`config_create_project`、`config_rename_project`（8 个稳定错误码经 `CommandError { code, message }` 返回）、`settings_enter` / `settings_exit`（工具层铺满 + 被测层平移，与预览 / 采集互斥）、`capture_start` 拒绝编辑中采集。`settings` 进入 `relayout` 判定，缩窗仍保持铺满。
- 工具层：第一行加「项目：…」按钮与 `#settings` 面板（名称 / 起始地址 / 预设 / 保存 / 关闭 / 项目列表）；结果类反馈全走 `showStatusBar`（I15）；**徽标 `BUILD_TAG` 与 `index.html` 的 `data-build` 同轮由 `v20261114-2` 递增为 `v20261115-1`**，`check-build.mjs` 一致性断言通过。
- 实跑：`cargo test --lib` **41 项通过 / 0 失败**（既有 36 + 新增 5：预设映射、结构化错误码、camelCase 视图、稳定 ID、设置模式互斥）、`check-repo.mjs --all` 退出码 0（tsc / vite build、Rust check、徽标与夹具断言、41/41、失败传播自测）。
- **未执行、不勾 1.1**：GUI 人工复验（完整重启 + 窗口徽标 `v20261115-1`、新建项目落盘与状态栏反馈、空名 / 非法 URL 负例、编辑中采集被拒、关闭后被测页尺寸不变）；1.3 的备份重建 10 份上限、只读实例完整交互、2.2 视口应用 / 2.3 地址记忆、3.4 完整模式仲裁、4.1 徽标内容指纹、4.2 人工复验、4.3 归档合入、D30 发行轮与 Gitee 云端复验仍未执行。机器 PASS 不替代人工（I11 / I16）。

### 1.1 首轮人工复验失败与修复（2026-11-15 14:06）

- 用户完成完整重启与徽标自证、设置面板可见且能点「新建项目并保存」，但红条：`已拒绝：invalid args 'startUrl' for command 'config_create_project': command config_create_project missing required key startUrl`。⇒ **Tauri 2 命令参数名映射问题，不是配置模型 / 存储失败**；Rust 侧 `start_url` / `preset_id` 在前端应为 `startUrl` / `presetId`。
- 已修：前端 invoke 改 camelCase；`check-build.mjs` 新增三条命令参数不变量断言（Rust snake_case、前端 camelCase、不再传 snake_case 键值），把「靠记性」升级为门禁；按 1.1 纪律徽标 `v20261115-1` → **`v20261115-2`** 并同步 `index.html` 的 `data-build`。
- 实跑：`check-repo.mjs --all` 退出码 0（41/41 单测、徽标一致 `v20261115-2`、三条参数断言全绿）。上一节记录的 `v20261115-1` 为当时值，历史原文保留不改写。
- **未执行、不勾 1.1**：GUI 重跑（徽标须为 `v20261115-2`）与第 3–5 步正 / 负例、返回后页面不变的人工观察；用户尚未给出三句结论。修复的机器结果不替代人工验收（I11 / I16）。

### 1.1 重跑回显与配置文件实读（2026-11-15 14:43–14:45）

- 用户重跑后**新建项目成功两次**（「我在你发之前做过一遍，之后又做了一遍」）⇒ `startUrl` / `presetId` 修复生效，`config_create_project` → `ConfigStore::commit` → 原子替换落盘链路通；状态栏 / 按钮 / 列表反馈按用户「其他没有问题」通过。
- 应用户要求实读 `<data-root>/config/projects.json`（618 B，14:43:37）：`revision: 2`，`projects` 两条同名「本地后台」，ID 分别 `p-1794724693549-1` / `p-1794725017893-2`，`activeProjectId` 指向后建的第二个；URL 规范化为 `https://example.com/`，视口 `phone 390×844`。同目录 `projects.lock` 为 0 字节锁文件（进程持有，不是残留）。
- 两条不是重复脏数据：[design](design.md) §2 明确「同名允许，以 ID 区分」；项目删除在 [proposal](proposal.md) Scope 留后续。列表当前不显示 ID ⇒ 已登记 [debt](../../../docs/knowledge/debt.md) 欠账，归 2.1 列表细化，本轮不改前端（徽标保持 `v20261115-2`）。
- **用户明确更正：负例还没做** ⇒ 1.1 不勾选；三条负例红条、关闭返回后页面尺寸、窗口徽标 `v20261115-2` 也未单独留回显。机器门禁 41/41 与三条参数断言不替代人工（I11 / I16）。
- 若要从空配置重来：删除数据根下 `config/projects.json` 后重启即为空项目集（文件缺失不是损坏）；**本轮未代用户删除或改写任何运行数据**。

### 1.1 复验收口：勾选（2026-11-15 14:53）

- 用户确认「负例全都没有问题」：三条负例（空名称 `CONFIG_INVALID_INPUT`、危险 scheme 错因含 scheme、编辑中采集被拒）均看到预期红条；此前正例两次成功、`config/projects.json` 实读 `revision: 2` / 两个不同稳定 ID；关闭返回按用户「其他没有问题」采信，未单独留回显。
- 徽标 `v20261115-2` 窗口下的完整链路已由人工确认：设置面板铺满 → 新建项目 → 落盘 → 状态栏反馈 → 负例拒绝 → 关闭返回。机器侧 `cargo test --lib` 41/41、三条命令参数断言、徽标一致断言、`check-repo.mjs --all` / `--staged` 退出码 0。
- 结论：**任务 1.1 勾选**（范围：配置模型 / 8 错误码 / 存储接入 / 设置面板 / 徽标纪律），E07 / E09 中 1.1 相关部分标记通过；**仍不覆盖** 2.x 完整 UI（视口应用、地址记忆、项目切换）、3.4 完整模式仲裁、1.4 数据根相对路径、1.5 备份重建、4.1 徽标内容指纹、4.2 全量人工复验、4.3 归档合入、D30 与 Gitee 云端复验。
- 本会话 12 个零碎本地提交按用户要求合并为一个实施轮提交（未 push，无远程历史改写）；规划闭环与 CI 修复两个提交保留。

### 1.2 / 1.3 收口：勾选（2026-11-16，机器层证据）

- **背景**：两项此前唯一未勾理由是「未接入启动与命令」（见各自 11-15 历史节）；任务 1.1 已于 11-15 完成接线并人工复验通过，收口条件成立。本轮**未新增功能代码**，只做 allow 纪律收口：两个模块的注释均承诺「接线后删除 crate 级 `#![allow(dead_code)]`」，本轮兑现——删除后由 `cargo check` 暴露真实未接线清单（6 组警告），再按归属加窄范围 allow 并注明任务：`DataRootRelativeUnsupported`→1.4、`rotate` / `set_custom`→2.2、`set_start_url`→2.1、`set_last_loaded_url`→2.3、`expect_revision`→3.x 编排、`reload` / `try_acquire` / `revision()` / `lock_path()` 与 `lock` / `lock_path` 字段→2.x 重取锁编排与测试。后续接线时删除对应 allow，即天然的接线清单。
- **1.2 判据核对**（`project_config.rs` + 14 项单测）：schema / revision / 稳定 ID / 名称 / URL / 尺寸 / 引用校验齐备；`from_json` 先按 schema 字段分码，未知版本≠损坏；存储层 `read_config` 把 `NotFound`（空配置，不是损坏）与读取 I/O 失败（`CONFIG_READ_FAILED`）分开，测试「读取失败损坏与未知版本分码」三分支覆盖；坏 JSON / 重复 ID / 悬空引用 / 坏 URL / 坏尺寸一律判损坏不发布模型。
- **1.3 判据核对**（`config_store.rs` + 9 项单测）：根内配置文件 + 同目录锁 / 临时文件；`try_lock` 独占写锁、`Mutex` 串行写队列、`create_new` + `write_all` + `sync_all` + `rename` 原子替换；写入 / 同步 / 替换三类注入失败原文件逐字节不变且临时文件清理；过期 revision 拒绝且不推进；真实子进程在替换前中断（退出码 9）后正式文件仍为完整旧版本、重开只读正式文件；双实例持锁只读 + `CONFIG_LOCKED` + 锁释放后 `try_acquire` 重取并重读 revision。
- **实跑**（macOS arm64 / Node 22.22.2 / cargo 1.99.1）：`cargo check` 0 警告、`cargo test --lib` **41 通过 / 0 失败**、`node scripts/check-repo.mjs --all` 退出码 0。
- **单列不随勾选**：备份重建与 10 份上限（1.5）、只读实例完整 GUI 交互与只读态展示（2.x / 4.2）、断电耐久性与 Windows / Linux 文件锁语义、E05 / E06 / E08 的 GUI 与真实文件人工部分、启动红条与相对根拒绝（1.4）、真实写盘观测（D30）。机器 PASS 不替代人工（I11）。

### 实施轮：任务 1.4 解析层相对根拒绝（2026-11-16，红条 GUI 人工复验未执行）

- **实现**：`capture.rs` 的数据根解析错误改为结构化 `DataRootError`（`RelativeUnsupported { source, raw }` / `Other`）；`--data-dir` 与 `LAYRESS_DATA_DIR` 两个显式来源分别做相对性判定（`Path::is_absolute`，不查文件系统 ⇒ 根不存在也同样拒绝、不随启动工作目录漂移），相对值以 `DATA_ROOT_RELATIVE_UNSUPPORTED` 拒绝，错误消息含来源、原始值与绝对路径提示。`resolve_data_root()` 与调用方同步改型。
- **启动接线**（`lib.rs` setup）：解析失败时按错误类型设稳定码——相对路径 ⇒ `DATA_ROOT_RELATIVE_UNSUPPORTED`（本错误码首次有实现路径抛出，模块内对应 `allow(dead_code)` 已删），其他解析错误维持既有 `CONFIG_WRITE_FAILED` 口径；`config_error` / `data_root_error` / `PageEvent::DataRoot` 三通道同轮更新，采集仍由「无根拒绝写盘」与 `data_root_error` 挡住。
- **前端**（徽标 `v20261115-2` → `v20261116-1`，`index.html` 同步）：`loadInitialState` 的数据根红条改为「有根或有无根错误都点亮」——相对根拒绝时 `dataRoot` 为空但 `dataRootError` 非空，此前不显示红条，正是 spec「解析失败无根时状态栏就绪后红条可见」的缺口；红条仍排在开场提示之后不被冲掉。相对根时 `config_store` 不打开，设置面板显示「配置不可用（DATA_ROOT_RELATIVE_UNSUPPORTED）」、保存命令失败、采集被拒 ⇒ 两类失败均禁用持久化与采集，不创建替代根。
- **实跑**（macOS arm64）：`cargo test --lib` **43 通过 / 0 失败**（41 + 新增 2：显式相对根两来源分别解析层拒绝、CLI 绝对覆盖 env 相对不误拒），`node scripts/check-repo.mjs --all` 退出码 0（徽标一致 `v20261116-1`、tsc / vite、Rust check、洁净度 / 链接 / 双语、失败传播自测）。
- **单列不勾 1.4**：红条实际可见性（三档：相对 CLI 根 / 相对 env 根 / CLI 绝对覆盖）待用户按 [app README](../../../apps/desktop/README.md)「任务 1.4 人工复验」三句确认；Windows 盘符路径在 Unix 解析层的表现属平台差异，随 Windows 实测单列；debt「相对路径解析基准」解析层部分已销账、红条部分待复验。

### 1.4 复验收口：勾选（2026-11-16 09:21，用户实机三档）

- 数据源：用户按 [app README](../../../apps/desktop/README.md)「任务 1.4 人工复验」三档实机操作后确认「都没问题」：① 相对 CLI 根红条可见且判据齐备（`DATA_ROOT_RELATIVE_UNSUPPORTED` + 原始值 + 绝对路径提示，不被开场提示冲掉）；② 相对 env 根来源正确；③ CLI 绝对覆盖 env 相对不误拒。窗口徽标按清单为 `v20261116-1`。
- 结论：**任务 1.4 勾选**（范围：解析层相对根拒绝、优先级、无根红条可见、app README / I13 同步、debt 解析层销账）。E05 标「相对根解析层 + 红条可见性通过」，E07 标「无根红条启动可见通过」。
- **仍不覆盖**：1.5 备份重建与 10 份上限、2.x 视口 / 地址 / 项目切换 UI、3.x 编排与 3.4 仲裁、4.1 徽标内容指纹、4.2 全量人工复验、4.3 归档合入；不可写有根路径沿归档 change 4.1 已人工复验的证据，不在本项重复执行；Windows 盘符路径表现随 Windows 实测单列；D30 与云端 CI 状态不变。机器门禁与单测不替代人工（I11），本轮人工结论依据用户回显。
