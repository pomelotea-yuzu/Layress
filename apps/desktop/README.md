# apps/desktop/ · Tauri 桌面工作台

> 当前状态：add-tauri-page-host 任务 1.x–2.x 完成并经用户人工复验；
> **3.x（采集与原地预览）代码实现完成、机器级真机探针通过（2026-11-12），视觉 / 交互人工复验未执行**。
> 4.x–5.x 未开始。任务勾选与证据以 change 的 `tasks.md` / `evidence-index.md` 为准。

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

Rust 工具链若不在 PATH，命令前加 `PATH="$HOME/.cargo/bin:$PATH"`。

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
