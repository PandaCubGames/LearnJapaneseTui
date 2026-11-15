# Proposal

## Why

`src/main.rs` prints a version banner and exits. There is no terminal setup, event handling or drawing, so the module picker, quizzes and input have nowhere to live. Every later change (issues #11 to #13) needs this loop first. Tracked in GitHub issue #10.

## What Changes

- Add a full-screen TUI: the app enters raw mode and the alternate screen on start, and always restores the terminal on normal exit, on error and on panic.
- Add the core loop: *draw → wait for an event or the next tick → update state*, repeated until the user quits.
- Handle three kinds of events: key presses, terminal resizes and a fixed tick (one named constant, 100 ms).
- Add a quit path: `q` on the placeholder screen, and `Ctrl+C` on every screen.
- Add a placeholder Welcome screen that proves the loop works. It shows Japanese text centred by display width, the terminal size, the elapsed time (driven by ticks) and the last character typed (proves IME text arrives as UTF-8).
- Split the code as `docs/architecture.md` §5 describes (`app`, `keys`, `session`, `ui`), so key handling, state changes and drawing are tested without a real terminal.
- Add the `ratatui` dependency (MIT), using the `crossterm` backend it re-exports (MIT).
- Raise `rust-version` in `Cargo.toml` from `1.85` to `1.88`, which `ratatui` 0.30 requires. The project version is not changed.
- `cargo run` no longer prints the version banner. It opens the TUI instead.

## Capabilities

### New Capabilities
- `app-shell`: the terminal lifecycle (enter, restore), the event loop (keys, resizes, ticks), quitting, and the first screen shown at start-up.

### Modified Capabilities

None. `openspec/specs/` is empty.

## Impact

- **Code:** `src/main.rs` is rewritten. New files: `src/app.rs`, `src/keys.rs`, `src/session.rs`, `src/ui.rs`.
- **Dependencies:** `ratatui` 0.30 and what it pulls in. Every licence in the tree must be GPL-3.0-compatible (`AGENTS.md` §6) and is checked as a task.
- **Toolchain:** minimum Rust goes from 1.85 to 1.88. `README.md` and `CONTRIBUTING.md` are updated if they name the old minimum.
- **Docs:** `CHANGELOG.md` gets an entry under *Unreleased*. `docs/architecture.md` keeps its "planned" status, with a note that the core loop now exists.
- **Not affected:** learning content, areas (Vocabulary, Kanji, Grammar), grading, romaji conversion, CI workflows and the project version.
