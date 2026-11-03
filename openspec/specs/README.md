# openspec/specs/ · 主规格

> 归属说明。这里放**已合入的能力口径**：一条需求配它的判据场景，长期有效。
> 当前为空——还没有已归档的 change，因此也没有已合入的口径。

## 命名与结构

```
specs/
└─ <capability>/spec.md      能力名用 kebab-case，与 change 归档后的实际范围一致
```

候选能力名（首个 change 立项时确认，不要提前扩）：`stress-conditions`、`geometry-scan`、
`action-reachability`、`condition-reduction`、`evidence-replay`、`report-format`、`data-storage`、`cli-interface`。

## 进入门槛

- **只有从已归档 change 合入的 delta 才写这里**（权威性：源码 > `changes/archive/` > `specs/` > `docs/` > `AGENTS.md`）。
- 每条需求必须带判据；判据不可核验的需求属于"措辞偏好"，不进 spec。
- spec 不得与 [`docs/knowledge/invariants.md`](../../docs/knowledge/invariants.md) 冲突；
  需要放松某条不变量时，那是一个 change 的事，不是在这里改一句话。
- 存储类口径必须复述 I12/I13（数据根、禁止写系统用户目录、解析结果可见），否则实现会漂。

## 尚未落实

`check:spec`（主 spec 与归档 change 的差集校验）依赖 pnpm workspace 与 openspec CLI，尚未建立；
建好之前，本目录的一致性由人工核对保证。
