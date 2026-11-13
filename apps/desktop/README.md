# apps/desktop/ · Tauri 桌面工作台

> 当前状态：add-tauri-page-host 任务 1.x–2.x 完成并经用户人工复验；
> **3.x 全部三项经用户人工复验通过并勾选（2026-11-12）**（竞态负例经「慢采 5s」补齐后确认）。
> 结果条已按用户选择移到**窗口最底部的独立状态栏 WebView**（不占工具条、不遮按钮）。4.x–5.x 未开始。
> 任务勾选与证据以 change 的 `tasks.md` / `evidence-index.md` 为准。

## 结构

```
apps/desktop/
├─ src/
│  ├─ main.ts                # 可信工具界面：导航、视口、采集、静态预览与坐标交互
│  └─ coords.ts              # 纯坐标换算（原图空间）+ 运行时往返自检
├─ index.html                # 工具条（96px）与原地静态预览层
├─ status.html               # 窗口底部独立状态栏（无脚本、无能力；Vite 双入口之一）
├─ vite.config.ts            # 双入口构建：main=index.html、status=status.html
├─ src-tauri/
│  ├─ fixtures/              # 内嵌本地样例页（含固定坐标参照块）
│  ├─ src/lib.rs             # 核心编排：三 WebView 布局、导航 / 权限、采集命令、预览布局
│  ├─ src/capture.rs         # WKWebView 原生快照、版本化元数据、数据根、竞态状态机与单测
│  ├─ src/fixture.rs         # 127.0.0.1 随机端口样例服务（仅回环，不写盘）
│  ├─ capabilities/          # 仅 "main" 标签获 core 权限；"page" 与 "status" 无任何能力
│  ├─ icons/                 # 应用图标（当前为占位图）
│  └─ tauri.conf.json        # 窗口留空（三 WebView 在 setup 创建）、devUrl、CSP
└─ package.json              # 前端依赖（@tauri-apps/api / cli 2.12.4、vite 7.3.8、ts 5.9.3）
```

## 本地命令（在 apps/desktop/ 下执行）

```bash
pnpm install          # 仓库根执行一次即可（workspace）
pnpm build            # tsc --noEmit + vite build
pnpm tauri dev        # 开发运行（先起 Vite dev server，再编译 Rust）
cargo check           # 在 src-tauri/ 下
cargo test --lib      # 采集状态机 / 哨兵键名 / PNG 与 base64 纯逻辑单测（6 项）

# 开发期无人值守探针：仅 debug 构建生效，自动打开样例页并采集一次，结果打到 stderr
LAYRESS_PROBE_CAPTURE=1 pnpm tauri dev
```

**真终端启动（重要）**：WorkBuddy 会话用内置 Node 运行时（`pnpm` 是 corepack shim），
真终端的 PATH 通常既没有 `pnpm` 也没有 `~/.cargo/bin`；系统自带 node / npm 不够用。
在任意目录直接粘贴这一行（已用干净 PATH 环境实跑验证：node 22.22.2 / pnpm 12.18.0 /
cargo 1.99.1 / tauri-cli 2.12.4）：

```bash
cd ~/Developer/project/layress/apps/desktop \
  && PATH="$HOME/.workbuddy/binaries/node/versions/22.22.2-6/bin:$HOME/.cargo/bin:$PATH" pnpm tauri dev
```

不要只写 `cd apps/desktop`——那条命令默认当前目录已经是仓库根。

## 任务 3.2 人工负例复验（开发期）

真实采集约 43 ms，直接点「采集」后人工来不及触发导航。**代码改动后先重启应用**（Ctrl+C 再跑上面的命令），然后：

1. 打开样例页，勾上工具条第二行的「慢采 5s」。
2. 点「采集」：**窗口最底部的独立状态栏**变成浅黄色，显示 `[started] 慢采已开启：… 5000 ms …`。
   状态栏与工具栏完全分离，不遮挡任何按钮；没有它说明还在旧窗口，请 Ctrl+C 完整重启。
3. 在这 5 秒内任选其一：点「刷新」、点「取消」、或再点一次「采集」。
4. 预期：状态栏变红，显示 `[invalidated] …` / `[cancelled] …`，最终显示
   `✗ 本次采集失败 … 已失效`，**不进入静态预览**；上次成功只被标为「旧现场」。
   状态栏只由 `status_bar_set` 命令更新，不会被加载 / 视口事件覆盖；
   按提示到 `<数据根>/captures/` 核对本次没有新增 PNG / JSON。
5. 取消勾选后再正常采集一次，状态栏应转绿显示采集完成。

若启动时状态行显示 `⚠ 工具界面 HTML 与脚本不同步…`，说明工具层 WebView 仍是旧页面，
必须 Ctrl+C 后重新运行启动命令，不能只靠热更新。

