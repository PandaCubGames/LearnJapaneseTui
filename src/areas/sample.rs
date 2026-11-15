// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 PandaCubGames

//! A test-only area with fixed questions at N5, N4 and N3. It proves that an area which only implements the contract can be practised (issue #21).

use super::Area;
use crate::practice::question::{Kind, Question};
use crate::practice::{Entry, Level, Levels};

/// The Sample area: all of `entries()`, filtered by level.
#[derive(Debug)]
pub struct Sample;

/// One question of the Sample area on its own, for tests that need a known question.
#[derive(Debug)]
pub struct Only {
    pub entry: usize,
    pub question: usize,
}

/// How do you read 高い?
pub const READING: Only = Only {
    entry: 0,
    question: 0,
};

/// What does 高い mean?
pub const MEANING: Only = Only {
    entry: 0,
    question: 1,
};

/// The polite past of 食べる.
pub const POLITE_PAST: Only = Only {
    entry: 1,
    question: 0,
};

fn question(kind: Kind, prompt: &str, accepted: &[&str], shown: &[&str]) -> Question {
    Question {
        prompt: prompt.to_string(),
        accepted: accepted.iter().map(|text| text.to_string()).collect(),
        shown: shown.iter().map(|text| text.to_string()).collect(),
        kind,
    }
}

/// The Sample area's entries, in order.
pub fn entries() -> Vec<Entry> {
    vec![
        Entry {
            levels: Levels::only(Level::N5),
            questions: vec![
                question(
                    Kind::Japanese,
                    "How do you read 高い?",
                    &["たかい"],
                    &["高い"],
                ),
                question(
                    Kind::Meaning,
                    "What does 高い mean?",
                    &["high", "tall", "expensive"],
                    &[],
                ),
            ],
        },
        Entry {
            levels: Levels::only(Level::N5).toggled(Level::N4),
            questions: vec![question(
                Kind::Japanese,
                "What is the polite past of 食べる?",
                &["食べました", "たべました"],
                &["食べる", "たべる"],
            )],
        },
        Entry {
            levels: Levels::only(Level::N3),
            questions: vec![question(
                Kind::Japanese,
                "How do you read 山?",
                &["やま"],
                &["山"],
            )],
        },
    ]
}

impl Area for Sample {
    fn name(&self) -> &'static str {
        "Sample"
    }

    fn questions(&self, levels: Levels) -> Vec<Entry> {
        entries()
            .into_iter()
            .filter(|entry| entry.levels.intersects(levels))
            .collect()
    }
}

impl Area for Only {
    fn name(&self) -> &'static str {
        "Sample"
    }

    fn questions(&self, levels: Levels) -> Vec<Entry> {
        let Some(mut entry) = entries().into_iter().nth(self.entry) else {
            return Vec::new();
        };
        if !entry.levels.intersects(levels) || self.question >= entry.questions.len() {
            return Vec::new();
        }
        entry.questions = vec![entry.questions.swap_remove(self.question)];
        vec![entry]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn strings(texts: &[&str]) -> Vec<String> {
        texts.iter().map(|text| text.to_string()).collect()
    }

    #[test]
    fn sample_has_questions() {
        assert!(Sample.has_questions());
        assert_eq!(Sample.name(), "Sample");
    }

    #[test]
    fn n3_gives_only_the_mountain() {
        let found = Sample.questions(Levels::only(Level::N3));
        assert_eq!(found.len(), 1);
        assert_eq!(found[0].questions.len(), 1);
        assert_eq!(found[0].questions[0].prompt, "How do you read 山?");
    }

    #[test]
    fn entries_carry_a_chosen_level() {
        let chosen = Levels::only(Level::N5).toggled(Level::N4);
        let found = Sample.questions(chosen);
        assert_eq!(found.len(), 2);
        assert!(found.iter().all(|entry| entry.levels.intersects(chosen)));
    }

    #[test]
    fn n2_and_n1_give_nothing() {
        let chosen = Levels::only(Level::N2).toggled(Level::N1);
        assert!(Sample.questions(chosen).is_empty());
    }

    #[test]
    fn reading_question() {
        let q = &entries()[0].questions[0];
        assert_eq!(q.prompt, "How do you read 高い?");
        assert_eq!(q.accepted, strings(&["たかい"]));
        assert_eq!(q.shown, strings(&["高い"]));
        assert_eq!(q.kind, Kind::Japanese);
    }

    #[test]
    fn meaning_question() {
        let q = &entries()[0].questions[1];
        assert_eq!(q.prompt, "What does 高い mean?");
        assert_eq!(q.accepted, strings(&["high", "tall", "expensive"]));
        assert!(q.shown.is_empty());
        assert_eq!(q.kind, Kind::Meaning);
    }

    #[test]
    fn polite_past_question() {
        let entry = &entries()[1];
        assert!(entry.levels.contains(Level::N5));
        assert!(entry.levels.contains(Level::N4));
        let q = &entry.questions[0];
        assert_eq!(q.prompt, "What is the polite past of 食べる?");
        assert_eq!(q.accepted, strings(&["食べました", "たべました"]));
        assert_eq!(q.shown, strings(&["食べる", "たべる"]));
        assert_eq!(q.kind, Kind::Japanese);
    }

    #[test]
    fn only_gives_one_question() {
        let found = READING.questions(Levels::ALL);
        assert_eq!(found.len(), 1);
        assert_eq!(found[0].questions, vec![entries()[0].questions[0].clone()]);
        let found = MEANING.questions(Levels::ALL);
        assert_eq!(found.len(), 1);
        assert_eq!(found[0].questions, vec![entries()[0].questions[1].clone()]);
        let found = POLITE_PAST.questions(Levels::ALL);
        assert_eq!(found[0].questions, vec![entries()[1].questions[0].clone()]);
    }

    #[test]
    fn only_respects_levels() {
        assert!(READING.questions(Levels::only(Level::N3)).is_empty());
    }
}
