# 贡献指南

<p><a href="CONTRIBUTING.en.md">English</a> | 简体中文</p>

感谢参与。本仓对**仓库洁净度**和**双语成对**有硬性门禁，请先看完这两节再提第一个 PR。

## 先读什么

1. [`AGENTS.md`](AGENTS.md) —— 接手索引：项目一句话、状态卡、纪律红线、坑该写在哪。
2. [`docs/knowledge/invariants.md`](docs/knowledge/invariants.md) —— 全部编号不变量，**不许在 PR 里重开**。
3. [`docs/knowledge/decisions.md`](docs/knowledge/decisions.md) —— 已定的技术选择与其推翻条件。
4. [`i18n/glossary.md`](i18n/glossary.md) —— 中英用词一律以它为准。

想法类讨论先开 Issue，不要直接提交实现。产品口径的核心问题是"关键操作可达性"，
任何让报告更接近"看起来坏"的实现方向都需要先对齐（见不变量 I1、I6）。

## 环境准备

```bash
# 每个克隆必须做一次：不挂 hook，洁净度门禁就只是 CI 里的一次事后报错
git config core.hooksPath .githooks

node scripts/check-hygiene.mjs --staged   # 暂存区体检
node scripts/check-i18n.mjs               # 双语成对体检
node scripts/check-i18n.mjs --selftest    # 怀疑门禁本身时先跑这条
```

Node ≥ 20。pnpm workspace 与 `pnpm check:ci` 尚未建立（见 [`docs/development/workflow.md`](docs/development/workflow.md)），
所以现阶段没有 build/test 步骤。

## 提交流程

行为变更走 openspec（口径见 [`docs/development/workflow.md`](docs/development/workflow.md)）：

1. 开 Issue 或在已有 Issue 下对齐问题陈述。
2. 建 `openspec/changes/<change-name>/`：`tasks.md` 拆步骤，`design.md` 写方案与取舍。
3. 按任务实施，**每个有价值的步骤一次 commit**；判据与实测证据留在 change 目录内。
4. 归档进 `openspec/changes/archive/<date>-<change-name>/`，合入主 spec。

纯文档修订、错别字、链接修复可直接提 PR，不必立项。

## 仓库洁净度

本仓严格控制进入历史的内容。**blob 一旦提交就删不干净**，所以拒绝由脚本执行，不靠 reviewer 眼力。

| 规则 | 触发 | 做法 |
|---|---|---|
| 垃圾场路径 | `artifacts/`、`.layress/`、`output/`、`.tmp*`、`node_modules/` 等出现在暂存区 | 手写结论改放 `docs/` 正确子目录；产物留在垃圾场即可 |
| 二进制/归档后缀 | `.msi .exe .dll .zip .7z .mp4` 等 | 只提交生成脚本，或登记豁免 |
| 体积 | 单文件 > 512 KB 判红，> 256 KB 警告 | 拆分、压缩为可再生成、或抬高豁免 `size=` |
| 凭据形态 | 私钥块、`sk-…`、`ghp_…`、`AKIA…`、赋值型 token | 移除；密钥由用户在应用内填写，仓库只留占位符 |
| 机器绝对路径 | 源码/配置里出现 `C:\Users\…` | 改相对路径或运行时解析的数据目录 |
| 忽略却已跟踪 | `.gitignore` 说不要但历史里有 | `git rm --cached <file>` 后重新提交 |

**豁免的唯一入口是 [`.git-hygiene-allowlist`](.git-hygiene-allowlist)，格式 `glob  #  理由`。**
没有理由的豁免直接判失败。加豁免前先问三句：能被脚本重生成吗？能不能放到 `docs/` 或 `datasets/`
的正确位置？是否其实只该躺在垃圾场（那就不需要豁免）？

截图与验收证据：`docs/reviews/**/*.png` 默认不入库。确需对外展示的证据目录按上面口径登记豁免。

## 双语与文档

- 需要成对的文档范围见 [`i18n/README.md`](i18n/README.md)；**改中文必须同一轮改英文**。
- 新增对外文档时，同时把它登记进 [`i18n/docs-manifest.json`](i18n/docs-manifest.json)。
- 有 `*.en.md` 却没登记 = 判红，防止绕开门禁的单边副本。
- 过程资产（research / product / engineering / reviews / drafts）不需要翻译。
- 术语新增或变更：先改 [`i18n/glossary.md`](i18n/glossary.md) 并同步英文版，再批量改文档。
- Issue 与 PR 表单**不做两份**：GitHub 侧同一表单内并列双语，Gitee 侧纯中文模板。

## Commit 与 PR 约定

- 标题格式 `<type>(<scope>): <一句话改动>`，如 `feat(reduce): 支持逐条件回退验证`。
  `type` 用 `feat fix docs test refactor build chore`；`scope` 用包名或目录名。
- 正文用中文，写**为什么**改；行为变化写清"之前 / 之后"。
- 一次 commit 做一件事。不要把垃圾场里的临时文件顺手 `add -A` 进去。
- **push 与打 tag 归维护者**：贡献者提到自己的 fork 即可，维护者负责双远程同步。
- PR 里请如实区分：机器判据通过 ≠ 人工视觉复核通过。未执行的复核单列"未执行"，不要勾完成。

## 行为准则与安全披露

- 行为准则见 [`CODE_OF_CONDUCT.md`](CODE_OF_CONDUCT.md)。
- 安全问题**不要开公开 Issue**：按 [`SECURITY.md`](SECURITY.md) 私下报告。
- 涉及扫描对象页面内容的漏洞（截图泄露、路径穿越、远程模型外发）按高风险处理，优先修。
