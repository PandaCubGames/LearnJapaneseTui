# Tasks

Types, signatures and wording are in `design.md` (D-numbers point there). Every task is one commit (`AGENTS.md` §5): after each one, `cargo fmt --all -- --check`, `cargo clippy --all-targets -- -D warnings` and `cargo test` must pass. Do not change the project version (`Cargo.toml`, `VERSION`, `Cargo.lock`). No new source files are needed; if you add one anyway it starts with the SPDX header.

**Existing tests:** do not change, rename, move, skip, `#[ignore]` or delete any existing test, with the single owner-approved exception in task 2.1. Adding new tests, new helpers and new `use` lines inside the existing `mod tests` blocks is allowed. If any existing test fails, fix the code, not the test; if that is impossible, stop and report.

## 1. Start hint on the Welcome screen

- [ ] 1.1 Add the start hint.
  - `src/text.rs`: add `pub const START_HINT: &str = "Press Enter to start";` with the doc comment from D6, after `GREETING`.
  - `src/ui.rs`, `draw_welcome`: insert `Line::from(text::START_HINT)` and then `Line::default()` right after the blank line that follows the greeting, so the block has the nine lines of D5 in that order. Change nothing else in `draw_welcome`.
  - `CHANGELOG.md`, *Unreleased* / *Added*: add one entry, `Welcome screen (issue #19): shows "Press Enter to start" below the greeting.`
  - New tests in `src/text.rs` `mod tests`: `start_hint_is_plain_ascii_text` asserts `START_HINT == "Press Enter to start"` and `START_HINT.is_ascii()`.
  - New helpers in `src/ui.rs` `mod tests`: `fn row_index(rows: &[Vec<String>], needle: &str) -> usize` returns the index of the first row whose `concat()` contains `needle` (panic with a message if none); `fn buffer(session: &Session) -> Buffer` draws like `render` and returns `terminal.backend().buffer().clone()` (import `ratatui::buffer::Buffer` and `ratatui::style::Color` in the test module).
  - New tests in `src/ui.rs` `mod tests`:
    - `fresh_welcome_shows_the_start_hint`: at 80x24 the screen text contains `Press Enter to start`.
    - `welcome_lines_are_in_order`: at 80x24, `row_index` of `text::TITLE` < `"日"` < `text::START_HINT` < `"Size: "` < `"Last key: "` < `text::QUIT_HINT`.
    - `start_hint_is_centred_by_display_width`: at 80x24, in the hint's row, cell 30 is `P`, cell 49 is `t`, and cells 0..30 and 50..80 are `" "`.
    - `welcome_stays_centred_after_a_resize`: `Session::new(80, 24)`, apply `Action::Resized { width: 100, height: 30 }`, render; `greeting_row(&rows)[38] == "日"` and the hint's row has `P` at cell 40.
    - `welcome_draws_at_more_sizes_without_panicking`: render at 1x1, 5x2, 20x3 and 200x60.
    - `welcome_uses_no_colour`: at 80x24, every cell of `buffer(...)` has `fg == Color::Reset` and `bg == Color::Reset`.
  - Verify: the six new `ui` tests and the new `text` test pass, all 29 existing tests still pass unchanged, and fmt and clippy are clean. Commit, e.g. `feat(welcome): show how to start on the Welcome screen`.

## 2. Owner-approved edit of one existing test

