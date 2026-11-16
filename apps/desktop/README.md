# apps/desktop/ · Tauri 桌面工作台

> 当前状态：**change 已全部完成并于 2026-11-14 归档**（1.x–4.x 经用户人工复验，4.2 按 D30 移出归发行轮；
> 5.1 门禁接入并登记 Gitee CI 首次失败回显；5.2 经四轮人工复验闭环，见文末「判据回填」；5.3 首次本机 release
> 构建登记体积 / WebView 前置 / 限制；5.4 归档并合入主规格 [`openspec/specs/desktop-page-host/`](../../openspec/specs/desktop-page-host/spec.md)）。
> 结果条为**窗口最底部的独立状态栏 WebView**；2026-11-14 本轮把工具层剩余反馈全部收敛到它（I15）。
> 任务勾选与证据以[归档 change](../../openspec/changes/archive/2026-11-14-add-tauri-page-host/tasks.md) 为准。

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
cargo test --lib      # 采集 / 配置模型 / 配置存储（含真实子进程双实例与中断）共 41 项

# 开发期无人值守探针：仅 debug 构建生效，自动打开样例页并采集一次，结果打到 stderr
LAYRESS_PROBE_CAPTURE=1 pnpm tauri dev
```

**任务 1.2 / 1.3 配置模型与存储（2026-11-16 收口勾选）**：`src/project_config.rs` 提供配置模型 / 校验 /
8 个稳定错误码；`src/config_store.rs` 提供 `<data-root>/config/projects.json` 的
独占写锁（`std::fs::File::try_lock`，MSRV 1.89，见 D33）、进程内串行写队列、同目录临时文件 + fsync + 原子替换、
revision 过期拒绝与分层错误码；真实子进程的双实例 / 中断测试同样在 `cargo test --lib` 覆盖范围内。
1.1 已接入启动与 Tauri 命令（`config_state` / `config_create_project` / `config_rename_project`）；
**仍属后续任务**：备份重建与 10 份上限（1.5）、启动错误红条与相对根拒绝（1.4）、
视口与地址 GUI（2.x）、重取锁编排（2.x/3.x）——模块内未接线 API 均以窄范围 `allow(dead_code)` 注明归属，接线时删除。

**真终端启动（重要）**：WorkBuddy 会话用内置 Node 运行时（`pnpm` 是 corepack shim），
真终端的 PATH 通常既没有 `pnpm` 也没有 `~/.cargo/bin`；系统自带 node / npm 不够用。
在任意目录直接粘贴这一行（已用干净 PATH 环境实跑验证：node 22.22.2 / pnpm 12.18.0 /
cargo 1.99.1 / tauri-cli 2.12.4）：

```bash
cd ~/Developer/project/layress/apps/desktop \
  && PATH="$HOME/.workbuddy/binaries/node/versions/22.22.2-6/bin:$HOME/.cargo/bin:$PATH" pnpm tauri dev
```

不要只写 `cd apps/desktop`——那条命令默认当前目录已经是仓库根。

## 人工复验清单（已归档）

历次任务的复验步骤与判据（3.2 / 2.2 / 4.x / 5.2 / 0.1 / 1.1 / 1.4 / 1.5）已归档至
[`docs/reviews/checklists/`](../../docs/reviews/checklists/README.md)；**2026-11-16 起新复验步骤直接在对话中提供**，不再写入本 README，结论记入对应 change 的 evidence-index。

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
  4.1 已人工复验通过（含不可写负例与不回落核对）；4.2 已按 D30 移出本 change，归首次打包后的发行轮。
- 构建缓存 `src-tauri/target/`、前端 `dist/`、运行数据根 `.layress/` 均不入库；`Cargo.lock` 入库。
- 机器 PASS 不等于人工验收：3.x 的截图内容、两种比例下的坐标落点、返回后输入与竞态负例已由用户复验通过；
  弹窗会话单独触发仍未执行；2.2 与 4.1 / 4.3 / 4.4 的 GUI 负例已于 2026-11-13 复验通过；4.2 已移出范围（D30）。