**改过前端 HTML/JS 后必须完整重启**：`tauri.conf.json` 的 CSP `default-src 'self'` 会挡掉 Vite dev 的
HMR websocket，`@vite/client` 无法推送页面重载，工具层 WebView 会一直保留启动那一刻的 DOM。
只关窗口、或在改动前「重启过」都不算；要杀净 `pnpm tauri dev` / vite / 应用进程再起。
可用 `curl -s http://localhost:5173/status.html | grep 'id="bar"'` 与磁盘 `status.html` 对照确认服务的是新版。

**工具层视口的已知限制**：Rust 给工具层的 bounds 是 `Logical(1280×96)`、窗口 scale 2，
但页面实测 `window.innerHeight=64`；`#bar` 高 96px 且 `overflow:hidden`，因此第三行状态 / 视口 / 请求文字
长期被裁不可见。这正是把结果条独立成第三个 WebView 的原因；采集反馈一律看窗口底部状态栏，不要依赖第三行。
开发期可用 `LAYRESS_PROBE_TOOL=1 pnpm tauri dev` 对状态栏自身截图（数据根 `status-probe.png`）自证可见。

「关闭应用窗口」这一路无法同时观察结果，退出清理留任务 4.4 单独验证。
机器侧已用 `LAYRESS_PROBE_CAPTURE=race` 在慢采窗口内自动刷新验证过「失效且 PNG 数不变」，
但那不替代人工在 GUI 上确认。

## 任务 2.2 人工负例复验（开发期，2026-11-13 用户已复验通过）

**先完整重启**：本轮改过 `main.ts` / `status.html`，CSP 会挡掉 Vite HMR，旧窗口仍是旧 DOM。
Ctrl+C 杀净 `pnpm tauri dev` / vite / 应用进程后，在任意目录粘贴：

```bash
cd ~/Developer/project/layress/apps/desktop \
  && PATH="$HOME/.workbuddy/binaries/node/versions/22.22.2-6/bin:$HOME/.cargo/bin:$PATH" pnpm tauri dev
```

先点「打开样例页」建立基线（记下底部状态栏的 `✓ 页面加载完成：…`、页面内容与地址栏），然后逐条输入并点「前往」：

| # | 地址栏输入 | 预期（看**窗口最底部独立状态栏**） |
|---|---|---|
| 1 | `javascript:alert(1)` | 红条 `✗ 已拒绝：… 不支持的 scheme`，无弹窗、无新窗口 |
| 2 | `file:///etc/passwd` | 红条 `✗ 已拒绝：… 不支持的 scheme`，本地文件不打开 |
| 3 | `example.com` | 红条 `✗ 已拒绝：… 地址无法解析 …`（无 scheme） |
| 4 | `http://` | 红条 `… 地址缺少主机名` |
| 5 | `http:///path` | 红条 `… 地址缺少主机名`（本轮新修的漏口） |
| 6 | `http://127.0.0.1:1/` | 红条 `… 目标不可达 …`，页面与地址栏保持不变 |
| 7 | `http://192.0.2.1/` | 数秒内红条 `… 目标不可达 …`，期间黄条 `正在校验地址` |
| 8 | 再点「打开样例页」 | 绿条 `✓ 页面加载完成：…`，地址栏恢复样例 URL，页面仍可点击 / 输入 |

每条都要核对三件事：**状态栏是红的、被测页面仍是原来那个（不跳转、不空白）、地址栏不残留刚输入的无效地址**。
第 8 条确认失败后正常导航没有被破坏。若红条始终没出现，先看第 7 条有没有黄条：没有就是旧窗口，必须完整重启。

**复验结论（2026-11-13，用户原话「第二个是 已拒绝：地址缺少主机名，其他没有问题」）**：8 条中 7 条符合预期，三条核对点与第 8 条正常导航均通过。第 2 条当时显示「地址缺少主机名」——错因归因错误，不是拒绝行为失效：本轮新增的 authority 前置检查抢在 scheme 白名单之前，把 `file:///etc/passwd` 按路径形状判成缺主机名。已把该检查限定为仅对 http(s) 生效，修后为「不支持的 scheme」，并加单测断言 file / ftp / data 的错因必须含 scheme、不得含主机名。

**复验补齐（2026-11-13 15:10）**：用户随后确认「新文案是对的了」，即修复后 `file:///etc/passwd` 显示「不支持的 scheme」已在 GUI 上人工复验通过，原单列项销账。E03 判据「明确拒绝、不误报加载成功、当前页面身份不变」由用户人工确认 ⇒ tasks 2.2 勾选。机器单测只覆盖校验与可达性的纯逻辑，不替代 GUI 复核（I11）。

