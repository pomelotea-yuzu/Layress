# 证据与未执行项

> 建档 2026-11-10，更新 2026-11-13。1.x–2.x 已构建、真实启动并经用户人工复验；
> **2.2 无效地址负例于 2026-11-13 经用户人工复验通过并勾选**（8 条中 7 条符合预期，第 2 条错因归因错误已修并加单测；
> 修后文案同日由用户 GUI 复验通过，原单列项已销账，详见下节）；
> 3.x 真机探针取得 PNG 与版本化元数据，**3.1 / 3.2 / 3.3 全部经用户人工复验通过并勾选**
> （竞态负例经慢采开关与底部结果条补齐后于 11:53 确认）；4.x 中 4.1 / 4.3 / 4.4 于 2026-11-13 15:42 经用户人工复验通过并勾选，4.2 已按 D30 移出本 change 范围（见下节），5.x 未开始。
> 实施完成状态只在 [tasks](tasks.md) 维护。
> 下表是证据归属与未执行项，不是验收报告。

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
- 2026-11-11 追加（用户建议）：应用设定视口后，页面边界注入 1px 固定定位描边（`html.__layress_frame_on::after`，
  pointer-events:none），按「设定视口且小于舞台」动态开关（page_set_size / Resized / 页面加载完成三个时机同步）。
  该边框是**工具辅助标记**：任务 3.x 采集真实画面时必须先关闭再截图；严格 CSP 页面可能拦截注入样式导致边框不显示
  （如实记录，不视为采集失败）。待用户复验。

## 人工复验结果（2026-11-11，用户实测）

- **通过**（用户口头确认"都点了、没问题"）：
  - E01：窗口内点击计数、输入回显、滚动、alert / confirm / prompt、子页路由（E01 相关部分）。
  - E02：样例页原生命令边界探测区全部显示 undefined / invoke 被拒绝。
  - E13：window.open / 下载 / 地理位置三个负例按钮均明确拒绝并上报工具条。
  - E04：设定视口后页面实测值与设定可核对；显示重叠修复后布局确认；视口边框确认。
- **未执行（单列，不得视为通过）**：
  - E03：无效 scheme / 不可达地址的人工负例（清单未包含，实现已完成待补测）。
  - E05–E12：采集、预览、写盘观测、退出清理等属 3.x / 4.x，未开始。
- **本轮修复记录**：显示重叠（工具条 CSS 内容超 TOOLBAR_H + 叠层布局）→ 互不重叠布局 + 96px 严格对齐，
  用户复验确认；视口边框功能按用户建议追加，复验确认。

**限制**：无 back/forward（tauri 2.12.5 Webview 未暴露历史接口）。

## 任务 2.2 实现补齐与机器证据（2026-11-13，人工复验未执行）

**背景**：2.2 的无效 scheme / 不可达地址人工负例自 2026-11-11 起一直未执行。本轮先清掉挡在人工复验前面的可观察性问题，**未勾选 2.2**。

**代码落点**：`apps/desktop/src/main.ts`（导航 / 加载 / 拒绝 / 刷新反馈改走底部状态栏；预检失败后按 `page_state` 把地址栏恢复成真实页面 URL）、`status.html`（状态栏注释泛化为全操作反馈）、`src-tauri/src/lib.rs`（抽出同步 `probe_reachable` + 4 项单测，`validate_url` 补 authority 前置校验）。

**机器级实跑（本机 macOS arm64，退出码均为 0）**：

| 命令 | 结果 |
|---|---|
| `cargo check`（`apps/desktop/src-tauri/`） | 编译通过 |
| `cargo test --lib` | 11 项通过 / 0 失败（原 6 项采集状态机 + 5 项地址校验 / 可达性 / 错因归因） |
| `pnpm build:desktop` | `tsc --noEmit` + Vite 构建通过 |
| `node scripts/check-repo.mjs --all` | 洁净度 0 失败 / 0 警告、34 项自测、66 篇文档链接、5 对双语 0 问题 |

**单测覆盖的正反样本**：11 类无效地址（空、无 scheme、javascript / file / data / ftp、空主机、`http:///path`、`http:////example.com/`、超范围端口）、HTTP(S) 合法地址与 scheme 大小写规范化、无监听的本地端口必须判不可达、有监听的本地端口必须通过（防「一律失败」假绿），以及 file / ftp / data 的错因必须归因到 scheme 而非主机名（用户人工复验反馈固化）。

