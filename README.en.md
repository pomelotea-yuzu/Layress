<div align="center">

# 🧪 Layress

<p><a href="README.md">简体中文</a> | English</p>

**Local UI Acceptance Assistant · 本地界面验收助手**

<p>
  <a href="LICENSE"><img src="https://img.shields.io/badge/License-MIT-blue.svg" alt="License: MIT"/></a>
  <img src="https://img.shields.io/badge/Release-none-lightgrey.svg" alt="No release yet"/>
</p>

</div>

> **Current status: the first Tauri page-host increment (add-tauri-page-host) has been implemented, manually verified and archived on 2026-11-14, with its spec merged into the main `desktop-page-host` spec; there is still no installable release and the product is not usable end to end yet.**
> This page describes the intended experience, not delivered features. See [`AGENTS.md`](AGENTS.md) §two and `git log` for actual progress.

## 🧭 What it is

Layress helps individuals and small teams using AI agents to build Web / Electron interfaces share acceptance tasks, issue evidence and recheck results between people and agents.

The shortest human workflow is: **see the interface → mark a region → add a short note → hand it to an agent → review the result**.
For example, mark the Save button and write “the footer covers this; saving should remain possible in a smaller window.” The tool assembles the scene, location and available context to reduce explanation and lookup work.

The first version is a **Tauri desktop workbench + shared task core + CLI**. Operate the target page, select a device and add feedback in one main window; MCP follows concrete needs.
Tauri uses system WebViews. Capture and emulation capabilities will be published after platform testing; a second browser is not bundled by default.
Supported target applications and agent connections must be verified individually. Arbitrary websites, frameworks and clients are not promised to work out of the box.

## 📌 Current status

- No installable product, npm release or tag. The first change, [add-tauri-page-host](openspec/changes/archive/2026-11-14-add-tauri-page-host/proposal.md), has been implemented and archived: single-window three-WebView layout (toolbar / page under test / status bar), HTTP(S) navigation with permission isolation, basic viewport, real WKWebView snapshots with in-place preview, and a portable data root. The capability spec lives at [`openspec/specs/desktop-page-host/`](openspec/specs/desktop-page-host/spec.md).
- The [product direction and usage](docs/product/product.md), Tauri desktop framework and [formal development roadmap](docs/product/roadmap.md) are confirmed. The rest of the [stage-one plan](docs/product/2026-11-10-stage-one-plan.md) remains under review; follow-up increments require a new change proposal first.
- Existing assets include documentation, directory ownership, public research and engineering knowledge, plus hygiene, link, bilingual, build and unit-test gates.
- GitHub [`pomelotea-yuzu/Layress`](https://github.com/pomelotea-yuzu/Layress) is the primary repository; Gitee [`yuzu-tea-duck/layress`](https://gitee.com/yuzu-tea-duck/layress) is the mirror. Remote synchronization must be checked live; agents do not push.
- The 11-03–11-09 phase is [closed](docs/research/2026-11-09-phase-closure.md) and remains available as historical reference. Its validation gates, four-module architecture and desktop platform commitments do not automatically carry forward.

## 🚀 Planned capabilities

| Capability | Intended experience |
|---|---|
| Visual feedback | Start from a scene or captured image, mark a location and add a short note without mandatory selectors, JSON or test cases |
| Human / agent collaboration | The workbench and CLI share tasks and evidence; agents retrieve context and return their handling results |
| Recheck after a fix | Connect the original issue with the updated scene; distinguish execution, failure and pending review. An agent claiming a fix does not automatically complete acceptance |
| Repeatable acceptance | Gradually save reusable checks and setup steps, incorporating established automated checks |

Detailed designs are written when needed; see the [roadmap index](docs/product/roadmap.md). Tauri is the chosen desktop framework; specific WebView integration, adaptation order and release dates remain open.

## ⌨️ Repository checks

Node ≥ 20. Only repository gates are currently runnable:

```bash
node scripts/check-repo.mjs --all      # local checks and CI
node scripts/check-repo.mjs --staged   # check the actual staged snapshot, as pre-commit does
```

Set `git config core.hooksPath .githooks` once per clone. Real product installation and usage commands will be documented when runnable.

## 🧱 Project layout

- [`AGENTS.md`](AGENTS.md): the single handoff entry, status and engineering rules.
- [`docs/product/`](docs/product/README.md): current product requirements, roadmap index and detailed designs.
- [`docs/`](docs/README.md): guides, architecture, workflow, knowledge, research and process records.
- [`packages/`](packages/README.md) / [`apps/`](apps/README.md): ownership placeholders; package and app directories await design, without inheriting the old four-module split.
- `scripts/` / `i18n/` / `openspec/` / `datasets/`: gates, bilingual policy, change workflow and case ownership.
- [`artifacts/`](artifacts/README.md): disposable output directory; only the placeholder note inside it is committed, everything else is not.

Disposable output belongs only in `artifacts/`; runtime data and logs belong in `.layress/`. Neither is committed.

## 📚 Documentation

- Current product: [product direction and usage](docs/product/product.md)
- Formal stage index: [development roadmap](docs/product/roadmap.md)
- Documentation and experience: [documentation map](docs/README.md) · [knowledge base](docs/knowledge/README.md)
- Development and terminology: [workflow](docs/development/workflow.md) · [glossary](i18n/glossary.md)

## 🤝 Contributing

Read [`CONTRIBUTING.md`](CONTRIBUTING.md) and [`AGENTS.md`](AGENTS.md). Registered public documents are updated in both languages together; internal documentation defaults to Chinese.
The first Tauri page-hosting change has been created. The rest of stage 1 and stage 2 will be discussed separately; creating change documents does not mean implementation is complete.

## 📄 License and commitments

- License: [MIT](LICENSE). The `Layress` brand is retained, unrelated to the npm packages `layrr` / `layerr` and the `layui` component library.
- **Local-first**: tasks, page material and screenshots stay local by default. Disclose data flow and cost when handing them to remote agents or models; never send them silently. See [`SECURITY.md`](SECURITY.md).
- **Portable-first**: data, logs, caches and temporary files stay under the product directory or a directory explicitly selected by the user. Never default to system user directories, elevate privileges, change the system PATH or silently fall back when writing fails. Development environment installation is separate from product storage commitments; see [`AGENTS.md`](AGENTS.md) §seven.
- Conduct and version history: [`CODE_OF_CONDUCT.md`](CODE_OF_CONDUCT.md) · [`CHANGELOG.md`](CHANGELOG.md).
