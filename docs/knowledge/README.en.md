# docs/knowledge/ · Knowledge base

<p>简体中文 | <a href="README.md">English</a></p>

> This directory is Layress's **committed knowledge base**: pitfalls hit during development, hard facts,
> decisions that are not reopenable.
> Its reason for existing: the sibling project put the same kind of material in a local memory directory
> (`.workbuddy/memory/`), which `.gitignore` excluded as a whole directory, and the result was that
> **developers cloning the repository, or another agent, never got 90% of the experience**. This repository
> does not repeat that loss.
>
> The entry pointer lives in [`/AGENTS.md`](../../AGENTS.md) §6; the entry format and the four writing
> disciplines are authoritative there.

## Five topics, by subject rather than by date

| File | Collects | Does not collect |
|---|---|---|
| [`engineering.md`](engineering.md) | Pitfalls in builds, tests, gates, dependencies, openspec usage | One-off running logs, emotions |
| [`runtime-probe.md`](runtime-probe.md) | Live-probe hard facts: CDP, windows, DPR, IME, timing, false-green criteria | Speculation that was never measured |
| [`invariants.md`](invariants.md) | Non-reopenable architecture invariants, each with the consequence when it fails to hold | Options that are still open for discussion |
| [`decisions.md`](decisions.md) | Key decisions: what was decided, why, at what cost | Old drafts already overturned by a later decision (those go to the change archive) |
| [`debt.md`](debt.md) | Open policy questions and debt, marked with who decides | Matters that already have a conclusion |

## Entry format (fixed — never convert it into free narrative)

```markdown
- [2026-11-03][P0] phenomenon ⇒ root cause ⇒ workaround ⇒ evidence
```

- **Level**: `P0` fatal (data corruption / verdict void) · `P1` causes a wrong verdict (false green / false red) · `P2` wastes time
- **One line, one subject.** Several subjects means splitting it into several entries.
- **Conclusions only**; the process stays in the corresponding change's `design.md` / `evidence-index.md`, and this line keeps only a pointer.
- **A pitfall hit a second time ⇒ promote it**: write it into `invariants.md`, or turn it straight into a gate / test assertion.
  A pitfall guarded only by memory will be stepped in again.

## Counter-example (why the rules above exist)

An experience entry in the sibling project looked like this: one 400-character line that strung four things
together with semicolons — "the window breakpoint straddling the line + the real keyboard's foreground
ownership + Monaco's timing transience + the fixture reinstall changing the run id" — with that round's test
counts hanging in front of it.
Its information density is extremely high, **but the next person to take over skips the whole line** — because
it requires the reader to already hold that round's context. This directory requires every entry to read cold:
understandable without opening that day's logs.
