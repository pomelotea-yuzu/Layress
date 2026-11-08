# debt.md · 未决口径与欠账

> 本文默认以中文维护。

> 这里只放**还没有结论**的事。已有结论的进 [`decisions.md`](decisions.md)，已知的坑进
> [`engineering.md`](engineering.md) / [`runtime-probe.md`](runtime-probe.md)。
> 每条必须写"归谁定夺"和"什么时候必须定"，否则会一直挂着。

| 事项 | 现状 | 归谁定夺 | 必须定下来的时点 |
|---|---|---|---|
| 首个验证方案批准 | 已成文为**提案**：[2026-11-03-first-validation-plan](../engineering/plans/2026-11-03-first-validation-plan.md) —— S0 探针对照实验 → S1 数据集 → S2 三组对照 → S3 指标与人工复核 → S4 go/shrink；时间盒建议 6 周 / 90h；§四 列出需用户拍板的六件事。**未批准** | 用户 | 进第一个 change 之前 |
| 探针技术栈选型 | **不预设结论**：由方案 S0 对照实验量出来（R1 Playwright + 系统 Chrome vs R2 裸 CDP；3 个已知答案样本 × 7 项测量 × 6 次重复），判据预写在方案 §二；其中"系统用户目录零增量"（I12，按实测平台列具体路径）是硬判据之一。**未选** | 第一个 change 的 design.md | 写第一行检测代码之前 |
| 几何阈值与 DPR 策略 | 遮挡/裁切的判定阈值、1px 描边取整、DPR=1/2 与缩放 200% 的档位划分**未实测**（前置项目仅证明"默认窗口宽度骑断点线"这一坑存在） | 首验数据 | 出第一份精确率数字之前 |
| 远程视觉模型是否引入 | 调研指出"结合 AI 看图"不构成差异；引入则要解决 `I8` 的数据流与成本声明 | 用户 | 确认层设计定稿时 |
| 证据导出的脱敏 | 截图与页面内容可能含姓名、地址、内部路径；默认留本地已定（`I8`），但**导出/分享**形态下的脱敏规则未定 | 用户 + 报告模块 design | 支持导出功能之前 |
| 数据集样本的再分发授权 | 采集真实开源项目的页面样本用于公开数据集，再分发许可与署名口径**未核** | 用户 | 数据集进仓库之前 |
| `layress.com` 注册 | whois 显示未注册（约 $14.48，2026-11-03 检索），占住品牌入口 | 用户 | 公开宣传之前 |
| npm org `@layress` | 需要 npm organization；与 `layrr`/`layerr`/`layui` 的区分说明要写进 README | 用户 | 首次发包之前 |
| `docs/guide/` 内容 | 快速开始指南要等 CLI 真能跑才写，否则 README 里只能写"尚未发布"（这是欠账，不是目标形态） | 首个可交付版本 | 首个 tag 之前 |
| 第三方声明与依赖许可核查 | 前置项目用 `docs/development/third-party-notices.md` + 许可证清单脚本；Layress 引入依赖后需要同等流程 | 首个发行轮 | 首次发行之前 |
| 桌面壳选型（D15） | Electron / Tauri **未选**；[三端打包规划](../engineering/plans/2026-11-06-tri-platform-packaging-plan.md) §四只锁三端包格式矩阵（Win 便携 zip、Linux AppImage、mac arm64 dmg/zip），壳选型留接口 | `apps/desktop` 首个 change 的 design | 写桌面代码之前 |
| macOS 签名 / 公证（D15） | 先不买 Apple Developer；mac 包未签名 + 本地绕过步骤，CI 预留签名/公证位，证书密钥由用户配置仓库留占位 | 用户 | 公开分发 mac 包之前 |
| Linux 数据目录落点（D15） | XDG 目录（`~/.config`/`~/.local/share`/`~/.cache`）按便携精神不默认写入；AppImage 只读挂载的数据落点、Playwright 缓存覆盖**未实测** | 发行轮 change design | M2 Linux 实机验证之前 |
| 原生依赖三端 prebuild 覆盖（D15） | 未引入依赖；引入时逐个核 Win x64 / Linux x64 / mac arm64 的 prebuild，缺则 CI 需本地编译 | 引入该依赖的 change | 引入该依赖之前 |
| Node LTS 基线（D15） | CI 现用 Node 20；三端统一基线待定（20 / 22），写进 `engines` 与 README | 发行轮 change | 首个 tag 之前 |
| 首验实机平台口径（D16） | 2026-11-08 开发机迁入 macOS arm64，原「首验先在 Windows + Chromium 跑」失效；**未重定**，不默认改成 mac，也不假装 Windows 仍成立 | 用户 + 首验方案修订 | 首验方案批准、进第一个 change 之前 |
| 痛闸门反证后的去留（D17 前提） | 2026-11-08 B 轮自主任务对照中，执行端如实报告遮挡、拒绝"假点击"并找到真实用户路径，**未出现假绿**；按调研 §九 属反证。D17「取证与最小切片并行」的推翻条件只覆盖"机制分叉被反证"，未覆盖"机制成立但痛点被反证"，前提被削弱 | 用户 | 决定是否进第 2 步 / 收缩范围之前 |

## 已知的"不打算现在做"

写出来是为了防止下个接手人以为是遗漏：

- 审美评分、自动修复、任意应用全自动探索、最小 HTML/CSS 自动抽取、全平台全浏览器支持、
  大规模多语言与字体矩阵 —— 调研 §7.2 明确列为**暂缓**。
- `packages/*` 与 `apps/desktop` 目前只有归属说明，没有代码，也没有 `package.json`。
  这是本轮口径（D13），不是半成品。
