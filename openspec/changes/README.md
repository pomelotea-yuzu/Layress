# openspec/changes/ · 活动变更

> 归属说明。这里放**正在进行**的 change，一个目录一个 change，动词开头的 kebab-case 命名。
> 当前为空：首个验证方案尚未批准（见 [`docs/knowledge/debt.md`](../../docs/knowledge/debt.md)），
> 立什么项是那个方案定了之后的事。

## 结构

```
changes/
└─ <verb-led-kebab-name>/
   ├─ tasks.md            拆步骤，每步带判据；完成状态只在这里维护
   ├─ design.md           方案、取舍、被否掉的路线与原因
   └─ evidence-index.md   实机/数据证据索引（判据 → 证据位置）
```

## 纪律

- **一个 change 只做一件事**。要改两条不相关的能力就拆两个 change，别裹成一笔。
- 判据要能反证：写清"已知坏样本必须判红"的对照，否则这份 change 不能证明工具有效（与 `check-i18n` 的
  `--selftest` 同一思路——永不命中的规则表现为全绿，实际什么都没检查）。
- 未执行项在 `tasks.md` 里如实写"未执行 + 原因（环境阻塞/需要人工/未排期）"，
  **不许勾完成**，也不许在 `AGENTS.md` 状态卡里写成已交付（I11）。
- 实机截图默认留本地：`.gitignore` 已排除 `docs/reviews/**/*.png`；证据索引只留**相对路径与判据描述**。
  确实要入库的证据按洁净度口径登记豁免 + 理由。
- 涉及存储路径的实现必须复述 I12/I13，并说明如何验证"没写 C 盘"。

## 归档

收口后整体移入 [`archive/`](archive/README.md)，delta 合入 [`../specs/`](../specs/README.md)。
**权威性：源码 > `archive/` > `specs/` > `docs/` > `AGENTS.md`**——判断进度只认前两者。

纯文档修订、错别字、链接修复不必立项。
