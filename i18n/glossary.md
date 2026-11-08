# 软件双语术语表

> 本表以中文说明维护，保留中英术语对照，无需英文说明副本。

版本：3（2026-11-08 补充「系统用户目录」三端口径）。适用范围：仓库门面与 `docs/` 双语版本、未来桌面 UI 与 CLI 人类可读输出、
错误文案的 `zh-CN` / `en-US` 资源。**代码标识、协议字段、错误码、CLI 参数、文件名原值不翻译。**

用词依据：调研原文 [`docs/research/2026-11-03-web-electron-ui-stress-test-tool-market-and-open-source-research.md`](../docs/research/2026-11-03-web-electron-ui-stress-test-tool-market-and-open-source-research.md)
的四类工作边界（§1.2）与建议定位（§5.2）。**同一概念只允许一种译法**；要改先改这张表，再批量改文档。

## 1. 产品与边界

| 概念 | 中文 | English | 约束 |
|---|---|---|---|
| 产品类别 | 界面压力测试 | UI stress testing (layout) | 首次出现必须补限定词，**不等于**负载/性能压测（load testing） |
| 关键操作可达性 | 关键操作可达性 | critical-action reachability | 产品核心口径；不写成"可用性 accessibility" |
| 视觉回归 | 视觉回归 | visual regression | 与已认可版本的差异；本产品不依赖人工基线，不要混为同类 |
| 几何异常检测 | 几何异常检测 | geometric anomaly detection | 只产出**候选**，见 §3 |
| 主动压力测试 | 主动压力测试 | active stress testing | 改变条件以扩大覆盖；与被动观察区分 |
| 缺陷确认与缩减 | 缺陷确认与复现缩减 | defect confirmation and reduction | 本产品重心；"缩减"不要写成"简化" |
| 人工正确截图基线 | 正确截图基线 | golden screenshot baseline | 本产品明确**不需要**，表述时保留否定式 |
| 本地优先 | 本地优先 | local-first | 数据默认不出本机；用远程模型必须另写数据流与成本 |
| Agent 友好 | Agent 友好 | agent-friendly | Agent 可发现能力、非交互调用并解析结构化结果；不要求产品内置模型 |

## 2. 压力条件

| 概念 | 中文 | English | 约束 |
|---|---|---|---|
| 视口宽度 | 视口宽度 | viewport width | 单位一律 CSS px，不写物理像素 |
| 内容区宽度 | 内容区宽度 | content width | 与视口宽度区分；被侧栏挤压的是后者 |
| 窄窗口 | 窄窗口 | narrow window | 条件之一，不是缺陷名 |
| 低窗口 | 低窗口 | short window | 高度维度；不写"矮窗口" |
| 长文本注入 | 长文本注入 | long-text injection | 长文件名、长路径等真实来源要写清 |
| 字体回退 | 字体回退 | font fallback | 与字体缩放分开记 |
| 文字放大 | 文字缩放 | text scaling / font zoom | 浏览器缩放与系统缩放要标明是哪一个 |
| 语言切换 | 语言切换 | locale switch | 文案换行会改变容器高度，属条件不属结果 |
| 准备步骤 | 准备步骤 | setup steps | 复现必需字段；不写"前置操作" |
| 界面状态 | 界面状态 | UI state | 侧栏展开、标签激活、错误提示出现等；必须与条件一起记录 |
| 排除范围 | 排除范围 | exclusion scope | 开发者声明不检查的部分 |
| 可接受滚动 | 可接受滚动 | acceptable scrolling | 需要滚动不算失败，滚到也够不着才算 |

## 3. 检测与判定

