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

**Layress** — 界面压力测试工具。把运行中的 Web／Electron 应用置于不同窗口尺寸、内容长度、
语言和字体条件下，**确认关键操作发生了什么损坏**，并把触发条件缩减成可重复回放的证据。

定位句：本地优先、无需人工正确截图基线、面向**关键操作可达性**，输出已缩减的触发条件 + 可回放证据。

- 调研与立项依据：[`docs/research/2026-11-03-web-electron-ui-stress-test-tool-market-and-open-source-research.md`](docs/research/2026-11-03-web-electron-ui-stress-test-tool-market-and-open-source-research.md)
- 四条边界（**不可混为一谈**，写代码前先背住）：视觉回归 ≠ 几何异常检测 ≠ 主动压力测试 ≠ 缺陷确认与缩减。
  本产品的重心在后两项；几何异常只是候选，**候选不等于缺陷**。

## 二、当前状态卡

> 每次接手**先更新这四行**，再干活。字段固定，不许改成自由叙事。

| 字段 | 当前值 |
|---|---|
| 阶段 | 需求与社区调研阶段；2026-11-05 停点与下次第一步见[综合调研 §十](docs/research/2026-11-05-user-needs-and-agent-augmentation-research.md)，先整理语言切换后编辑与取消的本地任务卡，私有明细仅留草稿；收益待验证，[V1 候选](docs/research/2026-11-04-vscode-candidate-readiness.md)未复现，首验[方案](docs/engineering/plans/2026-11-03-first-validation-plan.md)仍待修订与批准。2026-11-06 已锁三端打包承诺（D15，Win x64 / Ubuntu 24 x64 / mac arm64），验证仍分阶段，见[三端打包规划](docs/engineering/plans/2026-11-06-tri-platform-packaging-plan.md)。2026-11-07 起草[Agent 原生证据闸门](docs/engineering/plans/2026-11-07-agent-native-evidence-gate.md)作首验前端，以自有已打包冻结的 Electron 应用作基线（对象细节留本地草稿）；同日一次基线试用（641 CSS 宽 + en-US + 默认标签）：目标取消按钮经真实坐标命中为真绿，目标上 0 次假绿、假绿门槛 0/1 未达；预注册长标签坏样本未触发，真正待测的 top-layer 假绿面是弹窗背后的同名控件，试用未踩中；视觉待复核；暂不 GO，下次先补该最小对照；证据留本地草稿与 `artifacts/`（不入库）。未立 change。2026-11-08 开发机由 Windows 迁入 macOS arm64（zsh），门禁、索引、工程文档与知识库的本机口径同步；存储纪律统一改称「便携优先 · 不写系统用户目录」（原称「不碰 C 盘」，承诺不变），门禁补齐 macOS / Linux 系统目录与 POSIX 绝对路径判据；原「首验先在 Windows 跑」随开发机迁移失效，首验实机平台待随首验方案修订重定并登记 debt；三端打包承诺（D15）与调研外部样本事实不变。2026-11-08 仓库外盲测完成：模态遮罩下 DOM `click()` 触发而真实指针不触发（3/3 稳定），按跑前冻结判据第 1 步通过；验的是机制而非 11-07 痛闸门的痛点，人工真值未给（I11），首验方案仍未批准，见[盲测结果记录](docs/research/2026-11-08-blind-probe-result.md)。同日 B 轮自主任务对照未出现假绿（执行端如实报告遮挡并找到真实用户路径），按调研 §九 记录为反证，D17 前提削弱。用户选 A 收口：按调研 §九 接受反证，停止独立探针开发、不进第 2 步、不建 workspace、不立产品 change（D18）；D17 暂停，首验方案与闸门提案不再进入执行，未执行项仍单列 |
| 活动 change | 无（尚未确定首个执行范围） |
| 代码 | 无。仓库只有骨架、索引与门禁 |
| 发行 | 无 tag、未发 npm 包。远程领先/落后**跑命令查**（本卡不记 hash，一提交就过期）：`git fetch --all && git log --oneline '@{u}'..main`（远程名以 `git remote -v` 为准） |

判定真相的地方：进度 ⇒ `git log --oneline` 与 `openspec/changes/archive/`；
门禁基线 ⇒ 实跑 `node scripts/check-repo.mjs --all` 的回显；仓库体积 ⇒ `git count-objects -v`；
**push 状态 ⇒ 只认 `git ls-remote` 的回显，不认本文件写了什么**。

仓库与远程：本地 `~/Developer/layress`（macOS arm64 / zsh，2026-11-08 由 Windows 迁入）；分支 `main`；
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
7. 调研原文（§一 链接）—— 首版范围、数据集要求、对照设计、继续/收缩门槛都在这里定义。

## 四、怎么跑起来

**开发环境（2026-11-08 起）**：主力开发机 macOS arm64 / zsh / Node ≥ 20，CI runner 为 ubuntu。
门禁脚本只用 `node:path` 与 git 子进程，三端通用；**本地与云端跑同一条命令链**。
⚠️ 开发机 ≠ 产品目标平台：三端发行承诺（D15）与首版目标平台（调研 §7.2）不随开发机迁移而变。

**尚未建立 pnpm workspace**（本轮口径：结构 + 索引 + 门禁 + i18n）。当前公共门禁入口：

```bash
node scripts/check-repo.mjs --all       # 洁净度 + 全库链接 + 双语配对 + 两类自测
node scripts/check-repo.mjs --staged    # pre-commit：按实际索引快照跑同一条链
node scripts/check-i18n.mjs --links     # 单查全库 Markdown 本地目标路径
```

洁净度的正文、体积与豁免配置均读索引；单项自测与提交快照仅写入 `artifacts/` 并自动清理。
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
6. **push 归用户**。智能体仅在确有必要/有价值、或完成一小段开发时做本地 commit，是否提交由模型按价值判断，不为凑数提交；**不 push**，远程与 tag 由用户操作。

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
  - 连带坑（务必读）：三端上 `os.tmpdir()`、Electron `app.getPath('userData')`、Chromium
    `sessionData` 与 Playwright 浏览器缓存**默认都落在系统用户目录**，只改"数据目录"不够 ⇒ 详见
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
- **Agent 友好**：产品能力必须可由 Agent 发现、非交互调用并取得有版本的结构化结果；机器输出与日志分离，明确退出码与错误码，保留判定层级、证据引用和未执行项。CLI 是首版入口，MCP / Skill 复用核心能力；具体约束见 [`openspec/config.yaml`](openspec/config.yaml) I14，尚未实现。
- **本地优先**：扫描数据、页面内容、截图默认不出本机。用远程视觉模型必须显式标明数据流与成本。
- **候选不等于缺陷、可点击不等于业务成功**：这两条是产品口径，不是措辞问题（见 §一 四条边界与不变量）。

## 八、目录地图

```
入库 ── 门面   README CONTRIBUTING SECURITY CODE_OF_CONDUCT CHANGELOG（中英成对）· LICENSE（原文）
        接手   AGENTS.md（唯一入口，默认中文）
        i18n   i18n/README 口径 · i18n/glossary 术语表 · i18n/docs-manifest 成对登记
        工程   packages/<core|confirm|reduce|replay|cli> apps/desktop
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
