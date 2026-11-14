# engineering.md · 工程与门禁的坑

> 本文默认以中文维护。

> 条目格式与升级规则见 [`README.md`](README.md)。**未经 Layress 实测的条目一律标注来源，不当作本仓已验证事实。**

## 环境、行尾与本机配置

- [2026-11-09][P1] 接手时无法启动仓库门禁，用户说明此前依赖 WorkBuddy 自带 Node ⇒ 应用内置运行时未暴露给当前会话 PATH，不能据此判定整机未安装 ⇒ 开发环境使用独立安装且位于公共 PATH 的 Node ≥ 20，并在各工具会话实跑门禁确认；历史全绿不得替代本轮结果 ⇒ 证据：首次调用返回 `command not found: node`（退出码 127）；用户独立安装后当前会话识别 Node v26.13.0 / npm 11.20.0（macOS arm64），`node scripts/check-repo.mjs --all` 全绿，[`AGENTS.md`](../../AGENTS.md) §二已同步恢复状态。
- [2026-11-03][P0] Windows 上格式检查满屏误报，仓库内容其实是好的 ⇒ 系统级 `core.autocrlf=true` 把工作区检出成 CRLF，而 linter（Biome/ESLint）要求 LF ⇒ 用 [`.gitattributes`](../../.gitattributes) 强制文本 `eol=lf` 并给二进制/夹具标 `-text` ⇒ 来源：前置项目 ReBaseAgent 全仓排查一次；**Layress 尚未实跑验证**（macOS 上系统级 autocrlf 一般不生效，但克隆来的全局配置仍要查）。
- [2026-11-08][P1] 门禁的「机器绝对路径」与「系统用户目录」规则只按 Windows 形态写，开发机迁到 macOS 后 `/Users/<name>/…`、`~/Library/…` 完全不判红 ⇒ 判据是平台函数而非环境常量，换开发机必须回头查规则覆盖面 ⇒ 三端形态并列写进 `check-hygiene.mjs`，补 macOS Library / Linux XDG / POSIX 绝对路径三个反证用例，由 `--selftest` 证明确实判红 ⇒ 证据：本轮 `check-repo.mjs --all` 自测回显。
- [2026-11-09][P1] 迁机 macOS 后仍把「自有、已打包、开发冻结的 Electron 应用」当可复用基线，用户确认该应用**没有 Mac 版本** ⇒ D16 只改了「首验实机平台」口径，没核基线对象是否随开发机可用；「自有 + 已打包 + 冻结」不等于「当前机器上跑得起来」 ⇒ 迁机 / 换机必须同时核三件事：基线对象可启动性、探针栈可用性、证据原件是否随机器迁移；不可用时把对象与平台**一起**列欠账，不默认 Mac 能跑、也不假装 Windows 仍成立 ⇒ 证据：用户 2026-11-09 确认；[`debt.md`](debt.md)「首验实机平台口径」行、[`../engineering/plans/2026-11-07-agent-native-evidence-gate.md`](../engineering/plans/2026-11-07-agent-native-evidence-gate.md) §五。
- [2026-11-08][P1] 文档状态卡里写死 `git fetch github && git log --oneline github/main..main`，换克隆直接报错 ⇒ 远程名是**本机 git 配置**而非仓库事实：当前 Mac 克隆只有单个 `origin`（Gitee fetch/push + GitHub push URL），`github`／`gitee` 这两个名字并不存在 ⇒ 查未推提交一律用不依赖远程名的 `git fetch --all && git log --oneline '@{u}'..main`，文档注明「远程名以 `git remote -v` 为准」；远程配置归用户，智能体不改 ⇒ 证据：本轮 `git remote -v` 回显与状态卡命令失效；同一根因下 `git push github main && git push gitee main` 同样失效，工作流已一并改为先查 `git remote -v`。
- [2026-11-10][P2] 整目录移动 git 仓库后，当前会话的 shell 立刻报 `Working directory does not exist`，而文档读写工具仍正常 ⇒ Bash 类工具的 cwd 绑定会话启动时的工作区绝对路径，IDE 工作区与本机记忆索引同样按路径绑定；`.git` 内部全为相对路径，仓库本身无损，失效的是仓库外的本机绑定 ⇒ 移动后先在新路径实跑 git 与门禁，并在 IDE 重新打开工作区；会话内还要跑 shell 时临时让旧路径可进入、收尾删净，本机日志写入改指新路径 ⇒ 证据：2026-11-10 目录归位实跑回显、D27 与[`AGENTS.md`](../../AGENTS.md) §二。
- [2026-11-11][P2] 当前会话 PATH 里的 Node 来自 WorkBuddy 内置运行时（`~/.workbuddy/binaries/node/…`，v22.22.2）⇒ 应用自带运行时随应用更新 / 卸载而变，构建链会静默依赖它 ⇒ 有独立构建需求时改用独立安装并固定版本，门禁 `engines.node >=20` 只兜版本不兜来源 ⇒ 证据：2026-11-11 `node -v` 与 `which npm` 回显；与 2026-11-09 条目同一主题。
- [2026-11-11][P2] rustup 镜像安装：rsproxy 的 `rustup-init.sh` 脚本下载 rustup-init 报 404，直连二进制时又遇 504，脚本路径与镜像稳定性都不可假设 ⇒ 直接下载 `rustup/dist/<target>/rustup-init` 二进制再运行，镜像换 USTC（`mirrors.ustc.edu.cn/rust-static`）成功；crates 索引用 rsproxy sparse 另配 ⇒ 证据：2026-11-11 安装回显，rustc/cargo 1.99.1 就绪，`cargo check` 通过。
- [2026-11-11][P1] corepack 首次调用 pnpm 会交互确认下载，非交互会话直接挂起直到超时被杀 ⇒ 设 `COREPACK_ENABLE_DOWNLOAD_PROMPT=0` 后一次性放行 ⇒ CI / 智能体等非交互环境启用 corepack 必须带该环境变量 ⇒ 证据：2026-11-11 首次调用挂起（SIGTERM），设置后成功启用 pnpm 12.18.0。