**发现并修复的两个漏口**：① `http:///path` 被 url 解析器把路径段规范化成主机名，绕过 host 非空检查，只靠 DNS 碰巧失败兜底；已在校验阶段拒绝并加断言。② 该前置检查抢在 scheme 白名单之前，把 `file:///etc/passwd` 误报成「缺少主机名」；已限定仅 http(s) 生效并加错因断言。结论同轮回填 [`engineering.md`](../../../docs/knowledge/engineering.md)。

**后续结果（2026-11-13）**：上述 GUI 项已由用户人工复验通过，见下节「任务 2.2 用户人工复验」；2.2 已勾选。第 2 条错因归因错误在复验中发现并修复，**修后文案同日由用户 GUI 复验通过，原单列项已销账**。机器单测不替代人工验收（I11）。

## 任务 2.2 用户人工复验（2026-11-13，通过；单列项已销账）

**用户原话**：「第二个是 已拒绝：地址缺少主机名，其他没有问题」。按 [app README](../../../apps/desktop/README.md) 的 8 条清单执行；观察通道为窗口最底部独立状态栏（工具层第三行在 64px 视口下被裁，不依赖）。

| 结果 | 内容 |
|---|---|
| 通过 | 第 1 / 3 / 4 / 5 / 6 / 7 / 8 条均符合预期：危险 scheme 与畸形 http(s) 地址被明确拒绝、无 scheme 报无法解析、不可达地址报目标不可达、失败后正常导航恢复 |
| 通过 | 每条三核对点：红条出现、被测页面仍是原来那个（不跳转 / 不空白）、地址栏不残留无效地址 |
| 需修（已修） | 第 2 条 `file:///etc/passwd` 当时显示「已拒绝：地址缺少主机名」，实际应为「不支持的 scheme」；拒绝行为正确但**错因归因错误** |

**根因与修复**：本轮为拦 `http:///path` 新增的 authority 前置检查抢在 scheme 白名单之前，先按路径形状判错。已把该检查限定为仅对 http(s) 生效，并新增单测 `reject_reason_keeps_scheme_before_host_check`，断言 file / ftp / data 的错因含 scheme、不含主机名，http(s) 畸形地址仍报缺少主机名。修后 `cargo check` 0、11 项单测全通过。

**单列项销账（2026-11-13 15:10）**：修复后的第 2 条文案「不支持的 scheme」原仅单测断言，用户随后确认「新文案是对的了」⇒ 已在 GUI 上人工复验通过；其余 7 条与三核对点亦由用户人工确认。E03 判据「地址无效 / scheme 不受支持 / 目标不可达时明确拒绝，不把旧页面描述为新地址加载成功」已由用户人工确认 ⇒ **2.2 勾选、E03 回填通过**。机器单测不替代该判据（I11）。

## 任务 3.x 实施证据（2026-11-12）

**代码落点**：`apps/desktop/src-tauri/src/capture.rs`（WKWebView 原生快照、`layress.capture/v1` 元数据、
数据根最小解析、竞态状态机与 6 项单测）、`lib.rs`（采集 / 取消 / 退出预览命令、事件、平移式预览布局、
title 哨兵）、`src/coords.ts`（坐标换算与运行时往返自检）、`src/main.ts` + `index.html`（静态预览 UI）、
`fixtures/sample-page.html`（视口 CSS (24,24) 160×60 固定参照块）、`Cargo.toml`（仅 macOS 的 objc2 / block2）。

**机器级实跑（本机 macOS arm64，退出码均为 0）**：

