# 证据与未执行项

> 日期：2026-11-10。change 已建立并补做 OpenSpec 严格校验；**尚无产品构建、运行、截图、便携写盘或人工体验通过记录**。
> 实施完成状态只在 [tasks](tasks.md) 维护。下表是待采集的证据归属，不是测试报告。

## 工具链核对与最小工程（任务 1.1–1.3，2026-11-11）

- **本机**：macOS 27.2.1 (arm64)；Apple clang 21.0.0（Xcode CLT）；rustc/cargo **1.99.1**（rustup minimal，经 USTC 镜像安装，crates 走 rsproxy sparse）；Node **22.22.2**（当前 PATH 指向 WorkBuddy 内置运行时，见 engineering 登记）；pnpm **12.18.0**（corepack 启用）。
- **依赖固定**：`tauri = 2.12.5`（`unstable` 功能，多 WebView 前置）+ `tauri-build 2`（2.7.4）；`@tauri-apps/api` / `cli` 2.12.4；TypeScript 5.9.3（strict）；Vite 7.3.8。锁定文件 `pnpm-lock.yaml` / `Cargo.lock` 入库；`target/`、`dist/`、`gen/` 不入库。
- **实跑命令与退出码**（2026-11-11 本机回显）：
  - `pnpm install`：退出码 0，esbuild postinstall 按 `allowBuilds` 放行执行成功。
  - `pnpm build:desktop`（= `tsc --noEmit && vite build`）：退出码 0，产物 `dist/index.html` 0.32 KB + JS 1.28 KB。
  - `cargo check`（`apps/desktop/src-tauri/`）：退出码 0，tauri 2.12.5 + unstable 编译通过，dev profile 28.39s。
