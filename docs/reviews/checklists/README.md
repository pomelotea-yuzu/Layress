# 应用层人工复验清单（归档）

> 2026-11-16 自 `apps/desktop/README.md` 迁入：README 只留结构与口径，复验步骤集中归档于此。
> **新流程（用户 2026-11-16 定）**：之后的复验步骤直接在对话中提供，不再写入 app README；
> 复验**结论**仍记入对应 change 的 evidence-index 与本目录（如需留档）。
> 清单按任务原样保留（含已完成的历史判据），供回归与接手人查阅；章节标题未改，旧文锚点仍可跳转。
> 相对链接已按本目录深度修正（`../../` → `../../../`）。

---

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

## 任务 4.x 人工复验（开发期，2026-11-13 用户已复验通过）

### 0. 复验前准备（每换一个数据根都要重做）

本轮改过 Rust / 样例页 / 工具层 JS，**旧窗口一律不算**。在跑 `pnpm tauri dev` 的终端按 Ctrl+C，然后确认没有残留：

```bash
pgrep -fl "tauri dev|vite|layress-desktop"
```

有残留就按显示的 PID 逐个 `kill <pid>`。数据根**只在启动时解析**，所以下面 A / B / C 三档每换一次都要 Ctrl+C 重启，运行中改参数无效。

先记基线（后面判「没有新增文件」要用）：

```bash
ls -1 ~/Developer/project/layress/apps/desktop/src-tauri/target/debug/.layress/captures/*.png 2>/dev/null | wc -l
ls -ld ~/Library/Application\ Support/layress ~/Library/Caches/layress 2>/dev/null || echo "系统用户目录无 layress 目录"
```

把第一个数字记下来（记作 N）。

### 1. 4.1 · `--data-dir` 到底写哪里

**`--data-dir=` 后面写一个目录的绝对路径**——这个目录就是应用写截图、元数据、日志的「数据根」。不是文件名、不是 URL；**显式相对路径（含 `~`）已在解析层以 `DATA_ROOT_RELATIVE_UNSUPPORTED` 拒绝**（任务 1.4，2026-11-16），不再随启动工作目录漂移，根不存在也同样拒绝，两个来源（`--data-dir` / `LAYRESS_DATA_DIR`）分别报错。

它写在整条启动命令**末尾、第二个 `--` 之后**，用等号连写最稳：`pnpm tauri dev -- -- --data-dir=<绝对路径>`。
第一个 `--` 之后是 cargo runner 参数，第二个 `--` 之后才传给应用（见 `pnpm tauri dev --help` 的 Arguments 段）。

三档各起一次应用：

| 档 | `--data-dir` 后面写什么 | 验证什么 |
|---|---|---|
| A 默认 | 整个参数省略 | 默认落点 = exe 旁 `.layress/` |
| B 可写 | `/Users/yangzhicheng/Developer/project/layress/artifacts/custom-root` | 显式指定生效、产物落在那里 |
| C 不可写 | `/dev/null/layress-probe` | 明确失败、不回落、不兜底 |

B 档完整命令（可直接粘贴）：

```bash
cd ~/Developer/project/layress/apps/desktop \
  && PATH="$HOME/.workbuddy/binaries/node/versions/22.22.2-6/bin:$HOME/.cargo/bin:$PATH" \
     pnpm tauri dev -- -- --data-dir=/Users/yangzhicheng/Developer/project/layress/artifacts/custom-root
```

C 档只改末尾：

```bash
cd ~/Developer/project/layress/apps/desktop \
  && PATH="$HOME/.workbuddy/binaries/node/versions/22.22.2-6/bin:$HOME/.cargo/bin:$PATH" \
     pnpm tauri dev -- -- --data-dir=/dev/null/layress-probe
```

> `/dev/null` 是文件不是目录，`create_dir_all` 必定失败，不会往系统里写任何东西，比用受保护目录更安全。
> 不想依赖参数透传时，可用环境变量形式 `LAYRESS_DATA_DIR=<绝对路径> pnpm tauri dev`（不需要 `--`）。

**A 档（不带参数正常启动）**

1. 看**窗口最底部状态栏**：灰条应为 `数据根：…/apps/desktop/src-tauri/target/debug/.layress（来源：可执行文件目录旁 .layress/，可写）`。
2. 看终端 stderr：应有 `[layress] 数据根：…（来源：…，可写：是）`。
3. 点「打开样例页」→ 点「采集」，状态栏转绿；到 `<数据根>/captures/` 应多出一对 `<id>.png` + `<id>.json`。

**B 档（自定义可写根）**

1. 状态栏应显示 `数据根：/Users/yangzhicheng/Developer/project/layress/artifacts/custom-root（来源：命令行 --data-dir，可写）`。
2. 采集一次后跑：

   ```bash
   ls -1 /Users/yangzhicheng/Developer/project/layress/artifacts/custom-root/captures/
   ```

   应看到新 PNG / JSON；默认 `.layress/captures/` 的数量停在 N，没有往默认根写。
