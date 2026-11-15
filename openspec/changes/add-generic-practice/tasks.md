# Tasks

> **Prerequisite:** `add-japanese-answer-input` (#18) and `add-area-selection-screen` (#20) are both merged to `main`, and this branch starts from that `main`. Do not start otherwise.

Types, signatures, algorithms and wording are in `design.md` (D-numbers point there). Rules for every task:

- **One task, one commit** (`AGENTS.md` §5). After each task, `cargo fmt --all -- --check`, `cargo clippy --all-targets -- -D warnings` and `cargo test` pass. Tick the task's checkbox in this file in the same commit.
- Do not change the project version (`Cargo.toml`, `VERSION`, `Cargo.lock`) and do not add a dependency: `git diff main -- Cargo.toml Cargo.lock VERSION` stays empty.
- Every new source file starts with the SPDX header, then a `//!` module comment:
  ```rust
  // SPDX-License-Identifier: GPL-3.0-or-later
  // Copyright (C) 2026 PandaCubGames
  ```
- **Existing tests** (on `main`, including every test #18 and #20 added): do not change, rename, move, skip, `#[ignore]` or delete any of them. Do not edit #20's test helper `areas::fake::FakeArea`. Adding new tests, new test helpers, new test constants, local test types and new `use` lines inside existing `mod tests` blocks is allowed. If an existing test fails, fix the code; if that is impossible, stop and report (design.md D14).
- **Unused code** (design.md D13): add a method, field, constant or `Action` only in the task that gives it a non-test use. Never add `#[allow(...)]` or a new `#[expect(...)]`. Every field of a type must be read somewhere, at least by a test.
- If clippy asks for a different form of a short `match` (for example a one-arm key table such as 3.1's `Question` arm), use the form it suggests; the behaviour must stay the same.
- **Wording** comes from `src/text.rs` only. Tests find rows by content (`row_index`), never by a fixed row number.
- In the tests below, "pressed" means `KeyModifiers::NONE` and `KeyEventKind::Press`; use the existing helpers `press` and `with_kind` in `src/keys.rs` tests.

## 1. Level picker

- [ ] 1.1 Open the level picker after choosing an area with questions; Up/Down move, Esc goes back.
  - **First, check the prerequisites:** `src/areas.rs` has `trait Area` with `name` and `has_questions`, `areas::fake::FakeArea { name, has_questions }` exists, `Session::with_areas(u16, u16, &'static [&'static dyn Area])` and `Session::highlighted_area()` exist, `Action` has `Previous`, `Next`, `Choose`, `Back`, `Screen` has `AreaSelection` and `NoQuestions`, and `src/grading.rs` and `src/answer_field.rs` exist with the API listed in design.md Context. If anything differs, stop and report.
  - `src/practice.rs` (new): module doc `//! The generic practice module: what every area has in common. It never sees an area's own words, kanji or verbs, only questions (issue #21).` Then `mod level;` and `pub use level::{Level, Levels};`.
  - `src/practice/level.rs` (new): doc `//! JLPT levels and sets of levels.`; `Level` with `ALL` and private `bit`, and `Levels` with `only` and `contains` only (D2; derives as there).
  - `src/main.rs`: add `mod practice;` right after `mod keys;`. Nothing else.
  - `src/text.rs`: after #20's `NO_QUESTIONS_HINT`, add `LEVEL_PICKER_TITLE`, `LEVEL_PICKER_HINT`, `LEVEL_TICKED`, `LEVEL_UNTICKED` and `pub fn level_name(level: Level) -> &'static str` (D16), each with a doc comment (`/// Level picker: ...`). Import `crate::practice::Level`.
  - `src/session.rs`: add `Screen::LevelPicker`; fields `levels: Levels` (start `Levels::only(Level::N5)`) and `level_highlighted: usize` (start `0`), set in `with_areas`; getters `levels()` and `highlighted_level()` (D10). Change the `Choose` arm to D10's form (`has_questions()` → `LevelPicker` with `level_highlighted = 0`, else `NoQuestions`; drop #20's comment). Extend `Previous`/`Next` with the `LevelPicker` case (stop at 0 and at `Level::ALL.len() - 1`) and `Back` with `LevelPicker` → `AreaSelection`. Reword the doc comments of `Previous`, `Next` and `Back` as in D10.
  - `src/keys.rs`: add the `Screen::LevelPicker` arm with Up → `Previous`, Down → `Next` (no modifiers, repeats allowed), Esc pressed → `Back`, `_ => None` (D11). No Space and no Enter yet.
  - `src/ui.rs`: `fn draw_level_picker(frame: &mut Frame, session: &Session)` as in D12 without the last line (Space and the "Tick at least one level" line come in 1.2); add the arm in `draw`. Do not change `draw_centred` or the other screens.
  - `CHANGELOG.md`, *Unreleased* / *Added*, as the **last** item of the list: `- Generic practice (issue #21): choosing an area that has questions opens a JLPT level picker (N5 to N1, only N5 ticked at first); Up and Down move, Esc goes back to area selection.`
  - `docs/architecture.md`: in the status note, insert `The JLPT level picker exists (\`practice\`).` right before `Everything else here is still planned.`; in §2 *Levels*, after the bullet ending `has **any** of the chosen levels.`, add the bullet `- **Picker:** N5 to N1, each with a tick box. Up and Down move, Enter starts, Esc goes back to area selection. Only N5 is ticked when the app starts, and the ticks are kept while the app runs.`; in §6 delete the bullet `- **The level picker's default selection.**` (without the backslashes).
  - New tests, `src/practice/level.rs`: `all_lists_n5_to_n1` (`Level::ALL == [N5, N4, N3, N2, N1]`); `only_contains_just_that_level` (for every pair `l`, `m` of `Level::ALL`, `Levels::only(l).contains(m) == (l == m)`).
  - New tests, `src/text.rs`: `level_picker_wording_is_plain_ascii_text` (each of the four constants equals its exact D16 string and `is_ascii()`); `level_names` (`level_name` gives `"N5"` … `"N1"`); `tick_boxes_are_the_same_width` (both are 4 bytes and start with `[`).
  - New tests, `src/session.rs` (add `use crate::practice::{Level, Levels};` and constants `FULL_ONE` = one `FakeArea { name: "Full", has_questions: true }`, `FULL_TWO` = fakes `"Full A"` and `"Full B"`, both with questions; helper `fn on_picker(areas) -> Session` = `with_areas(80, 24, areas)`, `Start`, `Choose`):
    - `choosing_an_area_with_questions_opens_the_level_picker`: screen `LevelPicker`, `highlighted_level() == Level::N5`, `levels() == Levels::only(Level::N5)`.
    - `level_highlight_moves_and_stops`: `Next`, `Next`, `Previous` → `N4`; then `Previous` five times → `N5`; then `Next` six times → `N1`.
    - `moving_levels_keeps_the_area_highlight`: `FULL_TWO`, `Start`, `Next`, `Choose`, `Next` → `highlighted() == 1`.
    - `back_from_the_picker_returns_to_area_selection`: `FULL_TWO`, `Start`, `Next`, `Choose`, `Back` → `AreaSelection`, `highlighted() == 1`.
    - `picker_opens_on_n5_each_time`: `Next`, `Back`, `Choose` → `highlighted_level() == Level::N5`.
  - New tests, `src/keys.rs` (helper `fn on_level_picker(event: Event) -> Option<Action>`):
    - `arrows_move_on_the_level_picker`: Up → `Previous`, Down → `Next`, `with_kind(KeyCode::Down, KeyEventKind::Repeat)` → `Next`.
    - `esc_goes_back_on_the_level_picker`: Esc → `Back`; Alt+Esc and repeated Esc → `None`.
    - `ctrl_c_quits_on_the_level_picker` and `resize_is_followed_on_the_level_picker` (as #20's tests for area selection).
    - `unbound_keys_do_nothing_on_the_level_picker`: `q`, `a`, `x`, Left, Right, Tab and Shift+Down → `None`. Do not include Space or Enter.
  - New tests, `src/ui.rs` (constant `PICKABLE` = one `FakeArea { name: "Sample", has_questions: true }`; helper `fn on_picker(areas, width, height) -> Session`):
    - `level_picker_shows_the_levels_in_order`: `row_index` of `"Sample"` < `text::LEVEL_PICKER_TITLE` < `"N5"` < `"N4"` < `"N3"` < `"N2"` < `"N1"` < `text::LEVEL_PICKER_HINT`; the screen does not contain `text::AREA_SELECTION_TITLE`.
    - `level_picker_starts_with_n5_ticked_and_highlighted`: the `N5` row contains `"> [x] N5"`; the `N4`…`N1` rows contain `"[ ] N4"`… and no `">"`.
    - `level_lines_are_centred_by_display_width`: 80 wide: the `N5` row has `">"` at 36 and `"["` at 38; every other level row has `" "` at 36 and `"["` at 38.
    - `level_picker_stays_centred_after_a_resize`: `Resized { width: 100, height: 30 }`: every level row has `"["` at 48.
    - `moving_on_the_picker_moves_the_marker`: `Next`: the `N4` row has `">"` at 36, the `N5` row `" "` at 36.
    - `level_picker_draws_at_any_size_without_panicking` (1x1, 5x2, 20x3, 200x60) and `level_picker_uses_no_colour` (80x24, every cell `fg` and `bg` `Color::Reset`).
  - Verify: all new tests pass; every existing test passes unchanged, in particular #20's `choosing_an_area_with_questions_does_not_say_it_is_empty`; fmt and clippy clean. Commit, e.g. `feat(practice): open a JLPT level picker after choosing an area`.

- [ ] 1.2 Tick and untick levels with Space.
  - `src/practice/level.rs`: add `Levels::toggled` and `Levels::is_empty` (D2).
  - `src/session.rs`: `Action::Toggle` with its doc comment; arm: on `LevelPicker`, `levels = levels.toggled(highlighted_level())`.
  - `src/keys.rs`: in `LevelPicker`, `KeyCode::Char(' ')` pressed → `Toggle`.
  - `src/text.rs`: `NO_LEVEL_TICKED` (D16).
  - `src/ui.rs`: the last picker line is `text::NO_LEVEL_TICKED` when `session.levels().is_empty()`, else an empty line.
  - `CHANGELOG.md`: append to the entry from 1.1: ` Space ticks or unticks a level; with nothing ticked the picker says so and Enter does nothing. The ticks are kept while the app runs.`
  - `docs/architecture.md`: in the *Picker* bullet from 1.1, replace `Up and Down move, Enter starts` with `Up and Down move, Space ticks or unticks, Enter starts`, and add at its end ` With nothing ticked, the picker says so and Enter does nothing.`; in §2 *Keys*, replace `a key to tick or untick a level` with `Space to tick or untick a level`.
  - New tests, `src/practice/level.rs`: `toggled_ticks_and_unticks` (`only(N5).toggled(N4)` contains N5 and N4; toggling N4 again equals `only(N5)`); `empty_set` (`only(N5).toggled(N5).is_empty()`, `!only(N5).is_empty()`).
  - New tests, `src/session.rs`: `toggle_ticks_the_highlighted_level` (`Next`, `Toggle` → contains N5 and N4; `Toggle` → not N4); `unticking_the_last_level_leaves_none` (`Toggle` on N5 → `levels().is_empty()`); `ticks_are_kept_after_going_back` (`Next`, `Toggle`, `Back`, `Choose` → contains N5 and N4, `highlighted_level() == N5`); `toggle_only_works_on_the_picker` (`Toggle` on area selection leaves `levels() == only(N5)`).
  - New tests, `src/keys.rs`: `space_ticks_on_the_level_picker` (Space → `Toggle`; repeated Space and Ctrl+Space → `None`).
  - New tests, `src/text.rs`: `no_level_ticked_is_plain_ascii_text`.
  - New tests, `src/ui.rs`: `ticked_levels_show_an_x` (`Next`, `Toggle` → the `N4` row contains `"[x] N4"`); `nothing_ticked_says_so` (`Toggle` → screen contains `text::NO_LEVEL_TICKED`, below the hint row); `the_message_is_hidden_while_a_level_is_ticked` (fresh picker → screen does not contain it).
  - Verify as in 1.1. Commit, e.g. `feat(practice): tick JLPT levels with Space`.

## 2. The area contract

- [ ] 2.1 Add the contract types, `Area::questions`, the default `has_questions` and the test-only Sample area.
  - `src/practice/question.rs` (new): doc `//! A question and the entry it belongs to, as areas supply them.`; `Kind`, `Question`, `Entry` exactly as in D2, with the doc comments.
  - `src/practice.rs`: `mod question;`, `pub use question::{Entry, Kind, Question};`, and `pub fn selection` (D7) with its doc comment.
  - `src/practice/level.rs`: add `Levels::ALL` and `Levels::intersects` (D2).
  - `src/areas.rs`: the trait becomes D2's (add `questions` with its default body, give `has_questions` its default body, keep both names and signatures); `use crate::practice::{self, Entry, Levels};`; add `#[cfg(test)] pub mod sample;` after the other `mod` lines. Do not touch `ALL`, `all()` or `mod fake`.
  - `src/areas/vocabulary.rs`, `src/areas/kanji.rs`, `src/areas/grammar.rs`: delete only the `fn has_questions(&self) -> bool { false }` method (D2). Keep `name` and their tests.
  - `src/areas/sample.rs` (new, test only): doc `//! A test-only area with fixed questions at N5, N4 and N3. It proves that an area which only implements the contract can be practised (issue #21).`; `Sample`, `Only`, `READING`, `MEANING`, `POLITE_PAST` and `pub fn entries() -> Vec<Entry>` exactly as in D5. A private helper `fn question(kind: Kind, prompt: &str, accepted: &[&str], shown: &[&str]) -> Question` is fine.
  - `CHANGELOG.md`: append ` Areas supply their questions through one contract, \`Area::questions\`: entries with their JLPT levels, each with prompts, accepted answers and an exercise kind (Japanese or meaning). An area without questions is reported as empty.` (without the backslashes).
  - `docs/architecture.md` §5, **Area contract** bullet: replace #20's sentence `Its first part exists: ...` (up to its full stop) with, or if it is absent append: `It exists as \`Area::questions(levels)\` in \`src/areas.rs\`: the area's entries that have any of the chosen levels, each with its levels and its questions (prompt, accepted answers, the Japanese text the prompt shows, and the kind, Japanese or meaning). Whether an area has questions is worked out from it.`
  - New tests, `src/practice/level.rs`: `all_contains_every_level`; `intersects_needs_a_common_level` (`only(N5)` and `only(N5).toggled(N4)` intersect; `only(N5)` and `only(N1)` do not; the empty set intersects nothing, not even `ALL`).
  - New tests, `src/practice.rs` `mod tests` (helper `fn entry(levels: Levels, prompts: &[&str]) -> Entry` with one `Kind::Japanese` question per prompt, accepted `["あ"]`, shown empty): `selection_keeps_entries_with_any_chosen_level` (N5, N5+N4, N1 entries at levels N4+N3 → only the N5+N4 entry); `selection_drops_entries_without_questions`; `selection_keeps_the_order`.
  - New tests, `src/areas.rs` `mod tests` (local test types `Bare` = only `name`; `Hollow` = one N5 entry with no questions; `Levelless` = one entry with the empty level set and one question): `an_area_without_entries_has_no_questions`; `entries_without_questions_do_not_count`; `entries_without_a_level_do_not_count`; `every_listed_area_agrees_with_its_questions` (for each area in `all()`: `has_questions() == !practice::selection(area.questions(Levels::ALL), Levels::ALL).is_empty()`). Do not assert that any real area is empty.
  - New tests, `src/areas/sample.rs`: `sample_has_questions`; `n3_gives_only_the_mountain` (one entry, prompt `How do you read 山?`); `entries_carry_a_chosen_level` (at N5+N4 every entry intersects N5+N4); `n2_and_n1_give_nothing`; `reading_question` (all four fields of `entries()[0].questions[0]`); `meaning_question` (all four fields of `entries()[0].questions[1]`); `polite_past_question` (all four fields, and the entry's levels contain N5 and N4); `only_gives_one_question` (`READING.questions(Levels::ALL)` is one entry whose only question equals `entries()[0].questions[0]`; `MEANING` likewise with `[1]`); `only_respects_levels` (`READING.questions(Levels::only(Level::N3))` is empty).
  - New tests, `src/session.rs` (constant `SAMPLE: &[&dyn Area] = &[&Sample]`): `the_sample_area_opens_the_level_picker`.
  - Verify as in 1.1, and also that #20's `is_named_*` tests and `areas::tests` still pass. Commit, e.g. `feat(areas): let areas supply questions for chosen levels`.

- [ ] 2.2 Enter on the picker; "No questions at these levels yet".
  - `src/session.rs`: `Screen::NoQuestionsAtLevels`; `Action::Confirm` and `Action::Continue` with doc comments; `Confirm` on `LevelPicker` with a non-empty `levels` and a highlighted area: if `practice::selection(area.questions(levels), levels)` is empty → `NoQuestionsAtLevels`, else nothing yet (comment `// Practice starts here (task 3.1).`); `Continue` on `NoQuestionsAtLevels` → `LevelPicker`; `Back` also on `NoQuestionsAtLevels`.
  - `src/keys.rs`: in `LevelPicker`, Enter pressed → `Confirm`; new arm `NoQuestionsAtLevels`: Enter pressed → `Continue`, Esc pressed → `Back`, `_ => None`.
  - `src/text.rs`: `NO_QUESTIONS_AT_LEVELS`, `NO_QUESTIONS_AT_LEVELS_HINT`.
  - `src/ui.rs`: `fn draw_no_questions_at_levels(frame, session)` (D12) and its arm.
  - `CHANGELOG.md`: append ` Enter starts practice; levels without questions show "No questions at these levels yet", and Enter there goes back to the picker.`
  - New tests, `src/session.rs` (helper `fn sample_picker() -> Session` = `on_picker(SAMPLE)`): `levels_without_questions_say_so` (untick N5, tick N2 and N1, `Confirm` → `NoQuestionsAtLevels`); `levels_with_questions_do_not_say_so` (`Confirm` → not `NoQuestionsAtLevels`, not quitting); `confirm_with_nothing_ticked_does_nothing`; `continue_returns_to_the_picker_with_the_same_ticks`; `back_from_no_questions_at_levels_returns_to_area_selection`; `an_area_that_supplies_nothing_says_so_at_every_level` (`FULL_ONE`, `Confirm` → `NoQuestionsAtLevels`); `confirm_and_continue_act_only_on_their_own_screens` (`Confirm` on area selection, `Continue` on the picker and on area selection change nothing).
  - New tests, `src/keys.rs` (helper `fn on_no_questions_at_levels(event) -> Option<Action>`; constant `SAMPLE`): `enter_starts_on_the_level_picker` (Enter → `Confirm`; Ctrl+Enter and repeated Enter → `None`); `enter_and_esc_on_no_questions_at_levels` (`Continue`, `Back`); `other_keys_do_nothing_on_no_questions_at_levels` (`q`, `a`, Space, Up, Down → `None`); `ctrl_c_quits_on_no_questions_at_levels`; `no_questions_at_levels_through_the_key_table` (`Session::with_areas(80, 24, SAMPLE)`, feeding events through `action_for` and `apply` like #20's key-table tests: Enter, Enter → `LevelPicker`; Space; Down four times; Space; Enter → `NoQuestionsAtLevels`; Enter → `LevelPicker` with `levels() == Levels::only(Level::N1)`; Esc → `AreaSelection`; never quitting).
  - New tests, `src/text.rs`: `no_questions_at_levels_wording_is_plain_ascii_text`.
  - New tests, `src/ui.rs`: `no_questions_at_levels_shows_the_message` (`"Sample"` < message < hint; no `LEVEL_PICKER_TITLE`); `no_questions_at_levels_message_is_centred` (`"N"` at 24, spaces before); `no_questions_at_levels_draws_at_any_size_without_panicking`; `no_questions_at_levels_uses_no_colour`. Reach the screen with the Sample area and only N1 ticked.
  - Verify as in 1.1. Commit, e.g. `feat(practice): say when the chosen levels have no questions`.

## 3. The question screen

- [ ] 3.1 Pick questions at random and show the first one.
  - `src/practice/rng.rs` (new, D6) and `src/practice/pool.rs` (new, D7), with module docs. `src/practice.rs`: `mod pool; mod rng; pub use rng::Rng;`, `Drill` with only the fields `entries`, `entry`, `question`, and only `start` and `question` (D8). In this task `start` uses a local `Pool` and does not keep it: a stored `pool` field would never be read until `next` exists (4.1), and clippy rejects that.
  - `src/session.rs`: `Screen::Question`; private `const DEFAULT_SEED: u64 = 0;`; fields `rng` (`Rng::new(DEFAULT_SEED)`) and `drill: Option<Drill>` (`None`); `pub fn with_seed(self, seed: u64) -> Session`; `pub fn drill(&self) -> Option<&Drill>`; `Confirm` now uses `Drill::start` (D10) and removes the 2.2 comment; `Back` also on `Question` and sets `drill = None`.
  - `src/app.rs`: `pub fn seed_from(now: SystemTime) -> u64` (D6); `run_loop` builds `Session::new(...).with_seed(seed_from(SystemTime::now()))`. Nothing else in `app.rs` changes.
  - `src/keys.rs`: new arm `Question`: Esc pressed → `Back`, `_ => None` (3.2 replaces the fallback).
  - `src/text.rs`: `ANSWER_IN_JAPANESE`, `ANSWER_IN_ENGLISH`, `QUESTION_HINT`.
  - `src/ui.rs`: `draw_question` with the 13-row layout of D12 (`PROMPT_ROWS`, the helper `centred`); fill the name, prompt, how-to-answer and hint rows; leave the field, typo and score rows empty.
  - `CHANGELOG.md`: append ` Questions are picked at random, an entry at a time, and no entry repeats until every entry has been asked; the order is seeded from the clock.`
  - `docs/architecture.md` §2 *Questions*, at the end of the **Picking the next question** bullet: ` A new round never starts with the entry just asked. The random order comes from a small built-in generator seeded from the clock, so tests can fix it.`
  - New tests, `src/practice/rng.rs`: `seed_zero_matches_the_reference_values` (the three values in design.md Context fact 5); `same_seed_same_numbers`; `different_seeds_differ`; `below_stays_in_range` (every `n` in `1..=50`, 200 draws each, `< n`); `below_reaches_every_value` (`n = 5`, 200 draws, all of 0..5 seen); `below_zero_and_one_give_zero`.
  - New tests, `src/practice/pool.rs`: `empty_pool_gives_nothing`; `one_entry_every_time`; `every_entry_once_per_round` (len 3, seeds 0..20: picks 1-3, 4-6 and 7-9, each sorted, equal `[0, 1, 2]`); `no_entry_twice_in_a_row` (len 2 to 5, seeds 0..20, 50 picks); `same_seed_same_order` (len 4, 12 picks, two pools).
  - New tests, `src/practice.rs`: `start_with_nothing_practised_gives_none` (empty list; only an N3 entry with N5 chosen); `start_asks_a_practised_entry` (entries `a` at N5 and `b` at N1, N5 chosen, seeds 0..20 → prompt `a`); `start_skips_entries_without_questions`.
  - New tests, `src/session.rs` (constants `READING_ONLY = &[&sample::READING]`, `MEANING_ONLY = &[&sample::MEANING]`; helper `fn practising(areas, seed) -> Session` = `with_areas(80, 24, areas).with_seed(seed)`, `Start`, `Choose`, `Confirm`): `confirm_starts_practice` (`Question`, `drill()` is `Some`, prompt is one of the three N5 prompts of `sample::entries()`); `only_n3_asks_the_mountain`; `back_from_a_question_ends_practice` (`AreaSelection`, `drill()` is `None`); `same_seed_same_first_question` (seeds 0..10, two sessions each); `with_seed_keeps_the_welcome_screen`.
  - New tests, `src/app.rs`: `seed_from_the_epoch_is_zero`; `seed_mixes_seconds_and_nanoseconds` (`UNIX_EPOCH + Duration::new(1, 5)` → `(1 << 32) ^ 5`); `seed_before_the_epoch_is_zero`; `different_times_give_different_seeds` (1 ns apart).
  - New tests, `src/keys.rs` (helper `fn on_question(event) -> Option<Action>`): `esc_stops_on_the_question_screen` (Esc → `Back`; Alt+Esc and repeated Esc → `None`); `ctrl_c_quits_on_the_question_screen`; `resize_is_followed_on_the_question_screen`. Do not assert anything about letters, Space, Backspace or Enter there.
  - New tests, `src/text.rs`: `question_wording_is_plain_ascii_text`.
  - New tests, `src/ui.rs` (helper `fn practising(areas, width, height) -> Session`; a local test area `LongPrompt` with one N5 Japanese question whose prompt is `"word "` repeated 20 times, trimmed): `question_screen_shows_the_prompt` (`READING_ONLY`: `"Sample"` < `How do you read 高い?` < `text::ANSWER_IN_JAPANESE` < `text::QUESTION_HINT`; no `LEVEL_PICKER_TITLE`); `meaning_question_says_answer_in_english` (`MEANING_ONLY`: contains `text::ANSWER_IN_ENGLISH`, not `ANSWER_IN_JAPANESE`); `question_hint_is_centred` (`"E"` at 26); `long_prompt_wraps` (at least two rows contain `"word"`); `question_screen_draws_at_any_size_without_panicking`; `question_screen_uses_no_colour`.
  - Verify as in 1.1. Commit, e.g. `feat(practice): pick a random question and show it`.

- [ ] 3.2 Type the answer into a field on the question screen.
  - `src/practice.rs`: `Drill` gets the field `field: AnswerField` and the methods `field()` and `edit(edit)` (in this task `edit` only calls `self.field.apply(edit)`).
  - `src/session.rs`: `Action::Edit(Edit)` with its doc comment (`use crate::answer_field::Edit;`); arm: on `Question`, `drill.edit(e)`.
  - `src/keys.rs`: the `Question` fallback becomes `_ => answer_field::edit_for(event).map(Action::Edit)`.
  - `src/text.rs`: `ANSWER_LABEL`.
  - `src/ui.rs`: constants `FIELD_WIDTH: u16 = 30` and `ANSWER_LABEL_WIDTH: u16 = 8`; fill the field row as in D12, setting the cursor. Do not call `AnswerField::text()` in non-test code here.
  - `src/main.rs`: only if clippy now reports `unfulfilled_lint_expectations` on `mod answer_field;`, delete that attribute and its two comment lines (D13). Otherwise leave it for 4.1.
  - `CHANGELOG.md`: append ` The question screen has an answer field for romaji or Japanese typed through an IME.`
  - New tests, `src/practice.rs`: `edit_types_into_the_field` (`t a k a x`, `DeleteBack`, `i` → `field().text() == "takai"`).
  - New tests, `src/session.rs`: `typing_edits_the_answer` (`READING_ONLY`; `Edit(Insert('た'))`, `Edit(Insert('か'))` → `"たか"`); `edit_on_other_screens_does_nothing` (on the picker: screen stays, `drill()` is `None`); `back_drops_the_unfinished_answer` (type `たか`, `Back`, `Choose`, `Confirm` → field empty).
  - New tests, `src/keys.rs`: `typing_on_the_question_screen` (`a`, `q`, Space, `あ`, Shift+`A` → `Edit(Insert(..))`); `backspace_on_the_question_screen` (→ `Edit(DeleteBack)`); `held_keys_type_on_the_question_screen` (repeated `a`); `other_keys_do_nothing_on_the_question_screen` (Up, Tab, Ctrl+A → `None`); `typing_through_the_key_table` (`READING_ONLY`: feed `t a k a x`, Backspace, `i` → `"takai"`, then `q` → `"takaiq"`, not quitting).
  - New tests, `src/text.rs`: `answer_label_is_plain_ascii_text`.
  - New tests, `src/ui.rs` (helper `fn cursor(session: &Session) -> Position`, drawing on a `TestBackend` terminal and returning `terminal.get_cursor_position().unwrap()`): `answer_label_width_matches_the_text`; `answer_label_and_field_are_centred` (80 wide: the `"Answer: "` row has `"A"` at 21; cursor `(29, that row)`); `cursor_follows_the_typed_text` (type `たか` → cells 29 `"た"` and 31 `"か"`, cursor x 33); `field_stays_centred_after_a_resize` (100 wide → `"A"` at 31); `question_screen_with_text_uses_no_colour`.
  - Verify as in 1.1. Commit, e.g. `feat(practice): type the answer into the question screen`.

## 4. Answering

- [ ] 4.1 Submit, check and show feedback; Enter goes to the next question.
  - `src/practice/meaning.rs` (new): `Outcome`, `check` and `normalise` (D15), **without** the leading-word rule (4.3). `src/practice.rs`: `mod meaning;`.
  - `src/practice.rs`: `Drill` gets `pool` (now kept by `start`), `guard`, `typo_warning`, `feedback`; `Feedback` (D8); `submit` exactly as D9 without the counting lines; `next`; `typo_warning()`; `feedback()`; `edit` now does nothing while `feedback` is `Some` and clears `typo_warning`.
  - `src/session.rs`: `Screen::Feedback`; `Action::Submit` with its doc comment; `Submit` on `Question` → `Feedback` when `submit()` is `true`; `Continue` on `Feedback` → `drill.next(&mut self.rng)`, `Question`; `Back` also on `Feedback`.
  - `src/keys.rs`: in `Question`, Enter pressed → `Submit` (before the fallback); new arm `Feedback`: Enter pressed → `Continue`, Esc pressed → `Back`, `_ => None`.
  - `src/text.rs`: `TYPO_WARNING`, `CORRECT`, `WRONG`, `FEEDBACK_HINT`, `your_answer`, `read_as`, `expected` (D16).
  - `src/ui.rs`: the typo row of the question screen; `draw_feedback` with the 14-row layout of D12 (score row empty); its arm in `draw`.
  - `src/main.rs`: delete #18's `#[cfg_attr(not(test), expect(dead_code, ...))]` attribute and its two comment lines on each `mod` line for which clippy reports `unfulfilled_lint_expectations` (expected: `grading`, `kana`, `romaji`, and `answer_field` if 3.2 did not already). Afterwards no `expect(dead_code` is left in `src/main.rs`.
  - `CHANGELOG.md`: append ` Enter checks the answer (Japanese in any script, or an English meaning) and shows feedback with the accepted answers when it is wrong; an answer that cannot be read as Japanese gets a warning first. Enter goes to the next question and Esc stops. Until an area has content, only tests reach these screens.`
  - `docs/architecture.md`: in the status note replace `The JLPT level picker exists (\`practice\`).` with `The generic practice module exists (\`practice\`): level picker, question and feedback screens; only a test area has questions so far.`, and in #18's sentence about answer input and checking delete `, but no screen uses them yet` if present; in §2 *Answering and feedback*, add the bullet `- **Feedback** shows the answer, its reading when it differs, whether it was right, and every accepted answer when it was wrong.`
  - New tests, `src/practice/meaning.rs`: `same_meaning_is_correct`; `case_and_spaces_are_ignored` (`"  To   EAT "` against `"to eat"`); `other_meanings_are_wrong` (`cheap` against `high`, `tall`, `expensive`); `any_accepted_meaning_counts` (`tall`); `empty_answers_are_not_checked` (`""`, `"   "`, `"\t"`); `romaji_is_not_converted` (`takai` against `tall` → `Wrong`); `no_typo_for_unreadable_letters` (`xqzt` → `Wrong`).
  - New tests, `src/practice.rs` (helpers `fn drill_of(question: Question) -> Drill` and `fn type_text(drill: &mut Drill, text: &str)`): `correct_romaji_gives_feedback` (`takai` → `submit()` true; `Feedback { correct: true, answer: "takai", read_as: Some("たかい") }`); `correct_kana_has_no_reading` (`たかい` → `read_as: None`); `wrong_answer_keeps_its_reading` (`neko` → `correct: false`, `read_as: Some("ねこ")`; `ねこ` → `read_as: None`); `shown_text_is_not_accepted` (`高い` → wrong); `empty_submit_is_ignored` (`""`, `"  "` → `false`, no feedback); `typo_warns_first_then_counts` (`tabxru` → `false`, `typo_warning()`, field still `tabxru`; again → `true`, wrong); `editing_clears_the_typo_warning`; `meaning_answers_are_checked_as_english` (`Tall` → correct, `read_as: None`; `takai` → wrong); `no_edit_or_submit_during_feedback`; `next_starts_a_fresh_question` (no feedback, empty field, no warning); `next_follows_the_pool` (entries `a`, `b`, `c`, seeds 0..10, six questions: 1-3 distinct and 4-6 distinct); `both_questions_of_an_entry_are_asked` (one entry with two questions, 40 questions, both prompts seen).
  - New tests, `src/session.rs` (helper `fn answer(session: &mut Session, text: &str)` applying `Edit(Insert(c))` per char then `Submit`; helper `fn answer_correctly(session)` answering `question().accepted[0]`): `submit_shows_feedback`; `continue_shows_the_next_question` (`Question`, empty field); `back_from_feedback_returns_to_area_selection`; `submit_with_an_empty_answer_stays`; `typo_warning_stays_on_the_question`; `submit_and_continue_act_only_on_their_own_screens`; `only_n3_is_ever_asked` (20 answered questions, all `How do you read 山?`); `same_seed_same_questions` (all five levels ticked, seed 3, 10 questions answered correctly in two sessions → same prompts); `practice_goes_on_until_esc` (30 questions, still `Question`).
  - New tests, `src/keys.rs` (helper `fn on_feedback(event) -> Option<Action>`): `enter_submits_on_the_question_screen` (Enter → `Submit`; Ctrl+Enter and repeated Enter → `None`); `feedback_keys` (Enter → `Continue`, Esc → `Back`; `a`, Space, Backspace, Up, repeated Enter → `None`); `ctrl_c_quits_on_feedback`; `practice_start_to_finish_through_the_key_table`: `Session::with_areas(80, 24, SAMPLE).with_seed(1)`, all input through `action_for`: Enter, Enter, Down, Space, Down, Space, Enter → `Question`; six times: assert the prompt is one of `sample::entries()`'s prompts, type `accepted[0]` as `KeyCode::Char` events, Enter → `Feedback` with `correct`, Enter → `Question`; then type `ねこ`, Enter → `Feedback` and not correct (pick a question whose accepted answers do not contain `ねこ`, which is every Sample question); Esc → `AreaSelection` with `Sample` highlighted; never quitting.
  - New tests, `src/text.rs`: `practice_wording_is_plain_ascii_text`; `your_answer_and_read_as` (`"Your answer: takai"`, `"Read as: たかい"`); `expected_answers` (one → `"Correct answer: たかい"`, two → `"Correct answers: 食べました, たべました"`).
  - New tests, `src/ui.rs`: `typo_warning_is_shown` (`tabxru` submitted once → contains `text::TYPO_WARNING`, field row still shows `tabxru`); `feedback_for_a_correct_romaji_answer` (`Your answer: takai`, `Read as: たかい`, `Correct!`; no `Correct answer`); `feedback_for_a_correct_kana_answer` (no `Read as:`); `feedback_for_a_wrong_answer` (`POLITE_PAST`, `ねこ` → `Your answer: ねこ`, `Not quite.`, `Correct answers: 食べました, たべました`); `feedback_lines_are_in_order` (`"Sample"` < prompt < your answer < verdict < hint); `feedback_hint_is_centred` (`"P"` at 17); `feedback_draws_at_any_size_without_panicking`; `feedback_uses_no_colour`.
  - Verify as in 1.1; also `grep -n "expect(dead_code" src/main.rs` prints nothing. Commit, e.g. `feat(practice): check answers and show feedback`.

- [ ] 4.2 Count answers and show the score.
  - `src/practice.rs`: `Drill` gets `answered` and `correct` (0 in `start`), the counting lines of D9 in `submit`, and the getters `answered()` and `correct()`.
  - `src/text.rs`: `pub fn score(correct: u32, answered: u32) -> String`.
  - `src/ui.rs`: fill the score rows of the question and feedback screens.
  - `CHANGELOG.md`: append ` Each answer is counted once and the score is shown.`
  - `docs/architecture.md` §2 *Answering and feedback*: at the end of the bullet that starts `Each submitted answer is **counted once**`, add ` Until the results screen exists, the score is shown on the question and feedback screens and practice goes on until Esc.`
  - New tests, `src/practice.rs`: `each_answer_is_counted_once` (correct, `next`, wrong → `answered() == 2`, `correct() == 1`); `warnings_empty_submits_and_continuing_are_not_counted`; `a_warned_text_submitted_again_counts_as_wrong` (`answered() == 1`, `correct() == 0`).
  - New tests, `src/session.rs`: `counts_start_at_zero_each_time` (answer, `Back`, `Choose`, `Confirm` → `answered() == 0`).
  - New tests, `src/text.rs`: `score_text` (`"Score: 0 of 0"`, `"Score: 2 of 3"`).
  - New tests, `src/ui.rs`: `score_starts_at_zero`; `score_after_two_answers` (correct, Enter, wrong, Enter → `Score: 1 of 2` on the question screen); `continue_is_not_counted` (`Score: 1 of 1` on feedback and on the next question); `typo_warning_is_not_counted` (`Score: 0 of 0` with the warning shown, then `Score: 0 of 1` on feedback).
  - Verify as in 1.1. Commit, e.g. `feat(practice): count answers and show the score`.

- [ ] 4.3 Ignore a leading "to", "a", "an" or "the" in English answers.
  - `src/practice/meaning.rs`: the last step of `normalise` (D15).
  - `CHANGELOG.md`: append ` English answers ignore a leading "to", "a", "an" or "the".`
  - New tests, `src/practice/meaning.rs`: `leading_to_is_ignored_both_ways` (`eat` against `to eat`, `to eat` against `eat`); `articles_are_ignored` (`a mountain`, `the mountain`, `An mountain` against `mountain`); `only_a_whole_leading_word_is_removed` (`today` against `day` → `Wrong`).
  - New tests, `src/practice.rs`: `meaning_answer_without_to_is_correct` (a `Kind::Meaning` question accepting `to eat`, answer `eat` → correct).
  - Verify as in 1.1. Commit, e.g. `feat(practice): ignore a leading article in English answers`.

## 5. Integration check

- [ ] 5.1 Check the release binary in a pseudo-terminal (no person needed). Build with `cargo build --release` and run `target/release/tui-learnjapanese` under an 80x24 pty (e.g. Python's `pty` module; keep the script outside the repository). After each input wait about 0.5 s and look only at the output that arrived since that input. Confirm in order: at start the output contains `Press Enter to start`; after `\r` it contains `What do you want to practise?`; after `\r` (Vocabulary, which has no content yet) it contains `No questions in this area yet` and does **not** contain `Which JLPT levels`; after `\r` it contains `What do you want to practise?`; after `\x1b` (Esc) the process exits with status 0 within 2 s. Record each result in the report. This task changes no files: commit only the ticked checkbox, e.g. `chore(openspec): record the practice terminal check`. If a step fails, stop and report.

## Workflow follow-up

- The owner runs the real IME check once an area has Japanese questions (design.md Open Question 6): `たかい`, `タカイ`, `高い`, `takai` and Backspace on the question screen, on Linux, macOS, Windows and over SSH.
- Archive: this change adds new capabilities only, so it archives on its own, after #18's and #20's changes are archived.
- `CHANGELOG.md` and the status note in `docs/architecture.md` are also edited by #18 and #20; if they moved since this was written, keep their text and apply the edits above to the current wording.
