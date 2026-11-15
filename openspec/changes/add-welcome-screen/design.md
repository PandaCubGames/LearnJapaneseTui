# Design

## Context

See `proposal.md` for the motivation and `specs/welcome-screen/spec.md` and `specs/area-selection/spec.md` for the required behaviour.

- The code from #10 already has the shape this change needs: `keys::action_for(screen, event)` turns an event into an `Action` per `Screen`, `Session::apply` is the only place state changes, and `ui::draw` matches on `Session::screen()`. Adding a screen means adding a `Screen` variant, its key arm and its draw function.
- `Screen` is matched exhaustively in `keys::action_for` and `ui::draw`, so a new variant does not compile until both have an arm.
- The crate is a binary, so `cargo clippy --all-targets -- -D warnings` fails on any item that non-test code never uses, for example an `Action` variant that only tests construct. This decides the task order (D7).
- `openspec/specs/` is empty. The `app-shell` capability exists only as a delta of `add-main-application-loop`, which is complete (16/16) but not archived.
- The baseline is 29 passing tests (`cargo test`, checked while writing this design).

## Goals / Non-Goals

**Goals:**
- Enter on Welcome really changes the screen, and that is covered by tests that need no terminal.
- Issue #20 can replace the placeholder by editing only its own key arm, draw function and wording, without touching the Welcome screen.

**Non-Goals:**
- Any real behaviour on the area selection screen: the list, the highlight, Esc, `q`, the "no questions yet" message. All of it is #20.
- Removing or restyling the debug lines.
- A way back from area selection to Welcome. The planned flow (`docs/architecture.md` §1) has none.

## Decisions

### D1. Two new capabilities with ADDED requirements only; `app-shell` is left alone

The change adds `welcome-screen` and `area-selection` and does not touch `app-shell`.

- **Why not MODIFIED `app-shell`:** `app-shell` has no main spec yet. The OpenSpec 1.16 archive step refuses MODIFIED requirements for a spec that does not exist ("only ADDED requirements are allowed for new specs"). A MODIFIED delta would only archive after `add-main-application-loop` is archived, and a MODIFIED block must repeat every scenario of the original requirement word for word. ADDED-only deltas archive in either order.
- **Why it does not contradict `app-shell`:** its requirements stay true. `Ctrl+C` quits on every screen (also on the placeholder), `q` quits on Welcome, Enter does not quit (the *Other keys do not quit* scenario), and its *Welcome screen content* requirement lists lines the screen still shows; it does not say those are the only lines.
- **The cost:** what the Welcome screen shows is described in two capabilities until someone consolidates them (see Open Questions).
- **Why `area-selection` is its own capability:** #20 then modifies `area-selection` and never needs to touch `welcome-screen`. It also means this change has to be archived before #20 is, which is the natural order anyway.

### D2. Enter goes to a placeholder area selection screen

Enter on Welcome produces a new action, `Action::Start`. `Session::apply` turns it into a move to a new screen, `Screen::AreaSelection`, which draws a two-line placeholder.

Alternatives considered:
- **Wait for #20.** Rejected: #19 cannot be finished or tested on its own, and #20 has its own open questions (wrapping, `q`, how empty areas look) that would hold it up.
- **Build #19 and #20 together.** Rejected: one large change that mixes two issues, with more risk to existing tests and a longer review.
- **Map Enter to an action that changes nothing visible yet.** Rejected: the action would be dead code (fails clippy) or untestable in any useful way, and the user would see Enter do nothing, which is the bug the issue describes.

**What #20 inherits:** the `Screen::AreaSelection` variant, the `Action::Start` action and the transition in `Session::apply`, all unchanged. It replaces the body of the `Screen::AreaSelection` arm in `keys::action_for`, the function `ui::draw_area_selection`, and the two `AREA_SELECTION_*` constants in `src/text.rs` (removing or replacing those constants is #20's decision; the tests added here that pin the placeholder belong to #20's analysis of existing tests, see Open Questions). It adds its own state (the highlighted area) to `Session`.

### D3. Session

```rust
pub enum Screen { Welcome, AreaSelection }

pub enum Action {
    Quit,
    Typed(char),
    Resized { width: u16, height: u16 },
    Tick,
    /// Leave the Welcome screen for area selection.
    Start,
}
```

`Session::apply` gets one new arm:

```rust
Action::Start => {
    if self.screen == Screen::Welcome {
        self.screen = Screen::AreaSelection;
    }
}
```

