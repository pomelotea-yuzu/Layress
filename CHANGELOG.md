# 更新日志

<p><a href="CHANGELOG.en.md">English</a> | 简体中文</p>

本文件遵循 [Keep a Changelog](https://keepachangelog.com/zh-CN/1.1.0/) 与
[语义化版本](https://semver.org/lang/zh-CN/)。变更类型用 `新增 / 修复 / 变更 / 已移除 / 已废弃`。
**每个有价值的 commit 一次记录**，发布时汇总为版本段落。

## 未发布

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

## 版本与发布约定

- 版本从 `0.1.0` 起算；`0.x` 阶段允许破坏性变更，但必须在 `CHANGELOG` 里写明影响。
- 候选版本用 `-rc.N`；打包、哈希、验收记录与发布正文进 `docs/engineering/reports/`。
- **首验未收口前不打 tag、不发 npm 包**：数据集、对照组与继续/收缩门槛见
  [`docs/research/`](docs/research/README.md) 与 [`docs/knowledge/debt.md`](docs/knowledge/debt.md)。
- push 与 tag 归维护者；双远程（GitHub / Gitee）同步由维护者执行。
