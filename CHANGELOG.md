# 更新日志

<p><a href="CHANGELOG.en.md">English</a> | 简体中文</p>

本文件遵循 [Keep a Changelog](https://keepachangelog.com/zh-CN/1.1.0/) 与
[语义化版本](https://semver.org/lang/zh-CN/)。**按版本记录面向使用者的变更**，
变更类型用 `新增 / 修复 / 变更 / 已移除 / 已废弃`。

开发过程与逐笔改动以 `git log` 为准；决策、坑、调研与实验结论分别见
[`docs/knowledge/decisions.md`](docs/knowledge/decisions.md)、
[`docs/knowledge/engineering.md`](docs/knowledge/engineering.md)、
[`docs/research/`](docs/research/README.md) 与 [`AGENTS.md`](AGENTS.md) §二 状态卡。
2026-11-03～11-08 的仓库搭建过程条目原放在本文件「未发布」下，现回归各主题落点；
原文可从 Git 历史查（`git log -- CHANGELOG.md`）。

## 未发布

尚无发行版。当前仓库只有目录结构、接手索引、门禁与调研文档，没有可运行产品、没有 npm 包、没有 tag。
状态以 [`AGENTS.md`](AGENTS.md) §二 状态卡与 `git log` 为准。

## 版本与发布约定

- 版本从 `0.1.0` 起算；`0.x` 阶段允许破坏性变更，但必须在本文件写明影响。
- 候选版本用 `-rc.N`；打包、哈希、验收记录与发布正文进 `docs/engineering/reports/`。
- **首验未收口前不打 tag、不发 npm 包**：数据集、对照组与继续/收缩门槛见
  [`docs/research/`](docs/research/README.md) 与 [`docs/knowledge/debt.md`](docs/knowledge/debt.md)。
- push 与 tag 归维护者；双远程（GitHub / Gitee）同步由维护者执行。
