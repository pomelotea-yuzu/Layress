# 三端打包规划（Windows / Linux / macOS）

> 本文默认以中文维护。
>
> **状态**：提案（2026-11-06）。用户已拍板四项口径（见 §一与 [`../../knowledge/decisions.md`](../../knowledge/decisions.md) D15）；
> 具体工具链、数据目录落点与 CI 矩阵仍待首个发行轮 change 的 design 定稿。
> **2026-11-08 修订**：开发机由 Windows 迁入 macOS arm64，§一「本机」标注、首验实机平台与 §八 M1 随之改为待重定；
> 三端打包承诺、包格式矩阵与验证分阶段的结论不变。
> **本文不启动开发、不装依赖、不建 workspace**（遵守 D13）；它只把"三端从一开始就规划好"这件事写成可执行依据，
> 避免日后再从单平台拆成三平台。权威性排序见 [`../../README.md`](../../README.md)：本文属 `docs/`，低于源码与 openspec。

## 一、已拍板的口径（用户 2026-11-06 决定）

| 项 | 决定 | 影响 |
|---|---|---|
| 三端范围 | **打包承诺三端**：Windows x64、Linux Ubuntu 24 x64（虚拟机）、macOS arm64（Apple Silicon MacBook，**2026-11-08 起为当前开发机**） | 架构与 CI 从第一天按三端设计；不是"以后再移植" |
| 验证节奏 | **验证仍分阶段**：首验对照实验的实机平台**待重定**（本文写于 2026-11-06，当时开发机为 Windows，原「先在 Windows 跑」已随迁机失效，见 debt）；Linux / mac 实机验证列为后续里程碑 | 三端可打包 ≠ 三端已验证；未验证端单列，不勾完成（I11） |
| mac 架构 | 只出 **arm64**（M 系）；Intel Mac 暂不覆盖 | mac 矩阵最简，CI 与实机验证成本不翻倍 |
| 桌面壳 | **先不定**（Electron / Tauri 未选）；本文只锁分发形态与平台矩阵，壳选型留接口 | 见 §四与 [`../../knowledge/debt.md`](../../knowledge/debt.md) |
| mac 签名 | **先不买** Apple Developer（$99/年）；mac 包标注未签名 + 本地绕过步骤，签名/公证留位 | 见 §五与 debt；公开分发前再定 |

**边界澄清**：三端打包承诺指"产物形态与构建矩阵覆盖三端"，**不等于**三端浏览器/全浏览器矩阵（后者仍暂缓，见路线图），
也不等于首验就在三端跑。CLI 核心逻辑天然跨平台；三端差异主要落在**浏览器二进制、原生依赖、桌面包格式、数据目录落点**四处。

## 二、两种分发形态

Layress 首版形态是 **Agent 友好的 CLI + 可读报告 + 结构化结果**（D10 / I14），桌面 UI 后置。因此打包分两条线：

| 形态 | 载体 | 跨平台性质 | 三端差异点 |
|---|---|---|---|
| **CLI（首版主交付）** | npm scoped 包 `@layress/*`（D11）；核心检测与报告完整开源（I10） | TS/JS 逻辑本身平台无关 | 浏览器二进制按平台下载；原生依赖（若有）需各平台 prebuild；数据目录落点（§六） |
| **桌面应用（后置）** | `apps/desktop`，壳未定 | 壳决定包格式与运行时 | 包格式（§四）、签名（§五）、userData/sessionData 覆盖（I12/I13） |

CLI 先行意味着：**三端打包的第一优先级是让 `@layress/cli` 在三端能装、能跑、能产出同 schema 的结果**，
桌面壳选型可以晚于 CLI 三端验证，不阻塞首版。

## 三、CLI 三端打包要点

1. **核心逻辑平台无关**：扫描 / 确认 / 缩减 / 回放（I10）用 TS strict 写，不内嵌平台分支；
   平台差异隔离到"探针通道适配"与"数据目录解析"两个边界模块，便于三端各自验证。
2. **浏览器二进制按平台获取**：探针通道候选（Playwright / CDP 直连 / Electron 远程调试，见 debt）无论选哪种，
   Chromium/Chrome 的二进制都是**平台特定**的。
   - Playwright 路线：`playwright install chromium` 按宿主平台拉对应构建；三端各自下载，不打进 npm 包（体积 + 洁净度红线，二进制不进历史）。
   - 系统 Chrome 路线：依赖用户本机已装浏览器，三端探测路径不同（Win 注册表/Program Files、Linux `/usr/bin`、mac `/Applications`），探测失败要明确报错（I4/I12 精神：不静默回落）。
   - **I12 连带坑**：Playwright 默认把浏览器缓存写到 `~/.cache/ms-playwright`（Linux/mac）或用户目录（Win）——
     必须显式覆盖到数据根之下（`PLAYWRIGHT_BROWSERS_PATH` 等），否则违反"不碰系统盘/用户目录"。这条在三端都要实测。