| 命令 | 结果 |
|---|---|
| `cargo check`（`apps/desktop/src-tauri/`） | 编译通过，tauri 2.12.5 `unstable` + objc2 0.6.5 / block2 0.6.2 |
| `cargo test --lib` | 6 项通过 / 0 失败：竞态 4 项、哨兵键名一致性 1 项、PNG 尺寸与 base64 1 项 |
| `pnpm build:desktop` | `tsc --noEmit` + Vite 构建通过（产物 HTML 5.55 KB、JS 11.33 KB） |
| `node scripts/check-repo.mjs --all` | 洁净度 0 失败 / 0 警告、34 项自测、66 篇文档链接 0 问题、5 对双语 0 问题 |
| `../../tools/openspec/openspec validate add-tauri-page-host --strict --no-interactive --json` | 1 项通过 / 0 失败，`valid: true` |
| `LAYRESS_PROBE_CAPTURE=1 pnpm tauri dev` | 4 次真机探针均成功，末次 `cap-1794451671376-1`：PNG 2560×1408 RGBA / 237236 B、开始→完成 43 ms，`file` 识别为 PNG |

**真机采集元数据摘要（末次探针）**：`schema=layress.capture/v1`、`page.label=page`、
`page.url=http://127.0.0.1:62407/`、`generation=2`、`bounds=[1280,704]`、
`measured=1280×704 @DPR 2 / scroll 0 / session=tr4fzbtcdt-mwcy6xlf`、
`image=2560×1408 px / 237236 B`、`conditions.scroll=窗口滚动 y=0（内部容器滚动未取得）`、
`missing` 3 条（内部容器滚动与跨 iframe、动态画面原子性、严格 CSP 下注入未实测）、数据根绝对路径。
产物位于 `apps/desktop/src-tauri/target/debug/.layress/captures/`（`target/` 已忽略，**不入库**）；
探针未向 `artifacts/` 或系统用户目录写测试垃圾。

**按证据编号的机器结论**：

- **E05（机器 + 人工通过）**：视图级快照像素 = 逻辑视口 × DPR（1280×704 → 2560×1408），
  工具层与被测层互不重叠且 `takeSnapshot(nil)` 只取被测视图 bounds；用户目视确认截图无工具标记 / 其他窗口。
- **E06（机器 + 人工通过）**：状态机单测覆盖「新请求替代旧请求、导航作废在途、失败不替换旧图、
  预览退出不改身份」；`race` 探针在慢采窗口内刷新，回显采集失效且 PNG 数不变（11 → 11），
  证明失效结果未落盘、未替换 `last`；用户随后用「慢采 5s」在 GUI 上完成同类负例并确认流程验证成功。
- **E07 / E08（机器 + 人工通过）**：坐标往返自检在工具层运行时执行并把结论显示在预览头；
  参照块按 `imageW / 页面实测 w` 换算；预览只改被测层 position、保持 size ⇒ 不触发 resize、不销毁视图。
  用户确认两种比例下矩形落点不漂、遮挡正常、返回后输入值与点击计数保留、会话标识未变化；
  **弹窗会话未单独触发**，单列待补。
- **E09（部分）**：数据根解析、启动 / 采集前可写检查、日志 / 事件 / UI 显示已落地（3.x 写盘前置）；
  不可写负例与 WebView 真实写盘观测（4.1 / 4.2）未执行，不得写成便携通过（本节为 3.x 当时状态；4.1 已于 11-13 复验通过，4.2 已按 D30 移出）。

**本轮发现并已修的坑**（结论已同轮回填 [`engineering.md`](../../../docs/knowledge/engineering.md)
与 [`runtime-probe.md`](../../../docs/knowledge/runtime-probe.md)）：

- title 哨兵 JSON 键名写 `sx`/`sy`，Rust 结构字段为 `scroll_x`/`scroll_y` ⇒ 元数据把实现 bug
  静默标成「未取得」；已统一键名并新增单测断言键名一致性。
- 初始化脚本写的 `document.title` 会被页面自身 `<title>` 覆盖 ⇒ 补 `DOMContentLoaded` / `load` 再上报，
  首轮不改视口也能取得实测值。
- `State<'_, T>` 临时借用与块尾 `?` 组合触发 E0597 ⇒ 结果存局部变量再返回 / 拆独立函数。
- D27 仓库归位后共享 OpenSpec 入口由 `../tools/...` 变为 `../../tools/...`，工作流与本文件已同步。

**用户人工复验（2026-11-12，用户原话「其他三条是没有问题的」）**：