## 门禁

- [2026-11-03][P1] `.gitignore` 拦不住垃圾 ⇒ 它是默认放行机制，一个 `git add -f` 就能把数百 MB 的二进制写进历史且删不干净（前置项目的运行时目录里实际躺着一个 374 MB 安装包）⇒ 拒绝逻辑必须由 `scripts/check-hygiene.mjs` 执行并在 pre-commit + CI 双跑 ⇒ 证据：本仓 8 项门禁实测（垃圾场路径/体积/`.env`/凭据/无理由豁免/带理由豁免/忽略泄露）。
- [2026-11-03][P0] CI 显示全绿但门禁根本没跑 ⇒ Gitee Go 的 `commands` 逐条执行、**单条失败不短路**，前面红了整体仍判绿；且 `build@nodejs` 插件可能静默跳过自带的 Node 安装，容器里没有可用 `node` ⇒ 所有校验用 `&&` 链成**一条**命令，由最终退出码判败；Node 在命令里自装，`nodeVersion` 字段只当必填占位 ⇒ 来源：前置项目首跑实证，本仓 [`.workflow/ci.yml`](../../.workflow/ci.yml) 已按此写。
- [2026-11-03][P1] 豁免清单会变成考古现场 ⇒ 提交进去的 glob 后来没有一个文件命中，没人敢删 ⇒ `check-hygiene.mjs --all` 把零命中的豁免报成警告，删掉即可 ⇒ 只在 `--all` 判定，staged 模式本次提交没碰到该文件属正常。
- [2026-11-08][P0] 开发机迁到 macOS 后 pre-commit 门禁形同未挂 ⇒ `.githooks/pre-commit` 在索引里的 mode 是 `100644`，Windows 不依赖执行位，Git on macOS / Linux 直接静默跳过（commit 时只给一行 ignoredHook 提示），本地洁净度链只剩 CI 兜底 ⇒ 改为 `100755` 入库，并在 `check-hygiene.mjs` 加「hook 缺可执行位」断言 + 反证用例；**新增 hook 一律核对索引 mode** ⇒ 证据：`git ls-tree HEAD .githooks/pre-commit` 原为 100644，修复后 `git hook run pre-commit` 实跑通过。
- [2026-11-11][P1] pnpm ≥11/12 下 `package.json` 的 `pnpm.onlyBuiltDependencies` 与 workspace 旧键 `onlyBuiltDependencies` 不再授予构建权限，`ERR_PNPM_IGNORED_BUILDS` 反复出现 ⇒ 授权表改用 `pnpm-workspace.yaml` 的 `allowBuilds` 映射（`esbuild: true`）；且 pnpm 会自动写入 `allowBuilds: esbuild: set this to true or false` 占位段，与手写段冲突报 duplicate key ⇒ 首次安装报错后检查 `pnpm-workspace.yaml` 是否被写入占位键，合并为单个 `allowBuilds` 段再重装 ⇒ 证据：2026-11-11 三次 install 失败到修复成功的回显，esbuild postinstall 正常执行。
- [2026-11-11][P1] 图片文件（截图、过程产物）此前不在 `check-hygiene.mjs` 后缀黑名单，工作区里任何 `.png` 都能被 add 进索引 ⇒ 用户 2026-11-11 明确口径：只放行必要图片，过程性的要拦 ⇒ `FORBIDDEN_EXT` 增加 `.png/.jpg/.jpeg/.gif/.bmp/.webp/.ico/.icns/.tiff`，确需入库的图片（Tauri 应用图标）逐条在 `.git-hygiene-allowlist` 写理由豁免，并补「图片后缀反例」自测 ⇒ 证据：本轮 `check-hygiene.mjs --selftest` 与 `check-repo.mjs --all` 回显；`docs/reviews/**` 与 `artifacts/**` 的截图仍被 `.gitignore` 双重拦截。

## 测试