3. **原生依赖（若引入）**：如 better-sqlite3、node-gyp 类、esbuild/rollup 的平台二进制，
   需确认三端（含 mac arm64、Linux x64）都有 prebuild，否则 CI 要能本地编译。引入前逐个核 prebuild 覆盖，记入 design。
4. **Node 版本**：CI 现用 Node 20；三端统一一个 LTS 基线（20 或 22，发行轮定），写进 `engines` 与 README，避免三端 Node 漂移。
5. **npm 包不含平台二进制**：包内只放 JS + 脚本 + 占位；浏览器/原生二进制走安装时下载或 optionalDependencies 的平台包，
   守住"二进制不进历史"（AGENTS.md §五）。

## 四、桌面壳与包格式（壳未定，先锁矩阵）

壳选型（Electron vs Tauri）归 `apps/desktop` 首个 change 的 design，本文不预设。但**三端目标包格式先锁定**，壳选型须能满足：

| 平台 | 目标包格式 | 便携/安装口径（I12） | 备注 |
|---|---|---|---|
| Windows x64 | **便携 zip**（首选）；如需安装器则 NSIS 且**不提权、不改 PATH** | 解压即用，数据落产品目录 `data/` 或 `.layress/` | 便携 zip 最贴合 I12，优先 |
| Linux Ubuntu 24 x64 | **AppImage**（首选，便携免安装）；`.deb` 备选 | AppImage 单文件便携，数据落产品目录/用户指定 | Ubuntu 24 VM 验证；AppImage 需 FUSE 或 `--appimage-extract-and-run` |
| macOS arm64 | **`.dmg` + `.zip`**（未签名） | 数据落产品目录，**禁止** `~/Library/Application Support`（I12 明列） | 未签名绕过步骤见 §五；签名留位 |

- **Electron 路线**：electron-builder 三端目标（win zip/nsis、linux AppImage/deb、mac dmg/zip arm64）；
  `userData`/`sessionData` 必须显式覆盖到数据根（I12/I13），不用系统默认。
- **Tauri 路线**：tauri bundler 三端目标；依赖各端系统 WebView（Linux WebKitGTK 差异大，窗口/探针能力需另验证）；
  数据目录同样按 I12/I13 覆盖。
- 无论哪条路线，**桌面包不是执行必需入口**（I14）：CLI 能独立完成扫描/确认/缩减/回放，桌面只是可读报告与复核的便利层。

## 五、macOS 未签名分发与签名留位

- **现状（先不买）**：mac 包未签名、未公证。用户首次打开会被 Gatekeeper 拦。
  提供本地绕过步骤（右键打开 / `xattr -d com.apple.quarantine` / 系统设置允许），并在 README 与包内说明标注"未签名"。
- **留位**：CI 预留签名与公证步骤的位置（electron-builder 的 `afterSign` / tauri 的 signing 配置），
  证书与密钥**由用户在应用内/CI secret 配置，仓库只留占位符**（AGENTS.md §五凭据零容忍、[[feedback-secrets-user-enters]]）。
- **触发时点**：公开分发 mac 包之前再决定是否购买 Apple Developer；记入 debt，归用户定夺。
- **诚实标注**：未签名 mac 包在三端矩阵里标"可构建、未签名、实机验证待做"，不写成"已支持 mac"。

## 六、三端数据目录落点（I12 / I13）

I12 明列三端的禁止路径；Windows 用户目录与注册表、macOS `~/Library` 与 `/private/var/folders`、
Linux 的 XDG 目录（`~/.config`、`~/.local/share`、`~/.cache`）一律不默认写入。
三端统一解析口径（I13）：`--data-dir` / `LAYRESS_DATA_DIR` > 产品目录下数据根；解析结果与可写性出现在启动输出、日志、报告头部。

| 平台 | 禁止默认写入 | 产品目录数据根（默认） | 连带坑 |
|---|---|---|---|
| Windows | `%APPDATA%` `%LOCALAPPDATA%` `C:\Users\…` 注册表 | 产品目录下 `.layress/` 或 `data/` | `os.tmpdir()` 默认在 C 盘用户目录 ⇒ 显式覆盖 |
| Linux | `~/.config` `~/.local/share` `~/.cache`（便携精神） | 产品目录下 `.layress/` 或用户指定 | Playwright 缓存 `~/.cache/ms-playwright` ⇒ 覆盖；AppImage 挂载点只读 ⇒ 数据落挂载点外 |
| macOS | `~/Library/{Application Support,Caches,Preferences,Logs}`／`/private/var/folders`（I12 明列） | 产品目录（`.app` 旁或用户指定） | Chromium `sessionData`/`userData` 默认在 Library ⇒ 覆盖；`os.tmpdir()` 默认在 `/private/var/folders` ⇒ 同样覆盖 |

- **写不进就不写**（I12）：目标目录不可写时明确报错并要求指定，**禁止静默回落到用户/系统目录**——三端一致。
- **数据跟着产品放**：拷贝整个产品目录即迁移，删除即无残留；这条在三端都要实测验证，尤其 AppImage（只读挂载）与 mac `.app`（签名后目录结构）。
- 具体落点与 `os.tmpdir()`/`userData`/`sessionData` 三处覆盖的实测，归发行轮 change 的 evidence-index，本文只锁口径。

