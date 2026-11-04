# docs/architecture/ · 架构

> 归属说明。这里放**面向使用者与贡献者的架构文档**：模块划分、数据流、设计原则与边界。

## 这里放什么

- `overview.md`：`packages/{core,confirm,reduce,replay,cli}` 与 `apps/desktop` 的职责边界与数据流
- 机制类专题：几何候选如何产生、真实命中测试为什么是必需的、条件缩减的算法与停止条件、
  回放格式的字段与版本演进
- 存储与便携：数据根解析、`userData`/`sessionData` 覆盖、只读介质组合（不变量 I12–I13）

## 进入门槛

- 一篇架构文档只描述**已实现**的东西；未实现的写进 `openspec/changes/<name>/design.md`，不放这里。
- 默认只写中文；某篇确有对外阅读需求时，再提供同目录 `*.en.md` 并逐篇登记 [`双语清单`](../../i18n/docs-manifest.json)。
- 权威性低于源码与 `openspec/changes/archive/`（见 [`../README.md`](../README.md)）。
