# 开发者声明规约与证据契约 · 最小可用草案

> 本文默认以中文维护。
>
> **状态**：2026-11-09 草案，**可直接用的最小一版，不是论证文档**。不批准功能、不立 change、
> 不建 workspace、零代码。口径依据 [`openspec/config.yaml`](../../openspec/config.yaml) 的
> I1 / I3 / I4 / I6 / I9 / I11 / I14。
> **人工真值未给（I11）**：本文的纸面结果不是验收通过，机器或纸面 PASS 都不算人工复核。
>
> 这不是「要不要做 Layress」的第 N 轮讨论。用户 2026-11-08 的原意是「别再论证，先开始做」；
> 本文按此只交付三件东西：**开发者填什么、结果长什么样、拿现有观测填一份出来**。

## 一、开发者声明（跑前填，四项）

| 字段 | 含义 | 本次示例值（B 轮夹具） |
|---|---|---|
| `target_action` | 关键操作是什么，成功判据是什么 | 点击 `#target` 后 `window.__fired === true`，且 `#status` 从 `idle` 变 `fired` |
| `preparation` | 到达该操作前必须走的准备步骤 | 先在模态内点「取消」关闭遮罩，再点 `#target` |
| `scroll_policy` | 哪些滚动算可接受，哪些算受阻 | 本夹具不需要滚动；滚动后可达不算缺陷 |
| `exclusions` | 明确不算缺陷的情形 | 模态按设计阻止背景操作属于预期；关掉遮罩后可达属正常流程 |

> **来源标注（I4）**：上表是**事后按 11-08 B 轮执行端的转录补写**的，不是开发者跑前给的声明。
> 因此它只能演示字段形状，不能当作该轮实验的可信真值；缺失来源一律写 `unknown`，不猜测补造。

## 二、三态判定（I1）

- `confirmed`：声明的关键操作在**准备步骤全部完成后**仍不可达，触发条件四元组完整，且人工真值确认。
- `not-a-defect`：不可达属于 `exclusions`，或存在 `scroll_policy` 允许的替代入口。
- `inconclusive`：声明缺失或冲突、真值未给、四元组不全、观测通道失效、未执行复核 —— 任一命中即判此项。

**不许把 `inconclusive` 写成通过，也不许为得到 `confirmed` 丢弃必要前件**（I9）。

## 三、证据契约（有版本的结构化结果）

```json
{
  "schema_version": "1.0-draft",
  "run_id": "blind-task-b-2026-11-08",
  "declaration_source": "post-hoc-draft",
  "declaration": {
    "target_action": "#target click -> window.__fired === true",
    "preparation": "close modal via its Cancel, then click #target",
    "scroll_policy": "no scroll required",
    "exclusions": "modal blocking background controls is by design"
  },
  "conditions": {
    "viewport_css_px": "unknown",
    "state": "modal-open",
    "font_zoom": "unknown",
    "language": "unknown"
  },
  "verdict": "inconclusive",
  "verdict_reasons": [
    "declaration was written after the run, not supplied before it",
    "three of four trigger-condition fields are missing",
    "human ground truth not supplied"
  ],
  "unreplayable": true,
  "unexecuted": ["human review", "live replay", "competitor run"],
  "evidence_refs": ["artifacts/blind-task-result.md", "artifacts/blind-task-gate-check.md"]
}
```

字段口径：`conditions` 四元组缺任一项 ⇒ `unreplayable: true`（I3）；`verdict` 与运行是否成功分开，
`unexecuted` 单列（I11）；`evidence_refs` 用仓库内相对路径，导出时才物化绝对路径。

## 四、用 11-08 B 轮观测填好的实例

上面这份 JSON **就是填好的实例**，只填了已知事实：执行端先点 `#target` 被遮罩拦下并超时，
再关遮罩后点成功，两个验证信号都过；模型 / 版本、步骤数、耗时、视口、DPR、语言、字体缩放未报。

按 §二 判 `inconclusive`，理由三条：**声明是事后补的**（I4）、**四元组缺 3 项**（I3）、
**人工真值未给**（I11）。B 轮执行端原本两解 —— 「实锤 Bug」还是「正常流程」；规约没有强行选一边，
而是把「为什么现在不能判」写清楚。这就是规约的作用。

## 五、这份回填暴露了什么（事实，不是结论）

1. 现有两轮盲测结果**不是可回放证据**：四元组只记了界面状态，尺寸 / 字体缩放 / 语言全缺。
2. B 轮提示的「真正缺的是判定口径」，具体缺的是**跑前声明 + 条件记录**，不是更多检测代码。
3. 下一次真要跑，最小补丁是：**跑前把声明四项和四元组写进协议**，跑后按 schema 出结果。
   不需要造探针、不需要建 workspace、不需要再写一份「要不要做」的文档。

## 六、边界与未执行项

- 草案，不是规范定稿、不是需求文档、不是测试用例、不是配置文件格式、不是检测器。
- 未重跑夹具、未补人工真值、未做实机回放、未运行竞品、未联系维护者。
- 未写产品代码、未建 workspace、未立 change、未 push。按 [D18](../knowledge/decisions.md) 字面，
  这些仍在停止范围内；要开始写产品代码须用户显式推翻 D18，届时另立 change，不复活 D17。
- 草案若要晋升为当前依据：补至少一份**跑前**由开发者给出的声明 + 一份四元组完整、人工真值已给的
  真实任务观测，再按 [`docs/README.md`](../README.md) 的晋升规则改稳定主题名并修复引用。
