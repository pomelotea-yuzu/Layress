# 更新日志

<p><a href="CHANGELOG.en.md">English</a> | 简体中文</p>

本文件遵循 [Keep a Changelog](https://keepachangelog.com/zh-CN/1.1.0/) 与
[语义化版本](https://semver.org/lang/zh-CN/)。变更类型用 `新增 / 修复 / 变更 / 已移除 / 已废弃`。
**每个有价值的 commit 一次记录**，发布时汇总为版本段落。

## 未发布

### 开发环境迁移 macOS 与门禁跨平台加固 · 2026-11-08

- 变更 **开发机口径**：主力开发机由 Windows 迁入 macOS arm64 / zsh，本地仓库 `~/Developer/layress`；
  `AGENTS.md` §二/§四、`openspec/config.yaml`、`docs/development/workflow.md` 与 CONTRIBUTING 同步；
  显式区分「开发机 ≠ 产品目标平台」——三端发行承诺（D15）与调研 §7.2 首版范围不变。
- 变更 **存储纪律改名**：I12「不碰 C 盘」改称「便携优先 · 不写系统用户目录」，承诺不变；禁止路径补齐
  macOS `~/Library`／`/private/var/folders` 与 Linux XDG；门面双语、术语表、不变量、经验库与规划文档同轮对齐。
  本文件 2026-11-03 的历史条目仍保留旧称，不作改写。
- 新增 **门禁跨平台判据**：`check-hygiene.mjs` 的「机器绝对路径」收 POSIX `/Users/<name>/`／`/home/<name>/`，
  「系统用户目录」补 macOS Library／`/private/var/folders`／Linux XDG／`os.tmpdir()`／更多 Electron 默认路径；
  `--selftest` 增加 macOS Library、Linux XDG、POSIX 绝对路径三个反证用例。
- 变更 **首验实机平台口径**：原「先在 Windows 跑」随迁机失效，改为**待随首验方案修订重定**并登记
  [`docs/knowledge/debt.md`](docs/knowledge/debt.md)；三端打包规划、路线图与首验方案同步，未执行项仍单列。
- 变更 **本地提交纪律**：智能体仅在确有必要/有价值、或完成一小段开发时做本地 commit，是否提交由模型按价值判断；
  **push 与 tag 仍归维护者**。
- 修复 **pre-commit hook 在 macOS 被忽略**：`.githooks/pre-commit` 历史 mode 为 `100644`，Windows 下不依赖执行位，
  迁到 Mac 后 Git 直接跳过该 hook，pre-commit 洁净度链形同未挂；改为 `100755`，`git hook run pre-commit` 实跑通过，
  并在门禁增加「hook 缺可执行位」断言与反证用例防复发。

### 仓库骨架 · 2026-11-03

- 新增 目录结构与归属说明：`docs/{guide,architecture,development,knowledge,research,product,engineering,reviews}`、
  `datasets/`、`openspec/{config.yaml,specs,changes/archive}`、`packages/`、`apps/`、`scripts/`。
- 新增 接手索引 [`AGENTS.md`](AGENTS.md)：项目一句话、状态卡、必读顺序、洁净度红线、经验条目格式、目录地图。
  与前置项目的关键差异是**经验库入库**（`docs/knowledge/`），不再只存在本机记忆目录里。
- 新增 仓库洁净度门禁 `scripts/check-hygiene.mjs`：体积、后缀黑名单、垃圾场路径、凭据形态、
  忽略泄露五项默认拒绝；`.githooks/pre-commit` + CI 双跑；豁免只走 `.git-hygiene-allowlist` 且必须带理由。
  8 项正反用例实测通过（拦垃圾、放行正当入库、无理由豁免判红）。
- 新增 i18n 结构层：双语口径 [`i18n/README.md`](i18n/README.md)、冻结术语表
  [`i18n/glossary.md`](i18n/glossary.md)、成对登记 `i18n/docs-manifest.json`，
  以及双语门禁 `scripts/check-i18n.mjs`（缺对 / 断链 / 小节漂移 / 孤儿英文 + `--selftest` 反证）。
- 新增 中英成对的门面文档：[`README.md`](README.md)、[`CONTRIBUTING.md`](CONTRIBUTING.md)、
  [`SECURITY.md`](SECURITY.md)、[`CODE_OF_CONDUCT.md`](CODE_OF_CONDUCT.md)、本文件。
- 新增 工程约定：`.gitignore`（每段写明为什么不入库、有价值内容该去哪）、`.gitattributes`（强制 LF，
  二进制与夹具禁转换）、`.editorconfig`、`docs/knowledge/` 五条经验专题。
- 新增 双轨 CI 与模板配置：`.github/workflows/ci.yml`、`.github/ISSUE_TEMPLATE/`（表单内双语并列）、
  `.github/PULL_REQUEST_TEMPLATE.md`、`.gitee/`（纯中文模板）、`.workflow/ci.yml`（Gitee Go）。
- 新增 **存储纪律（不碰 C 盘）**：不变量 I12–I13 定义"便携优先、数据跟着产品放、不可写即报错不静默回落"，
  写入 `AGENTS.md` §七、`openspec/config.yaml`、`README`；`check-hygiene.mjs` 增加「系统用户目录」规则，
  产品代码里出现 `%APPDATA%`／`process.env.LOCALAPPDATA`／`getPath('userData')`／`C:\Users\` 形态即判红
  （实测：产品代码判红、脚本同类写法只警告）；`docs/knowledge/engineering.md` 记录连带坑 ——
  `os.tmpdir()` 与 Chromium `sessionData` 默认也在 C 盘，光改数据目录不够。
- 新增 双远程登记：主仓库 `https://github.com/pomelotea-yuzu/Layress`、镜像 `https://gitee.com/yuzu-tea-duck/layress`
  （两边均为空仓，`git ls-remote` 实测无 ref）；`AGENTS.md` 状态卡补"push 状态只认 `git ls-remote`"，
  `openspec/config.yaml` 记远程与"无 origin"口径，`SECURITY.md` 换成真实的私密漏洞报告入口，
  `.github/ISSUE_TEMPLATE/config.yml` 填入 Gitee 镜像 / 文档 / 接手索引 / 安全报告四条 contact_links。
- 变更 立项调研文档自仓库根迁入 [`docs/research/`](docs/research/README.md)。
- 未做 代码骨架（`package.json`、pnpm workspace、biome、测试链）——本轮口径见
  [`docs/knowledge/decisions.md`](docs/knowledge/decisions.md) D13，属计划而非遗漏。

### 首个验证方案提案与仓库上线 · 2026-11-03

- 新增 [`docs/engineering/plans/2026-11-03-first-validation-plan.md`](docs/engineering/plans/2026-11-03-first-validation-plan.md)：
  把调研 §七 的建议做成可执行方案——S0 探针通道对照实验、S1 数据集建设、S2 A/B/C 三组公平对照、
  S3 指标与人工复核、S4 go/shrink 判定；时间盒建议 6 周 / 约 90 小时（S1 占 40%，砍功能不砍样本量）；
  另附"需要用户拍板的六件事"与"失败会是什么样"（预先写死收缩条件，避免事后合理化）。**状态：提案，未批准。**
- 变更 探针选型口径：**不预设结论**，改由 S0 对照实验量出（R1 Playwright + 系统 Chrome vs R2 裸 CDP，
  3 个已知答案样本 × 7 项测量 × 6 次重复），其中"C 盘用户目录零字节增量"（不变量 I12）列为硬判据；
  判据写在实验之前，防止做完再找理由。
- 新增 仓库上线：`git init -b main`、`core.hooksPath=.githooks` 生效、双远程 `github`（主）+ `gitee`（镜像）无 `origin`；
  骨架按 6 笔分步提交入库，用户已把 `main` 推到两边（末笔 `e02e703`，`git ls-remote` 实测一致）。
- 变更 [`AGENTS.md`](AGENTS.md) 状态卡、[`README.md`](README.md) 当前状态与 [`docs/knowledge/debt.md`](docs/knowledge/debt.md)
  同步 init/push 实况，并销掉 debt 中"首次 commit 与双远程 push"一行——已定论的事不留在欠账表里。

## 版本与发布约定

- 版本从 `0.1.0` 起算；`0.x` 阶段允许破坏性变更，但必须在 `CHANGELOG` 里写明影响。
- 候选版本用 `-rc.N`；打包、哈希、验收记录与发布正文进 `docs/engineering/reports/`。
- **首验未收口前不打 tag、不发 npm 包**：数据集、对照组与继续/收缩门槛见
  [`docs/research/`](docs/research/README.md) 与 [`docs/knowledge/debt.md`](docs/knowledge/debt.md)。
- push 与 tag 归维护者；双远程（GitHub / Gitee）同步由维护者执行。
