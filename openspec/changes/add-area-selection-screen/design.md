# Design

## Context

See `proposal.md` for the motivation and `specs/area-selection/spec.md` for the required behaviour.

- The code has the shape from #10 and #19: `keys::action_for(screen, event)` turns an event into an `Action` per `Screen`, `Session::apply` is the only place state changes, and `ui::draw` matches on `Session::screen()`. `Screen` is matched exhaustively in `keys` and `ui`, so a new variant needs an arm in both.
- #19 left `Screen::AreaSelection`, `Action::Start` and its one-way transition, an empty key arm (`Screen::AreaSelection => None`), `ui::draw_area_selection` drawing a placeholder through the private helper `ui::draw_centred`, and the constants `text::AREA_SELECTION_SOON` and `text::AREA_SELECTION_QUIT_HINT`. This change keeps the first three and replaces the rest (`add-welcome-screen` design D2, "What #20 inherits").
- The crate is a binary: `cargo clippy --all-targets -- -D warnings` fails on any item, field or constant that non-test code never uses. This decides the task order (D9) and why the placeholder constants must go.
- `openspec/specs/` is empty. `area-selection` exists only as an ADDED delta of the unarchived change `add-welcome-screen` (complete, 4/4 tasks).
- The baseline is 51 passing tests (`cargo test`, run while writing this design). Issue #20's "Existing tests" section says nothing conflicts; that was written before #19 merged the placeholder tests, and is no longer true (D10).
- **Revision after the owner's pull request review:** the owner does not want this change to add a Vocabulary, Kanji or Grammar area. The screen is a generic place to select any area; it lists whatever the app registers, and this change registers none. Groups 1 to 5 of `tasks.md` were implemented with three built-in areas; group 6 removes them and adds the empty-list state. D2, D4, D6, D8, D9 and D11 were revised, D12 and D13 are new.
- Three facts were checked in a scratch crate (same toolchain, pinned `ratatui` 0.30) before relying on them: a centred `Paragraph` line keeps its trailing spaces (so padded lines of equal width start in the same column); `const ALL: &[&dyn Area] = &[&Vocabulary, ...]` (the list before the revision) compiles and can be returned as `&'static [&'static dyn Area]`; and with this module layout, clippy `-D warnings` rejects `Area::has_questions` and `FakeArea::has_questions` while no non-test code calls `has_questions`, and accepts them once `Session::apply` does.

## Goals / Non-Goals

**Goals:**
- One place lists the app's areas, and the screen reads its lines from there (D2). This is the seam #21 grows. The screen does not know or care which areas exist.
- A defined, tested state for an empty list, which is what the app ships with after this change (D12).
- Everything on both screens is testable without a terminal, including the "another area adds a line" rule, through test-only areas (D3).
- No test added here pins a fact that #11, #12, #13 or #21 will legitimately change, so those issues do not inherit test conflicts (D11).

