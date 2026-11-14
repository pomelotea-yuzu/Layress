# openspec/specs/ · 主规格

> 归属说明。这里放**已合入的能力口径**：一条需求配它的判据场景，长期有效。
> 当前已有 1 条能力：[`desktop-page-host/`](desktop-page-host/spec.md)，由 2026-11-14 归档的
> [add-tauri-page-host](../changes/archive/2026-11-14-add-tauri-page-host/proposal.md) 的 7 条 requirement 合入。
> 主 spec 与归档 change 的差集核对（`pnpm check:spec`）仍是待建门禁（见工作流质量门禁表）。

## 命名与结构

```
specs/
└─ <capability>/spec.md      能力名用 kebab-case，与 change 归档后的实际范围一致
```

首个活动 change 的能力名为 `desktop-page-host`；后续名称按实际增量确认，不恢复封存阶段的扫描 / 缩减等候选能力表，也不预建空规格。

## 进入门槛

- **只有从已归档 change 合入的 delta 才写这里**（权威性：源码 > `changes/archive/` > `specs/` > `docs/` > `AGENTS.md`）。
- 每条需求必须带判据；判据不可核验的需求属于"措辞偏好"，不进 spec。
- spec 不得与 [`docs/knowledge/invariants.md`](../../docs/knowledge/invariants.md) 冲突；
  需要放松某条不变量时，那是一个 change 的事，不是在这里改一句话。
- 存储类口径必须复述 I12/I13（数据根、禁止写系统用户目录、解析结果可见），否则实现会漂。

## 尚未落实

`check:spec`（主 spec 与归档 change 的差集校验）尚未接入公共门禁，workspace 也未建立。
共享 OpenSpec CLI 1.17.0 已安装并对首个 change 执行严格校验，调用见[工作流](../../docs/development/workflow.md)；这不代表主规格 / 归档差集检查已经自动化，当前仍需人工核对。