- [ ] 2.1 **Owner-approved exception (issue #19, D8). This is the only existing test this change edits.** In `src/keys.rs`, test `other_keys_and_events_do_nothing`, change `for code in [KeyCode::Enter, KeyCode::Esc, KeyCode::Up] {` to `for code in [KeyCode::Esc, KeyCode::Up] {`. Change nothing else: keep the test's name, its loop body and its `Event::FocusGained` assertion. Do not touch any other test. No production code changes in this commit. Verify `cargo test keys` passes (Enter still does nothing at this point, so the test passes before and after) and `git diff --stat` shows only `src/keys.rs` and this `tasks.md`. Commit, e.g. `test(keys): stop pinning Enter as a no-op on Welcome`, and say in the commit body that the owner approved this edit in issue #19.

## 3. Enter continues to a placeholder area selection screen

- [ ] 3.1 Add `Action::Start`, `Screen::AreaSelection` and the placeholder, in one commit (D7 explains why it cannot be split).
  - `src/session.rs`: add the variant `AreaSelection` to `Screen` and `Start` (with its doc comment) to `Action`, as in D3. Add the `Action::Start` arm to `Session::apply` exactly as in D3.
  - `src/keys.rs`: in the `Screen::Welcome` arm, add `KeyCode::Enter if key.modifiers.is_empty() => Some(Action::Start),` as the first arm. Add `Screen::AreaSelection => None,` to `match screen`. Leave rules 1 to 3 as they are (D4).
  - `src/text.rs`: add `AREA_SELECTION_SOON` and `AREA_SELECTION_QUIT_HINT` from D6.
  - `src/ui.rs`: move the centring code (the `rows` computation, the `Layout`, and `render_widget`) out of `draw_welcome` into `fn draw_centred(frame: &mut Frame, lines: Vec<Line>)`, and call it from `draw_welcome`. Add `fn draw_area_selection(frame: &mut Frame)` that calls `draw_centred` with the three lines of D5 (`AREA_SELECTION_SOON`, an empty line, `AREA_SELECTION_QUIT_HINT`), no styling. Add `Screen::AreaSelection => draw_area_selection(frame),` to `draw`.
  - `CHANGELOG.md`: extend the entry from 1.1 with `Enter continues to a placeholder area selection screen, which Ctrl+C quits; the real screen is issue #20.`
  - `docs/architecture.md`: in the status note at the top, say the core loop and the Welcome screen exist, area selection is a placeholder, and everything else is still planned. Do not change anything else in the file.
  - New tests in `src/session.rs` `mod tests`:
    - `start_moves_from_welcome_to_area_selection`: after `Action::Start`, `screen() == Screen::AreaSelection`.
    - `start_is_one_way`: after `Start`, applying `Start`, `Typed('a')`, `Tick` and `Resized { width: 100, height: 30 }` leaves `screen() == Screen::AreaSelection`.
    - `start_does_not_quit`: after `Start`, `should_quit()` is false.
  - New tests in `src/keys.rs` `mod tests` (add a helper `fn on_area_selection(event: Event) -> Option<Action>` next to `on_welcome`):
    - `enter_starts_on_welcome`: Enter with no modifiers → `Some(Action::Start)`.
    - `enter_release_does_nothing`: Enter with `KeyEventKind::Release` on Welcome → `None`.
    - `modified_enter_does_nothing_on_welcome`: Enter with `CONTROL` and Enter with `ALT` → `None`.
    - `ctrl_c_quits_on_area_selection`: → `Some(Action::Quit)`.
    - `resize_is_followed_on_area_selection`: `Event::Resize(100, 30)` → `Some(Action::Resized { width: 100, height: 30 })`.
    - `other_keys_do_nothing_on_area_selection`: `q`, `a`, `あ`, Enter, Esc and Up with no modifiers, and `Event::FocusGained` → `None`.
    - `enter_then_ctrl_c_through_the_key_table`: with a `Session::new(80, 24)`, feed events through `action_for(session.screen(), &event)` and `apply` each `Some` result: Enter → screen is `AreaSelection`; `q` → still `AreaSelection` and not quitting; `Ctrl+C` → `should_quit()` is true.
  - New tests in `src/text.rs` `mod tests`: `area_selection_messages_are_plain_ascii_text` asserts the exact strings of D6 and that both are ASCII.
  - New tests in `src/ui.rs` `mod tests`:
    - `area_selection_placeholder_is_shown`: at 80x24 after `Action::Start`, the screen text contains `Area selection is coming soon.` and `Press Ctrl+C to quit`, and does not contain `日`, `Press Enter to start` or `Last key:`.
    - `area_selection_follows_a_resize`: `Start`, then `Resized { width: 100, height: 30 }`; the row holding `Press Ctrl+C to quit` has `P` at cell 40 and blanks in cells 0..40.
    - `area_selection_draws_at_any_size_without_panicking`: after `Start`, render at 1x1, 5x2, 20x3 and 200x60.
    - `area_selection_uses_no_colour`: at 80x24 after `Start`, every cell of `buffer(...)` has `fg == Color::Reset` and `bg == Color::Reset`.
  - Verify: all new tests pass, every existing test passes (including the one edited in 2.1, unchanged since), fmt and clippy are clean with no dead-code warning, and `git diff main -- VERSION Cargo.toml Cargo.lock` prints nothing. Commit, e.g. `feat(welcome): continue to area selection with Enter`.

## 4. Integration check

- [ ] 4.1 Check the built binary in a pseudo-terminal (no person needed): build with `cargo build --release`, run `target/release/tui-learnjapanese` under a pty of 80x24 (for example with Python's `pty` module, script kept outside the repository), and confirm in order: the output contains `Press Enter to start`; after sending `\r` it contains `Area selection is coming soon.`; after sending `q` the process is still running; after sending `\x03` (Ctrl+C) it exits with status 0. Record each result in the report. This task changes no files; commit only the ticked checkbox, e.g. `chore(openspec): record the Welcome screen terminal check`. If Enter does not move to the placeholder in the pty, stop and report instead of changing the key table.

## Workflow follow-up

- The owner runs `cargo run --release` once in a real terminal window to confirm by eye: the Welcome screen, Enter, the placeholder, Ctrl+C, and a resize on both screens.
- Archive order: this change archives on its own (ADDED requirements only). Archiving `add-main-application-loop` first is recommended so `app-shell` reaches `openspec/specs/`. This change must be archived before #20's change, which modifies `area-selection`.
