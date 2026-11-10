# Contributing Guide

<p><a href="CONTRIBUTING.md">简体中文</a> | English</p>

Thanks for taking part. This repository enforces **repository hygiene, documentation links and public bilingual pairing**. Read the relevant rules before your first PR.

## What to read first

1. [`AGENTS.md`](AGENTS.md) — the handoff index: one-line scope, status card, red lines, where pitfalls go.
2. [`docs/product/product.md`](docs/product/product.md) and [`docs/product/roadmap.md`](docs/product/roadmap.md) — current product requirements and the formal stage index.
3. [`docs/knowledge/decisions.md`](docs/knowledge/decisions.md) — settled technical choices and what would overturn them.
4. [`i18n/glossary.md`](i18n/glossary.md) — the authority for every term pairing.

Align ideas around the actual task and scope first. Preserve the sources of user feedback, observations and automated checks; an agent claiming a fix does not automatically complete acceptance.
Old invariant numbers remain historical references. Current engineering and storage constraints come from `AGENTS.md` and `openspec/config.yaml`.

## Environment setup

```bash
# Required once per clone: without the hook, the hygiene gate degrades into a late CI failure
git config core.hooksPath .githooks

node scripts/check-repo.mjs --all      # hygiene, all Markdown links, bilingual pairing and self-tests
node scripts/check-repo.mjs --staged   # check the staged snapshot, as pre-commit does
```

Node ≥ 20. **The current development machine is macOS arm64 / zsh**, while CI runs on ubuntu; the gate
scripts are platform-neutral and local and cloud run the same command chain (the development machine is
not the product's target platform). The pnpm workspace and `pnpm check:ci` do not exist yet (see
[`docs/development/workflow.md`](docs/development/workflow.md)), so there is no build or test step at this stage.

Tauri is the selected desktop framework; implementation will involve a Rust native layer and a TypeScript frontend. The toolchain, WebView prerequisites and setup steps will be verified in the implementation change and have not been installed in this round. Rust and frontend checks must join the same public gate chain.

## Submission flow

Behavior changes go through openspec (details in [`docs/development/workflow.md`](docs/development/workflow.md)):

1. Open an Issue, or align on the problem statement under an existing one.
2. Create `openspec/changes/<change-name>/`: `tasks.md` breaks down the steps, `design.md` records the approach and trade-offs.
3. Implement task by task, **making a local commit only when it is genuinely necessary or a small development slice is complete** (the executor judges whether a commit is worthwhile; do not commit just to pad the log); criteria and measured evidence stay inside the change directory.
4. Archive into `openspec/changes/archive/<date>-<change-name>/` and merge the delta into the main specs.

Documentation-only fixes, typos and link repairs may go straight to a PR without a change proposal.

## Repository hygiene

This repository strictly controls what enters history. **A committed blob is never really deleted**, so
rejection is enforced by a script rather than by reviewer vigilance.

| Rule | Triggers on | What to do |
|---|---|---|
| Scratch directories | `artifacts/`, `.layress/`, `output/`, `.tmp*`, `node_modules/` appearing in the staged area | Hand-written conclusions belong in the right `docs/` subdirectory; generated output just stays in the scratch directory |
| Binary / archive suffixes | `.msi .exe .dll .zip .7z .mp4` and similar | Commit the generator script instead, or register an exemption |
| Size | > 512 KB per file fails, > 256 KB warns | Split it, make it regenerable, or raise `size=` on the exemption |
| Credential shapes | private key blocks, `sk-…`, `ghp_…`, `AKIA…`, assignment-style tokens | Remove them; keys are entered by users inside the app, the repository keeps placeholders only |
| Machine-absolute paths | `C:\Users\…`, `/Users/<name>/…` or `/home/<name>/…` in source or config | Use a relative path or a runtime-resolved data directory |
| Ignored yet tracked | `.gitignore` says no but history says yes | `git rm --cached <file>` and re-commit |

**The only exemption route is [`.git-hygiene-allowlist`](.git-hygiene-allowlist), formatted `glob  #  reason`.**
An exemption without a reason fails outright. Before adding one, ask three things: can a script regenerate this?
Can it live in the right place under `docs/` or `datasets/`? Should it actually stay in a scratch directory —
in which case no exemption is needed?

Screenshots and acceptance evidence: `docs/reviews/**/*.png` is not committed by default. Register an
exemption for evidence directories you genuinely intend to publish.

## Bilingual documentation

- Documentation defaults to Chinese. Maintained bilingual versions currently cover only the root README, CONTRIBUTING, SECURITY, CODE_OF_CONDUCT and CHANGELOG; **edit both languages in the same round for these registered files**.
- `AGENTS.md`, `docs/`, the knowledge base, plans and `i18n/` maintenance notes need only Chinese. Add English for individual documents with a public readership and register them in [`i18n/docs-manifest.json`](i18n/docs-manifest.json).
- New unregistered English versions fail the gate. Retired internal translations are archived locally under `docs/drafts/`, are not committed and need no synchronization.
- For terminology changes, edit [`i18n/glossary.md`](i18n/glossary.md) first, then update the public bilingual documents using those terms. The glossary itself needs no English copy.
- Issue and PR forms are **not duplicated**: GitHub carries both languages inside one form, Gitee ships
  Chinese-only templates.

## Commit and PR conventions

- Title format `<type>(<scope>): <one-line change>`, e.g. `feat(reduce): re-verify failure after each dropped condition`.
  `type` ∈ `feat fix docs test refactor build chore`; `scope` is a package or directory name.
- Commit bodies are written in Chinese and explain **why**. For behavior changes, state before / after.
- One commit does one thing. Do not sweep scratch files in with an opportunistic `add -A`.
- **Pushing and tagging belong to the maintainers**: contributors push to their own fork; maintainers keep
  the two remotes in sync.
- In the PR, distinguish honestly: a machine PASS ≠ human visual review passed. Unexecuted review is listed
  as "not executed" — never checked off.

## Code of conduct and security disclosure

- Code of conduct: [`CODE_OF_CONDUCT.md`](CODE_OF_CONDUCT.md).
- Do **not** file security problems as public Issues: report privately per [`SECURITY.md`](SECURITY.md).
- Vulnerabilities involving the scanned application's page content (screenshot leakage, path traversal,
  outbound traffic to remote models) are treated as high severity and fixed first.
