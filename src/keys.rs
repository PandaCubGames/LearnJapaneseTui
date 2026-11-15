// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 PandaCubGames

//! Turns a terminal event into an [`Action`] for the current screen.

use ratatui::crossterm::event::{Event, KeyCode, KeyEventKind, KeyModifiers};

use crate::session::{Action, Screen};

/// Returns the action for `event` on `screen`, or `None` when the event does
/// nothing there.
pub fn action_for(screen: Screen, event: &Event) -> Option<Action> {
    let key = match event {
        Event::Resize(width, height) => {
            return Some(Action::Resized {
                width: *width,
                height: *height,
            });
        }
        Event::Key(key) => key,
        _ => return None,
    };

    // Some terminals (Windows) report the release too. Acting on it would
    // make every key count twice.
    if key.kind == KeyEventKind::Release {
        return None;
    }

    // Raw mode turns off the terminal's own Ctrl+C, so it arrives as a key.
    if key.code == KeyCode::Char('c') && key.modifiers == KeyModifiers::CONTROL {
        return Some(Action::Quit);
    }

    match screen {
        Screen::Welcome => match key.code {
            KeyCode::Enter if key.modifiers.is_empty() => Some(Action::Start),
            KeyCode::Char('q') if key.modifiers.is_empty() => Some(Action::Quit),
            KeyCode::Char(c) if (key.modifiers - KeyModifiers::SHIFT).is_empty() => {
                Some(Action::Typed(c))
            }
            _ => None,
        },
        Screen::AreaSelection => match key.code {
            KeyCode::Up if key.modifiers.is_empty() => Some(Action::Previous),
            KeyCode::Down if key.modifiers.is_empty() => Some(Action::Next),
            KeyCode::Enter if key.modifiers.is_empty() && key.kind == KeyEventKind::Press => {
                Some(Action::Choose)
            }
            KeyCode::Esc if key.modifiers.is_empty() && key.kind == KeyEventKind::Press => {
                Some(Action::Quit)
            }
            _ => None,
        },
        Screen::NoQuestions => match key.code {
            KeyCode::Enter | KeyCode::Esc
                if key.modifiers.is_empty() && key.kind == KeyEventKind::Press =>
            {
                Some(Action::Back)
            }
            _ => None,
        },
        Screen::LevelPicker => match key.code {
            KeyCode::Up if key.modifiers.is_empty() => Some(Action::Previous),
            KeyCode::Down if key.modifiers.is_empty() => Some(Action::Next),
            KeyCode::Char(' ') if key.modifiers.is_empty() && key.kind == KeyEventKind::Press => {
                Some(Action::Toggle)
            }
            KeyCode::Esc if key.modifiers.is_empty() && key.kind == KeyEventKind::Press => {
                Some(Action::Back)
            }
            _ => None,
        },
    }
}

#[cfg(test)]
mod tests {
    use ratatui::crossterm::event::KeyEvent;

    use super::*;
    use crate::areas::{Area, fake::FakeArea};
    use crate::session::Session;

    fn press(code: KeyCode, modifiers: KeyModifiers) -> Event {
        Event::Key(KeyEvent::new(code, modifiers))
    }

    fn with_kind(code: KeyCode, kind: KeyEventKind) -> Event {
        Event::Key(KeyEvent::new_with_kind(code, KeyModifiers::NONE, kind))
    }

    fn on_welcome(event: Event) -> Option<Action> {
        action_for(Screen::Welcome, &event)
    }

    fn on_area_selection(event: Event) -> Option<Action> {
        action_for(Screen::AreaSelection, &event)
    }

    fn on_no_questions(event: Event) -> Option<Action> {
        action_for(Screen::NoQuestions, &event)
    }

    fn on_level_picker(event: Event) -> Option<Action> {
        action_for(Screen::LevelPicker, &event)
    }

    const EMPTY_ONE: &[&dyn Area] = &[&FakeArea {
        name: "Empty",
        has_questions: false,
    }];

    #[test]
    fn enter_starts_on_welcome() {
        assert_eq!(
            on_welcome(press(KeyCode::Enter, KeyModifiers::NONE)),
            Some(Action::Start)
        );
    }

    #[test]
    fn enter_release_does_nothing() {
        assert_eq!(
            on_welcome(with_kind(KeyCode::Enter, KeyEventKind::Release)),
            None
        );
    }

