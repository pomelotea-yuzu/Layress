# VS Code 优先候选：准备条件、成本与下次接续

> 核查日期：2026-11-04。接续[第二轮 GitHub 证据](2026-11-04-github-evidence-round-two.md)的 V1（[Issue #296538](https://github.com/microsoft/vscode/issues/296538)）。
> 当前停点（2026-11-05 续查）：**原 profile 仍缺失；已找到同一修复对应的更简单公开触发线索，候选尚未复现**。本文前六节保留 2026-11-04 核查记录，最新结论与接续见第七节；未下载应用 ZIP、安装依赖、启动旧版或安装扩展，未进入产品实现，也未批准首验提案。

## 一、这一步得到的结论

优先保留 V1，但尚不能称为可回放样本。历史 Windows ZIP 已确认可取得，修复前后源码差异也已核查；主要缺口是原始 profile 和可冻结的发布者元数据，而不是缺少一个扫描脚本。

下次先补触发输入，再考虑隔离试跑。若只能重建数据，须标为“依据真实修复派生的案例”，不能冒充原报告者的原始复现。整体需求调研仍未收口。

## 二、原始输入：已知与仍缺什么

| 项目 | 本轮核查结果 | 后续要求 |
|---|---|---|
| 原 profile | Issue 正文只有步骤、环境和附图，没有 profile 或扩展清单；本次评论 API 返回空列表 | 不猜扩展名，不把任意“很多扩展”当成原样本；需取得来源或明确采用派生数据 |
| 原始环境 | Windows x64 10.0.26100、VS Code 1.109.5，报告称新安装且无扩展；附图 1366×768 | 图像尺寸不能当 CSS 内容区；仍需实测宽高、DPI、缩放、字体、语言和窗口状态 |
| 触发输入 | 缺陷前父提交源码按未信任扩展、不同发布者、验证状态与依赖构造提示内容 | 冻结扩展 ID / 版本、发布者 ID / 显示名 / 验证状态、依赖或扩展包关系及返回的 manifest；扩展数量本身不是充分描述 |
| 信任状态 | 信任记录存于应用级存储，默认可信发布者或允许列表可能使提示跳过 | 必须隔离用户数据根，不能只新建一个共享应用存储的 profile；记录初始信任状态 |
| 提示前网络 | 导入先查询 gallery 和 manifest，再请求发布者信任 | 不把实时市场内容当稳定输入；需核查可保存的元数据、使用条件与受控替代方式 |

源码依据均固定在修复直接父提交 `37645e695f1d0e93cf1bb17a550a9e1fdf9d4867`：

- [extensionsResource.ts](https://github.com/microsoft/vscode/blob/37645e695f1d0e93cf1bb17a550a9e1fdf9d4867/src/vs/workbench/services/userDataProfile/browser/extensionsResource.ts)：导入解析扩展条目、查 gallery；含 token 的 apply 分支先请求发布者信任，再逐个安装。其他初始化路径可能跳过信任，不能任意换入口。
- [extensionManagementService.ts](https://github.com/microsoft/vscode/blob/37645e695f1d0e93cf1bb17a550a9e1fdf9d4867/src/vs/workbench/services/extensionManagement/common/extensionManagementService.ts)：提示会按不同发布者及验证状态生成不同长度内容；确认按钮会记录信任，取消会中断。此为父提交源码核查，不代表已实测原发行版的全部路径。

本次再次读取 Issue timeline 未成功；第二轮已核查的 Issue → PR 关系继续沿用，未从失败请求补造额外附件或复测结论。

## 三、版本与取得方式

| 路线 | 固定标识 | 本轮已核实 | 还不能宣称什么 |
|---|---|---|---|
| 原报告发行版 | 1.109.5，`072586267e68ece9a47aa43f8c108e0dcbf44622` | tag 对应提交与报告一致；目标 CSS 没有本次 `max-height:90vh` 与纵向滚动修复；官方 Windows x64 ZIP 最终 HEAD 200，179331203 字节，约 171 MiB | 尚未下载、校验或运行，不能称为复现成功 |
| 含修复的发行版候选 | 1.111.0，`ce099c1ed25d9eb3076c11e4a280f3eb52b4fbeb` | tag 的目标 CSS 已含两项修复；官方 ZIP 最终 HEAD 200，187207664 字节，约 178.5 MiB | 不声称这是最早修复发行版，也不把跨版本所有变化归因于这一补丁 |
| 精确源码对照 | 父提交 `37645e695f1d0e93cf1bb17a550a9e1fdf9d4867` → 修复 `d481546f4c48b6592932b8e5808a4eed42d8823b` | 已核对合并与完整 CSS patch；本轮另核查两端开发配置 | 尚未构建，不保证可直接走原商业发行版的导入流程 |

下载入口：[1.109.5 Windows x64 ZIP](https://update.code.visualstudio.com/1.109.5/win32-x64-archive/stable)、[1.111.0 Windows x64 ZIP](https://update.code.visualstudio.com/1.111.0/win32-x64-archive/stable)。入口返回 302，最终微软下载地址的路径分别含上述提交，最终响应 200；本轮只请求 HEAD，没有读取安装包正文。响应头只证明当时可取得及标注的长度，不代替下载后哈希、签名和运行版本核查。

两个 ZIP 压缩体积合计约 350 MiB；解压后、运行中和缓存体积未测。本轮未从 `1.110.0` 取得目标 CSS，不凭发布日期推断该版本包含修复。

## 四、准备成本：先区分目标再选路线

| 路线 | 主要准备项 | 价值与限制 | 当前排序 |
|---|---|---|---|
| 两个官方便携 ZIP | 下载校验、独立 data、触发 profile 与发布者数据、记录环境 | 较少构建依赖，适合先看原流程能否重现；发行版间有大量其他变化，只能先建立现象对照 | 触发输入补齐后优先评估 |
| 精确父 / 修复提交的 Code - OSS | npm 与开发 Node、Windows C/C++ 工具链、Electron 和原生模块构建、市场服务条件 | 更适合补丁因果对照，但源码产品默认没有内置 gallery 配置，不能假定与商业发行版同等可触发 | 不作为首次准备的默认路线 |
| 使用真实组件的局部派生页面 | 对应许可、组件及样式依赖、受控提示内容和环境 | 可减少网络与整应用负担，适合定位布局机制；不能替代原应用、真实 profile 的外部验证 | 仅在原流程准备成本过高时另议 |
| Joplin Web 备选 J1 | 见第二轮记录：历史 Yarn workspace、插件状态、浏览器与 AGPL / 素材权利核查 | 已有模态遮挡修复链，但主要是状态问题，不能替代 V1 的尺寸 × 长内容问题 | 保留备选，不同时铺开构建 |

以上为依赖和证据价值的定性排序，**没有测得安装耗时、构建耗时或人工工时**，不据此承诺几小时完成。

源码构建依据：两端 [.nvmrc](https://github.com/microsoft/vscode/blob/37645e695f1d0e93cf1bb17a550a9e1fdf9d4867/.nvmrc)均为 22.22.0，[.npmrc](https://github.com/microsoft/vscode/blob/37645e695f1d0e93cf1bb17a550a9e1fdf9d4867/.npmrc)指定 Electron 39.6.0 并要求源码构建原生依赖；[preinstall.ts](https://github.com/microsoft/vscode/blob/37645e695f1d0e93cf1bb17a550a9e1fdf9d4867/build/npm/preinstall.ts)允许不低于声明的 Node，拒绝 Yarn，在 Windows 检查 Visual Studio 2022 / 2019，并下载构建头文件。未检查本机是否具备完整工具链，也未绕过这些检查。

[product.json](https://github.com/microsoft/vscode/blob/37645e695f1d0e93cf1bb17a550a9e1fdf9d4867/product.json)为 Code - OSS、无 `extensionsGallery`；[scripts/code.bat](https://github.com/microsoft/vscode/blob/37645e695f1d0e93cf1bb17a550a9e1fdf9d4867/scripts/code.bat)启动前会运行准备步骤，[electron.ts](https://github.com/microsoft/vscode/blob/37645e695f1d0e93cf1bb17a550a9e1fdf9d4867/build/lib/electron.ts)将 Electron 写入仓库 `.build/electron`。不能只看到这个本地落点就认定下载缓存、头文件或其他临时文件也不会写 C 盘。

## 五、隔离与判据准备

- 按[官方便携模式说明](https://code.visualstudio.com/docs/setup/portable)，Windows 仅 ZIP 支持此模式；每版应有独立 `data/user-data`、`data/extensions` 和 `data/tmp`。`data` 存在时优先于 `--user-data-dir` / `--extensions-dir`，不要组合参数后误以为彼此独立。
- 便携模式默认仍用系统临时目录，需显式准备 `data/tmp`；同时核查启动进程、浏览器和依赖工具的临时 / 缓存落点。本轮只核查文档，**没有实测零 C 盘写入**。
- 不导入日常用户数据，不登录同步；停在安装确认框，通过布局测量、允许滚动后的指针命中或无副作用的 trial 判断，不按安装确认，不执行不明扩展。
- 失败对照须实际证明关键按钮在约定滚动方式下仍不可达；正常对照允许内容滚动，确认和取消可达。布局通过不等于安装业务成功。
- 实跑记录必须含版本与来源校验、完整准备步骤、触发输入来源、内容区尺寸、语言、字体 / 缩放、DPI、滚动前后状态及重复计数；人工视觉与交互复核仍单列。

## 六、下次接手先做什么

1. 先读本文与第二轮记录，保持需求调研阶段；原首验提案仍待修订与批准，没有活动 change。
2. 补 V1 的触发输入：寻找可核查的原 profile；若公开材料仍没有，则明确选择并说明派生输入，记录不同发布者与信任状态，不冒充原件。需要联系报告者时另行取得授权。
3. 输入准备路径确定后，再决定是否下载约 350 MiB 的两个 ZIP，完成来源与隔离核查后做最小现象复测；若需技术实验或实现，按流程确定范围和 change，不自动批准整份 6 周方案。
4. 若能重现，再让朴素规则与直接竞品面对同一组好坏状态，记录接入、运行、复核成本；若仍无法建立输入与真值，继续标为候选，不进入冻结评估集。

2026-11-04 停在准备核查完成处。竞品实跑、历史版本构建 / 运行、原始缺陷回放、人工验收、用户访谈与产品实现均未执行；当轮没有创建 change、下载应用包或对外联系。

## 七、2026-11-05 续查：输入缺口与更小的验证入口

### 结论与取舍

V1 继续保留为真实工作流候选，但公开材料仍不足以还原原 profile。新找到的 [Issue #114662](https://github.com/microsoft/vscode/issues/114662) 给出了更小的长文本弹窗触发方式，维护者后续明确将其与 V1 的同一修复关联；可以先评估它是否适合验证布局机制，再决定是否承担市场元数据准备成本。这是调研方向的成本排序，**不是决定开发该功能，也不是技术实验范围批准**。

### 本轮补齐的公开事实

| 来源 | 新核查结果 | 证据边界 |
|---|---|---|
| [V1 原帖](https://github.com/microsoft/vscode/issues/296538)、[评论 API](https://api.github.com/repos/microsoft/vscode/issues/296538/comments) | 正文仍只有步骤、环境与截图，评论数为 0，API 返回空数组；未找到 profile 附件 | 只说明所查原帖与关联材料没有提供原件，不宣称全网不存在 |
| [原帖截图](https://github.com/user-attachments/assets/b8cb4483-d7e0-4284-bd44-78b1afe3d918) | 已下载并查看：英文信任提示含多条已验证发布者域名与一段未验证发布者列表，弹窗上下内容超出图像边界 | 发布者显示名无法唯一反推出扩展 ID / 版本；截图不能证明实际滚动、输入命中或精确 CSS 尺寸，本轮也未作人工交互验收 |
| [V1 timeline](https://api.github.com/repos/microsoft/vscode/issues/296538/timeline) | 本轮读取成功，包含指向 PR #299048 的关联事件及随后关闭事件 | 未发现原报告者复测记录；关闭仍不等于本项目复现 |
| [PR #299048](https://github.com/microsoft/vscode/pull/299048)、[变更文件](https://api.github.com/repos/microsoft/vscode/pulls/299048/files) | 已合并，只改通用 `dialog.css`；约束高度并调整 flex / 纵向滚动，不是只针对 profile 的特殊补丁 | 修复机制已知不代表每种布局均已验证 |
| [PR 评审](https://github.com/microsoft/vscode/pull/299048#pullrequestreview-3884784201) | Copilot 自动评审提出盒模型和 footer 收缩 / 滚动疑问，另有[人工批准](https://github.com/microsoft/vscode/pull/299048#pullrequestreview-3885100042) | 自动意见不是已确认的新缺陷，人工批准也不是本项目好样本真值；极端高度及 footer 仍需按实际可达性判定 |

### profile 能固定什么，不能固定什么

本轮检查了固定父提交的导入、信任提示与 gallery 查询源码。原发行版 tag `1.109.5` 与父提交的 `extensionsResource.ts` blob 均为 `c06b870aea9b5e7db639a2e97ec5634d55bf6301`，`extensionManagementService.ts` blob 均为 `ea7f36418535b2879ef8d512143bc1600a355f87`；因此这两个文件的逻辑可用于分析原报告版本，但不据此推断所有依赖、服务配置和运行行为相同。

- **输入格式已查清，尚未制作可用样本**：profile 外层有 `name`，`extensions` 为数组序列化后的字符串；扩展项含 `identifier`，可含 `version`、`preRelease`、`disabled` 等。依据：[模板类型 L53](https://github.com/microsoft/vscode/blob/37645e695f1d0e93cf1bb17a550a9e1fdf9d4867/src/vs/workbench/services/userDataProfile/common/userDataProfile.ts#L53)、[扩展项 L24](https://github.com/microsoft/vscode/blob/37645e695f1d0e93cf1bb17a550a9e1fdf9d4867/src/vs/workbench/services/userDataProfile/browser/extensionsResource.ts#L24)。格式可写不等于所选扩展可查询、可安装或一定出现目标提示。
- **版本字段参与查询，但不是完整冻结**：[导入 L148](https://github.com/microsoft/vscode/blob/37645e695f1d0e93cf1bb17a550a9e1fdf9d4867/src/vs/workbench/services/userDataProfile/browser/extensionsResource.ts#L148)将 ID、版本与渠道交给 gallery；[查询 L697](https://github.com/microsoft/vscode/blob/37645e695f1d0e93cf1bb17a550a9e1fdf9d4867/src/vs/platform/extensionManagement/common/extensionGalleryService.ts#L697)处理指定版本，但发布者名称、域名和验证状态仍来自市场响应。不能只存一份带版本的 profile 就宣称回放稳定。
- **不是随便堆扩展就能触发**：[信任检查 L820](https://github.com/microsoft/vscode/blob/37645e695f1d0e93cf1bb17a550a9e1fdf9d4867/src/vs/workbench/services/extensionManagement/common/extensionManagementService.ts#L820)排除私有或已信任项，并按发布者去重；[正文 L906](https://github.com/microsoft/vscode/blob/37645e695f1d0e93cf1bb17a550a9e1fdf9d4867/src/vs/workbench/services/extensionManagement/common/extensionManagementService.ts#L906)逐条展开已验证域名，而全为多个未验证发布者时可能只显示概括语句。发布者构成、文本长度和实际顺序都会影响高度。
- **可在安装前观察，但准备并非零副作用**：[信任请求 L808](https://github.com/microsoft/vscode/blob/37645e695f1d0e93cf1bb17a550a9e1fdf9d4867/src/vs/workbench/services/extensionManagement/common/extensionManagementService.ts#L808)先取 manifest，再等待提示；[导入 L173](https://github.com/microsoft/vscode/blob/37645e695f1d0e93cf1bb17a550a9e1fdf9d4867/src/vs/workbench/services/userDataProfile/browser/extensionsResource.ts#L173)在该请求返回后才逐个安装。之前仍可能创建 profile、写设置或改启用状态；确认按钮会记录信任，取消会抛取消异常。必须隔离数据，不能把日常 profile 作为试验场。

若继续真实导入路线，至少还需准备：扩展 ID / UUID、版本与渠道、目标平台、gallery 原始响应和 manifest，以及发布者 ID / 显示名 / 域名 / 验证状态、实际展示顺序、已安装集与应用级信任记录。**本轮没有采集这些市场快照，也没有验证保存、使用或重放响应的条件。** profile 本身没有注入完整市场元数据的入口；离线服务替换或重定向属于另行评估的技术实验，不能当作已准备好的零成本手段。

### 新线索：同一修复的通用长文本弹窗

[Issue #114662](https://github.com/microsoft/vscode/issues/114662) 于 2021-01-20 报告自绘模态对话框缺少高度限制；原帖给出设置 `window.dialogStyle = custom`，并通过扩展 API 调用 `vscode.window.showInformationMessage('aaaaa\n'.repeat(100), { modal: true })` 的触发思路。

2026-07-28，项目成员在[评论](https://github.com/microsoft/vscode/issues/114662#issuecomment-5107806069)中明确指出，该问题由 `d481546f4c48b6592932b8e5808a4eed42d8823b` 解决并随 1.111.0 发行。本轮核对评论身份为 `MEMBER`；这是上游说明，不是本项目运行结果，也不宣称已确定最早修复版本。

另有 [Issue #125866](https://github.com/microsoft/vscode/issues/125866)：调试失败弹窗在 `window.zoomLevel: 2` 下不易关闭，维护者[标为 #114662 的重复](https://github.com/microsoft/vscode/issues/125866#issuecomment-947199020)。这提供了真实工作流与缩放条件的旁证，仍缺完整环境和本地复测。

这些材料支持“长内容与缩放导致操作区越界”不只出现在 profile 导入；它们来自同一项目、同一类组件，**不能当成多个独立产品的需求验证**。原帖 API 片段也不是可直接加载的完整扩展：若要试跑，仍需准备受控的最小测试扩展、隔离的历史应用与环境记录，并验证取消及滚动后的可达性。不得因此安装不明第三方扩展。

### 下次优先回答的问题与停止条件

1. **先比较验证成本**：评估 #114662 的受控扩展入口能否在历史 Windows 版中触发同一布局机制；若可行，再确定一个有限的技术实验范围。它可以减少 profile / 市场依赖，但不能冒充 V1 原件或原操作链复现。
2. **保留 V1 真实工作流价值**：仅当需要验证真实导入流程、且输入可追溯时继续准备；若市场快照或隔离成本过高，保持候选状态，不为凑数据集无限投入。
3. **先建立好坏状态再谈工具差异**：原帖代码、合并修复与评审批准均不自动成为真值；仍须记录内容区、文本、语言、字体 / 缩放、允许滚动与关键按钮命中。现有 Playwright / CDP 加人工检查能做到多少，也要计入对照成本。
4. **需求判断继续开放**：本轮支持这类故障确实存在，也提供了较小的验证入口；没有证明发生频率、现有检查的不足、使用者愿意接入或付费，以及自动缩减的收益。独立工具、插件、贡献上游或停止投入仍是开放选项。

检索边界：通过 GitHub CLI 只读查看原 Issue、评论、timeline、修复 PR / 文件 / 评审，并使用 `repo:microsoft/vscode "296538"`、`"trust publisher" "scroll"` 与标题中的 `dialog scroll` / `dialog height` 定向查询；查询均有返回数量限制，不代表全面覆盖。下载的截图与源码只留本地 `artifacts/`，不复制入评估集或 Git 历史。本轮未运行目标应用 / 竞品、未构建、未安装扩展、未作人工验收、未联系任何人，也未批准开发方向。
