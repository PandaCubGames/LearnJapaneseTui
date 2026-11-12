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
