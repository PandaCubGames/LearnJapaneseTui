# Tasks

Types, signatures and wording are in `design.md` (D-numbers point there). Every task is one commit (`AGENTS.md` §5): after each one, `cargo fmt --all -- --check`, `cargo clippy --all-targets -- -D warnings` and `cargo test` must pass. Do not change the project version (`Cargo.toml`, `VERSION`, `Cargo.lock`) and do not add a dependency. Every new source file starts with:

```rust
// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 PandaCubGames
```

**Existing tests:** do not change, rename, move, skip, `#[ignore]` or delete any existing test, except the four in group 1, and each of those only after the owner has approved that specific test (design.md, Open Questions 6). Adding new tests, new test helpers, new test constants and new `use` lines inside the existing `mod tests` blocks is allowed. If any other existing test fails, fix the code, not the test; if that is impossible, stop and report.

**Tests added here must not pin** that a real area (Vocabulary, Kanji, Grammar) has no questions, that the list has exactly three areas, or that Enter on an area with questions stays on the list (design.md D11). Use `areas::fake::FakeArea` for those cases.

## 1. Existing tests that pin the placeholder (each needs the owner's approval)

Do these four tasks first, in order. Each changes only the named test, no production code, and passes both before and after the feature. **If the owner has not approved a task's specific test, stop before that task and report; do not start group 2.** In each commit body, say that the owner approved this edit for issue #20.

- [ ] 1.1 **Requires the owner's approval for this specific test before it is done.** In `src/keys.rs`, test `other_keys_do_nothing_on_area_selection`: replace the array `[KeyCode::Char('q'), KeyCode::Char('a'), KeyCode::Char('あ'), KeyCode::Enter, KeyCode::Esc, KeyCode::Up]` with `[KeyCode::Char('q'), KeyCode::Char('a'), KeyCode::Char('あ')]`. Keep the test's name, its loop body and its `Event::FocusGained` assertion. Verify `cargo test keys` passes and `git diff --stat` shows only `src/keys.rs` and this `tasks.md`. Commit, e.g. `test(keys): stop pinning Enter, Esc and Up as no-ops on area selection`.
- [ ] 1.2 **Requires the owner's approval for this specific test before it is done.** In `src/ui.rs`, delete the test `area_selection_placeholder_is_shown` (the whole `#[test] fn`). Keep the `started` helper (still used by `area_selection_draws_at_any_size_without_panicking` and `area_selection_uses_no_colour`). Verify `cargo test` and clippy pass and `git diff --stat` shows only `src/ui.rs` and this `tasks.md`. Commit, e.g. `test(ui): drop the area selection placeholder text test`.
- [ ] 1.3 **Requires the owner's approval for this specific test before it is done.** In `src/ui.rs`, delete the test `area_selection_follows_a_resize`. Keep `row_index` (used by the Welcome tests). Verify as in 1.2. Commit, e.g. `test(ui): drop the area selection placeholder resize test`.
- [ ] 1.4 **Requires the owner's approval for this specific test before it is done.** In `src/text.rs`, delete the test `area_selection_messages_are_plain_ascii_text`. Do not remove the constants yet (`ui` still draws them until 2.1). Verify as in 1.2 with `src/text.rs`. Commit, e.g. `test(text): drop the area selection placeholder wording test`.

## 2. The list of areas, and choosing an area