## 任务 4.x 人工复验（开发期，2026-11-13 待用户执行）

**先完整重启**：本轮改过 Rust 与样例页。Ctrl+C 杀净 `pnpm tauri dev` / vite / 应用进程后按上面的启动命令重起。

### 4.1 数据根可见与不可写负例

1. 正常启动：**窗口最底部状态栏**应显示 `数据根：…/target/debug/.layress（来源：可执行文件目录旁 .layress/，可写）`；采集一次，产物落在该根 `captures/`。
2. 自定义可写根：`--data-dir` 指向任意可写目录：

   ```bash
   cd ~/Developer/project/layress/apps/desktop \
     && PATH="$HOME/.workbuddy/binaries/node/versions/22.22.2-6/bin:$HOME/.cargo/bin:$PATH" \
        pnpm tauri dev -- -- --data-dir="$HOME/Developer/project/layress/artifacts/custom-root"
   ```

   ⚠ 这里必须写**两个 `--`**：`tauri dev` 的第一个 `--` 之后是 runner(cargo) 参数，第二个 `--` 之后才传给应用
   （见 `pnpm tauri dev --help` 的 Arguments 段）。不想依赖参数透传可直接用第 4 步的环境变量形式。

   预期状态栏显示该目录与来源「命令行 --data-dir」，采集产物落在其中。
3. 不可写负例：把上面路径换成只读位置（如 `/System/Library/layress-probe`）。预期：启动日志 `[layress] 数据根不可写，采集将明确失败：…`、状态栏**红条**显示不可写与原因；点「采集」明确失败，**不**在 `~/Library/...` 或默认 `.layress/` 建兜底目录，也不改写目标路径。
4. 环境变量形式：`LAYRESS_DATA_DIR=… pnpm tauri dev` 应显示来源「环境变量」；命令行与环境变量同时给时命令行优先。

### 4.3 原生访问负例

打开样例页，逐个点「fetch 本地文件」/「invoke 文件读取命令」/「invoke 进程命令」/「invoke 采集命令」，每次看被测页 `#native-out`：全部应显示「被拒绝 / 原生桥不存在」，不能出现「意外成功」。Web 页面没有进程 / 文件 API，这四项以调用不存在的原生命令为代理，不声称它是完整的进程访问验证。

### 4.4 退出清理与重启

1. 打开样例页，勾「慢采 5s」，点「采集」，在 5 秒内关闭应用窗口。
2. 预期：本次不进入静态预览、`<数据根>/captures/` 没有本次新增 PNG / JSON；数据根内上次成功的截图仍在；数据根外文件没有被删除。
3. 重新启动，应用能正常打开、样例页可用，状态栏再次显示数据根。

**4.2 不在本轮复验范围**：便携的载体是打包后的产品目录，开发态没有该对象；真实写盘观测挂到首次打包之后，见 [debt](../../docs/knowledge/debt.md)。

## 口径

- 首增量运行 macOS arm64；多 WebView 依赖 Cargo `unstable` 功能，平台限制见 change 的 design。
- **截图**：只取被测 WKWebView 自身的 `takeSnapshot` 画面，不整屏截图、不 DOM 重绘；
  非 macOS 平台明确报「未实现」，不静默降级。采集前关闭工具注入的视口边框。
- **元数据**：`layress.capture/v1`，PNG 与 JSON 落在数据根 `captures/`；
  页面身份、导航代号、开始 / 完成时间、视口与图片条件、缺失条件逐条标注。
- **预览**：工具层铺满窗口、被测层仅平移出可视区且尺寸不变；返回时不销毁 / 重载页面。
  矩形只存原图像素坐标，两种展示比例共用一份坐标，界面显示坐标换算自检结果。
- **状态栏**：窗口最底部独立 WebView（label `status`，逻辑高 42px），无 capability、页面无脚本；
  文本与颜色由 `status_bar_set` 命令注入（灰=就绪 / 提示、黄=采集进行中、绿=成功、红=失效）。
  页面区高度统一由 `stage_height()` 计算，工具条与状态栏都不静默侵占被测视口。
- **数据根**：`--data-dir` / `LAYRESS_DATA_DIR` > 可执行文件旁 `.layress/`，
  启动与采集前各做一次可写检查；不可写明确失败，不回落系统用户目录。
  3.x 只落地最小解析与可见性，任务 4.1 的负例与 4.2 的写盘观测仍未执行。
- 构建缓存 `src-tauri/target/`、前端 `dist/`、运行数据根 `.layress/` 均不入库；`Cargo.lock` 入库。
- 机器 PASS 不等于人工验收：3.x 的截图内容、两种比例下的坐标落点、返回后输入与竞态负例已由用户复验通过；
  弹窗会话单独触发、2.2 无效地址负例与 4.x 仍未执行。
