<div align="center">

# 🧪 Layress

<p>简体中文 | <a href="README.md">English</a></p>

**UI Stress Testing Tool · 界面压力测试工具**

<p>
  <a href="LICENSE"><img src="https://img.shields.io/badge/License-MIT-blue.svg" alt="License: MIT"/></a>
  <img src="https://img.shields.io/badge/Release-none-lightgrey.svg" alt="No release yet"/>
  <img src="https://img.shields.io/badge/portable-first-brightgreen.svg" alt="Portable first, never touches the system drive"/>
</p>

</div>

> **Current status: research and scoping are done. There is no code and no release.**
> This page describes the **target shape**. For what is actually finished, see the status card in
> [`AGENTS.md`](AGENTS.md) §two and `git log`.

## 🧭 What it is

Layress puts a running Web／Electron application under varying window sizes, content lengths, locales and
fonts, looks for the conditions under which the interface breaks, and **keeps the trigger conditions**.

The question is not "what changed versus the approved version" but:

> Under an explicitly stated UI state, apply pressure on purpose, confirm what damage happened to a
> **critical action**, and reduce the trigger conditions to evidence that can be replayed.

Four kinds of work must not be conflated. Layress targets the last two:

| Work | Core question | What the result means |
|---|---|---|
| Visual regression | What differs from the approved version? | A diff may be an intended update or a regression |
| Geometric anomaly detection | Which elements overflow, overlap, clip or rewrap? | Produces **candidates**, which do not yet prove a defect |
| Active stress testing | Which condition changes make the UI fail? | Widens input and state coverage |
| Defect confirmation and reduction | Does it really block use? What is the least that still fails? | Produces evidence a developer can act on |

An ideal report looks like this (**illustration, not a measurement**):

> On the settings page, at content width 873–906 CSS px, height 720 CSS px, English copy, sidebar expanded,
> the Save button is covered by the footer bar and still cannot be reached with the pointer after scrolling.
> The report carries the target element, the covering element, a screenshot, environment details and replay steps.

## 📌 Current status

- **Unreleased**: no downloadable package, no npm package, no tag.
- The repository is initialized and pushed to both remotes: GitHub
  [`pomelotea-yuzu/Layress`](https://github.com/pomelotea-yuzu/Layress) (primary) and
  [`yuzu-tea-duck/layress`](https://gitee.com/yuzu-tea-duck/layress) (mirror), `main` = `e02e703`;
  still no tag and no downloadable package.
- The repository holds the directory layout, the handoff index, the bilingual and hygiene gates, and the
  scoping survey.
- The first validation plan (real-project dataset, control groups, continue/shrink thresholds) is written up
  as a proposal and **not yet approved**.
- Open questions and debt are collected in [`docs/knowledge/debt.md`](docs/knowledge/debt.md).

## 🚀 Planned capabilities

Per the survey, scan / confirm / reduce / replay are separable standalone modules, with detection and
reporting fully open source:

| Module | Planned location | Responsibility |
|---|---|---|
| Scan | `packages/core` | Collect geometric and occlusion candidates under declared stress conditions |
| Confirm | `packages/confirm` | Use real hit-testing to decide whether a critical action is reachable; emit a three-way verdict |
| Reduce | `packages/reduce` | Drop conditions one at a time, re-verify failure after each drop, keep the minimal trigger |
| Replay | `packages/replay` | Replay archived records only, adding no real external side effects |
| Entry points | `packages/cli` + MCP / CI integration | Human-readable report plus structured results, embeddable in existing test scripts |
| Desktop | `apps/desktop` | Electron adapter and result browsing, deliberately later |

First-release scope (Windows／Chromium plus one Electron adapter) and the deferred list (aesthetic scoring,
auto-fix, full platform matrix, minimal-HTML extraction) are defined in survey §7.2 and are not re-promised here.

## ⌨️ Quick start

Only two commands run against this repository today:

```bash
node scripts/check-hygiene.mjs --all    # repository hygiene check
node scripts/check-i18n.mjs             # bilingual pairing check (--selftest counter-proofs the gate)
```

To take part in development, read [`AGENTS.md`](AGENTS.md), then
[`docs/development/workflow.md`](docs/development/workflow.md), then [`i18n/README.md`](i18n/README.md),
in that order. Once the CLI is installable, this section becomes real installation steps and four usage paths.

## 🧱 Project layout

```
Layress/
├─ AGENTS.md              handoff index (single entry point, always paired)
├─ i18n/                  bilingual policy · glossary · pairing manifest
├─ docs/
│  ├─ guide/ architecture/ development/     user-facing
│  ├─ knowledge/                            pitfalls · invariants · decisions · debt
│  └─ research/ product/ engineering/ reviews/  engineering process assets
├─ datasets/              positive and negative case dataset
├─ openspec/              config.yaml · specs/ · changes/{,archive/}
├─ packages/  apps/       code (layout decided, not implemented)
└─ scripts/               check-hygiene · check-i18n
```

Local output goes to exactly two places and is never committed: `artifacts/` (rerunnable output) and
`.layress/` (runtime data and logs).

## 📚 Documentation

- Handoff index and discipline: [`AGENTS.md`](AGENTS.md)
- Documentation map: [`docs/README.md`](docs/README.md)
- Knowledge base (pitfalls and closed decisions): [`docs/knowledge/README.md`](docs/knowledge/README.md)
- Scoping and market survey: [`docs/research/`](docs/research/README.md)
- Bilingual glossary: [`i18n/glossary.md`](i18n/glossary.md)

## 🤝 Contributing

Start with [`CONTRIBUTING.md`](CONTRIBUTING.md), especially the **repository hygiene** section: this repo
strictly controls what enters history, rejections are enforced by `scripts/check-hygiene.mjs`, and every
exemption must state a reason. GitHub issue forms carry both languages and can be filled in either;
the Gitee side ships Chinese-only templates (`.gitee/`).

## 📄 License and notices

- License: [MIT](LICENSE)
- **Storage discipline (never touch the C drive)**: the product is portable by default — data, logs, caches
  and temporary files live **next to the product** (its data root, or a directory you name), and are never
  written to `%APPDATA%`／`%LOCALAPPDATA%` or the registry. You choose the install location; the installer
  does not elevate or edit the system PATH. Copying the directory migrates everything; deleting it leaves
  nothing behind. If a write fails the product errors out and asks for a directory — **no silent fallback to
  the user directory** (see invariants I12–I13).
- Security and privacy: [`SECURITY.md`](SECURITY.md) — local-first is a product commitment; the scanned
  application's page content stays on your machine by default.
- Code of conduct: [`CODE_OF_CONDUCT.md`](CODE_OF_CONDUCT.md)
- Changelog: [`CHANGELOG.md`](CHANGELOG.md)
- Naming: `Layress` = Layer + Stress. Unrelated to the npm packages `layrr` and `layerr` or to the `layui`
  component library — do not install the wrong one.
