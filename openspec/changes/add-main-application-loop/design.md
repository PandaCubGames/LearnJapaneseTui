# Design

## Context

See `proposal.md` for the motivation and `specs/app-shell/spec.md` for the required behaviour.

- `src/main.rs` is the only source file. It prints a banner and exits.
- `docs/architecture.md` §5 already names the core components: `app` (event loop), `keys` (terminal key → action, per screen), `session` (state and screen flow) and `ui` (drawing). This change creates those four files with only what the loop needs.
- The release profile sets `panic = "abort"`. On a panic the process aborts without unwinding, so **destructors do not run**. A "restore the terminal on `Drop`" guard would not work in release builds. The panic hook does run before the abort.
- Issue #10 fixes two decisions: `ratatui` with the `crossterm` backend, and a fixed tick held in one named constant, starting at 100 ms.

## Goals / Non-Goals

**Goals:**
- Key handling, state changes, tick timing and drawing are each testable without a real terminal.
- The loop in `app.rs` is thin glue with no logic of its own worth testing.
- Later screens and areas are added by extending `Screen`, `Action` and the per-screen key table, without touching the loop.

**Non-Goals:**
- The real screen flow (area selection, level picker, questions, results, Thank you).
- The Area contract, grading, romaji conversion and learning content.
- Command-line arguments (`--version`, `--help`).
- Mouse support, bracketed paste and colour themes.
- A text input field. The Welcome screen only remembers the last character typed.

## Decisions

### D1. Depend on `ratatui` only, and use the `crossterm` it re-exports

`Cargo.toml` gets one line:

```toml
ratatui = { version = "0.30", default-features = false, features = ["crossterm", "layout-cache"] }
```

All crossterm items are imported through `ratatui::crossterm::…`.

- **Why:** a separate `crossterm` entry can drift to a version different from the one `ratatui` was built against. Two copies of crossterm in one binary keep separate raw-mode state, which breaks restoring.
- **Why no default features:** the defaults add the calendar widget, macros and underline colours, none of which this app uses. Fewer features means a smaller binary and a smaller licence surface.
- **Alternative considered:** `ratatui` 0.29 with a direct `crossterm` 0.28 dependency, which keeps Rust 1.85. Rejected: it starts the project on a superseded release.

### D2. Raise the minimum Rust version to 1.88

`ratatui` 0.30 declares `rust-version = "1.88.0"`. `Cargo.toml` changes `rust-version` from `"1.85"` to `"1.88"`, and `README.md` line 35 and `CONTRIBUTING.md` line 10 are updated to match. This is the toolchain minimum, not the project version, which stays `0.0.0` (`AGENTS.md` §4).

### D3. Terminal lifecycle through `ratatui::try_init` and `ratatui::restore`

`app::run` is the only place that touches the real terminal:

```rust
pub fn run() -> io::Result<()> {
    let mut terminal = match ratatui::try_init() {
        Ok(terminal) => terminal,
        Err(error) => {
            let _ = ratatui::try_restore(); // undo a half-finished setup
            return Err(error);
        }
    };
    let result = run_loop(&mut terminal);
    ratatui::restore();
    result
}
```

- `try_init` enables raw mode, enters the alternate screen and installs a panic hook that restores the terminal and then calls the previous hook. This covers the panic case under `panic = "abort"` (see Context).
- `restore` is called on the single path out of `run_loop`, whether it returned `Ok` or `Err`. That covers normal exit and errors.
- `main` returns `ExitCode`. On `Err` it prints `tui-learnjapanese: {error}` to standard error **after** `run` has returned, so the message lands on the normal screen, and returns `ExitCode::FAILURE`.
- **Alternative considered:** a hand-written guard type with `Drop`. Rejected because of `panic = "abort"`, and because it duplicates what `ratatui` already provides.

### D4. Module layout and types

| File | Holds | Touches the terminal? |
|---|---|---|
| `src/main.rs` | `mod` declarations, `main` | no |
| `src/app.rs` | `TICK_INTERVAL`, `Ticker`, `run`, `run_loop` | `run` and `run_loop` only |
| `src/keys.rs` | `action_for` | no |
| `src/session.rs` | `Screen`, `Action`, `Session` | no |
| `src/ui.rs` | `draw`, `GREETING` | no (draws into a `Frame`) |

```rust
// session.rs
pub enum Screen { Welcome }

pub enum Action {
    Quit,
    Typed(char),
    Resized { width: u16, height: u16 },
    Tick,
}

pub struct Session { /* private: screen, ticks: u64, last_char: Option<char>,
                        width: u16, height: u16, quit: bool */ }

impl Session {
    pub fn new(width: u16, height: u16) -> Self;   // Screen::Welcome, zero ticks
    pub fn apply(&mut self, action: Action);       // the only way state changes
    pub fn screen(&self) -> Screen;
    pub fn should_quit(&self) -> bool;
    pub fn elapsed(&self) -> Duration;             // TICK_INTERVAL * ticks
    pub fn last_char(&self) -> Option<char>;
    pub fn size(&self) -> (u16, u16);              // (width, height)
}

// keys.rs
pub fn action_for(screen: Screen, event: &Event) -> Option<Action>;

// ui.rs
pub const GREETING: &str = "日本語を勉強しましょう！";
pub fn draw(frame: &mut Frame, session: &Session);

// app.rs
pub const TICK_INTERVAL: Duration = Duration::from_millis(100);
```

- **Why `apply` is the only way state changes:** every event becomes an `Action` first, so a test can drive the whole app by calling `apply` in sequence. This is the "state and an action give the next state" step from `docs/architecture.md` §5.
- **Why `keys::action_for` takes crossterm's `Event` directly:** a crossterm `Event` is plain data that tests can build by hand, so a second, app-owned event type would only add a translation layer.
- **Why `Screen` is passed to `action_for`:** keys depend on the screen (`docs/architecture.md`, *Keys*). `q` quits on Welcome, but on a question screen it will be a letter of the answer.

