# invariants.md · Architecture and policy invariants

<p>简体中文 | <a href="invariants.md">English</a></p>

> These entries are **not reopenable**. Each one states the consequence when it fails to hold — a "principle"
> with no consequence is only a wording preference, and that kind of thing does not belong here.
> The bar for adding an invariant: violating it has already cost one measurable loss, or it is the only
> guarantee of some product claim.

| # | Invariant | What happens if it does not hold |
|---|---|---|
| I1 | **A candidate is not a defect.** Geometric anomalies enter the candidate layer only, the confirm layer produces a three-valued verdict (confirmed / not-a-defect / inconclusive), and reports must preserve the distinction between layers | The report decays into alert noise: after the first false positive the user stops believing any line in it, and the tool loses its value outright |
| I2 | **Reachability verdicts rest on real hit testing.** Programmatic clicks may only be used for preparing state, never as evidence that something "can be clicked" | An occluded action gets reported as usable — the inverse failure of this product, and it costs more trust than a false negative does |
| I3 | **The trigger-condition quadruple is complete**: size (width × height) + UI state + font/scaling + language. Missing any one item ⇒ marked non-replayable; silently filling it in is not allowed | It fails to reproduce on another machine, and the "replayable evidence" deliverable exists in name only |
| I4 | **Missing, corrupt or unknown provenance must not be guessed by the UI into trustworthy fact.** Displaying unknown is the better outcome | The user fixes the bug against guessed conditions, changes the wrong place, and cannot come back and trace it |
| I5 | **Replay adds no real external side effects**; a rerun must be explicitly authorized and must state its side effects and cost | A "read-only" replay submits a form in the target application |
| I6 | **No golden screenshot baseline ≠ no specification.** Critical actions, setup steps, acceptable scrolling and exclusion scope must be declared by the developer | The tool starts deciding "what counts as important" on the user's behalf, and the false-positive rate and the subjectivity both run out of control |
| I7 | **Datasets are physically separated**: the subset used to tune rules and the final evaluation subset do not overlap, and the evaluation set never enters the rule-iteration loop | Metrics climb together with the rules while not a single defect is found in an external project, and the first-validation conclusion is void |
| I8 | **Local-first**: page content, screenshots and scan results do not leave the machine by default. Using a remote model requires an explicit data flow and cost statement, and **must never silently degrade to remote** | Customer UI data is sent out without the user knowing; this is a legal-grade incident, not a privacy slogan |
| I9 | **Reduction is complete only while the failure is preserved**: every condition removed must be verified as still reproducing, and a necessary precondition may not be dropped in order to pass | A "minimal reproduction" ships that does not reproduce at all; the developer tries it once and abandons the tool |
| I10 | **The four modules — scan / confirm / reduce / replay — are usable independently**, and the core detection and reporting are fully open source | The free part is not enough to finish one thing on its own, and "open source" is left standing as a signboard only |
| I11 | **A machine PASS ≠ human approval**: visual and interaction verdicts belong to human review, and unexecuted review is listed separately | The status card reads all green while nobody has actually glanced at anything, and whoever takes over keeps pushing forward on that basis ⇒ the whole stretch is redone |
| I12 | **Never touch the C drive (portable first)**: data, logs, caches and temp files are written to the product's own directory or a directory the user explicitly specifies; they **must not** be written by default to `%APPDATA%` / `%LOCALAPPDATA%` / `C:\Users\…` / the registry / `~/Library/Application Support`. The install location is the user's choice, and the installer neither elevates nor edits the system PATH. **If it cannot be written, do not write**: when the target directory is unavailable, raise an explicit error and require the user to specify one — **silent fallback to the user directory is forbidden** | Copying the directory does not migrate it as a whole, deleting it leaves residue, write permission is denied in enterprise environments, and the user's private data is scattered across the system drive without them knowing; "portable" decays into a marketing phrase |
| I13 | **The data-directory resolution policy is single and visible**: precedence `--data-dir` / `LAYRESS_DATA_DIR` > the data root under the product directory; the resolved result (including whether it is writable) must appear in the startup output, the logs and the report header | The user cannot verify whether I12 is being obeyed; the same product writes its evidence to different places on two machines, and replay cannot find the original data |

## Differences from the sibling project (do not copy it verbatim)

- ReBaseAgent's core asset is the **immutability of traces and snapshots** (append-only; a fork never modifies
  its parent). Layress has no "run history" layer, so `I3`/`I9` carry the same responsibility:
  **evidence is void once it is incomplete or reduced too far**.
- What corresponds to ReBaseAgent's `I: keys captured by the agent stay in memory only` is `I8` here —
  what goes out is **page content** rather than keys, at the same sensitivity level, and screenshots may carry
  names, addresses and internal paths.
- ReBaseAgent already adopted the policy "a portable build's business data lands in `data/` next to the exe";
  Layress **keeps it and goes one step further**: caches and temp files (including Chromium/Electron's
  `sessionData` and `userData`) likewise must not land in the C-drive user directory —
  see I12 and the "Storage and portability" section of [`engineering.md`](engineering.md).
