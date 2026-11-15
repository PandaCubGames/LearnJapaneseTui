# Proposal

## Why

The Welcome screen built in #10 only proves the main loop works: Enter does nothing, and the only thing a user can do is quit. It should be the app's real first screen, greeting the user and leading into the app. Tracked in GitHub issue #19.

## What Changes

- The Welcome screen gets a new line, `Press Enter to start`, between the greeting and the debug lines.
- Enter on the Welcome screen continues to the next screen in the flow (`docs/architecture.md` §1), area selection.
- Area selection itself is issue #20 and is not built here. This change adds a minimal **placeholder** for it: one screen that says area selection is coming and that `Ctrl+C` quits. Issue #20 replaces its content and keys.
- `q` and `Ctrl+C` still quit on the Welcome screen. On the placeholder only `Ctrl+C` quits (and resizes still work); every other key does nothing. There is no way back to the Welcome screen, as in the planned flow.
- Kept as they are, at the owner's decision: the debug lines (terminal size, elapsed time, last key typed) and the quit hint. Removing the debug lines is a later change.
- Both screens are centred by terminal display width, stay centred after a resize, draw at any size without panicking and use no colour.
- All new wording goes in `src/text.rs`.
- One existing test, `keys::tests::other_keys_and_events_do_nothing`, loses `KeyCode::Enter` from its list of keys that do nothing on Welcome. The owner approved this exact edit in issue #19. No other existing test changes.

Not in scope: removing the debug lines, the real area selection screen (#20), the Thank you screen, a logo, ASCII art or animation.

## Capabilities

### New Capabilities
- `welcome-screen`: the app's first screen: what it shows, how it is laid out, and the keys that start the app or quit it.
- `area-selection`: the screen the user reaches by starting from Welcome. In this change it is only a placeholder that can be left with `Ctrl+C`; issue #20 modifies this capability to add the real list of areas.

### Modified Capabilities

None. `openspec/specs/` is empty. The `app-shell` capability exists only as a delta in the unarchived change `add-main-application-loop`, so it cannot be modified yet (see `design.md` D1). Nothing here contradicts it: `Ctrl+C` quits on every screen, `q` quits on Welcome and Enter does not quit.

## Impact

- **Code:** `src/session.rs` (a new `Screen` variant and a new `Action`), `src/keys.rs` (Enter on Welcome, a key table for the placeholder), `src/ui.rs` (the new line and the placeholder screen), `src/text.rs` (new wording). `src/app.rs` and `src/main.rs` do not change.
- **Tests:** new tests in all four files; the one approved edit above.
- **Docs:** `CHANGELOG.md` (*Unreleased*), the status note in `docs/architecture.md`.
- **Not affected:** dependencies, the project version, CI, learning content.
