// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 PandaCubGames

//! The event loop and the terminal lifecycle.

use std::time::{Duration, Instant};

/// How often the app updates itself when no input arrives.
pub const TICK_INTERVAL: Duration = Duration::from_millis(100);

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
