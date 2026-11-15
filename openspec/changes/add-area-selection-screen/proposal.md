# Proposal

## Why

Enter on the Welcome screen leads to a placeholder that says "Area selection is coming soon." and can only be left with `Ctrl+C`. The app needs the real screen where the user chooses what to practise: every practice session starts from it and returns to it (`docs/architecture.md` §1). Tracked in GitHub issue #20.

## What Changes

- The area selection placeholder is replaced by a list of areas, one per line: Vocabulary, Kanji, Grammar. The first area is highlighted when the screen opens; Up and Down move the highlight and stop at the first and last line.
- The highlight is a `> ` marker in front of the area's name, so it is visible without colour.
- Enter chooses the highlighted area. An area with no questions shows a new screen with the area's name and `No questions in this area yet`; Enter (or Esc) there goes back to the list, with the same area still highlighted. Until the content modules exist (#11, #12, #13), all three areas have no questions, so all three behave this way.
- Esc on the list quits the app (until the Thank you screen exists). `Ctrl+C` quits on both screens. `q`, letters, `j`/`k` and number keys do nothing.
- A hint line names the keys: `Up/Down to move, Enter to choose, Esc to leave`.
- A new module, `src/areas.rs`, is the one place that lists the app's areas. Each area is a type implementing a small `Area` trait (a name and whether it has any questions), in its own file under `src/areas/`. Adding an area later adds one file and one line to the list; the screen and the other areas do not change. Issue #21 (the generic practice module) grows this trait into the full area contract.
- Both screens are centred by terminal display width, stay centred after a resize, draw at any size without panicking and use no colour.
- All wording goes in `src/text.rs`. The two placeholder constants are removed.
- **Existing tests:** four tests on `main` pin the placeholder and conflict with this change. The owner approved editing all four for issue #20 and asked that none be removed. Each keeps its name and intent and changes only what the new screen forces; `design.md` D10 records the decision and the edits, and `tasks.md` gives each test's exact new body (the key test as its own task before the feature, the other three inside the feature task, because they assert the new screen).

Not in scope: the areas' content (#11, #12, #13); the JLPT level picker, question count, question, feedback and results screens (#21); the Thank you screen (until it exists, Esc quits directly); what Enter does on an area that has questions (#21; in this change it never shows the "no questions" screen and does not quit).

## Capabilities

### New Capabilities

None.

### Modified Capabilities
- `area-selection`: the placeholder requirements (content, keys, layout) are removed and replaced by the list of areas, the highlight and its keys, choosing an area, the "no questions" screen, leaving with Esc, the hint line, and the layout rules. This capability is defined by the unarchived change `add-welcome-screen`, so `add-welcome-screen` must be archived before this change is (see `design.md` D1).

`welcome-screen` and `app-shell` do not change. Nothing here contradicts them: `Ctrl+C` still quits on every screen, `q` still quits on Welcome, and Enter on Welcome still leads to area selection.

## Impact

- **Code:** new `src/areas.rs`, `src/areas/vocabulary.rs`, `src/areas/kanji.rs`, `src/areas/grammar.rs`; changes in `src/main.rs` (one `mod areas;` line), `src/session.rs` (area list, highlight, new actions, a new screen), `src/keys.rs` (keys for both screens), `src/ui.rs` (drawing both screens), `src/text.rs` (new wording, placeholder wording removed). `src/app.rs` does not change.
- **Tests:** new tests in every file above; four existing tests are edited, as approved by the owner; none is removed (see `design.md` D10).
- **Docs:** `CHANGELOG.md` (*Unreleased*), the status note and Components section of `docs/architecture.md`.
- **Not affected:** dependencies, the project version, CI, learning content, `ATTRIBUTION.md`.
