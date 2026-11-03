# apps/ · 应用壳

> 归属说明，**当前没有代码**。这里放消费包层能力的应用外壳。

| 目录 | 计划内容 |
|---|---|
| `desktop` | Electron 适配：结果浏览、扫描发起与回放查看；按调研 §5.3 **后置**于 CLI |

为什么后置：先把检测、确认、缩减、回放的可靠性做硬，UI 是包装（决策 D10）。
先写壳会得到一个"能演示但不能证明任何东西"的东西。

约定（建应用时同时生效）：

- **便携优先**：业务数据落产品目录下的 `data/`，运行期数据与日志落 `.layress/`；
  `userData` 与 Chromium `sessionData` 必须显式覆盖到数据根之下，不得用系统默认（I12、I13）。
  只改"数据目录"不够——`os.tmpdir()` 也在 C 盘（见 [`/docs/knowledge/engineering.md`](../docs/knowledge/engineering.md)）。
- 安装位置由用户选择；安装器不提权、不改系统 PATH；目标目录不可写时**报错并要求指定，不静默回落**。
- 打包产物与中间物不入库（`release/`、`dist/`、`build/` 已忽略）；构建记录与哈希进
  [`docs/engineering/reports/`](../docs/engineering/reports/)。
- 界面类验收必须附实机证据与人工复核结论，机器判据不能替代（I11）。
