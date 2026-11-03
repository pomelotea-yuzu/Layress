# Development workflow

<p>简体中文 | <a href="workflow.md">English</a></p>

This page is the authority on day-to-day operations. When a policy changes, update
[`/AGENTS.md`](../../AGENTS.md) and this file together — both language copies at once.

## Environment and daily commands

- Node ≥ 20; package manager pnpm (once the workspace exists, pin `packageManager` in the root `package.json`).
- Once per clone: `git config core.hooksPath .githooks`. Without the hook, hygiene problems surface only in CI.

```bash
# What actually runs today
node scripts/check-hygiene.mjs --staged
node scripts/check-hygiene.mjs --all
node scripts/check-i18n.mjs
node scripts/check-i18n.mjs --selftest

# Target chain once the workspace lands (not built yet — see docs/knowledge/debt.md)
# pnpm check:build && pnpm check:typecheck && pnpm check:test \
#   && pnpm check:lint && pnpm check:i18n && pnpm check:spec && pnpm check:hygiene
```

## Quality gates

| Gate | Command | Rejects | Status |
|---|---|---|---|
| Hygiene | `check-hygiene.mjs` | size, binary suffixes, scratch paths, credential shapes, ignored-yet-tracked | Active |
| Bilingual pairing | `check-i18n.mjs` | missing twin, broken link, section drift, orphan English | Active |
| Build / typecheck / test / lint | `pnpm check:*` | code quality and regressions | Not built |
| Spec consistency | `pnpm check:spec` | diff between main specs and archived changes | Not built |

Two hard rules:

- **Local and cloud run the same chain.** CI (`.github/workflows/ci.yml` and `.workflow/ci.yml`) must invoke the
  same commands as pre-commit, otherwise "green locally, red remotely" eats the evening.
- **A machine PASS is not acceptance.** Visual and interaction verdicts belong to human review; unexecuted review
  is listed as "not executed" in project status.

## Repository hygiene operations

What to do when the gate blocks you, in decision order:

1. Can a script regenerate it? ⇒ leave it in `artifacts/` or `.layress/`; it needs neither committing nor an exemption.
2. Is it valuable hand-written content? ⇒ move it to the right place: conclusions into the proper `docs/`
   subdirectory, samples into `datasets/`.
3. Does it genuinely have to be committed as a binary? ⇒ add one line to
   [`.git-hygiene-allowlist`](../../.git-hygiene-allowlist) as `glob  #  reason` (plus `size=NNNK/M` if needed),
   then `git add -f`. A reason-less exemption fails the gate.
4. "Ignored yet tracked"? ⇒ `git rm --cached <path>` and re-commit; do not bend `.gitignore` to accommodate history.

`.gitignore` maintenance discipline: every new block must state **what it is** and **where valuable content
belongs instead**. An ignore rule without a reason is something nobody dares delete three months later.

## Bilingual operations

Adding or editing a paired document:

1. Check the scope: front door / handoff index / user-facing docs / knowledge base ⇒ paired required;
   process assets ⇒ not required (see [`/i18n/README.md`](../../i18n/README.md)).
2. Write the Chinese source, then the English `X.en.md` in the same round, with the language entry directly
   under the level-1 heading.
3. Register a new document in [`/i18n/docs-manifest.json`](../../i18n/docs-manifest.json). Unregistered means
   unchecked, but an English copy that is not registered fails as `TWIN_UNREGISTERED`.
4. New terminology goes into [`/i18n/glossary.md`](../../i18n/glossary.md) and its English twin before any document uses it.
5. Run `node scripts/check-i18n.mjs` locally and see it green before opening the PR.

Issue and PR forms are **not duplicated**: GitHub puts both languages side by side in the same yml field
(`English / 中文`), Gitee ships Chinese-only. Change one and align the other two in the same round — the gate
cannot check semantic equivalence, that one is on people.

## OpenSpec process

```
openspec/
├─ config.yaml        project bible: positioning, stack, architecture invariants, execution boundaries
├─ specs/             main specs (capability wording, long-lived once merged)
└─ changes/           active changes
   └─ archive/        archived changes (the authoritative home of decisions and evidence)
```

A change's life cycle:

1. `openspec/changes/<name>/tasks.md` breaks down the steps; `design.md` records the approach, trade-offs and
   the routes that were rejected.
2. Implement task by task, keeping the criteria for each; live evidence goes in `evidence-index.md` (screenshots
   stay local by default — see the hygiene section).
3. On completion, archive to `openspec/changes/archive/<YYYY-MM-DD>-<name>/` and merge the delta into `specs/`.
4. **Authority order**: source code > `changes/archive/` > `specs/` > `docs/` > `AGENTS.md`.
   Judge progress from the first two; everything else may be stale.

Documentation-only fixes, typos and link repairs need no change proposal.

## Documentation and naming

- Session artifacts carry a `YYYY-MM-DD-` prefix; long-lived documents do not.
- A new directory must be registered both in the [`/docs/README.md`](../README.md) map and in that directory's own
  `README.md`; unregistered means ownership undecided — do not create it yet.
- Every reorganization leaves a path change log (end of `docs/README.md`), otherwise nobody knows where files went.
- Knowledge entries follow the format in [`docs/knowledge/`](../knowledge/README.md) and do **not** get written into
  `AGENTS.md` body — that file has a size budget.

## Commit and push

- One commit does one thing; title `<type>(<scope>): <one-line change>`, body in Chinese explaining *why*.
- **Commit after every valuable step**; do not hoard. But **do not push**.
- Pushing, tagging and dual-remote sync belong to the maintainer. Idempotent reruns look like
  `git push github main && git push gitee main`; push tags separately and **never move or force-push a published tag**.
- Push counts come from `git ls-remote` output, never from what a document claims.
- `--no-verify` to bypass the hygiene gate is forbidden. If a bypass feels necessary, the gate is wrong — fix the
  script rather than hand-approving it every time.
