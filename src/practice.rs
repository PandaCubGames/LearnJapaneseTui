// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 PandaCubGames

//! The generic practice module: what every area has in common. It never sees an area's own words, kanji or verbs, only questions (issue #21).

mod level;
mod meaning;
mod pool;
pub(crate) mod question;
mod rng;

use crate::answer_field::{AnswerField, Edit};
use crate::grading::{Outcome, TypoGuard};
use pool::Pool;
use question::Kind;

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
    pool: Pool,
    field: AnswerField,
    guard: TypoGuard,
    typo_warning: bool,
    feedback: Option<Feedback>,
}

/// What the user is told after submitting an answer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Feedback {
    pub correct: bool,
    /// The answer as typed, without spaces at both ends.
    pub answer: String,
    /// The reading, when the answer was read as Japanese and it differs from `answer`.
    pub read_as: Option<String>,
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
            pool,
            field: AnswerField::default(),
            guard: TypoGuard::default(),
            typo_warning: false,
            feedback: None,
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

    /// Edits the answer. Does nothing while the feedback is shown.
    pub fn edit(&mut self, edit: Edit) {
        if self.feedback.is_none() {
            self.field.apply(edit);
            self.typo_warning = false;
        }
    }

    /// Checks the answer. Returns `true` when it was checked and the feedback is
    /// now shown; `false` when nothing was typed, when the text looks like a typo
    /// (the warning is then shown), or when the feedback is already shown.
    pub fn submit(&mut self) -> bool {
        if self.feedback.is_some() {
            return false;
        }
        let typed = self.field.text().to_string();
        let question = &self.entries[self.entry].questions[self.question];
        let accepted: Vec<&str> = question.accepted.iter().map(String::as_str).collect();
        let (correct, read_as) = match question.kind {
            Kind::Japanese => {
                let shown: Vec<&str> = question.shown.iter().map(String::as_str).collect();
                match self.guard.submit(&typed, &accepted, &shown) {
                    Outcome::Empty => return false,
                    Outcome::LooksLikeTypo => {
                        self.typo_warning = true;
                        return false;
                    }
                    Outcome::Correct { read_as } => (true, Some(read_as)),
                    Outcome::Wrong { read_as } => (false, read_as),
                }
            }
            Kind::Meaning => match meaning::check(&typed, &accepted) {
                meaning::Outcome::Empty => return false,
                meaning::Outcome::Correct => (true, None),
                meaning::Outcome::Wrong => (false, None),
            },
        };
        let answer = typed.trim().to_string();
        let read_as = read_as.filter(|reading| *reading != answer);
        self.feedback = Some(Feedback {
            correct,
            answer,
            read_as,
        });
        true
    }

    /// Moves on to a new question, with an empty field.
    pub fn next(&mut self, rng: &mut Rng) {
        if let Some(entry) = self.pool.next(rng) {
            self.entry = entry;
            self.question = rng.below(self.entries[entry].questions.len());
        }
        self.field = AnswerField::default();
        self.guard = TypoGuard::default();
        self.typo_warning = false;
        self.feedback = None;
    }

    /// Whether the "this does not read as Japanese" warning is shown.
    pub fn typo_warning(&self) -> bool {
        self.typo_warning
    }

    /// The feedback, once the answer was checked.
    pub fn feedback(&self) -> Option<&Feedback> {
        self.feedback.as_ref()
    }
}

#[cfg(test)]
mod tests {
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

    fn question(kind: Kind, prompt: &str, accepted: &[&str], shown: &[&str]) -> Question {
        Question {
            prompt: prompt.to_string(),
            accepted: accepted.iter().map(|text| text.to_string()).collect(),
            shown: shown.iter().map(|text| text.to_string()).collect(),
            kind,
        }
    }

    fn reading() -> Question {
        question(
            Kind::Japanese,
            "How do you read 高い?",
            &["たかい"],
            &["高い"],
        )
    }

    fn drill_of(question: Question) -> Drill {
        let entry = Entry {
            levels: Levels::only(Level::N5),
            questions: vec![question],
        };
        Drill::start(vec![entry], Levels::only(Level::N5), &mut Rng::new(0)).unwrap()
    }

    fn type_text(drill: &mut Drill, text: &str) {
        for c in text.chars() {
            drill.edit(Edit::Insert(c));
        }
    }

    fn feedback_of(drill: &Drill) -> Feedback {
        drill.feedback().unwrap().clone()
    }

    #[test]
    fn correct_romaji_gives_feedback() {
        let mut drill = drill_of(reading());
        type_text(&mut drill, "takai");
        assert!(drill.submit());
        assert_eq!(
            feedback_of(&drill),
            Feedback {
                correct: true,
                answer: "takai".to_string(),
                read_as: Some("たかい".to_string()),
            }
        );
    }

    #[test]
    fn correct_kana_has_no_reading() {
        let mut drill = drill_of(reading());
        type_text(&mut drill, "たかい");
        assert!(drill.submit());
        let feedback = feedback_of(&drill);
        assert!(feedback.correct);
        assert_eq!(feedback.read_as, None);
    }

