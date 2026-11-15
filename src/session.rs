// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 PandaCubGames

//! The app's state. It changes only through [`Session::apply`], so the whole
//! app can be driven in tests without a terminal.

use std::time::Duration;

use crate::app::TICK_INTERVAL;
use crate::areas::{self, Area};

/// The screen currently shown.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Screen {
    Welcome,
    AreaSelection,
    /// The chosen area has no questions.
    NoQuestions,
}

/// Something that happened, already translated from a terminal event.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Quit,
    Typed(char),
    Resized {
        width: u16,
        height: u16,
    },
    Tick,
    /// Leave the Welcome screen for area selection.
    Start,
    /// Move the highlight to the previous area.
    Previous,
    /// Move the highlight to the next area.
    Next,
    /// Choose the highlighted area.
    Choose,
    /// Leave the "no questions" screen for the list.
    Back,
}

#[derive(Debug)]
pub struct Session {
    screen: Screen,
    ticks: u64,
    last_char: Option<char>,
    width: u16,
    height: u16,
    quit: bool,
    areas: &'static [&'static dyn Area],
    highlighted: usize,
}

impl Session {
    /// Starts on the Welcome screen, in a terminal of the given size.
    pub fn new(width: u16, height: u16) -> Self {
        Self::with_areas(width, height, areas::all())
    }

