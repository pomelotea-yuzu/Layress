# 开发工作流

> 本文默认以中文维护。

本页是日常操作的权威说明。口径变了必须同时改 [`/AGENTS.md`](../../AGENTS.md) 与本文件；两者均默认只维护中文。

当前产品与阶段以[产品依据](../product/product.md)和[正式路线图索引](../product/roadmap.md)为准；2026-11-10 已建立首个 [add-tauri-page-host](../../openspec/changes/add-tauri-page-host/proposal.md)，只覆盖 Tauri 页面承载首增量，尚未编码。[第一阶段计划](../product/2026-11-10-stage-one-plan.md)后续范围不自动获批，不恢复旧验证方案的批准闸门或包划分。

## 环境与日常命令

- **开发机（2026-11-08 起）**：macOS arm64 / zsh / Node ≥ 20；CI runner 为 ubuntu。门禁脚本只用
  `node:path` 与 git 子进程，三端通用，**本地与云端跑同一条命令链**。开发机平台 ≠ 产品目标平台：
  旧 D15 与旧首验平台已封存，当前支持范围由设计与实测确定。
- Node ≥ 20；包管理器 pnpm（workspace 建立后在根 `package.json` 固定 `packageManager` 字段）。
- Tauri 采用 Rust 原生层与 TypeScript 前端；具体 Rust / Tauri 版本、系统 WebView 依赖和构建步骤在实现 change 确定，本轮未安装、不编造可运行命令。后续 Rust 与前端检查一起接入现有公共入口，不另设只在本机运行的门禁链。
- 每个克隆一次：`git config core.hooksPath .githooks`。不挂 hook，洁净度问题会推迟到 CI 才暴露。

```bash
# 公共入口：完整门禁与自测；提交时检查索引快照，CI 检查干净检出
node scripts/check-repo.mjs --all
node scripts/check-repo.mjs --staged

# 单项排查
node scripts/check-hygiene.mjs --all
node scripts/check-hygiene.mjs --selftest
node scripts/check-i18n.mjs --links
node scripts/check-i18n.mjs
node scripts/check-i18n.mjs --selftest

# workspace 落地后的目标命令链（尚未建立，见 docs/knowledge/debt.md）
# pnpm check:build && pnpm check:typecheck && pnpm check:test \
#   && pnpm check:lint && pnpm check:i18n && pnpm check:spec && pnpm check:hygiene
```

## 质量门禁

| 门禁 | 命令 | 拦什么 | 状态 |
|---|---|---|---|
| 洁净度 | `check-hygiene.mjs` | 索引中的体积、二进制后缀、垃圾场路径、凭据、豁免与忽略泄露 | 生效 |
| 全库链接 | `check-i18n.mjs --links` | 全部已跟踪 Markdown 的相对路径、仓库根路径及引用式链接断链，不依赖双语登记 | 生效 |
| 双语成对 | `check-i18n.mjs` | 缺英文、语言入口标签标反、小节漂移、孤儿英文 | 生效 |
| 构建 / 类型 / 测试 / lint | `pnpm check:*` | 代码质量与回归 | 待建 |
| spec 一致性 | `pnpm check:spec` | 主 spec 与归档 change 的差集 | 待建 |

两条硬约定：

- **本地与云端跑同一条链**：公共入口 [`scripts/check-repo.mjs`](../../scripts/check-repo.mjs) 顺序运行洁净度、洁净度自测、文档门禁自测、全库链接与对外双语配对，任一步失败即停止。CI 两侧调用 `--all`；pre-commit 调用 `--staged`，先导出完整索引快照，按该快照的脚本、忽略规则、双语清单与文档检查，未暂存的修复不能掩盖提交里的错误。普通 `--all` 的链接与配对检查读取已跟踪的工作区文档，CI 干净检出时与索引一致。
- **机器 PASS ≠ 验收**。视觉与交互判定归人工复核；未执行的复核在项目状态里单列"未执行"。

实现 Agent 接口时，change 必须定义输入输出 schema、退出码与错误码，并验证无 TTY 调用、缺参、机器输出解析、状态分层、超时 / 取消和证据引用；验收口径见 [`openspec/config.yaml`](../../openspec/config.yaml) 的 Agent 接口约定。此项为待实现要求，不计作当前门禁已覆盖。

## 仓库洁净度操作

被拦下后的标准动作，按顺序判断：

