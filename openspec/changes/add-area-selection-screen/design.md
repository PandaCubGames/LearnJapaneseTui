# Design

## Context

See `proposal.md` for the motivation and `specs/area-selection/spec.md` for the required behaviour.

- The code has the shape from #10 and #19: `keys::action_for(screen, event)` turns an event into an `Action` per `Screen`, `Session::apply` is the only place state changes, and `ui::draw` matches on `Session::screen()`. `Screen` is matched exhaustively in `keys` and `ui`, so a new variant needs an arm in both.
- #19 left `Screen::AreaSelection`, `Action::Start` and its one-way transition, an empty key arm (`Screen::AreaSelection => None`), `ui::draw_area_selection` drawing a placeholder through the private helper `ui::draw_centred`, and the constants `text::AREA_SELECTION_SOON` and `text::AREA_SELECTION_QUIT_HINT`. This change keeps the first three and replaces the rest (`add-welcome-screen` design D2, "What #20 inherits").
- The crate is a binary: `cargo clippy --all-targets -- -D warnings` fails on any item, field or constant that non-test code never uses. This decides the task order (D9) and why the placeholder constants must go.
- `openspec/specs/` is empty. `area-selection` exists only as an ADDED delta of the unarchived change `add-welcome-screen` (complete, 4/4 tasks).
- The baseline is 51 passing tests (`cargo test`, run while writing this design). Issue #20's "Existing tests" section says nothing conflicts; that was written before #19 merged the placeholder tests, and is no longer true (D10).
- Three facts were checked in a scratch crate (same toolchain, pinned `ratatui` 0.30) before relying on them: a centred `Paragraph` line keeps its trailing spaces (so padded lines of equal width start in the same column); `const ALL: &[&dyn Area] = &[&Vocabulary, ...]` compiles and can be returned as `&'static [&'static dyn Area]`; and with this module layout, clippy `-D warnings` rejects `Area::has_questions` and `FakeArea::has_questions` while no non-test code calls `has_questions`, and accepts them once `Session::apply` does.

## Goals / Non-Goals

**Goals:**
- One place lists the app's areas, and the screen reads its lines from there (D2). This is the seam #21 grows.
- Everything on both screens is testable without a terminal, including the "another area adds a line" rule, through test-only areas (D3).
- No test added here pins a fact that #11, #12, #13 or #21 will legitimately change, so those issues do not inherit test conflicts (D11).