- **E05 通过**：截图目视确认无工具条、无视口边框、无其他窗口混入。
- **E07 通过**：原始比例 / 适应区域两模式下矩形落点不漂，原图坐标数值一致，预览遮挡正常。
- **E08 通过**：返回页面后输入框内容与点击计数保留、会话标识未变化。
- **E06（当时未执行，后续已通过）**：第 4 项「采集期间导航 / 关闭」因采集仅约 43 ms，用户来不及触发，明确说明「采集的太快了导致我还来不及关」；补「慢采 5s」开关与底部结果条后，用户于 11:53 确认「流程是验证成功了没有问题」，**任务 3.2 已勾选**。
- 单列：弹窗会话未单独触发；任务 2.2 的无效地址 / 不可达负例仍未测（本节为 2026-11-12 当时状态；2.2 已于 11-13 复验通过，见前节）。

**慢采开关（为 E06 人工复验补的可控手段）**：工具条新增「慢采 5s」复选框（仅开发构建显示；
Rust `capture_delay` 只在 debug 生效、上限 15 s、默认 0，产品路径零影响）。勾选后采集在
「已拿到 PNG，但尚未校验身份 / 写盘 / 显示」之间额外等待 5 s，期间点「刷新」/「取消」/ 再次「采集」
即可触发失效；失败状态行会提示到数据根 `captures/` 核对本次没有新文件。

**E06 运行级机器证据（不替代人工验收）**：`LAYRESS_PROBE_CAPTURE=race cargo run` 在慢采 3 s 窗口内
第 300 ms 自动 `page_refresh`，回显
`[layress][probe] race 通过：采集已失效（本次采集已失效：页面已切换或被新请求替代，结果已丢弃）；captures PNG 数 11 → 11`
⇒ 导航确实作废在途采集，且失效结果未落盘。**GUI 人工负例仍待用户用慢采开关复验**。

**用户反馈与 UI 修复（2026-11-12 追加）**：用户按慢采步骤操作后反馈「按照你的顺序点完，是没有出现状态行的」，
即预期失败提示不可见。定位到两处，均与采集逻辑无关：

1. 失败提示只写工具条第三行 `#status-line`，它和视口 / 请求信息共用一行，长文本被 flex 挤压省略，
   又会被 `Status loading / loaded` 事件覆盖；旧请求被新采集替代时还被 `captureSeq` 静默忽略。
2. 开发期新增 DOM 用模块顶层 `assertElement` 获取，HTML 与 JS 不同步（旧窗口未重载、Vite HMR 只推 JS）
   会直接抛异常，使整个工具界面脚本停摆、按钮与状态行全失效——这能解释「什么都没出现」。
   参数名映射已核对 Tauri 2.12.5 宏默认 camelCase（`delay_ms` → `delayMs`），**不是**慢采未生效的原因。

修复：新增 `#capture-toast` 常驻结果条（只由 Capture 阶段更新、`title` 存完整文本、旧请求结果也留痕、
不被加载 / 视口事件覆盖），新增元素一律可选获取，并在 HTML/JS 不同步时于启动时给出明确重启指引；
`#status-line` 改为 flex 优先。**该 UI 修复本身尚未经用户 GUI 复验，任务 3.2 仍不勾选。**

**第二轮反馈（2026-11-12 追加）**：用户按修复后的步骤操作仍反馈「没有黄色的条」。核对：Vite devUrl 返回的 HTML、
磁盘 `index.html` 与 `dist/index.html` 均含 `capture-toast` / `slow-capture`；二进制内不含前端资源
（dev 模式走 devUrl），排除二进制缓存。旧窗口启动时间早于 11:20 的 UI 改动，且 CSP `default-src 'self'`
挡掉 Vite HMR websocket，页面不会自动重载 ⇒ 工具层 WebView 仍是改动前的 DOM，不是代码没生效、也不是用户操作错。
已杀净旧 pnpm / vite / 应用进程并重新起完整 `pnpm tauri dev`（回显 VITE ready + `Running target/debug/layress-desktop`）。
**UI 修复与 3.2 负例仍待用户在新窗口确认，不勾选**。

