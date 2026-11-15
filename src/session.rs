// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 PandaCubGames

//! The app's state. It changes only through [`Session::apply`], so the whole
//! app can be driven in tests without a terminal.

use std::time::Duration;

use crate::app::TICK_INTERVAL;

/// The screen currently shown.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Screen {
    Welcome,
}

/// Something that happened, already translated from a terminal event.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Quit,
    Typed(char),
    Resized { width: u16, height: u16 },
    Tick,
}

#[derive(Debug)]
pub struct Session {
    screen: Screen,
    ticks: u64,
    last_char: Option<char>,
    width: u16,
    height: u16,
    quit: bool,
}

impl Session {
    /// Starts on the Welcome screen, in a terminal of the given size.
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            screen: Screen::Welcome,
            ticks: 0,
            last_char: None,
            width,
            height,
            quit: false,
        }
    }

    pub fn apply(&mut self, action: Action) {
        match action {
            Action::Quit => self.quit = true,
            Action::Typed(c) => self.last_char = Some(c),
            Action::Resized { width, height } => {
                self.width = width;
                self.height = height;
            }
            Action::Tick => self.ticks = self.ticks.saturating_add(1),
        }
    }

    pub fn screen(&self) -> Screen {
        self.screen
    }

    pub fn should_quit(&self) -> bool {
        self.quit
    }

    /// Time since start, counted in ticks, so it is approximate.
    pub fn elapsed(&self) -> Duration {
        let ticks = u32::try_from(self.ticks).unwrap_or(u32::MAX);
        TICK_INTERVAL.saturating_mul(ticks)
    }

    pub fn last_char(&self) -> Option<char> {
        self.last_char
    }

    /// The terminal size as `(width, height)`.
    pub fn size(&self) -> (u16, u16) {
        (self.width, self.height)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_session_starts_on_welcome_with_nothing_recorded() {
        let session = Session::new(80, 24);
        assert_eq!(session.screen(), Screen::Welcome);
        assert_eq!(session.elapsed(), Duration::ZERO);
        assert_eq!(session.last_char(), None);
        assert_eq!(session.size(), (80, 24));
        assert!(!session.should_quit());
    }

    #[test]
    fn quit_sets_should_quit() {
        let mut session = Session::new(80, 24);
        session.apply(Action::Quit);
        assert!(session.should_quit());
    }

    #[test]
    fn typed_keeps_the_last_character() {
        let mut session = Session::new(80, 24);
        session.apply(Action::Typed('あ'));
        assert_eq!(session.last_char(), Some('あ'));
        session.apply(Action::Typed('a'));
        assert_eq!(session.last_char(), Some('a'));
    }

    #[test]
    fn resized_updates_the_size() {
        let mut session = Session::new(80, 24);
        session.apply(Action::Resized {
            width: 100,
            height: 30,
        });
        assert_eq!(session.size(), (100, 30));
    }

    #[test]
    fn ticks_advance_elapsed_time() {
        let mut session = Session::new(80, 24);
        for _ in 0..25 {
            session.apply(Action::Tick);
        }
        assert_eq!(session.elapsed(), Duration::from_millis(2500));
    }

    #[test]
    fn only_quit_sets_should_quit() {
        let mut session = Session::new(80, 24);
        session.apply(Action::Tick);
        session.apply(Action::Typed('q'));
        session.apply(Action::Resized {
            width: 1,
            height: 1,
        });
        assert!(!session.should_quit());
    }
}
