# Contributing to tui-learnjapanese

Thanks for your interest in helping! This guide explains how changes are made in this project. Please read it before opening a pull request.

## Quick start

Codex users should also read [Codex setup](docs/codex-setup.md).

1. **Install the tools:**
   - [Rust](https://www.rust-lang.org/tools/install) 1.85+ with `rustfmt` and `clippy`:
     ```sh
     rustup component add rustfmt clippy
     ```
   - [Commitizen](https://commitizen-tools.github.io/commitizen/), for commit messages and validation:
     ```sh
     pipx install commitizen
     ```
   - [pre-commit](https://pre-commit.com/), for the automatic checks:
     ```sh
     pipx install pre-commit
     ```
2. **Clone the repo and enable the hooks** (once per clone):
   ```sh
   pre-commit install --hook-type commit-msg --hook-type pre-commit
   ```
3. **Build and run:**
   ```sh
   cargo run
   ```

## Workflow

### 1. One branch per change

Never commit directly to `main`; the hooks will refuse. Start every change on a new branch from an up-to-date `main`:

```sh
git switch main
git pull
git switch -c feat/hiragana-quiz
```

Branch names follow `<type>/<short-kebab-description>`:

| Prefix | Use for |
|---|---|
| `feat/` | New functionality |
| `fix/` | Bug fixes |
| `test/` | Tests or experiments |
| `docs/` | Documentation only |
| `refactor/` | Restructuring without behavior changes |
| `chore/` | Tooling, config, dependencies |
| `ci/` | CI pipelines |

Keep one logical change per branch. Put unrelated work on a separate branch.

### 2. Commit with Conventional Commits

Commit messages must follow [Conventional Commits](https://www.conventionalcommits.org/):

```
type(scope): short summary

Optional body explaining *why* the change was made.
```

Examples:

```
feat(verbs): add te-form conjugation drill
fix(input): convert "nn" to ん correctly
docs: explain how to install a Japanese font
```

- Allowed types: `feat`, `fix`, `docs`, `style`, `refactor`, `perf`, `test`, `build`, `ci`, `chore`, `revert`.
- Mark breaking changes with `!` (e.g. `feat!: change save file format`) or a `BREAKING CHANGE:` footer.
- The easiest way to get this right is `cz commit`, which asks you for each part. Plain `git commit` works too, as long as the message follows the format.

**Signed commits are encouraged.** Signing shows your commits as "Verified" on GitHub. The simplest way is with an SSH key:
1. Create a key: `ssh-keygen -t ed25519 -f ~/.ssh/id_ed25519_github_signing`.
2. Add the `.pub` file on GitHub under Settings → SSH and GPG keys, with key type **Signing Key**.
3. Turn on signing in your clone:
   ```sh
   git config gpg.format ssh
   git config user.signingkey ~/.ssh/id_ed25519_github_signing.pub
   git config commit.gpgsign true
   ```

See [GitHub's guide](https://docs.github.com/en/authentication/managing-commit-signature-verification) for details.

### 3. Automatic checks

These run on every commit through pre-commit:

| Check | What it does |
|---|---|
| Commitizen | Rejects commit messages that don't follow the format |
| `cargo fmt` | Code must be formatted (`cargo fmt --all` fixes it) |
| `cargo clippy` | No lint warnings allowed |
| gitleaks, private-key check | Blocks commits containing secrets |
| Large-file check | Blocks accidentally added large files |
| Whitespace / YAML / TOML | Basic hygiene |
| No commits to `main` | Forces the branch workflow |

Don't bypass the hooks with `--no-verify`. If a check fails, fix the cause.

Before opening a pull request, make sure these pass:

```sh
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test
```

### 4. Open a pull request

- Push your branch and open a pull request against `main`.
- The **PR title must also follow Conventional Commits**, e.g. `feat(vocab): add JLPT N5 word list`.
- CI checks every commit in the PR and the title. The PR can't be merged until it passes.
- Describe what changed and why, and how you tested it.
- **Only the project owner can merge pull requests into `main`.** The owner is the only one with write access. `main` is protected: it only accepts pull requests with signed commits and a passing CI check, and can't be force-pushed or deleted. The owner reviews your PR and merges it with a merge commit, so keep your commits clean and meaningful.

## Versioning and releases

The project uses [Semantic Versioning](https://semver.org/) (`vMAJOR.MINOR.PATCH`):

- **MAJOR:** breaking changes.
- **MINOR:** new features.
- **PATCH:** bug fixes.

While the version is `0.x`, breaking changes bump MINOR.

**Only the owner changes the project version, by hand, for now.** Keep `Cargo.toml`, `VERSION`, and the root package version in `Cargo.lock` consistent. Agents and automation must leave the version unchanged and must not run `cz bump` or other automatic bump commands. Commitizen is used for commit messages and validation.

Agents may create an annotated, signed release tag only after the owner explicitly approves the exact tag and target commit, the manually set version matches, the tag is confirmed absent locally and remotely, and signing is available. If any prerequisite cannot be verified, leave the tag uncreated and report the blocker. Publishing the tag requires separate explicit approval. See `AGENTS.md` §4 for the complete rule.

## Code guidelines

- **Japanese text:**
  - Text is UTF-8 everywhere.
  - Measure display width in terminal columns, not bytes or `char`s. Kana and kanji are two columns wide.
- **Terminal compatibility:**
  - Everything must work over SSH and with the keyboard only: no mouse requirement, no images, no clipboard or GUI dependencies.
  - Handle terminal resizes and limited-color terminals.
- **Single executable:** learning content and assets are embedded in the binary. Don't add runtime file dependencies.
- **Modules:** a new learning module must not require changes to existing modules.

## Licensing

This project is licensed under **GPL-3.0-or-later**. By submitting a contribution, you agree that it's licensed under the same terms.

- **Every source file starts with this header:**
  ```rust
  // SPDX-License-Identifier: GPL-3.0-or-later
  // Copyright (C) 2026 PandaCubGames
  ```
  If you contribute a substantial new file, you may add your own copyright line below it.
- **Dependencies** must have a GPL-3.0-compatible license.
  - OK: MIT, Apache-2.0, BSD, ISC, Zlib, MPL-2.0, LGPL, GPL-3.0.
  - Not allowed: GPL-2.0-only, "non-commercial", proprietary or unlicensed crates.
- **Learning content** (vocabulary, example sentences, dictionary or kanji data) needs a redistributable, GPL-compatible license, such as CC0, CC BY or CC BY-SA 4.0. Add its source, license and attribution to [ATTRIBUTION.md](ATTRIBUTION.md) in the same pull request.
- **Never copy content from commercial textbooks, paid courses or websites** without explicit permission. Content you write yourself is fine.

## Security and privacy

This repository is public. Never commit:
- secrets: API keys, tokens, passwords, private keys, `.env` files,
- personal data, or paths from your machine.

The hooks scan for secrets, but you're responsible for what you commit. If you accidentally commit a secret, treat it as compromised and replace it. Deleting it in a later commit doesn't remove it from history.

## Using AI assistants

AI-assisted contributions are welcome. You're responsible for reviewing and understanding the code you submit. If an AI tool co-wrote a commit, credit it with a `Co-Authored-By:` trailer. Don't include links to private chat sessions in commit messages.

The repository is set up for both [Claude Code](https://claude.com/claude-code) and [Codex](https://openai.com/codex/):

- **Project rules** for agents are in [`AGENTS.md`](AGENTS.md). Codex reads it directly, and `CLAUDE.md` imports it for Claude Code. Put new rules in `AGENTS.md`.
- **OpenSpec skills** are in `.claude/skills/` (Claude Code) and `.agents/skills/` (Codex). They're generated by the OpenSpec CLI.
- **Subagents** (`spec-writer`, `spec-implementer`) are in `.claude/agents/` and `.codex/agents/`. If you change one, change both.