- [ ] 2.1 Show the list and let Enter choose an area, in one commit (design.md D9 explains why it cannot be split).
  - **`src/areas.rs` (new):** module doc `//! The practice areas the app knows about, listed in one place.` Then `mod grammar; mod kanji; mod vocabulary;`, `use` of the three structs, the trait `Area: std::fmt::Debug` with `fn name(&self) -> &'static str` and `fn has_questions(&self) -> bool` (doc comments from D2), `const ALL: &[&dyn Area] = &[&Vocabulary, &Kanji, &Grammar];` with the doc comment from D2, and `pub fn all() -> &'static [&'static dyn Area] { ALL }`. At the end, `#[cfg(test)] pub mod fake` with `FakeArea` exactly as in D3, then `#[cfg(test)] mod tests`.
  - **`src/areas/vocabulary.rs`, `src/areas/kanji.rs`, `src/areas/grammar.rs` (new):** each has a module doc (`//! The Vocabulary area. Its words and questions come with issue #11.`; Kanji: `#12`; Grammar: `#13`), `#[derive(Debug)] pub struct Vocabulary;` (`Kanji`, `Grammar`) and `impl Area for ...` with `name()` returning `text::AREA_VOCABULARY` (`AREA_KANJI`, `AREA_GRAMMAR`) and `has_questions()` returning `false`.
  - **`src/main.rs`:** add `mod areas;` right after `mod app;` (alphabetical order). Nothing else.
  - **`src/text.rs`:** remove `AREA_SELECTION_SOON` and `AREA_SELECTION_QUIT_HINT` and put the nine constants of D8, with their doc comments, in their place (between `QUIT_HINT` and `NO_KEY`).
  - **`src/session.rs`:** as in D4: `Screen::NoQuestions`; `Action::Choose` and `Action::Back` with their doc comments (not `Previous`/`Next` yet); fields `areas` and `highlighted`; `new` calls the new `pub fn with_areas(width, height, areas)`; `Start` also sets `highlighted = 0`; the `Choose` and `Back` arms, including the `// An area with questions starts practice here (issue #21).` comment; getters `areas()`, `highlighted()`, `highlighted_area()`.
  - **`src/keys.rs`:** replace `Screen::AreaSelection => None,` with a `match key.code` that has only the Enter arm of D5 (`Choose`, `Press` only, no modifiers) and `_ => None`. Add `Screen::NoQuestions` with only Enter → `Back` (no modifiers, `Press` only) and `_ => None`. No Up, Down or Esc yet. If clippy asks for a different form of these one-arm matches, use the form it suggests; the behaviour must stay the same.
  - **`src/ui.rs`:** `draw_area_selection(frame, session)` and the new `draw_no_questions(frame, session)` exactly as in D6, both through the unchanged `draw_centred`; add the `Screen::NoQuestions` arm to `draw`. Import `ratatui::text::Span`. Do not change `draw_welcome` or `draw_centred`.
  - **`CHANGELOG.md`**, *Unreleased* / *Added*, as the **last** entry of the list: `- Area selection screen (issue #20): lists Vocabulary, Kanji and Grammar, with the first one highlighted by a "> " marker. Enter on an area with no questions shows "No questions in this area yet", and Enter there goes back to the list; all three areas are empty until their content exists. The areas are listed in one place, \`src/areas.rs\`, so adding one adds a line.`
  - **`docs/architecture.md`:** in the status note at the top, replace `Only the core loop and the Welcome screen exist in code` … `here is still planned.` (the first three lines of the note) with `Only the core loop, the Welcome screen and area selection exist in code (\`app\`, \`areas\`, \`keys\`, \`session\` and \`ui\`). The areas have no questions yet. Everything else here is still planned.` In §5, at the end of the **Area contract** bullet, add: `Its first part exists: \`src/areas.rs\` lists the areas, each with a name and whether it has any questions.` Change nothing else.
  - **New tests, `src/areas.rs` `mod tests`:**
    - `the_list_starts_with_vocabulary_kanji_grammar`: the names of `all()` start with `["Vocabulary", "Kanji", "Grammar"]` (use `starts_with`, not equality).
    - `area_names_are_unique_and_not_empty`: no name in `all()` is empty, and no two are equal.
  - **New tests, one per area file `mod tests`:** `is_named_vocabulary` (`Vocabulary.name() == "Vocabulary"`), `is_named_kanji`, `is_named_grammar`. Do not test `has_questions` on the real areas.
  - **New tests, `src/text.rs` `mod tests`:**
    - `area_selection_wording_is_plain_ascii_text`: each of the nine D8 constants equals its exact string and `is_ascii()`.
    - `area_markers_are_the_same_width`: `AREA_MARKER.len() == 2`, `AREA_NO_MARKER == "  "`, `AREA_MARKER.starts_with('>')`.
  - **New tests, `src/session.rs` `mod tests`** (add `use crate::areas::{self, Area, fake::FakeArea};` and constants `EMPTY_THREE` = fakes named `"A"`, `"B"`, `"C"` with `has_questions: false`, and `ONE_FULL` = one fake `"Full"` with `has_questions: true`):
    - `new_session_lists_the_apps_areas`: `Session::new(80, 24).areas().len() == areas::all().len()` and `highlighted() == 0`.
    - `start_highlights_the_first_area`: `Session::new`, `Start`; `highlighted_area().map(|area| area.name()) == Some("Vocabulary")`.
    - `choosing_an_empty_area_shows_no_questions`: `with_areas(80, 24, EMPTY_THREE)`, `Start`, `Choose`; screen is `NoQuestions`, highlighted area is `"A"`, not quitting.
    - `choosing_an_area_with_questions_does_not_say_it_is_empty`: `ONE_FULL`, `Start`, `Choose`; screen is not `NoQuestions` and not quitting.
    - `back_returns_to_the_list`: `EMPTY_THREE`, `Start`, `Choose`, `Back`; screen is `AreaSelection`, `highlighted() == 0`.
    - `choose_and_back_act_only_on_their_own_screens`: `EMPTY_THREE`: `Choose` and `Back` on Welcome leave it on Welcome; after `Start`, `Back` leaves it on `AreaSelection`; after `Choose`, another `Choose` leaves it on `NoQuestions`.
    - `choosing_with_no_areas_does_nothing`: `with_areas(80, 24, &[])`, `Start`, `Choose`; screen is `AreaSelection` and `highlighted_area()` is `None`.
  - **New tests, `src/keys.rs` `mod tests`** (add a helper `fn on_no_questions(event: Event) -> Option<Action>` next to `on_area_selection`, `use crate::areas::{Area, fake::FakeArea};`, and a constant `EMPTY_ONE` = one fake `"Empty"` without questions):
    - `enter_chooses_on_area_selection`: Enter, no modifiers → `Some(Action::Choose)`.
    - `modified_or_held_enter_does_nothing_on_area_selection`: Ctrl+Enter, Alt+Enter and `with_kind(KeyCode::Enter, KeyEventKind::Repeat)` → `None`.
    - `enter_goes_back_on_no_questions`: → `Some(Action::Back)`.
    - `modified_or_held_enter_does_nothing_on_no_questions`: as above, on `NoQuestions` → `None`.
    - `ctrl_c_quits_on_no_questions`: → `Some(Action::Quit)`.
    - `resize_is_followed_on_no_questions`: `Event::Resize(100, 30)` → `Some(Action::Resized { width: 100, height: 30 })`.
    - `other_keys_do_nothing_on_no_questions`: `q`, `a`, `あ`, Up, Down (no modifiers) and `Event::FocusGained` → `None`. Do not include Esc (it goes back from 4.1 on).
    - `enter_through_the_key_table_with_an_empty_area`: `Session::with_areas(80, 24, EMPTY_ONE)`; feed events through `action_for(session.screen(), &event)` and `apply` each `Some` result, like `enter_then_ctrl_c_through_the_key_table`: Enter → `AreaSelection`; Enter → `NoQuestions`; Enter → `AreaSelection`; never quitting.
  - **New tests, `src/ui.rs` `mod tests`** (add helpers `fn started_with(areas: &'static [&'static dyn Area], width: u16, height: u16) -> Session` = `Session::with_areas` + `Start`, and `fn chosen_with(...)` = `started_with` + `Choose`; constants `EMPTY_KANJI` = one fake `"Kanji"` without questions, `FOUR` = fakes `"Alpha"`, `"Beta"`, `"Gamma"`, `"Delta"` without questions, `WIDE_PAIR` = fakes `"Vocabulary"` and `"漢字"` without questions):
    - `area_selection_lists_the_areas_in_order`: `started(80, 24)`: the screen contains `text::AREA_SELECTION_TITLE`; `row_index` of the title < `"Vocabulary"` < `"Kanji"` < `"Grammar"`; the screen does not contain `"Area selection is coming soon."`, `"日"`, `"Press Enter to start"` or `"Last key:"`.
    - `first_area_is_highlighted_with_a_marker`: `started(80, 24)`: exactly one row contains `"> "`, and it contains `"> Vocabulary"`.
    - `area_list_is_centred_by_display_width`: `started(80, 24)`: in the `Vocabulary` row, cell 34 is `">"`, cell 36 is `"V"` and cells 0..34 are `" "`; in the `Kanji` and `Grammar` rows, cells 34 and 35 are `" "` and cell 36 is `"K"` / `"G"`.
    - `area_names_line_up_by_display_width`: `started_with(WIDE_PAIR, 80, 24)`: the `Vocabulary` row has `"V"` at 36 and the `漢字` row has `"漢"` at 36.
    - `another_area_adds_a_line`: `started_with(FOUR, 80, 24)`: `row_index` of `"Alpha"` < `"Beta"` < `"Gamma"` < `"Delta"`.
    - `area_selection_hint_is_centred_below_the_list`: `started(80, 24)`: the hint row (`row_index(&rows, text::AREA_SELECTION_HINT)`) is below the `Grammar` row, has `"U"` at cell 17 and `" "` in cells 0..17.
    - `area_list_stays_centred_after_a_resize`: `started(80, 24)`, then `Resized { width: 100, height: 30 }`: the `Vocabulary` row has `"V"` at 46; the hint row has `"U"` at 27.
    - `area_selection_with_no_areas_still_draws`: `started_with(&[], 80, 24)`: the screen contains the title and the hint; also render at 1x1 without panicking.
    - `no_questions_screen_shows_the_area_and_the_message`: `chosen_with(EMPTY_KANJI, 80, 24)`: `row_index` of `"Kanji"` < `text::NO_QUESTIONS` < `text::NO_QUESTIONS_HINT`; the screen does not contain `text::AREA_SELECTION_TITLE` or `text::AREA_SELECTION_HINT`.
    - `no_questions_message_is_centred`: `chosen_with(EMPTY_KANJI, 80, 24)`: the message row has `"N"` at 25 and `" "` in cells 0..25.
    - `no_questions_draws_at_any_size_without_panicking`: `chosen_with(EMPTY_KANJI, w, h)` at 1x1, 5x2, 20x3 and 200x60.
    - `no_questions_uses_no_colour`: at 80x24, every cell of `buffer(...)` has `fg == Color::Reset` and `bg == Color::Reset`.
  - Verify: all new tests pass; every remaining existing test passes unchanged (in particular `enter_then_ctrl_c_through_the_key_table`, `area_selection_uses_no_colour`, `area_selection_draws_at_any_size_without_panicking` and `start_is_one_way`); fmt and clippy are clean with no dead-code warning; `git diff main -- VERSION Cargo.toml Cargo.lock` prints nothing. Commit, e.g. `feat(areas): list the areas and choose one on area selection`.

