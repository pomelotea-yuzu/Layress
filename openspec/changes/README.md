# openspec/changes/ · 活动变更

> 归属说明。这里放**正在进行**的 change，一个目录一个 change，动词开头的 kebab-case 命名。
> 当前活动 change：[add-tauri-page-host](add-tauri-page-host/proposal.md)（2026-11-10 建立）。首增量为 Tauri 单窗口页面承载、基础视口、真实采集与原地预览；尚未开始编码或实机验证，其余阶段范围不自动获批。

## 活动入口与归属

| change | 范围与入口 |
|---|---|
| `add-tauri-page-host/` | [提案](add-tauri-page-host/proposal.md)、[设计](add-tauri-page-host/design.md)、[任务](add-tauri-page-host/tasks.md)、[证据计划](add-tauri-page-host/evidence-index.md)；其 `specs/desktop-page-host/` 存放[待实现规格增量](add-tauri-page-host/specs/desktop-page-host/spec.md)，归档前不冒充主规格 |

## 结构

```
changes/
└─ <verb-led-kebab-name>/
   ├─ .openspec.yaml      schema 与建立日期
   ├─ proposal.md         为什么做、范围、能力与影响
   ├─ design.md           方案、取舍、被否掉的路线与原因
   ├─ specs/<capability>/spec.md  待实现规格增量，归档后合入主规格
   ├─ tasks.md            拆步骤，每步带判据；完成状态只在这里维护
   ├─ review.md           用户切换另一 Agent 审阅产出，原编写 Agent 处理后提交
   └─ evidence-index.md   实机/数据证据索引（判据 → 证据位置）
```

## 纪律

- **一个 change 只做一件事**。要改两条不相关的能力就拆两个 change，别裹成一笔。
- **写完 change → 用户切换另一 Agent 审阅 → 原编写 Agent 修订、检查、提交**：另一 Agent 在对应目录产出 `review.md`，用户回到原编写 Agent 后，由其处理意见并通过必要检查，即可本地提交，无需再次确认。不得用编写者自审、空占位或机器 PASS 跳过该流程；由用户切换 Agent，不要求自动生成子 Agent。Agent 审阅不替代人工验收，用户保留最终决定权。
- 判据要能反证：写清"已知坏样本必须判红"的对照，否则这份 change 不能证明工具有效（与 `check-i18n` 的
  `--selftest` 同一思路——永不命中的规则表现为全绿，实际什么都没检查）。
- 未执行项在 `tasks.md` 里如实写"未执行 + 原因（环境阻塞/需要人工/未排期）"，
  **不许勾完成**，也不许在 `AGENTS.md` 状态卡里写成已交付。
- 实机截图默认留本地：`.gitignore` 已排除 `docs/reviews/**/*.png`；证据索引只留**相对路径与判据描述**。
  确实要入库的证据按洁净度口径登记豁免 + 理由。
- 涉及存储路径的实现必须说明如何满足 `AGENTS.md` 的便携要求，并验证没有默认写系统用户目录（按实测平台列具体路径，不只列 Windows）。

## 归档

收口后整体移入 [`archive/`](archive/README.md)，delta 合入 [`../specs/`](../specs/README.md)。
**权威性：源码 > `archive/` > `specs/` > `docs/` > `AGENTS.md`**——判断进度只认前两者。

纯文档修订、错别字、链接修复不必立项。