## 七、CI 构建矩阵

现 CI（`.github/workflows/ci.yml` + `.workflow/ci.yml`）只在 ubuntu 跑门禁链（洁净度/链接/双语/自测），**尚无三端构建**。
workspace 骨架落地后，构建矩阵分两层：

| 层 | 跑什么 | runner | 时点 |
|---|---|---|---|
| 门禁层（现有） | `check-repo.mjs --all` → 日后 `pnpm check:ci` | ubuntu（Gitee 侧只 linux） | 已就位，骨架落地后接 `pnpm check:ci` |
| 三端构建/验证层（新增） | 三端装依赖 + typecheck + test + 打包 + 冒烟 | GitHub Actions：`windows-latest` / `ubuntu-24.04` / `macos-latest`（arm64） | 首个发行轮 change |

- **Gitee 侧限制**：Gitee Go 只有 linux runner，无法跑 win/mac 构建。口径：**门禁链两侧一致**（D09），
  三端构建矩阵以 GitHub Actions 为主；Gitee 侧只跑门禁 + linux 构建，win/mac 构建结果不在 Gitee 重复。这条要写进 workflow，避免"两侧命令不一致"。
- **mac runner 架构**：`macos-latest` 现为 arm64，与用户 MacBook（M 系）一致；若需 x64 另选 `macos-13`，但当前只承诺 arm64。
- **浏览器/原生二进制在 CI 的获取**：三端 runner 各自下载对应平台构建，缓存按平台 key 隔离；不提交二进制（洁净度）。
- **实机验证 ≠ CI 冒烟**：CI 能构建+冒烟不等于探针/命中测试在该端正确（I11）。三端实机验证（尤其 Linux VM 的窗口/DPR、mac 的命中测试）单列里程碑，见 §八。

## 八、分阶段验证里程碑（打包承诺 ≠ 已验证）

| 里程碑 | 内容 | 进入下一步的证据 | 状态 |
|---|---|---|---|
| M0 三端可构建 | CI 矩阵三端装依赖 + typecheck + test + 打包通过 | 三端产物可生成，冒烟启动不崩 | 未执行（待骨架 + 发行轮 change） |
| M1 首验实机验证（平台待重定） | 首验对照实验在重定后的平台跑（沿用首验提案 S0–S4） | 探针/命中/存储路径约束有实测依据 | 未执行（首验提案待批准、平台待重定） |
| M2 Linux 实机验证 | Ubuntu 24 VM 跑探针与数据目录覆盖 | 窗口/DPR/输入法时序、AppImage 数据落点实测；Playwright 缓存覆盖验证 | 未执行 |
| M3 macOS 实机验证 | MacBook（arm64，现为开发机）跑探针与未签名绕过 | 命中测试、Library 路径覆盖、未签名打开流程实测 | 未执行 |

- **顺序**：M0 可与 M1 并行准备；M1 先行（平台待重定），M2/M3 在 CLI 核心稳定后展开，不阻塞首版。
- **诚实**：每个里程碑的未执行项单列，机器构建通过不写成实机验证通过（I11）；环境阻塞（如 VM 无 GPU、mac 未签名）如实标注。

## 九、未决项（进 debt，归谁定夺）

| 事项 | 现状 | 归谁 | 必须定的时点 |
|---|---|---|---|
| 桌面壳选型 | Electron / Tauri 未选；本文只锁包格式矩阵 | `apps/desktop` 首个 change 的 design | 写桌面代码之前 |
| mac 签名/公证 | 先不买，未签名 + 绕过步骤；CI 留位 | 用户 | 公开分发 mac 包之前 |
| Linux 数据目录口径 | XDG 目录按便携精神不默认写入，具体落点与 AppImage 只读挂载处理未实测 | 发行轮 change design | M2 之前 |
| 原生依赖 prebuild 覆盖 | 未引入依赖；引入时逐个核三端（含 mac arm64）prebuild | 引入依赖的 change | 引入该依赖之前 |
| Node LTS 基线 | CI 现用 20；三端统一基线待定（20/22） | 发行轮 change | 首个 tag 之前 |

## 十、未执行与边界提醒

- 本文是**规划提案**，未建 workspace、未装依赖、未写代码、未跑三端构建、未做实机验证（遵守 D13 与调研阶段口径）。
- 三端打包承诺已锁，但**首验实机平台待重定**（原 Windows 口径随 2026-11-08 迁机失效，见 debt）；Linux/mac 实机验证是后续里程碑，未执行项不勾完成（I11）。
- 壳选型、mac 签名、Linux 数据落点等未决项已进 debt，不在本文强行定死。
- 下一步若用户批准进入骨架，按 [`../../development/workflow.md`](../../development/workflow.md) 建 workspace 与 `pnpm check:ci`，
  三端构建矩阵在首个发行轮 change 落地；本文随之从提案晋升为稳定依据（命名去日期前缀，见 [`../../README.md`](../../README.md) 晋升规则）。