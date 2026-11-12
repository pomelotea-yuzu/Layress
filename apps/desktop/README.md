# apps/desktop/ · Tauri 桌面工作台

> 当前状态：add-tauri-page-host 任务 1.x–2.x 完成并经用户人工复验；
> **3.x 全部三项经用户人工复验通过并勾选（2026-11-12）**（竞态负例经「慢采 5s」补齐后确认）。
> 用户另提出结果条压住工具栏、希望移到窗口最底下，形态待确认。4.x–5.x 未开始。
> 任务勾选与证据以 change 的 `tasks.md` / `evidence-index.md` 为准。

## 结构

```
apps/desktop/
├─ src/
│  ├─ main.ts                # 可信工具界面：导航、视口、采集、静态预览与坐标交互
│  └─ coords.ts              # 纯坐标换算（原图空间）+ 运行时往返自检
├─ index.html                # 工具条（96px）与原地静态预览层
├─ src-tauri/
│  ├─ fixtures/              # 内嵌本地样例页（含固定坐标参照块）
│  ├─ src/lib.rs             # 核心编排：双 WebView、导航 / 权限、采集命令、预览布局
│  ├─ src/capture.rs         # WKWebView 原生快照、版本化元数据、数据根、竞态状态机与单测
│  ├─ src/fixture.rs         # 127.0.0.1 随机端口样例服务（仅回环，不写盘）
│  ├─ capabilities/          # 仅 "main" 标签获 core 权限，被测页面无任何能力
│  ├─ icons/                 # 应用图标（当前为占位图）
│  └─ tauri.conf.json        # 窗口留空（双 WebView 在 setup 创建）、devUrl、CSP
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
2. 点「采集」：**工具条底部会出现黄色「采集结果条」**（贴 WebView 视口底，会盖住第二行下半部分，
   但 `pointer-events:none`，按钮仍可点），显示 `[started] 慢采已开启：… 5000 ms …`。
   这条是判断慢采是否生效的依据；没有它说明还在旧窗口，请 Ctrl+C 完整重启。
3. 在这 5 秒内任选其一：点「刷新」、点「取消」、或再点一次「采集」。
4. 预期：结果条变红，显示 `[invalidated] …` / `[cancelled] …`，最终显示
   `✗ 本次采集失败 … 已失效`，**不进入静态预览**；上次成功只被标为「旧现场」。
   结果条只由采集阶段更新，不会被加载 / 视口事件覆盖；文字过长时悬停可看完整内容（含数据根路径），
   按提示到 `<数据根>/captures/` 核对本次没有新增 PNG / JSON。
5. 取消勾选后再正常采集一次，结果条应转绿显示采集完成。

若启动时状态行显示 `⚠ 工具界面 HTML 与脚本不同步…`，说明工具层 WebView 仍是旧页面，
必须 Ctrl+C 后重新运行启动命令，不能只靠热更新。

**改过前端 HTML/JS 后必须完整重启**：`tauri.conf.json` 的 CSP `default-src 'self'` 会挡掉 Vite dev 的
HMR websocket，`@vite/client` 无法推送页面重载，工具层 WebView 会一直保留启动那一刻的 DOM。
只关窗口、或在改动前「重启过」都不算；要杀净 `pnpm tauri dev` / vite / 应用进程再起。
可用 `curl -s http://localhost:5173/ | grep capture-toast` 与磁盘 `index.html` 对照确认服务的是新版。

**工具层视口的已知限制**：Rust 给工具层的 bounds 是 `Logical(1280×96)`、窗口 scale 2，
但页面实测 `window.innerHeight=64`；`#bar` 高 96px 且 `overflow:hidden`，因此第三行状态 / 视口 / 请求文字
长期被裁不可见。采集反馈一律看贴视口底部的结果条，不要依赖第三行。
开发期可用 `LAYRESS_PROBE_TOOL=1 pnpm tauri dev` 对工具层自身截图（数据根 `tool-probe.png`）自证结果条可见。

「关闭应用窗口」这一路无法同时观察结果，退出清理留任务 4.4 单独验证。
机器侧已用 `LAYRESS_PROBE_CAPTURE=race` 在慢采窗口内自动刷新验证过「失效且 PNG 数不变」，
但那不替代人工在 GUI 上确认。

## 口径

- 首增量运行 macOS arm64；多 WebView 依赖 Cargo `unstable` 功能，平台限制见 change 的 design。
- **截图**：只取被测 WKWebView 自身的 `takeSnapshot` 画面，不整屏截图、不 DOM 重绘；
  非 macOS 平台明确报「未实现」，不静默降级。采集前关闭工具注入的视口边框。
- **元数据**：`layress.capture/v1`，PNG 与 JSON 落在数据根 `captures/`；
  页面身份、导航代号、开始 / 完成时间、视口与图片条件、缺失条件逐条标注。
- **预览**：工具层铺满窗口、被测层仅平移出可视区且尺寸不变；返回时不销毁 / 重载页面。
  矩形只存原图像素坐标，两种展示比例共用一份坐标，界面显示坐标换算自检结果。
- **数据根**：`--data-dir` / `LAYRESS_DATA_DIR` > 可执行文件旁 `.layress/`，
  启动与采集前各做一次可写检查；不可写明确失败，不回落系统用户目录。
  3.x 只落地最小解析与可见性，任务 4.1 的负例与 4.2 的写盘观测仍未执行。
- 构建缓存 `src-tauri/target/`、前端 `dist/`、运行数据根 `.layress/` 均不入库；`Cargo.lock` 入库。
- 机器 PASS 不等于人工验收：截图内容、两种比例下的坐标落点、返回后输入 / 弹窗会话保留仍需复验。
