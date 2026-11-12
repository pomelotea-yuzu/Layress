# Layress · 接手索引（AGENTS.md）

> 本文默认以中文维护。

> **这是唯一入口。** 任何开发者、任何智能体，读完本文件就该能动手。
> 各工具的本机记忆文件（`CLAUDE.md`、`.cursor/`、`.trae/` 等）一律不入库；若你的工具只认它们，
> 让它指向本文件，**不要另抄一份内容**——两处口径迟早分叉。
>
> **体积预算**：本文件 ≤ 300 行 / ≤ 18 KB。写不下说明内容该进 [`docs/knowledge/`](docs/knowledge/README.md)，
> 不是该在这里加段落。超预算由 `scripts/check-hygiene.mjs` 之外的**人工审阅**负责（脚本不查行数）。
>
> **权威性排序**（冲突时以靠左的为准）：
> `packages/*/src` 源码 > `openspec/changes/archive/` > `openspec/specs/` > `docs/` > 本文件。
> ⚠️ 本文件**已知会滞后**：判断真实进度只认 `git log` 与 `openspec/changes/archive/`。

---

## 一、项目一句话

**Layress** — 面向个人开发者和小团队的本地界面验收助手。人和 Agent 共用任务、现场与复查结果；
人可以圈画位置并简单描述，让 Agent 理解要改什么，修改后检查原问题。

首版形态：**Tauri 桌面工作台 + 共用任务核心 + CLI**（2026-11-10 用户确认）；一个主窗口承载被测画面、设备配置与原地反馈。MCP 按具体需要增加，系统 WebView 的采集与模拟能力仍须实测。
当前依据：[产品方向与使用方式](docs/product/product.md)。开发者无需先写选择器、JSON 或正式用例才能提出反馈。
旧[立项调研](docs/research/2026-11-03-web-electron-ui-stress-test-tool-market-and-open-source-research.md)按历史背景查阅，旧实现范围不自动恢复；
自动检测候选、用户意见、实际观测与缺陷确认仍须分清，Agent 声称修好不自动等于验收完成。

## 二、当前状态卡

> 每次接手**先更新这四行**，再干活。字段固定，不许改成自由叙事。

| 字段 | 当前值 |
|---|---|
| 阶段 | **add-tauri-page-host 任务 1.x、2.1/2.3/2.4 与 3.x 全部完成，含用户人工复验**（2026-11-11 / 11-12）：原生快照、版本化元数据、两模式坐标、返回会话，以及慢采开关下的 GUI 竞态负例均实测通过；结果条已按用户选择移到窗口最底部独立状态栏（三 WebView）；2.2 无效地址负例未测，4.x–5.x 未开始 |
| 活动 change | [add-tauri-page-host](openspec/changes/add-tauri-page-host/proposal.md)（[tasks](openspec/changes/add-tauri-page-host/tasks.md)：1.x + 2.1/2.3/2.4 + 3.1/3.2/3.3 已勾选；2.2 与 4.x–5.x 未完成；证据见 [evidence-index](openspec/changes/add-tauri-page-host/evidence-index.md)） |
| 代码 | `apps/desktop/`：单窗口三 WebView（工具条 / 被测页面 / 底部状态栏，互不重叠）、HTTP(S) 导航 + 可达性预检、视口设定与 title 哨兵、视口边框、新窗口 / 下载 / 权限拒绝、内嵌样例服务；3.x 新增：WKWebView 原生快照（仅被测视图）、`layress.capture/v1` 元数据落盘、采集竞态状态机与 6 项单测、原地静态预览（工具层铺满 + 被测层仅平移不改尺寸）、原图坐标换算自检与参照块、数据根最小解析、开发期探针与「慢采 5s」复验开关、窗口底部独立状态栏（`status.html` + `status_bar_set`，Vite 双入口） |
| 发行 | 无产品发行。本地提交不 push、不打 tag；push 状态不能沿用旧计数：`git fetch --all && git log --oneline '@{u}'..main`，并用 `git ls-remote` 逐端核对（远程名以 `git remote -v` 为准） |

判定真相的地方：进度 ⇒ `git log --oneline` 与 `openspec/changes/archive/`；
门禁基线 ⇒ 实跑 `node scripts/check-repo.mjs --all` 的回显；仓库体积 ⇒ `git count-objects -v`；
**push 状态 ⇒ 只认 `git ls-remote` 的回显，不认本文件写了什么**。

