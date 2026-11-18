# apps/desktop/ · Tauri 桌面工作台

> 当前状态：**首个 change 已于 2026-11-14 归档**（1.x–4.x 经人工复验，4.2 按 D30 移出归发行轮；
> 5.1 门禁接入；5.2 经四轮人工复验闭环；5.3 首次本机 release 构建登记限制；5.4 合入主规格）。
> **第二个 change [add-project-device-profiles](../../openspec/changes/add-project-device-profiles/tasks.md) 实施中**：
> 0.1 / 1.x / 2.x / 3.1 / 3.2 / 3.4 / 4.1 已勾选，剩 3.3 / 4.2 / 4.3 共 3 项；活动规格见该 change 两份 delta，
> 勾选与证据以其 [tasks](../../openspec/changes/add-project-device-profiles/tasks.md) /
> [evidence-index](../../openspec/changes/add-project-device-profiles/evidence-index.md) 为准（归档后以 `openspec/specs/` 为准）。
> 结果条为**窗口最底部的独立状态栏 WebView**；工具层反馈全收敛到它（I15）。
> 首个 change 的勾选与证据以[归档 change](../../openspec/changes/archive/2026-11-14-add-tauri-page-host/tasks.md) 为准。

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
cargo test --lib      # 采集 / 配置模型 / 配置存储 / 命令编排共 55 项（含真实子进程双实例与中断）

# 开发期无人值守探针：仅 debug 构建生效，自动打开样例页并采集一次，结果打到 stderr
LAYRESS_PROBE_CAPTURE=1 pnpm tauri dev

# 任务 4.1 工具层源码指纹：改前端源码后取新指纹，连同递增的徽标回填 check-build.mjs 的 EXPECTED_TOOL_SOURCE
node scripts/check-build.mjs --print-fingerprint   # 在仓库根执行
```

**配置记忆（第二个 change，1.x / 2.x / 3.x / 4.1 已落地）**：`src/project_config.rs` 提供配置模型 / 校验 /
8 个稳定错误码；`src/config_store.rs` 提供 `<data-root>/config/projects.json` 的
独占写锁（`std::fs::File::try_lock`，MSRV 1.89，见 D33）、进程内串行写队列、同目录临时文件 + fsync + 原子替换、
revision 过期拒绝、备份重建（10 份上限）与分层错误码；真实子进程双实例 / 中断测试在 `cargo test --lib` 覆盖内。
`lib.rs` 接入启动恢复 / 项目切换 / 三元身份绑定 / 未保存重试 / 预览仲裁 / 退出顺序等命令；
工具层设置面板支持创建 / 改名 / 视口 / 起始地址 / 最近地址 / 切换 / 重建。
**版本自证（I16）**：`check-build.mjs --all` 除比徽标相等外，还按确定源码清单计算内容指纹，
改业务源码忘更新徽标 / 指纹、`src/` 新增模块未纳入清单均判红；`--selftest` 含 5 条指纹反例。
**仍属本 change 未勾项**：3.3 写盘失败的「已应用未保存」GUI 路径、4.2 全量人工复验、4.3 归档合入主规格。

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
- **项目配置**：`layress.project-config/v1`，落在数据根 `config/projects.json`（`revision` / 项目数组 /
  活动引用）；同目录 `projects.lock` + 临时文件 + 最多 10 份 `projects.backup.*.json` 原始字节备份。
  项目 ID 稳定、名称不作主键；`lastLoadedUrl` 是主文档完成加载的实际地址，不代表业务成功。
  SPA history / hash 变化不触发主文档加载，首版不追踪该限制，编辑面板明示。
- **预览**：工具层铺满窗口、被测层仅平移出可视区且尺寸不变；返回时不销毁 / 重载页面。
  矩形只存原图像素坐标，两种展示比例共用一份坐标，界面显示坐标换算自检结果。
- **状态栏**：窗口最底部独立 WebView（label `status`，逻辑高 42px），无 capability、页面无脚本；
  文本与颜色由 `status_bar_set` 命令注入（灰=就绪 / 提示、黄=采集进行中、绿=成功、红=失效）。
  页面区高度统一由 `stage_height()` 计算，工具条与状态栏都不静默侵占被测视口。
- **数据根（I12 / I13）**：`--data-dir` / `LAYRESS_DATA_DIR` > 可执行文件旁 `.layress/`，
  启动与采集前各做一次可写检查；不可写明确失败，不回落系统用户目录。两个显式来源的有效值若为
  相对路径（含 `~` / `./` / `../`）一律在解析层以 `DATA_ROOT_RELATIVE_UNSUPPORTED` 拒绝、不随启动工作目录漂移，
  CLI 有效绝对值覆盖 env 相对值不误拒；解析失败无根时底部状态栏就绪后红条可见（1.4 已实机复验）。
  首个 change 的不可写负例与不回落核对已复验；打包产物与 WebView 全进程写盘观测按 D30 归发行轮。
- **工具层版本自证（I16）**：窗口徽标为当前 `main.ts` 写入值，改前端源码必须递增 `BUILD_TAG` 与
  `index.html[data-build]` 并同步 `check-build.mjs` 的指纹登记；徽标为「加载中」或缺省时 GUI 结论作废。
- 构建缓存 `src-tauri/target/`、前端 `dist/`、运行数据根 `.layress/` 均不入库；`Cargo.lock` 入库。
- 机器 PASS 不等于人工验收：首个 change 的 3.x 截图内容 / 两种比例坐标落点 / 返回后输入与竞态、2.2 与
  4.1 / 4.3 / 4.4 GUI 负例已复验；第二个 change 的 0.1 / 1.x / 2.x / 3.1 / 3.2 / 3.4 已由用户实机勾选，
  **3.3 写盘失败路径与 4.2 全量复验待用户验收**；弹窗会话单独触发、D30 打包与跨平台仍未执行 / 单列。

