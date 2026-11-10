# 开发者操作系统分布：平台适配的背景核查

> 调研日期：2026-11-10。状态：**公开调查与统计资料核查，未做产品实测、访谈、需求抽样或官方原始数据逐项复核**。
> 触发问题：会用 Layress 这类本地界面验收助手的开发者主要使用什么操作系统，平台适配顺序该怎么排。
> 本文是**过程记录 / 背景资料**，不是平台范围决定，也不恢复旧阶段的三端发行承诺（见[阶段封存](2026-11-09-phase-closure.md)、[接手索引](../../AGENTS.md) §四）。
> 口径沿用[受众与常用界面检查流程](2026-11-09-audience-and-workflow-research.md)：公开调查只能作背景，不能推算本产品用户规模、采用意愿或市场需求。
> 现行产品依据：[产品方向与使用方式](../product/product.md)；平台相关未决项：[欠账](../knowledge/debt.md)。

## 当前结论

1. **没有针对本产品细分人群的操作系统统计。**「用 AI Agent 开发 Web / Electron 界面、愿意安装本地桌面验收工具的个人开发者与小团队」没有独立的平台分布调查；现有数据都是外圈人群。
2. **三层外圈数据合起来是双峰，不是单一主流平台**：按绝对人数 Windows 最大；按早期采用者密度、独立开发者与开源讨论热度，macOS 最集中；Linux 桌面占比最小。两组结论可以同时成立，因为样本圈层不同。
3. **平台适配顺序属建议、未定稿**：首版优先保证 Windows 与 macOS，Linux 桌面后置但 CI 继续跑 Ubuntu。归用户定夺，时点为首个页面承载增量进入实机验证之前；本文不改动现有「macOS arm64 首验」建议，见下节。
4. **三种「平台」不能混为一谈**：开发机平台、产品发行适配平台、被测应用的目标平台。首版工作台跑在开发机上，适配顺序才按开发机分布走。
5. **本文不证明任何一端有真实用户。**要变成证据，需要竞品 issue 里的自报平台或首个可用版本的真实平台分布，见「未执行项」。

## 数据来源与口径

来源等级：**A** = 官方页面或官方报告正文；**B** = 媒体 / 第三方对原始数据的转述，本轮未核到原始图表或 CSV。
检索截止 2026-11-10；网页抓取原件按现有做法只留 `artifacts/`、不入库。

