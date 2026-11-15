// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 PandaCubGames

//! Every piece of text the app shows to the user lives here, so wording is
//! reviewed and changed in one place. Other modules only lay it out.
//!
//! Fixed messages are constants. A message with values in it is a function
//! that returns the finished text.
//!
//! Text must not use characters of ambiguous width, such as the
//! multiplication sign or the em dash: terminals set up for CJK draw them
//! two columns wide, which breaks centring.

use std::fmt::Display;
use std::time::Duration;

/// The program's name, as shown in messages.
pub const APP_NAME: &str = env!("CARGO_PKG_NAME");

/// Welcome screen: the name and version.
pub const TITLE: &str = concat!(env!("CARGO_PKG_NAME"), " v", env!("CARGO_PKG_VERSION"));

/// Welcome screen: "Let's study Japanese!"
pub const GREETING: &str = "日本語を勉強しましょう！";

/// Welcome screen: how to go on.
pub const START_HINT: &str = "Press Enter to start";

/// Welcome screen: how to leave the app.
pub const QUIT_HINT: &str = "Press q or Ctrl+C to quit";

/// Area selection: the heading above the list.
pub const AREA_SELECTION_TITLE: &str = "What do you want to practise?";

/// Area selection: the keys.
pub const AREA_SELECTION_HINT: &str = "Up/Down to move, Enter to choose, Esc to leave";

/// Area selection: in front of the highlighted area.
pub const AREA_MARKER: &str = "> ";

/// Area selection: in front of every other area, as wide as `AREA_MARKER`.
pub const AREA_NO_MARKER: &str = "  ";

/// Area selection: shown in place of the list when the app has no areas.
pub const AREA_SELECTION_EMPTY: &str = "Nothing to practise yet";

/// Area names, as shown in the list.
pub const AREA_VOCABULARY: &str = "Vocabulary";
pub const AREA_KANJI: &str = "Kanji";
pub const AREA_GRAMMAR: &str = "Grammar";

/// Shown when the chosen area has no questions.
pub const NO_QUESTIONS: &str = "No questions in this area yet";

/// How to leave the "no questions" screen.
pub const NO_QUESTIONS_HINT: &str = "Press Enter to go back";

/// Welcome screen: shown in place of the last key before any key is typed.
pub const NO_KEY: &str = "-";

/// Welcome screen: the terminal size and the time since start, to a tenth
/// of a second.
pub fn status(width: u16, height: u16, elapsed: Duration) -> String {
    let tenths = elapsed.as_millis() / 100;
    format!(
        "Size: {width}x{height}   Elapsed: {}.{} s",
        tenths / 10,
        tenths % 10
    )
}

/// Welcome screen: the last character typed.
pub fn last_key(key: Option<char>) -> String {
    match key {
        Some(key) => format!("Last key: {key}"),
        None => format!("Last key: {NO_KEY}"),
    }
}

/// Printed to standard error when the app has to stop.
pub fn fatal_error(error: impl Display) -> String {
    format!("{APP_NAME}: {error}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn title_is_the_name_and_version() {
        assert_eq!(
            TITLE,
            format!("tui-learnjapanese v{}", env!("CARGO_PKG_VERSION"))
        );
    }

    #[test]
    fn status_shows_size_and_tenths_of_a_second() {
        assert_eq!(
            status(80, 24, Duration::ZERO),
            "Size: 80x24   Elapsed: 0.0 s"
        );
        assert_eq!(
            status(100, 30, Duration::from_millis(12_345)),
            "Size: 100x30   Elapsed: 12.3 s"
        );
    }

    #[test]
    fn last_key_shows_the_character_or_a_placeholder() {
        assert_eq!(last_key(None), "Last key: -");
        assert_eq!(last_key(Some('a')), "Last key: a");
        assert_eq!(last_key(Some('あ')), "Last key: あ");
    }

    #[test]
    fn fatal_error_names_the_app() {
        assert_eq!(fatal_error("no terminal"), "tui-learnjapanese: no terminal");
    }

    #[test]
    fn start_hint_is_plain_ascii_text() {
        assert_eq!(START_HINT, "Press Enter to start");
        assert!(START_HINT.is_ascii());
    }

    #[test]
    fn area_selection_messages_are_plain_ascii_text() {
        assert_eq!(AREA_SELECTION_TITLE, "What do you want to practise?");
        assert_eq!(
            AREA_SELECTION_HINT,
            "Up/Down to move, Enter to choose, Esc to leave"
        );
        assert!(AREA_SELECTION_TITLE.is_ascii());
        assert!(AREA_SELECTION_HINT.is_ascii());
    }

    #[test]
    fn fixed_messages_are_not_empty() {
        for message in [APP_NAME, TITLE, GREETING, QUIT_HINT, NO_KEY] {
            assert!(!message.is_empty());
        }
    }

    #[test]
    fn area_and_no_questions_wording_is_plain_ascii_text() {
        for (text, expected) in [
            (AREA_VOCABULARY, "Vocabulary"),
            (AREA_KANJI, "Kanji"),
            (AREA_GRAMMAR, "Grammar"),
            (NO_QUESTIONS, "No questions in this area yet"),
            (NO_QUESTIONS_HINT, "Press Enter to go back"),
        ] {
            assert_eq!(text, expected);
            assert!(text.is_ascii());
        }
    }

    #[test]
    fn area_markers_are_the_same_width() {
        assert_eq!(AREA_MARKER, "> ");
        assert_eq!(AREA_NO_MARKER, "  ");
        assert_eq!(AREA_MARKER.len(), AREA_NO_MARKER.len());
        assert!(AREA_MARKER.is_ascii());
        assert!(AREA_NO_MARKER.is_ascii());
    }

    #[test]
    fn empty_area_list_wording_is_plain_ascii_text() {
        assert_eq!(AREA_SELECTION_EMPTY, "Nothing to practise yet");
        assert!(AREA_SELECTION_EMPTY.is_ascii());
    }
}