    #[test]
    fn wrong_answer_keeps_its_reading() {
        let mut drill = drill_of(reading());
        type_text(&mut drill, "neko");
        assert!(drill.submit());
        let feedback = feedback_of(&drill);
        assert!(!feedback.correct);
        assert_eq!(feedback.read_as, Some("ねこ".to_string()));

        let mut drill = drill_of(reading());
        type_text(&mut drill, "ねこ");
        assert!(drill.submit());
        assert_eq!(feedback_of(&drill).read_as, None);
    }

    #[test]
    fn shown_kanji_is_not_accepted() {
        let kanji = || {
            question(
                Kind::Japanese,
                "How do you read 高い?",
                &["高い", "たかい"],
                &["高い"],
            )
        };
        let mut drill = drill_of(kanji());
        type_text(&mut drill, "高い");
        assert!(drill.submit());
        assert!(!feedback_of(&drill).correct);

        let mut drill = drill_of(kanji());
        type_text(&mut drill, "takai");
        assert!(drill.submit());
        assert!(feedback_of(&drill).correct);
    }

    #[test]
    fn shown_kana_is_accepted() {
        let plain = || {
            question(
                Kind::Japanese,
                "What is the plain non-past of する?",
                &["する"],
                &["する"],
            )
        };
        let mut drill = drill_of(plain());
        type_text(&mut drill, "する");
        assert!(drill.submit());
        assert!(feedback_of(&drill).correct);

        let mut drill = drill_of(plain());
        type_text(&mut drill, "suru");
        assert!(drill.submit());
        let feedback = feedback_of(&drill);
        assert!(feedback.correct);
        assert_eq!(feedback.read_as, Some("する".to_string()));
    }

    #[test]
    fn empty_submit_is_ignored() {
        let mut drill = drill_of(reading());
        assert!(!drill.submit());
        type_text(&mut drill, "  ");
        assert!(!drill.submit());
        assert!(drill.feedback().is_none());
    }

    #[test]
    fn typo_warns_first_then_counts() {
        let mut drill = drill_of(reading());
        type_text(&mut drill, "tabxru");
        assert!(!drill.submit());
        assert!(drill.typo_warning());
        assert_eq!(drill.field().text(), "tabxru");
        assert!(drill.submit());
        assert!(!feedback_of(&drill).correct);
    }

    #[test]
    fn editing_clears_the_typo_warning() {
        let mut drill = drill_of(reading());
        type_text(&mut drill, "tabxru");
        assert!(!drill.submit());
        drill.edit(Edit::DeleteBack);
        assert!(!drill.typo_warning());
    }

    #[test]
    fn meaning_answers_are_checked_as_english() {
        let tall = || question(Kind::Meaning, "What does 高い mean?", &["tall"], &[]);
        let mut drill = drill_of(tall());
        type_text(&mut drill, "Tall");
        assert!(drill.submit());
        let feedback = feedback_of(&drill);
        assert!(feedback.correct);
        assert_eq!(feedback.read_as, None);

        let mut drill = drill_of(tall());
        type_text(&mut drill, "takai");
        assert!(drill.submit());
        assert!(!feedback_of(&drill).correct);
    }

    #[test]
    fn no_edit_or_submit_during_feedback() {
        let mut drill = drill_of(reading());
        type_text(&mut drill, "takai");
        assert!(drill.submit());
        let before = feedback_of(&drill);
        drill.edit(Edit::Insert('x'));
        assert_eq!(drill.field().text(), "takai");
        assert!(!drill.submit());
        assert_eq!(feedback_of(&drill), before);
    }

    #[test]
    fn next_starts_a_fresh_question() {
        let mut drill = drill_of(reading());
        type_text(&mut drill, "takai");
        assert!(drill.submit());
        drill.next(&mut Rng::new(1));
        assert!(drill.feedback().is_none());
        assert_eq!(drill.field().text(), "");
        assert!(!drill.typo_warning());
    }

    #[test]
    fn next_follows_the_pool() {
        for seed in 0..10 {
            let entries = ["a", "b", "c"]
                .iter()
                .map(|prompt| entry(Levels::only(Level::N5), &[prompt]))
                .collect();
            let mut rng = Rng::new(seed);
            let mut drill = Drill::start(entries, Levels::only(Level::N5), &mut rng).unwrap();
            let mut prompts = vec![drill.question().prompt.clone()];
            for _ in 0..5 {
                drill.next(&mut rng);
                prompts.push(drill.question().prompt.clone());
            }
            for round in prompts.chunks(3) {
                let mut sorted = round.to_vec();
                sorted.sort();
                assert_eq!(sorted, ["a", "b", "c"], "{prompts:?}");
            }
        }
    }

    #[test]
    fn both_questions_of_an_entry_are_asked() {
        let entries = vec![entry(Levels::only(Level::N5), &["one", "two"])];
        let mut rng = Rng::new(0);
        let mut drill = Drill::start(entries, Levels::only(Level::N5), &mut rng).unwrap();
        let mut seen = vec![drill.question().prompt.clone()];
        for _ in 0..40 {
            drill.next(&mut rng);
            seen.push(drill.question().prompt.clone());
        }
        assert!(seen.iter().any(|prompt| prompt == "one"));
        assert!(seen.iter().any(|prompt| prompt == "two"));
    }
}