- [2026-11-03][P0] 并行跑测试出现一片红，逐包复跑全绿 ⇒ 并行争用端口/临时目录/单例服务 ⇒ 判回归只认单包复跑与串行全链，并行结果不作证据 ⇒ 来源：前置项目；**Layress 建测试链前先按此约定**。
- [2026-11-03][P1] 「全量绿」无法核对 ⇒ 只报"通过"没有可比对的量 ⇒ 全量结果必须留三元组：**文件数 + 用例数 + Errors 行**，回归判定时逐项对齐；计数小幅波动（宿主 shim 影响临时目录）以"已收集全过 + Errors=0"为准，不追求逐位精确。
- [2026-11-03][P1] 没有 jsdom 时写出"镜像断言" ⇒ 测试只是把实现重抄一遍，真缺陷照样绿 ⇒ 无 DOM 环境时只写**源码结构断言**并显式标注它不覆盖运行时行为 ⇒ 来源：前置项目；Layress 若走 Node 侧检测模块同样适用。
- [2026-11-08][P1] 盲测执行端的提示词与协议里写「你是执行者不是评审者」「只记录观测不做判断」「不要推测实验目的」，独立性从输入端就破了 ⇒ 角色框架暗示有人在设计与评估结果，**否定式禁令会把被否定的概念主动递给执行端**；协议标题/schema 描述里的「实验」「盲」「结论」同样泄漏 ⇒ 提示词只留正面任务描述（读哪几份文件、按协议跑、按 schema 写结果），约束改由 schema 的字段结构承载（没有 verdict 字段就没有结论可写），协议与 schema 全量改为正面陈述 ⇒ 证据：用户 2026-11-08 指出，本轮收紧后盲测目录内元信息词扫描 0 命中。

- [2026-11-08][P1] 夹具里顶层 `var status = document.getElementById('status')` 之后 `status.textContent='fired'` 永远不生效，`#status` 保持 "idle" ⇒ `window.status` 是浏览器遗留全局属性，赋值会被转成字符串，后续属性写入静默失败 ⇒ 观测变量一律用不易冲突的命名（如 `statusEl`），并至少提供两个独立观测通道（`window.__fired` + DOM 文本）⇒ 证据：本轮盲测执行端在四个页面一致观测到该现象并如实记录，改用 `window.__fired` 后结论不受影响。
- [2026-11-08][P1] 夹具只做静态结构自检（有 `#target` / `#status` / listener / 无注释）就放行，运行时才发现观测通道失效 ⇒ 静态断言只证明"写了这些字"，不证明浏览器里行为正确；本机当时无浏览器内核，于是既没做运行时验证，也没把它列为未执行项 ⇒ 无运行环境时把运行时验证**单列为未执行项**并在交付物里注明，别把静态 PASS 写成验收通过（I11）⇒ 证据：本轮 `artifacts/blind-run-gate-check.md` 与执行端 notes。
- [2026-11-08][P1] 协议写「在其所在容器内滚动使其可见」仍不足以让执行端一次判对 ⇒ 只说目标状态、没给容器内裁剪的可操作判据，第一轮 B 页漏判、命中 `html`，执行端自行修正后才重跑 ⇒ 协议里的准备步骤要给出可机械核对的可见性条件（如目标边框盒与滚动容器可视区的关系）⇒ 证据：执行端转录与 `artifacts/blind-run-gate-check.md`。

## 断言字面量

- [2026-11-03][P0] 断言里的字面量永远红或永远绿，且看不出原因 ⇒ 直接在参数里写非 ASCII 字符（`U+FFFD`、emoji）会被工具链吞成空串，于是「包含某非法字符」的断言变成「包含空字符串」= 恒真 ⇒ 非 ASCII 断言参数一律写 `\uXXXX` 转义，并在自测里验证转义本身生效 ⇒ 来源：前置项目实证。

## 存储与便携（便携优先 · 不写系统用户目录）

