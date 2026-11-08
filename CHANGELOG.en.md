# Changelog

<p><a href="CHANGELOG.md">简体中文</a> | English</p>

This file follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/) and
[Semantic Versioning](https://semver.org/). **It records user-facing changes by release**;
change types are `Added / Fixed / Changed / Removed / Deprecated`.

Development history and individual commits belong to `git log`. Decisions, pitfalls, research and
experiment findings live in [`docs/knowledge/decisions.md`](docs/knowledge/decisions.md),
[`docs/knowledge/engineering.md`](docs/knowledge/engineering.md),
[`docs/research/`](docs/research/README.md) and the status card in [`AGENTS.md`](AGENTS.md) §two.
The repository setup entries dated 2026-11-03 to 2026-11-08 used to sit under "Unreleased" here and now
live with their respective topics; the original text remains in Git history (`git log -- CHANGELOG.md`).

## Unreleased

No release yet. The repository holds the directory layout, handoff index, gates and research documents;
there is no runnable product, no npm package and no tag. For current state see the status card in
[`AGENTS.md`](AGENTS.md) §two and `git log`.

## Versioning and release conventions

- Versioning starts at `0.1.0`; breaking changes are allowed during `0.x` but must state their impact here.
- Candidates use `-rc.N`; packaging, hashes, acceptance records and release copy live in `docs/engineering/reports/`.
- **No tag and no npm publish before the first validation closes**: dataset, control groups and the
  continue/shrink thresholds are defined in [`docs/research/`](docs/research/README.md) and
  [`docs/knowledge/debt.md`](docs/knowledge/debt.md).
- Pushing and tagging belong to the maintainer, who keeps the two remotes (GitHub / Gitee) in sync.
