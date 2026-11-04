# openspec/changes/archive/ · 已归档变更

> 归属说明。这里放**已收口**的 change，是整个仓库里权威性最高的文档落点（仅次于源码）。
> 当前为空——还没有归档过任何 change。

## 命名

```
archive/
└─ <YYYY-MM-DD>-<verb-led-kebab-name>/
    ├─ tasks.md
    ├─ design.md
    └─ evidence-index.md
```

日期 = 归档日期，不是开始日期。同一目录名不复用；重做的能力开新 change，旧的留着。

## 为什么这里比 docs/ 权威

- `design.md` 记录**当时为什么这样决定、否掉了哪些路线**——这类内容在代码里看不见，在 `docs/` 里会被后续
  整理冲淡。要推翻一个决定，先在这里找到它的原始理由。
- `evidence-index.md` 记录判据与实测证据的对应关系，是"这件事到底做没做"的凭据。
- 因此 [`docs/knowledge/decisions.md`](../../../docs/knowledge/decisions.md) 只写**当前口径的摘要 + 指针**，
  完整论证留在归档里；被推翻的旧决策不在 decisions 表里保留，只在归档里。

## 归档时的动作

1. 目录整体移入 `archive/`，加日期前缀。
2. delta 合入 [`../../specs/`](../../specs/README.md)，并核对既有场景没有丢失（只增不改是常见但危险的做法）。
3. 若引入了新的长期口径 → 更新 `docs/knowledge/{invariants,decisions}.md`（默认中文）。
4. 若踩了新的坑 → 按条目格式写进 `docs/knowledge/{engineering,runtime-probe}.md`（默认中文）。
5. `CHANGELOG.md` 加一条，`AGENTS.md` 状态卡改"活动 change / 最新归档"。
