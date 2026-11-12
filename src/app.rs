// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 PandaCubGames

//! The event loop and the terminal lifecycle.

use std::io;
use std::time::{Duration, Instant};

use ratatui::DefaultTerminal;
use ratatui::crossterm::{cursor, event, execute};

use crate::keys;
use crate::session::{Action, Session};
use crate::ui;

/// How often the app updates itself when no input arrives.
pub const TICK_INTERVAL: Duration = Duration::from_millis(100);

/// Takes over the terminal, runs the app until the user quits, and gives the
/// terminal back, whether the loop ended normally or with an error.
pub fn run() -> io::Result<()> {
    let mut terminal = match ratatui::try_init() {
        Ok(terminal) => terminal,
        Err(error) => {
            // Undo a half-finished setup, e.g. raw mode on but no alternate screen.
            let _ = ratatui::try_restore();
            return Err(error);
        }
    };
    show_cursor_on_panic();
    let result = run_loop(&mut terminal);
    ratatui::restore();
    result
}

/// `ratatui::try_init` installs a panic hook that leaves raw mode and the
/// alternate screen, but not one that shows the cursor: it leaves that to the
/// terminal's destructor. Release builds abort on panic, so destructors never
/// run and the cursor would stay hidden.
fn show_cursor_on_panic() {
    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        let _ = execute!(io::stdout(), cursor::Show);
        hook(info);
    }));
}

fn run_loop(terminal: &mut DefaultTerminal) -> io::Result<()> {
    let size = terminal.size()?;
    let mut session = Session::new(size.width, size.height);
    let mut ticker = Ticker::new(TICK_INTERVAL, Instant::now());
    while !session.should_quit() {
        terminal.draw(|frame| ui::draw(frame, &session))?;
        if event::poll(ticker.timeout(Instant::now()))?
            && let Some(action) = keys::action_for(session.screen(), &event::read()?)
        {
            session.apply(action);
        }
        if ticker.due(Instant::now()) {
            session.apply(Action::Tick);
        }
    }
    Ok(())
}

/// Decides when the next tick is due.
///
/// The caller passes the current time in, so tests never sleep. The deadline
/// is kept between calls, so a stream of key presses cannot postpone a tick.
pub struct Ticker {
    interval: Duration,
    next: Instant,
}

impl Ticker {
    pub fn new(interval: Duration, now: Instant) -> Self {
        Self {
            interval,
            next: now + interval,
        }
    }

    /// How long to wait for an event before the next tick is due.
    pub fn timeout(&self, now: Instant) -> Duration {
        self.next.saturating_duration_since(now)
    }

    /// Returns `true` once per elapsed interval. After a stall longer than
    /// one interval the missed ticks are dropped, not replayed in a burst.
    pub fn due(&mut self, now: Instant) -> bool {
        if now < self.next {
            return false;
        }
        self.next += self.interval;
        if self.next <= now {
            self.next = now + self.interval;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INTERVAL: Duration = Duration::from_millis(100);

    fn ms(millis: u64) -> Duration {
        Duration::from_millis(millis)
    }

    #[test]
    fn timeout_counts_down_to_the_deadline() {
        let start = Instant::now();
        let ticker = Ticker::new(INTERVAL, start);
        assert_eq!(ticker.timeout(start), ms(100));
        assert_eq!(ticker.timeout(start + ms(30)), ms(70));
        assert_eq!(ticker.timeout(start + ms(250)), Duration::ZERO);
    }

    #[test]
    fn due_only_at_the_deadline() {
        let start = Instant::now();
        let mut ticker = Ticker::new(INTERVAL, start);
        assert!(!ticker.due(start + ms(99)));
        assert!(ticker.due(start + ms(100)));
        assert_eq!(ticker.timeout(start + ms(100)), ms(100));
    }

    #[test]
    fn steady_ticks_do_not_drift() {
        let start = Instant::now();
        let mut ticker = Ticker::new(INTERVAL, start);
        // Each check arrives 5 ms late, but the deadlines stay on the grid.
        assert!(ticker.due(start + ms(105)));
        assert!(ticker.due(start + ms(205)));
        assert!(ticker.due(start + ms(305)));
        assert_eq!(ticker.timeout(start + ms(305)), ms(95));
    }

    #[test]
    fn a_stall_gives_one_tick_not_a_burst() {
        let start = Instant::now();
        let mut ticker = Ticker::new(INTERVAL, start);
        assert!(ticker.due(start + ms(1000)));
        assert!(!ticker.due(start + ms(1000)));
        assert_eq!(ticker.timeout(start + ms(1000)), ms(100));
    }
}
