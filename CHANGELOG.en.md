# Changelog

<p><a href="CHANGELOG.md">简体中文</a> | English</p>

This file follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/) and
[Semantic Versioning](https://semver.org/). Change types are `Added / Fixed / Changed / Removed / Deprecated`.
**One entry per valuable commit**, rolled up into a release section when shipping.

## Unreleased

### Development machine moved to macOS and cross-platform gate hardening · 2026-11-08

- Changed **development machine assumptions**: the main machine moved from Windows to macOS arm64 / zsh,
  local checkout at `~/Developer/layress`. `AGENTS.md` §two/§four, `openspec/config.yaml`,
  `docs/development/workflow.md` and CONTRIBUTING were updated; **development machine ≠ product target
  platform** is now explicit — the three-platform distribution promise (D15) and the first-release scope in
  survey §7.2 are unchanged.
- Changed **storage discipline naming**: invariant I12 "never touch the C drive" is now "portable first,
  never write to system user directories" with no change to the promise. Banned paths now cover macOS
  `~/Library`／`/private/var/folders` and Linux XDG directories; public bilingual documents, the glossary,
  invariants, knowledge base and plans were aligned in the same round. The 2026-11-03 entries above keep
  the historical name verbatim.
- Added **cross-platform gate rules**: the machine-absolute-path rule in `check-hygiene.mjs` now catches
  POSIX `/Users/<name>/`／`/home/<name>/`; the system-user-directory rule adds macOS Library,
  `/private/var/folders`, Linux XDG, `os.tmpdir()` and more Electron default paths. `--selftest` gained
  three counter-examples (macOS Library, Linux XDG, POSIX absolute path).
- Changed **first-validation host platform**: "run it on Windows first" lapsed with the machine move and is
  now **to be settled when the first validation plan is revised**, registered in
  [`docs/knowledge/debt.md`](docs/knowledge/debt.md); the packaging plan, roadmap and validation proposal
  follow suit, with unexecuted items still listed separately.
- Changed **local commit discipline**: the agent commits locally only when genuinely necessary/valuable or
  when a small development slice is complete; the model judges whether a commit is worthwhile.
  **Pushing and tagging still belong to the maintainer.**
- Fixed **pre-commit hook ignored on macOS**: `.githooks/pre-commit` was committed with mode `100644`;
  Windows does not rely on the executable bit, but Git on macOS skips such a hook outright, so the local
  hygiene chain never ran and only CI was left to catch problems. Changed to `100755`; `git hook run
  pre-commit` now passes. The gate also gained a "hook missing executable bit" assertion and a counter-example
  so this cannot recur.
- Changed **remote naming**: checking unpushed commits no longer assumes a remote name — use
  `git fetch --all && git log --oneline '@{u}'..main`. `AGENTS.md` and `openspec/config.yaml` now state that
  remote names come from `git remote -v`: the current macOS clone uses a single `origin` aggregating Gitee
  fetch/push and a GitHub push URL, while the historical "no origin" note only described the initial setup.
  The `git push github main && git push gitee main` line in `workflow.md` failed for the same reason and now
  uses `git push origin main` (multiple push URLs in one command), with explicit URLs as a fallback.
- Added a **knowledge-base write-back rule**: problems found during development, disproved assumptions and
  policy changes must be written into the matching `docs/knowledge/` topic in the same round.
  `.workbuddy/memory/` is a local daily log only — other agents and people who clone the repository cannot
  see it, so writing there alone is the same as not writing at all. The rule now lives in `AGENTS.md` §six,
  the knowledge-base README and the commit checklist; this round's pitfalls (such as hard-coded remote
  names breaking a documented command) were backfilled as well.

### Repository skeleton · 2026-11-03

- Added directory layout with ownership notes: `docs/{guide,architecture,development,knowledge,research,product,engineering,reviews}`,
  `datasets/`, `openspec/{config.yaml,specs,changes/archive}`, `packages/`, `apps/`, `scripts/`.
- Added the handoff index [`AGENTS.md`](AGENTS.md): one-line scope, status card, reading order, hygiene red lines,
  knowledge entry format, directory map. The key difference from the sibling project is that the
  **knowledge base is committed** (`docs/knowledge/`) instead of living only in a local agent memory directory.
- Added the repository hygiene gate `scripts/check-hygiene.mjs`: default-deny on size, binary suffixes, scratch
  directories, credential shapes and ignored-yet-tracked files; run by both `.githooks/pre-commit` and CI;
  exemptions only via `.git-hygiene-allowlist` and each must carry a reason. Eight positive and negative cases
  verified (junk blocked, legitimate additions allowed, reason-less exemptions fail).
