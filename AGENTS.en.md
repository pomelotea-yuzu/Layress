# Layress · Handoff index (AGENTS.md)

<p>简体中文 | <a href="AGENTS.md">English</a></p>

> **This is the single entry point.** Any developer, any agent — after reading this file you should be
> able to start working.
> Tool-local memory files (`CLAUDE.md`, `.cursor/`, `.trae/`, …) are never committed; if your tool only
> recognizes those, make it point here — **do not copy the content elsewhere**, because two copies of a
> policy diverge sooner or later.
>
> **Size budget**: this file stays under 300 lines / 18 KB. If it does not fit, the content belongs in
> [`docs/knowledge/`](docs/knowledge/README.md), not in another paragraph here. Enforcing the budget is a
> **human review** job, outside `scripts/check-hygiene.mjs` (the script does not count lines).
>
> **Authority order** (leftmost wins on conflict):
> `packages/*/src` source code > `openspec/changes/archive/` > `openspec/specs/` > `docs/` > this file.
> ⚠️ This file **is known to lag**: judge real progress only from `git log` and `openspec/changes/archive/`.

---

## 1. The project in one line

**Layress** — a UI stress-testing tool. It puts a running Web／Electron application under different window
sizes, content lengths, locales and fonts, **confirms what damage happened to a critical action**, and
reduces the trigger conditions to evidence that can be replayed.

Positioning: local-first, no golden screenshot baseline required, focused on **critical-action reachability**,
shipping reduced trigger conditions plus replayable evidence.

- Survey and scoping basis: [`docs/research/2026-11-03-web-electron-ui-stress-test-tool-market-and-open-source-research.md`](docs/research/2026-11-03-web-electron-ui-stress-test-tool-market-and-open-source-research.md)
- Four boundaries (**never conflate them**; memorize before writing code): visual regression ≠ geometric
  anomaly detection ≠ active stress testing ≠ defect confirmation and reduction.
  This product centers on the last two; a geometric anomaly is only a candidate, and **a candidate is not a defect**.

## 2. Current status card

> On every handoff, **update these four rows first**, then work. The fields are fixed — do not turn them
> into free-form narrative.

| Field | Current value |
|---|---|
| Phase | Scoped and approved as a direction; first validation plan not approved |
| Active change | None (`openspec/changes/` is empty) |
| Code | None. The repository holds skeleton, index and gates only |
| Release | None. Both remotes exist but are **empty** (nothing pushed), no tag, no npm package |

Where truth actually lives: progress ⇒ `git log --oneline` and `openspec/changes/archive/`;
gate baseline ⇒ the printed output of `node scripts/check-hygiene.mjs --all`; repository size ⇒ `git count-objects -v`;
**push status ⇒ only `git ls-remote` output counts, never what this file claims**.

Repository and remotes: local `D:\Layress`; branch `main`;
`github` = `https://github.com/pomelotea-yuzu/Layress.git` (primary) and
`gitee` = `https://gitee.com/yuzu-tea-duck/layress.git` (mirror) — **there is no `origin`**.
`git init` has not been run locally; creating repositories, tags and pushing are the owner's operations
(measured 2026-11-03 with `git ls-remote`: neither remote has any ref).

## 3. Required reading (about 10 minutes)

1. This file, §1 §2 §5 §6.
2. [`openspec/config.yaml`](openspec/config.yaml) — the project bible: positioning, stack, **architecture invariants**, execution boundaries.
3. [`docs/knowledge/README.md`](docs/knowledge/README.md) — the knowledge base layout and **entry format** (this decides whether the file is still useful in a year).
4. [`docs/development/workflow.md`](docs/development/workflow.md) — gate chain, openspec process, commit and push policy.
5. [`docs/README.md`](docs/README.md) — the documentation map: where a new document goes and how it is named.
6. [`i18n/README.md`](i18n/README.md) — bilingual policy and pairing registration; **check whether a document needs a twin before writing it**.
   Term pairings always come from [`i18n/glossary.md`](i18n/glossary.md).
7. The survey itself (linked in §1) — first-release scope, dataset requirements, control design and the
   continue/shrink thresholds are all defined there.

## 4. Getting running

