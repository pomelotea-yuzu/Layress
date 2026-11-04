# 第二轮 GitHub 证据核查：修复链、维护者取舍与候选样本

> 核查日期：2026-11-04。状态：**桌面证据核查，尚未实跑**；不构成首验方案或开发范围批准。
> 接续[首轮社区证据](2026-11-04-community-evidence-round-one.md)，对应[简短路线图](../product/roadmap.md)阶段 1。
> 后续：优先候选的原始输入、版本取得方式、准备成本与下次停点见[准备核查记录](2026-11-04-vscode-candidate-readiness.md)。

## 一、当前结论

- GitHub 只读访问已恢复，本轮通过官方 CLI 查询仓库、Issue、评论、timeline、PR 合并状态与固定提交源码；未修改远程内容。
- 三个直接竞品的仓库、版本和关键实现已核查：宽度定位、有限状态探索、Agent 压力配方及本地化检查均已有路径，但本轮未找到独立效果或持续采用证据，不据此判定无人使用。
- **关闭不等于修复**：既要核查 Issue 的关闭原因，也要看 PR 是否真正合并，以及是否有维护者接受的修复或复测证据。
- **布局异常不自动等于值得报警的缺陷**：维护者可能认为极端尺寸不值得承担修复风险。需记录支持范围、关键操作与可接受替代入口，不能只统计红框。
- 核查到 1 条 Joplin Web 与 3 条 VS Code 的修复候选链，其中 Windows 导入配置后安装确认按钮越界最贴近首验；另有平台未明和键盘交互案例，适用范围分别标注。它们均未本地回放，不算已入评估集。

## 二、核查方法与边界

GitHub 查询使用已获用户授权的官方便携 gh；不读取或记录令牌，不发帖、提 Issue 或 PR。先检索原文，再核对 timeline 的关联关系、PR 的 `merged` 与 `merged_at`，必要时读固定提交的相关源码。搜索只返回所列查询的前若干项，不能据此宣称覆盖所有相关缺陷。

Joplin 查询包括 `repo:laurent22/joplin is:issue button window small`、`toolbar overflow`、`in:title dialog`、`in:title clipped`，以及 `repo:laurent22/joplin is:pr 11799`。`clipped` 还会命中网页剪藏语义，无关结果不计为布局证据。

## 三、直接竞品：能力存在，不等于采用或效果已证实

元数据为本轮 API 快照，日期按 UTC。三仓均为 `main`、未归档，LICENSE 正文均为 MIT；复用须保留许可与版权声明。Issue 与 PR 分开计数，stars / forks 仅作检索背景，不代表效果、采用率或付费需求。

