// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 PandaCubGames

//! The generic practice module: what every area has in common. It never sees an area's own words, kanji or verbs, only questions (issue #21).

mod level;
mod pool;
pub(crate) mod question;
mod rng;

use crate::answer_field::{AnswerField, Edit};
use pool::Pool;

pub use level::{Level, Levels};
pub use question::{Entry, Question};
pub use rng::Rng;

/// The entries practised at `levels`: those with any of `levels` and at least one
/// question, in the order given.
pub fn selection(entries: Vec<Entry>, levels: Levels) -> Vec<Entry> {
    entries
        .into_iter()
        .filter(|entry| entry.levels.intersects(levels) && !entry.questions.is_empty())
        .collect()
}

/// One run of practice: the entries being asked and the question on screen.
#[derive(Debug)]
pub struct Drill {
    entries: Vec<Entry>,
    entry: usize,
    question: usize,
    field: AnswerField,
}

impl Drill {
    /// Starts practice on the entries of `entries` that have a chosen level and a
    /// question; `None` when there are none.
    pub fn start(entries: Vec<Entry>, levels: Levels, rng: &mut Rng) -> Option<Drill> {
        let entries = selection(entries, levels);
        let mut pool = Pool::new(entries.len());
        let entry = pool.next(rng)?;
        let question = rng.below(entries[entry].questions.len());
        Some(Drill {
            entries,
            entry,
            question,
            field: AnswerField::default(),
        })
    }

    /// The question on screen.
    pub fn question(&self) -> &Question {
        &self.entries[self.entry].questions[self.question]
    }

    /// The answer being typed.
    pub fn field(&self) -> &AnswerField {
        &self.field
    }

    /// Edits the answer.
    pub fn edit(&mut self, edit: Edit) {
        self.field.apply(edit);
    }
}

#[cfg(test)]
mod tests {
    use super::question::Kind;
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

    #[test]
    fn start_with_nothing_practised_gives_none() {
        let mut rng = Rng::new(0);
        assert!(Drill::start(Vec::new(), Levels::ALL, &mut rng).is_none());
        let n3 = vec![entry(Levels::only(Level::N3), &["x"])];
        assert!(Drill::start(n3, Levels::only(Level::N5), &mut rng).is_none());
    }

    #[test]
    fn start_asks_a_practised_entry() {
        for seed in 0..20 {
            let entries = vec![
                entry(Levels::only(Level::N5), &["a"]),
                entry(Levels::only(Level::N1), &["b"]),
            ];
            let drill = Drill::start(entries, Levels::only(Level::N5), &mut Rng::new(seed));
            assert_eq!(drill.unwrap().question().prompt, "a");
        }
    }

    #[test]
    fn start_skips_entries_without_questions() {
        for seed in 0..20 {
            let entries = vec![
                entry(Levels::only(Level::N5), &[]),
                entry(Levels::only(Level::N5), &["kept"]),
            ];
            let drill = Drill::start(entries, Levels::only(Level::N5), &mut Rng::new(seed));
            assert_eq!(drill.unwrap().question().prompt, "kept");
        }
    }

    #[test]
    fn edit_types_into_the_field() {
        let mut drill = Drill::start(
            vec![entry(Levels::only(Level::N5), &["a"])],
            Levels::only(Level::N5),
            &mut Rng::new(0),
        )
        .unwrap();
        for c in ['t', 'a', 'k', 'a', 'x'] {
            drill.edit(Edit::Insert(c));
        }
        drill.edit(Edit::DeleteBack);
        drill.edit(Edit::Insert('i'));
        assert_eq!(drill.field().text(), "takai");
    }
}