    #[test]
    fn modified_enter_does_nothing_on_welcome() {
        assert_eq!(
            on_welcome(press(KeyCode::Enter, KeyModifiers::CONTROL)),
            None
        );
        assert_eq!(on_welcome(press(KeyCode::Enter, KeyModifiers::ALT)), None);
    }

    #[test]
    fn ctrl_c_quits_on_area_selection() {
        assert_eq!(
            on_area_selection(press(KeyCode::Char('c'), KeyModifiers::CONTROL)),
            Some(Action::Quit)
        );
    }

    #[test]
    fn resize_is_followed_on_area_selection() {
        assert_eq!(
            on_area_selection(Event::Resize(100, 30)),
            Some(Action::Resized {
                width: 100,
                height: 30
            })
        );
    }

    #[test]
    fn other_keys_do_nothing_on_area_selection() {
        for code in [KeyCode::Char('q'), KeyCode::Char('a'), KeyCode::Char('あ')] {
            assert_eq!(on_area_selection(press(code, KeyModifiers::NONE)), None);
        }
        assert_eq!(on_area_selection(Event::FocusGained), None);
    }

    #[test]
    fn enter_then_ctrl_c_through_the_key_table() {
        let mut session = Session::new(80, 24);
        let feed = |session: &mut Session, event: Event| {
            if let Some(action) = action_for(session.screen(), &event) {
                session.apply(action);
            }
        };
        feed(&mut session, press(KeyCode::Enter, KeyModifiers::NONE));
        assert_eq!(session.screen(), Screen::AreaSelection);
        feed(&mut session, press(KeyCode::Char('q'), KeyModifiers::NONE));
        assert_eq!(session.screen(), Screen::AreaSelection);
        assert!(!session.should_quit());
        feed(
            &mut session,
            press(KeyCode::Char('c'), KeyModifiers::CONTROL),
        );
        assert!(session.should_quit());
    }

    #[test]
    fn resize_becomes_resized() {
        assert_eq!(
            on_welcome(Event::Resize(100, 30)),
            Some(Action::Resized {
                width: 100,
                height: 30
            })
        );
    }

    #[test]
    fn ctrl_c_quits() {
        assert_eq!(
            on_welcome(press(KeyCode::Char('c'), KeyModifiers::CONTROL)),
            Some(Action::Quit)
        );
    }

    #[test]
    fn q_quits_on_welcome() {
        assert_eq!(
            on_welcome(press(KeyCode::Char('q'), KeyModifiers::NONE)),
            Some(Action::Quit)
        );
    }

    #[test]
    fn releases_are_ignored() {
        assert_eq!(
            on_welcome(with_kind(KeyCode::Char('q'), KeyEventKind::Release)),
            None
        );
        assert_eq!(
            on_welcome(with_kind(KeyCode::Char('a'), KeyEventKind::Release)),
            None
        );
    }

    #[test]
    fn characters_are_typed() {
        assert_eq!(
            on_welcome(press(KeyCode::Char('a'), KeyModifiers::NONE)),
            Some(Action::Typed('a'))
        );
        assert_eq!(
            on_welcome(press(KeyCode::Char('A'), KeyModifiers::SHIFT)),
            Some(Action::Typed('A'))
        );
        assert_eq!(
            on_welcome(press(KeyCode::Char('あ'), KeyModifiers::NONE)),
            Some(Action::Typed('あ'))
        );
    }

    #[test]
    fn a_held_key_repeats() {
        assert_eq!(
            on_welcome(with_kind(KeyCode::Char('a'), KeyEventKind::Repeat)),
            Some(Action::Typed('a'))
        );
    }

    #[test]
    fn other_modifiers_do_nothing() {
        assert_eq!(
            on_welcome(press(KeyCode::Char('q'), KeyModifiers::CONTROL)),
            None
        );
        assert_eq!(
            on_welcome(press(KeyCode::Char('a'), KeyModifiers::ALT)),
            None
        );
    }

    #[test]
    fn other_keys_and_events_do_nothing() {
        for code in [KeyCode::Esc, KeyCode::Up] {
            assert_eq!(on_welcome(press(code, KeyModifiers::NONE)), None);
        }
        assert_eq!(on_welcome(Event::FocusGained), None);
    }

    #[test]
    fn enter_chooses_on_area_selection() {
        assert_eq!(
            on_area_selection(press(KeyCode::Enter, KeyModifiers::NONE)),
            Some(Action::Choose)
        );
    }