仓库与远程：本地 `~/Developer/project/layress/`（macOS arm64 / zsh，2026-11-08 由 Windows 迁入，2026-11-10 由 `~/Developer/layress/` 归位到同级 `project/` 下，见 D27）；分支 `main`；
项目约定的双远程为 `github` = `https://github.com/pomelotea-yuzu/Layress.git`（主仓库）、
`gitee` = `https://gitee.com/yuzu-tea-duck/layress.git`（镜像）。**远程名一律以 `git remote -v` 为准**：
当前 Mac 克隆只用单个 `origin`（fetch/push 指向 Gitee，另挂 GitHub push URL），历史记载的「无 origin」
仅描述 2026-11-03 建仓当时，不是跨克隆事实。
本地已 `git init`（分支 `main`、`core.hooksPath=.githooks`）；建仓、tag 与 push 归用户操作，
2026-11-03 用户已推过 `main`。**是否还有未推提交按上一行的命令查，别信任何文字记载。**

## 三、必读顺序（约 10 分钟）

1. 本文件 §一 §二 §五 §六。
2. [`openspec/config.yaml`](openspec/config.yaml) — 项目圣经：定位、技术栈、**架构不变量**、执行边界。
3. [`docs/knowledge/README.md`](docs/knowledge/README.md) — 经验库的结构与**条目格式**（决定一年后这份文件还有没有用）。
4. [`docs/development/workflow.md`](docs/development/workflow.md) — 门禁链、openspec 流程、commit 与 push 口径。
5. [`docs/README.md`](docs/README.md) — 文档地图，新文档该放哪、怎么命名。
6. [`i18n/README.md`](i18n/README.md) — 双语口径与成对登记规则；**动手写文档前先确认它要不要成对**。
   中英用词一律查 [`i18n/glossary.md`](i18n/glossary.md)。
7. [产品方向与使用方式](docs/product/product.md)与[开发路线图索引](docs/product/roadmap.md)—— 已确认要求与六阶段规划；设计按需下沉，当前仅讨论与文档整理，未开始实现。[受众调研](docs/research/2026-11-09-audience-and-workflow-research.md)按背景查阅。

## 四、怎么跑起来

**开发环境（2026-11-08 起）**：主力开发机 macOS arm64 / zsh / Node ≥ 20，CI runner 为 ubuntu。
门禁脚本只用 `node:path` 与 git 子进程，三端通用；**本地与云端跑同一条命令链**。
⚠️ 开发机 ≠ 产品目标平台：旧 D15 与旧首验平台已随阶段封存，当前平台和适配范围按产品设计与实测另定。