**第三轮反馈与真根因（2026-11-12）**：用户在新窗口仍反馈「还是没出现你说的黄条」。
改用 `LAYRESS_PROBE_TOOL=1` 对工具层 WebView 自身做 DOM 回报 + `takeSnapshot` 截图，日志给出
`toast=true status=true innerH=64 body=2551 href=http://localhost:5173/`、
`bounds=Logical(1280×96)`、窗口 `Physical(2560×1600)`、`scale=2.0`：
新版 HTML 确实加载、结果条元素确实存在，但**工具层 CSS 视口只有 64px**，`#bar` 高 96px 且
`overflow:hidden`，第三行状态文字与锚在 `#bar` 底部的结果条一起被裁掉；
不是 z-index 遮挡、不是 HTML 版本、也不是 CSP。修复为 `position:fixed; bottom:0`（贴视口底），
复跑探针截图已能清晰读出黄色诊断条文字，属机器 + 目视实证；
后续用户在真实采集流程中确认结果条可见、竞态流程验证成功，**任务 3.2 已勾选**。
用户另反馈结果条半透明压在工具栏上观感不好（要求移到最底下），已作为 UI 调整待办，不影响本次验收结论；
工具条第三行状态信息不可见已登记为 UI 欠账。

**结果条形态落地（2026-11-12 用户选定后）**：用户选「窗口底部独立状态栏」。实现为第三个 WebView
（label `status`、`WebviewUrl::App("status.html")`、Vite 双入口、无 capability、页面无脚本），
bounds 实测 `Logical(0,758) 1280×42`（窗口 800 逻辑高，758+42=800），状态栏页面回报
`bar=true msg=true innerH=42 href=http://localhost:5173/status.html`。
`status_bar_set` 命令走真实链路注入文本与级别；`LAYRESS_PROBE_TOOL=1` 的 `status-probe.png`
显示浅绿色条与「[probe] 底部状态栏可见性自检：看到这条说明结果条在窗口最底部且不遮工具栏」，
属机器 + 目视实证。工具层 DOM 的结果浮层已删除，不再遮挡工具栏；
页面区高度统一由 `stage_height()` 计算。该调整不改变 3.x 已通过的验收结论。

**仍未执行（单列，不得视为通过；本节为 11-12 当时状态，2.2 已于 11-13 复验通过）**：弹窗会话单独触发；
4.1 不可写负例、4.2 写盘观测、4.3 原生命令 / 文件 / 进程访问负例、4.4 退出清理；
5.x 构建 / 类型 / 测试接入公共门禁、规格正反场景与归档；Windows / Linux、真实移动设备、
复杂登录站点、严格 CSP 注入、动态画面原子性、PNG 经 base64 走 IPC 的大图开销。

## 任务 4.x 实施与前置条件（2026-11-13）

**用户口径修正（15:15，原话「现在不是还在开发，没有打包的需求，便携还不"存在"吧」）**：便携优先（I12）的载体是打包后的产品目录；当前 `pnpm tauri dev` 下 exe 在 `src-tauri/target/debug/`、数据根在 `target/debug/.layress/`，同时存在 Vite / pnpm / cargo / WKWebView 辅助进程的写入，既没有「产品目录」可删，也无法按 E10 把写入归因到产品本身。因此 4.2 在开发态是**前置条件未满足 / 未执行**，不是「观测不完整 / 不确定」；本 change 不搭观测脚本、不勾 4.2、不宣称便携，真实观测挂到打包后的发行轮，登记为未决口径（[debt](../../../docs/knowledge/debt.md)）。**2026-11-13 15:49 用户选方案 A（D30），正式移出本 change 范围，该未决项已定夺。**

**本轮代码落点**：`capture.rs`（`resolve_data_root_from` 纯函数 + 2 项单测：优先级 / 不回落系统用户目录 / 不可写明确失败不兜底）、`main.ts`（`DataRoot` 事件与 `page_state` 初始化都把数据根解析结果点亮到底部状态栏，不可写为红条，I13 + I15）、`fixtures/sample-page.html`（文件 / 进程 / 采集命令的原生访问负例探测区，E02、E13）、`lib.rs`（`WindowEvent::Destroyed` 作废在途采集后只清理数据根内 `.write-probe` 残留并打印保留说明，不删除数据根外任何文件，E11）。

