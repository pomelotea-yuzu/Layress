# 盲测探针机制验证结果与立项依据补充

> 本文默认以中文维护。
>
> **状态**：2026-11-08 一次性实验记录。结果原件、预注册、揭盲表与机械对照记录均在 `artifacts/`（不入库）；
> 执行端输入在仓库外 `~/Developer/layress-blind-run/`。**本文不是首验方案批准，也不构成 GO 决策。**
> 人工真值未给（I11），机器对照不等于人工验收。

## 一、本轮做了什么

在仓库外的空白目录用四个自造 HTML 夹具，由无上下文的外部模型按协议执行两种点击通道、每页 3 次重复，只输出结构化观测。设计侧不参与执行端解读；判据跑前冻结于 `artifacts/blind-run-preregistration.md`，跑后不追改。

- 通道 P：页面上下文里对 `#target` 调用 `element.click()`。
- 通道 H：先 `document.elementFromPoint` 记录命中元素，再用真实指针点击该坐标。
- 环境：macOS 26.6 arm64 / Chrome Headless Shell 158.0.8127.18 / Playwright 1.65.0 (Node 22.22.2) / GLM-5.3-Flash。

## 二、结果

| 编号 | 通道 P（×3） | 通道 H（×3） | `hit_top_element` | `scroll_used` |
|---|---|---|---|---|
| A | fired | fired | `button #target` | false |
| B | fired | fired | `button #target` | true |
| C | fired | **not fired** | `div #overlay .overlay` | false |
| D | fired | fired | `button #target` | false |

四个编号 `unexecuted` 全为空；通道 H 自身 3/3 一致。C 是唯一分叉项：DOM `click()` 不经命中测试仍触发回调，真实指针落在全屏遮罩上不触发。

## 三、对照冻结判据

| 跑前判据 | 实测 | 结果 |
|---|---|---|
| 至少 1 个编号稳定 P=fired / H=not fired，且命中元素非 `#target` | C 3/3 | 满足 |
| 其余编号两通道一致 | A/B/D 3/3 | 满足 |
| 通道 H 重复不一致 ⇒ 不进第 2 步 | 无 | 不触发 |
| 负例分叉且不能用可接受滚动解释 ⇒ 复核夹具 | 无 | 不触发 |

**机械对照结论：第 1 步通过**，具备进入第 2 步（三态判定 + 条件缩减）的前提。是否进入归用户定夺。

## 四、结论边界（必须一起读）

1. **验的是机制，不是痛点**。11-07 证据闸门第一段问的是「当前强 Agent + 自带工具单独完成任务时会不会给假绿」；本轮执行端按协议逐项记录两个通道，没有自主选择点击方式，也没有给出可达性判定。尤其 Playwright `locator.click()` 自带 actionability 检查，被遮罩挡住时会拦下——这类调用不属于通道 P。要闭环"要不要开发"，还差一个极小的自主任务对照。
2. **自造夹具不冒充真实缺陷样本**。外部真实项目样本仍要补（调研 §7.3 至少一半外部）。
3. **夹具 v1 有观测缺陷**：顶层 `var status` 与 `window.status` 遗留全局属性冲突，`#status` 始终为 "idle"。执行端如实记录并改用 `window.__fired`，`fired` 判定不受影响。按冻结声明不修夹具、不重跑，本轮作为 v1 归档。
4. **人工真值未给**（I11）。机器 PASS 不等于人工通过。
5. 执行端把 Playwright 浏览器缓存放在 `~/Library/Caches/ms-playwright/`；这是实验工具缓存、不是产品代码，不计作违反 I12，但 spike 转产品代码时须按三端打包规划 §三 显式覆盖 `PLAYWRIGHT_BROWSERS_PATH`。

## 五、立项依据补充：先例存在但后来者靠顺手取胜

用户 2026-11-08 提出「想的简单一点，直接开始开发也是可以的」。核实到的社区案例（机制层面）：

| 后来者 → 先例 | 赢在哪个可测机制 |
|---|---|
| Linear → Jira | 300ms 渲染预算、键盘优先、主动砍掉无限自定义；设计中心从管理者换到一线开发者 |
| Figma → Sketch | 浏览器 + 实时协作写进架构；Sketch 后来补协作也回不去 |
| Tailwind → Bootstrap/Sass | 消掉命名与 HTML/CSS 间跳转的认知负荷 |
| uv → pip/Poetry/pyenv | 单一二进制、一个心智模型，保留 `uv pip` 兼容旧肌肉记忆 |
| Vite → CRA/Webpack | 冷启动、单配置、不用 eject |
| Playwright → Selenium | auto-wait、role-based locator、内置 trace |
| Raycast → Spotlight / Supabase → Firebase | 单一命令栏；底层标准 PostgreSQL，学到的东西带得走 |

**反例**：Mercurial 命令更一致、对新手更友好，仍输给 Git——赢的是 GitHub 的协作网络。单靠顺手不够，得让顺手可被别人看见。

**证据边界**：份额 / ARR / stars / 满意度来自媒体分析与厂商案例页，口径不统一，只作方向性证据；上表可核对的是产品机制。项目自身调研 §3.1、§2.1、§3.5 也早已承认「精确区间不是新概念」「这不构成无人做过的证据」，定位句（§5.2）的差异点一直是**交付形态**：已缩减的触发条件 + 可回放证据。

## 六、下一步（归用户定夺）

- **A**：按冻结判据进第 2 步，把机制做成最小探针（三态判定 + 条件缩减 + 可回放证据），痛点证据后补。
- **B**：先补一个自主任务对照（同一个干净会话，给它 C 页让它自己完成「取消」，看它选哪种点击方式、报不报成功），把 11-07 痛闸门的问题一起闭环，再进第 2 步。

## 证据指针

- 结果原件 `~/Developer/layress-blind-run/results/result.json`；副本 `artifacts/blind-run-result.json`
- 跑前冻结 `artifacts/blind-run-preregistration.md`；揭盲表 `artifacts/blind-run-answer-key.md`；机械对照 `artifacts/blind-run-gate-check.md`
- 以上四份均不入库（一次性证据，沿用 11-07 试用的本地证据口径）
