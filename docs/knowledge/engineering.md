# engineering.md · 工程与门禁的坑

> 本文默认以中文维护。

> 条目格式与升级规则见 [`README.md`](README.md)。**未经 Layress 实测的条目一律标注来源，不当作本仓已验证事实。**

## 环境与行尾

- [2026-11-03][P0] Windows 上格式检查满屏误报，仓库内容其实是好的 ⇒ 系统级 `core.autocrlf=true` 把工作区检出成 CRLF，而 linter（Biome/ESLint）要求 LF ⇒ 用 [`.gitattributes`](../../.gitattributes) 强制文本 `eol=lf` 并给二进制/夹具标 `-text` ⇒ 来源：前置项目 ReBaseAgent 全仓排查一次；**Layress 尚未实跑验证**（macOS 上系统级 autocrlf 一般不生效，但克隆来的全局配置仍要查）。
- [2026-11-08][P1] 门禁的「机器绝对路径」与「系统用户目录」规则只按 Windows 形态写，开发机迁到 macOS 后 `/Users/<name>/…`、`~/Library/…` 完全不判红 ⇒ 判据是平台函数而非环境常量，换开发机必须回头查规则覆盖面 ⇒ 三端形态并列写进 `check-hygiene.mjs`，补 macOS Library / Linux XDG / POSIX 绝对路径三个反证用例，由 `--selftest` 证明确实判红 ⇒ 证据：本轮 `check-repo.mjs --all` 自测回显。

## 门禁

- [2026-11-03][P1] `.gitignore` 拦不住垃圾 ⇒ 它是默认放行机制，一个 `git add -f` 就能把数百 MB 的二进制写进历史且删不干净（前置项目的运行时目录里实际躺着一个 374 MB 安装包）⇒ 拒绝逻辑必须由 `scripts/check-hygiene.mjs` 执行并在 pre-commit + CI 双跑 ⇒ 证据：本仓 8 项门禁实测（垃圾场路径/体积/`.env`/凭据/无理由豁免/带理由豁免/忽略泄露）。
- [2026-11-03][P0] CI 显示全绿但门禁根本没跑 ⇒ Gitee Go 的 `commands` 逐条执行、**单条失败不短路**，前面红了整体仍判绿；且 `build@nodejs` 插件可能静默跳过自带的 Node 安装，容器里没有可用 `node` ⇒ 所有校验用 `&&` 链成**一条**命令，由最终退出码判败；Node 在命令里自装，`nodeVersion` 字段只当必填占位 ⇒ 来源：前置项目首跑实证，本仓 [`.workflow/ci.yml`](../../.workflow/ci.yml) 已按此写。
- [2026-11-03][P1] 豁免清单会变成考古现场 ⇒ 提交进去的 glob 后来没有一个文件命中，没人敢删 ⇒ `check-hygiene.mjs --all` 把零命中的豁免报成警告，删掉即可 ⇒ 只在 `--all` 判定，staged 模式本次提交没碰到该文件属正常。
- [2026-11-08][P0] 开发机迁到 macOS 后 pre-commit 门禁形同未挂 ⇒ `.githooks/pre-commit` 在索引里的 mode 是 `100644`，Windows 不依赖执行位，Git on macOS / Linux 直接静默跳过（commit 时只给一行 ignoredHook 提示），本地洁净度链只剩 CI 兜底 ⇒ 改为 `100755` 入库，并在 `check-hygiene.mjs` 加「hook 缺可执行位」断言 + 反证用例；**新增 hook 一律核对索引 mode** ⇒ 证据：`git ls-tree HEAD .githooks/pre-commit` 原为 100644，修复后 `git hook run pre-commit` 实跑通过。

## 测试

- [2026-11-03][P0] 并行跑测试出现一片红，逐包复跑全绿 ⇒ 并行争用端口/临时目录/单例服务 ⇒ 判回归只认单包复跑与串行全链，并行结果不作证据 ⇒ 来源：前置项目；**Layress 建测试链前先按此约定**。
- [2026-11-03][P1] 「全量绿」无法核对 ⇒ 只报"通过"没有可比对的量 ⇒ 全量结果必须留三元组：**文件数 + 用例数 + Errors 行**，回归判定时逐项对齐；计数小幅波动（宿主 shim 影响临时目录）以"已收集全过 + Errors=0"为准，不追求逐位精确。
- [2026-11-03][P1] 没有 jsdom 时写出"镜像断言" ⇒ 测试只是把实现重抄一遍，真缺陷照样绿 ⇒ 无 DOM 环境时只写**源码结构断言**并显式标注它不覆盖运行时行为 ⇒ 来源：前置项目；Layress 若走 Node 侧检测模块同样适用。

## 断言字面量

- [2026-11-03][P0] 断言里的字面量永远红或永远绿，且看不出原因 ⇒ 直接在参数里写非 ASCII 字符（`U+FFFD`、emoji）会被工具链吞成空串，于是「包含某非法字符」的断言变成「包含空字符串」= 恒真 ⇒ 非 ASCII 断言参数一律写 `\uXXXX` 转义，并在自测里验证转义本身生效 ⇒ 来源：前置项目实证。

## 存储与便携（便携优先 · 不写系统用户目录）

- [2026-11-03][P0] 号称便携的产品仍在系统用户目录长垃圾 ⇒ 只把"数据目录"挪到产品旁不够：Windows `os.tmpdir()` = `%TEMP%` = `C:\Users\<user>\AppData\Local\Temp`，macOS 落在 `/private/var/folders/…`，Linux 落在 `/tmp` 或 XDG 缓存；Electron `app.getPath('userData')` 与 Chromium `sessionData`/`Cache`/`GPUCache` 三端默认同样落在系统用户目录 ⇒ 三处必须一起覆盖：`app.setPath('userData', <数据根>/userdata)`、`session.setPath('sessionData', ...)`、自建临时目录并显式传给 `--user-data-dir`；**禁止把 `os.tmpdir()` 当默认值用** ⇒ 来源：前置项目 `.gitignore` 已记录"一次性工作目录（含 Electron sessionData 缓存等临时文件）"这一坑；**Layress 待实测确认**。
- [2026-11-03][P0] 目标目录不可写时静默回落到用户目录 ⇒ 常见写法是"写不进就 `%APPDATA%` / `~/Library` / XDG 兜底"，直接违反不变量 I12，且用户完全不知道数据去了哪 ⇒ 启动做可写探测；不可写就明确报错并要求 `--data-dir`，**不允许回落**。
- [2026-11-03][P1] 用户无法验证数据落点 ⇒ 解析出的数据根从不出现在输出里 ⇒ 启动横幅、日志与报告头部都要写 `resolvedDataDir` 与可写性检查结果（I13）。
- [2026-11-03][P1] 便携包换机器/换盘符后证据路径失效 ⇒ 配置、报告与数据集里存了 `D:\...` 或 `/Users/<name>/...` 这类绝对路径 ⇒ 仓库与产物内一律相对数据根、用 posix 分隔符；只在导出给用户时才物化成绝对路径。门禁的「机器绝对路径」检查三端形态都收，就是为这条兜底。
- [2026-11-03][P1] 装在只读介质（U 盘、光盘）或被安全软件管控的目录时崩溃 ⇒ 没有降级路径 ⇒ 明确支持"只读产品目录 + 用户指定可写数据根"这一组合，并在文档里写清。

## 依赖与产物

- （待填）首个 `pnpm install` 之后，把实际遇到的解析/锁定/离线打包坑写在这里。