- [2026-11-10][P1] 写盘观测无事件容易被误写成便携通过 ⇒ 观测链可能没捕获事件、漏掉系统 WebView 辅助进程或无法排除背景写入 ⇒ 先以隔离夹具根外写入验证检测链，再对真实应用记录进程 / 时间归因、前后对照与覆盖盲区，证据不足标不确定 ⇒ 证据：[review 的 E10 意见](../../openspec/changes/add-tauri-page-host/review.md)与[已补证据计划](../../openspec/changes/add-tauri-page-host/evidence-index.md)，实际写盘验证尚未执行。
- [2026-11-10][P1] WKWebView 不支持 `data_directory`，`data_store_identifier` 也不是任意路径，非持久模式不证明无运行期写入 ⇒ 把跨平台 API 名称当作一致的存储能力会误判便携通过 ⇒ 首个 change 分别记录平台限制、非持久会话设置和实际写盘观测，不静默使用系统持久目录 ⇒ 证据：[首个 change 设计与官方 API 核查](../../openspec/changes/add-tauri-page-host/design.md)，仅文档核对，尚未实机验证。
- [2026-11-10][P1] 桌面载体转 Tauri 后仍容易套用统一 Chromium 与 Electron 路径覆盖假设 ⇒ Tauri 复用平台 WebView，引擎、控制与存储路径不同 ⇒ 按实际引擎验证视口 / 截图 / 模拟，另核会话、缓存与临时目录，未验证不声明便携或真机兼容 ⇒ 证据：D25、[阶段计划的技术依据](../product/2026-11-10-stage-one-plan.md)与官方 Tauri 架构 / WebView 文档；本项目尚未实测。
- [2026-11-03][P0] 号称便携的产品仍在系统用户目录长垃圾 ⇒ 只把"数据目录"挪到产品旁不够：Windows `os.tmpdir()` = `%TEMP%` = `C:\Users\<user>\AppData\Local\Temp`，macOS 落在 `/private/var/folders/…`，Linux 落在 `/tmp` 或 XDG 缓存；Electron `app.getPath('userData')` 与 Chromium `sessionData`/`Cache`/`GPUCache` 三端默认同样落在系统用户目录 ⇒ 三处必须一起覆盖：`app.setPath('userData', <数据根>/userdata)`、`session.setPath('sessionData', ...)`、自建临时目录并显式传给 `--user-data-dir`；**禁止把 `os.tmpdir()` 当默认值用** ⇒ 来源：前置项目 `.gitignore` 已记录"一次性工作目录（含 Electron sessionData 缓存等临时文件）"这一坑；**Layress 待实测确认**。
- [2026-11-03][P0] 目标目录不可写时静默回落到用户目录 ⇒ 常见写法是"写不进就 `%APPDATA%` / `~/Library` / XDG 兜底"，直接违反不变量 I12，且用户完全不知道数据去了哪 ⇒ 启动做可写探测；不可写就明确报错并要求 `--data-dir`，**不允许回落**。
- [2026-11-03][P1] 用户无法验证数据落点 ⇒ 解析出的数据根从不出现在输出里 ⇒ 启动横幅、日志与报告头部都要写 `resolvedDataDir` 与可写性检查结果（I13）。
- [2026-11-03][P1] 便携包换机器/换盘符后证据路径失效 ⇒ 配置、报告与数据集里存了 `D:\...` 或 `/Users/<name>/...` 这类绝对路径 ⇒ 仓库与产物内一律相对数据根、用 posix 分隔符；只在导出给用户时才物化成绝对路径。门禁的「机器绝对路径」检查三端形态都收，就是为这条兜底。
- [2026-11-03][P1] 装在只读介质（U 盘、光盘）或被安全软件管控的目录时崩溃 ⇒ 没有降级路径 ⇒ 明确支持"只读产品目录 + 用户指定可写数据根"这一组合，并在文档里写清。

## 依赖与产物

- [2026-11-10][P1] 共享安装只指定 npm prefix 仍可能让 OpenSpec 配置、缓存和临时资料落入系统用户目录 ⇒ 安装前缀与 CLI 的 XDG / 临时目录是不同控制面 ⇒ 共享启动入口统一设置 XDG、TMPDIR、npm cache / logs，并关闭遥测与自动更新，项目内只留调用说明 ⇒ 证据：OpenSpec 1.17.0 安装源码核对、实际共享入口与[工作流](../development/workflow.md)。
- [2026-11-10][P2] npm 11.20.0 同时把 userconfig 和 globalconfig 指向 `/dev/null` 时安装报 double-loading config ⇒ npm 不接受两个配置层复用同一文件路径 ⇒ 在授权工具根提供两个独立配置文件，再显式指定 prefix / cache / logs ⇒ 证据：本轮首次安装报错与随后 57 个包安装成功，OpenSpec `--version` 输出 1.17.0。
- [2026-11-10][P1] OpenSpec `status` 返回 `isComplete: true` 容易被当成实施完成 ⇒ 该命令描述规划文档状态，任务未必已勾选 ⇒ 以 tasks 与实机证据判断实现，规格校验与规划完成分别记录 ⇒ 证据：[首个 change 校验记录](../../openspec/changes/add-tauri-page-host/evidence-index.md)，本轮实施任务仍全部未执行。
- [2026-11-11][P2] 首个 `pnpm install` 与 cargo 依赖解析本身无额外坑（锁文件一次成型，`Cargo.lock` 107 KB 在体积门禁内）；依赖构建审批坑见上方门禁节 `allowBuilds` 条目，cargo 镜像见环境节 rustup 条目 ⇒ 首次引入 workspace 的解析结论：默认 registry（npmjs）与 rsproxy sparse 均可直连，暂不需仓库级镜像配置 ⇒ 证据：2026-11-11 `pnpm install` / `cargo check` 退出码 0。

## 知识库与交接

