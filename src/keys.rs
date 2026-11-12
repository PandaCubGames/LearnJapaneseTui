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
            KeyCode::Char('q') if key.modifiers.is_empty() => Some(Action::Quit),
            KeyCode::Char(c) if (key.modifiers - KeyModifiers::SHIFT).is_empty() => {
                Some(Action::Typed(c))
            }
            _ => None,
        },
    }
}

#[cfg(test)]
mod tests {
    use ratatui::crossterm::event::KeyEvent;

    use super::*;

    fn press(code: KeyCode, modifiers: KeyModifiers) -> Event {
        Event::Key(KeyEvent::new(code, modifiers))
    }

    fn with_kind(code: KeyCode, kind: KeyEventKind) -> Event {
        Event::Key(KeyEvent::new_with_kind(code, KeyModifiers::NONE, kind))
    }

    fn on_welcome(event: Event) -> Option<Action> {
        action_for(Screen::Welcome, &event)
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
        for code in [KeyCode::Enter, KeyCode::Esc, KeyCode::Up] {
            assert_eq!(on_welcome(press(code, KeyModifiers::NONE)), None);
        }
        assert_eq!(on_welcome(Event::FocusGained), None);
    }
}