## 3. Moving the highlight

- [ ] 3.1 Up and Down move the highlight.
  - `src/session.rs`: add `Action::Previous` and `Action::Next` with their doc comments and the two arms of D4.
  - `src/keys.rs`: in the `Screen::AreaSelection` arm, add `KeyCode::Up` → `Previous` and `KeyCode::Down` → `Next`, both only without modifiers, repeats allowed (D5), before the Enter arm.
  - `CHANGELOG.md`: append to the entry from 2.1: ` Up and Down move the highlight and stop at the first and last area.`
  - New tests, `src/session.rs` `mod tests` (reuse `EMPTY_THREE`):
    - `next_and_previous_move_the_highlight`: `Start`, `Next` → 1, `Next` → 2, `Previous` → 1.
    - `highlight_stops_at_the_last_area`: `Start`, `Next` three times → 2.
    - `highlight_stops_at_the_first_area`: `Start`, `Previous` → 0.
    - `moving_only_works_on_area_selection`: `Next` on Welcome leaves `highlighted() == 0`; after `Start` and `Choose` (on `NoQuestions`), `Next` leaves it at 0.
    - `back_keeps_the_moved_highlight`: `Start`, `Next`, `Choose`: highlighted area is `"B"`; `Back`: screen is `AreaSelection` and `highlighted() == 1`.
    - `moving_with_no_areas_does_nothing`: `with_areas(80, 24, &[])`, `Start`, `Next`, `Previous` → `highlighted() == 0`.
  - New tests, `src/keys.rs` `mod tests`:
    - `arrows_move_on_area_selection`: Up → `Some(Action::Previous)`, Down → `Some(Action::Next)`.
    - `held_arrows_keep_moving`: `with_kind(KeyCode::Down, KeyEventKind::Repeat)` → `Next`; same for Up → `Previous`.
    - `modified_arrows_do_nothing_on_area_selection`: Shift+Down, Ctrl+Down, Alt+Up → `None`.
    - `unbound_keys_do_nothing_on_area_selection`: `j`, `k`, `1`, Left, Right and Tab (no modifiers) → `None`.
  - New tests, `src/ui.rs` `mod tests`:
    - `moving_the_highlight_moves_the_marker`: `started(80, 24)`, apply `Next`: the `Kanji` row has `">"` at 34 and the `Vocabulary` row has `" "` at 34.
  - Verify as in 2.1. Commit, e.g. `feat(areas): move the highlight with Up and Down`.