- [2026-11-09][P1] 产品与路线图已换成界面验收助手，README、贡献 / 安全说明、模块索引及 config 仍保留旧四模块与平台承诺 ⇒ 仅更新讨论入口，未核查现行门面和执行约束 ⇒ 收工提交前逐项同步中英门面、表单、模块归属、config 和接手状态，历史研究保留并标时效 ⇒ 证据：本轮一致性复核，现行依据见[产品](../product/product.md)、[路线图](../product/roadmap.md)与[文档地图](../README.md)。
- [2026-11-09][P1] 产品讨论从工作台连续退到专项规则与单一技术接入，尚未核查剩余任务是否高频 ⇒ 把实现成本、样例范围与目标受众混成同一个维度 ⇒ 先按日常任务、已有替代和采用成本比较；首轮测试环境不自动等于产品永久范围，也不以支持平台数量证明需求广度 ⇒ 证据：用户担心“越做越小众”，[受众与常用流程调研](../research/2026-11-09-audience-and-workflow-research.md)与 D21。
- [2026-11-09][P1] “重新开始”被理解为不读旧想法，单次遮罩任务的反证又容易被扩大成整个方向无价值 ⇒ 历史执行状态、可引用材料与实验外推边界未分开 ⇒ 旧材料可以查，用户新目标写入新入口；引用反证同时写清工具、任务与场景，按新授权推进，不恢复旧闸门 ⇒ 证据：用户本轮澄清、[新阶段讨论](../product/2026-11-09-agent-ui-review-direction.md)与 D20。
- [2026-11-08][P1] 一轮改造踩的坑只落进 `.workbuddy/memory/`，下个 Agent 与 clone 仓库的人一个字都看不到 ⇒ 该目录被 `.gitignore` 排除，只是本机当日流水；前置项目 90% 经验就是这样丢的（D03）⇒ 开发中的问题**同轮回填** `docs/knowledge/` 对应专题，本机日志只留流水与索引；纪律写进 [`README.md`](README.md)「回填纪律」、`AGENTS.md` §六与提交流程自检 ⇒ 证据：本轮迁移改造后经用户指出才补齐回填。
- [2026-11-08][P1] 以为"新对话 + 空白工作空间"就能让执行端完全无上下文，结果它开口称「老大」，还主动复用了 `~/Developer/tools/ui-obs/` 与浏览器默认共享缓存 ⇒ 用户级 `SOUL.md`／`MEMORY.md` 是**跨工作空间注入**的，空白目录只挡得住项目级 `AGENTS.md`，挡不住用户级身份锚点与长期规则 ⇒ 严格盲测要么换账号／产品／API 直连，要么在判据里显式登记"独立性受限"这一层，不能把"无上下文"当已成立前提；本轮注入内容不含假设，故不影响"有没有假绿"的观测 ⇒ 证据：B 轮执行端原始回复与 `SOUL.md` 身份锚点行、用户级 `MEMORY.md` 第 7–8 行逐字对应，见 `artifacts/blind-task-gate-check.md`。
- [2026-11-09][P1] D18 收口已写进 `decisions.md`／`debt.md`／状态卡，但 `roadmap.md`「当前接手从哪里开始」与综合调研 §十 仍写着「下次第一步：整理真实任务卡」，接手人会按旧指引重启已被停止的探针开发 ⇒ **收口类决定只改了结论落点，没同轮更新全库所有「当前接手 / 下次第一步」入口**；文档权威性排序靠后，但入口是接手人第一个读的东西 ⇒ 凡写「停止 / 收缩 / 暂停」的决定，同轮列出全库接续入口清单（状态卡、路线图、停点节、提案状态块、目录索引）逐一标时效，旧指引保留但注明「收口前口径、不再执行」 ⇒ 证据：2026-11-09 接手复核发现的口径分叉，本轮已逐处对齐。

## 采集与预览实现（任务 3.x，2026-11-12）

