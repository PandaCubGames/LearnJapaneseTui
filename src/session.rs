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