3. 优先级（诱饵路径只作对照，不会被写入）：

   ```bash
   cd ~/Developer/project/layress/apps/desktop \
     && PATH="$HOME/.workbuddy/binaries/node/versions/22.22.2-6/bin:$HOME/.cargo/bin:$PATH" \
        LAYRESS_DATA_DIR=/Users/yangzhicheng/Developer/project/layress/artifacts/decoy \
        pnpm tauri dev -- -- --data-dir=/Users/yangzhicheng/Developer/project/layress/artifacts/custom-root
   ```

   状态栏仍应显示「来源：命令行 --data-dir」，路径是 `custom-root` 而不是 `decoy`。

**C 档（不可写负例）**

1. 终端应出现 `[layress] 数据根不可写，采集将明确失败：数据根不可创建（/dev/null/layress-probe）：…`。
2. 状态栏应为**红条**：`数据根：/dev/null/layress-probe（来源：命令行 --data-dir，不可写：…）`。
3. 点「打开样例页」→ 点「采集」：状态栏红条 `✗ 本次采集失败 …`，**不进入静态预览**。
4. 核对没有偷偷回落：

   ```bash
   ls -ld ~/Library/Application\ Support/layress ~/Library/Caches/layress 2>/dev/null || echo "无兜底目录"
   ls -1 ~/Developer/project/layress/apps/desktop/src-tauri/target/debug/.layress/captures/*.png 2>/dev/null | wc -l
   ```

   第一行应是「无兜底目录」；第二行仍为 N，说明这次失败没有在默认根写新文件。

### 2. 4.3 · 原生访问负例（在 A 档或 B 档应用里做）

1. 点「打开样例页」，在**被测页面**（窗口中间那块，不是顶部工具条）往下滚到
   「文件 / 进程访问负例（E02、E13，应全部被拒绝）」这一段。
2. 依次点四个按钮，每点一次看该段下方的蓝色结果行（`#native-out`，只显示最后一条）：

   | 点哪个按钮 | 结果行应包含 | 判定 |
   |---|---|---|
   | `fetch 本地文件` | `被拒绝`（或 CSP 相关错误） | 出现「意外成功」= 不通过 |
   | `invoke 文件读取命令` | `被拒绝` 或 `原生桥不存在（已拒绝）` | 同上 |
   | `invoke 进程命令` | 同上 | 同上 |
   | `invoke 采集命令` | 同上 | 同上 |

3. 顺便核对上方「原生命令边界（E02）」段：`__TAURI_INTERNALS__=undefined`、`invoke调用=被拒绝（…）`，不能出现 `invoke调用=意外成功`。
4. 如实口径：Web 页面本身没有进程 / 文件 API，这四个按钮用「调用不存在的原生命令」作代理，验证的是 capability 边界，**不声称**覆盖完整进程访问。

### 3. 4.4 · 退出清理与重启（在 A 档或 B 档应用里做）

1. 先记当前 captures 的 PNG 数（用第 0 步命令），记作 N。
2. 「打开样例页」→ 勾工具条第二行「慢采 5s」→ 点「采集」→ 状态栏出现浅黄 `[started] 慢采已开启：…5000 ms…`。
3. **在 5 秒内直接关闭应用窗口**（点窗口红叉 / Cmd+W，不是点「取消」按钮）。
4. 用 A 档命令重新启动，然后核对：

   ```bash
   ls -1 ~/Developer/project/layress/apps/desktop/src-tauri/target/debug/.layress/captures/*.png 2>/dev/null | wc -l
   ```

   - 数字仍是 N ⇒ 被关掉的这次采集没有落盘；
   - 原有 N 个 PNG / JSON 仍在，没被删；
   - 数据根以外（仓库其它目录、`~/Library/...`）没有被删除或改动；
   - 应用能正常打开、样例页可点击、状态栏再次显示数据根。
5. 终端退出时应有 `[layress] 退出：数据根 … 保留，未清理数据根外文件`。

### 4. 判据

4.1 / 4.3 / 4.4 已于 2026-11-13 15:42 由用户人工确认全部通过（原话「做完了，没遇到问题」），tasks 对应项已勾选、E09 / E02 / E11 已回填通过。
**4.2 已移出本 change 范围（D30）**：便携的载体是打包后的产品目录，开发态没有该对象；真实写盘观测归首次打包后的发行轮 change，见 [debt](../../../docs/knowledge/debt.md)。
机器单测只覆盖纯逻辑，不替代 GUI 复核（I11）。

## 任务 5.2 人工复验清单（macOS GUI，2026-11-14 **已完成**，四轮全部通过）

