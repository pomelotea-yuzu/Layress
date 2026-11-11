# apps/desktop/ · Tauri 桌面工作台

> 当前状态：add-tauri-page-host 首增量实施中（任务 1.x 完成最小骨架，页面承载在 2.x+ 实现）。

## 结构

```
apps/desktop/
├─ src/                  # 可信工具界面（TypeScript strict，Vite 构建；暂不引入 UI 框架）
├─ src-tauri/            # Rust 原生层（tauri 2.12.5，含 unstable 多 WebView 功能）
│  ├─ capabilities/      # Tauri v2 能力声明：仅 "main" 标签获 core 权限，被测页面 WebView 拿不到
│  ├─ icons/             # 应用图标（当前为占位图，正式设计后替换）
│  └─ tauri.conf.json    # 窗口、devUrl（localhost:5173）、CSP 配置
└─ package.json          # 前端依赖（@tauri-apps/api 2.12.4 / cli 2.12.4 / vite 7.3.8 / ts 5.9.3）
```

## 本地命令（在 apps/desktop/ 下执行）

```bash
pnpm install          # 仓库根执行一次即可（workspace）
pnpm tauri dev        # 开发运行（先起 Vite dev server，再编译 Rust）
pnpm build            # tsc --noEmit + vite build（类型检查 + 前端产物）
cargo check           # Rust 编译检查（在 src-tauri/ 下，或 pnpm tauri dev 触发）
```

## 口径

- 首增量运行 macOS arm64；多 WebView 依赖 Cargo `unstable` 功能，平台限制见 change 的 design。
- 产品数据与临时文件遵守便携优先承诺（AGENTS.md §七）：写产品目录 `.layress/` 或用户显式指定目录，
  不落系统用户目录；数据根解析与可写性在任务 4.x 实现并可见。
- 构建缓存 `src-tauri/target/` 与前端 `dist/` 均不入库；`Cargo.lock` 入库锁定依赖。
