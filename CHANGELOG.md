# Changelog

All notable changes to this project are documented here.
The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and the project uses [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Project rules (`AGENTS.md`, formerly `CLAUDE.md`): branch-per-change workflow, merge approval, SemVer.
- Project overview: Rust, TUI-only, modular learning (verbs, vocab, …), Japanese text support on all platforms.
- Single-file distribution, SSH support, Japanese (IME) and romaji input requirements.
- Commitizen (`.cz.toml`) and pre-commit hooks (`.pre-commit-config.yaml`).
- Rust project scaffold (Cargo, edition 2024) with a size-optimized release profile.
- `cargo fmt` and `cargo clippy` pre-commit hooks; Commitizen reads the version from `Cargo.toml`.
- Secret scanning (gitleaks, private-key and large-file hooks), secret ignore patterns, and open source hygiene rules.
- License: GPL-3.0-or-later (`LICENSE`, SPDX headers, `Cargo.toml`), license rules for dependencies and learning content, and `ATTRIBUTION.md`.
- `README.md` and `CONTRIBUTING.md`.
- Project hosted on GitHub (PandaCubGames/LearnJapaneseTui); merges into `main` go through pull requests.
- Only the repository owner merges pull requests (sole write access); `main` protected by branch protection and the "Protect main" ruleset (PRs, signed commits, no force-push or deletion).
- Signed commits: SSH commit and tag signing (shown as Verified on GitHub); documented for contributors.
- Commit identity and attribution rules: GitHub noreply author, Claude co-author trailer only.
- CI: GitHub Actions workflow that checks every commit merged to `main` (and PR titles) against Conventional Commits.
- `VERSION` file and this changelog.
- Codex support: project rules moved to `AGENTS.md` (shared by all agents; `CLAUDE.md` imports it), Codex subagents in `.codex/agents/`, and tool-neutral merge and attribution rules.
- Codex model pins for the subagents (`gpt-6-astra` for planning, `gpt-6.1-terra` for implementation), Codex setup guide (`docs/codex-setup.md`), and permission for the spec-writer roles to run OpenSpec planning commands inside `openspec/`.
- Draft architecture document (`docs/architecture.md`, linked from `README.md`): screen flow with a JLPT level picker, design decisions, the Vocabulary, Kanji and Grammar data models, rule-based conjugation, and the component layout. Design only; no code yet.
- Agent rule: when working through an OpenSpec change's `tasks.md`, agents commit after every task, one atomic Conventional Commit per task.
- Testing rules for agents and OpenSpec: every new feature adds tests, every bug fix adds a regression test, and existing tests are never changed without the owner's decision.
- Agent rule: before its first commit, every agent makes sure `pre-commit` and Commitizen are installed and the hooks are enabled, and never commits without them.
- Main application loop (issue #10): `cargo run` opens a full-screen TUI built with `ratatui`. It handles key presses, terminal resizes and a 100 ms tick, quits with `q` or `Ctrl+C`, and restores the terminal on exit, on error and on panic. A placeholder Welcome screen shows Japanese text centred by display width.
- `scripts/setup-dev.sh`: installs the development tools at the versions the repository pins (Rust with `rustfmt`, `clippy` and the musl target, OpenSpec, Commitizen, pre-commit) and enables the git hooks.
- Welcome screen (issue #19): shows "Press Enter to start" below the greeting. Enter continues to a placeholder area selection screen, which Ctrl+C quits; the real screen is issue #20.
- Area selection screen (issue #20): lists Vocabulary, Kanji and Grammar, with the first one highlighted by a "> " marker. Enter on an area with no questions shows "No questions in this area yet", and Enter there goes back to the list; all three areas are empty until their content exists. The areas are listed in one place, `src/areas.rs`, so adding one adds a line. Up and Down move the highlight and stop at the first and last area. Esc quits from the list (until the Thank you screen exists) and goes back from the "no questions" screen.
- Japanese answer input and checking (issue #18): romaji is read letter for letter into hiragana (`shi`/`si`, `kitte` → きって, `kin'en` → きんえん, `-` → ー; `nn` before a vowel is accepted both ways, so `konnichiha` and `konnnichiha` both match こんにちは) and katakana is folded to hiragana, so `takai`, `たかい`, `タカイ` and `高い` all match `高い`. Long vowels must be typed out (`toukyou`), mixed romaji and Japanese is wrong, and text that cannot be read gets one typo warning before it counts as wrong. Answers are typed into a text field that keeps IME text as typed, deletes whole characters with Backspace and is drawn by display width. No screen uses it yet; the practice screen is issue #21.
- Generic practice (issue #21): choosing an area that has questions opens a JLPT level picker (N5 to N1, only N5 ticked at first); Up and Down move, Esc goes back to area selection. Space ticks or unticks a level; with nothing ticked the picker says so and Enter does nothing. The ticks are kept while the app runs. Areas supply their questions through one contract, `Area::questions`: entries with their JLPT levels, each with prompts, accepted answers and an exercise kind (Japanese or meaning). An area without questions is reported as empty. Enter starts practice; levels without questions show "No questions at these levels yet", and Enter there goes back to the picker. Questions are picked at random, an entry at a time, and no entry repeats until every entry has been asked; the order is seeded from the clock. The question screen has an answer field for romaji or Japanese typed through an IME. Enter checks the answer (Japanese in any script, or an English meaning) and shows feedback with the accepted answers when it is wrong; an answer that cannot be read as Japanese gets a warning first. Enter goes to the next question and Esc stops. Until an area has content, only tests reach these screens. Each answer is counted once and the score is shown.

### Changed
- The minimum Rust version is now 1.88 (was 1.85), which `ratatui` requires.
- Versioning is manual and owner-only for now: agents and automation never change the version or run `cz bump`; release tags need explicit owner approval for the exact tag and commit, plus working signing, and publishing a tag needs separate approval.
