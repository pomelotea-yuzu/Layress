# docs/ · Documentation map and index

<p>简体中文 | <a href="README.md">English</a></p>

This directory has two layers: **user-facing documentation** (`guide` / `architecture` / `development`)
and **engineering process assets** (`research` / `product` / `engineering` / `reviews` / `drafts`).
The latter are surveys, discussions, reviews, plans and acceptance records — not product documentation.

## Authority order

When sources disagree, this is the precedence for judging project state:

1. Source code under `packages/*/src` and `apps/*/src`
2. `openspec/changes/archive/` — tasks / design / evidence-index of archived changes
3. `openspec/specs/` — capability wording merged into the main specs
4. This directory
5. [`/AGENTS.md`](../AGENTS.md) — the handoff index, known to lag behind

A date in a process document is its expiry: **dated documents describe that moment**, not the current state.

## User-facing docs

| Directory | Contents | Bilingual |
|---|---|---|
| [`guide/`](guide/README.md) | Quick start: installation, four ways in, core concepts | Required |
| [`architecture/`](architecture/README.md) | Architecture overview: module split, data flow, design principles | Required |
| [`development/`](development/workflow.md) | Development workflow: gate chain, openspec process, docs and commit conventions | `workflow.md` required |

## Knowledge base

[`knowledge/`](knowledge/README.md) — the most important mechanism in this repository: pitfalls, live-probe
hard facts, non-reopenable invariants and key decisions are all **committed**, so any developer or any agent
that clones the repo can pick the work up. Entry format and the promotion rules live in
[`knowledge/README.md`](knowledge/README.md).

| File | Collects |
|---|---|
| [`knowledge/engineering.md`](knowledge/engineering.md) | Pitfalls in builds, tests, gates, dependencies, openspec usage |
| [`knowledge/runtime-probe.md`](knowledge/runtime-probe.md) | Live-probe hard facts: CDP, windows, DPR, IME, timing, false-green criteria |
| [`knowledge/invariants.md`](knowledge/invariants.md) | Architecture and policy invariants, each with the consequence of violating it |
| [`knowledge/decisions.md`](knowledge/decisions.md) | Decision lookup: choice / rationale / cost / what would overturn it |
| [`knowledge/debt.md`](knowledge/debt.md) | Open policy questions and debt, with owner and the deadline that forces a call |

## Engineering process assets

| Directory | Contents | Bilingual |
|---|---|---|
| [`research/`](research/README.md) | Market and competitor surveys, related research, name clearance | Not required |
| [`product/`](product/README.md) | Positioning, roadmap, capability boundaries | Not required |
| [`engineering/`](engineering/README.md) | `plans/` execution plans · `reports/` delivery and build reports · `notes/` feature notes and workflow conventions | Not required |
| [`reviews/`](reviews/README.md) | Live acceptance and review evidence; **process screenshots stay local by default** | Not required |
| `drafts/` | Personal working drafts — **the whole directory is never committed** | — |

## Bilingual scope

Pairing rules, the forms exception and the gate's check list are all in
[`/i18n/README.md`](../i18n/README.md). One-line version: **the front door, the handoff index,
user-facing docs and the knowledge base must ship in both languages; process assets are not
translated.** Paired status is whatever [`/i18n/docs-manifest.json`](../i18n/docs-manifest.json)
registers, enforced by `node scripts/check-i18n.mjs`.

## Naming conventions

- **`YYYY-MM-DD-` prefix**: every session artifact (surveys, logs, reviews, reports, replies) — keeps
  them searchable in time order.
- **No prefix**: long-lived documents (`workflow.md`, `decisions.md`, `invariants.md`).
- Pick a directory from the tables above first, then register a line here; **cross-directory links are
  relative to the Markdown file that contains them**.
- Term pairings come from [`/i18n/glossary.md`](../i18n/glossary.md); code identifiers and error codes stay raw.
- A directory holding only a README placeholder states its ownership and its bar for entry, nothing more.

```bash
node scripts/check-i18n.mjs   # a dead link written into this map goes red here
```

## Path change log

- **2026-11-03 repository skeleton**: established the nine categories guide / architecture / development /
  knowledge / research / product / engineering / reviews / drafts; `docs/drafts/`, `docs/reviews/**/*.png`,
  `artifacts/` and `.layress/` declared never-committed; the survey moved out of the repository root into
  [`research/`](research/README.md).
- **Where the policy comes from**: the sibling project ReBaseAgent's two sweeps on 2026-09-24 (regrouping)
  and 2026-10-12 (draft archival), plus the lesson of its `output/` directory decaying
  (see [`/AGENTS.md`](../AGENTS.md) §five).
