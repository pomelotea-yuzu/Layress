# Bilingual glossary

<p>简体中文 | <a href="glossary.md">English</a></p>

Version: 1 (frozen 2026-11-03). Applies to: both language versions of the repository front door and
`docs/`, and — once built — the `zh-CN` / `en-US` resources behind desktop UI, human-readable CLI
output and error copy. **Code identifiers, protocol fields, error codes, CLI flags and file names are
never translated.**

Basis: the four-work boundary (§1.2) and the proposed positioning (§5.2) of
[`docs/research/2026-11-03-web-electron-ui-stress-test-tool-market-and-open-source-research.md`](../docs/research/2026-11-03-web-electron-ui-stress-test-tool-market-and-open-source-research.md).
**One concept, one rendering.** To change a term, edit this table first, then sweep the documents —
not the other way round.

## 1. Product and boundaries

| Concept | 中文 | English | Constraint |
|---|---|---|---|
| Product category | 界面压力测试 | UI stress testing (layout) | Qualifier required on first mention; **not** load/performance stress testing |
| Core promise | 关键操作可达性 | critical-action reachability | The product's central term; do not render as "usability" or "accessibility" |
| Visual regression | 视觉回归 | visual regression | Diff against an approved version; this product ships no such baseline, do not conflate |
| Geometric anomaly detection | 几何异常检测 | geometric anomaly detection | Yields **candidates** only, see §3 |
| Active stress testing | 主动压力测试 | active stress testing | Changing conditions to widen coverage; distinct from passive observation |
| Confirmation and reduction | 缺陷确认与复现缩减 | defect confirmation and reduction | The centre of gravity here; "reduction", never "simplification" |
| Golden baseline | 正确截图基线 | golden screenshot baseline | Explicitly **not required** by this product; keep the negative phrasing |
| Local-first | 本地优先 | local-first | Data stays on the machine; remote models require a stated data flow and cost |

## 2. Stress conditions

| Concept | 中文 | English | Constraint |
|---|---|---|---|
| Viewport width | 视口宽度 | viewport width | Always CSS px, never physical pixels |
| Content width | 内容区宽度 | content width | Distinguish from viewport width; the squeezed one is this |
| Narrow window | 窄窗口 | narrow window | A condition, not a defect name |
| Short window | 低窗口 | short window | The height dimension |
| Long-text injection | 长文本注入 | long-text injection | Name the real source: long file names, long paths |
| Font fallback | 字体回退 | font fallback | Recorded separately from font scaling |
| Text scaling | 文字缩放 | text scaling / font zoom | State whether browser zoom or OS zoom |
| Locale switch | 语言切换 | locale switch | Copy reflow changes container height — a condition, not a result |
| Setup steps | 准备步骤 | setup steps | Required repro field; not "preconditions" |
| UI state | 界面状态 | UI state | Sidebar expanded, tab active, error toast shown; always recorded with the conditions |
| Exclusion scope | 排除范围 | exclusion scope | What the developer declares out of scope |
| Acceptable scrolling | 可接受滚动 | acceptable scrolling | Needing to scroll is not a failure; scrolling and still not reaching it is |

## 3. Detection and verdicts

| Concept | 中文 | English | Constraint |
|---|---|---|---|
| Candidate | 候选（几何候选） | candidate | **A candidate is not a defect**; reports must keep this layer of wording |
| Overflow | 溢出 | overflow | Content exceeds its container |
| Clipping | 裁切 | clipping | Cut off by the container, visually gone |
| Occlusion | 遮挡 | occlusion | Covered by another element; record the covering element |
| Overlap | 重叠 | overlap | Two elements pressing on each other |
| Line-wrap growth | 换行导致的高度增长 | line-wrap growth | The mechanism behind dialogs growing taller after a locale switch |
| Verdict | 判定 | verdict | Outcome of one confirmation: confirmed / not-a-defect / inconclusive |
| False positive | 误报 | false positive | A candidate judged a defect when it is not |
| False negative | 漏报 | false negative | A real defect that went undetected |
| False green | 假绿 | false green | **The check itself is broken** yet reports pass; a tool defect, not a measurement outcome |
| Human review | 人工复核 | human review | A machine PASS ≠ human approval; unexecuted review is listed separately |
| Aggregation | 缺陷聚合 | defect aggregation | Merging several alerts that share one root cause |
| Ignore entry | 忽略登记 | ignore entry | A confirmed-won't-fix item, and it must carry a reason |

## 4. Evidence and replay

| Concept | 中文 | English | Constraint |
|---|---|---|---|
| Scan | 扫描 | scan | One of four independently usable modules |
| Confirm | 确认 | confirm | Same |
| Reduction | 条件缩减 | condition reduction | Shrink trigger conditions until it still fails; **no promise** of auto-extracting minimal HTML |
| Replay | 回放 | replay | Replays an existing record only; **adds no** real external side effects |
| Rerun | 重跑 | rerun | Actually executes; never interchangeable with replay in wording |
| Trigger conditions | 触发条件 | trigger conditions | Beyond the width band: height, state, font, scaling |
| Replayable evidence | 可回放证据 | replayable evidence | The product's final deliverable |
| Run record | 运行记录 | scan run / run | One archived scan, environment and artifact list included |
| Negative case | 负例 | negative case | Samples that look broken but are fine: ellipsis, scroll areas, drawers, overlays |
| Positive case | 正例 | positive case | Real historical defect samples |
| Dataset | 数据集 | dataset | Tuning subset and final evaluation subset **must be separated** to prevent overfitting |

## 5. Never translated

These stay byte-identical in both languages:

- Code and protocol: class and field names, error codes, event names, CSS property names such as `viewport` or `overflow`
- Command line: package names `@layress/*`, CLI flags and subcommands, environment variables `LAYRESS_*`
- Tools and formats: CDP method names, `DOM.getNodeForLocation`, `Screenshot.captureScreenshot`, `.jsonl`, `.png`
- Units and numbers: CSS px, DPI, DPR, percentages, durations, byte counts
- File names and paths, including sample names under `docs/` and `datasets/` — rename a sample and its
  content hash no longer matches what a replay expects

## 6. Storage and distribution

| Concept | 中文 | English | Constraint |
|---|---|---|---|
| Portable build | 便携包 / 便携版 | portable build | Copying the directory migrates it; deleting it leaves nothing behind — **not** a marketing synonym for "no installer" |
| Data root | 数据根 | data root | The data root under the product's own directory, or a user-specified one; resolution must be visible (I13) |
| Data directory | 数据目录 | data directory | Distinguish from install location: the former travels with the product, the latter is the user's choice |
| Install location | 安装位置 | install location | Chosen by the user; the installer must not elevate or edit the system PATH |
| User data directory | 用户数据目录 | user data directory (`userData`) | Electron/Chromium term; defaults to `C:\Users\…\AppData`, so this product **must** override it. The identifier `userData` stays raw |
| Session data | 会话数据 | `sessionData` | Same for the Chromium cache directory; keep the field name raw |
| Scratch directory | 临时目录 | scratch / temp directory | Must live under the data root; `os.tmpdir()` defaults to the C drive and **must not be used as the default** |
| Read-only medium | 只读介质 | read-only medium | Support the combination "product directory read-only + user-chosen writable data root" |
| Silent fallback | 静默回落 | silent fallback | Falling back to the user directory when a write fails; **forbidden** here, see invariant I12 |
