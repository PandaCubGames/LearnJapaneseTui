// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 PandaCubGames

//! The generic practice module: what every area has in common. It never sees an area's own words, kanji or verbs, only questions (issue #21).

mod level;
pub(crate) mod question;

pub use level::{Level, Levels};
pub use question::Entry;

/// The entries practised at `levels`: those with any of `levels` and at least one
/// question, in the order given.
pub fn selection(entries: Vec<Entry>, levels: Levels) -> Vec<Entry> {
    entries
        .into_iter()
        .filter(|entry| entry.levels.intersects(levels) && !entry.questions.is_empty())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::question::{Kind, Question};
    use super::*;

    fn entry(levels: Levels, prompts: &[&str]) -> Entry {
        Entry {
            levels,
            questions: prompts
                .iter()
                .map(|prompt| Question {
                    prompt: prompt.to_string(),
                    accepted: vec!["あ".to_string()],
                    shown: Vec::new(),
                    kind: Kind::Japanese,
                })
                .collect(),
        }
    }

    fn prompts(entries: &[Entry]) -> Vec<&str> {
        entries
            .iter()
            .map(|entry| entry.questions[0].prompt.as_str())
            .collect()
    }

    #[test]
    fn selection_keeps_entries_with_any_chosen_level() {
        let n5_n4 = Levels::only(Level::N5).toggled(Level::N4);
        let entries = vec![
            entry(Levels::only(Level::N5), &["n5"]),
            entry(n5_n4, &["n5 n4"]),
            entry(Levels::only(Level::N1), &["n1"]),
        ];
        let chosen = Levels::only(Level::N4).toggled(Level::N3);
        assert_eq!(prompts(&selection(entries, chosen)), ["n5 n4"]);
    }

    #[test]
    fn selection_drops_entries_without_questions() {
        let entries = vec![
            entry(Levels::only(Level::N5), &[]),
            entry(Levels::only(Level::N5), &["kept"]),
        ];
        assert_eq!(
            prompts(&selection(entries, Levels::only(Level::N5))),
            ["kept"]
        );
    }

    #[test]
    fn selection_keeps_the_order() {
        let entries = vec![
            entry(Levels::only(Level::N5), &["c"]),
            entry(Levels::only(Level::N5), &["a"]),
            entry(Levels::only(Level::N5), &["b"]),
        ];
        assert_eq!(
            prompts(&selection(entries, Levels::only(Level::N5))),
            ["c", "a", "b"]
        );
    }
}
