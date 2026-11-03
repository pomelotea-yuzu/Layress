# i18n · Bilingual policy

<p>简体中文 | <a href="README.md">English</a></p>

This directory governs two things: **documentation-layer bilingualism** (in force now) and
**product-string-layer bilingualism** (in force once `packages/` holds code).
Framing: internationalization means China-first users, an English product name, and the
structural readiness to publish overseas — not "turn the product English".

## Scope and the two layers

| Layer | Contents | Status | Enforcement |
|---|---|---|---|
| Documentation | The public front door and the handoff-critical `.md` files plus their `*.en.md` twins | Active | `node scripts/check-i18n.mjs` |
| Product strings | `zh-CN` / `en-US` resources for desktop UI, human-readable CLI output, error copy | Not enabled | Same script extended, plus a batch-attribution manifest |

What the string layer will look like (documented now so the layout doesn't fight it later):
`packages/i18n/locales/{zh-CN,en-US}/*.json` with identical key structures, interpolation
parameters and rich-text slots; `zh-CN` is the default, and the English entry is never a
machine-translation dump.

## Pairing rules (documentation layer)

- Chinese is the **source**, English the **translation**: `X.md` → `X.en.md` in the same
  directory, not under an `en/` subtree. Side by side, both land in the same diff, so a
  single-sided edit is visible.
- Required to be paired: the **public front door** (README, CONTRIBUTING, SECURITY,
  CODE_OF_CONDUCT, CHANGELOG), the **handoff index** (AGENTS.md), **user-facing docs**
  (`docs/guide`, `docs/architecture`, `docs/development`) and the **knowledge base**
  (`docs/knowledge`, whose whole point is that anyone can pick the project up).
- Not required: `docs/research`, `docs/product`, `docs/engineering`, `docs/reviews`,
  `docs/drafts`, `datasets`, `openspec/changes`. Process assets are valuable as process;
  force-translating them only manufactures maintenance debt.
- Registration is what counts: a document entering the paired scope must be listed in
  [`docs-manifest.json`](docs-manifest.json). **Unregistered = unchecked = Chinese-only is
  allowed**; registered without a twin = red.
- The reverse counts too: an unregistered `*.en.md` fails as `TWIN_UNREGISTERED`, which
  closes the loophole of a one-sided copy that slips past the gate.

## Forms and templates: not two copies

Translating an issue form or a PR template into two copies leaves submitters unsure which to
fill and reviewers unable to diff the wording. Instead:

- **GitHub issue forms** (`.github/ISSUE_TEMPLATE/*.yml`): both languages inside one form —
  `name: Bug report / Bug 报告`, `label: What happened / 遇到的问题`, with the English
  paragraph first and the Chinese one right after. One form, either language fills it.
- **PR template** (`.github/PULL_REQUEST_TEMPLATE.md`): headings read
  `## Summary / 改动说明`, and guidance comments come in pairs — `<!-- English -->`
  immediately followed by `<!-- 中文 -->`.
- **Gitee side** (`.gitee/*.zh-CN.md`): Chinese only. Contributors on mainland networks
  do not need a second English read.
- All three must stay equivalent in substance: change one and update the other two in the
  same round. The gate does not check yml-vs-md semantic equivalence — that one is on you.

## Language entry format

Every paired document carries a cross-link directly under the level-1 heading, Chinese
pointing to English and English pointing back:

```markdown
<p>简体中文 | <a href="README.md">English</a></p>
```

A missing entry fails as `NO_LANG_LINK`. It has to sit right next to the title; buried in
the body it might as well not exist.

## Translation discipline

- **Same-round edits**: a policy change lands in Chinese and English within one commit.
  Two commits leave the English on the old policy while readers assume it states current
  fact — worse than having no English at all.
- **Match conclusions, not sentences**: split or merge Chinese sentences as English needs,
  but section order, section count, table rows, conclusions and warnings must line up. The
  gate diffs `##` section counts and fenced block counts, and drift is red.
- **Raw values are never translated**: code identifiers, protocol fields, error codes, file
  names, CLI flags, CSS properties, units and package names stay as they are. Look up term
  pairings in [`glossary.md`](glossary.md).
- **Never adjust numbers in passing**: thresholds, counts and dates must be literally
  identical in both copies. Fixing one and forgetting the other is the classic source of a
  forked policy.
- English is written by a person, not emitted by a machine: batch translation starts only
  after the glossary is frozen, otherwise one concept ships three renderings.

## The gate

```bash
node scripts/check-i18n.mjs               # check every paired document in the manifest
node scripts/check-i18n.mjs --selftest    # counter-proof: missing twin / broken link / drift must go red
```

Rules: `TWIN_MISSING` · `ZH_MISSING` · `NO_LANG_LINK` · `NO_TITLE` · `HEADING_DRIFT` ·
`CODE_BLOCK_DRIFT` · `THIN_TWIN` (the English body is suspiciously small next to the Chinese) ·
`BROKEN_LINK` · `TWIN_UNREGISTERED` (orphan English).

`--selftest` is the gate's own proof: 6 counter-examples plus 1 positive case. Without it, a
rule that never matches reports "bilingual all green" while checking nothing at all. Once the
workspace lands this joins `pnpm check:i18n`, the `check:ci` chain and CI.
