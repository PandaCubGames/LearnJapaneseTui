# Proposal

## Why

Enter on the Welcome screen leads to a placeholder that says "Area selection is coming soon." and can only be left with `Ctrl+C`. The app needs the real screen where the user chooses what to practise: every practice session starts from it and returns to it (`docs/architecture.md` §1). Tracked in GitHub issue #20.

The screen is a generic place to select an area, any area. It must not matter which areas exist: it lists whatever the app has registered, so that areas can be added later, or not. **Owner decision (pull request review):** this change registers no area. Vocabulary, Kanji and Grammar, if they come, are added by their own issues (#11, #12, #13), each with its own file and its own entry in the list.

## What Changes

- The area selection placeholder is replaced by a list of the areas the app has registered, one per line, in the registered order. The first area is highlighted when the screen opens; Up and Down move the highlight and stop at the first and last line.
- The highlight is a `> ` marker in front of the area's name, so it is visible without colour.
- Enter chooses the highlighted area. An area with no questions shows a new screen with the area's name and `No questions in this area yet`; Enter (or Esc) there goes back to the list, with the same area still highlighted.
- **No area is registered by this change.** With an empty list the screen shows the heading, the line `Nothing to practise yet` in place of the list, and the same key hint. Up, Down and Enter do nothing there; Esc and `Ctrl+C` quit.
- Esc on the list quits the app (until the Thank you screen exists). `Ctrl+C` quits on both screens. `q`, letters, `j`/`k` and number keys do nothing.
- A hint line names the keys: `Up/Down to move, Enter to choose, Esc to leave`.
- A new module, `src/areas.rs`, is the one place that registers the app's areas, through a small `Area` trait (a name and whether it has any questions). The list is empty in this change. Adding an area later adds its own file under `src/areas/` and one entry in the list; the screen and the other areas do not change. Issue #21 (the generic practice module) grows this trait into the full area contract.
- Both screens, including the empty list, are centred by terminal display width, stay centred after a resize, draw at any size without panicking and use no colour.
- All wording goes in `src/text.rs`. The two placeholder constants are removed.
- **Existing tests:** four tests on `main` pin the placeholder and conflict with this change. The owner approved editing all four for issue #20 and asked that none be removed. Each keeps its name and intent and changes only what the new screen forces; `design.md` D10 records the decision and the edits, and `tasks.md` gives each test's exact new body. The revision for "no built-in areas" leaves all four exactly as approved (they still pass with the empty list, `design.md` D12).

Not in scope: any area and its content (#11, #12, #13); the JLPT level picker, question count, question, feedback and results screens (#21); the Thank you screen (until it exists, Esc quits directly); what Enter does on an area that has questions (#21; in this change it never shows the "no questions" screen and does not quit).

## Capabilities

### New Capabilities

None.

### Modified Capabilities
- `area-selection`: the placeholder requirements (content, keys, layout) are removed and replaced by the list of registered areas, the empty list, the highlight and its keys, choosing an area, the "no questions" screen, leaving with Esc, the hint line, and the layout rules. This capability is defined by the unarchived change `add-welcome-screen`, so `add-welcome-screen` must be archived before this change is (see `design.md` D1).

`welcome-screen` and `app-shell` do not change. Nothing here contradicts them: `Ctrl+C` still quits on every screen, `q` still quits on Welcome, and Enter on Welcome still leads to area selection.

## Impact

- **Code:** new `src/areas.rs` (the `Area` trait, the empty list and test-only fake areas); changes in `src/main.rs` (one `mod areas;` line), `src/session.rs` (area list, highlight, new actions, a new screen), `src/keys.rs` (keys for both screens), `src/ui.rs` (drawing both screens and the empty list), `src/text.rs` (new wording, placeholder wording removed). No file under `src/areas/` exists after this change. `src/app.rs` does not change.
- **Tests:** new tests in every file above; four existing tests are edited, as approved by the owner; none is removed (see `design.md` D10). Tests added earlier on this branch that pinned the built-in areas are rewritten with test-only areas or deleted (`tasks.md` group 6).
- **Docs:** `CHANGELOG.md` (*Unreleased*), the status note and Components section of `docs/architecture.md`.
- **Not affected:** dependencies, the project version, CI, learning content, `ATTRIBUTION.md`.
