// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 PandaCubGames

//! Checking English answers: plain text, never converted to kana.

/// The result of checking an English answer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Outcome {
    /// Nothing was typed.
    Empty,
    /// The answer matches an accepted meaning.
    Correct,
    /// The answer matches none of them.
    Wrong,
}

/// Checks an English answer against the accepted meanings.
pub fn check(typed: &str, accepted: &[&str]) -> Outcome {
    let typed = normalise(typed);
    if typed.is_empty() {
        Outcome::Empty
    } else if accepted.iter().any(|meaning| normalise(meaning) == typed) {
        Outcome::Correct
    } else {
        Outcome::Wrong
    }
}

/// Lower case, with no spaces at both ends and one space between words.
fn normalise(text: &str) -> String {
    text.to_lowercase()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same_meaning_is_correct() {
        assert_eq!(check("tall", &["tall"]), Outcome::Correct);
    }

    #[test]
    fn case_and_spaces_are_ignored() {
        assert_eq!(check("  To   EAT ", &["to eat"]), Outcome::Correct);
    }

    #[test]
    fn other_meanings_are_wrong() {
        assert_eq!(
            check("cheap", &["high", "tall", "expensive"]),
            Outcome::Wrong
        );
    }

    #[test]
    fn any_accepted_meaning_counts() {
        assert_eq!(
            check("tall", &["high", "tall", "expensive"]),
            Outcome::Correct
        );
    }

    #[test]
    fn empty_answers_are_not_checked() {
        for typed in ["", "   ", "\t"] {
            assert_eq!(check(typed, &["tall"]), Outcome::Empty);
        }
    }

    #[test]
    fn romaji_is_not_converted() {
        assert_eq!(check("takai", &["tall"]), Outcome::Wrong);
    }

    #[test]
    fn no_typo_for_unreadable_letters() {
        assert_eq!(check("xqzt", &["tall"]), Outcome::Wrong);
    }
}
