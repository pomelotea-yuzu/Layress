# Security Policy

<p><a href="SECURITY.md">简体中文</a> | English</p>

**Please do not file security problems publicly.** Anything involving the scanned application's page
content, credentials, or outbound data is treated as high severity and fixed first.

## Supported scope

| Version / platform | Status |
|---|---|
| No release yet | The repository currently holds structure and gates only — no runnable product (see the status section in [`README.md`](README.md)) |
| Windows + Chromium | Planned first-release target |
| macOS / Linux | Not promised; to be decided after the first validation closes |
| Electron adapter | Planned, alongside the first validation |

> The table above describes **product target platforms, not the development machine**: development
> currently happens on macOS arm64 (moved from Windows on 2026-11-08), with three-platform distribution
> per D15. Moving the development machine does not change the supported scope.

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
3. **Remote vision models sending data out.** Invariant I8 requires data to stay local by default, outbound
   traffic to be explicitly declared with cost, and **no silent fallback to remote**. Any implementation that
   bypasses those three is a security defect, not a configuration preference.
4. **The debug channel is over-privileged.** A CDP／remote-debugging port means full script execution against
   the target page; if the port binds to a non-loopback address, other local processes or the LAN can attach.
5. **Injection into probe execution.** Target URLs, selectors, setup-step scripts and exclusion scopes are
   user-controlled inputs; missing validation before they reach an execution path allows unauthorized actions.
6. **Page content reaching model prompts.** If a vision/language model is used for confirmation, text on the
   page can carry prompt-injection payloads that steer verdicts or trigger outbound traffic.
7. **Credential handling.** Model keys and target-app sessions are entered by users inside the app; plaintext
   must not appear in logs, reports or traces. The repository gate fails `sk-`／`ghp_`／private-key shapes outright.
8. **Read-only promise.** Diagnostic entry points (CLI, MCP) are read-only by default; writing requires explicit
   authorization and must never modify the project under scan.

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