    /// Like `new`, with a given list of areas instead of the app's own.
    pub fn with_areas(width: u16, height: u16, areas: &'static [&'static dyn Area]) -> Self {
        Self {
            screen: Screen::Welcome,
            ticks: 0,
            last_char: None,
            width,
            height,
            quit: false,
            areas,
            highlighted: 0,
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

    /// The areas in the list, in order.
    pub fn areas(&self) -> &'static [&'static dyn Area] {
        self.areas
    }

    /// The index of the highlighted area in `areas()`. It stays `0` when the
    /// list is empty, and `highlighted_area()` is then `None`.
    pub fn highlighted(&self) -> usize {
        self.highlighted
    }

    /// The highlighted area, or `None` when there are no areas.
    pub fn highlighted_area(&self) -> Option<&'static dyn Area> {
        self.areas.get(self.highlighted).copied()
    }

    /// The terminal size as `(width, height)`.
    pub fn size(&self) -> (u16, u16) {
        (self.width, self.height)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::areas::fake::FakeArea;

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
    fn start_moves_from_welcome_to_area_selection() {
        let mut session = Session::new(80, 24);
        session.apply(Action::Start);
        assert_eq!(session.screen(), Screen::AreaSelection);
    }

    #[test]
    fn start_is_one_way() {
        let mut session = Session::new(80, 24);
        session.apply(Action::Start);
        session.apply(Action::Start);
        session.apply(Action::Typed('a'));
        session.apply(Action::Tick);
        session.apply(Action::Resized {
            width: 100,
            height: 30,
        });
        assert_eq!(session.screen(), Screen::AreaSelection);
    }

    #[test]
    fn start_does_not_quit() {
        let mut session = Session::new(80, 24);
        session.apply(Action::Start);
        assert!(!session.should_quit());
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

    const EMPTY_THREE: &[&dyn Area] = &[
        &FakeArea {
            name: "A",
            has_questions: false,
        },
        &FakeArea {
            name: "B",
            has_questions: false,
        },
        &FakeArea {
            name: "C",
            has_questions: false,
        },
    ];

    const ONE_FULL: &[&dyn Area] = &[&FakeArea {
        name: "Full",
        has_questions: true,
    }];

    fn name_of(session: &Session) -> Option<&'static str> {
        session.highlighted_area().map(|area| area.name())
    }

    #[test]
    fn new_session_lists_the_apps_areas() {
        let session = Session::new(80, 24);
        assert_eq!(session.areas().len(), areas::all().len());
        assert_eq!(session.highlighted(), 0);
    }

    #[test]
    fn start_highlights_the_first_area() {
        let mut session = Session::new(80, 24);
        session.apply(Action::Start);
        assert_eq!(name_of(&session), Some("Vocabulary"));
    }

    #[test]
    fn choosing_an_empty_area_shows_no_questions() {
        let mut session = Session::with_areas(80, 24, EMPTY_THREE);
        session.apply(Action::Start);
        session.apply(Action::Choose);
        assert_eq!(session.screen(), Screen::NoQuestions);
        assert_eq!(name_of(&session), Some("A"));
        assert!(!session.should_quit());
    }

    #[test]
    fn choosing_an_area_with_questions_does_not_say_it_is_empty() {
        let mut session = Session::with_areas(80, 24, ONE_FULL);
        session.apply(Action::Start);
        session.apply(Action::Choose);
        assert_ne!(session.screen(), Screen::NoQuestions);
        assert!(!session.should_quit());
    }

    #[test]
    fn back_returns_to_the_list() {
        let mut session = Session::with_areas(80, 24, EMPTY_THREE);
        session.apply(Action::Start);
        session.apply(Action::Choose);
        session.apply(Action::Back);
        assert_eq!(session.screen(), Screen::AreaSelection);
        assert_eq!(session.highlighted(), 0);
    }

    #[test]
    fn choose_and_back_act_only_on_their_own_screens() {
        let mut session = Session::with_areas(80, 24, EMPTY_THREE);
        session.apply(Action::Choose);
        session.apply(Action::Back);
        assert_eq!(session.screen(), Screen::Welcome);
        session.apply(Action::Start);
        session.apply(Action::Back);
        assert_eq!(session.screen(), Screen::AreaSelection);
        session.apply(Action::Choose);
        session.apply(Action::Choose);
        assert_eq!(session.screen(), Screen::NoQuestions);
    }

    #[test]
    fn choosing_with_no_areas_does_nothing() {
        let mut session = Session::with_areas(80, 24, &[]);
        session.apply(Action::Start);
        session.apply(Action::Choose);
        assert_eq!(session.screen(), Screen::AreaSelection);
        assert!(session.highlighted_area().is_none());
    }

    #[test]
    fn next_and_previous_move_the_highlight() {
        let mut session = Session::with_areas(80, 24, EMPTY_THREE);
        session.apply(Action::Start);
        session.apply(Action::Next);
        assert_eq!(session.highlighted(), 1);
        session.apply(Action::Next);
        assert_eq!(session.highlighted(), 2);
        session.apply(Action::Previous);
        assert_eq!(session.highlighted(), 1);
    }

    #[test]
    fn highlight_stops_at_the_last_area() {
        let mut session = Session::with_areas(80, 24, EMPTY_THREE);
        session.apply(Action::Start);
        for _ in 0..3 {
            session.apply(Action::Next);
        }
        assert_eq!(session.highlighted(), 2);
    }

    #[test]
    fn highlight_stops_at_the_first_area() {
        let mut session = Session::with_areas(80, 24, EMPTY_THREE);
        session.apply(Action::Start);
        session.apply(Action::Previous);
        assert_eq!(session.highlighted(), 0);
    }

    #[test]
    fn moving_only_works_on_area_selection() {
        let mut session = Session::with_areas(80, 24, EMPTY_THREE);
        session.apply(Action::Next);
        assert_eq!(session.highlighted(), 0);
        session.apply(Action::Start);
        session.apply(Action::Choose);
        assert_eq!(session.screen(), Screen::NoQuestions);
        session.apply(Action::Next);
        assert_eq!(session.highlighted(), 0);
    }

    #[test]
    fn back_keeps_the_moved_highlight() {
        let mut session = Session::with_areas(80, 24, EMPTY_THREE);
        session.apply(Action::Start);
        session.apply(Action::Next);
        session.apply(Action::Choose);
        assert_eq!(name_of(&session), Some("B"));
        session.apply(Action::Back);
        assert_eq!(session.screen(), Screen::AreaSelection);
        assert_eq!(session.highlighted(), 1);
    }

    #[test]
    fn moving_with_no_areas_does_nothing() {
        let mut session = Session::with_areas(80, 24, &[]);
        session.apply(Action::Start);
        session.apply(Action::Next);
        session.apply(Action::Previous);
        assert_eq!(session.highlighted(), 0);
    }

    #[test]
    fn an_empty_list_highlights_nothing() {
        let mut session = Session::with_areas(80, 24, &[]);
        assert!(session.areas().is_empty());
        assert_eq!(session.highlighted(), 0);
        assert!(session.highlighted_area().is_none());
        session.apply(Action::Start);
        assert_eq!(session.screen(), Screen::AreaSelection);
        assert_eq!(session.highlighted(), 0);
        assert!(session.highlighted_area().is_none());
    }
}