- The guard keeps the transition one-way and safe if a later screen ever emits `Start`.
- No new public method. `Session::screen()` already exposes what the tests need.
- `Start` does not set `should_quit`.

### D4. Key table

Rules 1 to 3 of the existing table (resize, ignore releases, `Ctrl+C` quits on every screen) do not change. The per-screen part becomes:

- `Screen::Welcome`, checked in this order:
  1. `KeyCode::Enter` with no modifiers → `Some(Action::Start)` (new).
  2. `Char('q')` with no modifiers → `Quit` (unchanged).
  3. `Char(c)` with no modifiers or only `SHIFT` → `Typed(c)` (unchanged).
  4. Anything else → `None`.
- `Screen::AreaSelection`: everything → `None`. Only rules 1 and 3 apply there, so `Ctrl+C` quits and resizes are followed.

Decisions taken here:
- **`q` on Welcome still quits.** `keys::tests::q_quits_on_welcome` pins it and the spec keeps it.
- **On the placeholder, `q`, Esc, Enter, arrows and typed characters do nothing.** Whether `q` leaves area selection is an open question of #20, and Esc leaving is part of #20's scope. Binding them now would decide #20's questions for it.
- **Ctrl+Enter and other modified Enters do nothing on Welcome**, the same way modified letters do (`other_modifiers_do_nothing`).
- **A held Enter** (`KeyEventKind::Repeat`) also starts. The repeat that follows arrives on the placeholder, where Enter does nothing, so a held key cannot skip a screen.
- **IME:** while an IME is composing, Enter commits the composition inside the IME and the app only receives the committed text, so committing kana does not start the app.

### D5. Drawing

The Welcome screen gets the start hint and a blank line after it. Nine lines, centred as today (`Alignment::Center` horizontally, `Flex::Center` with the height taken from the number of lines):

```text
tui-learnjapanese v0.0.0            (bold)
(empty)
日本語を勉強しましょう！
(empty)
Press Enter to start                (new)
(empty)                             (new)
Size: 80x24   Elapsed: 12.3 s
Last key: あ
Press q or Ctrl+C to quit
```

The placeholder, with the same centring:

```text
Area selection is coming soon.
(empty)
Press Ctrl+C to quit
```

- The centring code moves from `draw_welcome` into a private helper, `fn draw_centred(frame: &mut Frame, lines: Vec<Line>)`, used by both `draw_welcome` and the new `draw_area_selection(frame: &mut Frame)`. The Welcome screen draws exactly as before apart from the two new lines.
- No colours, only the existing bold title. Nothing does its own arithmetic on the size, so tiny terminals clip as today.
- Horizontal positions do not depend on the number of lines, so the greeting stays at column 28 in 80 and 81 columns. Only the vertical position moves, and no existing test pins a row (D8).

### D6. Wording in `src/text.rs`

```rust
/// Welcome screen: how to go on.
pub const START_HINT: &str = "Press Enter to start";

/// Area selection, until the list of areas exists.
pub const AREA_SELECTION_SOON: &str = "Area selection is coming soon.";

/// Area selection: how to leave the app. Only Ctrl+C works there for now.
pub const AREA_SELECTION_QUIT_HINT: &str = "Press Ctrl+C to quit";
```

All three are plain ASCII, so none has an ambiguous width. `QUIT_HINT` is not reused on the placeholder because `q` does not quit there. The existing test `fixed_messages_are_not_empty` is not extended (that would edit it); a new test covers the new constants.

### D7. Task order and commits

Each task is one commit that builds and passes `cargo fmt --all -- --check`, `cargo clippy --all-targets -- -D warnings` and `cargo test`:

1. The start hint (`text::START_HINT` and the Welcome line). Self-contained: the constant is used as soon as it exists.
2. **The owner-approved test edit**, alone (D8). Enter still does nothing at this commit, so the edited test passes before and after.
3. Enter continues: `Screen::AreaSelection`, `Action::Start`, the session arm, both key arms, the placeholder drawing and its wording, with their tests. These cannot be split: the new `Screen` variant does not compile without arms in `keys` and `ui`, and `Action::Start` or `Screen::AreaSelection` without a non-test user is dead code that clippy rejects.
4. Documentation.
5. A scripted check in a pseudo-terminal (no file changes; committed as the ticked checkbox).

### D8. Existing tests

