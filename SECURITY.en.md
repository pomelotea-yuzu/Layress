# Security Policy

<p><a href="SECURITY.md">简体中文</a> | English</p>

**Please do not file security problems publicly.** Anything involving the scanned application's page
content, credentials, or outbound data is treated as high severity and fixed first.

## Supported scope

| Version / platform | Status |
|---|---|
| No release yet | The repository currently holds structure and gates only — no runnable product (see the status section in [`README.md`](README.md)) |
| Tauri desktop workbench and CLI | Desktop framework selected, not implemented; no verified support matrix yet. System WebView capabilities require platform testing |
| Target browsers / Electron applications and agent clients | Support will be published by tested integration; arbitrary application or client compatibility is not promised |

> Development happens on macOS arm64; working on the development machine does not establish product support. The old D15 desktop distribution matrix was closed with the previous phase. See the current [product requirements](docs/product/product.md) and [roadmap](docs/product/roadmap.md).

The version support matrix, dependency update policy and end-of-support notes will be added under
[`docs/development/`](docs/development/README.md) when the first candidate ships.

## Layress risk surface

This tool touches sensitive data by design, so the risk surface is spelled out instead of a bare
"please report vulnerabilities":

1. **The scanned content is the sensitive data.** Screenshots, DOM fragments and long-text samples may
   carry names, addresses, internal file paths and business identifiers — easier to underestimate than credentials.
2. **Evidence export and sharing.** To stay reproducible, reports include URLs, element locators, states and
   screenshots; exporting or uploading is disclosure. The redaction policy is undecided (see
   [`docs/knowledge/debt.md`](docs/knowledge/debt.md)).
3. **Handing material to remote agents or models.** Page material stays local by default; outbound data flow and cost must be explicit, with **no silent fallback to remote**. Any implementation that
   bypasses those three is a security defect, not a configuration preference.
4. **The debug channel is over-privileged.** A CDP／remote-debugging port means full script execution against
   the target page; if the port binds to a non-loopback address, other local processes or the LAN can attach.
5. **Injection into probe execution.** Target URLs, selectors, setup-step scripts and exclusion scopes are
   user-controlled inputs; missing validation before they reach an execution path allows unauthorized actions.
6. **Page content reaching model prompts.** If a vision/language model is used for confirmation, text on the
   page can carry prompt-injection payloads that steer verdicts or trigger outbound traffic.
7. **Credential handling.** Model keys and target-app sessions are entered by users inside the app; plaintext
   must not appear in logs, reports or traces. The repository gate fails `sk-`／`ghp_`／private-key shapes outright.
8. **Operation scope.** Tasks, feedback and evidence are written under the product data directory. Reading a task does not authorize modifying the target project or rerunning actions with external side effects. The user's existing agent performs fixes within its own authorization; handling results remain separate from acceptance.
9. **Desktop native permissions.** Isolate target pages from the tool interface. External pages must not receive Tauri native command, filesystem or process access. System WebView sessions, caches and temporary directories are also subject to portable storage requirements.

## Data flow and privacy commitments

- **Local-first**: scan runs, screenshots and results stay on the machine by default, under the user-chosen
  data directory or `.layress/` (never committed).
- No telemetry is sent. If usage analytics are ever introduced, this page will state what is collected and how
  to turn it off before the code ships.
- When a remote model is used, the product must show plainly: what leaves the machine, to whom, how cost is
  computed, and whether the whole task can stay local.
- This repository stores **no** real credentials, no real-project screenshots, no content from any scanned application.

## Reporting

- **GitHub (primary repository)**: use the private disclosure entry
  **Security → Report a vulnerability** (`https://github.com/pomelotea-yuzu/Layress/security/advisories/new`),
  not a public Issue.
- **Gitee (mirror)**: Gitee has no private advisory channel — contact the maintainer privately first, and never
  put details in a public issue or comment.
- Please include if possible: affected version or commit, reproduction steps, which data classes are involved
  (page content? credentials?), and the plausible blast radius.
- Redact the report itself: keep only the error shape in logs, never full credentials or real scanned page content.

## Handling commitments

- Acknowledgement: within 72 hours.
- Assessment and reply: preliminary severity and plan within 7 working days.
- Fix window: high severity ships first, even if other work slips.
- Disclosure: timing agreed after the fix ships, 90 days by default; contributors may be credited or stay anonymous.
- Known unresolved risks are listed on this page and in the status card as they are — knowing about a problem
  is not a reason to hide it.