- **许可核对**：tauri / tauri-build / @tauri-apps/* = MIT OR Apache-2.0；Vite = MIT；TypeScript = Apache-2.0；与本项目 MIT 兼容。完整第三方声明仍是发行前欠账（debt）。
- **工程落点**：根 `package.json` + `pnpm-workspace.yaml`（apps/*、packages/*、allowBuilds 审批表）；`apps/desktop/`（TS strict 前端骨架 + `src-tauri/`）；门禁收紧见 [engineering](../../../docs/knowledge/engineering.md)（图片后缀默认拦截，应用图标走 allowlist 理由豁免）。
- **对应证据表状态**：E01 / E12 / E14 的工具链与构建部分已有本机记录；E01 的**运行与人工操作、E02–E11、E13 及 E12 的 CI 侧**仍未执行，下表状态不变。

## 任务 2.x 实现（2026-11-11，运行级 / 人工验证未执行）

**代码落点**：`apps/desktop/src-tauri/src/lib.rs`（单窗口双 WebView 编排）、`fixture.rs` + `fixtures/sample-page.html` / `sub-page.html`（内嵌本地样例服务）、前端 `src/main.ts`（工具条 UI）。

**机器级事实（本机回显，退出码 0）**：

- `cargo check`：双 WebView 编排（`add_child` + incognito + 导航 / 加载 / 标题 / 新窗口 / 下载 / 权限回调）编译通过。
- `pnpm build:desktop`（tsc strict + vite）：工具层 UI 编译通过（产物 2.28 KB HTML + 4.36 KB JS）。
- 真实启动：`pnpm tauri dev` 全量编译 341 crates 后 `Running target/debug/layress-desktop`，进程持续存活、无 panic；样例服务监听 `127.0.0.1:52355`（随机端口，仅回环）。
- 样例服务路由（curl 实测）：`/` 200 HTML、`/sub` 200、`/download` 带 `Content-Disposition: attachment`、未知路径 404。

**实现要点（对应任务）**：

- 2.1：配置不定义窗口；`setup` 中 `WindowBuilder` 建唯一主窗口，工具层（label "main"，auto_resize 铺底）与被测层（label "page"）均 `incognito`；capability 仅列 "main"，"page" 无任何命令 / 事件权限。
- 2.2：`validate_url`（scheme 白名单 http/https + 主机校验）→ TCP 可达性预检（3s 超时，失败报错不触碰当前页面）→ 已存在页面原地 `navigate`（保留会话）否则 `add_child` 创建；加载状态经 `on_page_load` 回报。
- 2.3：样例页含点击计数、输入回显、滚动块、alert/confirm/prompt、同源路由链接；`on_new_window` → `Deny`、`on_download` → `false`、`on_permission_request` → `Deny`，均为明确拒绝并经事件上报工具条。
- 2.4：宽高设定写入状态并 `set_bounds` 固定页面视口；真实视口经初始化脚本写 title 哨兵（`innerWidth/innerHeight/devicePixelRatio`）→ `on_document_title_changed` 解析回传；未设定尺寸时页面跟随窗口，设定后窗口缩小保留视口并提示「容纳不下」。

**仍未执行（不得视为通过）**：

- E01：窗口内人工操作样例页（点击 / 输入 / 滚动 / 弹窗 / 路由）——应用已启动，待人工核对。
- E02：被测页面调用原生命令的实际拒绝结果（样例页已内置探测显示区）。
- E03：无效 scheme / 不可达地址在 UI 的实际拒绝表现（TCP 预检逻辑已实现，未实测）。
- E04：两个尺寸的视口核对、窗口缩小时的可见性提示（逻辑已实现，未实测）。
- E13：新窗口 / 下载 / 权限请求的实际拒绝反馈（回调已实现，未实测）。
- 样例页 title 哨兵在真实页面的上报是否可靠（wry macOS 支持，未实测）。

**已知显示问题与修复（2026-11-11，用户复验反馈）**：

- 用户反馈：样例页首行文字被工具 UI 穿过、宽高输入被窗口摆放挡住一部分；功能操作本身全部正常。
- 根因：叠层布局（工具层铺满 + 页面覆盖）下条带 CSS 内容高度（约 110px）超过 TOOLBAR_H（88px），
  加上 auto_resize 与显式 bounds 混用的坐标解释差异空间。
- 修复：改为显式互不重叠布局（工具条 `[0, TOOLBAR_H]`、页面 `[TOOLBAR_H, 窗口底]`，Resized 时双显式
  set_bounds）；TOOLBAR_H 与 CSS 高度统一为 96px 且 `overflow: hidden`；加窗口最小尺寸 520×400。
  修复后重编译启动正常；**待用户复验确认**，确认前不视为通过。

**限制**：无 back/forward（tauri 2.12.5 Webview 未暴露历史接口）。

## Agent 审阅与提交状态

2026-11-10 用户要求撤回最近的工具记录提交 `4b64372`，已保留全部文件改动，共享 OpenSpec 安装与下述校验事实仍有效。
用户随后明确最终流程：由用户切换另一 Agent 产出本目录 [review.md](review.md)，再回原编写 Agent 读取并修改；意见处理完成、必要检查通过后即可本地提交，无需再确认。当前 P1 修订已核对，P2 四项规划缺口已补齐；产品实施与实机验证仍未执行。本轮按用户指令提交上述修订及共享工具校验记录，完整暂存快照检查结果见 review 末节。

## 依据核对

- 本轮已读取 [Tauri WebviewBuilder API](https://docs.rs/tauri/latest/tauri/webview/struct.WebviewBuilder.html)与[Window API](https://docs.rs/tauri/latest/tauri/window/struct.Window.html)，API 限制见 [design](design.md)；原始抓取留 `artifacts/`，不入库。
- 建档时 PATH 未找到 OpenSpec CLI、Cargo、rustc 或 pnpm；随后按用户指定位置共享安装 `@fission-ai/openspec@1.17.0`，通过隔离配置 / 缓存 / 临时目录并关闭遥测的入口运行。安装方式见[工作流](../../../docs/development/workflow.md)。产品依赖解析、编译和实机探针仍未执行。

## 已执行的文档校验

在仓库根目录执行共享入口（当前布局为 `../tools/openspec/openspec`）：

- `--version`：退出码 0，输出 `1.17.0`；本机 Node `26.13.0` 满足该 CLI 要求的 `>=22.12.0`。
- `config path`：退出码 0，解析到共享工具根内的 `config/openspec/config.json`，不使用默认用户配置目录。
- `validate add-tauri-page-host --strict --no-interactive --json`：退出码 0，1 项通过、0 项失败，`valid: true`、`issues: []`。
- `status --change add-tauri-page-host --json`：退出码 0，proposal / specs / design / tasks 四类规划文档均为 `done`，返回 `isPlanningComplete: true` **与 `isComplete: true`**，并给出 `nextSteps` 建议进入 apply。这两个字段描述的是**规划产物齐备**，不是实施完成：本 change 的 tasks 全部未勾选、E01–E14 证据均为「未执行」，不得据此写成任务已完成或可以提交。

原始 CLI 输出中的本机绝对路径不入库；上述命令、版本、退出码与关键结果作为可重跑的文档校验摘要。

## 实施证据计划

| 编号 | 判据与反例 | 应保留的证据 | 当前状态 |
|---|---|---|---|
| E01 | 单窗口内导航、输入、滚动与页面内弹窗 | 固定依赖 / 系统版本、样例来源、操作步骤与人工结果 | 未执行，尚无产品工程 |
| E02 | 被测页面调用原生命令须拒绝 | 权限配置、负例请求与实际拒绝结果 | 未执行 |
| E03 | 无效 scheme / 不可达地址不能报成功 | 输入、失败状态、当前页面身份 | 未执行 |
| E04 | 两尺寸实际视口与配置一致，布局变化不得偷改 | 实际视口 / DPR / 原生尺寸、条件差异与人工结果 | 未执行 |
| E05 | 真实截图无工具层，URL / 条件绑定本次页面 | 版本化元数据、截图引用与采集开始 / 完成状态 | 未执行 |
| E06 | 采集失败或导航竞态不能显示旧图为新成功 | 可重复的失败 / 切换步骤与结果 | 未执行 |
| E07 | 临时矩形在两种展示比例下对应原图 | 已知区域坐标、展示比例、人工落点与转换验证 | 未执行 |
| E08 | 返回原 WebView 不因预览切换重载 | 切换前后的输入 / 弹窗与页面身份，人工观察 | 未执行 |
| E09 | 不可写数据根必须失败，不兜底 | 输入目录条件、可写检查与错误，无关目录未改变的观测 | 未执行 |
| E10 | 非持久模式也要验证越界写入；已知坏样本必须检出，不完整观测不能通过 | 观测工具 / 权限与覆盖范围、正反对照、进程 / 时间归因、前后差异、排除无关活动依据，方法见下节 | 未执行，不能声明便携通过 |
| E11 | 退出取消采集并释放资源 | 进程 / 资源与重启观察、残留情况 | 未执行 |
| E12 | 本地与 CI 共用检查，平台结果不混写 | 实际命令、退出码、平台适用范围、本地构建大小及依赖成本 | 未执行 |
| E13 | 页面新窗口 / 下载 / 设备权限请求受到明确处理 | 触发样例、请求类型、允许 / 拒绝理由、顶层窗口 / 文件 / 权限的实际变化 | 未执行 |
| E14 | 未执行、失败、不确定与通过分开记录 | 规格场景与证据编号对照、各平台 / 人工项的实际状态，验证缺失项不能被汇总为通过 | 未执行 |

实施后截图等运行资料放产品数据根；一次性探针输出只放 `artifacts/`，索引用相对路径定位，不提交真实页面、二进制或本机绝对路径。
当前没有预先创建证据目录或虚构文件链接；实际产生证据时再登记路径、命令与结论。

## E10 写盘观测方法（实施时执行）

1. 记录系统版本、观测工具 / 权限、目标与 WebView 辅助进程、启动至退出及延迟清理的时间范围；在不启动目标的对照时段记录背景活动。目录快照与可用的文件事件观测相互核对，不能只看退出后是否残留。
2. 先在 `artifacts/` 隔离夹具内设置测试数据根和根外的模拟禁止写入目录：由已知测试进程在禁止目录创建、修改并删除哨兵文件。观测链必须识别越界事件，合法根内写入则不得误报；不为反例故意污染真实系统用户目录。此反例只验证检测链，不能代替真实 WebView 写盘观测。
3. 对真实应用分别观测启动、导航、输入、采集、预览、退出，覆盖约定数据根与本平台可能的会话 / 缓存 / 日志 / 临时目录；进程过滤必须说明 WebView 系统辅助进程是否覆盖，不能默认全部都是应用子进程。
4. 将事件按进程身份、路径、时间和前后对照归因；不能仅凭系统目录变动认定为产品写入，也不能把无法归因的事件全部忽略。记录排除无关系统活动的依据、覆盖盲区和残留。
5. 已知坏样本未检出、事件丢失、权限不足、辅助进程覆盖不完整或归因不清时，结果标为失败 / 不确定 / 未执行，并保留原因；不得标为“未发现越界，因此便携通过”。只有声明的测试范围内证据充分时才能记录该范围通过，不外推为所有系统写入均被覆盖。

## 未覆盖范围

Windows / Linux 实机、移动真机、多引擎、复杂登录 / 新窗口、持久会话、设备预设恢复、完整批注与反馈、Agent 接入均不在本 change 已验证范围。
文档门禁与结构核对可在建档时执行，其结果另由提交记录确认，不改变上述运行项状态。
