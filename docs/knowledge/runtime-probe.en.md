# runtime-probe.md · Live-probe hard facts

<p>简体中文 | <a href="runtime-probe.md">English</a></p>

> Entry format: see [`README.md`](README.md). **Entries marked "Source: sibling project ReBaseAgent" are not yet verified in Layress** —
> treating them as a working hypothesis for the observed phenomenon is fine, treating them as facts verified in this repository is not:
> Layress has not chosen its probe stack yet.
> The shared characteristic of these traps: **a red probe is not a red product, and a green probe is not a correct product**.

## Channels and timing

- [2026-11-03][P0] When CDP `Runtime.evaluate` returns an object, the caller gets an exception or a swallowed result ⇒ the return payload of the evaluation channel must be a serializable primitive type ⇒ probes always `return JSON.stringify(...)`, and when a field-level verdict is needed return the object explicitly and assert field by field ⇒ Source: sibling project ReBaseAgent, proven across three probe batches.
- [2026-11-03][P0] The probe "looks normal" but the target control is never found ⇒ the port was taken by an orphan dev host from a previous round, the freshly started dev silently switched ports, and CDP is attached to the old instance ⇒ before probing, explicitly stop everything and confirm port ownership; end the probe with `process.exit()`; leaving anything behind is forbidden ⇒ Source: sibling project ReBaseAgent.
- [2026-11-03][P1] The browser/Electron started inside a background task is reclaimed before the probe reads it ⇒ "start→probe→stop" must be written as one chained command within a single call — across calls the instance is already gone ⇒ Source: sibling project ReBaseAgent.
- [2026-11-03][P1] Reading a value from a lazily loaded editor/chart is occasionally undefined ⇒ it is not mounted on first paint, so the timing is sensitive ⇒ turn every read into polling with a timeout instead of a fixed sleep; a fixed sleep is guaranteed to go red on a slow machine.

## Input and focus

- [2026-11-03][P0] A programmatic `.click()` passes while a real pointer click is blocked ⇒ `modal`/top-layer and occluding layers apply only to real hit-testing, and a programmatic click bypasses the very thing this product is supposed to test ⇒ **a reachability verdict must go through `dispatchMouseEvent` at real coordinates**; programmatic clicks are for setup state only. This is a product-level invariant, not a probe preference.
- [2026-11-03][P1] Real keyboard injection works intermittently, and the focus ownership looks correct ⇒ the channel is sensitive to which window owns the foreground; another window stealing focus makes ownership wander ⇒ explicitly pre-place focus before every batch of key events and point the instruction at the physical control; a rerun passing again ≠ random noise. Source: sibling project ReBaseAgent.
- [2026-11-03][P1] `insertText` loses the first character or writes into the wrong control ⇒ focus was not set beforehand; `Tab` travels through a rich-text editor and changes the selection; `Enter` is injected carrying `\r` ⇒ handle the three keyboard-capture cases separately, and assert the target is focused before measuring. Source: sibling project ReBaseAgent.
- [2026-11-03][P1] In an input method (IME) composition state, the injected text and the finally committed content differ ⇒ Chinese/Japanese input goes through synthetic events; the verdict reads only the committed value ⇒ locale-switch stress conditions must cover the IME path, otherwise what is being tested is the English-only case.
- [2026-11-03][P1] A control reports "focused/selected" while the UI has not actually changed ⇒ the overlay layer swallowed the focus yet the state was still updated, so the verdict is a false green ⇒ assert that no `[open]` dialog exists before a write action, and pair every state verdict with a visual verdict. Source: sibling project ReBaseAgent.

## Geometry and viewport

- [2026-11-03][P0] The same "narrow window" defect reproduces at widths a dozen px apart on different machines ⇒ the default window width sits exactly on a CSS breakpoint line (measured in the sibling project at 1360 physical width ≈ 963 CSS px) ⇒ stress conditions must record all three of **content-area CSS px**, DPR and scaling, and must be preceded by an explicit resize to a known step ⇒ Source: sibling project ReBaseAgent; Layress's first verification must self-test this line.
- [2026-11-03][P1] The width band gets recorded but does not reproduce on another machine ⇒ only the width is recorded, not the height/state/font/scaling, and the occlusion condition is joint ⇒ trigger conditions are stored as the four-tuple: size (width×height) + state + font scaling + language. Missing any one item does not count as replayable.
- [2026-11-03][P1] The boundaries of the same element differ under DPR=1 and DPR=2 ⇒ physical/logical pixel conversion plus rounding of the 1px stroke ⇒ use CSS px for verdict thresholds and store DPR alongside the record; run a 200% scaling batch separately. Source: sibling project ReBaseAgent.
- [2026-11-03][P1] Screenshot comparison picks up a stale frame after a window resize ⇒ frame submission is asynchronous ⇒ wait one rAF plus a stability check before capturing; do not trust `wait: 500ms`.

## Verdicts and false greens

- [2026-11-03][P0] The assertion is always true ⇒ the expected value was derived from the implementation under test (same source), or the assertion argument was swallowed into an empty string by the toolchain (non-ASCII literal) ⇒ a verdict must be falsifiable: injecting a known-bad sample must go red; write assertion arguments as `\uXXXX` escapes and self-test that the escaping takes effect ⇒ see the matching entry in [`engineering.md`](engineering.md).
- [2026-11-03][P1] The probe is red ⇒ triage before reporting a defect: a stale snapshot value / selector drift / a changed fixture run id can all stop the probe from reading anything while the product is fine ⇒ in the report, "probe failure" and "candidate defect" are registered separately — the former goes into this file, the latter into the dataset.
- [2026-11-03][P1] A visual/interaction verdict gets a machine PASS and is then ticked off as done ⇒ visual correctness belongs to human review ⇒ a review that was not executed is listed separately as "not executed" and never written into what was delivered; machine counts are not acceptance.
