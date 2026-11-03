# debt.md · Open policy questions and debt

<p>简体中文 | <a href="debt.md">English</a></p>

> Only matters with **no conclusion yet** live here. Concluded ones go to [`decisions.md`](decisions.md),
> known pitfalls to [`engineering.md`](engineering.md) / [`runtime-probe.md`](runtime-probe.md).
> Every entry must state "who decides" and "when it must be decided", or it will hang here indefinitely.

| Item | Current state | Who decides | When it must be settled |
|---|---|---|---|
| Approval of the first validation plan | Written up as a **proposal**: [2026-11-03-first-validation-plan](../engineering/plans/2026-11-03-first-validation-plan.md) — S0 probe-channel comparison → S1 dataset → S2 three-arm control → S3 metrics and human review → S4 go/shrink; suggested timebox 6 weeks / 90 h; §four lists the six calls the owner must make; **not approved** | Owner | Before entering the first change |
| Probe technology-stack selection | **No conclusion preset**: it comes out of the plan's S0 comparison experiment (R1 Playwright + system Chrome vs R2 raw CDP; 3 known-answer samples × 7 measurements × 6 repeats), with the decision criteria pre-written in plan §two — "zero byte growth in the C-drive user profile" (I12) being one of the hard criteria. **Not chosen** | The first change's design.md | Before writing the first line of detection code |
| Geometric thresholds and DPR policy | The occlusion/clipping verdict thresholds, 1px stroke rounding, and the banding of DPR=1/2 with 200% scaling are **unmeasured** (the sibling project only proved that the "default window width straddles the breakpoint line" pitfall exists) | First-validation data | Before producing the first precision figures |
| Whether to introduce a remote vision model | The survey states that "combining AI to look at images" does not constitute a difference; introducing one means settling the `I8` data flow and cost declaration | Owner | When the confirm layer's design is finalized |
| Redaction when exporting evidence | Screenshots and page content may contain names, addresses and internal paths; staying local by default is settled (`I8`), but the redaction rules under the **export/share** form are undecided | Owner + the report module's design | Before exporting is supported |
| Redistribution rights for dataset samples | Collecting page samples from real open-source projects for a public dataset: the redistribution license and the attribution wording are **unchecked** | Owner | Before the dataset enters the repository |
| `layress.com` registration | whois shows it unregistered (about $14.48, looked up 2026-11-03); it secures the brand entry point | Owner | Before any public promotion |
| npm org `@layress` | An npm organization is required; the write-up distinguishing it from `layrr`/`layerr`/`layui` has to go into the README | Owner | Before the first package is published |
| `docs/guide/` contents | The quick-start guide gets written only once the CLI genuinely runs, otherwise the README can only say "not released yet" (that is debt, not the target shape) | The first deliverable version | Before the first tag |
| Third-party notices and dependency license checks | The sibling project used `docs/development/third-party-notices.md` plus a license manifest script; Layress needs an equivalent process once it takes on dependencies | The first release round | Before the first release |

## Known "not going to do this now"

Written out so the next person does not mistake them for an oversight:

- Aesthetic scoring, auto-fix, fully automatic exploration of arbitrary applications, automatic extraction of
  minimal HTML/CSS, all-platform all-browser support, large-scale multilingual and font matrices —
  survey §7.2 lists these explicitly as **deferred**.
- `packages/*` and `apps/desktop` currently carry only a statement of ownership: no code and no
  `package.json`. That is this round's policy (D13), not a half-finished artifact.
