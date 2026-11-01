# tui-learnjapanese — Project Rules

These rules apply to every change in this repository, by humans and agents alike.

`README.md` (for users) and `CONTRIBUTING.md` (for human contributors) summarize these rules publicly. When a rule here changes, update those files in the same branch so they never contradict this file.

## 0. Project overview

- **What:** an application for learning Japanese.
- **Language:** Rust (stable toolchain, Cargo).
- **Interface:** TUI only, running in the terminal. No GUI and no web frontend.
- **Modular:** learning content is organized into independent modules the user picks from, for example:
  - Verbs (planned)
  - Vocabulary (planned)
  - More modules to be defined later.
  Adding a new module must not require changes to existing modules.
- **Japanese text everywhere:** the app must display and accept Japanese (kana, kanji) on any PC (Linux, macOS, Windows):
  - All text is UTF-8 internally.
  - Layout uses terminal display width, not byte or `char` count, because CJK characters take two columns.
  - Fonts are up to the user's terminal; the app can't bundle them.
- **Single-file distribution:** each release is one self-contained executable per platform. Users don't install anything else (no runtime, no separate data files, no installer).
  - Learning content and assets are embedded in the binary.
  - Linux builds are statically linked (`musl`), so they run on any distro.
  - Release targets: Linux x86_64/aarch64, macOS x86_64/aarch64, Windows x86_64.
- **Works over SSH:** the app relies only on standard terminal features: no mouse requirement, no image protocols, no local GUI or clipboard dependencies. It must handle resizes and terminals with limited color.
- **Input:** answers can be typed either way:
  - **Japanese** through the user's own IME (kana/kanji arrive as normal UTF-8 text).
  - **Romaji**, which the app converts to kana itself (e.g. `taberu` → `たべる`).
  Both are accepted wherever the app asks for Japanese.

## 1. Repository

- This project is a Git repository. The default branch is `main`.
- A remote (GitHub or Forgejo — not decided yet) will be added later. Until then, all work is local.
- Once a remote exists, merges into `main` go through a Pull Request / Merge Request.

## 2. Branching — every change gets its own branch

- **Never commit directly to `main`.** The only exception was the initial bootstrap commit.
- Every change — feature, fix, test, experiment, docs, refactor — starts on a new branch created from an up-to-date `main`.
- Branch naming: `<type>/<short-kebab-description>`
  - `feat/` new functionality
  - `fix/` bug fixes
  - `test/` tests or experiments
  - `docs/` documentation only
  - `refactor/` code restructuring with no behavior change
  - `chore/` tooling, config, dependencies
  - Example: `feat/hiragana-quiz`, `fix/romaji-input-crash`
- Keep one logical change per branch. Unrelated work goes on a separate branch.

## 3. Finishing a branch — ask before merging

When the tasks on a branch are done:

1. Make sure all tests pass and the working tree is clean.
2. Bump the version (see §4) and update `CHANGELOG.md` on the branch.
3. **Ask the user for approval to merge into `main`.** Present a short summary: what changed, the proposed version bump, and test results.
4. Only after the user approves:
   - **Now (no remote):** `git checkout main && git merge --no-ff --no-verify <branch>`, then tag the release. `--no-verify` is needed because the `no-commit-to-branch` hook blocks every commit on `main`, merge commits included. Use it **only** for user-approved merges, never to skip hooks on regular commits.
   - **Later (with remote):** push the branch and open a PR/MR to `main`; the user merges it.
5. Delete the branch after it has been merged.

Never merge, tag, push, or delete a branch without the user's explicit approval for that specific branch.

## 4. Versioning — Semantic Versioning `vMAJOR.MINOR.PATCH`

- The source of truth for the version is `version` in `Cargo.toml`. `VERSION` mirrors it (without the `v` prefix, e.g. `0.1.0`). Commitizen updates both; never edit them by hand.
- Releases are marked with annotated Git tags prefixed with `v`, e.g. `v0.1.0`.
- Bump rules:
  - **MAJOR** — incompatible/breaking changes (e.g. save-data or config format changes users must migrate).
  - **MINOR** — new backward-compatible functionality.
  - **PATCH** — backward-compatible bug fixes.
  - Docs/chore/test-only branches don't require a bump unless the user asks for one.
- While the version is `0.x.y`, the project is in initial development; breaking changes may bump MINOR instead of MAJOR.
- `1.0.0` marks the first stable release — only when the user decides.