### D5. Key table

`action_for` checks these in order and returns the first match:

1. `Event::Resize(w, h)` → `Resized { width: w, height: h }`.
2. A key event whose `kind` is `KeyEventKind::Release` → `None`. `Press` and `Repeat` continue. Windows reports releases, and without this every key acts twice.
3. `Char('c')` with the `CONTROL` modifier, on any screen → `Quit`.
4. On `Screen::Welcome`:
   - `Char('q')` with no modifiers → `Quit`.
   - `Char(c)` with no modifiers or only `SHIFT` → `Typed(c)`.
5. Anything else → `None`.

Raw mode turns off the terminal's own `Ctrl+C` signal, so `Ctrl+C` arrives as a key event and rule 3 is the only thing that makes it quit.

### D6. Tick timing: a small `Ticker` driven by the caller's clock

```rust
pub struct Ticker { interval: Duration, next: Instant }

impl Ticker {
    pub fn new(interval: Duration, now: Instant) -> Self;   // next = now + interval
    pub fn timeout(&self, now: Instant) -> Duration;        // next - now, zero if past
    pub fn due(&mut self, now: Instant) -> bool;
}
```

`due` returns `false` when `now < next`. Otherwise it advances `next` by one `interval`; if `next` is still not after `now` (the app stalled for more than one interval), it sets `next = now + interval`; then it returns `true`.

The loop:

```rust
fn run_loop(terminal: &mut DefaultTerminal) -> io::Result<()> {
    let size = terminal.size()?;
    let mut session = Session::new(size.width, size.height);
    let mut ticker = Ticker::new(TICK_INTERVAL, Instant::now());
    while !session.should_quit() {
        terminal.draw(|frame| ui::draw(frame, &session))?;
        if event::poll(ticker.timeout(Instant::now()))? {
            if let Some(action) = keys::action_for(session.screen(), &event::read()?) {
                session.apply(action);
            }
        }
        if ticker.due(Instant::now()) {
            session.apply(Action::Tick);
        }
    }
    Ok(())
}
```

- **Why the deadline is kept across iterations:** a plain `poll(TICK_INTERVAL)` restarts the wait on every key, so steady typing would starve the tick (spec scenario *Continuous input*).
- **Why `Ticker` takes `now` as an argument:** tests pass made-up instants and never sleep.
- **Why missed ticks are dropped, not replayed:** after a stall (for example a suspended SSH session) a burst of catch-up ticks is worse than a clock that lost time. `elapsed` is therefore approximate, which is all the placeholder needs.
- The screen is redrawn once per loop pass, so at most once per event or tick. `ratatui` only writes the cells that changed.

### D7. Welcome screen layout

Seven lines, each centred horizontally with `Alignment::Center`, and the block centred vertically:

```text
tui-learnjapanese v0.0.0            (bold)
(empty)
日本語を勉強しましょう！
(empty)
Size: 80x24   Elapsed: 12.3 s
Last key: あ                         ("Last key: -" before any key)
Press q or Ctrl+C to quit
```

- The version comes from `env!("CARGO_PKG_VERSION")`. The size comes from `Session::size`, the time from `Session::elapsed` with one decimal.
- `ratatui` measures text with the `unicode-width` crate, so centring is by display width. No direct `unicode-width` dependency is added in this change.
- **No East Asian Ambiguous-width characters in UI text.** Characters such as `×` and `—` are one column in most terminals and two in terminals set up for CJK, so a line containing them cannot be centred reliably. The size is written `80x24`, and the old banner's `—` is not reused.
- Only the bold modifier is used. No colours, so the screen reads the same in a monochrome terminal.
- When the terminal is smaller than the content, `ratatui` clips to the buffer. `draw` does no arithmetic of its own that could underflow.

### D8. Tests

All tests are `#[cfg(test)] mod tests` inside the file they test. None opens a terminal.

- `session.rs`: `apply` for each `Action`, and `elapsed`.
- `keys.rs`: every row of the key table in D5, built from hand-made `Event` values.
- `app.rs`: `Ticker` with made-up `Instant`s.
- `ui.rs`: `draw` into `Terminal::new(TestBackend::new(w, h))`, then assertions on the buffer.

## Risks / Trade-offs

- [The licence of a transitive dependency is not GPL-3.0-compatible] → Task 1.2 lists every licence in the tree before any code is written. If one fails, the work stops and the owner decides.
- [`default-features = false` leaves out something `try_init` or `TestBackend` needs] → Task 1.1 builds straight away. If it fails, the implementer reports the error instead of changing the feature list on their own.
- [The musl static build breaks] → `crossterm` is pure Rust on Unix, so this is unlikely. Task 6.3 builds the `x86_64-unknown-linux-musl` target to confirm.
- [The panic path cannot be covered by an automated test] → Task 6.2 checks it by hand, in a release build, with a temporary panic that is not committed.
- [Terminals with the kitty keyboard protocol could report `Ctrl+C` differently] → The app never enables that protocol, so crossterm reports `Char('c')` with `CONTROL`.
- [`q` as a quit key will clash with typed answers] → It is bound on the Welcome screen only (D5). `Ctrl+C` is the quit key that works everywhere.
- [Raising the minimum Rust to 1.88 excludes older toolchains] → Accepted. Release binaries are prebuilt, so only people building from source are affected.

## Open Questions

- Whether the placeholder Welcome screen's diagnostic lines (size, elapsed time, last key) stay once the real Welcome screen is designed. This is decided by the change that builds the screen flow.