- [2026-11-12][P1] title 哨兵上报的 JSON 键名写 `sx`/`sy`，Rust 结构字段是 `scroll_x`/`scroll_y`，实测视口 / DPR 都有值、滚动条件却一直被判「未取得」 ⇒ serde 对未知键静默忽略，而「缺失信息如实标注」的设计恰好把这类实现 bug 伪装成合理缺失 ⇒ 键名统一，并加单测断言哨兵脚本包含全部字段名且示例 JSON 能反序列化（第 6 项 `cargo test --lib`）；凡「单向通道 + Option 字段」都要有键名一致性断言 ⇒ 证据：三次真机探针元数据对比与 [change 设计](../../openspec/changes/add-tauri-page-host/design.md)。
- [2026-11-12][P1] `initialization_script` 在文档早期写 `document.title` 会被页面自身 `<title>` 覆盖，首轮不改视口时拿不到页面实测值 ⇒ 哨兵只在初始化 / resize 时刻上报，HTML 解析完成后标题已变 ⇒ 在 `DOMContentLoaded` 与 `load` 再各上报一次（同时在 `scroll` 上报）；页面脚本后续自行改标题仍是已知限制 ⇒ 证据：首轮探针元数据 `measured: null`，补报后 `1280×704 @DPR 2` 与 session 均取得。
- [2026-11-12][P2] async 命令里 `let x = { let state = app.state::<T>(); state.lock()?.method() };` 报 E0597「state 存活不够久」 ⇒ 块尾表达式的 `?` 产生临时 `ControlFlow`，其析构晚于 `State` 局部变量 ⇒ 把锁与结果存局部变量再返回（`let ok = s.is_current(..); ok`），或把语句拆成独立函数并在 `if let` 末尾补 `;` ⇒ 证据：2026-11-12 `cargo check` 连续 4 处同类报错与修复回显。
- [2026-11-12][P1] 直接用 objc2 调 WKWebView 原生方法时，依赖版本必须与 wry 的间接依赖对齐 ⇒ objc2 / block2 的类型按 crate 版本区分，版本不一致会让 `DynBlock` 等类型不匹配（或编译期难查的 ABI 风险） ⇒ `Cargo.toml` 里固定 `objc2 = 0.6.5` / `block2 = 0.6.2`（与 wry 0.57.2 一致）并放在 `[target.'cfg(target_os = "macos")'.dependencies]`，非 macOS 明确返回「未实现」而非降级 ⇒ 证据：本机 registry 源码核对与 `cargo check` 退出码 0。
- [2026-11-12][P2] 本机 `cargo` 不在 WorkBuddy 会话的 PATH 里，直接 `cargo check` 报 command not found ⇒ rustup 装在 `~/.cargo/bin` 但会话 PATH 未包含 ⇒ 所有 Rust 命令显式加 `PATH="$HOME/.cargo/bin:$PATH"`，文档命令也写明；这不是工具链未安装 ⇒ 证据：2026-11-12 首次编译失败回显与后续成功。
- [2026-11-12][P1] 采集失败 / 失效提示「没出现」，用户会以为逻辑根本没跑 ⇒ 提示只写在与视口信息共用的 `#status-line`，长文本被 flex 挤压省略、又被导航的 `loading / loaded` 事件覆盖，旧请求响应还被 `captureSeq` 静默忽略 ⇒ 结果类反馈必须有**独立常驻区**（`#capture-toast`，只由采集阶段更新、`title` 存全文、旧请求也留痕），状态分层不能被普通事件冲掉 ⇒ 证据：用户 2026-11-12 反馈与 [change 证据](../../openspec/changes/add-tauri-page-host/evidence-index.md)。
- [2026-11-12][P1] 新增 DOM 元素用模块顶层 `assertElement` 获取，HTML 与 JS 版本不同步时整个工具界面脚本崩掉 ⇒ Vite HMR 可能只推 JS 而旧窗口仍是旧 HTML，顶层断言抛异常后 `bindUi` / `listen` 全不执行，表现为按钮无反应、状态行空白 ⇒ 新增元素一律 `getElementById` 可选获取并判空降级，启动时检测缺失元素给出「Ctrl+C 完整重启」指引；只有核心骨架元素才用强断言 ⇒ 证据：同上与 [app README](../../apps/desktop/README.md) 复验节。
- [2026-11-12][P1] 前端 HTML/JS 改完、用户「已经重启」后新 UI 仍不出现（连续两轮反馈状态行 / 黄色结果条都没有） ⇒ 窗口实际启动时间早于改动；`tauri.conf.json` 的 `app.security.csp = default-src 'self'` 会挡掉 Vite dev 的 HMR websocket，`@vite/client` 推不出 full-reload，工具层 WebView 一直保留旧 DOM；dev 模式走 devUrl、二进制内不嵌前端资源（`grep` 0 命中），排除二进制缓存 ⇒ 每次改前端必须**杀净 pnpm / tauri / vite / 应用进程后起完整 dev**，不能只信「我重启过」或 HMR；验证用 `curl devUrl` 与磁盘 / dist 比对元素标记，并核对 PID 是否晚于改动时间 ⇒ 证据：2026-11-12 三次用户反馈、Vite 返回与 dist 均含 `capture-toast` 而旧窗口无该元素、重起后 Vite ready + `Running target/debug/layress-desktop`。
- [2026-11-12][P1] 采集结果条连续两轮「看不见」，既不是 HTML 版本也不是 z-index ⇒ 工具层 WebView 的 Rust bounds 为 Logical(1280×96)、窗口 scale=2，但页面实测 `window.innerHeight=64`；`#bar` CSS 高度 96px 且 `overflow:hidden`，任何 `top:96px` 或锚在 `#bar` 底部（absolute bottom:0）的元素都落在可见区之外被裁 ⇒ 反馈条一律 `position:fixed; bottom:0`（贴**视口**底，不锚容器），并用开发期 `LAYRESS_PROBE_TOOL=1` 对工具层自身 `takeSnapshot` 截图自证可见，不再让用户当唯一测试员 ⇒ 证据：2026-11-12 探针日志 `toast=true status=true innerH=64` 与 `tool-probe.png` 中清晰可见的黄色诊断条。
- [2026-11-12][P2] 共享 OpenSpec 入口在工作流与证据记录里仍写 `../tools/openspec/openspec`，照抄会 command not found ⇒ D27 把仓库归位到 `~/Developer/project/layress/` 后，工具根 `~/Developer/tools/` 相对层级由一级变两级，文档没跟着改 ⇒ 现行入口统一为 `../../tools/openspec/openspec`，并注明旧路径只描述归位前布局；**目录归位类决定必须同轮检查所有相对路径命令** ⇒ 证据：2026-11-12 `../../tools/openspec/openspec validate --strict --json` 实跑 1 通过 / 0 失败，[工作流](../development/workflow.md)与 [evidence-index](../../openspec/changes/add-tauri-page-host/evidence-index.md) 已同步。
- [2026-11-12][P2] 发给用户的启动命令在真终端连续两次失败（先 `cd apps/desktop` 找不到目录，再 `command not found: pnpm`） ⇒ 命令默认了沙箱的当前目录与 PATH：仓库在 `~/Developer/project/layress/`，`pnpm` 是 WorkBuddy 内置 Node 22.22.2-6 的 corepack shim，真终端只有系统 node 26 / npm，`cargo` 也在 `~/.cargo/bin` ⇒ 给用户的命令一律带**绝对路径 cd + 显式 PATH 前缀**（同时前置 WorkBuddy Node bin 与 `~/.cargo/bin`），并先用 `env -i` 干净 PATH 实跑 `pnpm tauri --version` 验证后再发 ⇒ 证据：2026-11-12 两次终端报错与干净环境回显（node 22.22.2 / pnpm 12.18.0 / cargo 1.99.1 / tauri-cli 2.12.4），命令已写入 [app README](../../apps/desktop/README.md)。
- [2026-11-12][P2] macOS 上用 `grep -c 'bar.pending\|#fff8e1' file` 验证内容返回 0 命中，差点把「新版已生效」误判成「未生效」 ⇒ BSD grep 的 BRE **不支持 `\|` 交替**（GNU 扩展），模式被当成字面量 `|#` 匹配 ⇒ 本机核对内容用 `grep -E 'a|b'` 或分两次单模式；另验证编译产物时字符串字面量**不含源码引号**（查 `"pending"` 永远 0 命中，应查 `pending`），且 tauri dev 的 watcher 会在保存 src-tauri 源码后自动重编二进制，`cargo run` 显示 0.1s Finished + 无 Compiling 行不等于代码没进二进制，以产物 grep / mtime 为准 ⇒ 证据：2026-11-12 状态栏 pending 档验证中两次假阴性、改 `grep -aoc 'pending'` 后 18 命中，Vite 返回内容肉眼可见新样式。