> 本轮只改**反馈呈现通道**（`main.ts`）：剩余 `statusLine` 直接赋值全部收敛到底部状态栏（I15），
> 补 E04「窗口容纳不下」与 E13「额外入口被拒绝」两条红条，返回页面 / 尺寸 / 取消 / 初始化提示改走状态栏。
> 业务逻辑未动，1.x–4.x 已通过的业务判据不重跑；下面 A–D 是**此前未单独执行**或**本轮新通道**的正反场景。
> 观察通道统一为**窗口最底部独立状态栏**；被测页内的结果行看窗口中间那块页面。
> 逐条对照见 [evidence-index「任务 5.2」](../../../openspec/changes/archive/2026-11-14-add-tauri-page-host/evidence-index.md)。
> **复验结论见文末「判据回填」**，清单保留作历史步骤记录。

### 0. 完整重启 + 版本自证（必做，不做下面全部作废）

改过 `index.html` / `main.ts`，CSP `default-src 'self'` 会挡掉 Vite HMR，旧窗口仍是旧 DOM。
**2026-11-14 首轮复验就跑在旧窗口上**：A 组（3.x 旧链路）通过，B / C / D（本轮新增）全部无反馈，
而 Vite 服务的 `main.ts` 已含新代码 ⇒ 窗口里的 JS 是改动前那份。因此现在用**版本徽标**自证，不再靠「我重启过」。

在跑 `pnpm tauri dev` 的终端按 Ctrl+C，然后确认下面这条**输出为空**：

```bash
pgrep -fl "tauri dev|vite|layress-desktop"
```

有残留就按 PID 逐个 `kill <pid>`，再跑一次 `pgrep` 确认为空。然后在任意目录粘贴：

```bash
cd ~/Developer/project/layress/apps/desktop \
  && PATH="$HOME/.workbuddy/binaries/node/versions/22.22.2-6/bin:$HOME/.cargo/bin:$PATH" pnpm tauri dev
```

本轮改的是 **Rust 层与内嵌样例页**（`fixtures/sample-page.html` 由 `include_str!` 编译期嵌入），
前端没动 ⇒ 徽标仍是 `v20261114-2`；重启后终端应出现 `Compiling layress-desktop`，否则改动没进进程。

**启动后第一件事：看工具条第二行最右侧**，必须显示灰色的 `v20261114-2`：

| 看到什么 | 判定 |
|---|---|
| 灰色 `v20261114-2` | 新窗口，可开始复验 |
| 红色「加载中…」 | HTML 是新的但脚本没跑完 / 被拦，**不可复验** |
| 徽标整个不出现 | 仍是旧 HTML / 旧窗口，**不可复验** |
| 状态栏红条 `⚠ 工具界面 HTML 与脚本不同步…` | 同上 |

任一行命中「不可复验」就回到第 0 步重来，别往下点。徽标对了再点「打开样例页」，
确认状态栏 `✓ 页面加载完成：…` 与样例页内容都在。

### A · 采集失败且存在旧图（E06，此前未单独人工核对）

1. 点「采集」正常采一次 ⇒ 状态栏绿条 `✓ 采集完成 cap-…`，进入静态预览，记下这个 id。
2. 点「返回页面」，勾上工具条第二行的「慢采 5s」。
3. 再点「采集」⇒ 状态栏浅黄 `[started] 慢采已开启 … 5000 ms …`。
4. 在 5 秒内点「刷新」（或「取消」/ 再点一次「采集」）。
5. **预期**：状态栏红条 `✗ 本次采集失败 … 上次成功 <刚才的 id> 仅作旧现场，不代表本次成功 …`；
   **不进入静态预览**，上次成功的图不被当作本次结果。红条较长时以开头到「仅作旧现场」这段为准。
6. 按红条提示到 `<数据根>/captures/` 核对本次没有新增 PNG / JSON。
7. 取消勾选后再正常采集一次，状态栏转绿 ⇒ 失败后仍能正常采集。

### B · 弹窗会话与返回页面（E08，此前「弹窗会话未单独触发」）

1. 在被测页「页面内弹窗」区点 `confirm`（或 `prompt`），完成弹窗后确认下方 `confirm 返回：…` 文字留在页面上。
2. 在「点击 / 输入 / 会话状态」区点几次计数按钮、在输入框写一段文字，记下数值。
3. 点「采集」→ 预览里切「原始比例」/「适应区域」各看一眼 → 点「返回页面」。
4. **预期**：状态栏**绿条** `已返回原 WebView（会话标识未变化）；输入与弹窗请人工复核`；
   输入值、点击计数、`confirm 返回：…` 文字都还在，页面没有被重载。
5. 出现红条 `⚠ 返回后页面会话标识变化…` 或弹窗结果消失 ⇒ 不通过，直接反馈。

### C · 窗口容纳不下（E04，本轮把提示搬到状态栏）