    #[test]
    fn modified_or_held_enter_does_nothing_on_area_selection() {
        assert_eq!(
            on_area_selection(press(KeyCode::Enter, KeyModifiers::CONTROL)),
            None
        );
        assert_eq!(
            on_area_selection(press(KeyCode::Enter, KeyModifiers::ALT)),
            None
        );
        assert_eq!(
            on_area_selection(with_kind(KeyCode::Enter, KeyEventKind::Repeat)),
            None
        );
    }

    #[test]
    fn enter_goes_back_on_no_questions() {
        assert_eq!(
            on_no_questions(press(KeyCode::Enter, KeyModifiers::NONE)),
            Some(Action::Back)
        );
    }

    #[test]
    fn modified_or_held_enter_does_nothing_on_no_questions() {
        assert_eq!(
            on_no_questions(press(KeyCode::Enter, KeyModifiers::CONTROL)),
            None
        );
        assert_eq!(
            on_no_questions(press(KeyCode::Enter, KeyModifiers::ALT)),
            None
        );
        assert_eq!(
            on_no_questions(with_kind(KeyCode::Enter, KeyEventKind::Repeat)),
            None
        );
    }

    #[test]
    fn ctrl_c_quits_on_no_questions() {
        assert_eq!(
            on_no_questions(press(KeyCode::Char('c'), KeyModifiers::CONTROL)),
            Some(Action::Quit)
        );
    }

    #[test]
    fn resize_is_followed_on_no_questions() {
        assert_eq!(
            on_no_questions(Event::Resize(100, 30)),
            Some(Action::Resized {
                width: 100,
                height: 30
            })
        );
    }

    #[test]
    fn other_keys_do_nothing_on_no_questions() {
        for code in [
            KeyCode::Char('q'),
            KeyCode::Char('a'),
            KeyCode::Char('あ'),
            KeyCode::Up,
            KeyCode::Down,
        ] {
            assert_eq!(on_no_questions(press(code, KeyModifiers::NONE)), None);
        }
        assert_eq!(on_no_questions(Event::FocusGained), None);
    }

    #[test]
    fn enter_through_the_key_table_with_an_empty_area() {
        let mut session = Session::with_areas(80, 24, EMPTY_ONE);
        let feed = |session: &mut Session, event: Event| {
            if let Some(action) = action_for(session.screen(), &event) {
                session.apply(action);
            }
        };
        feed(&mut session, press(KeyCode::Enter, KeyModifiers::NONE));
        assert_eq!(session.screen(), Screen::AreaSelection);
        feed(&mut session, press(KeyCode::Enter, KeyModifiers::NONE));
        assert_eq!(session.screen(), Screen::NoQuestions);
        feed(&mut session, press(KeyCode::Enter, KeyModifiers::NONE));
        assert_eq!(session.screen(), Screen::AreaSelection);
        assert!(!session.should_quit());
    }

    #[test]
    fn arrows_move_on_area_selection() {
        assert_eq!(
            on_area_selection(press(KeyCode::Up, KeyModifiers::NONE)),
            Some(Action::Previous)
        );
        assert_eq!(
            on_area_selection(press(KeyCode::Down, KeyModifiers::NONE)),
            Some(Action::Next)
        );
    }

    #[test]
    fn held_arrows_keep_moving() {
        assert_eq!(
            on_area_selection(with_kind(KeyCode::Down, KeyEventKind::Repeat)),
            Some(Action::Next)
        );
        assert_eq!(
            on_area_selection(with_kind(KeyCode::Up, KeyEventKind::Repeat)),
            Some(Action::Previous)
        );
    }

    #[test]
    fn modified_arrows_do_nothing_on_area_selection() {
        for (code, modifiers) in [
            (KeyCode::Down, KeyModifiers::SHIFT),
            (KeyCode::Down, KeyModifiers::CONTROL),
            (KeyCode::Up, KeyModifiers::ALT),
        ] {
            assert_eq!(on_area_selection(press(code, modifiers)), None);
        }
    }

    #[test]
    fn unbound_keys_do_nothing_on_area_selection() {
        for code in [
            KeyCode::Char('j'),
            KeyCode::Char('k'),
            KeyCode::Char('1'),
            KeyCode::Left,
            KeyCode::Right,
            KeyCode::Tab,
        ] {
            assert_eq!(on_area_selection(press(code, KeyModifiers::NONE)), None);
        }
    }