- Version bumps are done with Commitizen: `cz bump` (on the branch, before asking to merge). It computes the bump from the commit history, updates `VERSION`, updates `CHANGELOG.md` and creates the `vX.Y.Z` tag. Use `cz bump --dry-run --yes` first to show the proposed version in the merge summary (`--yes` avoids an interactive prompt that otherwise hangs non-interactive shells, e.g. before the first tag exists).

## 5. Commits — Commitizen (`cz`)

- Commit messages follow Conventional Commits: `type(scope): summary`, e.g. `feat(quiz): add katakana mode`. Commitizen enforces this.
- Write commits with `cz commit` (interactive), or with `git commit` using a valid message. The `commit-msg` hook rejects messages that don't follow the format.
- Commitizen config lives in `.cz.toml`; hooks live in `.pre-commit-config.yaml`.
  - After cloning, run `pre-commit install --hook-type commit-msg --hook-type pre-commit`.
  - The hooks also block commits directly on `main`.
  - For Rust files, the hooks run `cargo fmt --check` and `cargo clippy -D warnings`. Code must be formatted and free of warnings to commit.

## 6. Open source hygiene — no secrets, no private data

This project will be published as open source. Treat everything committed, including history, as public forever.

- **Never commit:**
  - secrets: private keys, API tokens, passwords, `.env` files, certificates,
  - personal data: real names, emails, addresses, machine-specific paths like `/home/<user>/...`,
  - anything copied from private sources.
- **Configuration that needs secrets** reads them from environment variables or local files listed in `.gitignore`. Document the variable name, never the value.
- **Automatic checks:** the `gitleaks`, `detect-private-key` and `check-added-large-files` hooks scan every commit. Never bypass them on regular commits. If a hook flags something, remove it; don't silence the rule.
- **If a secret is ever committed:** stop, tell the user, and treat the secret as compromised (rotate it). Deleting it in a later commit is not enough, because it stays in history.
- **Before the first public push:** scan the full history with `gitleaks git .` and review commit author emails.

### License — GPL-3.0-or-later

- The project's code is licensed under the **GNU GPL v3.0 or later** (SPDX: `GPL-3.0-or-later`).
  - Full text in `LICENSE`. Never edit it.
  - `Cargo.toml` declares `license = "GPL-3.0-or-later"`.
- Copyright holder: **PandaCubGames** (the GitHub account), not a real name. See "Never commit" above.
- **Every source file starts with an SPDX header:**
  ```rust
  // SPDX-License-Identifier: GPL-3.0-or-later
  // Copyright (C) 2026 PandaCubGames
  ```
- **Contributions:** by submitting code, contributors agree it's licensed under GPL-3.0-or-later. No CLA (contributor license agreement).
- **Dependencies** must be GPL-3.0-compatible.
  - OK: MIT, Apache-2.0, BSD-2/3-Clause, ISC, Zlib, Unicode-3.0, MPL-2.0, LGPL, GPL-3.0.
  - Not allowed: GPL-2.0-only, proprietary or "non-commercial" licenses, SSPL, or no license at all.
  - Check a crate's license before adding it.
- **Learning content** (vocabulary, example sentences, dictionaries, kanji data) is tracked separately from code.
  - Only include content whose license allows redistribution and is compatible with GPL-3.0, e.g. CC0, CC BY, CC BY-SA 4.0. Examples of such sources: JMdict and KANJIDIC2 (CC BY-SA 4.0), Tatoeba (CC BY 2.0 FR).
  - Record each source's origin, license and required attribution in `ATTRIBUTION.md`, and show attribution in the app's About screen.
  - Never use content from commercial textbooks, paid courses or scraped websites.
  - Content written for this project is GPL-3.0-or-later, like the code.

## 7. Building and checking

- `cargo run` to run, `cargo test` to test, `cargo build --release` for a release binary in `target/release/`.
- The release profile (LTO, strip, `panic = "abort"`) keeps the output a single small executable.
- Before asking to merge, all of these must pass: `cargo fmt --all -- --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`.
- Small, focused commits with messages that explain *why*, not just *what*.
- **Author identity:** all commits use the GitHub noreply address, `PandaCubGames <38363869+PandaCubGames@users.noreply.github.com>`, set in the repo's local git config. Never commit with a personal email.
- **AI attribution:** commits made with Claude end with the co-author trailer only:
  ```
  Co-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>
  ```
  **Never add `Claude-Session:` lines or any other links to private sessions.**