| 步骤 | 预期 |
|---|---|
| 宽 / 高填 `1000` / `600`，点「应用尺寸」 | 状态栏绿条 `✓ 视口已生效 1000×600 · 窗口内完整显示…`；被测页「实际视口与引擎」区显示 `innerWidth 1000 px / innerHeight 600 px` |
| 改成 `1280` / `704`，点「应用尺寸」 | 状态栏**红条** `⚠ 窗口容纳不下已设定视口 1280×704：视口保持不变，未静默缩放（请放大窗口）` |
| 紧接着看被测页 | `innerWidth/innerHeight` **仍是 1280 / 704**，没有被偷偷改宽高、也没有缩放成「适配窗口」 |
| 把窗口拖得更小 / 放大回去 | 红条**始终浮在最上层、不被页面盖住**；放大到装得下后，下一次视口变化会回到绿条 |

> 2026-11-14 第二轮反馈：红条其实有，但窗口缩小后被被测页面挡住了。根因是页面 WebView 在点
> 「打开样例页」时才 `add_child`，后创建的视图压在启动期创建的状态栏之上。现已用
> `status.reparent(&window)`（macOS wry = `contentView.addSubview`）把状态栏提回最上层。
> 若窗口已被放大到能装下 `1280×704`，先拖小一点再观察；判据是「设定值不变 + 明确提示 + 红条可见」。

### D · 新增反馈通道（I15 收敛 / E13）

| 操作 | 状态栏预期 |
|---|---|
| 不勾慢采、直接点「取消」 | 灰条 `当前没有在途采集`（以前只写在被裁的第三行） |
| 点「应用尺寸」（见 C） | 绿条 / 红条按可见性给出，不再只有第三行 |
| 点被测页「window.open 新窗口」 | 红条 `✗ 已拒绝 new-window …（未开新窗口 / 未写下载 / 未授设备权限）`，页面结果行显示「新窗口未创建」 |
| 点「触发下载」 | 红条 `✗ 已拒绝 download …`，终端有 `[layress][request] 下载请求已拒绝：…`，不产生下载文件 |
| 点「请求地理位置」 | 页面结果行**立即**变成 `地理位置：已点击，正在核对 navigator.geolocation …`，随后为三者之一：`本 WebView 未提供 navigator.geolocation…` / `权限请求 2s 内未得到系统回调…` / `geolocation 被拒绝…`；把最终那行文字与终端 `[layress][request]` 日志贴回来 |

**下载 / 地理位置的口径**：首轮把「没红条」判成待复核是对的，但根因不同——
下载夹具原来返回 `Content-Type: text/plain`，WKWebView 会直接渲染成页面，根本不进 wry 的下载
delegate，`on_download` 自然不触发；已改成 `application/octet-stream`，让负例真的发起下载请求。
第三轮用户澄清地理位置是**「点了没反应」**：权限请求可能既不回调也不报错，页面结果行一直空白。
现已给按钮加点击即时反馈 + 2s 超时兜底，并顺手给坐标参照块加 `pointer-events: none`
（`z-index:9999` 的纯视觉标记本不该拦截点击，虽不是本次根因）。
这几种情况都**不是拒绝失效**，但也**不算「系统回调明确拒绝」已验证**，单列未执行；
Rust 回调的 stderr 日志用来区分「没触发」与「触发后被拒绝」。

### 判据回填

**首轮结果（2026-11-14，旧窗口，判据作废）**：A 组通过；B「返回页面后绿条文字不变」、
C「1280×704 与缩小窗口都没有红条」、D「几项都没有底部状态栏反馈」。
核对事实：Vite 服务的 `main.ts` 已含本轮新代码（`lastViewSignal` / `视口已生效` / `窗口容纳不下`），
而 A 的失败红条来自 3.x 旧链路且正常 ⇒ **窗口加载的是改动前那份 JS**。
B / C / D **未取得有效判据，既不计通过也不计失败**，单列待重跑；不是产品缺陷结论。
已加工具条版本徽标与 `check-build.mjs` 一致性断言，防止第四次靠「重启过了」猜。

**第二轮结果（2026-11-14，新窗口，真缺陷）**：C 的红条确实出现，但窗口缩小到一定程度后被被测页面挡住；
D 的「触发下载」「请求地理位置」没有红条（window.open 有）。定位为两个真缺陷 + 一个平台限制：
① 页面 WebView 后创建压住状态栏 ⇒ 用 `reparent` 把状态栏提回最上层（已修，待复验）；
② 下载夹具 `Content-Type: text/plain` 让 WKWebView 直接渲染、不进下载回调 ⇒ 改 `application/octet-stream`（已修，待复验）；
③ `navigator.geolocation` 可能不存在，样例页直接 return ⇒ 未发起权限请求，`on_permission_request` 不触发，
**如实单列未执行，不宣称「系统回调明确拒绝」已验证**。B 组本轮未再反馈，仍待重跑。

**第三轮结果（2026-11-14 12:08）**：用户确认「其他没有问题，地理位置是点了没反应」⇒
**A、B、C 及 D 的取消 / 应用尺寸 / window.open / 下载全部通过**；状态栏层级与下载夹具两个真缺陷修复有效。
地理位置已加即时反馈 + 超时兜底，样例页为编译期嵌入 ⇒ 需重启后单独重测这一项。