**Non-Goals:**
- Questions, JLPT levels, the practice screens, what Enter does on an area that has questions (all #21).
- Any content (#11, #12, #13).
- The Thank you screen. Esc quits directly.
- Marking empty areas in the list (D7, Open Questions).

## Decisions

### D1. Spec delta: REMOVED + ADDED on `area-selection`, archived after `add-welcome-screen`

The delta removes the three placeholder requirements and adds eleven new ones. It does not use MODIFIED.

- **Why not MODIFIED:** every placeholder requirement is replaced, not adjusted. Their names ("...placeholder...") would be wrong afterwards, and a MODIFIED block must restate the whole requirement anyway. REMOVED with a Reason and a Migration says plainly what happened.
- **Why not a new capability** (e.g. `area-list`) with ADDED only: the placeholder requirements would then stay in `openspec/specs/area-selection/spec.md` for ever, contradicting the new ones ("every other key does nothing" against "Esc quits").
- **Archive order, verified** on a scratch copy of `openspec/` with OpenSpec 1.16.0:
  - `add-main-application-loop` → `add-welcome-screen` → `add-area-selection-screen`: all three archive; `area-selection` ends with exactly the eleven new requirements (`+ 11, - 3`), and `openspec validate --specs --strict` passes.
  - Archiving this change **before** `add-welcome-screen` does **not** fail. It prints only a warning ("3 REMOVED requirement(s) ignored for new spec") and creates `area-selection` from the ADDED part. Archiving `add-welcome-screen` afterwards then silently adds the three placeholder requirements back, leaving 14 contradictory requirements. So the order is a rule the owner must follow, not one the tool enforces.
  - `add-main-application-loop` is independent of this change (it owns `app-shell`); archiving it first is recommended, not required.
- `openspec validate add-area-selection-screen --strict` passes now, before any archive.

### D2. The seam for #21: `src/areas.rs`

```rust
// src/areas.rs
mod grammar;
mod kanji;
mod vocabulary;

/// One thing the user can practise. #21 grows this into the full area
/// contract (questions for a set of JLPT levels).
pub trait Area: std::fmt::Debug {
    /// The name shown in the list of areas.
    fn name(&self) -> &'static str;
    /// Whether the area has any question at all. An area without questions
    /// can still be chosen; it says so.
    fn has_questions(&self) -> bool;
}

/// Every area, in the order the list shows them. Adding an area adds one
/// entry here and its own file under `src/areas/`.
const ALL: &[&dyn Area] = &[&Vocabulary, &Kanji, &Grammar];

pub fn all() -> &'static [&'static dyn Area] { ALL }
```

- `src/areas/vocabulary.rs`, `src/areas/kanji.rs`, `src/areas/grammar.rs` each hold one unit struct (`pub struct Vocabulary;` etc., `#[derive(Debug)]`) implementing `Area`: `name()` returns `text::AREA_VOCABULARY` (`AREA_KANJI`, `AREA_GRAMMAR`), `has_questions()` returns `false`. Their module docs name the issue that fills them: #11 (vocabulary), #12 (kanji), #13 (grammar).
- **A trait, not a struct with fields.** `docs/architecture.md` §5 calls it a contract each area implements, and #21 adds methods to it (questions for levels). With a struct, #21 would have to add fields to every area at once.
- **One file per area now.** #11, #12 and #13 then each grow only their own file and never touch each other's. Putting all three in `areas.rs` would make each of them edit a file shared with the other two.
- **`Debug` as a supertrait** keeps `#[derive(Debug)]` on `Session`, which now holds the list.
- **What #21 is expected to extend:** add methods to `Area` (for example `fn questions(&self, levels: ...) -> ...`); it may give `has_questions` a default body built on them, keeping its name and signature. Replace the comment-marked branch in `Session::apply` (D4) where an area with questions is chosen, adding its own `Screen` variants and actions. Add its test-only areas through `Session::with_areas` and `areas::fake` (D3).
- **What #21 must not need to change:** `areas::all()` and the `ALL` list shape, `Session::with_areas`/`areas()`/`highlighted()`/`highlighted_area()`, the `Screen::AreaSelection` and `Screen::NoQuestions` arms in `keys` and `ui`, the `Previous`/`Next`/`Choose`/`Back` actions, and any test added here.
- **What #11, #12, #13 change:** only their own file's `has_questions` body (and whatever #21's contract adds), plus their content.

### D3. Test-only areas: `Session::with_areas` and `areas::fake`

```rust
// src/session.rs
pub fn new(width: u16, height: u16) -> Self { Self::with_areas(width, height, areas::all()) }
/// Like `new`, with a given list of areas instead of the app's own.
pub fn with_areas(width: u16, height: u16, areas: &'static [&'static dyn Area]) -> Self
```

```rust
// src/areas.rs, at the end
#[cfg(test)]
pub mod fake {
    use super::Area;

    /// An area for tests, with a fixed name and a fixed answer to `has_questions`.
    #[derive(Debug)]
    pub struct FakeArea {
        pub name: &'static str,
        pub has_questions: bool,
    }

    impl Area for FakeArea {
        fn name(&self) -> &'static str { self.name }
        fn has_questions(&self) -> bool { self.has_questions }
    }
}
```

Tests build lists as constants, e.g. `const EMPTY_PAIR: &[&dyn Area] = &[&FakeArea { name: "Vocabulary", has_questions: false }, &FakeArea { name: "漢字", has_questions: false }];`.

- `with_areas` is not `#[cfg(test)]`: `new` calls it, so it is used in non-test code, and #21 can reuse it.
- Without fakes, "another area adds a line", "an area with questions does not show the message" and display-width alignment of Japanese names could not be tested at all, and every empty-area test would pin the real areas as empty (D11).

### D4. Session

```rust
pub enum Screen { Welcome, AreaSelection, NoQuestions }

pub enum Action {
    Quit, Typed(char), Resized { width: u16, height: u16 }, Tick, Start, // unchanged
    /// Move the highlight to the previous area.
    Previous,
    /// Move the highlight to the next area.
    Next,
    /// Choose the highlighted area.
    Choose,
    /// Leave the "no questions" screen for the list.
    Back,
}
```

New fields on `Session`: `areas: &'static [&'static dyn Area]` and `highlighted: usize` (starts at `0`).

New and changed arms of `Session::apply`:

```rust
Action::Start => {
    if self.screen == Screen::Welcome {
        self.screen = Screen::AreaSelection;
        self.highlighted = 0;
    }
}
Action::Previous => {
    if self.screen == Screen::AreaSelection {
        self.highlighted = self.highlighted.saturating_sub(1);
    }
}
Action::Next => {
    if self.screen == Screen::AreaSelection && self.highlighted + 1 < self.areas.len() {
        self.highlighted += 1;
    }
}
Action::Choose => {
    if self.screen == Screen::AreaSelection
        && let Some(area) = self.highlighted_area()
        && !area.has_questions()
    {
        self.screen = Screen::NoQuestions;
    }
    // An area with questions starts practice here (issue #21).
}
Action::Back => {
    if self.screen == Screen::NoQuestions {
        self.screen = Screen::AreaSelection;
    }
}
```

New getters:

```rust
/// The areas in the list, in order.
pub fn areas(&self) -> &'static [&'static dyn Area]
/// The index of the highlighted area in `areas()`.
pub fn highlighted(&self) -> usize
/// The highlighted area, or `None` when there are no areas.
pub fn highlighted_area(&self) -> Option<&'static dyn Area>   // self.areas.get(self.highlighted).copied()
```

- Every new action checks the screen, so an action that arrives on the wrong screen changes nothing, as `Start` already does.
- **Ends stop, no wrap** (Open Questions 1): `saturating_sub` and the length check. With three short lines, wrapping saves little and a jump from the bottom to the top is easy to miss.
- **Back keeps the highlight** on the area just chosen, so the user can move on from where they were. "The first area is highlighted when the screen opens" is honoured on arrival from Welcome (`Start` resets it). Open Questions 4.
- **An empty list** (only possible through `with_areas`): arrows and Enter do nothing, the screen shows the heading and the hint.
- **An area with questions:** `Choose` changes nothing for now. No test pins "stays on the list", only "does not show the message and does not quit", which #21's level picker also satisfies.

### D5. Key table

Rules 1 to 3 of `keys::action_for` (resize, ignore releases, `Ctrl+C` quits everywhere) and the `Screen::Welcome` arm do not change. The arms become:

```rust
Screen::AreaSelection => match key.code {
    KeyCode::Up if key.modifiers.is_empty() => Some(Action::Previous),
    KeyCode::Down if key.modifiers.is_empty() => Some(Action::Next),
    KeyCode::Enter if key.modifiers.is_empty() && key.kind == KeyEventKind::Press => {
        Some(Action::Choose)
    }
    KeyCode::Esc if key.modifiers.is_empty() && key.kind == KeyEventKind::Press => {
        Some(Action::Quit)
    }
    _ => None,
},
Screen::NoQuestions => match key.code {
    KeyCode::Enter | KeyCode::Esc
        if key.modifiers.is_empty() && key.kind == KeyEventKind::Press =>
    {
        Some(Action::Back)
    }
    _ => None,
},
```

(The final code is built over three tasks; see `tasks.md`.)

- **Held keys:** arrows accept `KeyEventKind::Repeat`, so holding Down keeps moving. Enter and Esc act only on `KeyEventKind::Press`, so a held key cannot pass through two screens (Welcome → list → "no questions" → list, or "no questions" → list → quit). Terminals that report repeats as presses (most Unix terminals without the kitty keyboard protocol) are not protected by this; see Risks.
- **Esc quits** with `Action::Quit` until the Thank you screen exists. No new action is needed now; the Thank you change will swap it.
- **`q` does nothing on the list** (Open Questions 2). Esc is the documented way to leave, and the practice screens #21 adds after this one accept typed answers, where `q` cannot quit; teaching "Esc leaves" from the start keeps the app consistent. It also keeps the existing test `enter_then_ctrl_c_through_the_key_table` valid (it asserts `q` does not quit on area selection).
- **No `j`/`k`, no number keys** (Open Questions 2). The issue names arrows only; extra bindings are easy to add later and would each need their own tests.
- **Esc on "no questions"** goes back, like Enter. `docs/architecture.md` §2 *Keys* has Esc go back to area selection from every screen after it.
- **Modifiers:** Enter, Esc and the arrows act only without modifiers, like Enter on Welcome.

### D6. Drawing

Area selection, centred with the existing `draw_centred`:

```text
What do you want to practise?
(empty)
> Vocabulary
  Kanji
  Grammar
(empty)
Up/Down to move, Enter to choose, Esc to leave
```

- `fn draw_area_selection(frame: &mut Frame, session: &Session)` (signature gains `session`). Each area line is three spans: the marker (`text::AREA_MARKER` = `"> "` on the highlighted line, `text::AREA_NO_MARKER` = `"  "` on the others), the name, and trailing spaces up to the display width of the widest name (`Span::raw(name).width()`, which is what `ratatui` uses to lay out). All area lines then have the same width, so `Alignment::Center` starts them all in the same column: in 80 columns the block is 12 wide, the marker is at column 34 and every name starts at 36.
- **Marker, not reversed text.** It is plain ASCII, survives every terminal, copy-paste and screen readers, and is checked by reading the buffer's symbols. Reversed text was considered: it is a modifier, not a colour, so the existing `area_selection_uses_no_colour` test (fg and bg are `Color::Reset`) would still pass, but it adds nothing the marker does not already show.
- No colour, no bold on this screen.

"No questions" screen, `fn draw_no_questions(frame: &mut Frame, session: &Session)`:

```text
Kanji                              (the highlighted area's name; empty if none)
(empty)
No questions in this area yet
(empty)
Press Enter to go back
```

- `ui::draw` gets `Screen::NoQuestions => draw_no_questions(frame, session)` and `Screen::AreaSelection => draw_area_selection(frame, session)`.
- Too-small terminals clip as on the other screens; nothing does its own arithmetic on the frame size, and the padding subtraction cannot underflow because it is taken from the maximum.

### D7. Empty areas look like the others in the list

Not marked before they are chosen (Open Questions 3). Right now all three are empty, so a mark would sit on every line and say nothing; the issue already defines what happens when an empty area is chosen. If the owner wants a mark later, it is one more span and one constant.

### D8. Wording in `src/text.rs`

Added (all plain ASCII, so no ambiguous widths):

```rust
/// Area selection: the heading above the list.
pub const AREA_SELECTION_TITLE: &str = "What do you want to practise?";
/// Area selection: the keys.
pub const AREA_SELECTION_HINT: &str = "Up/Down to move, Enter to choose, Esc to leave";
/// Area selection: in front of the highlighted area.
pub const AREA_MARKER: &str = "> ";
/// Area selection: in front of every other area, as wide as `AREA_MARKER`.
pub const AREA_NO_MARKER: &str = "  ";
/// Area names, as shown in the list.
pub const AREA_VOCABULARY: &str = "Vocabulary";
pub const AREA_KANJI: &str = "Kanji";
pub const AREA_GRAMMAR: &str = "Grammar";
/// Shown when the chosen area has no questions.
pub const NO_QUESTIONS: &str = "No questions in this area yet";
/// How to leave the "no questions" screen.
pub const NO_QUESTIONS_HINT: &str = "Press Enter to go back";
```

Removed: `AREA_SELECTION_SOON` and `AREA_SELECTION_QUIT_HINT` (unused once the placeholder is gone, so clippy rejects them).

- The hint says `Up/Down`, not arrow symbols: `↑`/`↓` have ambiguous East Asian width and would break centring in CJK terminals (module doc of `text.rs`).
- **Area names live in `text.rs`**, as the issue and the module doc require. The cost: adding an area also adds one constant here, besides its file and its line in `areas::ALL`. Open Questions 5.

### D9. Task order and commits

Each task is one commit that builds and passes fmt, clippy and tests (`AGENTS.md` §5):

- **Task 1.1: the owner-approved edit of `keys::tests::other_keys_do_nothing_on_area_selection`** (D10). It comes first because the edited test passes both before and after the feature, while the feature's Enter arm cannot pass with the old test in place.
- **Task 2.1: the list and choosing an area:** the `areas` module, the session state, `Choose`/`Back`, `Screen::NoQuestions`, Enter on both screens, drawing both screens, the wording, removal of the placeholder constants, the owner-approved edits of the three other existing tests (D10), the `CHANGELOG.md` entry and the `docs/architecture.md` notes. Kept together because `Screen::NoQuestions` needs arms in `keys` and `ui` to compile, the placeholder constants must disappear in the commit that stops drawing them, `has_questions` and the fake areas need a non-test caller (checked, see Context), and the three edited tests assert the new screen and wording, so they fail before it exists and the old versions fail after it.
- **Task 3.1: Up and Down** (extends the `CHANGELOG.md` entry).
- **Task 4.1: Esc**, quit on the list and back on "no questions" (extends the `CHANGELOG.md` entry and the architecture status note).
- **Task 5.1:** scripted check of the release binary in a pseudo-terminal (no file changes).

### D10. Existing tests

All 51 tests on `main` were checked against this design.

| Test | Effect |
|---|---|
| `app::tests::*` (4) | Unaffected: `src/app.rs` does not change. |
| `keys::tests::a_held_key_repeats`, `characters_are_typed`, `ctrl_c_quits`, `q_quits_on_welcome`, `releases_are_ignored`, `resize_becomes_resized`, `other_modifiers_do_nothing`, `other_keys_and_events_do_nothing` | Unaffected: Welcome arm unchanged (Esc and Up still do nothing on Welcome). |
| `keys::tests::enter_starts_on_welcome`, `enter_release_does_nothing`, `modified_enter_does_nothing_on_welcome` | Unaffected: Welcome arm unchanged. |
| `keys::tests::ctrl_c_quits_on_area_selection`, `resize_is_followed_on_area_selection` | Unaffected: rules 1 and 3 still come before the per-screen arms. |
| `keys::tests::enter_then_ctrl_c_through_the_key_table` | Unaffected: Enter on Welcome → `AreaSelection`; `q` there still does nothing (D5); `Ctrl+C` quits. |
| **`keys::tests::other_keys_do_nothing_on_area_selection`** | **Conflict, edit approved (task 1.1).** Asserts Enter, Esc and Up give `None` on area selection; they now give `Choose`, `Quit` and `Previous`. |
| `session::tests::*` (9) | Unaffected. `Session::new(w, h)` keeps its signature and starts on Welcome; `Start` still moves Welcome → `AreaSelection` once; `start_is_one_way` applies only `Start`, `Typed`, `Tick`, `Resized`, none of which leave area selection; `only_quit_sets_should_quit` applies none of the new actions. |
| `text::tests::title_is_the_name_and_version`, `status_shows_size_and_tenths_of_a_second`, `last_key_shows_the_character_or_a_placeholder`, `fatal_error_names_the_app`, `start_hint_is_plain_ascii_text`, `fixed_messages_are_not_empty` | Unaffected: the constants and functions they use are unchanged. |
| **`text::tests::area_selection_messages_are_plain_ascii_text`** | **Conflict, edit approved (task 2.1).** Uses `AREA_SELECTION_SOON` and `AREA_SELECTION_QUIT_HINT`, which this change removes (unused constants fail clippy), so the test no longer compiles. |
| `ui::tests::` all Welcome tests (`greeting_*`, `a_fresh_session_shows_every_line`, `typed_character_and_elapsed_time_are_shown`, `fresh_welcome_shows_the_start_hint`, `welcome_*`, `start_hint_is_centred_by_display_width`, `any_size_draws_without_panicking`) | Unaffected: `draw_welcome` and `draw_centred` do not change. |
| `ui::tests::area_selection_draws_at_any_size_without_panicking` | Unaffected: now draws the list at those sizes. |
| `ui::tests::area_selection_uses_no_colour` | Unaffected: the marker is text and nothing sets a colour; it now also covers the highlighted line. |
| **`ui::tests::area_selection_placeholder_is_shown`** | **Conflict, edit approved (task 2.1).** Asserts the screen contains `Area selection is coming soon.` and `Press Ctrl+C to quit`, which are no longer drawn. |
| **`ui::tests::area_selection_follows_a_resize`** | **Conflict, edit approved (task 2.1).** Looks for the row containing `Press Ctrl+C to quit` and panics when it is missing. |

Keeping the placeholder text on screen only to keep these tests passing was considered and rejected: the issue replaces the placeholder, a `Press Ctrl+C to quit` line next to a hint that says `Esc to leave` would mislead, and the tests' purpose (placeholder content) no longer exists.

**Decision: the owner approved editing these four tests for issue #20 and asked that none be removed.** Each keeps its name, its place in its file and its intent; only what the new screen forces changes. The full new bodies are in `tasks.md`.

- a. `keys::tests::other_keys_do_nothing_on_area_selection` (`src/keys.rs`), task 1.1: the array becomes `[KeyCode::Char('q'), KeyCode::Char('a'), KeyCode::Char('あ')]` (`KeyCode::Enter`, `KeyCode::Esc` and `KeyCode::Up` removed). The loop body and the `Event::FocusGained` assertion stay. It still pins that `q`, letters and kana do nothing on area selection, and passes before and after the feature, so it is its own commit before it.
- b. `ui::tests::area_selection_placeholder_is_shown` (`src/ui.rs`), task 2.1: the two positive assertions change from `"Area selection is coming soon."` and `"Press Ctrl+C to quit"` to `"What do you want to practise?"` and `"Up/Down to move, Enter to choose, Esc to leave"` (the heading and the hint that take their place). The negative loop over `"日"`, `"Press Enter to start"` and `"Last key:"` stays as it is.
- c. `ui::tests::area_selection_follows_a_resize` (`src/ui.rs`), task 2.1: the needle `"Press Ctrl+C to quit"` becomes `text::AREA_SELECTION_HINT`, the expected symbol `"P"` becomes `"U"`, and both `40`s become `27`. The resize to 100x30 stays.
- d. `text::tests::area_selection_messages_are_plain_ascii_text` (`src/text.rs`), task 2.1: `AREA_SELECTION_SOON` / `"Area selection is coming soon."` becomes `AREA_SELECTION_TITLE` / `"What do you want to practise?"`, and `AREA_SELECTION_QUIT_HINT` / `"Press Ctrl+C to quit"` becomes `AREA_SELECTION_HINT` / `"Up/Down to move, Enter to choose, Esc to leave"`, in both the equality and the `is_ascii()` checks. The two placeholder constants are still removed in the same commit (unused constants fail clippy).

Edits b, c and d assert the new screen and wording, so they cannot pass before the feature and are part of task 2.1's commit, each labelled in the task as an owner-approved edit. No other existing test changes (re-checked against `src/keys.rs`, `src/ui.rs`, `src/text.rs` and `src/session.rs` after this decision).

**Columns, checked by rendering.** `ratatui` centres a line at `area_width / 2 - line_width / 2`, each division rounded down (this is also why the existing greeting test finds `日` at column 28 in both 80 and 81 columns). The hint is 46 columns wide: column `40 - 23 = 17` at 80, `50 - 23 = 27` at 100. The area block is 12 columns (marker 2 + `Vocabulary` 10, shorter names padded): marker at `40 - 6 = 34`, names at 36 at 80; marker at 44, names at 46 at 100. The heading (29 columns) starts at 26 at 80, and so does `No questions in this area yet` (29 columns). The new bodies of b, c and d, a prototype of D6's drawing and D8's constants were run in a scratch copy of the crate: all existing tests, including the three edited ones, pass, and `cargo fmt --check` and `cargo clippy --all-targets -- -D warnings` are clean.

**Planned new tests trimmed so they do not repeat the edited ones:**
- `area_selection_lists_the_areas_in_order` keeps the order of the heading and the three areas and the absence of `Area selection is coming soon.`; it no longer checks that the heading is present or that `日`, `Press Enter to start` and `Last key:` are absent (edit b does).
- `area_list_stays_centred_after_a_resize` keeps the area block after the resize (marker at 44, names at 46); it no longer checks the hint at 27 (edit c does).
- `area_selection_wording_is_plain_ascii_text` becomes `area_and_no_questions_wording_is_plain_ascii_text` and checks only the area names and the two "no questions" constants; the heading and hint are covered by edit d, the markers by `area_markers_are_the_same_width`, which now checks their exact strings.
- No planned new test is dropped entirely: each still checks something the edited tests do not.

### D11. New tests, and what they deliberately do not pin

All new tests are unit tests in the file they cover, without a terminal; they are named per task in `tasks.md`. Rules followed so later issues do not inherit conflicts:

- **No test asserts that a real area has no questions** (`Vocabulary.has_questions() == false`, "Enter on Vocabulary shows the message"). #11, #12 and #13 will make that false. Empty-area behaviour is tested with `FakeArea`.
- **No test asserts the list has exactly three areas.** Real-area tests check that the names *start with* Vocabulary, Kanji, Grammar, in that order; ends-of-list tests use fakes.
- **No test asserts that Enter on an area with questions stays on the list** (#21 changes that); only that it does not show the message and does not quit.
- Rows are found by content (`row_index`), never by a fixed row number, as in the existing tests.

## Risks / Trade-offs

- [Archiving in the wrong order silently produces a contradictory `area-selection` spec] → D1 states the order; the owner archives `add-welcome-screen` before this change. Verified on a scratch copy.
- [Most Unix terminals report a held key as repeated presses, so a held Enter can still pass Welcome → list → "no questions" → list] → Harmless (no screen is skipped irreversibly; Esc is not reached). The `Press` guard helps where repeats are reported (Windows, kitty protocol).
- [Esc is detected by crossterm only after a short wait, to tell it from escape sequences] → Standard terminal behaviour; the scripted check (task 5.1) confirms Esc quits.
- [Area names in `text.rs` mean adding an area touches one more file] → D8; Open Questions 5.
- [Task 2.1 edits three existing tests in the same commit as the feature, which makes the edits harder to review than separate commits] → Each edit is labelled in the task and the commit body, its exact body is written out in `tasks.md`, and the verify step checks that no test is removed or renamed.

## Open Questions

For the owner:

1. **Arrows at the ends (D4):** recommended **stop** at the first and last area. Alternative: wrap around.
2. **Other keys on the list (D5):** recommended **`q` does nothing**, and **no `j`/`k` and no number keys**. If you want `q` to leave as on Welcome, two existing tests are then affected: `keys::tests::other_keys_do_nothing_on_area_selection` (also remove `KeyCode::Char('q')` from its array) and `keys::tests::enter_then_ctrl_c_through_the_key_table` (asserts `q` does not quit on area selection), and the plan changes.
3. **Empty areas in the list (D7):** recommended **not marked**; they look like the others until chosen.
4. **Highlight after coming back from "no questions" (D4):** recommended **stays on the area just chosen**. Alternative: back to the first area each time.
5. **Where area names live (D8):** recommended **`src/text.rs`** (issue #20 and the `text.rs` rule). Alternative: each area's own file, so that adding an area changes no shared wording file; this would be an exception to "all wording in `text.rs`".
6. **Archive order (D1):** please archive `add-welcome-screen` (and preferably `add-main-application-loop` first) before archiving this change. Both are complete. Archiving this one first does not fail but leaves the old placeholder requirements in the main spec.
7. **Test names that read oddly after the approved edits (D10), for a possible later rename:** `ui::tests::area_selection_placeholder_is_shown` now checks the real screen, not a placeholder (a later name could be e.g. `area_selection_heading_and_hint_are_shown`). The other three names still fit. This change keeps every name, because a rename was not approved; renaming would be a separate owner decision.

(The former question 6, approval of the four conflicting tests, is decided: see D10.)
