# 贡献指南

<p><a href="CONTRIBUTING.en.md">English</a> | 简体中文</p>

感谢参与。本仓对**仓库洁净度、文档链接和对外双语成对**有硬性门禁，请先阅读相关规则再提第一个 PR。

## 先读什么

1. [`AGENTS.md`](AGENTS.md) —— 接手索引：项目一句话、状态卡、纪律红线、坑该写在哪。
2. [`docs/product/product.md`](docs/product/product.md) 与 [`docs/product/roadmap.md`](docs/product/roadmap.md) —— 当前产品要求与正式阶段索引。
3. [`docs/knowledge/decisions.md`](docs/knowledge/decisions.md) —— 已定的技术选择与其推翻条件。
4. [`i18n/glossary.md`](i18n/glossary.md) —— 中英用词一律以它为准。

想法类讨论先对齐实际使用任务与范围。用户意见、实际观测和自动检查需保留来源，Agent 声称修好不自动等于验收完成。
旧不变量编号保留为历史参考，当前工程与存储约束以 `AGENTS.md` 和 `openspec/config.yaml` 为准。

## 环境准备

```bash
# 每个克隆必须做一次：不挂 hook，洁净度门禁就只是 CI 里的一次事后报错
git config core.hooksPath .githooks

node scripts/check-repo.mjs --all      # 洁净度、全库链接、双语及门禁自测
node scripts/check-repo.mjs --staged   # 暂存后检查实际提交快照，pre-commit 同此
```

Node ≥ 20。**当前开发机为 macOS arm64 / zsh**，CI runner 为 ubuntu；门禁脚本三端通用，本地与云端跑同一条命令链（开发机平台不等于产品目标平台）。pnpm workspace 与 `pnpm check:ci` 尚未建立（见 [`docs/development/workflow.md`](docs/development/workflow.md)），
所以现阶段没有 build/test 步骤。

桌面载体已确定为 Tauri；实现时将涉及 Rust 原生层与 TypeScript 前端。具体工具链、WebView 前置依赖和安装步骤待实现 change 核实，本轮未安装；新增 Rust / 前端检查仍须接入同一条公共门禁。

## 提交流程

行为变更走 openspec（口径见 [`docs/development/workflow.md`](docs/development/workflow.md)）：

1. 开 Issue 或在已有 Issue 下对齐问题陈述。
2. 建 `openspec/changes/<change-name>/`：`tasks.md` 拆步骤，`design.md` 写方案与取舍。
3. 编写 Agent 写完 change 后，等待用户切换到**另一 Agent 审阅并在对应目录产出 `review.md`**；用户再回到原编写 Agent，由其读取并处理意见、修订 change、完成必要检查，随后即可本地提交，无需再次确认。不得用编写者自审、空占位或机器 PASS 替代审阅；Agent 审阅不替代人工验收，用户保留最终决定权。
4. 按任务实施，满足上述审阅前置条件后，**仅在确有必要/有价值或完成一小段开发时做本地 commit**，不为凑数提交；判据与实测证据留在 change 目录内。
5. 归档进 `openspec/changes/archive/<date>-<change-name>/`，合入主 spec。

纯文档修订、错别字、链接修复可直接提 PR，不必立项。

## 仓库洁净度

本仓严格控制进入历史的内容。**blob 一旦提交就删不干净**，所以拒绝由脚本执行，不靠 reviewer 眼力。

| 规则 | 触发 | 做法 |
|---|---|---|
| 垃圾场路径 | `artifacts/`、`.layress/`、`output/`、`.tmp*`、`node_modules/` 等出现在暂存区 | 手写结论改放 `docs/` 正确子目录；产物留在垃圾场即可 |
| 二进制/归档后缀 | `.msi .exe .dll .zip .7z .mp4` 等 | 只提交生成脚本，或登记豁免 |
| 体积 | 单文件 > 512 KB 判红，> 256 KB 警告 | 拆分、压缩为可再生成、或抬高豁免 `size=` |
| 凭据形态 | 私钥块、`sk-…`、`ghp_…`、`AKIA…`、赋值型 token | 移除；密钥由用户在应用内填写，仓库只留占位符 |
| 机器绝对路径 | 源码/配置里出现 `C:\Users\…`、`/Users/<name>/…`、`/home/<name>/…` | 改相对路径或运行时解析的数据目录 |
| 忽略却已跟踪 | `.gitignore` 说不要但历史里有 | `git rm --cached <file>` 后重新提交 |

**豁免的唯一入口是 [`.git-hygiene-allowlist`](.git-hygiene-allowlist)，格式 `glob  #  理由`。**
没有理由的豁免直接判失败。加豁免前先问三句：能被脚本重生成吗？能不能放到 `docs/` 或 `datasets/`
的正确位置？是否其实只该躺在垃圾场（那就不需要豁免）？

截图与验收证据：`docs/reviews/**/*.png` 默认不入库。确需对外展示的证据目录按上面口径登记豁免。

## 双语与文档

- 文档默认中文；当前中英双版本仅覆盖根目录 README、CONTRIBUTING、SECURITY、CODE_OF_CONDUCT、CHANGELOG，**这些登记文件改中文时同轮改英文**。
- `AGENTS.md`、`docs/`、经验库、方案与 `i18n/` 维护说明只需中文；确有对外阅读需求时再逐篇提供英文并登记进 [`i18n/docs-manifest.json`](i18n/docs-manifest.json)。
- 新英文版未登记会判红；已停止维护的内部译本只在 `docs/drafts/` 本地归档，不入库，也不要求同步。
- 术语新增或变更：先改 [`i18n/glossary.md`](i18n/glossary.md)，再更新使用该术语的对外中英文件；术语表本身无需英文副本。
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
