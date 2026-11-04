# 首轮社区证据核查：需求、替代方案与真实缺陷线索

> 检索与核查日期：2026-11-04。状态：**部分完成的桌面调研**，不构成开发立项或首验方案批准。
> 对应[简短路线图](../product/roadmap.md)阶段 1；补充[立项调研](2026-11-03-web-electron-ui-stress-test-tool-market-and-open-source-research.md)，不替代原文。
> 后续：GitHub 授权已完成，新增核查与当前待续项见[第二轮 GitHub 证据](2026-11-04-github-evidence-round-two.md)；下文保留本轮结束时的证据边界。

## 一、本轮结论与证据边界

- 已核查 7 条中英文社区讨论的问题、回答或评论，另核查 1 条真实缺陷记录及 4 份官方资料。它们支持继续研究复核成本、状态和环境控制，但不足以确定首批用户、发生频率或独立工具价值。
- 不能把截图比较困难直接当成对 Layress 的需求：其中有些问题已有现成办法，且与关键操作失效不是同一任务。
- 一个真实缺陷说明：按钮裁切可能与重复进入状态和渲染时序有关，不能预设都是窄窗口问题。该案例不属于首版平台，尚不能作为评估样本。
- **未执行**：竞品实跑、GitHub 仓库活跃度与 Issue / PR 核查、缺陷本地复现、维护者访谈、试用与付费验证。没有对外发帖或联系维护者。
- 调研继续；本轮没有批准探针选型、样本规模、预算或实现范围。

## 二、检索与核查方法

围绕“现有检查流程、误报与维护成本、已有方案足够的反证、真实操作受阻”检索，中文重点查看测试之家与 V2EX，英文查看 Stack Overflow、Hacker News 与 Reddit。检索词包括 `视觉 回归 测试 误报`、`视觉 基线 维护`、`testing responsive layout`、`screenshot comparison`、`playwright baseline screenshots update`。

搜索摘要仅用于找线索。中文讨论通过正文及评论核查；Stack Overflow 页面直接访问返回 403，改用官方公开 Stack Exchange API，读取问题正文、创建时间、回答正文和采纳状态。官方资料读取正文。Mozilla Bugzilla 通过公开 REST API 读取状态、时间和评论。网络请求保留 TLS 验证；部分直连失败的请求改用现有代理核查。

下列结果是定向便利样本，有站点、语言、存活页面与检索词偏差；多条讨论涉及同一工具，不能视为独立市场样本。历史记录只证明当时有人报告过问题，不证明最新版本仍有相同问题。搜索返回的无关页面和重复结果不计为证据。

## 三、社区讨论：需求与反证同时记录

日期为主题创建日期；后续回答时间在表内另列。个人反馈与作者自述均未由本项目实跑验证。