**Owner-approved edit (issue #19, the only one):** `keys::tests::other_keys_and_events_do_nothing` in `src/keys.rs`. The owner has approved exactly this edit: remove `KeyCode::Enter` from the array, so the loop reads `for code in [KeyCode::Esc, KeyCode::Up]`. The `FocusGained` assertion and everything else in the test stay as they are. Nothing else in that test, and no other existing test, is changed.

Every other existing test was checked against this design and passes unchanged:

| Test | Why it still passes |
|---|---|
| `ui::tests::a_fresh_session_shows_every_line` | Checks that strings are contained in the screen; all of them are still drawn. It does not check that nothing else is drawn. |
| `ui::tests::typed_character_and_elapsed_time_are_shown` | Same: `Last key: あ` and `Elapsed: 12.3 s` are still drawn. |
| `ui::tests::greeting_is_centred_by_display_width`, `greeting_starts_at_the_same_column_in_an_odd_width` | `greeting_row` finds the row by the symbol `日`, which only the greeting contains, so the row number does not matter. Horizontal centring is unchanged (28 in 80 and 81 columns). |
| `ui::tests::greeting_is_24_columns_but_12_characters` | Greeting unchanged. |
| `ui::tests::any_size_draws_without_panicking` | A taller block is clipped like the old one; no arithmetic on the size. |
| `session::tests::new_session_starts_on_welcome_with_nothing_recorded` | `Session::new` still starts on `Screen::Welcome`. |
| `session::tests::only_quit_sets_should_quit` | Applies only `Tick`, `Typed` and `Resized`, none of which changed. |
| other `session::tests` | Their arms are unchanged. |
| `keys::tests::characters_are_typed`, `q_quits_on_welcome`, `ctrl_c_quits`, `resize_becomes_resized`, `releases_are_ignored`, `a_held_key_repeats`, `other_modifiers_do_nothing` | All run on `Screen::Welcome` with keys whose arms are unchanged. The Enter arm comes before the `Char` arms and matches only `KeyCode::Enter`. |
| `text::tests::*` | Existing constants and functions unchanged. |
| `app::tests::*` | `src/app.rs` is not changed. |

No existing test pins a row number or the screen's height.

### D9. New tests

Named in `tasks.md`. All are unit tests in the file they cover, with no terminal. `src/ui.rs` gets one new test helper, `fn buffer(session: &Session) -> Buffer`, for the colour checks; the existing `render`, `text` and `greeting_row` helpers are reused and not changed.

## Risks / Trade-offs

- [The placeholder ships if #20 is late, and a user reaching it can only quit with `Ctrl+C`] → The placeholder says so on screen. Acceptable for an unreleased `0.0.0`.
- [Welcome requirements are split between `app-shell` and `welcome-screen`] → D1. Consolidation is an open question; nothing contradicts.
- [Tests added here that pin the placeholder (for example "Esc does nothing on area selection") will conflict with #20, and §7 forbids changing them without the owner] → #20's issue already says Esc leaves. Listed as an open question so the owner can decide now whether those tests should exist, or accept that #20 will ask for their removal.
- [A terminal reports Return as something other than `KeyCode::Enter`] → Not handled specially. Task 5.1 checks Enter in a pseudo-terminal; if it fails there, the implementer reports it instead of adding mappings.

## Open Questions

For the owner:

1. **Where Enter goes (D2).** This change adds a placeholder area selection screen so Enter can be built and tested now. The alternatives are waiting for #20 or building #19 and #20 together. Please confirm the placeholder.
2. **Placeholder tests and #20.** The new tests `keys::tests::other_keys_do_nothing_on_area_selection` and `ui::tests::area_selection_placeholder_is_shown` pin placeholder behaviour that #20 will change (Esc will leave; the text will become a list). Under §7, #20 will then have to ask you to change or remove them. Options: (a) accept that now, as planned here; (b) leave Esc out of `other_keys_do_nothing_on_area_selection` so #20 does not conflict on Esc. The tasks follow (a) unless you choose (b). Either way the `area-selection` spec requirement is modified by #20.
3. **Esc and `q` on the placeholder.** Planned: both do nothing (D4), leaving the decision to #20. Alternative: Esc quits now, matching #20's scope.
4. **Archiving `add-main-application-loop`.** Not required for this change: it uses only ADDED requirements and validates and archives on its own. It is recommended to archive `add-main-application-loop` before or together with this one so `openspec/specs/` holds `app-shell`. A later change may move the Welcome requirements from `app-shell` into `welcome-screen`.