## 导航负例与地址校验（任务 2.2，2026-11-13）

- [2026-11-13][P1] 无效地址负例的「已拒绝」提示只写工具层 `#status-line`，与 3.x 采集结果条踩的是同一个根因（工具层 CSS 视口 64px、`#bar` 96px + `overflow:hidden` 裁掉第三行）⇒ 同一坑在本 change 内第二次命中，机器逻辑正确但 GUI 上完全看不见，人工必然反馈「什么都没出现」⇒ 结果类反馈统一走 `showStatusBar` → `status_bar_set` → 底部独立状态栏，并把约定升级为不变量 I15；工具层剩余直接赋值列进 debt ⇒ 证据：本文件 2026-11-12 第 77 / 80 行、[change 证据](../../openspec/changes/add-tauri-page-host/evidence-index.md)、本轮 `pnpm build:desktop` 退出码 0。
- [2026-11-13][P1] `Url::parse("http:///path")` 把路径段 `path` 规范化成主机名，`validate_url` 的 host 非空检查放行 ⇒ 畸形地址靠后续 DNS / 连接失败「碰巧」拒绝，mDNS 可能把该名字解析成内网主机，边界不可靠 ⇒ 解析前按原始串核对 authority 段存在且不以 `/` 开头；新增 4 项单测覆盖无效地址、scheme 大小写规范化、关闭端口与监听端口正反样本 ⇒ 证据：`cargo test --lib` 首轮 1 失败（`http:///path` 通过校验）→ 修复后 10 项全通过。
- [2026-11-13][P2] 可达性预检的 `to_socket_addrs()` 是阻塞 DNS，不受 `connect_timeout`（3 s）约束 ⇒ 不可达域名可能让「正在校验地址」停留远超 3 s，被误读为卡死 ⇒ 人工复验优先用 `127.0.0.1:1` / `192.0.2.1` 这类不依赖 DNS 的地址；DNS 超时覆盖仍是 5.x 前欠账 ⇒ 证据：`lib.rs::probe_reachable` 与单测 `probe_reachable_rejects_closed_port`。
- [2026-11-13][P1] 用户人工复验发现 `file:///etc/passwd` 报「地址缺少主机名」而不是「scheme 不受支持」⇒ 上一条新增的 authority 前置检查抢在 scheme 白名单之前执行，先按路径形状判错，把危险 scheme 拒绝伪装成格式问题，听起来像补上主机名就能打开本地文件 ⇒ authority 检查限定为仅 http(s) 生效，并加单测断言 file / ftp / data 的错因必须含 scheme、不得含主机名，http(s) 畸形地址仍报缺少主机名；**拒绝理由必须归因到真正的边界**，只断言「返回 Err」的单测会漏掉这类错因失真 ⇒ 证据：用户 2026-11-13 复验原话、`cargo test --lib` 11 项全通过、`reject_reason_keeps_scheme_before_host_check`。
- [2026-11-13][P1] 开发态直接做「便携写盘观测」⇒ 便携优先（I12）的载体是打包后的产品目录；`pnpm tauri dev` 下 exe 在 `src-tauri/target/debug/`、数据根在 `target/debug/.layress/`，旁边混有 Vite / pnpm / cargo / WKWebView 辅助进程写入，既没有「产品目录」可删，也无法按 E10 把写入归因到产品本身 ⇒ 这类判据在开发态不是「观测不完整 / 不确定」，而是**前置条件未满足 / 未执行**；本 change 不搭观测脚本、不勾 4.2、不宣称便携，真实观测挂到打包后的发行轮并登记为未决口径；15:49 用户选方案 A（D30）后已定夺为「正式移出本 change 范围」，不再作为本 change 欠账 ⇒ 证据：用户 2026-11-13 15:15 质疑、本机 `sudo -n` 被沙箱禁止（`fs_usage` / `dtrace` 需 root）、`pgrep` 显示的 dev 进程树、D30。
- [2026-11-13][P2] 给用户的 `--data-dir` 启动命令写成 `pnpm tauri dev -- --data-dir=…` ⇒ Tauri CLI 2.12.4 的 `dev` 把第一个 `--` 之后的参数交给 runner（cargo），只有**第二个** `--` 之后才传给应用二进制（`tauri dev -- [runnerArgs] -- [appArgs]`），参数会被 cargo 吃掉甚至报错 ⇒ 应用参数一律写 `pnpm tauri dev -- -- --data-dir=…`；不依赖透传时改用 `LAYRESS_DATA_DIR` 环境变量；**发给用户的 CLI 命令先查 `--help` 的 Arguments 段** ⇒ 证据：`pnpm tauri dev --help` 回显 Usage / Arguments，双 `--` 形式尚未在真终端实跑，人工复验时一并核对。
- [2026-11-13][P2] 一次性脚本批量改文档时，第二段替换仍基于旧 `t` 快照又 `write_text` 一次，把前段替换结果覆盖回去 ⇒ 门禁只查洁净度 / 链接 / 双语，查不出「内容被还原」，`git diff --stat` 也只有少量行变化 ⇒ 批量改动必须在同一变量上累积、**只落盘一次**，落盘后用 `grep` 复核新增小标题与关键命令确实存在；机器 PASS 不等于内容已写入 ⇒ 证据：4.x 复验清单首次细化后 README 仍是旧版，重跑后增量 137 行。
- [2026-11-13][P2] 把构建检查直接塞进 `--staged` 门禁链会失败 ⇒ `check-repo.mjs --staged` 用 `git checkout-index` 导出纯索引快照，里面没有 `node_modules` / `target` 缓存，无法编译 ⇒ 由公共入口为构建项显式指定 `cwd: ROOT` 与原始 `process.env`，洁净度 / 链接 / 双语仍按索引快照；在脚本与工作流里写明这一差异，且构建失败仍让整链退出非 0，不提供跳过开关 ⇒ 证据：`scripts/check-repo.mjs` checks 项与 `scripts/check-build.mjs` 注释、本机 `--all` 实跑退出码 0。