**pnpm workspace 已建立（2026-11-11，首个 change 任务 1.3）**：根 `package.json` + `pnpm-workspace.yaml`（apps/*、packages/*），`apps/desktop/` 为 Tauri 骨架（tauri 2.12.5 unstable / Vite 7.3.8 / TS 5.9.3 strict）。当前可跑：

```bash
pnpm install                      # 首次 / 依赖变更后（esbuild 构建脚本经 allowBuilds 审批）
pnpm build:desktop                # tsc --noEmit + vite build（前端类型与产物检查）
cargo check                       # 在 apps/desktop/src-tauri/ 下（需 Rust 工具链）
cargo test --lib                  # 采集状态机 / PNG 与 base64 纯逻辑单测（5 项）
pnpm tauri dev                    # 开发运行（GUI，须本机交互验证）
LAYRESS_PROBE_CAPTURE=1 pnpm tauri dev   # 开发期无人值守采集探针，仅 debug 构建生效

node scripts/check-repo.mjs --all       # 洁净度 + 全库链接 + 双语配对 + 两类自测
node scripts/check-repo.mjs --staged    # pre-commit：按实际索引快照跑同一条链
node scripts/check-i18n.mjs --links     # 单查全库 Markdown 本地目标路径
```

构建 / 类型检查尚未接入 `check-repo.mjs` 公共入口（属任务 5.1），当前须单独跑 `pnpm build:desktop`、`cargo check` 与 `cargo test --lib`。
工作区排查用 `--all`；部分暂存的提交以 `--staged` 为准。

每个克隆做一次（不做就等于没有门禁）：

```bash
git config core.hooksPath .githooks
```

Workspace 骨架落地后按 `docs/development/workflow.md` 建立 `pnpm check:ci`，并接入现有
`check-repo.mjs` 公共入口，让 pre-commit 与 CI 一起继承新增检查；**云端和本地必须跑同一条命令链**。

## 五、仓库洁净度红线

用户口径：**严格控制 push 上传，仓库要干净，不传垃圾。** 落法是三层，缺一层都会被撞穿：

| 层 | 位置 | 性质 |
|---|---|---|
| 兜底忽略 | [`.gitignore`](.gitignore) | 每段必须写「这是什么」+「有价值的东西该去哪」。默认放行，**不是防线** |
| 机器门禁 | [`scripts/check-hygiene.mjs`](scripts/check-hygiene.mjs) | **默认拒绝**。体积、后缀、垃圾场路径、凭据形态、忽略泄露五项 |
| 可审计豁免 | [`.git-hygiene-allowlist`](.git-hygiene-allowlist) | `glob + 一句话理由`；**无理由 = 门禁失败**；CI 里顺带查失效豁免 |

六条不许碰的：

1. **两个垃圾场，只许这两个**：`artifacts/`（一次性产物，可重跑复原）与 `.layress/`（运行时数据与日志）。
   手写结论、审阅记录、方案**不许**先扔这里再说——直接写进 `docs/` 的正确子目录。
   （前置项目的 `output/` 就是"随手放"烂掉的，本仓不给第三个垃圾场起名的机会。）
2. **二进制不进历史**。blob 进了历史就删不干净。安装包、抓包、录屏、模型文件一律走豁免 + 理由，
   或者干脆只提交生成脚本。
3. **凭据零容忍**：密钥由用户在应用内填写，仓库只留占位符；`.env*`、`.mcp.json`、含绝对路径的本机配置永不入库。
4. **截图默认留本地**：`docs/reviews/**/*.png` 已忽略。验收证据真要对外展示才 `add -f`。
5. **`git add -f` 之前**先问三句：能被脚本重生成吗？能不能放到 `docs/` 或 `datasets/` 的正确位置？
   是否其实只该躺在垃圾场（那就根本不需要豁免）？
6. **change 先由另一 Agent 审阅，原编写 Agent 修订后提交，push 归用户**。写完 change 后等待用户切换另一 Agent，在对应目录产出 `review.md`；用户回到原编写 Agent 后，由其读取并处理意见、通过必要检查，即可做相关本地 commit，无需再次索要确认。不得以编写者自审、空占位或机器 PASS 跳过此流程；Agent 审阅不替代人工验收。满足条件后仍只在确有价值时提交，不凑数；**不 push**，远程与 tag 由用户操作。

## 六、经验与坑怎么写（本仓最重要的机制）

前置项目的经验库有 90% 落在**本机记忆目录**里，被 `.gitignore` 整目录排除 ⇒ clone 仓库的人拿不到。
本仓把同一套东西**入库**，并且规定格式，避免它退化成没人读的流水账。
**回填纪律**：开发中每发现一个坑、推翻一个假设或变更一条口径，**同轮**写进对应专题；
`.workbuddy/memory/` 只记本机当日流水，别的 Agent 与接手人看不到，只写那里等于没写。

落点：[`docs/knowledge/`](docs/knowledge/README.md) 五个专题，**按主题不按日期**。

| 文件 | 收什么 |
|---|---|
| `engineering.md` | 构建、测试、门禁、依赖、openspec 用法上的坑 |
| `runtime-probe.md` | 实机探针的硬事实：CDP、窗口、DPR、输入法、时序、假绿判据 |
| `invariants.md` | **不许重开的架构不变量**，每条给出不成立时的后果 |
| `decisions.md` | 关键决策速查：决定了什么、为什么、代价是什么 |
| `debt.md` | 未决口径与欠账，标注归谁定夺 |

四条写法纪律：

1. **一条一行一主题**，格式固定：`- [YYYY-MM-DD][级别] 现象 ⇒ 根因 ⇒ 规避 ⇒ 证据`。
   级别用 `P0` 致命 / `P1` 会误判 / `P2` 浪费时间的。
2. **只写结论，不写过程**。过程写在对应 change 的 `design.md` / `evidence-index.md`，
   这一行只留指针。几百字挤一行的日期日志，下个接手人只会跳过。
3. **同一坑第二次踩 ⇒ 升级**：从条目升进 `invariants.md` 或直接变成门禁/脚本断言。
   靠记性防的坑一定会再踩；只有代码拦住的才算防住。
4. **机器 PASS ≠ 人工通过**。视觉/交互类判定归人工复核，未执行的项**单列**，不许勾完成，
   也不许在状态卡里写成已交付。

## 七、口径与纪律

- **语言**：所有产出（代码注释、文档、commit message）默认用中文；产品名 `Layress` 保持英文。
- **存储纪律：便携优先 · 不写系统用户目录**（用户硬性要求，与洁净度同级；原称「不碰 C 盘」，2026-11-08 改为跨平台表述，承诺本身不变）：
  - 产品默认**便携优先**：数据、日志、缓存、临时文件写在**产品自身目录**（默认 `.layress/`，或 `data/`），
    或用户显式指定的目录。
  - **不得**默认写入任何一端的系统用户目录：Windows `C:\Users\<user>\AppData\*`／`%APPDATA%`／
    `%LOCALAPPDATA%`／`%TEMP%`／注册表；macOS `~/Library/{Application Support,Caches,Preferences,Logs}`／
    `/private/var/folders`；Linux `~/.config`／`~/.local/share`／`~/.cache`。
    安装位置由用户选择，安装器不提权、不改系统 PATH。
  - **数据跟着产品放**：拷贝整个产品目录即完成迁移；删除该目录即无残留。
  - 连带坑（务必读）：系统 WebView 的会话 / 缓存和 Tauri 默认应用路径须逐端核查并覆盖；旧探针涉及的
    `os.tmpdir()`、Electron `userData` / `sessionData` 与 Playwright 缓存也会默认写系统用户目录，只改"数据目录"不够 ⇒ 详见
    [`docs/knowledge/engineering.md`](docs/knowledge/engineering.md) 与不变量 I12–I13。
  - 数据目录的解析结果必须在报告、日志与 UI 里**显示出来**，否则用户无法验证这条承诺。
- **文档默认中文，对外门面按需双语**（详见 [`i18n/README.md`](i18n/README.md)）：
  - 当前维护中英双版本的文件仅为根目录 `README.md`、`CONTRIBUTING.md`、`SECURITY.md`、
    `CODE_OF_CONDUCT.md`、`CHANGELOG.md`；同目录 `*.en.md` 同轮修改，并登记进 [`i18n/docs-manifest.json`](i18n/docs-manifest.json)。
  - 本文件、`docs/` 全部文档、经验库、方案、调研与 `i18n/` 维护说明**默认只写中文**；
    以后确有对外阅读需求的文档再逐篇登记，不按目录一律翻译。已有内部英文版移入 `docs/drafts/` 本地归档，不入库。
  - 表单类（GitHub Issue 表单、PR 模板）**不做两份**：同一表单内并列
    `label: English / 中文`，Gitee 侧另给纯中文模板（`.gitee/*.zh-CN.md`）。
  - 中英用词查 [`i18n/glossary.md`](i18n/glossary.md)；术语表本身只维护中文说明，**代码标识、协议字段、错误码原值不翻译**。
- **国际化 = 中国用户优先 + 英文产品名 + 具备海外发布能力**，不等于"把产品改成英文"。
- **命名**：`docs/` 内已批准且作为当前依据的方案、现行规则、已核实事实用 `<english-kebab-case>.md`；探讨、调研、待批提案与当次记录用 `YYYY-MM-DD-<english-kebab-case>.md`。正文标注状态与证据，`README.md` 与 `*.en.md` 保留惯例；晋升与改名细则见 [`docs/README.md`](docs/README.md)。
- **Agent 友好**：产品能力必须可发现、可非交互调用并取得有版本的结构化结果；机器输出与日志分离，退出码、错误码、证据与未执行项明确。桌面工作台与 CLI 共用任务核心，MCP / Skill 按需增加；约束见 [`openspec/config.yaml`](openspec/config.yaml) 的 Agent 接口约定，尚未实现。
- **本地优先**：扫描数据、页面内容、截图默认不出本机。用远程视觉模型必须显式标明数据流与成本。
- **候选不等于缺陷、可点击不等于业务成功**：用户意见、实际观测、自动检查和 Agent 处理结果保留来源，未执行的人工复核单列，见[产品依据](docs/product/product.md)。

## 八、目录地图

```
入库 ── 门面   README CONTRIBUTING SECURITY CODE_OF_CONDUCT CHANGELOG（中英成对）· LICENSE（原文）
        接手   AGENTS.md（唯一入口，默认中文）
        i18n   i18n/README 口径 · i18n/glossary 术语表 · i18n/docs-manifest 成对登记
        工程   packages/ 共用能力 · apps/ 应用入口（具体划分未定）
               scripts/check-repo.mjs 公共入口 · check-hygiene.mjs · check-i18n.mjs
        知识   docs/README 文档地图
              docs/guide architecture development        指南与工程说明（默认中文）
              docs/knowledge                             ★ 经验库（默认中文），见 §六
              docs/research product engineering/* reviews 过程资产（默认中文）
              datasets/                                 正负例数据集（最有积累价值的资产）
              openspec/  config.yaml + specs/ + changes/{,archive/}
        接入   .github/{workflows,ISSUE_TEMPLATE}（表单内双语）· .gitee（纯中文）· .workflow .githooks
不入库 .gitignore .layress/ artifacts/ docs/drafts/ .workbuddy/ .trae/ .cursor/ .claude/ .vscode/
```

## 九、本文件的维护规则

- 状态卡（§二）每次接手更新；其余段落**只在口径变了**时改，改不动就说明该改代码或该改门禁。
- **仅对登记的对外中英文档要求同轮同步**：范围以 `i18n/docs-manifest.json` 的 `docs` 为准；
  本文件与其他内部文档默认中文；停止维护的内部译本只在 `docs/drafts/` 本地归档，不再同步。
- 新增目录必须同时在 `docs/README.md` 地图与本仓对应 `README.md` 登记归属；
  没登记 = 归属未定，先别建。
- 每次整理文档要留「路径变更记录」（见 `docs/README.md` 末尾），否则三个月后没人知道东西搬去哪了。
