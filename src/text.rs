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

/// Area selection, until the list of areas exists.
pub const AREA_SELECTION_SOON: &str = "Area selection is coming soon.";

/// Area selection: how to leave the app. Only Ctrl+C works there for now.
pub const AREA_SELECTION_QUIT_HINT: &str = "Press Ctrl+C to quit";

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
        assert_eq!(AREA_SELECTION_SOON, "Area selection is coming soon.");
        assert_eq!(AREA_SELECTION_QUIT_HINT, "Press Ctrl+C to quit");
        assert!(AREA_SELECTION_SOON.is_ascii());
        assert!(AREA_SELECTION_QUIT_HINT.is_ascii());
    }

    #[test]
    fn fixed_messages_are_not_empty() {
        for message in [APP_NAME, TITLE, GREETING, QUIT_HINT, NO_KEY] {
            assert!(!message.is_empty());
        }
    }
}
