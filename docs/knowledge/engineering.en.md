# engineering.md · Build, test and gate pitfalls

<p>简体中文 | <a href="engineering.md">English</a></p>

> Entry format and upgrade rules: see [`README.md`](README.md). **Any entry not measured on Layress itself carries its source — never treat it as a fact verified in this repository.**

## Environment and line endings

- [2026-11-03][P0] Format checks light up a full screen of false positives on Windows while the repository content is actually fine ⇒ a machine-wide `core.autocrlf=true` checks the working tree out as CRLF, while the linters (Biome/ESLint) require LF ⇒ use [`.gitattributes`](../../.gitattributes) to force `eol=lf` on text and mark binaries/fixtures `-text` ⇒ Source: sibling project ReBaseAgent, one full-repository sweep; **not yet verified by an actual run in Layress**.

## Gates

- [2026-11-03][P1] `.gitignore` cannot stop garbage ⇒ it is an allow-by-default mechanism: one `git add -f` writes hundreds of MB of binaries into history and they never get cleaned out (a 374 MB installer actually sat in the sibling project's runtime directory) ⇒ the rejection logic must be executed by `scripts/check-hygiene.mjs` and run in both pre-commit and CI ⇒ evidence: 8 gate checks measured in this repository (scratch paths / size / `.env` / credentials / exemption without a reason / exemption with a reason / ignore leakage).
- [2026-11-03][P0] CI reports all green while the gate never ran ⇒ Gitee Go executes `commands` entry by entry and **does not short-circuit on a failure**, so an earlier red still yields an overall green; and the `build@nodejs` plugin may silently skip its own Node installation, leaving no usable `node` in the container ⇒ chain every check with `&&` into **one** command so the final exit code decides red or green; install Node inside that command and treat `nodeVersion` as a required placeholder only ⇒ Source: the sibling project's first run; this repository's [`.workflow/ci.yml`](../../.workflow/ci.yml) is already written that way.
- [2026-11-03][P1] The exemption list turns into an archaeological dig ⇒ a glob committed once ends up matching no file at all, and nobody dares delete it ⇒ `check-hygiene.mjs --all` reports zero-hit exemptions as warnings, so just delete them ⇒ judged only under `--all`; in staged mode this commit simply not touching that file is normal.

## Testing

- [2026-11-03][P0] Tests run in parallel show a wall of red, while reruns package by package are all green ⇒ parallel runs contend for ports / temp directories / singleton services ⇒ judge a regression only from single-package reruns and a serial full chain; parallel results are not evidence ⇒ Source: sibling project ReBaseAgent; **Layress follows this convention before building its test chain**.
- [2026-11-03][P1] "Full-suite green" cannot be audited ⇒ reporting only "passed" leaves no quantity to compare against ⇒ a full run must keep the triple: **file count + test count + the Errors line**, aligned item by item when judging a regression; small wobbles in the counts (host shims affecting the scratch directory) are judged by "everything collected passed + Errors=0", not by digit-exact equality.
- [2026-11-03][P1] With no jsdom, tests turn into "mirror assertions" ⇒ the test just retypes the implementation, so a real defect stays green anyway ⇒ without a DOM environment, write only **source-structure assertions** and state explicitly that they do not cover runtime behavior ⇒ Source: sibling project ReBaseAgent; equally applicable if Layress's detection modules run Node-side.

## Assertion literals

- [2026-11-03][P0] A literal inside an assertion is forever red or forever green, and the reason is invisible ⇒ writing a non-ASCII character directly in the argument (`U+FFFD`, emoji) gets swallowed into an empty string by the toolchain, so the assertion "contains an illegal character" becomes "contains an empty string" = always true ⇒ write every non-ASCII assertion argument as `\uXXXX` escapes and verify in a self-test that the escaping itself takes effect ⇒ Source: sibling project ReBaseAgent, proven empirically.

## Storage and portability (nothing on the C drive)

- [2026-11-03][P0] A product that calls itself portable still grows garbage on the C drive ⇒ moving only the "data directory" next to the product is not enough: on Windows `os.tmpdir()` = `%TEMP%` = `C:\Users\<user>\AppData\Local\Temp`, and Electron's `app.getPath('userData')` plus Chromium's `sessionData`/`Cache`/`GPUCache` default to the same C-drive user directory ⇒ all three must be overridden together: `app.setPath('userData', <data root>/userdata)`, `session.setPath('sessionData', ...)`, a self-created scratch directory passed explicitly to `--user-data-dir`; **using `os.tmpdir()` as the default value is forbidden** ⇒ Source: sibling project ReBaseAgent, whose `.gitignore` already records this trap as "one-off working directory (holding Electron sessionData cache and other temporary files)"; **pending verification by an actual run in Layress**.
- [2026-11-03][P0] Silent fallback to the user directory when the target directory is not writable ⇒ the common pattern is "if the write fails, fall back to `%APPDATA%`", which directly violates invariant I12, and the user has no idea at all where the data went ⇒ probe writability at startup; if it is not writable, raise an explicit error and require `--data-dir` — **no fallback allowed**.
- [2026-11-03][P1] The user cannot verify where the data lands ⇒ the resolved data root never appears in any output ⇒ the startup banner, the logs and the report header must all state `resolvedDataDir` and the writability check result (I13).
- [2026-11-03][P1] After a portable build changes machine or drive letter, the evidence paths break ⇒ configuration, reports and datasets stored absolute paths like `D:\...` ⇒ inside the repository and the artifacts everything is relative to the data root and uses posix separators; absolute paths are materialized only when exporting to the user. The gate's "machine absolute path" check exists as the backstop for exactly this.
- [2026-11-03][P1] Crashes when installed on a read-only medium (USB stick, optical disc) or into a directory policed by security software ⇒ there is no degraded path ⇒ explicitly support the combination "read-only product directory + user-specified writable data root" and state it clearly in the documentation.

## Dependencies and artifacts

- (to be filled) After the first `pnpm install`, record here the resolution / locking / offline-packaging traps actually hit.
