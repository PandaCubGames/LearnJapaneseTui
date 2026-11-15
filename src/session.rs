// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 PandaCubGames

//! The app's state. It changes only through [`Session::apply`], so the whole
//! app can be driven in tests without a terminal.

use std::time::Duration;

use crate::app::TICK_INTERVAL;
use crate::areas::{self, Area};
use crate::practice::{self, Level, Levels};

/// The screen currently shown.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Screen {
    Welcome,
    AreaSelection,
    /// The chosen area has no questions.
    NoQuestions,
    /// Choosing the JLPT levels to practise.
    LevelPicker,
    /// No question exists at the ticked levels.
    NoQuestionsAtLevels,
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
    /// Move the highlight up (area selection, level picker).
    Previous,
    /// Move the highlight down (area selection, level picker).
    Next,
    /// Choose the highlighted area.
    Choose,
    /// Tick or untick the highlighted level.
    Toggle,
    /// Start practice with the ticked levels.
    Confirm,
    /// Go on: back to the level picker.
    Continue,
    /// Go back to area selection.
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
    levels: Levels,
    level_highlighted: usize,
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
            levels: Levels::only(Level::N5),
            level_highlighted: 0,
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
            Action::Previous => match self.screen {
                Screen::AreaSelection => self.highlighted = self.highlighted.saturating_sub(1),
                Screen::LevelPicker => {
                    self.level_highlighted = self.level_highlighted.saturating_sub(1);
                }
                _ => {}
            },
            Action::Next => match self.screen {
                Screen::AreaSelection if self.highlighted + 1 < self.areas.len() => {
                    self.highlighted += 1;
                }
                Screen::LevelPicker if self.level_highlighted + 1 < Level::ALL.len() => {
                    self.level_highlighted += 1;
                }
                _ => {}
            },
            Action::Choose => {
                if self.screen == Screen::AreaSelection
                    && let Some(area) = self.highlighted_area()
                {
                    if area.has_questions() {
                        self.screen = Screen::LevelPicker;
                        self.level_highlighted = 0;
                    } else {
                        self.screen = Screen::NoQuestions;
                    }
                }
            }
            Action::Toggle => {
                if self.screen == Screen::LevelPicker {
                    self.levels = self.levels.toggled(self.highlighted_level());
                }
            }
            Action::Confirm => {
                if self.screen == Screen::LevelPicker
                    && !self.levels.is_empty()
                    && let Some(area) = self.highlighted_area()
                    && practice::selection(area.questions(self.levels), self.levels).is_empty()
                {
                    self.screen = Screen::NoQuestionsAtLevels;
                }
                // Practice starts here (task 3.1).
            }
            Action::Continue => {
                if self.screen == Screen::NoQuestionsAtLevels {
                    self.screen = Screen::LevelPicker;
                }
            }
            Action::Back => {
                if matches!(
                    self.screen,
                    Screen::NoQuestions | Screen::LevelPicker | Screen::NoQuestionsAtLevels
                ) {
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

    /// The index of the highlighted area in `areas()`.
    pub fn highlighted(&self) -> usize {
        self.highlighted
    }

    /// The highlighted area, or `None` when there are no areas.
    pub fn highlighted_area(&self) -> Option<&'static dyn Area> {
        self.areas.get(self.highlighted).copied()
    }

    /// The levels ticked in the level picker.
    pub fn levels(&self) -> Levels {
        self.levels
    }

    /// The level the level picker's highlight is on.
    pub fn highlighted_level(&self) -> Level {
        Level::ALL
            .get(self.level_highlighted)
            .copied()
            .unwrap_or(Level::N5)
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
    use crate::practice::{Level, Levels};

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

    const FULL_ONE: &[&dyn Area] = &[&FakeArea {
        name: "Full",
        has_questions: true,
    }];

    const FULL_TWO: &[&dyn Area] = &[
        &FakeArea {
            name: "Full A",
            has_questions: true,
        },
        &FakeArea {
            name: "Full B",
            has_questions: true,
        },
    ];

    fn on_picker(areas: &'static [&'static dyn Area]) -> Session {
        let mut session = Session::with_areas(80, 24, areas);
        session.apply(Action::Start);
        session.apply(Action::Choose);
        session
    }

    #[test]
    fn choosing_an_area_with_questions_opens_the_level_picker() {
        let session = on_picker(FULL_ONE);
        assert_eq!(session.screen(), Screen::LevelPicker);
        assert_eq!(session.highlighted_level(), Level::N5);
        assert_eq!(session.levels(), Levels::only(Level::N5));
    }

    #[test]
    fn level_highlight_moves_and_stops() {
        let mut session = on_picker(FULL_ONE);
        session.apply(Action::Next);
        session.apply(Action::Next);
        session.apply(Action::Previous);
        assert_eq!(session.highlighted_level(), Level::N4);
        for _ in 0..5 {
            session.apply(Action::Previous);
        }
        assert_eq!(session.highlighted_level(), Level::N5);
        for _ in 0..6 {
            session.apply(Action::Next);
        }
        assert_eq!(session.highlighted_level(), Level::N1);
    }

    #[test]
    fn moving_levels_keeps_the_area_highlight() {
        let mut session = Session::with_areas(80, 24, FULL_TWO);
        session.apply(Action::Start);
        session.apply(Action::Next);
        session.apply(Action::Choose);
        session.apply(Action::Next);
        assert_eq!(session.highlighted(), 1);
    }

    #[test]
    fn back_from_the_picker_returns_to_area_selection() {
        let mut session = Session::with_areas(80, 24, FULL_TWO);
        session.apply(Action::Start);
        session.apply(Action::Next);
        session.apply(Action::Choose);
        session.apply(Action::Back);
        assert_eq!(session.screen(), Screen::AreaSelection);
        assert_eq!(session.highlighted(), 1);
    }

    #[test]
    fn picker_opens_on_n5_each_time() {
        let mut session = on_picker(FULL_ONE);
        session.apply(Action::Next);
        session.apply(Action::Back);
        session.apply(Action::Choose);
        assert_eq!(session.highlighted_level(), Level::N5);
    }

    #[test]
    fn toggle_ticks_the_highlighted_level() {
        let mut session = on_picker(FULL_ONE);
        session.apply(Action::Next);
        session.apply(Action::Toggle);
        assert!(session.levels().contains(Level::N5));
        assert!(session.levels().contains(Level::N4));
        session.apply(Action::Toggle);
        assert!(!session.levels().contains(Level::N4));
    }

    #[test]
    fn unticking_the_last_level_leaves_none() {
        let mut session = on_picker(FULL_ONE);
        session.apply(Action::Toggle);
        assert!(session.levels().is_empty());
    }

    #[test]
    fn ticks_are_kept_after_going_back() {
        let mut session = on_picker(FULL_ONE);
        session.apply(Action::Next);
        session.apply(Action::Toggle);
        session.apply(Action::Back);
        session.apply(Action::Choose);
        assert!(session.levels().contains(Level::N5));
        assert!(session.levels().contains(Level::N4));
        assert_eq!(session.highlighted_level(), Level::N5);
    }

    #[test]
    fn toggle_only_works_on_the_picker() {
        let mut session = Session::with_areas(80, 24, FULL_ONE);
        session.apply(Action::Start);
        session.apply(Action::Toggle);
        assert_eq!(session.levels(), Levels::only(Level::N5));
    }

    const SAMPLE: &[&dyn Area] = &[&crate::areas::sample::Sample];

    #[test]
    fn the_sample_area_opens_the_level_picker() {
        let session = on_picker(SAMPLE);
        assert_eq!(session.screen(), Screen::LevelPicker);
    }

    fn sample_picker() -> Session {
        on_picker(SAMPLE)
    }

    /// Unticks N5 and ticks N2 and N1.
    fn only_high_levels(session: &mut Session) {
        session.apply(Action::Toggle);
        session.apply(Action::Next);
        session.apply(Action::Next);
        session.apply(Action::Next);
        session.apply(Action::Toggle);
        session.apply(Action::Next);
        session.apply(Action::Toggle);
    }

    #[test]
    fn levels_without_questions_say_so() {
        let mut session = sample_picker();
        only_high_levels(&mut session);
        session.apply(Action::Confirm);
        assert_eq!(session.screen(), Screen::NoQuestionsAtLevels);
    }

    #[test]
    fn levels_with_questions_do_not_say_so() {
        let mut session = sample_picker();
        session.apply(Action::Confirm);
        assert_ne!(session.screen(), Screen::NoQuestionsAtLevels);
        assert!(!session.should_quit());
    }

    #[test]
    fn confirm_with_nothing_ticked_does_nothing() {
        let mut session = sample_picker();
        session.apply(Action::Toggle);
        session.apply(Action::Confirm);
        assert_eq!(session.screen(), Screen::LevelPicker);
    }

    #[test]
    fn continue_returns_to_the_picker_with_the_same_ticks() {
        let mut session = sample_picker();
        only_high_levels(&mut session);
        let ticks = session.levels();
        session.apply(Action::Confirm);
        session.apply(Action::Continue);
        assert_eq!(session.screen(), Screen::LevelPicker);
        assert_eq!(session.levels(), ticks);
    }

    #[test]
    fn back_from_no_questions_at_levels_returns_to_area_selection() {
        let mut session = sample_picker();
        only_high_levels(&mut session);
        session.apply(Action::Confirm);
        session.apply(Action::Back);
        assert_eq!(session.screen(), Screen::AreaSelection);
    }

    #[test]
    fn an_area_that_supplies_nothing_says_so_at_every_level() {
        let mut session = on_picker(FULL_ONE);
        session.apply(Action::Confirm);
        assert_eq!(session.screen(), Screen::NoQuestionsAtLevels);
    }

    #[test]
    fn confirm_and_continue_act_only_on_their_own_screens() {
        let mut session = Session::with_areas(80, 24, SAMPLE);
        session.apply(Action::Start);
        session.apply(Action::Confirm);
        session.apply(Action::Continue);
        assert_eq!(session.screen(), Screen::AreaSelection);
        session.apply(Action::Choose);
        session.apply(Action::Continue);
        assert_eq!(session.screen(), Screen::LevelPicker);
    }
}