| # | 来源与等级 | 样本 / 时间 | 关键数字 | 口径与限制 |
|---|---|---|---|---|
| S1 | [Stack Overflow 2025 · Technology](https://survey.stackoverflow.co/2025/technology)（官方页面 A）与[方法说明](https://survey.stackoverflow.co/2025/methodology)（A） | OS 题 31,569 份（页面标注 64.4% 应答率）；整份调查 49,009 份，2025-05-29～06-23，177 国 | 页面文字仅给出 Ubuntu 个人使用 28%（对比 Android 29%）；题目原文为「What is the primary operating system in which you work?」 | 官方页面文字**未列出** Windows / macOS / WSL 等完整百分比，图表数据未从页面正文取得；见下方「来源冲突」 |
| S2 | [Windows Latest 2026-10-05 报道](https://www.windowslatest.com/2026/10/05/microsoft-distinguished-engineer-fires-back-at-anti-windows-11-rage-bait-says-more-developers-use-windows-than-linux-or-macos/)（B）、[FOSS Post 对 2025 OS 题的整理](https://fosspost.org/linux-it-job-market-demand/)（B） | 转述 Stack Overflow 2025 OS 题 | 职业使用：Windows 49.5%、macOS 32.9%、Ubuntu 27.7%、WSL 16.8%、非 WSL Linux 16.7%；个人使用：Windows 56.7%、macOS 32.7%、Ubuntu 27.8%、非 WSL Linux 17.6% | 两篇均称该题**允许多选**、分 personal / professional 两列，因此加总远超 100%，不是市场份额；数字为二手转述 |
| S3 | 同上 Windows Latest 报道（B）转述 The Pragmatic Engineer 社交投票 | Gergely Orosz 在 X / LinkedIn 的自愿投票 | macOS 61.6%、Linux 24.2%、Windows 12.8%（X）；LinkedIn 结果接近 | 样本偏 VC 创业公司与大厂，作者本人承认不代表全行业；本轮未读原始投票页 |
| S4 | [StatCounter 中国桌面 OS 份额](https://gs.statcounter.com/os-market-share/desktop/china)（官方页面 A，数值取 2025-05 索引快照 B） | 网页流量统计，非开发者调查 | Windows 79.91%、OS X 9.03% + macOS 1.51%（合计约 10.5%）、Linux 1.38%、Unknown 8.18% | 浏览器流量份额，**不等于开发者占比**；OS X 与 macOS 被分开归类；页面为动态图表，未逐月核对，另有月份快照数值差异较大 |
| S5 | [JetBrains 2025 报告首页](https://www.jetbrains.com/lp/devecosystem-2025)（A）与[官方解读](https://blog.jetbrains.com/de/research/2025/10/state-of-developer-ecosystem-2025)（A） | 24,534 份、2025-04～06，按地理 / 雇佣 / 语言 / 产品使用加权，JetBrains 用户可能偏多 | 85% 开发者经常使用 AI 编码工具、62% 至少依赖一种 AI 助手 / Agent / AI 编辑器；报告页列中国大陆为最大单一受访地区（约 20%） | 提供「用 Agent 写代码已是主流背景」的旁证；**该报告未公布操作系统分布**，不能用来推算平台 |
| S6 | [JetBrains 中国开发者专文](https://blog.jetbrains.com/research/2026/01/insights-into-china-s-developer-landscape-key-trends/)（A） | 同一调查的中国子样本 | Java 为 60% 中国开发者首选语言；近 1/4 职业开发者参与小程序开发 | 说明国内技术栈与国际样本有别，但同样**没有 OS 数据**；不据此推断国内开发者的系统占比 |

### 来源冲突：S1 与 S2 的题目口径不一致

S1 官方页面把 OS 题写作「primary operating system」（字面像单选），S2 两家二手来源却称该题允许多选、并给出 personal / professional 两列。
本轮**没有**下载 [Stack Overflow 2025 原始 CSV](https://stackoverflow.seektable.com/) 或读取图表数据，无法判定哪一方准确。
因此本文所有 S2 百分比只按「二手转述的、口径待核的排序信号」使用：可以说明 Windows 在全体开发者中排第一、macOS 次之、Linux 桌面更小，**不可**用于计算任何比例、差值或 Layress 用户数。
后续若这份数字要进对外文案或平台决策，必须先核原始 CSV 的题目类型与分母。

## 对平台适配的含义（建议，未定稿）

本产品采用 Tauri + 系统 WebView（[产品依据](../product/product.md)），三端引擎不同，平台顺序直接决定实测成本：Windows 为 WebView2、macOS 为 WKWebView、Linux 为 WebKitGTK。

| 顺序 | 建议 | 依据 | 反证 / 代价 |
|---|---|---|---|
| 1 | Windows | 全体开发者人数盘最大（S2）；中国桌面流量压倒性（S4）；Gitee 镜像侧受众以国内为主 | WebView2 的窗口 resize、DPR 与字体渲染问题最多，需要真实 Windows 环境才能测；当前开发机是 macOS arm64 |
| 2 | macOS | 当前开发机、海外开源与 AI 工具早期采用者密度高（S3）；演示与录屏观感较好，对「就业作品集与开源影响力」目标权重不低 | S3 是偏创业 / 大厂的自愿投票，不能外推到全部用户；macOS 大盘占比小（S4） |
| 3 | Linux 桌面 | 桌面份额最小（S4），且 WebKitGTK 的图形与合成问题在 Tauri 官方排障资料中最复杂 | 开源社区声音可能偏高，后置不等于放弃；CI runner 仍是 Ubuntu，门禁链必须继续在 Linux 通过 |

四条必须分开记的边界：

- **首验平台 ≠ 发行适配优先级**。[欠账](../knowledge/debt.md)现有条目建议 macOS arm64 首验，理由是开发机便利；本文的排序说的是「先为哪些用户做扎实」，两者的判据不同，不互相推翻，也都不构成已批准的平台范围。
- **CI 是 Linux ≠ 用户是 Linux**。CI 跑门禁是工程约束，不能拿来当目标用户证据。
- **WSL 不是第四个桌面平台**。S2 的 WSL 16.8% 指的是 Windows 宿主内的 Linux 工具链；Tauri 桌面窗口仍跑在 Windows 上。但 CLI 若被 WSL 内的 Agent 调用，路径、IPC 与写盘边界要单独验证。
- **开发者系统 ≠ 被测应用目标系统**。产品文档已要求浏览器设备模拟与真机验证分开标明；本文只讨论工作台运行在哪个开发机上。

## 未执行项

- 未下载 Stack Overflow 2025 原始 CSV / 读取 OS 图表，S1 与 S2 的口径冲突未消解。
- 未逐月核对 StatCounter 动态图表；未取得按「开发者」而非「网页流量」拆分的国内 OS 数据。
- 未核查相近产品（ProofShot、React Grab、Agentation 等，见[先例与差异化核查](2026-11-10-integrated-review-differentiation.md)）issue 中用户自报的操作系统；这是把推断变成细分证据的最低成本路径。
- 未做访谈、问卷、竞品实跑、产品实机验证，未统计任何一端真实用户或需求强度。
- 未决定平台范围、未新增 change、未改产品依据与状态卡、未写入经验库的已验证事实（实机硬事实才进 `runtime-probe.md`）。

## 结论怎么升级

1. 先核 S1 / S2 的题目口径；若原始 CSV 支持二手数字，再把它从「排序信号」升为可比数据。
2. 翻竞品 issue 与讨论，按「自报平台」计数，明确这是便利样本、不是抽样调查。
3. 首个可用版本让 CLI 的结构化输出带平台标识（属于实现 change 范围，本文不预设字段）。
4. 平台范围仍由用户定夺；任何「支持某平台」的对外说法，都以实机验证结果为准，不以本文数据为准。
