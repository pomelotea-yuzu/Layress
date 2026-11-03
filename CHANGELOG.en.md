# Changelog

<p>简体中文 | <a href="CHANGELOG.md">English</a></p>

This file follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/) and
[Semantic Versioning](https://semver.org/). Change types are `Added / Fixed / Changed / Removed / Deprecated`.
**One entry per valuable commit**, rolled up into a release section when shipping.

## Unreleased

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

## Versioning and release conventions

- Versioning starts at `0.1.0`; breaking changes are allowed during `0.x` but must state their impact here.
- Candidates use `-rc.N`; packaging, hashes, acceptance records and release copy live in `docs/engineering/reports/`.
- **No tag and no npm publish before the first validation closes**: dataset, control groups and the
  continue/shrink thresholds are defined in [`docs/research/`](docs/research/README.md) and
  [`docs/knowledge/debt.md`](docs/knowledge/debt.md).
- Pushing and tagging belong to the maintainer, who keeps the two remotes (GitHub / Gitee) in sync.
