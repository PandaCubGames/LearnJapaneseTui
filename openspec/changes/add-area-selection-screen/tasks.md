# Tasks

Types, signatures and wording are in `design.md` (D-numbers point there). Every task is one commit (`AGENTS.md` §5): after each one, `cargo fmt --all -- --check`, `cargo clippy --all-targets -- -D warnings` and `cargo test` must pass. Do not change the project version (`Cargo.toml`, `VERSION`, `Cargo.lock`) and do not add a dependency. Every new source file starts with:

```rust
// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 PandaCubGames
```

**Existing tests:** the owner approved editing exactly four existing tests for issue #20, and asked that none be removed (design.md D10): `keys::tests::other_keys_do_nothing_on_area_selection` (task 1.1), and `ui::tests::area_selection_placeholder_is_shown`, `ui::tests::area_selection_follows_a_resize` and `text::tests::area_selection_messages_are_plain_ascii_text` (all three inside task 2.1). Make exactly the edits written in those tasks: keep each test's name and its place in the file, and do not delete any of them. Do not change, rename, move, skip, `#[ignore]` or delete any other existing test. Adding new tests, new test helpers, new test constants and new `use` lines inside the existing `mod tests` blocks is allowed. If any other existing test fails, fix the code, not the test; if that is impossible, stop and report.

**Tests added here must not pin** that a real area (Vocabulary, Kanji, Grammar) has no questions, that the list has exactly three areas, or that Enter on an area with questions stays on the list (design.md D11). Use `areas::fake::FakeArea` for those cases.

**Revision (owner feedback on the pull request):** groups 1 to 5 are implemented and stay ticked. Group 6 removes the three built-in areas, so the app registers no area, and adds a defined empty-list state (design.md D12). Tests added here must also not pin what `areas::all()` contains: not empty, not a length, not names (design.md D11).

## 1. Existing key test that pins Enter, Esc and Up as no-ops

Do this task first. It changes only the named test, no production code, and passes both before and after the feature. The other three approved edits cannot pass before the new screen exists, so they are part of task 2.1, not of this group.

