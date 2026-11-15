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

use crate::practice::Level;

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

/// Area names, as shown in the list.
pub const AREA_VOCABULARY: &str = "Vocabulary";
pub const AREA_KANJI: &str = "Kanji";
pub const AREA_GRAMMAR: &str = "Grammar";

/// Shown when the chosen area has no questions.
pub const NO_QUESTIONS: &str = "No questions in this area yet";

/// How to leave the "no questions" screen.
pub const NO_QUESTIONS_HINT: &str = "Press Enter to go back";

/// Level picker: the heading above the levels.
pub const LEVEL_PICKER_TITLE: &str = "Which JLPT levels do you want to practise?";

/// Level picker: the keys.
pub const LEVEL_PICKER_HINT: &str =
    "Up/Down to move, Space to tick, Enter to start, Esc to go back";

/// Level picker: in front of a ticked level.
pub const LEVEL_TICKED: &str = "[x] ";

/// Level picker: in front of an unticked level, as wide as `LEVEL_TICKED`.
pub const LEVEL_UNTICKED: &str = "[ ] ";

/// Level picker: shown while no level is ticked.
pub const NO_LEVEL_TICKED: &str = "Tick at least one level";

/// "No questions at these levels" screen: the message.
pub const NO_QUESTIONS_AT_LEVELS: &str = "No questions at these levels yet";

/// "No questions at these levels" screen: how to go on.
pub const NO_QUESTIONS_AT_LEVELS_HINT: &str = "Press Enter to choose other levels";

/// Question screen: how to answer a Japanese question.
pub const ANSWER_IN_JAPANESE: &str = "Answer in Japanese or romaji";

/// Question screen: how to answer a meaning question.
pub const ANSWER_IN_ENGLISH: &str = "Answer in English";

/// Question screen: the keys.
pub const QUESTION_HINT: &str = "Enter to answer, Esc to stop";

/// Level picker: the name of a level.
pub fn level_name(level: Level) -> &'static str {
    match level {
        Level::N5 => "N5",
        Level::N4 => "N4",
        Level::N3 => "N3",
        Level::N2 => "N2",
        Level::N1 => "N1",
    }
}

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
    fn level_picker_wording_is_plain_ascii_text() {
        for (text, expected) in [
            (
                LEVEL_PICKER_TITLE,
                "Which JLPT levels do you want to practise?",
            ),
            (
                LEVEL_PICKER_HINT,
                "Up/Down to move, Space to tick, Enter to start, Esc to go back",
            ),
            (LEVEL_TICKED, "[x] "),
            (LEVEL_UNTICKED, "[ ] "),
        ] {
            assert_eq!(text, expected);
            assert!(text.is_ascii());
        }
    }

    #[test]
    fn level_names() {
        let names: Vec<_> = Level::ALL.iter().map(|&level| level_name(level)).collect();
        assert_eq!(names, ["N5", "N4", "N3", "N2", "N1"]);
    }

    #[test]
    fn tick_boxes_are_the_same_width() {
        assert_eq!(LEVEL_TICKED.len(), 4);
        assert_eq!(LEVEL_UNTICKED.len(), 4);
        assert!(LEVEL_TICKED.starts_with('['));
        assert!(LEVEL_UNTICKED.starts_with('['));
    }

    #[test]
    fn no_level_ticked_is_plain_ascii_text() {
        assert_eq!(NO_LEVEL_TICKED, "Tick at least one level");
        assert!(NO_LEVEL_TICKED.is_ascii());
    }

    #[test]
    fn no_questions_at_levels_wording_is_plain_ascii_text() {
        for (text, expected) in [
            (NO_QUESTIONS_AT_LEVELS, "No questions at these levels yet"),
            (
                NO_QUESTIONS_AT_LEVELS_HINT,
                "Press Enter to choose other levels",
            ),
        ] {
            assert_eq!(text, expected);
            assert!(text.is_ascii());
        }
    }

    #[test]
    fn question_wording_is_plain_ascii_text() {
        for (text, expected) in [
            (ANSWER_IN_JAPANESE, "Answer in Japanese or romaji"),
            (ANSWER_IN_ENGLISH, "Answer in English"),
            (QUESTION_HINT, "Enter to answer, Esc to stop"),
        ] {
            assert_eq!(text, expected);
            assert!(text.is_ascii());
        }
    }
}
