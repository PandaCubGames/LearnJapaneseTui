# tui-learnjapanese — Project Rules

These rules apply to every change in this repository, by humans and agents alike.

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
   - **Now (no remote):** `git checkout main && git merge --no-ff <branch>`, then tag the release.
   - **Later (with remote):** push the branch and open a PR/MR to `main`; the user merges it.
5. Delete the branch after it has been merged.

Never merge, tag, push, or delete a branch without the user's explicit approval for that specific branch.

## 4. Versioning — Semantic Versioning `vMAJOR.MINOR.PATCH`

- The current version lives in the `VERSION` file (without the `v` prefix, e.g. `0.1.0`).
- Releases are marked with annotated Git tags prefixed with `v`, e.g. `v0.1.0`.
- Bump rules:
  - **MAJOR** — incompatible/breaking changes (e.g. save-data or config format changes users must migrate).
  - **MINOR** — new backward-compatible functionality.
  - **PATCH** — backward-compatible bug fixes.
  - Docs/chore/test-only branches don't require a bump unless the user asks for one.
- While the version is `0.x.y`, the project is in initial development; breaking changes may bump MINOR instead of MAJOR.
- `1.0.0` marks the first stable release — only when the user decides.

## 5. Commits

- Use Conventional Commits: `type(scope): summary` — e.g. `feat(quiz): add katakana mode`.
- Small, focused commits with messages that explain *why*, not just *what*.