## CI 与状态栏反馈（任务 5.1 回显 / 5.2，2026-11-14）

- [2026-11-14][P1] 5.1 把构建门禁接入公共链后，Gitee Go 首次 CI 红而 GitHub Actions 绿 ⇒ Gitee 流水线只自装 Node 20，缺 Rust / pnpm 12 / Linux WebView 系统依赖 / `pnpm install`；`check-build.mjs` 的「工具链缺失即失败、不提供跳过开关」正确挡住「文档门禁绿 ⇒ 编译根本没跑」的假绿，不是脚本缺陷 ⇒ **每个调用公共链的 CI 入口都必须提供等价工具链前置**，逐端记录首次回显；不给门禁加环境跳过开关，也不把「配置已写」冒充「云端已通过」 ⇒ 证据：2026-11-13 16:04 Gitee 日志（前三项全过、`exit status 255`）、[change 证据 5.1](../../openspec/changes/add-tauri-page-host/evidence-index.md)、D31。
- [2026-11-14][P1] 底部状态栏是**单条文本覆盖通道**，把所有操作都收敛到 `showStatusBar`（I15）后会互相盖：启动提示盖数据根、`applySize` 成功绿条盖「窗口容纳不下」红条、常态 Viewport / 加载事件盖失败红条 ⇒ 后写者赢，与消息重要程度无关；且 Rust 在 `page_set_size` 内部先 `emit_viewport` 再返回，前端 `await` 后的回写必然晚于事件 ⇒ 启动提示先写、数据根后写；成功反馈交给 Viewport 事件按「设定值 / 可见性」变化信号统一给，删掉 `applySize` 回写；常态 Viewport 回报不占状态栏，只在信号变化时写 ⇒ 证据：`main.ts` 的 `lastViewSignal` 与 `loadInitialState` / `applySize` / `Viewport` 分支，本轮 `pnpm build:desktop` 退出码 0；判据仍需 GUI 人工复核。
- [2026-11-14][P2] 一次性把 CI 工具链补进 Gitee 流水线却无法本机验证 ⇒ `.workflow/ci.yml` 的 apt 权限、镜像可达性、crates 下载耗时都取决于 Gitee 容器，本机只有 YAML 解析与单链结构可查 ⇒ 配置改动只声明「已补齐前置、云端复验待 push」，并在 debt 登记容器能力未实测；不要把 YAML 解析通过写成 CI 通过 ⇒ 证据：本机 `ruby -ryaml` 解析回显（commands 1 条、12 个 `&&`）、[change 证据 5.1](../../openspec/changes/add-tauri-page-host/evidence-index.md)。