| 编号与原文 | 核查到的内容 | 能支持什么，以及不能推出什么 |
|---|---|---|
| C1 · [海豚 Web UI 自动化测试平台](https://testerhome.com/topics/4099)，2016-02-05 | 工具作者在正文及 #8（03-07）说明动态数据造成 diff 噪声、lazyload 影响截图时机，并介绍相同数据下人工批准与更新基线的流程；#9（06-15）用户报告接口不稳导致长流程脚本失败，但未明确使用海豚 | 支持调查数据与时序控制成本；不能推算误报率，也不能认定无基线扫描就能解决接口问题。已有 mock、基线和等待机制也是替代方案 |
| C2 · [一种新的测试方法：视觉感知测试](https://testerhome.com/topics/33832)，2022-07-15 | 评论质疑随机业务和条件组合带来的图片准备投入、像素比较局限；正文的商城按钮遮挡是说明性假设，没有具体缺陷记录 | 支持追问准备与维护成本；按钮遮挡示例**不计为真实缺陷**，也不能把评论质疑当成实际放弃工具的案例 |
| C3 · [AUITestAgent 介绍与讨论](https://testerhome.com/topics/40255)，2024-07-15 | 07-18 评论询问控件偏移、布局改坏能否拦截；作者 07-19 回答尚未针对一般布局问题特化，准确率不高，实际应用需专项检测 | 有具体能力需求的提问和项目方承认的限制；不是独立使用效果。正文的功能错误实验指标不等于布局检测指标，不能外推到 Layress |
| C4 · [Fuzzy screenshot comparison with Selenium](https://stackoverflow.com/questions/14284915/fuzzy-screenshot-comparison-with-selenium)，2013-01-11 | 提问者希望忽略轻微渲染伪影；2013-03-28 的[采纳回答](https://stackoverflow.com/a/15682469)自述最终使用 ImageMagick 的 PAE 比较，认为适合自身场景 | 同时保留困难与已解决的反证；历史像素比较问题不证明需要新工具，更不证明现代工具仍未解决 |
| C5 · [Playwright 有界面与无界面截图差异](https://stackoverflow.com/questions/77245555/how-to-ignore-eliminate-difference-playwright-screenshots-in-headed-and-headless)，2023-10-06 | 用户报告元素截图从 200×167 变为 200×168；有回答建议固定裁切或 viewport，另有 2024-03-20 回答称加 500ms 等待对自身有效；本次 API 返回无采纳回答 | 支持环境、稳定时机和截图尺寸需分别记录；根因及修复未获提问者确认，不能把固定等待当成通用解决方案 |
| C6 · [截图尺寸不匹配](https://stackoverflow.com/questions/77867240/screen-size-mismatch-causing-screenshot-comparisons-to-fail)，2024-01-23 | 提问者自述约 100 页、约 60% 比较因尺寸不同失败；[回答](https://stackoverflow.com/a/77888185)推测 17px 差异来自滚动条，并建议控制动画、指针位置或截取元素；无采纳回答 | 支持继续核查规模化准备与排障负担；百分比仅是该用户自述，滚动条根因也未确认，不是行业统计 |
| C7 · [Playwright trace 与 HAR 的区别](https://stackoverflow.com/questions/72139571/how-does-playwright-tracing-file-compare-to-har-file)，2022-05-06 | [回答](https://stackoverflow.com/a/72141480)说明 trace 已有截图、DOM 快照、日志与方法调用信息；官方资料 O2 进一步核实这些能力 | 属于替代能力证据。不能把带截图和步骤的报告本身当作差异；trace 也不等于自动搜索压力条件或缩减触发条件 |

## 四、真实缺陷线索：先验证适用范围

**B1 · [Mozilla Bug 2057680](https://bugzilla.mozilla.org/show_bug.cgi?id=2057680)：Container Settings dialog cropped after first use, hiding Cancel/Done buttons。**

- 创建于 2026-07-24；本轮读取状态为 `RESOLVED / FIXED`，目标里程碑为 `155 Branch`，最近变更时间为 2026-07-30。状态来自维护系统，不代表本项目已复验。
- 评论 #0：报告者在 macOS 15.7.5、Nightly 154 中首次打开容器设置正常，再次打开同类弹窗时看不到 Cancel / Done；手动调整面板大小可让按钮重新出现。报告者明确认为窗口大小似乎不是决定因素，关闭设置标签或窗口后也会再现。
- 评论 #1：另一条反馈称调整大小后，下一次打开仍可能过小而裁切按钮。评论 #2 给出回归区间；#4、#5 记录补丁和落地；#9 的维护者申请说明称修复是通过模块导入让弹窗同步渲染，并自报已覆盖自动化测试、在 Nightly 验证。
- 可追溯的修复入口：[D314267](https://phabricator.services.mozilla.com/D314267)、[autoland revision](https://hg.mozilla.org/integration/autoland/rev/bd86aeb7ccd9)。本轮只核查 Bugzilla 中的发布记录，**未审查补丁正文，未下载录像，未本地复现**。
- 对 Layress 的启示：准备步骤必须区分首次和再次打开；尺寸压力与状态或时序引起的裁切应分别归因。不能只保存一个窗口宽度就声称可回放。
- 适用范围限制：这是 macOS 上 Firefox 自身设置界面，不是已确认可由首版 Windows / Chromium Web 或 Electron 通道接入的页面。只作为真实历史缺陷线索，**不纳入首版评估集，不据此扩大平台承诺**；许可与样本再分发也尚未核查。

## 五、官方资料：避免重复实现已有能力

以下均于 2026-11-04 读取正文；页面未提供可直接采用的统一发布日期，故不填发布日期。

| 编号与来源 | 已核实内容 | 对后续验证的约束 |
|---|---|---|
| O1 · [Playwright Auto-waiting](https://playwright.dev/docs/actionability) | `locator.click()` 已检查 Visible、Stable、Receives Events、Enabled；Receives Events 检查动作位置的指针命中目标，`force` 会跳过部分检查；Visible 的定义还包括 `opacity:0` 的元素 | 遮挡命中并非空白能力。应与已有动作检查公平比较，不能把“能点击”当成业务成功，也不能把框架可见性当成人眼可见 |
| O2 · [Playwright Trace viewer](https://playwright.dev/docs/trace-viewer) | 已有动作前后 DOM 快照、截图序列、日志、网络信息等诊断能力 | 需要证明压力条件搜索、确认或缩减的额外收益；避免独立重做一套相同轨迹展示 |
| O3 · [Electron Automated Testing](https://electronjs.org/docs/latest/tutorial/automated-testing) | 官方列出 WebdriverIO、Selenium、Playwright 和自定义驱动；Playwright 的 Electron 支持仍标记为 experimental | Electron 不是没有自动化基础。复用与适配成本应实测，不能据此宣称已经支持目标应用 |
| O4 · [WCAG 2.2 Understanding Reflow](https://www.w3.org/WAI/WCAG22/Understanding/reflow.html) | 说明重排应避免信息或功能丢失，解释 320 CSS px 与起始 1280px、400% 缩放的关系，并列出需要二维布局的例外 | 提供任务边界和合理负例参考，不是市场需求统计。滚动本身不能一律判坏；尺寸、缩放、内容类型和例外需要区分 |

## 六、尚未取得的证据与下一步

| 缺口 | 本轮状态 | 后续动作 |
|---|---|---|
| GitHub 直接竞品及候选项目 | 官方便携 gh 已准备并校验，用户已允许只读运行；CLI 未登录，公开 API 查询仍要求认证 | 完成本机授权后，优先核查原调研的 breakhound、ui-review、LocalePass，再查目标项目的 Issue、修复 PR、版本与活跃度；不把旧 README 或 stars 当成采用证据 |
| V2EX、HN、Reddit 原文 | 访问失败或未取得正文；搜索摘要未纳入已核实证据 | 后续访问恢复再核查，保持“待核查”状态，不补造作者、日期和结论 |
| 首版可接入的真实外部样本 | B1 有缺陷与修复链，但不满足首版平台；尚无可用的外部评估集 | 找到 Windows / Chromium Web 或 Electron 的真实缺陷，同时收集滚动、折叠菜单等合理负例；核对修复前后版本、准备步骤及许可 |
| 竞品能否解决同一案例 | 未运行，未知 | 先选同一小组案例、同一目标操作，记录配置、运行与人工复核成本；未跑通就如实记原因 |
| 首批用户、重复使用与收益 | 无访谈或试用证据 | 准备围绕近期真实事故、现有流程、发生频率、处理耗时与接入负担的问题；对外联系另行授权 |

当前建议仍是继续阶段 1。下一轮优先补“首版可接入案例 + 竞品实际表现”，再决定首验提案如何收缩或改写；本轮材料不足以批准原 6 周 / 90 小时方案。