**Non-Goals:**
- Questions, JLPT levels, the practice screens, what Enter does on an area that has questions (all #21).
- Any area and any content (#11, #12, #13). This change registers no area.
- The Thank you screen. Esc quits directly.
- Marking empty areas in the list (D7, Open Questions).

## Decisions

### D1. Spec delta: REMOVED + ADDED on `area-selection`, archived after `add-welcome-screen`

The delta removes the three placeholder requirements and adds twelve new ones (eleven before the revision, plus *Empty list of areas*). It does not use MODIFIED.

- **Why not MODIFIED:** every placeholder requirement is replaced, not adjusted. Their names ("...placeholder...") would be wrong afterwards, and a MODIFIED block must restate the whole requirement anyway. REMOVED with a Reason and a Migration says plainly what happened.
- **Why not a new capability** (e.g. `area-list`) with ADDED only: the placeholder requirements would then stay in `openspec/specs/area-selection/spec.md` for ever, contradicting the new ones ("every other key does nothing" against "Esc quits").
- **Archive order, verified** on a scratch copy of `openspec/` with OpenSpec 1.16.0:
  - `add-main-application-loop` → `add-welcome-screen` → `add-area-selection-screen`: all three archive; `area-selection` ends with exactly the twelve new requirements (`+ 12, - 3`), and `openspec validate --specs --strict` passes (re-checked after the revision on a scratch copy of `openspec/`; the archive prints the non-blocking warning "Consider splitting changes with more than 10 deltas", as it already did with eleven).
  - Archiving this change **before** `add-welcome-screen` does **not** fail. It prints only a warning ("3 REMOVED requirement(s) ignored for new spec") and creates `area-selection` from the ADDED part. Archiving `add-welcome-screen` afterwards then silently adds the three placeholder requirements back, leaving 14 contradictory requirements. So the order is a rule the owner must follow, not one the tool enforces.
  - `add-main-application-loop` is independent of this change (it owns `app-shell`); archiving it first is recommended, not required.
- `openspec validate add-area-selection-screen --strict` passes now, before any archive.

### D2. The seam for #21: `src/areas.rs`

```rust
// src/areas.rs (after the revision: no `mod` lines, no `use` lines)

/// One thing the user can practise. #21 grows this into the full area
/// contract (questions for a set of JLPT levels).
pub trait Area: std::fmt::Debug {
    /// The name shown in the list of areas.
    fn name(&self) -> &'static str;
    /// Whether the area has any question at all. An area without questions
    /// can still be chosen; it says so.
    fn has_questions(&self) -> bool;
}

/// Every area, in the order the list shows them. There are none yet. Adding
/// an area adds its own file under `src/areas/` and one entry here.
const ALL: &[&dyn Area] = &[];

pub fn all() -> &'static [&'static dyn Area] { ALL }
```

- **No built-in area (owner decision, revision).** Groups 1 to 5 added `src/areas/vocabulary.rs`, `src/areas/kanji.rs` and `src/areas/grammar.rs` (one unit struct each, `has_questions() == false`) and the name constants `text::AREA_VOCABULARY`, `AREA_KANJI`, `AREA_GRAMMAR`. Task 6.2 deletes all of them: the screen must not depend on which areas exist, and whether Vocabulary, Kanji or Grammar exist at all is decided by their own issues. `ALL` is the empty slice and `src/areas/` does not exist after this change.
- **A trait, not a struct with fields.** `docs/architecture.md` §5 calls it a contract each area implements, and #21 adds methods to it (questions for levels). With a struct, #21 would have to add fields to every area at once.
- **One file per area, when one is added.** Each area lives in its own file under `src/areas/` and never touches another area's file. The only shared edit is one entry in `ALL` (plus one `mod` and one `use` line in `src/areas.rs`).
- **`Debug` as a supertrait** keeps `#[derive(Debug)]` on `Session`, which now holds the list.
- **What #21 is expected to extend:** add methods to `Area` (for example `fn questions(&self, levels: ...) -> ...`); it may give `has_questions` a default body built on them, keeping its name and signature. Replace the comment-marked branch in `Session::apply` (D4) where an area with questions is chosen, adding its own `Screen` variants and actions. Add its test-only areas through `Session::with_areas` and `areas::fake` (D3).
- **What #21 must not need to change:** `areas::all()` and the `ALL` list shape, `Session::with_areas`/`areas()`/`highlighted()`/`highlighted_area()`, the `Screen::AreaSelection` and `Screen::NoQuestions` arms in `keys` and `ui`, the `Previous`/`Next`/`Choose`/`Back` actions, and any test added here.
- **What an area change (#11, #12, #13 or any later one) adds:** its own new file `src/areas/<area>.rs` with a unit struct implementing `Area` (and whatever #21's contract adds), its content, one `mod <area>;` and one `use <area>::<Struct>;` line in `src/areas.rs`, one entry in `ALL`, and its display name (see D8 and Open Questions 5 for where the name lives). It does not touch `session`, `keys`, `ui` or any other area. Its tests must not assert that `areas::all()` is empty or has a fixed length (D11).

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
- **An empty list** (what `Session::new` gives after the revision, and `with_areas(.., &[])`): see D12. `highlighted()` stays `0` and `highlighted_area()` is `None`; arrows and Enter change nothing. No code indexes `areas` directly: `highlighted_area` uses `get`, `Next` checks `highlighted + 1 < len` (false for `len == 0`), `Previous` uses `saturating_sub`, and drawing iterates.
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

Area selection, centred with the existing `draw_centred` (shown with three registered areas named `First`, `Second`, `Third`, as in the tests):

```text
What do you want to practise?
(empty)
> First
  Second
  Third
(empty)
Up/Down to move, Enter to choose, Esc to leave
```

With no registered area (D12):

```text
What do you want to practise?
(empty)
Nothing to practise yet
(empty)
Up/Down to move, Enter to choose, Esc to leave
```

- `fn draw_area_selection(frame: &mut Frame, session: &Session)` (signature gains `session`). Each area line is three spans: the marker (`text::AREA_MARKER` = `"> "` on the highlighted line, `text::AREA_NO_MARKER` = `"  "` on the others), the name, and trailing spaces up to the display width of the widest name (`Span::raw(name).width()`, which is what `ratatui` uses to lay out). All area lines then have the same width, so `Alignment::Center` starts them all in the same column: with `First`/`Second`/`Third` the block is 8 wide (marker 2 + `Second` 6); in 80 columns the marker is at column 36 and every name starts at 38, in 100 columns at 46 and 48. With `Vocabulary` (10) and `漢字` (4 columns) the block is 12 wide and both names start at 36 in 80 columns.
- When `session.areas()` is empty, one line `text::AREA_SELECTION_EMPTY` is pushed after the heading's blank line instead of area lines (the area loop then pushes nothing). Exact code in `tasks.md` 6.1.
- **Marker, not reversed text.** It is plain ASCII, survives every terminal, copy-paste and screen readers, and is checked by reading the buffer's symbols. Reversed text was considered: it is a modifier, not a colour, so the existing `area_selection_uses_no_colour` test (fg and bg are `Color::Reset`) would still pass, but it adds nothing the marker does not already show.
- No colour, no bold on this screen.

"No questions" screen, `fn draw_no_questions(frame: &mut Frame, session: &Session)`:

```text
Only                               (the highlighted area's name; empty if none)
(empty)
No questions in this area yet
(empty)
Press Enter to go back
```

- `ui::draw` gets `Screen::NoQuestions => draw_no_questions(frame, session)` and `Screen::AreaSelection => draw_area_selection(frame, session)`.
- Too-small terminals clip as on the other screens; nothing does its own arithmetic on the frame size, and the padding subtraction cannot underflow because it is taken from the maximum.

### D7. Empty areas look like the others in the list

Not marked before they are chosen (Open Questions 3). The issue already defines what happens when an empty area is chosen, and while areas are being added most of them will be empty, so a mark would sit on most lines and say little. If the owner wants a mark later, it is one more span and one constant.

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
/// Area selection: shown in place of the list when the app has no areas.
pub const AREA_SELECTION_EMPTY: &str = "Nothing to practise yet";
/// Shown when the chosen area has no questions.
pub const NO_QUESTIONS: &str = "No questions in this area yet";
/// How to leave the "no questions" screen.
pub const NO_QUESTIONS_HINT: &str = "Press Enter to go back";
```

Removed: `AREA_SELECTION_SOON` and `AREA_SELECTION_QUIT_HINT` (unused once the placeholder is gone, so clippy rejects them). Groups 1 to 5 also added `AREA_VOCABULARY`, `AREA_KANJI` and `AREA_GRAMMAR`; task 6.2 removes them with the area files (they would be unused, and clippy rejects them). `AREA_SELECTION_EMPTY` is added by task 6.1, between `AREA_NO_MARKER` and `NO_QUESTIONS`.

- The hint says `Up/Down`, not arrow symbols: `↑`/`↓` have ambiguous East Asian width and would break centring in CJK terminals (module doc of `text.rs`).
- **Area names:** this change has none. When #11, #12 or #13 add an area, its display name follows the `text.rs` rule (all wording in `src/text.rs`) unless the owner decides otherwise; see the note in Open Questions 5.

### D9. Task order and commits

Each task is one commit that builds and passes fmt, clippy and tests (`AGENTS.md` §5):

- **Task 1.1: the owner-approved edit of `keys::tests::other_keys_do_nothing_on_area_selection`** (D10). It comes first because the edited test passes both before and after the feature, while the feature's Enter arm cannot pass with the old test in place.
- **Task 2.1: the list and choosing an area:** the `areas` module, the session state, `Choose`/`Back`, `Screen::NoQuestions`, Enter on both screens, drawing both screens, the wording, removal of the placeholder constants, the owner-approved edits of the three other existing tests (D10), the `CHANGELOG.md` entry and the `docs/architecture.md` notes. Kept together because `Screen::NoQuestions` needs arms in `keys` and `ui` to compile, the placeholder constants must disappear in the commit that stops drawing them, `has_questions` and the fake areas need a non-test caller (checked, see Context), and the three edited tests assert the new screen and wording, so they fail before it exists and the old versions fail after it.
- **Task 3.1: Up and Down** (extends the `CHANGELOG.md` entry).
- **Task 4.1: Esc**, quit on the list and back on "no questions" (extends the `CHANGELOG.md` entry and the architecture status note).
- **Task 5.1:** scripted check of the release binary in a pseudo-terminal (no file changes).
- **Group 6, the revision (D12, D13):**
  - **Task 6.1: the empty-list line.** `AREA_SELECTION_EMPTY`, its drawing, and its tests through `Session::with_areas(.., &[])`. It comes first: the three built-in areas still exist, so the real list is untouched, and the new constant is already used by `ui` (no dead code). No docs change yet.
  - **Task 6.2: remove the built-in areas.** The three files, their `mod`/`use` lines, the three name constants, the branch-only tests that pinned them (deleted or rewritten with `FakeArea`), the `CHANGELOG.md` entry and `docs/architecture.md`. One commit, because deleting the files makes the name constants unused (clippy) and makes the tests that render the real list fail, so neither can be left for a later commit.
  - **Task 6.3:** the pseudo-terminal check again, with the new expected observations (no file changes besides the checkbox).

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

**Columns, checked by rendering.** `ratatui` centres a line at `area_width / 2 - line_width / 2`, each division rounded down (this is also why the existing greeting test finds `日` at column 28 in both 80 and 81 columns). The hint is 46 columns wide: column `40 - 23 = 17` at 80, `50 - 23 = 27` at 100. The area block was 12 columns with the built-in areas of groups 1 to 5 (marker 2 + `Vocabulary` 10, shorter names padded): marker at `40 - 6 = 34`, names at 36 at 80; marker at 44, names at 46 at 100. After the revision the tests use their own lists; their columns are in D6 and the empty list's in D12. The heading (29 columns) starts at 26 at 80, and so does `No questions in this area yet` (29 columns). The new bodies of b, c and d, a prototype of D6's drawing and D8's constants were run in a scratch copy of the crate: all existing tests, including the three edited ones, pass, and `cargo fmt --check` and `cargo clippy --all-targets -- -D warnings` are clean.

**Re-checked for the revision (D12).** After task 6.2 these four tests run against the real, now empty, list (`Session::new`). The empty-list screen still draws the heading and the hint, the hint is centred horizontally on its own (it does not depend on the list), so `area_selection_placeholder_is_shown` and `area_selection_follows_a_resize` (hint `U` at 27 at 100 wide) still pass; `area_selection_messages_are_plain_ascii_text` uses only the heading and hint constants, which do not change; `other_keys_do_nothing_on_area_selection` uses only the key table. All four keep exactly their current bodies; group 6 does not touch them. Verified by running them on a scratch copy with groups 6.1 and 6.2 applied (D13).

**Planned new tests trimmed so they do not repeat the edited ones:**
- `area_selection_lists_the_areas_in_order` keeps the order of the heading and the three areas and the absence of `Area selection is coming soon.`; it no longer checks that the heading is present or that `日`, `Press Enter to start` and `Last key:` are absent (edit b does).
- `area_list_stays_centred_after_a_resize` keeps the area block after the resize (marker at 44, names at 46); it no longer checks the hint at 27 (edit c does).
- `area_selection_wording_is_plain_ascii_text` becomes `area_and_no_questions_wording_is_plain_ascii_text` and checks only the area names and the two "no questions" constants; the heading and hint are covered by edit d, the markers by `area_markers_are_the_same_width`, which now checks their exact strings.
- No planned new test is dropped entirely: each still checks something the edited tests do not.

### D11. New tests, and what they deliberately do not pin

All new tests are unit tests in the file they cover, without a terminal; they are named per task in `tasks.md`. Rules followed so later issues do not inherit conflicts:

- **No test asserts what `areas::all()` contains** (revision): not that it is empty, not its length, not its names. #11, #12 and #13 will each add an entry. Tests that need a known list, including the empty one, build it with `Session::with_areas`. `areas::tests::area_names_are_unique_and_not_empty` checks every registered name and holds for any list (vacuously for the empty one), and the owner-approved `ui` tests through `Session::new` check only the heading and the hint, which every list shows.
- **No test asserts that a real area has no questions** (`Vocabulary.has_questions() == false`, "Enter on Vocabulary shows the message"). #11, #12 and #13 will make that false. Empty-area behaviour is tested with `FakeArea`.
- **No test asserts the list has exactly three areas.** Ends-of-list tests use fakes. (Before the revision, one test checked that the real names start with Vocabulary, Kanji, Grammar; task 6.2 deletes it.)
- **No test asserts that Enter on an area with questions stays on the list** (#21 changes that); only that it does not show the message and does not quit.
- Rows are found by content (`row_index`), never by a fixed row number, as in the existing tests.

### D12. Revision: no built-in areas, and the empty list

The owner reviewed the pull request: the screen is a place to select areas, any area; it should not matter which ones exist, so that areas can be added later, or not. This change therefore registers none (`ALL = &[]`), and the empty list is the state the app actually ships with until #11, #12 or #13 lands.

**What stays (the generic mechanism):** the `Area` trait, `areas::all()`, `Session::with_areas`, `areas()`, `highlighted()`, `highlighted_area()`, the test-only `FakeArea`, the `> ` highlight, `Previous`/`Next`/`Choose`/`Back`, `Screen::NoQuestions` and its drawing and wording, and the whole key table. None of it changes in group 6.

**The empty list, drawn** (five lines, centred as one block vertically, each line centred horizontally by display width):

| Line | Text | Width | Column at 80 | Column at 100 |
|---|---|---|---|---|
| 1 | `What do you want to practise?` (`AREA_SELECTION_TITLE`) | 29 | 26 | 36 |
| 2 | blank | | | |
| 3 | `Nothing to practise yet` (`AREA_SELECTION_EMPTY`, new) | 23 | 29 | 39 |
| 4 | blank | | | |
| 5 | `Up/Down to move, Enter to choose, Esc to leave` (`AREA_SELECTION_HINT`) | 46 | 17 | 27 |

Columns follow `area_width / 2 - line_width / 2` with integer division (D10): `40 - 11 = 29` and `50 - 11 = 39` for the new line. Verified by rendering on a scratch copy (D13). With areas, line 3 is replaced by the area lines and nothing else moves, so the screen keeps one layout.

- **Wording:** `Nothing to practise yet`. Plain ASCII, short, British spelling like the heading ("practise"), and "yet" says areas are expected later without promising which. Open Questions 8.
- **No marker** is drawn on the empty list: there is nothing to highlight.
- **Same hint** as with areas, so the screen has one layout and the owner-approved hint tests hold for any list. Up/Down/Enter are then named but do nothing; accepted for now, Open Questions 9.
- **Keys** (no change to `keys`): Up → `Previous`, Down → `Next`, Enter → `Choose` are still produced by the key table, and `Session::apply` turns each into no change (D4). Esc → `Quit`; `Ctrl+C` → `Quit`.
- **Getters on an empty list:** `areas()` is the empty slice, `highlighted()` returns `0` (its documented meaning "index into `areas()`" only holds when `highlighted_area()` is `Some`), and `highlighted_area()` returns `None`. `highlighted()` keeps its `usize` signature rather than becoming `Option<usize>`, because #21 relies on it and `highlighted_area()` already expresses "nothing highlighted"; its doc comment gains one sentence (task 6.1).
- **Not skipping the screen when the list is empty** (Open Questions 10): the Welcome → area selection → (Esc) flow stays the same for every build, the user sees why there is nothing to do, and no test or key path depends on how many areas exist.

### D13. Dead code with no implementor of `Area`, checked

The concern: in this binary crate, `cargo clippy --all-targets -- -D warnings` rejects any item non-test code never uses. After task 6.2 only the test-only `FakeArea` implements `Area`.

Checked on scratch copies of this branch (outside the worktree, own `CARGO_TARGET_DIR`), toolchain `cargo 1.100.0`, `clippy 0.1.100`:

1. Only the three area files, their `mod`/`use` lines and the three name constants removed, `ALL = &[]`: `cargo clippy -- -D warnings` (non-test build) prints only `Checking tui-learnjapanese ...` and `Finished`, with **no warning**. `--all-targets` fails only because the test `text::tests::area_and_no_questions_wording_is_plain_ascii_text` still names the removed constants (`error[E0425]: cannot find value AREA_VOCABULARY / AREA_KANJI / AREA_GRAMMAR`); this is why that test is rewritten in the same task.
2. Tasks 6.1 and 6.2 applied exactly as written in `tasks.md`: `cargo fmt --all -- --check` clean, `cargo clippy --all-targets -- -D warnings` clean, `cargo clippy -- -D warnings` clean, `cargo test`: 109 passed after 6.1, 105 passed after 6.2 (4 deleted tests, 0 failed).

Why nothing becomes dead: the trait's methods are called through `&dyn Area` from non-test code (`ui::draw_area_selection` and `draw_no_questions` call `name()`, `Session::apply` calls `has_questions()`), and rustc counts a call through a trait object as a use whether or not a non-test implementor exists. `Screen::NoQuestions` is constructed in `Session::apply` (`Choose`), `Choose`/`Back`/`Previous`/`Next` are constructed in `keys::action_for`, `NO_QUESTIONS`, `NO_QUESTIONS_HINT`, `AREA_MARKER`, `AREA_NO_MARKER` and the new `AREA_SELECTION_EMPTY` are read by `ui`, `areas::all()` by `Session::new`. So no lint suppression is needed. If the implementer's clippy reports anything else, they must stop and report it, not add an `#[allow]` (Open Questions 11).

## Risks / Trade-offs

- [Archiving in the wrong order silently produces a contradictory `area-selection` spec] → D1 states the order; the owner archives `add-welcome-screen` before this change. Verified on a scratch copy.
- [Most Unix terminals report a held key as repeated presses, so a held Enter can still pass Welcome → list → "no questions" → list] → Harmless (no screen is skipped irreversibly; Esc is not reached). The `Press` guard helps where repeats are reported (Windows, kitty protocol).
- [Esc is detected by crossterm only after a short wait, to tell it from escape sequences] → Standard terminal behaviour; the scripted check (task 5.1) confirms Esc quits.
- [Area names in `text.rs` mean adding an area touches one more file] → D8; note in Open Questions 5, decided when the first area is added.
- [The shipped app has nothing to practise until an area is added, and the hint names Up/Down/Enter, which do nothing on the empty list] → Owner decision (D12). The `Nothing to practise yet` line says why; Open Questions 9 offers an empty-list hint if wanted.
- [Nothing in the code base exercises `Area` with a real implementor until #11 to #13] → The full mechanism is tested through `FakeArea` (D3), and D13 shows it is not dead code.
- [Task 2.1 edits three existing tests in the same commit as the feature, which makes the edits harder to review than separate commits] → Each edit is labelled in the task and the commit body, its exact body is written out in `tasks.md`, and the verify step checks that no test is removed or renamed.

## Open Questions

For the owner:

1. **Arrows at the ends (D4):** recommended **stop** at the first and last area. Alternative: wrap around.
2. **Other keys on the list (D5):** recommended **`q` does nothing**, and **no `j`/`k` and no number keys**. If you want `q` to leave as on Welcome, two existing tests are then affected: `keys::tests::other_keys_do_nothing_on_area_selection` (also remove `KeyCode::Char('q')` from its array) and `keys::tests::enter_then_ctrl_c_through_the_key_table` (asserts `q` does not quit on area selection), and the plan changes.
3. **Empty areas in the list (D7):** recommended **not marked**; they look like the others until chosen.
4. **Highlight after coming back from "no questions" (D4):** recommended **stays on the area just chosen**. Alternative: back to the first area each time.
5. **Note for #11, #12, #13 (was: where area names live):** this change has no area name any more, so nothing is decided here. The area changes follow the `text.rs` rule (name constant in `src/text.rs`) unless the owner chooses, in one of them, to keep the name in the area's own file so that adding an area changes no shared wording file.
6. **Archive order (D1):** please archive `add-welcome-screen` (and preferably `add-main-application-loop` first) before archiving this change. Both are complete. Archiving this one first does not fail but leaves the old placeholder requirements in the main spec.
7. **Test names that read oddly after the approved edits (D10), for a possible later rename:** `ui::tests::area_selection_placeholder_is_shown` now checks the real screen, not a placeholder (a later name could be e.g. `area_selection_heading_and_hint_are_shown`). The other three names still fit. This change keeps every name, because a rename was not approved; renaming would be a separate owner decision.

8. **Empty-list wording (D12):** recommended **`Nothing to practise yet`**. Alternatives: `No areas to practise yet`, `There is nothing to practise yet`. Changing it later changes only `text::AREA_SELECTION_EMPTY` and the tests that pin it (`text::tests::empty_area_list_wording_is_plain_ascii_text`, `ui::tests::empty_area_list_is_centred`, whose columns depend on the width).
9. **Hint on the empty list (D12):** recommended **the same hint as with areas**, so the screen has one layout and the owner-approved hint tests hold for any list. Alternative: a shorter hint such as `Esc to leave` on the empty list only; the owner-approved tests `area_selection_placeholder_is_shown` and `area_selection_follows_a_resize` render the real list, so with an empty real list they would then fail, which would require the owner's approval to change them again. Not planned.
10. **Skip area selection when the list is empty?** Recommended **no, show it** (D12). Skipping would make Enter on Welcome do something else depending on the build, and the owner-approved tests that render the real list after Enter would fail.
11. **Lint suppressions:** none is needed (D13). If clippy ever reports dead code that can only be kept with `#[allow(dead_code)]` or similar, the implementer stops and reports; the owner chooses between suppressing it, removing the item, or adding a non-test use.

**Decided by the owner:**
- The four conflicting tests on `main` may be edited (D10); none removed.
- **No built-in areas:** the screen lists whatever the app has registered, and this change registers none. Vocabulary, Kanji and Grammar are added later by their own issues (#11, #12, #13), each with its own file and its own entry in the list (D2, D12).