**第四轮结果（2026-11-14 12:19，闭环，5.2 勾选）**：重启后的新窗口（应用进程 12:18:24 重编拉起，
本机 `curl 127.0.0.1:51728` 已核对其样例服务吐出的页面含全部第四轮修复标记、脚本 `node --check` 通过）
点「请求地理位置」⇒ 页面错误回调收到 **`geolocation 被拒绝：1 User denied Geolocation`**（PERMISSION_DENIED），
**权限未授予 ⇒ E13 地理位置项通过**。
状态栏绿条不变、终端无 `[layress][request]` 日志是因为拒绝由 WebKit 系统层给出、
**应用层 `on_permission_request` 未被系统调用**——红条 / 日志不出现属平台行为，已如实单列「未执行」。
另登记硬事实：状态栏只反映应用层事件，**页面内权限被拒不会、也不应改变状态栏绿条**。

机器侧（13 项单测 + `check-repo.mjs --all` 全链 + OpenSpec strict validate）已全绿。
A / B / C / D 四组全部经用户人工复验通过，[tasks 5.2](../../../openspec/changes/archive/2026-11-14-add-tauri-page-host/tasks.md) 已勾选；
剩余 5.3（本机开发运行与首个本地构建的真实大小、WebView 前置依赖及限制）、5.4（门禁实跑与归档）未开始。


---

## 任务 1.4 人工复验（当前 change，2026-11-16 **已完成**，三档全部通过）

解析层单测已过（两来源分别拒绝、CLI 绝对覆盖 env 相对不误拒、根不存在也拒绝）；同日用户按下列三档实机复验「都没问题」，**1.4 已勾选**，清单保留供回归：

1. **相对 CLI 根**：`pnpm tauri dev -- -- --data-dir=layress-relative-root`（故意写相对路径）。
   判据：状态栏红条含 `DATA_ROOT_RELATIVE_UNSUPPORTED`、原始相对值与「改用绝对路径」提示，且在开场提示**之后**显示不被冲掉；终端 stderr 有「数据根解析失败」；此时点「采集」应被拒（数据根未解析）。
2. **相对 env 根**：`LAYRESS_DATA_DIR=./rel-env pnpm tauri dev`（无 `--`）。
   判据：红条来源显示「环境变量 LAYRESS_DATA_DIR」，同样拒绝。
3. **CLI 绝对覆盖 env 相对**：`LAYRESS_DATA_DIR=./rel-env pnpm tauri dev -- -- --data-dir=<任意可写绝对路径>`。
   判据：不误拒，状态栏显示「来源：命令行 --data-dir，可写」，采集正常。

## 任务 1.5 人工复验（开发期，备份重建待用户确认）

机器层 5 项单测已过（成功备份字节 / 10 份上限 / 备份失败 / 提交失败 / 双实例 LOCKED）；**GUI 判据待人工**。每档前先 Ctrl+C 关干净旧程序，启动命令同上（真终端 + PATH 前缀），窗口右上角徽标应为 **`v20261116-2`**：

1. **制造损坏配置**：默认数据根在 `apps/desktop/src-tauri/target/debug/.layress/config/`，先启动一次（生成目录）后退出，把 `projects.json` 内容改成一个坏文件（如 `not-json`）。
2. **重启看错误态**：底部状态栏红条 + 打开「项目」设置面板应显示 `配置不可用（CONFIG_CORRUPT）：…`，且出现红色可点的「备份并重建空配置」按钮。
3. **两段确认**：点第一次只变红色警告「⚠ 会清空项目记忆（截图保留）——再点一次确认」（6 秒不点自动复位）；再点一次 → 状态栏绿色「已备份原配置并重建为空配置」，设置面板恢复「尚无项目记录」。
4. **核对落盘**：`config/` 里出现 `projects.backup.<时间戳>.<后缀>.json` 且内容正是坏 JSON；`projects.json` 已是合法空配置；`captures/` 里的截图原样未动。
5. **负例**：重建成功后再点该按钮 → 按钮已隐藏（配置正常不再提供）；对正常配置直接调命令会被「无需重建」拒绝。

三句结论（错误态看得到吗 / 两段确认与重建成功吗 / 备份与截图落盘对吗）贴回，通过后 1.5 勾选。

## 任务 0.1 布局可行性复验（当前 change，2026-11-15 已实机验证并勾选）

> 目的：在扩展完整项目 UI **之前**，验证「工具层铺满工作区 + 被测层等尺寸平移出可视区」真能容纳表单。
> 依据：[design §1](../../../openspec/changes/add-project-device-profiles/design.md)、
> [tasks 0.1](../../../openspec/changes/add-project-device-profiles/tasks.md)、E07 / E09。
> 现有 `apply_layout(app, true)`（静态预览已用）就是铺满机制；本轮新增 `LAYRESS_PROBE_LAYOUT=1` 探针把它量化。
> **探针只改 Rust debug 分支，不改 `index.html` / `main.ts` ⇒ 徽标仍应是 `v20261114-2`。**
> 探针注入的「0.1 布局诊断表单」是临时 DOM，不代表项目 UI 已实现；退出时自动移除。