## 4. Leaving with Esc

- [ ] 4.1 Esc quits from the list and goes back from "no questions".
  - `src/keys.rs`: in `Screen::AreaSelection`, add `KeyCode::Esc` → `Some(Action::Quit)` (no modifiers, `Press` only). In `Screen::NoQuestions`, make the Enter arm `KeyCode::Enter | KeyCode::Esc` with the same guard (D5).
  - `CHANGELOG.md`: append to the entry: ` Esc quits from the list (until the Thank you screen exists) and goes back from the "no questions" screen.`
  - `docs/architecture.md`: in the status note, after `The areas have no questions yet.` add ` Esc on area selection quits directly until the Thank you screen exists.`
  - New tests, `src/keys.rs` `mod tests`:
    - `esc_quits_on_area_selection`: → `Some(Action::Quit)`.
    - `esc_goes_back_on_no_questions`: → `Some(Action::Back)`.
    - `modified_or_held_esc_does_nothing`: Alt+Esc and `with_kind(KeyCode::Esc, KeyEventKind::Repeat)` → `None` on both `AreaSelection` and `NoQuestions`.
    - `esc_through_the_key_table`: `Session::with_areas(80, 24, EMPTY_ONE)`, fed through `action_for` as in 2.1: Enter, Enter → `NoQuestions`; Esc → `AreaSelection`, not quitting; Esc → `should_quit()` is true.
  - Verify as in 2.1 (in particular `other_keys_and_events_do_nothing` still passes: Esc on Welcome still does nothing). Commit, e.g. `feat(areas): leave area selection with Esc`.