| 仓库与固定源码提交 | 创建 / 最近 push | stars / forks | Issue 总数 / PR 总数 | 版本核查 |
|---|---|---|---|---|
| [Seybel/breakhound](https://github.com/Seybel/breakhound/commit/2e6b43f9b639404084f8564eac559c542888bf53)，`2e6b43f9b639404084f8564eac559c542888bf53` | 2026-10-03 / 2026-10-22 | 6 / 0 | 0 / 1 | [GitHub Release v1.0.0](https://github.com/Seybel/breakhound/releases/tag/v1.0.0)（10-14）与当前源码包 1.0.10 不同，不能混称 |
| [cdmx-in/ui-review](https://github.com/cdmx-in/ui-review/commit/605631484b21989354359279007f9e1c4633dc37)，`605631484b21989354359279007f9e1c4633dc37` | 2026-08-02 / 2026-08-04 | 3 / 0 | 0 / 0 | [v1.2.0](https://github.com/cdmx-in/ui-review/releases/tag/v1.2.0)（08-04）对应本次源码提交 |
| [CodingRasi/LocalePass](https://github.com/CodingRasi/LocalePass/commit/fd4b1d9f7a9164fd96ad716f6a94e2237fdd31c6)，`fd4b1d9f7a9164fd96ad716f6a94e2237fdd31c6` | 2026-04-05 / 2026-04-05 | 0 / 0 | 0 / 0 | [Release 1.0.2](https://github.com/CodingRasi/LocalePass/releases/tag/1.0.2) 与 README 为 1.0.2，源码包及 CLI 常量仍为 1.0.1；实跑须记录 SHA，不能只记版本字符串 |

### K1 · breakhound：最直接对照，但不保证穷举发现

- 声明：[固定版 README](https://github.com/Seybel/breakhound/blob/2e6b43f9b639404084f8564eac559c542888bf53/README.md)包含路由发现、320–1920px 扫描、宽度区间、菜单等状态探索，以及 CLI、扩展和 MCP。Node ≥20.16，首次安装需下载 Chromium；认证页面需准备会话。
- 源码核查：[sweep.ts](https://github.com/Seybel/breakhound/blob/2e6b43f9b639404084f8564eac559c542888bf53/packages/engine/src/sweep.ts)已有粗采样、状态变化区间二分、窄区间复测和不稳定标记，但端点状态相同时会停止该区间细化，不能推出逐像素穷举保证。[states.ts](https://github.com/Seybel/breakhound/blob/2e6b43f9b639404084f8564eac559c542888bf53/packages/engine/src/states.ts)已有点击展开、重载隔离及状态增量检查，按组和数量限制探索，文档默认最多 5 个状态；不等于任意业务路径或业务成功确认。
- 模型与成本：规则扫描不需要模型；可选视觉意见与确定性 verify 分开，不能把模型意见当成同等的检测结论。
- 社区证据：唯一 [PR #1](https://github.com/Seybel/breakhound/pull/1)是未合并的图片优化机器人提交；本轮没有公开用户 Issue 可用于估计误报或真实采用。没有 Issue 不表示没有问题或没有用户。
- 对 Layress：宽度区间、状态扫描和截图组织均已有实现，适合先实跑或复用；不能单独宣传为独有增益。

### K2 · ui-review：Agent 配方与检测脚本须分别评价

- 声明：[固定版 README](https://github.com/cdmx-in/ui-review/blob/605631484b21989354359279007f9e1c4633dc37/README.md)介绍四断点、视觉复核、基线差分和 thorough 压力测试。执行依赖 Agent 宿主、`agent-browser` 及浏览器安装；安装不需账号不等于模型使用免费。
- 源码核查：[detect.js](https://github.com/cdmx-in/ui-review/blob/605631484b21989354359279007f9e1c4633dc37/skills/ui-review/scripts/detect.js)已有几何、文本、图片与浮层检查，遍历开放 Shadow DOM；每类数量与重叠扫描范围有限，不能据此宣称覆盖 iframe 或闭合 Shadow DOM。[SKILL.md](https://github.com/cdmx-in/ui-review/blob/605631484b21989354359279007f9e1c4633dc37/skills/ui-review/SKILL.md)及配方要求 Agent 执行字体等待、冻结动画、动态区域遮罩、长文本 / RTL 注入等，不等于独立确定性执行器。
- 社区证据：本仓无 Issue 或 PR；外部检索找到[资源收录 PR #391](https://github.com/composio-community/awesome-claude-plugins/pull/391)与[收录申请 #2399](https://github.com/hesreallyhim/awesome-claude-code/issues/2399)，本轮均开放、无评论。只能记为收录 / 推广迹象，不能算独立使用效果；仓内样例与自测也不是外部采用。
- 对 Layress：可以借鉴压力场景库；如果首验坚持零远程模型预算，只比较本地检测脚本时须明确这是子能力对照，不能将其当成完整 Agent 工作流的全部效果。

### K3 · LocalePass：本地化检查与截图比较，不是主动操作确认

- 声明：[固定版 README](https://github.com/CodingRasi/LocalePass/blob/fd4b1d9f7a9164fd96ad716f6a94e2237fdd31c6/README.md)介绍未翻译、裁切、lang / RTL 检查与截图回归，输出多种报告。依赖 Node ≥20、Chromium、源码构建；图像比较另调用 ImageMagick。
- 源码核查：[CLI 源码](https://github.com/CodingRasi/LocalePass/blob/fd4b1d9f7a9164fd96ad716f6a94e2237fdd31c6/packages/cli/src/index.ts)确有页面 × 语言 × 视口循环、会话注入与报告输出；未翻译、RTL 等为启发式判断，检查已准备的语言页面，不等于生成真实翻译压力或确认关键操作。
- **需要先验证的源码风险**：[compareImages，L1055](https://github.com/CodingRasi/LocalePass/blob/fd4b1d9f7a9164fd96ad716f6a94e2237fdd31c6/packages/cli/src/index.ts#L1055)捕获比较命令异常后，从输出提取首个数字，无匹配则返回零差异；上游随后可能归为 `matched`。这说明环境错误存在被混为通过的风险；本轮未执行故障注入，**不写成已复现缺陷或误报率**。比较前应验证正常、已知不同、工具不可用等情况确实分开。
- 社区证据：维护活动集中在创建当天，本轮未找到用户 Issue 或外部贡献，不能由此判定无人使用或工具无效。
- 对 Layress：可参考输入配置和报告格式，但应先核查其环境健康检查和截图结果，不能直接把竞品 PASS 当作样本真值。

### 共同证据边界

本轮完整读取相关 README、LICENSE 和上述关键实现，属于文档与局部源码核查，不是完整安全或正确性审计。公开外部检索覆盖竞品名称或仓库名在 Issue / PR 标题正文中的出现，不覆盖所有评论、私有项目、站外社区或未索引内容。未运行这三款工具，因此没有精确率、性能或真实费用结论。

三者都较新、公开采用信号较少，足以作为技术对照，尚不足以代表成熟市场需求。后续还应了解开发者实际使用的 Playwright、人工检查、视觉回归等替代工作流，而不只比较相近的小型开源项目。

## 四、Joplin：一个可追溯修复候选与三类反例

### J1 · Web 删除确认框显示在父弹窗下面

- 原文：[Issue #11799](https://github.com/laurent22/joplin/issues/11799)，2025-02-07 创建。报告说明点击 Delete 后，确认框出现在已有弹窗下面，且当时只在 Web app 测试；评论进一步限定为 Web，不应外推到 iOS / Android 原生确认框。
- 修复链：早期 [PR #13715](https://github.com/laurent22/joplin/pull/13715) 已关闭但**未合并**；最终 [PR #14337](https://github.com/laurent22/joplin/pull/14337) 于 2026-02-18 合并，随后 Issue 关闭。
- 修复提交：[e6604e369f4a922c9f441b7f3c086e6cb645de1d](https://github.com/laurent22/joplin/commit/e6604e369f4a922c9f441b7f3c086e6cb645de1d)；直接父提交为 `60f76afa54db5c184f3167f40c695aa2132014a6`。这是一对待回放的修复前后候选版本；PR 的 base SHA 不等于合并提交的直接父 SHA，不混用。
- 源码抽查：在 PR head `c2f00832d901f41ed474f26dd2efb2f23ba119fb` 完整读取 [Modal.tsx](https://github.com/laurent22/joplin/blob/c2f00832d901f41ed474f26dd2efb2f23ba119fb/packages/app-mobile/components/Modal.tsx) 与 [Dialog.tsx](https://github.com/laurent22/joplin/blob/c2f00832d901f41ed474f26dd2efb2f23ba119fb/packages/lib/components/Dialog.tsx)，确认 Web 分支改用共享 Dialog，通过原生 `<dialog>.showModal()` 管理模态层。此为局部源码核查，非完整审计。
- 测试证据边界：PR 提供 Chrome、Firefox、Safari 等演示说明；本轮未下载或复核录像。查询到 36 个变更文件，没有路径含 `test` 或 `spec` 的文件，**不据此断言项目没有测试**，也不把演示说明写成本项目实测。
- 建议准备步骤：隔离测试环境中安装可丢弃的测试插件，进入插件信息弹窗，触发删除确认；核查确认框的显示层级和真实指针可达性。确认与取消应分开；检测不需要执行最终卸载，不能触碰真实插件或用户数据。
- 已有条件：Web app、嵌套弹窗状态、修复链与代码版本。缺失条件：旧版在 Windows / Chromium 的构建结果、精确浏览器版本、内容区宽高、语言、字体与缩放、目标定位、真实输入判据与重复次数。其原始问题主要是界面状态而非已证实的尺寸压力，不能据此证明压力搜索有效。
- 负例设计：正常模态下，父弹窗暂时不可点击应是预期；最上层确认框可达、取消后恢复父层才是对照。必须区分“模态机制按设计阻止背景操作”和“必要确认框被挡住”。

### J2 · 极窄富文本工具栏：维护者拒绝修复

[Issue #15341](https://github.com/laurent22/joplin/issues/15341)（2026-05-11）报告 Linux、Joplin 3.5.13 下缩窄笔记区域导致工具栏重叠、越界，Issue 标为 `closed / completed`。关联 [PR #15338](https://github.com/laurent22/joplin/pull/15338) 实际 `merged=false`；[维护者评论](https://github.com/laurent22/joplin/pull/15338#issuecomment-4422722695)表示对极窄界面不太关心，担心该改动引入隐蔽问题而不合并。

这不是已修复样本，也不能直接当作“无缺陷”负例：它证明的是维护者对支持尺寸和风险的取舍，不是已经证明操作完全可达。应先取得应用支持范围与关键操作要求，再决定告警级别；不能把报警数量当价值。

### J3 · Windows 网站响应式报告：被 stale 关闭

[Issue #15280](https://github.com/laurent22/joplin/issues/15280)（2026-05-06）报告网站在约 767px 附近移动菜单内容出现在导航区域，还描述关闭按钮易误点相邻链接；这是 Joplin **网站**，不是桌面应用。窗口高度、精确环境与输入证据不完整。

本轮状态为 `closed / not_planned`，带 `stale` 标签，未核实到合并修复。可保留为 Web 压力场景线索，不能以关闭状态补造正确版本；按钮换行本身也不能替代操作受阻证明。

### J4 · 环境字段与正文矛盾、文案裁切与操作失效混淆

- [Issue #9723](https://github.com/laurent22/joplin/issues/9723)报告更新弹窗过高、无法访问关闭按钮；表单写 Windows，但正文引用的是 macOS 经历。当前仍为 open。原始表单字段不能当成 Windows 已复现证据，须回到来源及实际环境核查。
- [Issue #11792](https://github.com/laurent22/joplin/issues/11792)报告插件信息弹窗 Update 后的法语状态文字被截断，当前 open；没有证据证明操作不可达。适合作为信息可读性候选，不能直接计入关键操作阻断。
- [Issue #12198](https://github.com/laurent22/joplin/issues/12198)报告 Windows、3.3.7 切换编辑器时工具栏可见项与 overflow 菜单分配变化，当前 open；本轮未复核附图。控件进入菜单不自动意味着功能丢失，最终必须检查替代入口。
- [Issue #15065](https://github.com/laurent22/joplin/issues/15065)报告 Windows 调整弹窗后侧栏未随之调整，关闭原因为 `not_planned`，带 `stale` 标签；同样不能当已修复样本。

## 五、VS Code：三个候选的适用范围与回放缺口

本轮定向检索 `small window`、`cut off dialog`、`narrow button`、`height dialog`、`zoom inaccessible`，结合 bug / verified 标签；核查原文、评论或 timeline、合并与代码差异。没有将仅关闭、out-of-scope 或缺修复证据的线程当作已修复样本。

### V1 · Windows 导入配置：安装确认按钮越出低高度窗口（优先核查）

- [Issue #296538](https://github.com/microsoft/vscode/issues/296538)，2026-02-20：Windows x64 `10.0.26100`、VS Code 1.109.5（`072586267e68ece9a47aa43f8c108e0dcbf44622`），新安装且无扩展。导入含大量扩展的 profile，创建后出现发布者信任对话框，内容过高又无法滚动，`Trust Publisher & Install` 按钮不可见。
- 操作链：Manage → Profiles → Import Profile → 选择配置文件 → Create → 信任确认框。原帖附图标注 1366×768，**不能当作已经核实的 CSS 内容区尺寸**。
- [PR #299048](https://github.com/microsoft/vscode/pull/299048) 于 2026-03-03 合并，修复提交为 [d481546f4c48b6592932b8e5808a4eed42d8823b](https://github.com/microsoft/vscode/commit/d481546f4c48b6592932b8e5808a4eed42d8823b)，直接父提交为 `37645e695f1d0e93cf1bb17a550a9e1fdf9d4867`。
- 已读取唯一变更文件 `src/vs/base/browser/ui/dialog/dialog.css` 的完整 patch：添加 `max-height:90vh`、允许 flex 内容收缩及纵向滚动，使消息区滚动而操作区保留。注意 commit message 错引为 #296528，PR 正文与 Issue 关联才指向 #296538；不能只靠提交信息匹配。
- 适用性：明确的 Windows / Electron 关键操作受阻，且有尺寸与长内容组合，优先进入**待复现候选池**。负例是内容可滚动、确认与取消可达，不能把滚动条判成问题。
- 缺口：原 profile 文件、扩展及发布者数量、在线扩展元数据快照、真实内容区宽高、DPI / 缩放、字体与语言、可稳定取得的旧版环境。尚未证明在指定环境下可重复。
- 安全准备：未来验证先停在信任确认状态，核查指针可达性即可，不需要执行安装。若必须准备扩展数据，应使用隔离 profile 和受控数据；本轮没有安装扩展，也不将安装业务成功作为布局判据。

### V2 · 高缩放或窄侧栏：调试启动控件被标题挤占（条件候选）

[Issue #324893](https://github.com/microsoft/vscode/issues/324893)，2026-07-08，条件为开启 Modern UI，在高缩放或窄 Run and Debug 侧栏中，标题挤占 Run / 播放按钮和启动配置下拉框。原文没有给出平台、版本和缩放数值，不能认定 Windows 已复现。

[PR #324905](https://github.com/microsoft/vscode/pull/324905)当日合并，修复 [a2b5f48611d5330d7e5c6613d889c9d66f174588](https://github.com/microsoft/vscode/commit/a2b5f48611d5330d7e5c6613d889c9d66f174588)调整操作区 flex、限制标题并保障启动控件最小宽度，Issue 带 verified。标题省略但启动与配置选择仍可达，应保留为正常对照。

仍需补平台、构建、Modern UI 设置、侧栏与窗口实际宽度、缩放、语言和调试配置。未来若验证启动按钮，不应执行真实业务调试任务；布局可达性和调试成功应分开。

### V3 · Windows 窄窗口与隐藏菜单：键盘焦点落回文件（交互候选）

[Issue #233257](https://github.com/microsoft/vscode/issues/233257)，2024-11-06：Windows x64 `10.0.26100`、1.94.2，报告者升级至 1.95.1 后仍出现。设置 `menubarVisibility=toggle`，窗口约半屏，编辑器保持焦点，再依次按 Alt、V、W，预期为 View → Word Wrap；全部菜单折叠到汉堡按钮时，按键可能落入文件，写入 `vw`。

[PR #233960](https://github.com/microsoft/vscode/pull/233960)于 2024-11-15 合并，修复 [1770cc47a3a5e3bd708779d902de62f36b466cbf](https://github.com/microsoft/vscode/commit/1770cc47a3a5e3bd708779d902de62f36b466cbf)在顶层菜单全部折叠时将焦点转至 overflow 按钮。

汉堡菜单本身不是缺陷，需要检查焦点与命令结果；缺精确宽度、DPI、界面语言、键盘布局和按键时序。此例证明“看上去可见”不足以覆盖全部交互，但**暂不作为首验指针可达性能力的必做目标**，避免据此扩大首版范围。未来只能在隔离的空白文档中验证，不更改用户文件。

### 接入与许可

[实际修复版本 LICENSE](https://github.com/microsoft/vscode/blob/d481546f4c48b6592932b8e5808a4eed42d8823b/LICENSE.txt)已核查为 MIT；[README](https://github.com/microsoft/vscode/blob/main/README.md)区分源码仓库与微软发行版，不能把品牌发行包一概当成 MIT。宜核查对应修复与父提交的 Code - OSS 构建；也可另查旧发行包可用性，但本轮未验证下载及完整工具链。旧版依赖、扩展市场访问、Electron 数据与缓存重定向都需单独确认，不能视为随取随跑。

## 六、Joplin 接入与许可前提

- 构建入口：在 PR base `45b44b962e866349dbec574830df2bfb14eb93ea` 读取[构建说明](https://github.com/laurent22/joplin/blob/45b44b962e866349dbec574830df2bfb14eb93ea/readme/dev/BUILD.md)与 `packages/app-mobile/package.json`；该包标为 3.6.0、Node ≥20、Yarn workspace，Web 开发入口为 `packages/app-mobile` 下的 `yarn serve-web`，文档默认端口 8088。此为接入线索，未验证直接父提交的全部依赖可安装或启动。
- [对应版本 LICENSE](https://github.com/laurent22/joplin/blob/45b44b962e866349dbec574830df2bfb14eb93ea/LICENSE)规定默认 `AGPL-3.0-or-later`，子目录自带 LICENSE 时另行适用；图标与标志有额外限制。GitHub 自动识别返回 `NOASSERTION`，不能把它解释为无许可。
- 本轮只登记链接、摘要和提交，不复制应用源码、截图、图标或录像进 Layress。实际复用、派生样本或再分发须核查对应子目录及素材权利，不能因 Layress 为 MIT 就直接混入。
- 后续运行必须使用隔离的合成数据，并覆盖依赖缓存、临时目录和浏览器数据目录，满足便携与路径要求；不为了取证启动真实账户同步或改变现有用户环境。

## 七、尚未执行与下一步

本轮未运行竞品或目标应用，未构建历史版本、人工复核截图或回放缺陷，也未联系维护者。候选记录不算冻结评估集。

1. **先核查准备成本**：V1 有 Windows、低高度与长内容的操作阻断证据，优先补原 profile、旧版本及环境条件；J1 可作为嵌套模态候选，但须评估 Joplin Web 的构建、插件准备和许可成本。V2 缺平台，V3 超出首验指针目标，均不直接承诺交付。
2. **再确定最小对照**：区分未滚动时不可见、允许滚动后可达、折叠到替代入口、真实受阻；约定好坏样本真值、运行环境与成本记录。竞品先做已知正常、已知失败及环境不可用的健康检查，再比较同一案例。
3. **补使用者证据**：这些小型竞品的公开社区信号很薄，不能用相互对比证明需求。继续收集实际 Playwright / 人工检查流程、近期事故、复核耗时、接入负担及愿意试用的对象；访谈或对外联系另行授权。
4. **依据证据修订首验提案**：固定版本、样本授权、压力条件与状态分别记录；先解决可复现性和比较口径，再决定预算与 change。是否为独立工具、插件或上游贡献保持开放。

当前仍处路线图阶段 1。GitHub 访问障碍已解除，但研究尚未收口，尚未批准原 6 周 / 90 小时方案，也未启动产品实现。