- [x] 1.1 **Owner-approved edit of existing test `other_keys_do_nothing_on_area_selection` (issue #20).** In `src/keys.rs`, test `other_keys_do_nothing_on_area_selection`: replace the array `[KeyCode::Char('q'), KeyCode::Char('a'), KeyCode::Char('あ'), KeyCode::Enter, KeyCode::Esc, KeyCode::Up]` with `[KeyCode::Char('q'), KeyCode::Char('a'), KeyCode::Char('あ')]`. Keep the test's name, its place in the file, its loop body and its `Event::FocusGained` assertion. Verify `cargo test`, fmt and clippy pass and `git diff --stat` shows only `src/keys.rs` and this `tasks.md`. Commit, e.g. `test(keys): stop pinning Enter, Esc and Up as no-ops on area selection`; in the commit body, say that the owner approved this edit for issue #20.

## 2. The list of areas, and choosing an area

- [x] 2.1 Show the list and let Enter choose an area, in one commit (design.md D9 explains why it cannot be split). This commit also holds three owner-approved edits of existing tests (below), because they assert the new screen and wording and cannot pass before it.
  - **`src/areas.rs` (new):** module doc `//! The practice areas the app knows about, listed in one place.` Then `mod grammar; mod kanji; mod vocabulary;`, `use` of the three structs, the trait `Area: std::fmt::Debug` with `fn name(&self) -> &'static str` and `fn has_questions(&self) -> bool` (doc comments from D2), `const ALL: &[&dyn Area] = &[&Vocabulary, &Kanji, &Grammar];` with the doc comment from D2, and `pub fn all() -> &'static [&'static dyn Area] { ALL }`. At the end, `#[cfg(test)] pub mod fake` with `FakeArea` exactly as in D3, then `#[cfg(test)] mod tests`.
  - **`src/areas/vocabulary.rs`, `src/areas/kanji.rs`, `src/areas/grammar.rs` (new):** each has a module doc (`//! The Vocabulary area. Its words and questions come with issue #11.`; Kanji: `#12`; Grammar: `#13`), `#[derive(Debug)] pub struct Vocabulary;` (`Kanji`, `Grammar`) and `impl Area for ...` with `name()` returning `text::AREA_VOCABULARY` (`AREA_KANJI`, `AREA_GRAMMAR`) and `has_questions()` returning `false`.
  - **`src/main.rs`:** add `mod areas;` right after `mod app;` (alphabetical order). Nothing else.
  - **`src/text.rs`:** remove `AREA_SELECTION_SOON` and `AREA_SELECTION_QUIT_HINT` and put the nine constants of D8, with their doc comments, in their place (between `QUIT_HINT` and `NO_KEY`).
  - **`src/session.rs`:** as in D4: `Screen::NoQuestions`; `Action::Choose` and `Action::Back` with their doc comments (not `Previous`/`Next` yet); fields `areas` and `highlighted`; `new` calls the new `pub fn with_areas(width, height, areas)`; `Start` also sets `highlighted = 0`; the `Choose` and `Back` arms, including the `// An area with questions starts practice here (issue #21).` comment; getters `areas()`, `highlighted()`, `highlighted_area()`.
  - **`src/keys.rs`:** replace `Screen::AreaSelection => None,` with a `match key.code` that has only the Enter arm of D5 (`Choose`, `Press` only, no modifiers) and `_ => None`. Add `Screen::NoQuestions` with only Enter → `Back` (no modifiers, `Press` only) and `_ => None`. No Up, Down or Esc yet. If clippy asks for a different form of these one-arm matches, use the form it suggests; the behaviour must stay the same.
  - **`src/ui.rs`:** `draw_area_selection(frame, session)` and the new `draw_no_questions(frame, session)` exactly as in D6, both through the unchanged `draw_centred`; add the `Screen::NoQuestions` arm to `draw`. Import `ratatui::text::Span`. Do not change `draw_welcome` or `draw_centred`.
  - **`CHANGELOG.md`**, *Unreleased* / *Added*, as the **last** entry of the list: `- Area selection screen (issue #20): lists Vocabulary, Kanji and Grammar, with the first one highlighted by a "> " marker. Enter on an area with no questions shows "No questions in this area yet", and Enter there goes back to the list; all three areas are empty until their content exists. The areas are listed in one place, \`src/areas.rs\`, so adding one adds a line.`
  - **`docs/architecture.md`:** in the status note at the top, replace `Only the core loop and the Welcome screen exist in code` … `here is still planned.` (the first three lines of the note) with `Only the core loop, the Welcome screen and area selection exist in code (\`app\`, \`areas\`, \`keys\`, \`session\` and \`ui\`). The areas have no questions yet. Everything else here is still planned.` In §5, at the end of the **Area contract** bullet, add: `Its first part exists: \`src/areas.rs\` lists the areas, each with a name and whether it has any questions.` Change nothing else.
  - **Owner-approved edit of existing test `area_selection_placeholder_is_shown` (issue #20).** In `src/ui.rs`, keep the test's name, its `#[test]` attribute and its place in the file (between the `started` helper and `area_selection_follows_a_resize`). Change only its two positive `assert!` lines (`"Area selection is coming soon."` and `"Press Ctrl+C to quit"`). The full new body is:

    ```rust
    #[test]
    fn area_selection_placeholder_is_shown() {
        let screen = text(&render(&started(80, 24)));
        assert!(screen.contains("What do you want to practise?"));
        assert!(screen.contains("Up/Down to move, Enter to choose, Esc to leave"));
        for unexpected in ["日", "Press Enter to start", "Last key:"] {
            assert!(!screen.contains(unexpected), "unexpected {unexpected:?}");
        }
    }
    ```
  - **Owner-approved edit of existing test `area_selection_follows_a_resize` (issue #20).** In `src/ui.rs`, keep the test's name and place. Change only the row's needle (from `"Press Ctrl+C to quit"` to `text::AREA_SELECTION_HINT`), the expected symbol (`"P"` to `"U"`) and the column (both `40`s to `27`: at 100 columns the 46-column hint starts at `100/2 - 46/2 = 27`). The full new body is:

    ```rust
    #[test]
    fn area_selection_follows_a_resize() {
        let mut session = started(80, 24);
        session.apply(Action::Resized {
            width: 100,
            height: 30,
        });
        let rows = render(&session);
        let row = &rows[row_index(&rows, text::AREA_SELECTION_HINT)];
        assert_eq!(row[27], "U");
        assert!(row[..27].iter().all(|symbol| symbol == " "));
    }
    ```
  - **Owner-approved edit of existing test `area_selection_messages_are_plain_ascii_text` (issue #20).** In `src/text.rs`, keep the test's name and place (between `start_hint_is_plain_ascii_text` and `fixed_messages_are_not_empty`). It switches from the two removed placeholder constants to the two constants that take their place on the screen: the heading and the key hint. The full new body is (this is `cargo fmt` output, keep the line breaks):

    ```rust
    #[test]
    fn area_selection_messages_are_plain_ascii_text() {
        assert_eq!(AREA_SELECTION_TITLE, "What do you want to practise?");
        assert_eq!(
            AREA_SELECTION_HINT,
            "Up/Down to move, Enter to choose, Esc to leave"
        );
        assert!(AREA_SELECTION_TITLE.is_ascii());
        assert!(AREA_SELECTION_HINT.is_ascii());
    }
    ```
  - **New tests, `src/areas.rs` `mod tests`:**
    - `the_list_starts_with_vocabulary_kanji_grammar`: the names of `all()` start with `["Vocabulary", "Kanji", "Grammar"]` (use `starts_with`, not equality).
    - `area_names_are_unique_and_not_empty`: no name in `all()` is empty, and no two are equal.
  - **New tests, one per area file `mod tests`:** `is_named_vocabulary` (`Vocabulary.name() == "Vocabulary"`), `is_named_kanji`, `is_named_grammar`. Do not test `has_questions` on the real areas.
  - **New tests, `src/text.rs` `mod tests`:**
    - `area_and_no_questions_wording_is_plain_ascii_text`: each of `AREA_VOCABULARY`, `AREA_KANJI`, `AREA_GRAMMAR`, `NO_QUESTIONS` and `NO_QUESTIONS_HINT` equals its exact string from D8 and `is_ascii()`. (`AREA_SELECTION_TITLE` and `AREA_SELECTION_HINT` are checked by the edited `area_selection_messages_are_plain_ascii_text`; the markers by the next test.)
    - `area_markers_are_the_same_width`: `AREA_MARKER == "> "`, `AREA_NO_MARKER == "  "`, `AREA_MARKER.len() == AREA_NO_MARKER.len()`, and both `is_ascii()`.
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
    - `area_selection_lists_the_areas_in_order`: `started(80, 24)`: `row_index` of `text::AREA_SELECTION_TITLE` < `"Vocabulary"` < `"Kanji"` < `"Grammar"`; the screen does not contain `"Area selection is coming soon."`. (The title and hint being present, and `"日"`, `"Press Enter to start"` and `"Last key:"` being absent, are checked by the edited `area_selection_placeholder_is_shown`.)
    - `first_area_is_highlighted_with_a_marker`: `started(80, 24)`: exactly one row contains `"> "`, and it contains `"> Vocabulary"`.
    - `area_list_is_centred_by_display_width`: `started(80, 24)`: in the `Vocabulary` row, cell 34 is `">"`, cell 36 is `"V"` and cells 0..34 are `" "`; in the `Kanji` and `Grammar` rows, cells 34 and 35 are `" "` and cell 36 is `"K"` / `"G"`.
    - `area_names_line_up_by_display_width`: `started_with(WIDE_PAIR, 80, 24)`: the `Vocabulary` row has `"V"` at 36 and the `漢字` row has `"漢"` at 36.
    - `another_area_adds_a_line`: `started_with(FOUR, 80, 24)`: `row_index` of `"Alpha"` < `"Beta"` < `"Gamma"` < `"Delta"`.
    - `area_selection_hint_is_centred_below_the_list`: `started(80, 24)`: the hint row (`row_index(&rows, text::AREA_SELECTION_HINT)`) is below the `Grammar` row, has `"U"` at cell 17 and `" "` in cells 0..17.
    - `area_list_stays_centred_after_a_resize`: `started(80, 24)`, then `Resized { width: 100, height: 30 }`: the `Vocabulary` row has `">"` at 44 and `"V"` at 46; the `Kanji` row has `"K"` at 46. (The hint at 27 after the same resize is checked by the edited `area_selection_follows_a_resize`.)
    - `area_selection_with_no_areas_still_draws`: `started_with(&[], 80, 24)`: the screen contains the title and the hint; also render at 1x1 without panicking.
    - `no_questions_screen_shows_the_area_and_the_message`: `chosen_with(EMPTY_KANJI, 80, 24)`: `row_index` of `"Kanji"` < `text::NO_QUESTIONS` < `text::NO_QUESTIONS_HINT`; the screen does not contain `text::AREA_SELECTION_TITLE` or `text::AREA_SELECTION_HINT`.
    - `no_questions_message_is_centred`: `chosen_with(EMPTY_KANJI, 80, 24)`: the message row has `"N"` at 26 and `" "` in cells 0..26. (`ratatui` centres a line at `area_width / 2 - line_width / 2` with integer division: `40 - 14 = 26` for the 29-column message.)
    - `no_questions_draws_at_any_size_without_panicking`: `chosen_with(EMPTY_KANJI, w, h)` at 1x1, 5x2, 20x3 and 200x60.
    - `no_questions_uses_no_colour`: at 80x24, every cell of `buffer(...)` has `fg == Color::Reset` and `bg == Color::Reset`.
  - Verify: all new tests pass; the three edited tests pass with exactly the bodies above; every other existing test passes unchanged (in particular `enter_then_ctrl_c_through_the_key_table`, `area_selection_uses_no_colour`, `area_selection_draws_at_any_size_without_panicking` and `start_is_one_way`); `git diff main -- src/ui.rs src/text.rs` shows no removed `#[test]` and no renamed test; fmt and clippy are clean with no dead-code warning; `git diff main -- VERSION Cargo.toml Cargo.lock` prints nothing. Commit, e.g. `feat(areas): list the areas and choose one on area selection`; in the commit body, name the three edited tests and say that the owner approved these edits for issue #20.

## 3. Moving the highlight

- [x] 3.1 Up and Down move the highlight.
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

- [x] 4.1 Esc quits from the list and goes back from "no questions".
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

- [x] 5.1 Check the release binary in a pseudo-terminal (no person needed). Build with `cargo build --release`, run `target/release/tui-learnjapanese` under an 80x24 pty (e.g. Python's `pty` module; keep the script outside the repository). After each input, wait about 0.5 s and look only at the output that arrived **since that input** (`ratatui` redraws changed cells only). Confirm in order: at start the output contains `Press Enter to start`; after `\r` it contains `What do you want to practise?`; after `\x1b[B` (Down) and then `\r` it contains `Kanji` and `No questions in this area yet`; after `\r` it contains `What do you want to practise?`; after `q` the process is still running; after `\x1b` (Esc) the process exits with status 0 within 2 s. Record each result in the report. This task changes no files; commit only the ticked checkbox, e.g. `chore(openspec): record the area selection terminal check`. If a step fails, stop and report instead of changing the key table.

## 6. Owner feedback: no built-in areas

The owner does not want this change to add a Vocabulary, Kanji or Grammar area: the screen lists whatever the app has registered, and this change registers none (design.md D12). Do the tasks in order; each is one commit that builds and passes `cargo fmt --all -- --check`, `cargo clippy --all-targets -- -D warnings` and `cargo test` on its own.

**Do not touch the four owner-approved tests** (`keys::tests::other_keys_do_nothing_on_area_selection`, `ui::tests::area_selection_placeholder_is_shown`, `ui::tests::area_selection_follows_a_resize`, `text::tests::area_selection_messages_are_plain_ascii_text`) or any other test that exists on `main` (listed in design.md D10). They must pass unchanged after every task in this group. Only the tests named in the table below change, and they exist only on this branch. Do not add `#[allow(...)]` or any other lint suppression: if clippy reports dead code or anything else not described here, stop and report (design.md D13, Open Questions 11).

**Branch-only tests and what group 6 does with them** (every test not listed here stays exactly as it is):

| Test | File | Task | Action |
|---|---|---|---|
| `the_list_starts_with_vocabulary_kanji_grammar` | `src/areas.rs` | 6.2 | Delete (pinned the built-in areas). |
| `area_names_are_unique_and_not_empty` | `src/areas.rs` | | Keep unchanged (holds for any list; vacuous for the empty one, meaningful once areas are added). |
| `is_named_vocabulary` | `src/areas/vocabulary.rs` | 6.2 | Delete with the file. |
| `is_named_kanji` | `src/areas/kanji.rs` | 6.2 | Delete with the file. |
| `is_named_grammar` | `src/areas/grammar.rs` | 6.2 | Delete with the file. |
| `area_and_no_questions_wording_is_plain_ascii_text` | `src/text.rs` | 6.2 | Rewrite as `no_questions_wording_is_plain_ascii_text` (same place in the file), without the three area names. |
| `area_markers_are_the_same_width` | `src/text.rs` | | Keep. |
| `new_session_lists_the_apps_areas` | `src/session.rs` | | Keep (compares with `areas::all()`, true for any list). |
| `start_highlights_the_first_area` | `src/session.rs` | 6.2 | Rewrite with `EMPTY_THREE`: expects `Some("A")`. |
| all other `session` tests added by this branch | `src/session.rs` | | Keep (they use `EMPTY_THREE`, `ONE_FULL` or `&[]`). |
| all `keys` tests added by this branch | `src/keys.rs` | | Keep (they use `EMPTY_ONE` or no session). |
| `area_selection_lists_the_areas_in_order` | `src/ui.rs` | 6.2 | Rewrite with `THREE` (`First`, `Second`, `Third`). |
| `first_area_is_highlighted_with_a_marker` | `src/ui.rs` | 6.2 | Rewrite with `THREE`: the marked row contains `> First`. |
| `area_list_is_centred_by_display_width` | `src/ui.rs` | 6.2 | Rewrite with `THREE`: marker at 36, names at 38 (block 8 wide at 80). |
| `area_names_line_up_by_display_width` | `src/ui.rs` | | Keep (`WIDE_PAIR` is test data; `Vocabulary` and `漢字` both at 36). |
| `another_area_adds_a_line` | `src/ui.rs` | | Keep (`FOUR`). |
| `area_selection_hint_is_centred_below_the_list` | `src/ui.rs` | 6.2 | Rewrite with `THREE`: hint below `Third`, `U` at 17. |
| `area_list_stays_centred_after_a_resize` | `src/ui.rs` | 6.2 | Rewrite with `THREE`: at 100 wide, marker at 46, `First` and `Second` at 48. |
| `area_selection_with_no_areas_still_draws` | `src/ui.rs` | | Keep. |
| `no_questions_screen_shows_the_area_and_the_message` | `src/ui.rs` | 6.2 | Rewrite: constant `EMPTY_KANJI` (name `"Kanji"`) becomes `ONE_EMPTY` (name `"Only"`); the row needle `"Kanji"` becomes `"Only"`. |
| `no_questions_message_is_centred`, `no_questions_draws_at_any_size_without_panicking`, `no_questions_uses_no_colour` | `src/ui.rs` | 6.2 | Rewrite only the constant name `EMPTY_KANJI` → `ONE_EMPTY` (columns unchanged: the message is centred on its own). |
| `moving_the_highlight_moves_the_marker` | `src/ui.rs` | 6.2 | Rewrite with `THREE`: after `Next`, `Second` has `>` at 36 and `First` has `" "` at 36. |

- [x] 6.1 Show `Nothing to practise yet` when the list of areas is empty. The three built-in areas still exist in this task, so the real list does not change; the new state is reached only through `Session::with_areas(.., &[])`.
  - **`src/text.rs`:** add, right after `AREA_NO_MARKER` and before the `/// Area names, as shown in the list.` block:

    ```rust
    /// Area selection: shown in place of the list when the app has no areas.
    pub const AREA_SELECTION_EMPTY: &str = "Nothing to practise yet";
    ```
  - **`src/ui.rs`, `draw_area_selection`:** right after the line `let mut lines = vec![Line::from(text::AREA_SELECTION_TITLE), Line::default()];` insert:

    ```rust
        if session.areas().is_empty() {
            lines.push(Line::from(text::AREA_SELECTION_EMPTY));
        }
    ```
    Change nothing else in `ui.rs` outside `mod tests`.
  - **`src/session.rs`:** only the doc comment of `highlighted()` changes, to two lines: `/// The index of the highlighted area in \`areas()\`. It stays \`0\` when the` / `/// list is empty, and \`highlighted_area()\` is then \`None\`.` No code change (design.md D4, D12).
  - **New tests**, each appended at the end of the file's `mod tests` (this is `cargo fmt` output):
    - `src/text.rs`:

      ```rust
      #[test]
      fn empty_area_list_wording_is_plain_ascii_text() {
          assert_eq!(AREA_SELECTION_EMPTY, "Nothing to practise yet");
          assert!(AREA_SELECTION_EMPTY.is_ascii());
      }
      ```
    - `src/ui.rs` (uses the existing helpers `render`, `buffer`, `row_index`, `text`, `started_with` and the constant `FOUR`):

      ```rust
      #[test]
      fn empty_area_list_says_there_is_nothing_to_practise() {
          let rows = render(&started_with(&[], 80, 24));
          let order = [
              row_index(&rows, text::AREA_SELECTION_TITLE),
              row_index(&rows, text::AREA_SELECTION_EMPTY),
              row_index(&rows, text::AREA_SELECTION_HINT),
          ];
          assert!(order.windows(2).all(|pair| pair[0] < pair[1]), "{order:?}");
          assert!(!text(&rows).contains(text::AREA_MARKER));
      }

      #[test]
      fn a_list_with_areas_does_not_say_it_is_empty() {
          let screen = text(&render(&started_with(FOUR, 80, 24)));
          assert!(!screen.contains(text::AREA_SELECTION_EMPTY));
      }

      #[test]
      fn empty_area_list_is_centred() {
          let mut session = started_with(&[], 80, 24);
          let rows = render(&session);
          let empty = &rows[row_index(&rows, text::AREA_SELECTION_EMPTY)];
          assert_eq!(empty[29], "N");
          assert!(empty[..29].iter().all(|symbol| symbol == " "));
          assert_eq!(rows[row_index(&rows, text::AREA_SELECTION_HINT)][17], "U");
          session.apply(Action::Resized {
              width: 100,
              height: 30,
          });
          let rows = render(&session);
          let empty = &rows[row_index(&rows, text::AREA_SELECTION_EMPTY)];
          assert_eq!(empty[39], "N");
          assert!(empty[..39].iter().all(|symbol| symbol == " "));
          assert_eq!(rows[row_index(&rows, text::AREA_SELECTION_HINT)][27], "U");
      }

      #[test]
      fn empty_area_list_draws_at_any_size_without_panicking() {
          for (width, height) in [(1, 1), (5, 2), (20, 3), (200, 60)] {
              render(&started_with(&[], width, height));
          }
      }

      #[test]
      fn empty_area_list_uses_no_colour() {
          let buffer = buffer(&started_with(&[], 80, 24));
          assert!(
              buffer
                  .content()
                  .iter()
                  .all(|cell| cell.fg == Color::Reset && cell.bg == Color::Reset)
          );
      }
      ```
      (Columns: `40 - 23/2 = 29` and `50 - 23/2 = 39` for the 23-column line; `17` and `27` for the 46-column hint.)
    - `src/session.rs`:

      ```rust
      #[test]
      fn an_empty_list_highlights_nothing() {
          let mut session = Session::with_areas(80, 24, &[]);
          assert!(session.areas().is_empty());
          assert_eq!(session.highlighted(), 0);
          assert!(session.highlighted_area().is_none());
          session.apply(Action::Start);
          assert_eq!(session.screen(), Screen::AreaSelection);
          assert_eq!(session.highlighted(), 0);
          assert!(session.highlighted_area().is_none());
      }
      ```
    - `src/keys.rs`:

      ```rust
      #[test]
      fn only_esc_leaves_an_empty_list() {
          let mut session = Session::with_areas(80, 24, &[]);
          let feed = |session: &mut Session, event: Event| {
              if let Some(action) = action_for(session.screen(), &event) {
                  session.apply(action);
              }
          };
          feed(&mut session, press(KeyCode::Enter, KeyModifiers::NONE));
          assert_eq!(session.screen(), Screen::AreaSelection);
          for code in [KeyCode::Up, KeyCode::Down, KeyCode::Enter] {
              feed(&mut session, press(code, KeyModifiers::NONE));
              assert_eq!(session.screen(), Screen::AreaSelection, "{code:?}");
              assert_eq!(session.highlighted(), 0, "{code:?}");
              assert!(session.highlighted_area().is_none(), "{code:?}");
              assert!(!session.should_quit(), "{code:?}");
          }
          feed(&mut session, press(KeyCode::Esc, KeyModifiers::NONE));
          assert!(session.should_quit());
      }

      #[test]
      fn ctrl_c_quits_an_empty_list() {
          let mut session = Session::with_areas(80, 24, &[]);
          let feed = |session: &mut Session, event: Event| {
              if let Some(action) = action_for(session.screen(), &event) {
                  session.apply(action);
              }
          };
          feed(&mut session, press(KeyCode::Enter, KeyModifiers::NONE));
          assert_eq!(session.screen(), Screen::AreaSelection);
          feed(
              &mut session,
              press(KeyCode::Char('c'), KeyModifiers::CONTROL),
          );
          assert!(session.should_quit());
      }
      ```
  - No `CHANGELOG.md` or `docs/` change in this task (task 6.2 rewrites the entry once the built-in areas are gone).
  - Verify: check once that `empty_area_list_says_there_is_nothing_to_practise` fails with the `if` block in `draw_area_selection` taken out, then put it back. `cargo test` passes with 109 tests (100 before + 9 new); fmt and clippy are clean; `git diff --stat` shows only `src/text.rs`, `src/ui.rs`, `src/session.rs`, `src/keys.rs` and this `tasks.md`; `git diff` removes no line inside any `mod tests` block. Commit, e.g. `feat(areas): say when there is nothing to practise`.

- [x] 6.2 Remove the built-in areas, so the app registers none. One commit (design.md D9).
  - **Delete** `src/areas/vocabulary.rs`, `src/areas/kanji.rs` and `src/areas/grammar.rs` (with their tests `is_named_vocabulary`, `is_named_kanji`, `is_named_grammar`). The `src/areas/` directory is then gone.
  - **`src/areas.rs`:** delete the lines `mod grammar;`, `mod kanji;`, `mod vocabulary;`, `use grammar::Grammar;`, `use kanji::Kanji;`, `use vocabulary::Vocabulary;` and the blank lines after each group, so the module doc is followed by one blank line and the `Area` trait's doc comment. Replace the doc comment and value of `ALL` with:

    ```rust
    /// Every area, in the order the list shows them. There are none yet. Adding
    /// an area adds its own file under `src/areas/` and one entry here.
    const ALL: &[&dyn Area] = &[];
    ```
    Keep the `Area` trait, `all()`, `pub mod fake` and `FakeArea` exactly as they are. In `mod tests`, delete `the_list_starts_with_vocabulary_kanji_grammar`; keep `area_names_are_unique_and_not_empty` unchanged.
  - **`src/text.rs`:** delete the block `/// Area names, as shown in the list.`, `pub const AREA_VOCABULARY ...`, `pub const AREA_KANJI ...`, `pub const AREA_GRAMMAR ...` and its following blank line. In `mod tests`, replace `area_and_no_questions_wording_is_plain_ascii_text` (same place) with:

    ```rust
    #[test]
    fn no_questions_wording_is_plain_ascii_text() {
        for (text, expected) in [
            (NO_QUESTIONS, "No questions in this area yet"),
            (NO_QUESTIONS_HINT, "Press Enter to go back"),
        ] {
            assert_eq!(text, expected);
            assert!(text.is_ascii());
        }
    }
    ```
  - **`src/session.rs` `mod tests`:** the body of `start_highlights_the_first_area` becomes:

    ```rust
        let mut session = Session::with_areas(80, 24, EMPTY_THREE);
        session.apply(Action::Start);
        assert_eq!(name_of(&session), Some("A"));
    ```
  - **`src/ui.rs` `mod tests`:**
    - Add, right before the constant `EMPTY_KANJI`:

      ```rust
      const THREE: &[&dyn Area] = &[
          &FakeArea {
              name: "First",
              has_questions: false,
          },
          &FakeArea {
              name: "Second",
              has_questions: false,
          },
          &FakeArea {
              name: "Third",
              has_questions: false,
          },
      ];
      ```
    - Rename the constant `EMPTY_KANJI` to `ONE_EMPTY` (all five uses) and change its `name: "Kanji"` to `name: "Only"`. In `no_questions_screen_shows_the_area_and_the_message`, `row_index(&rows, "Kanji")` becomes `row_index(&rows, "Only")`.
    - `area_selection_lists_the_areas_in_order`: `render(&started(80, 24))` becomes `render(&started_with(THREE, 80, 24))`; the needles `"Vocabulary"`, `"Kanji"`, `"Grammar"` become `"First"`, `"Second"`, `"Third"`. The rest (title first, no `Area selection is coming soon.`) stays.
    - `first_area_is_highlighted_with_a_marker`: `started(80, 24)` becomes `started_with(THREE, 80, 24)`; `"> Vocabulary"` becomes `"> First"`.
    - `area_list_is_centred_by_display_width`, new body:

      ```rust
          let rows = render(&started_with(THREE, 80, 24));
          let first = &rows[row_index(&rows, "First")];
          assert_eq!(first[36], ">");
          assert_eq!(first[38], "F");
          assert!(first[..36].iter().all(|symbol| symbol == " "));
          for (name, initial) in [("Second", "S"), ("Third", "T")] {
              let row = &rows[row_index(&rows, name)];
              assert_eq!(row[36], " ");
              assert_eq!(row[37], " ");
              assert_eq!(row[38], initial);
          }
      ```
      (Block = marker 2 + `Second` 6 = 8 columns: `40 - 4 = 36`.)
    - `area_selection_hint_is_centred_below_the_list`: `started(80, 24)` becomes `started_with(THREE, 80, 24)`; `row_index(&rows, "Grammar")` becomes `row_index(&rows, "Third")`. `U` at 17 stays.
    - `area_list_stays_centred_after_a_resize`, new body:

      ```rust
          let mut session = started_with(THREE, 80, 24);
          session.apply(Action::Resized {
              width: 100,
              height: 30,
          });
          let rows = render(&session);
          let first = &rows[row_index(&rows, "First")];
          assert_eq!(first[46], ">");
          assert_eq!(first[48], "F");
          assert_eq!(rows[row_index(&rows, "Second")][48], "S");
      ```
    - `moving_the_highlight_moves_the_marker`, new body:

      ```rust
          let mut session = started_with(THREE, 80, 24);
          session.apply(Action::Next);
          let rows = render(&session);
          assert_eq!(rows[row_index(&rows, "Second")][36], ">");
          assert_eq!(rows[row_index(&rows, "First")][36], " ");
      ```
    - The helper `started` stays: the owner-approved tests and `area_selection_draws_at_any_size_without_panicking` / `area_selection_uses_no_colour` still use it, now with the real (empty) list.
  - **`CHANGELOG.md`**, *Unreleased* / *Added*: replace the whole line that starts with `- Area selection screen (issue #20):` with:

    `- Area selection screen (issue #20): lists the areas the app has registered, with the first one highlighted by a "> " marker. Up and Down move the highlight and stop at the first and last area. Enter on an area with no questions shows "No questions in this area yet", and Enter or Esc there goes back to the list. Esc quits from the list (until the Thank you screen exists). No area is registered yet, so the screen says "Nothing to practise yet"; each area adds its own file under \`src/areas/\` and one entry in \`src/areas.rs\`.`
  - **`docs/architecture.md`:** in the status note, replace `The areas have no questions yet.` with `No area is registered yet, so area selection says "Nothing to practise yet".` In §5, replace `Its first part exists: \`src/areas.rs\` lists the areas, each with a name and whether it has any questions.` with `Its first part exists: \`src/areas.rs\` lists the registered areas, each with a name and whether it has any questions. None is registered yet; each area adds its own file under \`src/areas/\` and one entry in that list.` Change nothing else (the planned design that names Vocabulary, Kanji and Grammar stays as it is).
  - Verify: `cargo test` passes with 105 tests (109 − 4 deleted); fmt, `cargo clippy --all-targets -- -D warnings` and `cargo clippy -- -D warnings` are clean with no `#[allow]` added (if not, stop and report, design.md D13); `git grep -n -E 'Vocabulary|Kanji|Grammar' -- src` shows only the `WIDE_PAIR` test data in `src/ui.rs` (`name: "Vocabulary"` and the needle in `area_names_line_up_by_display_width`); `git diff HEAD -- src` changes none of the four owner-approved tests and no test that exists on `main` (design.md D10); `git diff main -- VERSION Cargo.toml Cargo.lock` prints nothing. Commit, e.g. `feat(areas): list only registered areas and register none`; in the commit body say the owner asked for no built-in areas in the pull request review.

- [ ] 6.3 Re-run the pseudo-terminal check on the release binary (no file changes besides this checkbox). Same method as 5.1: `cargo build --release`, run `target/release/tui-learnjapanese` under an 80x24 pty with a script kept outside the repository; after each input wait about 0.5 s and look only at the output that arrived since that input. Confirm in order: at start the output contains `Press Enter to start`; after `\r` it contains `What do you want to practise?`, `Nothing to practise yet` and `Up/Down to move, Enter to choose, Esc to leave`; after `\x1b[B` (Down), then `\x1b[A` (Up), then `\r` (Enter), the process is still running after each and the output since each input does not contain `No questions in this area yet`; after `q` the process is still running; after `\x1b` (Esc) the process exits with status 0 within 2 s. Record each result in the report. Commit only the ticked checkbox, e.g. `chore(openspec): record the empty area selection terminal check`. If a step fails, stop and report.

## Workflow follow-up

- The owner runs `cargo run --release` once in a real terminal to confirm by eye: Enter from Welcome shows `Nothing to practise yet` with the heading and the hint, Up, Down and Enter do nothing there, Esc quits, and a resize keeps it centred. The list with areas, the marker and the "no questions" screen are covered by tests with test-only areas and become visible once #11, #12 or #13 registers an area.
- Archive order (design.md D1): archive `add-main-application-loop` (recommended) and `add-welcome-screen` (required) **before** this change. Archiving this change first does not fail but leaves the three placeholder requirements in `openspec/specs/area-selection/spec.md` once `add-welcome-screen` is archived.
