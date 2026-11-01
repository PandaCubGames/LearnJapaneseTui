# Changelog

All notable changes to this project are documented here.
The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and the project uses [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Project rules (`CLAUDE.md`): branch-per-change workflow, merge approval, SemVer.
- Project overview: Rust, TUI-only, modular learning (verbs, vocab, …), Japanese text support on all platforms.
- Single-file distribution, SSH support, Japanese (IME) and romaji input requirements.
- Commitizen (`.cz.toml`) and pre-commit hooks (`.pre-commit-config.yaml`).
- Rust project scaffold (Cargo, edition 2024) with a size-optimized release profile.
- `cargo fmt` and `cargo clippy` pre-commit hooks; Commitizen reads the version from `Cargo.toml`.
- Secret scanning (gitleaks, private-key and large-file hooks), secret ignore patterns, and open source hygiene rules.
- License: GPL-3.0-or-later (`LICENSE`, SPDX headers, `Cargo.toml`), license rules for dependencies and learning content, and `ATTRIBUTION.md`.
- `README.md` and `CONTRIBUTING.md`.
- Project hosted on GitHub (PandaCubGames/LearnJapaneseTui); merges into `main` go through pull requests.
- Only the repository owner merges pull requests (GitHub ruleset + branch protection on `main`).
- Commit identity and attribution rules: GitHub noreply author, Claude co-author trailer only.
- `VERSION` file and this changelog.