## 5. Integration check

- [ ] 5.1 Check the release binary in a pseudo-terminal (no person needed). Build with `cargo build --release`, run `target/release/tui-learnjapanese` under an 80x24 pty (e.g. Python's `pty` module; keep the script outside the repository). After each input, wait about 0.5 s and look only at the output that arrived **since that input** (`ratatui` redraws changed cells only). Confirm in order: at start the output contains `Press Enter to start`; after `\r` it contains `What do you want to practise?`; after `\x1b[B` (Down) and then `\r` it contains `Kanji` and `No questions in this area yet`; after `\r` it contains `What do you want to practise?`; after `q` the process is still running; after `\x1b` (Esc) the process exits with status 0 within 2 s. Record each result in the report. This task changes no files; commit only the ticked checkbox, e.g. `chore(openspec): record the area selection terminal check`. If a step fails, stop and report instead of changing the key table.

## Workflow follow-up

- The owner runs `cargo run --release` once in a real terminal to confirm by eye: Enter from Welcome, the marker moving with Up and Down, the "no questions" screen, Enter and Esc back, Esc quitting, and a resize on both screens.
- Archive order (design.md D1): archive `add-main-application-loop` (recommended) and `add-welcome-screen` (required) **before** this change. Archiving this change first does not fail but leaves the three placeholder requirements in `openspec/specs/area-selection/spec.md` once `add-welcome-screen` is archived.