- Added the i18n structural layer: the bilingual policy [`i18n/README.md`](i18n/README.md), the frozen glossary
  [`i18n/glossary.md`](i18n/glossary.md), the pairing manifest `i18n/docs-manifest.json`, and the pairing gate
  `scripts/check-i18n.mjs` (missing twin / broken link / section drift / orphan English, plus `--selftest`).
- Added paired front-door documents in both languages: [`README.md`](README.md), [`CONTRIBUTING.md`](CONTRIBUTING.md),
  [`SECURITY.md`](SECURITY.md), [`CODE_OF_CONDUCT.md`](CODE_OF_CONDUCT.md), and this file.
- Added engineering conventions: `.gitignore` (every block states why it is excluded and where valuable content
  belongs), `.gitattributes` (LF forced, binaries and fixtures conversion-proof), `.editorconfig`, and the five
  `docs/knowledge/` topics.
- Added dual-track CI and templates: `.github/workflows/ci.yml`, `.github/ISSUE_TEMPLATE/` (both languages inside
  one form), `.github/PULL_REQUEST_TEMPLATE.md`, `.gitee/` (Chinese-only templates), `.workflow/ci.yml` (Gitee Go).
- Added **storage discipline (never touch the C drive)**: invariants I12–I13 define portable-first data,
  data living next to the product, and erroring out instead of silently falling back; written into
  `AGENTS.md` §seven, `openspec/config.yaml` and the README. `check-hygiene.mjs` gained a "system user
  directory" rule that fails product code containing `%APPDATA%`／`process.env.LOCALAPPDATA`／
  `getPath('userData')`／`C:\Users\` shapes (verified: red in product code, warning-only in scripts).
  `docs/knowledge/engineering.md` records the knock-on trap — `os.tmpdir()` and Chromium `sessionData`
  also default to the C drive, so moving the "data directory" alone is not enough.
- Added dual-remote registration: primary `https://github.com/pomelotea-yuzu/Layress` and mirror
  `https://gitee.com/yuzu-tea-duck/layress` (both empty — `git ls-remote` shows no ref on either side);
  `AGENTS.md` status card gained "push status is judged only from `git ls-remote`", `openspec/config.yaml`
  records the remotes and the "no origin" policy, `SECURITY.md` now points at the real private advisory entry,
  and `.github/ISSUE_TEMPLATE/config.yml` carries four contact_links (Gitee mirror / docs / handoff index / security).
- Changed the scoping survey, moved out of the repository root into [`docs/research/`](docs/research/README.md).
- Not done code scaffolding (`package.json`, pnpm workspace, biome, test chain) — this round's scope is recorded in
  [`docs/knowledge/decisions.md`](docs/knowledge/decisions.md) D13; it is planned, not an oversight.

### First validation plan proposal and repository go-live · 2026-11-03

- Added [`docs/engineering/plans/2026-11-03-first-validation-plan.md`](docs/engineering/plans/2026-11-03-first-validation-plan.md):
  survey §seven turned into something executable — S0 probe-channel comparison experiment, S1 dataset build,
  S2 fair three-arm control (A/B/C), S3 metrics and human review, S4 go/shrink verdict; suggested timebox
  6 weeks / ~90 hours (S1 takes 40% — cut features, never the sample size); plus "the six calls the owner must
  make" and "what failure looks like" (shrink conditions pre-written so the outcome cannot be rationalised later).
  **Status: proposal, not approved.**
- Changed the probe-stack stance: **no preset conclusion** — it comes out of the S0 experiment
  (R1 Playwright + system Chrome vs R2 raw CDP; 3 known-answer samples × 7 measurements × 6 repeats), with
  "zero byte growth in the C-drive user profile" (invariant I12) listed as a hard criterion. The criteria are
  written before the experiment, so results cannot be post-hoc justified.
- Added repository go-live: `git init -b main`, `core.hooksPath=.githooks` active, dual remotes `github`
  (primary) + `gitee` (mirror) with no `origin`; the skeleton landed in 6 staged commits and the owner pushed
  `main` to both sides (last one `e02e703`, verified identical via `git ls-remote`).
- Changed [`AGENTS.md`](AGENTS.md) status card, [`README.md`](README.md) current status and
  [`docs/knowledge/debt.md`](docs/knowledge/debt.md) to reflect the real init/push state, and removed the
  "first commit and pushing to the two remotes" row from the debt table — settled items do not stay in debt.

## Versioning and release conventions

- Versioning starts at `0.1.0`; breaking changes are allowed during `0.x` but must state their impact here.
- Candidates use `-rc.N`; packaging, hashes, acceptance records and release copy live in `docs/engineering/reports/`.
- **No tag and no npm publish before the first validation closes**: dataset, control groups and the
  continue/shrink thresholds are defined in [`docs/research/`](docs/research/README.md) and
  [`docs/knowledge/debt.md`](docs/knowledge/debt.md).
- Pushing and tagging belong to the maintainer, who keeps the two remotes (GitHub / Gitee) in sync.