1. 这东西能被脚本重新生成吗？⇒ 留在 `artifacts/` 或 `.layress/`，**不需要**入库也不需要豁免。
2. 它是有价值的手写内容吗？⇒ 移到正确位置：结论进 `docs/` 对应子目录，样本进 `datasets/`。
3. 它确实必须作为二进制入库吗？⇒ 在 [`.git-hygiene-allowlist`](../../.git-hygiene-allowlist) 加一行
   `glob  #  理由`（必要时 `size=NNNK/M`），再 `git add -f`。无理由的豁免判红。
4. 报"忽略却已跟踪"？⇒ `git rm --cached <path>` 后重新提交，别改 `.gitignore` 迁就历史。

`.gitignore` 的维护纪律：每新增一段，必须写清**这是什么**与**有价值的东西该去哪**；
只写路径不写理由的忽略规则，三个月后没人敢删。

## 双语操作

新增或修改成对文档的流程：

1. 文档默认中文。仅 [`/i18n/docs-manifest.json`](../../i18n/docs-manifest.json) 的 `docs` 中登记的对外文件要求成对；当前为根目录 README、CONTRIBUTING、SECURITY、CODE_OF_CONDUCT、CHANGELOG。
2. 内部文档（含本页、接手索引、经验库与方案）只需中文；以后逐篇确认对外阅读需求后才加英文并登记。
3. 登记文件以中文为源，同轮更新同目录 `X.en.md`，一级标题下互放语言入口。未登记的新增英文版判 `TWIN_UNREGISTERED`。
4. 已停止维护的内部译本移入 `docs/drafts/YYYY-MM-DD-internal-translations/` 本地归档，附原路径索引，不入库、不要求同步；中文页移除旧语言入口。新术语只需先维护 [`/i18n/glossary.md`](../../i18n/glossary.md) 中的中英对照。
5. 本地跑 `node scripts/check-i18n.mjs` 确认对外配对检查全绿再提。

Issue/PR 表单**不做两份**：GitHub 侧在同一个 yml 字段里并列 `English / 中文`，Gitee 侧纯中文。
改任一处要同轮对齐另外两处——脚本查不了语义等价，这条靠人。

## OpenSpec 流程

```
openspec/
├─ config.yaml        项目圣经：定位、技术栈、架构不变量、执行边界
├─ specs/             主 spec（能力口径，合入后长期有效）
└─ changes/           活动 change
   └─ archive/        已归档 change（决策与证据的权威落点）
```

一个 change 的生命周期：

1. `openspec/changes/<name>/proposal.md` 写范围与影响，`design.md` 写方案与取舍，`specs/<capability>/spec.md` 写规格增量，`tasks.md` 拆实施步骤；schema / 日期记在 `.openspec.yaml`，先准备可执行方案，再按授权实施。
2. 按任务实施，每步留判据；实机证据记在 `evidence-index.md`（截图默认留本地，见洁净度）。
3. 收口后归档 `openspec/changes/archive/<YYYY-MM-DD>-<name>/`，delta 合入 `specs/`。
4. **权威性排序**：源码 > `changes/archive/` > `specs/` > `docs/` > `AGENTS.md`。
   判断进度只认前两者，文档一律可能是旧的。

纯文档修订、错别字、链接修复不必立项。

### Agent 审阅与本地提交（2026-11-10 建立，同日修订）

用户于 2026-11-10 明确最终流程：

1. 编写 Agent 写完 change，保留工作区改动，等待用户切换到另一 Agent。
2. 审阅 Agent 在对应 `openspec/changes/<name>/` 产出 `review.md`，记录结论、带文件位置与建议动作的问题清单、实跑检查与静态推断的区分、未执行项；不要求额外多模型交叉或用户手写审阅。
3. 用户回到原编写 Agent，由其读取 review、处理意见并修改 change，运行必要检查。
4. 意见处理完成且检查通过后，原编写 Agent 即可做相关本地提交，无需再让用户确认一次；不 push、不打 tag。

编写者自行生成 review、空文件占位、规划齐备或机器 PASS 均不能代替另一 Agent 的审阅；用户切换 Agent，不要求编写 Agent 自动生成子 Agent。若审阅仍有实际未解决问题，应处理或说明，不能仅凭文件存在就当成通过，也不能先提交附带的工具记录绕过等待。
Agent 审阅属**机器复核，不替代人工验收**：视觉 / 交互人工复核、实机验证与未执行项仍单列。文档意见已落实与对应产品运行验证已通过分别记录，用户保留最终决定权。
这一规则适用于 change 相关的文档与实现提交；它是本地提交的前置条件，不代替产品实现授权，也不恢复旧阶段的市场 / 探针批准闸门。

### 共享 OpenSpec CLI（2026-11-10 已安装）

