<div align="center">

# 🧪 Layress

<p><a href="README.en.md">English</a> | 简体中文</p>

**界面压力测试工具 · UI Stress Testing Tool**

<p>
  <a href="LICENSE"><img src="https://img.shields.io/badge/License-MIT-blue.svg" alt="License: MIT"/></a>
  <img src="https://img.shields.io/badge/Release-none-lightgrey.svg" alt="尚无发行版"/>
  <img src="https://img.shields.io/badge/%E4%B8%8D%E7%A2%B0%20C%20%E7%9B%98-portable-brightgreen.svg" alt="便携优先，不碰 C 盘"/>
</p>

</div>

> **当前状态：调研与立项完成，尚无代码、尚无发行版。**
> 本页描述的是**目标形态**。做到哪一步了请看 [`AGENTS.md`](AGENTS.md) §二 状态卡与 `git log`。

## 🧭 这是什么

把运行中的 Web／Electron 应用置于不同窗口尺寸、内容长度、语言和字体条件下，寻找界面在什么情况下失效，
并**保留触发条件**。

Layress 关注的不是"和上一版比哪里变了"，而是：

> 在明确的界面状态下主动施压，确认**关键操作**发生了什么损坏，并把触发条件缩减成可重复回放的证据。

四类工作不能混为一谈，本产品做后两项：

| 工作 | 核心问题 | 结果的含义 |
|---|---|---|
| 视觉回归 | 与已认可的版本相比哪里变了？ | 差异可能是正常更新，也可能是回归 |
| 几何异常检测 | 哪些元素溢出、重叠、裁切或换行？ | 产生**候选**，尚不能直接证明是缺陷 |
| 主动压力测试 | 改变哪些条件会让界面出问题？ | 扩大输入与状态覆盖范围 |
| 缺陷确认与复现缩减 | 是否真的影响使用？最少保留哪些条件仍会失败？ | 生成便于开发者处理的证据 |

一个理想报告长这样（**示例，非实测**）：

> 设置页在内容区宽度 873–906 CSS px、高度 720 CSS px、英文文案、侧栏展开时，保存按钮被底栏遮挡；
> 滚动后仍无法通过指针点击。报告包含目标元素、遮挡元素、截图、环境信息和回放步骤。

## 📌 当前状态

- **未发布**：没有可下载的包，没有 npm 包，没有 tag。
- 仓库已初始化并推到双远程：GitHub [`pomelotea-yuzu/Layress`](https://github.com/pomelotea-yuzu/Layress)（主）
  与 Gitee [`yuzu-tea-duck/layress`](https://gitee.com/yuzu-tea-duck/layress)（镜像）；仍然没有 tag、没有可下载包。
  是否还有未推提交：`git fetch github && git log --oneline github/main..main`。
- 仓库内容 = 目录结构 + 接手索引 + 双语与洁净度门禁 + 立项调研。
- 首个验证方案（真实项目数据集、对照组、继续/收缩门槛）已写成建议，**尚未批准**。
- 已知的欠账与未决口径集中在 [`docs/knowledge/debt.md`](docs/knowledge/debt.md)。

## 🚀 计划中的能力

按调研的建议形态，扫描 / 确认 / 缩减 / 回放分离为可独立使用的模块，检测与报告核心完整开源：

| 模块 | 计划位置 | 职责 |
|---|---|---|
| 扫描 | `packages/core` | 在声明的压力条件下采集几何与遮挡候选 |
| 确认 | `packages/confirm` | 用真实命中测试判定关键操作是否可达，产出三态判定 |
| 缩减 | `packages/reduce` | 逐条去掉条件、每去掉一次仍要复现，得到最小触发条件 |
| 回放 | `packages/replay` | 只重放既有记录，不新增真实外部副作用 |
| 入口 | `packages/cli` + MCP / CI 集成 | 可读报告与结构化结果，可接进现有测试脚本 |
| 桌面 | `apps/desktop` | Electron 适配与结果浏览，后置 |

首版范围（Windows／Chromium + 一个 Electron 适配）、暂缓项（审美评分、自动修复、全平台矩阵、
最小 HTML 自动抽取）都写在调研 §7.2，不在这里重复承诺。

## ⌨️ 快速开始

现在仓库里能跑的只有两条门禁命令：

```bash
node scripts/check-hygiene.mjs --all    # 仓库洁净度体检
node scripts/check-i18n.mjs             # 双语成对体检（--selftest 反证门禁本身）
```

想参与开发，按顺序读 [`AGENTS.md`](AGENTS.md) → [`docs/development/workflow.md`](docs/development/workflow.md)
→ [`i18n/README.md`](i18n/README.md)。等 CLI 可以安装，这一节会换成真实的上手路径与四条使用场景。

## 🧱 项目结构

```
Layress/
├─ AGENTS.md              接手索引（唯一入口，中英成对）
├─ i18n/                  双语口径 · 术语表 · 成对登记
├─ docs/
│  ├─ guide/ architecture/ development/     面向使用者
│  ├─ knowledge/                            经验库：坑 · 不变量 · 决策 · 欠账
│  └─ research/ product/ engineering/ reviews/  工程过程资产
├─ datasets/              正负例数据集
├─ openspec/              config.yaml · specs/ · changes/{,archive/}
├─ packages/  apps/       代码（骨架已定，尚未实现）
└─ scripts/               check-hygiene · check-i18n
```

不入库的本地产物只去两个地方：`artifacts/`（可重跑产物）与 `.layress/`（运行时数据与日志）。

## 📚 文档

- 接手索引与纪律：[`AGENTS.md`](AGENTS.md)
- 文档地图：[`docs/README.md`](docs/README.md)
- 经验库（踩过的坑与不许重开的决定）：[`docs/knowledge/README.md`](docs/knowledge/README.md)
- 立项与市场调研：[`docs/research/`](docs/research/README.md)
- 双语术语表：[`i18n/glossary.md`](i18n/glossary.md)

## 🤝 贡献

先看 [`CONTRIBUTING.md`](CONTRIBUTING.md)，特别是**仓库洁净度**一节：本仓严格控制进入历史的文件，
拒绝逻辑由 `scripts/check-hygiene.mjs` 执行，豁免必须登记理由。
Issue 表单中英并列可直接填；Gitee 侧提供纯中文模板（`.gitee/`）。

## 📄 许可与声明

- 许可证：[MIT](LICENSE)
- **📦 存储纪律（不碰 C 盘）**：产品默认便携，数据、日志、缓存与临时文件**跟着产品放**
  （产品目录下的数据根，或你显式指定的目录），不写 `%APPDATA%`／`%LOCALAPPDATA%`／注册表；
  安装位置由你选择，不提权、不改系统 PATH。拷贝整个目录即完成迁移，删除即无残留。
  写不进时直接报错并要求指定目录，**不静默回落到用户目录**（见不变量 I12–I13）。
- 安全与隐私：[`SECURITY.md`](SECURITY.md) —— 本地优先是产品承诺，扫描对象的页面内容默认不出本机。
- 行为准则：[`CODE_OF_CONDUCT.md`](CODE_OF_CONDUCT.md)
- 更新日志：[`CHANGELOG.md`](CHANGELOG.md)
- 命名说明：`Layress` = Layer + Stress。与 npm 上的 `layrr`／`layerr` 和组件库 `layui` **无关**，
  请勿装错包。