    #[test]
    fn esc_quits_on_area_selection() {
        assert_eq!(
            on_area_selection(press(KeyCode::Esc, KeyModifiers::NONE)),
            Some(Action::Quit)
        );
    }

    #[test]
    fn esc_goes_back_on_no_questions() {
        assert_eq!(
            on_no_questions(press(KeyCode::Esc, KeyModifiers::NONE)),
            Some(Action::Back)
        );
    }

    #[test]
    fn modified_or_held_esc_does_nothing() {
        assert_eq!(
            on_area_selection(press(KeyCode::Esc, KeyModifiers::ALT)),
            None
        );
        assert_eq!(
            on_area_selection(with_kind(KeyCode::Esc, KeyEventKind::Repeat)),
            None
        );
        assert_eq!(
            on_no_questions(press(KeyCode::Esc, KeyModifiers::ALT)),
            None
        );
        assert_eq!(
            on_no_questions(with_kind(KeyCode::Esc, KeyEventKind::Repeat)),
            None
        );
    }

    #[test]
    fn esc_through_the_key_table() {
        let mut session = Session::with_areas(80, 24, EMPTY_ONE);
        let feed = |session: &mut Session, event: Event| {
            if let Some(action) = action_for(session.screen(), &event) {
                session.apply(action);
            }
        };
        feed(&mut session, press(KeyCode::Enter, KeyModifiers::NONE));
        feed(&mut session, press(KeyCode::Enter, KeyModifiers::NONE));
        assert_eq!(session.screen(), Screen::NoQuestions);
        feed(&mut session, press(KeyCode::Esc, KeyModifiers::NONE));
        assert_eq!(session.screen(), Screen::AreaSelection);
        assert!(!session.should_quit());
        feed(&mut session, press(KeyCode::Esc, KeyModifiers::NONE));
        assert!(session.should_quit());
    }

    #[test]
    fn arrows_move_on_the_level_picker() {
        assert_eq!(
            on_level_picker(press(KeyCode::Up, KeyModifiers::NONE)),
            Some(Action::Previous)
        );
        assert_eq!(
            on_level_picker(press(KeyCode::Down, KeyModifiers::NONE)),
            Some(Action::Next)
        );
        assert_eq!(
            on_level_picker(with_kind(KeyCode::Down, KeyEventKind::Repeat)),
            Some(Action::Next)
        );
    }

    #[test]
    fn esc_goes_back_on_the_level_picker() {
        assert_eq!(
            on_level_picker(press(KeyCode::Esc, KeyModifiers::NONE)),
            Some(Action::Back)
        );
        assert_eq!(
            on_level_picker(press(KeyCode::Esc, KeyModifiers::ALT)),
            None
        );
        assert_eq!(
            on_level_picker(with_kind(KeyCode::Esc, KeyEventKind::Repeat)),
            None
        );
    }

    #[test]
    fn ctrl_c_quits_on_the_level_picker() {
        assert_eq!(
            on_level_picker(press(KeyCode::Char('c'), KeyModifiers::CONTROL)),
            Some(Action::Quit)
        );
    }

    #[test]
    fn resize_is_followed_on_the_level_picker() {
        assert_eq!(
            on_level_picker(Event::Resize(100, 30)),
            Some(Action::Resized {
                width: 100,
                height: 30
            })
        );
    }

    #[test]
    fn unbound_keys_do_nothing_on_the_level_picker() {
        for code in [
            KeyCode::Char('q'),
            KeyCode::Char('a'),
            KeyCode::Char('x'),
            KeyCode::Left,
            KeyCode::Right,
            KeyCode::Tab,
        ] {
            assert_eq!(on_level_picker(press(code, KeyModifiers::NONE)), None);
        }
        assert_eq!(
            on_level_picker(press(KeyCode::Down, KeyModifiers::SHIFT)),
            None
        );
    }

    #[test]
    fn space_ticks_on_the_level_picker() {
        assert_eq!(
            on_level_picker(press(KeyCode::Char(' '), KeyModifiers::NONE)),
            Some(Action::Toggle)
        );
        assert_eq!(
            on_level_picker(with_kind(KeyCode::Char(' '), KeyEventKind::Repeat)),
            None
        );
        assert_eq!(
            on_level_picker(press(KeyCode::Char(' '), KeyModifiers::CONTROL)),
            None
        );
    }
}