**机器级实跑（本机，退出码 0）**：`cargo check`；`cargo test --lib` **13 项通过 / 0 失败**（新增 2 项数据根单测）；`pnpm build:desktop`；`node scripts/check-repo.mjs --all`。

**用户人工复验（2026-11-13 15:42，原话「做完了，没遇到问题」）**：4.1 / 4.3 / 4.4 按 [app README](../../../apps/desktop/README.md) 清单执行，全部通过 ⇒ tasks 三项勾选、E09 / E02 / E11 回填通过。4.2 已按 D30 移出范围（非勾选）。机器单测不替代人工验收（I11）。

## 任务 4.x 用户人工复验（2026-11-13 15:42，通过）

| 任务 | 用户确认的内容 | 判定 |
|---|---|---|
| 4.1 | A 默认根启动状态栏显示数据根与「可写」、终端日志一致；B `--data-dir` 自定义可写根生效、产物落在该处、命令行优先于环境变量；C `/dev/null/layress-probe` 不可写 ⇒ 红条 + 采集明确失败 + `~/Library` 与默认根无兜底目录、PNG 数不变 | 通过（E09） |
| 4.3 | 四个探测按钮（fetch 本地文件 / invoke 文件读取 / 进程 / 采集命令）全部被拒绝，无「意外成功」；原生命令边界探测区仍为 undefined / 被拒绝 | 通过（E02、E13） |
| 4.4 | 慢采 5s 期间关窗口 ⇒ 本次未落盘（PNG 数不变）、旧截图保留、数据根外无删除或改动；重启正常打开、状态栏再次显示数据根 | 通过（E11） |
| 4.2 | 开发态没有打包后的产品目录，混入 Vite / cargo / WKWebView 辅助进程写入无法归因；用户选方案 A / D30 移出本 change | **移出范围（D30），归发行轮** |

**如实口径**：4.3 的「进程 / 文件访问」以调用不存在的原生命令为代理，Web 页面本身无这类 API，不声称覆盖完整进程访问；4.4 的「数据根外无改动」只代表本次人工观察范围，不外推为所有系统写入均被覆盖。以上均由用户人工确认，机器单测与门禁不替代（I11）。

## 任务 5.1 门禁接入（2026-11-13）

**代码落点**：新增 [`check-build.mjs`](../../../scripts/check-build.mjs)（`--all` 实跑三条检查、`--selftest` 用恒失败命令反证退出码传播），接入 [`check-repo.mjs`](../../../scripts/check-repo.mjs) 的默认 checks 链；根 `package.json` 加 `check:ci`；[`.github/workflows/ci.yml`](../../../.github/workflows/ci.yml) 补 pnpm / Rust / Linux WebView 系统依赖并调用同一条 `--all` 链。

**机器级实跑（本机 macOS arm64 / darwin，退出码 0）**：`node scripts/check-repo.mjs --all` ⇒ `pnpm build:desktop`（tsc strict + Vite，922 ms）、`cargo check`（138 ms）、`cargo test --lib`（13 项通过 / 0 失败，121 ms）退出码均为 0；`check-build.mjs --selftest` 断言「恒成功放行、恒失败被拦」通过。任一失败即整链退出非 0，**不提供跳过开关**。

**平台适用范围（不混写）**：已实跑 = macOS arm64（Node 22.22.2 / cargo 1.99.1 / pnpm 12.18.0）；配置已更新但首次运行回显未取得 = ubuntu-latest（CI，需用户 push 后确认）；未验证 = Windows、真实移动设备、非 macOS 原生快照实现。

**口径差异（如实记录）**：`--staged` 的索引快照不含 `node_modules` / `target` 缓存，无法编译，因此构建类检查由 `check-repo.mjs` 指定在真实检出目录（`ROOT`）执行；洁净度 / 链接 / 双语仍按索引快照。构建失败照样让整条链退出非 0，不是绕过。

**单列未执行**：CI ubuntu 侧首次运行回显（需用户 push）；Windows 平台。两者不得因本机全绿汇总为通过（I11、E14）。

## Agent 审阅与提交状态