当前开发机采用项目外共享工具目录 `tools/openspec/`，固定 `@fission-ai/openspec@1.17.0`，由用户指定位置；本机布局下从 Layress 根目录通过以下入口调用：

```bash
../tools/openspec/openspec --version
../tools/openspec/openspec validate add-tauri-page-host --strict --no-interactive --json
../tools/openspec/openspec status --change add-tauri-page-host --json
```

其他项目在自己的项目根目录使用同一个工具入口的实际路径，工作目录保持为被操作项目。上述相对路径仅适用于本机同级 `tools/` 布局，clone 仓库不会自动带来工具安装，不将本机绝对路径或依赖目录提交到仓库。
该版本要求 Node ≥ 22.12.0，本机 Node 26.13.0 已实跑；仓库原有门禁 Node ≥ 20 的要求不因此改写。本轮未安装 Tauri / Rust / pnpm 产品工具链。

共享入口将 XDG 配置 / 缓存 / 数据、npm 缓存 / 日志与临时目录定位到工具根，设置 `OPENSPEC_TELEMETRY=0`、`DO_NOT_TRACK=1` 和 `OPENSPEC_NO_UPDATE_CHECK=1`；不做全局安装、不修改系统 PATH。使用该入口，直接调用原始 `node_modules/.bin/openspec` 会绕过这些目录与隐私设置。
安装使用 npm 官方源、精确版本、工具根内的独立用户 / 全局配置与 `--ignore-scripts`，依赖和锁文件留工具目录；本轮已验证无需依赖安装脚本即可运行校验。升级由显式操作更新，不自动拉取 `latest` 或修改其他项目。

本轮 `validate` 通过，1 个 change、0 个问题；`status` 确认四类规划文档齐备。`isPlanningComplete` / `isComplete` 是该命令的规划状态，不等于任务已实施；实际实施仍看 change 的未勾选任务与实机证据。
仓库文档门禁与 OpenSpec 语义校验分别记录；共享 CLI 可用尚不等于已建立 workspace 或自动 `check:spec` 门禁。

## 文档与命名

- `docs/` 内已批准且作为当前依据的方案、现行规则与已核实事实用 `<english-kebab-case>.md`；日常探讨、调研、待批提案与当次记录用 `YYYY-MM-DD-<english-kebab-case>.md`。`README.md` 与英文 `*.en.md` 保留惯例，细则见 [`/docs/README.md`](../README.md)。
- 提案获批或结论晋升时，正文记录状态、日期、范围与证据；同步修复引用、双语登记与路径变更记录。历史过程记录保留，避免两份现行口径。
- 新目录要同时在 [`/docs/README.md`](../README.md) 地图和该目录 `README.md` 登记归属；没登记 = 归属未定，先别建。
- 每次整理留「路径变更记录」（`docs/README.md` 末尾），否则三个月后没人知道文件搬去哪了。
- 经验条目按格式写进 [`docs/knowledge/`](../knowledge/README.md)，不要写进 `AGENTS.md` 正文——它有体积预算。

## Commit 与 push

- 一次 commit 做一件事；标题 `<type>(<scope>): <一句话改动>`，正文中文写"为什么"。
- **提交前自检**：本轮踩的坑、被推翻的假设与口径变更，是否已在 [`docs/knowledge/`](../knowledge/README.md)
  对应专题留条目？只写 `.workbuddy/memory/` 等于没写（该目录不入库，别的 Agent 与接手人拿不到）。
- **仅在确有必要/有价值、或完成一小段开发时做本地 commit**；是否提交由模型/执行者按价值判断，
  不为凑数提交，也不攒一大坨；change 相关提交须先满足本页的 Agent 审阅 `review.md` 前置条件，等待时保留工作区修改；**不 push**。
- push、打 tag、双远程同步归维护者。**先跑 `git remote -v`**：远程名是本机配置，不写死在文档里。
  当前 Mac 克隆的 `origin` 挂了两条 push URL（Gitee + GitHub），一条 `git push origin main` 即推两端；
  远程名分开配置时逐端 `git push <远程名> main`，或直接用显式 URL
  （`git@gitee.com:yuzu-tea-duck/layress.git`、`git@github.com:pomelotea-yuzu/Layress.git`）。
  多 pushurl 顺序推送、任一端失败即中止，重跑幂等；推完用 `git ls-remote` 逐端核对 hash。
  tag 单独推，**不要移动或强推已发布的 tag**。
- push 计数只认 `git ls-remote` 的回显，不认文档里写的"已推"。
- 禁止 `--no-verify` 绕过洁净度门禁。真要绕过，说明门禁写错了，去修脚本而不是每次手工放行。