### 第 1 步：把旧程序关干净

在跑着 `pnpm tauri dev` 的终端按 **Ctrl+C**，然后粘贴这一行：

```bash
pgrep -fl "tauri dev|vite|layress-desktop"
```

**必须没有任何输出**。有输出就按显示的数字（PID）逐个 `kill 12345`，再粘贴一次确认空。
不关干净，启动的还是改动前的程序，日志会跟旧的一模一样。

### 第 2 步：粘贴这一条命令，什么都别点

```bash
cd ~/Developer/project/layress/apps/desktop \
  && PATH="$HOME/.workbuddy/binaries/node/versions/22.22.2-6/bin:$HOME/.cargo/bin:$PATH" \
     LAYRESS_PROBE_LAYOUT=1 pnpm tauri dev
```

等终端出现这两样，才算新程序起来了：

- `Compiling layress-desktop` —— 本轮改的是 Rust，**没有这行 = 没重编译 ⇒ 回第 1 步**；
- `[layress][layoutprobe] 探针 r3 已启动…` —— **没有 `r3` 就是旧程序**。

窗口工具条第二行最右应是灰色 `v20261114-2`（0.1 复验当时的值；现行徽标见下方 1.1 复验清单）。**不用点「打开样例页」、不用拖窗口**，
探针会自己打开样例页、自己把窗口缩到最小。

### 第 3 步：什么都不用点，看 14 秒

终端出现 `[layress][layoutprobe] 开始铺满工具层（约 14 秒）…` 后，
**探针会自己把窗口缩到最小**（终端会打印 `已自动缩到最小窗口 520×400`），不用拖窗口、不用点按钮。

这 14 秒里用眼睛回答三个问题（用「能 / 不能 / 被挡了」回答即可）：

1. 白色「0.1 布局诊断表单」里的 4 个输入框和 2 个按钮，是不是都看得全、没被切掉？**大窗口和缩到最小时各看一眼**；
2. 窗口最底下那条状态栏，有没有被白表单或页面盖住？**缩到最小时再确认一次**；
3. 用鼠标点一下「项目名称」输入框，能不能正常打字？

### 第 4 步：把终端文字整段贴回来

从 `Running target/debug/layress-desktop` 那行开始，到最后一行 `已恢复常态`，**全部复制**发给我，
再加上第 3 步三个问题的答案。

贴之前瞄一眼：里面有 `probe=r3`、`已自动打开样例页` 和 `已自动缩到最小窗口 520×400` 才对；
**缺任意一行就别贴**，说明还是旧程序，回第 1 步重来。

下面的表格是编写者判据，操作的人不用管。

### 3. 判据（逐条对照，任一条不满足就不勾 0.1）

| 观测 | 通过 | 不满足时 |
|---|---|---|
| `fullN.cssH` | 接近当前窗口逻辑高，**不是常态的 64** | `set_bounds` 没换来 CSS 工作区；先走下面「退路」，不直接扩展 UI |
| `controls/visible/hit/focusable` | 大窗口 6/6/6/6；小窗口记录 `clipped` 里哪些控件被裁 | 被裁或 `hit<6` ⇒ 记下 rect 与 top/bottom，判断能否靠滚动 / 布局容纳 |
| `page` bounds | 铺满前后 **size 不变**，只有 position 移到视口外；`configured` 不变 | 尺寸被改 ⇒ 违反 D28「返回不重建会话」，标阻塞 |
| `status` bounds | 始终在窗口底部、逻辑高 42 | 缩窗后被盖 ⇒ 先核 `bring_status_to_front` / reparent，再复验 |
| `build` | `v20261114-2` | 不符合 ⇒ 版本自证失败，本轮结果作废 |
| `restore` | 工具层回 96px、page position 回 0、`controls=0`（诊断表单已移除） | 没恢复 ⇒ 探针或布局有残留，标失败 |
| 预览→设置→返回 | 布局切换不销毁 / 重载被测页，返回后原预览现场可恢复 | 属 3.4 业务仲裁；0.1 只验布局层，未实现部分单列 |
| 编辑中采集被拒 | 业务规则尚未实现（任务 3.4） | **0.1 不得假称通过**，单列为后续实现 |

### 4. 退路与阻塞（design §1）

`cssH` 仍 64 时，先在同一窗口修工具层原生 bounds / CSS：核对 `TOOLBAR_H=96` 与 scale 的映射、
`#bar` 固定高与 `overflow:hidden`、诊断容器是否该改为贴视口的 `position:fixed`；修完重跑第 1 步，
**仍须保留被测实际尺寸、底部状态栏与权限隔离**。两条路径都不可行 ⇒ 记录失败与实测值、把依赖 UI 的任务标阻塞，
不用工具条第三行藏表单、不开第二个顶层窗口、不假通过。