**No pnpm workspace yet** (this round's scope: structure + index + gates + i18n). Only three commands are
real right now:

```bash
node scripts/check-hygiene.mjs --all      # whole-repository hygiene check
node scripts/check-hygiene.mjs --staged   # staged-area check (what pre-commit runs)
node scripts/check-i18n.mjs               # bilingual pairing check (--selftest counter-proves the gate is not false green)
```

Once per clone (skip this and the gate effectively does not exist):

```bash
git config core.hooksPath .githooks
```

After the workspace skeleton lands, build `pnpm check:ci` as described in `docs/development/workflow.md` and
wire `check:ci`, `check-hygiene --all` and `check-i18n` into both pre-commit and CI —
**cloud and local must run the same command chain**, otherwise "green locally, red remotely" eats the evening.

## 5. Repository hygiene red lines

Owner's policy: **strictly control what gets pushed; the repository stays clean; no junk.** Enforced in
three layers — remove any one and it gets breached:

| Layer | Location | Nature |
|---|---|---|
| Backstop ignores | [`.gitignore`](.gitignore) | Every block must state "what this is" + "where valuable content goes instead". Allow-by-default — **not a defense** |
| Machine gate | [`scripts/check-hygiene.mjs`](scripts/check-hygiene.mjs) | **Default-deny**: size, suffixes, scratch paths, credential shapes, ignore leakage |
| Auditable exemptions | [`.git-hygiene-allowlist`](.git-hygiene-allowlist) | `glob + one-line reason`; **no reason = gate failure**; CI also flags dead exemptions |

Six things not to touch:

1. **Two scratch directories, only these two**: `artifacts/` (one-off output, regenerable by rerunning) and
   `.layress/` (runtime data and logs). Hand-written conclusions, review records and plans must **never** be
   dropped there "and moved later" — write them straight into the right `docs/` subdirectory.
   (The sibling project's `output/` rotted exactly because of "put it here for now"; this repo refuses a third dump site.)
2. **No binaries in history.** A committed blob is never really deleted. Installers, captures, recordings and
   model files go through an exemption with a reason, or you commit only the generator script.
3. **Zero tolerance on credentials**: keys are entered by users inside the app, the repository holds
   placeholders only; `.env*`, `.mcp.json` and local config containing absolute paths are never committed.
4. **Screenshots stay local by default**: `docs/reviews/**/*.png` is ignored. Only evidence you genuinely
   intend to publish gets `add -f`.
5. **Before any `git add -f`**, ask three questions: can a script regenerate this? Can it live in the right
   place under `docs/` or `datasets/`? Should it actually stay in a scratch directory — then no exemption is needed?
6. **Pushing belongs to the owner.** An agent commits after every valuable step and **does not push**;
   remotes and tags are the owner's operation.

## 6. How to write pitfalls and experience (the most important mechanism here)

In the sibling project about 90% of accumulated experience sat in a **local memory directory**, excluded from
git by a whole-directory rule ⇒ anyone cloning the repo never saw it. This repository **commits** the same
material and fixes the format, so it does not decay into a changelog nobody reads.

Location: [`docs/knowledge/`](docs/knowledge/README.md), five topics — **by subject, not by date**.

| File | Collects |
|---|---|
| `engineering.md` | Pitfalls in builds, tests, gates, dependencies, openspec usage |
| `runtime-probe.md` | Live-probe hard facts: CDP, windows, DPR, IME, timing, false-green criteria |
| `invariants.md` | **Non-reopenable architecture invariants**, each with the consequence of violating it |
| `decisions.md` | Decision lookup: what was decided, why, at what cost |
| `debt.md` | Open policy questions and debt, with owner and deadline |

Four writing rules:

1. **One line, one subject**, fixed format: `- [YYYY-MM-DD][level] phenomenon ⇒ root cause ⇒ workaround ⇒ evidence`.
   Levels: `P0` fatal / `P1` causes a wrong verdict / `P2` wastes time.
2. **Conclusions only, not the process.** The process lives in the change's `design.md` / `evidence-index.md`;
   this line keeps only a pointer. A dated log line packing several hundred characters makes the next person skip the whole file.
3. **A pitfall hit twice gets promoted**: out of a bullet and into `invariants.md`, or straight into a gate /
   script assertion. Something guarded only by memory will be hit again; only what code blocks is actually blocked.
4. **A machine PASS is not human approval.** Visual and interaction verdicts belong to human review;
   unexecuted items are **listed separately**, never checked off, never written into the status card as delivered.

## 7. Policy and discipline

- **Language**: all output (code comments, docs, commit messages) defaults to Chinese; the product name `Layress` stays English.
- **Storage discipline: never touch the C drive** (an explicit owner requirement, same rank as hygiene):
  - The product is **portable-first**: data, logs, caches and temporary files are written into **the product's
    own directory** (by default `.layress/`, or `data/`) or a directory the user explicitly names.
  - It must **not** write by default to `C:\Users\<user>\AppData\*`, `%APPDATA%`, `%LOCALAPPDATA%`, the
    registry, or `~/Library/Application Support`. The user chooses the install location; the installer does not
    elevate and does not edit the system PATH.
  - **Data travels with the product**: copying the whole directory completes the migration; deleting it leaves nothing behind.
  - The knock-on trap (read this): on Windows `os.tmpdir()` and Electron `app.getPath('userData')` / Chromium
    `sessionData` **all default to the C drive user profile**, so moving the "data directory" alone is not enough ⇒
    see [`docs/knowledge/engineering.md`](docs/knowledge/engineering.md) and invariants I12–I13.
  - The resolved data directory must be **printed** in reports, logs and the UI, otherwise the user cannot verify this promise.
- **Bilingual is a structural layer, not a translation done later** (details in [`i18n/README.md`](i18n/README.md)):
  - Every `.md` in the "public front door" or "handoff-critical" set must be paired with a same-directory
    `*.en.md`, committed **in the same round**, and registered in [`i18n/docs-manifest.json`](i18n/docs-manifest.json).
    Chinese-only and unregistered = allowed; registered without a twin = `check:i18n` goes red.
  - Forms (GitHub issue forms, PR template) are **not duplicated**: both languages sit side by side inside one
    form (`label: English / 中文`), and Gitee gets its own Chinese-only templates (`.gitee/*.zh-CN.md`).
  - Process assets (`docs/research`, `product`, `engineering`, `reviews`, `drafts`) are **not required to be
    paired** — their value is the process itself, and force-translating only manufactures maintenance debt.
  - Term pairings follow [`i18n/glossary.md`](i18n/glossary.md); **code identifiers, protocol fields and error
    codes keep their raw values**.
- **Internationalization = China-first users + an English product name + the ability to publish overseas**, not "turn the product English".
- **Naming**: session artifacts carry a `YYYY-MM-DD-` prefix, long-lived documents do not. Details in [`docs/README.md`](docs/README.md).
- **Local-first**: scan data, page content and screenshots stay on the machine by default. A remote vision
  model requires an explicit statement of data flow and cost.
- **A candidate is not a defect, and "clickable" is not "the business operation succeeded"**: these are product
  policy, not phrasing preferences (see the four boundaries in §1 and the invariants).

## 8. Directory map

```
committed ── front door  README CONTRIBUTING SECURITY CODE_OF_CONDUCT CHANGELOG LICENSE AGENTS.md (each with an *.en.md)
           i18n         i18n/README policy · i18n/glossary terms · i18n/docs-manifest pairing registry
           engineering  packages/<core|confirm|reduce|replay|cli> apps/desktop
                        scripts/check-hygiene.mjs · scripts/check-i18n.mjs
           knowledge    docs/README documentation map
                        docs/guide architecture development      user-facing (same-directory *.en.md)
                        docs/knowledge                           ★ knowledge base, see §6
                        docs/research product engineering/* reviews process assets (no pairing required)
                        datasets/                              positive/negative case dataset (the most valuable asset)
                        openspec/  config.yaml + specs/ + changes/{,archive/}
           integration  .github/{workflows,ISSUE_TEMPLATE} (both languages in one form) · .gitee (Chinese-only) · .workflow .githooks
never committed  .gitignore .layress/ artifacts/ docs/drafts/ .workbuddy/ .trae/ .cursor/ .claude/ .vscode/
```

## 9. Maintenance rules for this file

- The status card (§2) updates on every handoff; other sections change **only when the policy changes** —
  if a section cannot be changed, that is a sign the code or a gate should change instead.
- **Paired documents are never edited on one side only**: this file and every front-door document have an
  `*.en.md`. A Chinese edit lands with its English edit in the same round. Two commits leave the English copy
  on the old policy while readers assume it states current fact — worse than having no English copy.
- A new directory must be registered both in the `docs/README.md` map and in that directory's own `README.md`;
  unregistered means ownership undecided — do not create it yet.
- Every documentation reshuffle leaves a path change log (end of `docs/README.md`), otherwise in three months
  nobody knows where the files went.