| 概念 | 中文 | English | 约束 |
|---|---|---|---|
| 候选 | 候选（几何候选） | candidate | **候选不等于缺陷**；报告措辞必须保留这一层 |
| 溢出 | 溢出 | overflow | 内容超出容器 |
| 裁切 | 裁切 | clipping | 被容器切掉，视觉上不可见 |
| 遮挡 | 遮挡 | occlusion | 被别的元素盖住；须记录遮挡方 |
| 重叠 | 重叠 | overlap | 两个元素互相压住 |
| 换行膨胀 | 换行导致的高度增长 | line-wrap growth | 语言/放大后弹窗变高的机制 |
| 判定 | 判定 | verdict | 一次确认的结论：confirmed / not-a-defect / inconclusive |
| 误报 | 误报 | false positive | 候选被判为缺陷但实际不是 |
| 漏报 | 漏报 | false negative | 真实缺陷未被发现 |
| 假绿 | 假绿 | false green | **判据本身失效**导致显示通过；与误报不同，属于工具缺陷 |
| 人工复核 | 人工复核 | human review | 机器 PASS ≠ 人工通过；未执行的复核单列 |
| 聚合 | 缺陷聚合 | defect aggregation | 同一根因的多条告警合并 |
| 忽略 | 忽略登记 | ignore entry | 已确认不处理的项，须带理由 |

## 4. 证据与回放

| 概念 | 中文 | English | 约束 |
|---|---|---|---|
| 扫描 | 扫描 | scan | 四个可独立使用模块之一 |
| 确认 | 确认 | confirm | 同上 |
| 缩减 | 条件缩减 | condition reduction | 把触发条件缩到最小仍可失败；**不承诺**自动抽取最小 HTML |
| 回放 | 回放 | replay | 只重放既有记录，**不新增**真实外部副作用 |
| 重跑 | 重跑 | rerun | 会真实执行；与 replay 严格区分，措辞不可互换 |
| 触发条件 | 触发条件 | trigger conditions | 宽窄区间之外还须含高度、状态、字体、缩放 |
| 可回放证据 | 可回放证据 | replayable evidence | 产品最终交付物 |
| 运行记录 | 运行记录 | scan run / run | 一次扫描的存档，含环境与产物清单 |
| 负例 | 负例 | negative case | 合理省略、滚动区域、抽屉菜单、浮层等**看起来坏但没问题**的样本 |
| 正例 | 正例 | positive case | 真实历史缺陷样本 |
| 数据集 | 数据集 | dataset | 调规则用的子集与最终评估子集**必须分开**，防过拟合 |

## 5. 原值不翻译

下列保持原样，中英两边写法完全一致：

- 代码与协议：类名、字段名、错误码、事件名、`viewport`、`overflow` 等 CSS 属性名
- 命令行：包名 `@layress/*`、CLI 参数与子命令、环境变量 `LAYRESS_*`
- 工具与格式：CDP 方法名、`DOM.getNodeForLocation`、`Screenshot.captureScreenshot`、`.jsonl`、`.png`
- 单位与数值：CSS px、DPI、DPR、百分比、时间、字节数
- 文件名与路径：包括 `docs/`、`datasets/` 下的样本名（样本名一旦改，内容哈希与回放对不上）

## 6. 存储与分发

| 概念 | 中文 | English | 约束 |
|---|---|---|---|
| 便携版 | 便携包 / 便携版 | portable build | 拷贝目录即迁移、删除目录即无残留；**不等于**"绿色版免安装"的宣传语 |
| 数据根 | 数据根 | data root | 产品自身目录下的数据根，或用户显式指定；解析结果必须可见（I13） |
| 数据目录 | 数据目录 | data directory | 与"安装位置"区分；前者随产品走，后者由用户选择 |
| 安装位置 | 安装位置 | install location | 由用户自主选择；安装器不提权、不改系统 PATH |
| 系统用户目录 | 系统用户目录 | system user directory | 三端统称：Windows `%APPDATA%`／`C:\Users\…`、macOS `~/Library`／`/private/var/folders`、Linux XDG 目录；不变量 I12 禁止默认写入，**不再写作「C 盘」** |
| 用户数据目录 | 用户数据目录 | user data directory (`userData`) | Electron/Chromium 术语，三端默认都落在系统用户目录 ⇒ **本产品必须覆盖**；`userData` 原值不翻译 |
| 会话数据 | 会话数据 | `sessionData` | 同上，Chromium 缓存目录；字段名原值保留 |
| 临时目录 | 临时目录 | scratch / temp directory | 必须落在数据根之下；`os.tmpdir()` 三端默认都指向系统临时目录，**不得当默认值** |
| 只读介质 | 只读介质 | read-only medium | 支持"产品目录只读 + 用户指定可写数据根"的组合 |
| 回落 | 静默回落 | silent fallback | 写不进时退到用户目录的行为；本产品**禁止**，见不变量 I12 |