### 5. 回显记录与 0.1 结论（2026-11-15）

首轮用户实机回显：大窗口 1280×800 与最小窗口 520×400 铺满后，诊断表单均为 6/6/6/6 且无裁切；
`cssH = tool bounds 逻辑高 − 32`（96→64、800→768、400→368，9 组一致）⇒ **布局可行性通过**。
但 `full3` 因 Resized 触发 `relayout` 复位成 96px（探针竞态，已修），全程 `page=None`，
且未收到三句人工观察 ⇒ **0.1 仍未通过、未勾选**。逐轮数据与未执行项见
[evidence-index](../../../openspec/changes/add-project-device-profiles/evidence-index.md)。

第二轮回显（12:53）与首轮**逐字一致**（含 `Finished dev profile … in 9.15s`、`full3` 595×96、
`restore` 仍 `controls=6`），且不含 `probe=r2` ⇒ 跑的是修复前二进制或旧日志，**不采信、0.1 状态不变**。
随后把探针升级为 **r2**：回显强制带 `probe=r2`，并自动打开样例页。

第三轮回显（13:03，`probe=r2`）：确认是新程序；自动打开样例页，全程 `page=Some`（1280×662，
铺满前后 **size 不变**、只 position 3280→0）；6 轮 `fullN` 全部铺满、6/6/6/6 无裁切；
`restore` 工具层回 96px、`controls=0`（表单已移除）；人工三句「能 / 没挡 / 能」。
⇒ **大窗口布局可行性、被测页尺寸、状态栏层级、焦点、恢复均通过**。但 6 轮全是 1280×800
（用户未缩窗）⇒ **小窗口与 Resized 竞态修复在 r2 未验证**，0.1 仍未勾选。

本轮升级 **r3**：探针在第 4 轮前自己把窗口设为 520×400（主动触发 Resized 以验证 `relayout` 保持铺满），
结束前还原 1280×800；回显强制 `probe=r3`。**操作者不用再拖窗口。**

历史重跑要点（r3 已全部满足，保留作记录）：① 按第 1 步关净旧进程并看到 `Compiling layress-desktop`；
② 确认回显含 `probe=r3`、`已自动打开样例页` 与 `已自动缩到最小窗口 520×400`；③ `full0–2` 应为 1280×800、
`full3–5` 应为 520×400，且 6 轮 tool 高始终等于当前窗口高（不再出现 96）；④ 给三句肉眼观察 + 整段回显。

**第四轮回显（13:16，`probe=r3`）：0.1 布局可行性前置通过并勾选。** `full0–2` 1280×800、
`full3–5` 自动缩到 520×400 且 tool 高始终等于窗口高（缩窗后没有再出现 96px ⇒ Resized 竞态修复生效）；
大 / 小窗口均 6/6/6/6 无裁切，cssH 768 / 368；状态栏恒在底部 42 且人工确认「没挡」，焦点「能」；
`restore` 工具层回 96、窗口还原、page 归位、`controls=0`；人工三句「和刚刚一样能，没挡，能」。
`configured=None`（跟随窗口模式）下缩窗时 page bounds 1280×662→520×262 属既有行为，不是布局偷改视口；
「设定固定视口后缩窗不变」反例未在 0.1 执行，单列到 3.4 回归。逐轮数据与未执行项见
[evidence-index](../../../openspec/changes/add-project-device-profiles/evidence-index.md)「0.1 r3 回显与勾选」。

### 6. 回填

结果写进 [evidence-index](../../../openspec/changes/add-project-device-profiles/evidence-index.md) 的 E07 / E09 行与逐场景表；
0.1 已勾选**仅布局可行性**（口径见 [tasks 0.1](../../../openspec/changes/add-project-device-profiles/tasks.md)），
1.1 之后完整 UI 可开始，预览→设置→返回 / 编辑中拒绝采集等业务仲裁仍归 3.4、未执行。
探针是 debug-only：不设 `LAYRESS_PROBE_LAYOUT` 时产品路径零影响；
release 构建未实跑（空实现分支），发行轮单列。

## 任务 1.1 人工复验清单（2026-11-15，已完成）

> 本轮改过 `index.html` / `main.ts`，CSP 会挡 Vite HMR，**必须完整重启**；窗口徽标不是 `v20261115-2` 时下面全部作废。

### 0. 完整重启

Ctrl+C → `pgrep -fl "tauri dev|vite|layress-desktop"` 无输出 → 重新跑 `pnpm tauri dev`（真终端命令见上文）→
终端出现 `Compiling layress-desktop` → 工具条第二行最右显示灰色 `v20261115-2`。

### 1. 正例：新建项目并落盘（对话里说的「第 4 步」就是这一节）

> 下面编号 0–4 是唯一口径；对话里若出现「重做第 3 步」这类回指，一律以这里为准。