2026-11-10 用户要求撤回最近的工具记录提交 `4b64372`，已保留全部文件改动，共享 OpenSpec 安装与下述校验事实仍有效。
用户随后明确最终流程：由用户切换另一 Agent 产出本目录 [review.md](review.md)，再回原编写 Agent 读取并修改；意见处理完成、必要检查通过后即可本地提交，无需再确认。当前 P1 修订已核对，P2 四项规划缺口已补齐；产品实施与实机验证仍未执行。本轮按用户指令提交上述修订及共享工具校验记录，完整暂存快照检查结果见 review 末节。

## 依据核对

- 本轮已读取 [Tauri WebviewBuilder API](https://docs.rs/tauri/latest/tauri/webview/struct.WebviewBuilder.html)与[Window API](https://docs.rs/tauri/latest/tauri/window/struct.Window.html)，API 限制见 [design](design.md)；原始抓取留 `artifacts/`，不入库。
- 建档时 PATH 未找到 OpenSpec CLI、Cargo、rustc 或 pnpm；随后按用户指定位置共享安装 `@fission-ai/openspec@1.17.0`，通过隔离配置 / 缓存 / 临时目录并关闭遥测的入口运行。安装方式见[工作流](../../../docs/development/workflow.md)。产品依赖解析、编译和实机探针仍未执行。

## 已执行的文档校验

在仓库根目录执行共享入口（D27 归位后的本机布局为 `../../tools/openspec/openspec`）：

- `--version`：退出码 0，输出 `1.17.0`；本机 Node `26.13.0` 满足该 CLI 要求的 `>=22.12.0`。
- `config path`：退出码 0，解析到共享工具根内的 `config/openspec/config.json`，不使用默认用户配置目录。
- `validate add-tauri-page-host --strict --no-interactive --json`：退出码 0，1 项通过、0 项失败，`valid: true`、`issues: []`。
- `status --change add-tauri-page-host --json`：退出码 0，proposal / specs / design / tasks 四类规划文档均为 `done`，返回 `isPlanningComplete: true` **与 `isComplete: true`**，并给出 `nextSteps` 建议进入 apply。这两个字段描述的是**规划产物齐备**，不是实施完成：本 change 的 tasks 全部未勾选、E01–E14 证据均为「未执行」，不得据此写成任务已完成或可以提交。

原始 CLI 输出中的本机绝对路径不入库；上述命令、版本、退出码与关键结果作为可重跑的文档校验摘要。

## 实施证据计划

| 编号 | 判据与反例 | 应保留的证据 | 当前状态 |
|---|---|---|---|
| E01 | 单窗口内导航、输入、滚动与页面内弹窗 | 固定依赖 / 系统版本、样例来源、操作步骤与人工结果 | 通过（2026-11-11 用户人工复验：单窗口点击 / 输入 / 滚动 / 弹窗 / 子页路由） |
| E02 | 被测页面调用原生命令须拒绝 | 权限配置、负例请求与实际拒绝结果 | 通过（2026-11-11 人工确认原生命令探测区全部 undefined / 被拒绝；2026-11-13 用户复验 4.3 四个探测按钮全部被拒绝，未出现「意外成功」） |
| E03 | 无效 scheme / 不可达地址不能报成功 | 输入、失败状态、当前页面身份 | 通过（2026-11-13 用户人工复验 8 条 + 三核对点；第 2 条错因已修且修后文案同日 GUI 复验通过） |
| E04 | 两尺寸实际视口与配置一致，布局变化不得偷改 | 实际视口 / DPR / 原生尺寸、条件差异与人工结果 | 通过（2026-11-11 用户人工复验：设定视口实测可核对、窗口容纳不下保留视口并提示） |
| E05 | 真实截图无工具层，URL / 条件绑定本次页面 | 版本化元数据、截图引用与采集开始 / 完成状态 | 通过（机器探针 + 2026-11-12 用户目视） |
| E06 | 采集失败或导航竞态不能显示旧图为新成功 | 可重复的失败 / 切换步骤与结果 | 通过（6 项单测 + race 探针 11→11 + 2026-11-12 用户慢采 GUI 负例） |
| E07 | 临时矩形在两种展示比例下对应原图 | 已知区域坐标、展示比例、人工落点与转换验证 | 通过（机器自检 + 2026-11-12 用户两模式核对） |
| E08 | 返回原 WebView 不因预览切换重载 | 切换前后的输入 / 弹窗与页面身份，人工观察 | 通过（用户确认输入值 / 点击计数 / 会话标识）；弹窗会话未单独触发 |
| E09 | 不可写数据根必须失败，不兜底 | 输入目录条件、可写检查与错误，无关目录未改变的观测 | 通过（2026-11-13 用户人工复验 A/B/C 三档：启动可见、自定义根生效、不可写明确失败且不回落；另有 2 项单测与启动状态栏显示） |
| E10 | 非持久模式也要验证越界写入；已知坏样本必须检出，不完整观测不能通过 | 观测工具 / 权限与覆盖范围、正反对照、进程 / 时间归因、前后差异、排除无关活动依据，方法见下节 | **移出本 change 范围（2026-11-13 方案 A / D30）**：归首次打包后的发行轮 change；本 change 不宣称便携通过，E10 观测方法保留供发行轮复用 |
| E11 | 退出取消采集并释放资源 | 进程 / 资源与重启观察、残留情况 | 通过（2026-11-13 用户人工复验：慢采期间关窗口本次未落盘、旧图保留、数据根外无改动、重启正常；代码只清理数据根内探针残留） |
| E12 | 本地与 CI 共用检查，平台结果不混写 | 实际命令、退出码、平台适用范围、本地构建大小及依赖成本 | 部分（本机 + 配置）：2026-11-13 任务 5.1 已把构建 / 类型 / 13 项单测接入 `check-repo.mjs` 公共链并本机实跑全绿（含失败传播自测）；CI ubuntu 配置已更新、首次回显待用户 push；Windows 未验证 |
| E13 | 页面新窗口 / 下载 / 设备权限请求受到明确处理 | 触发样例、请求类型、允许 / 拒绝理由、顶层窗口 / 文件 / 权限的实际变化 | 通过（2026-11-11 用户人工复验：window.open / 下载 / 地理位置均明确拒绝并上报） |
| E14 | 未执行、失败、不确定与通过分开记录 | 规格场景与证据编号对照、各平台 / 人工项的实际状态，验证缺失项不能被汇总为通过 | 未执行 |

实施后截图等运行资料放产品数据根；一次性探针输出只放 `artifacts/`，索引用相对路径定位，不提交真实页面、二进制或本机绝对路径。
当前没有预先创建证据目录或虚构文件链接；实际产生证据时再登记路径、命令与结论。

## E10 写盘观测方法（按 D30 移出发行轮，本 change 不执行；方法保留供复用）

1. 记录系统版本、观测工具 / 权限、目标与 WebView 辅助进程、启动至退出及延迟清理的时间范围；在不启动目标的对照时段记录背景活动。目录快照与可用的文件事件观测相互核对，不能只看退出后是否残留。
2. 先在 `artifacts/` 隔离夹具内设置测试数据根和根外的模拟禁止写入目录：由已知测试进程在禁止目录创建、修改并删除哨兵文件。观测链必须识别越界事件，合法根内写入则不得误报；不为反例故意污染真实系统用户目录。此反例只验证检测链，不能代替真实 WebView 写盘观测。
3. 对真实应用分别观测启动、导航、输入、采集、预览、退出，覆盖约定数据根与本平台可能的会话 / 缓存 / 日志 / 临时目录；进程过滤必须说明 WebView 系统辅助进程是否覆盖，不能默认全部都是应用子进程。
4. 将事件按进程身份、路径、时间和前后对照归因；不能仅凭系统目录变动认定为产品写入，也不能把无法归因的事件全部忽略。记录排除无关系统活动的依据、覆盖盲区和残留。
5. 已知坏样本未检出、事件丢失、权限不足、辅助进程覆盖不完整或归因不清时，结果标为失败 / 不确定 / 未执行，并保留原因；不得标为“未发现越界，因此便携通过”。只有声明的测试范围内证据充分时才能记录该范围通过，不外推为所有系统写入均被覆盖。

## 未覆盖范围

Windows / Linux 实机、移动真机、多引擎、复杂登录 / 新窗口、持久会话、设备预设恢复、完整批注与反馈、Agent 接入均不在本 change 已验证范围。
文档门禁与结构核对可在建档时执行，其结果另由提交记录确认，不改变上述运行项状态。