1. **打开面板**：看窗口**最上面第一行**（有长地址输入框那行），最右边有个按钮写着「项目：无」⇒ 点它。
   屏幕中间应出现白色大面板，标题「项目设置」，里面有 3 个输入控件和 2 个按钮；工具条仍在最上面。
   白面板没出现就先停，把第一行你看到的按钮名字告诉我，别继续填。
2. **填表**：
   - 第 1 个框（标签「项目名称」）填 `本地后台`；
   - 第 2 个框（标签「起始地址」）填 `https://example.com`；
   - 第 3 个下拉（标签「视口预设」）选 `手机 390×844`（不选也行，默认桌面 1280×800）。
3. **保存**：点面板里的「新建项目并保存」按钮。
4. **看三个地方，三个都对才算过**：
   - 窗口**最底下那条横条**（状态栏）变绿，写 `✓ 项目已保存：本地后台 · revision 1`；
   - 第一行按钮文字从「项目：无」变成「项目：本地后台」；
   - 白面板下半部分出现 `本地后台（https://example.com/ · phone 390×844）`（URL 带尾斜杠是规范化结果）。
   出现红条、按钮文字没变或列表空白 ⇒ 把终端文字贴我，先别做负例。
5. **看文件**（可选）：终端开头有 `[layress] 数据根：/…`，到该目录找 `config/projects.json`，
   应有 `layress.project-config/v1`、`"revision": 1` 和刚才的项目。

### 2. 负例：三次拒绝（不用重启，徽标仍是 `v20261115-2`）

> 前置：白色「项目设置」面板要开着；已经关了就点第一行「项目：本地后台」重新打开。
> 每次只看**窗口最底下那条横条**（状态栏），出现红条才算这条过。
> 每条做完把两个框改回 `本地后台` / `https://example.com`，再做下一条，别让上一条的坏值影响下一条。

**负例 1 · 空名称**

1. 点「项目名称」框 → 按 `⌘+A` 全选 → 按 `Delete` 清空（框里什么都不留）。
2. 点「新建项目并保存」。
3. 底部横条应变红：`✗ 已拒绝：CONFIG_INVALID_INPUT：…`（名称去空白后须为 1–80 个字符）。
4. 把名称填回 `本地后台`。

**负例 2 · 危险 scheme**

1. 点「起始地址」框 → `⌘+A` 全选 → 改成 `file:///etc/passwd`。
2. 点「新建项目并保存」。
3. 底部横条应变红，同样 `CONFIG_INVALID_INPUT`，错因里要有「scheme」；**不能**写成「地址缺少主机名」。
4. 把地址改回 `https://example.com`。

**负例 3 · 编辑中禁止采集**

1. 白面板保持开着，别点「关闭」。
2. 点工具条**第二行**的「采集」按钮（在「应用尺寸」右边）。
3. 底部横条应变红：`✗ 已拒绝：项目设置编辑中禁止采集；请先关闭设置`。
4. 这一步不生成截图，也不改 `projects.json`。

三次都做完，点面板里的「关闭」⇒ 白面板消失、页面回来；把这三条「看到 / 没看到」+ 终端文字贴我。
可选：先点「采集」进入静态预览，再点「项目：…」⇒ 红条提示先返回页面（不做这条也不影响 1.1 判定）。

### 3. 返回与未变

点「关闭」⇒ 面板消失、工具层回 96px、被测页面回到原位且尺寸不变（已打开样例页时核对「实际视口与引擎」数值）；
再点「项目：本地后台」可重新打开，状态仍是 revision 1。

### 4. 贴回给编写者

把终端从 `Running target/debug/layress-desktop` 开始的全部文字，加上第 1 / 2 / 3 节每步「看到 / 没看到」，
再加三句结论（白面板可见、状态栏反馈、关闭后页面不变）发我；
机器门禁 41/41 不替代人工（I11 / I16）。14:53 用户确认三条负例全通过，正例两次成功、关闭返回无异常 ⇒ **1.1 已勾选**（见 evidence-index「1.1 复验收口：勾选」）。

### 附：首轮复验失败与修复（2026-11-15 14:06）

首轮点「新建项目并保存」红条：`已拒绝：invalid args 'startUrl' for command 'config_create_project': command config_create_project missing required key startUrl`。
⇒ Tauri 2 把 Rust 命令的 `start_url` / `preset_id` 参数映射为前端 `startUrl` / `presetId`；前端误传 snake_case 键，命令收不到参数，**不是配置模型或存储失败**。
已修：前端改 camelCase 键；`check-build.mjs` 增加三条命令参数不变量断言（Rust 保持 snake_case、前端用 camelCase、不再传 snake_case 键值）；按纪律徽标再递增为 **`v20261115-2`**（`index.html` 同步）。
**重跑时以 `v20261115-2` 为准**；本轮机器门禁 41/41 与三条断言全绿，GUI 结果仍待人工复核。
