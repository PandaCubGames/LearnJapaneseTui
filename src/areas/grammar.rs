// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 PandaCubGames

//! The Grammar area: conjugation practice (issue #13). Each word is one entry;
//! its questions are its nine forms, and the answers come from `conjugation`.

use super::Area;
use crate::conjugation::{self, Form, Formality, Group};
use crate::practice::question::{Kind, Question};
use crate::practice::{Entry, Level, Levels};
use crate::text;

/// The forms asked of every word, in this order.
const ASKED: [(Form, Formality); 9] = [
    (Form::NonPast, Formality::Plain),
    (Form::Negative, Formality::Plain),
    (Form::Past, Formality::Plain),
    (Form::PastNegative, Formality::Plain),
    (Form::Te, Formality::Plain),
    (Form::NonPast, Formality::Polite),
    (Form::Negative, Formality::Polite),
    (Form::Past, Formality::Polite),
    (Form::PastNegative, Formality::Polite),
];

/// A word to conjugate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Word {
    /// As usually written; the same as `reading` for a word written in kana.
    written: &'static str,
    /// In hiragana.
    reading: &'static str,
    /// In plain English (ASCII), shown in the question.
    meaning: &'static str,
    group: Group,
    level: Level,
}

/// Builds a `Word`, so each word of `WORDS` fits on one line.
const fn word(
    written: &'static str,
    reading: &'static str,
    meaning: &'static str,
    group: Group,
    level: Level,
) -> Word {
    Word {
        written,
        reading,
        meaning,
        group,
        level,
    }
}

/// Every word, by group, in the order of design.md Appendix A.
const WORDS: &[Word] = &[
    word("食べる", "たべる", "to eat", Group::Ichidan, Level::N5),
    word("見る", "みる", "to see", Group::Ichidan, Level::N5),
    word("起きる", "おきる", "to get up", Group::Ichidan, Level::N5),
    word("寝る", "ねる", "to sleep", Group::Ichidan, Level::N5),
    word("教える", "おしえる", "to teach", Group::Ichidan, Level::N5),
    word("決める", "きめる", "to decide", Group::Ichidan, Level::N4),
    word(
        "調べる",
        "しらべる",
        "to look up",
        Group::Ichidan,
        Level::N4,
    ),
];

/// The Grammar area.
#[derive(Debug)]
pub struct Grammar;

impl Area for Grammar {
    fn name(&self) -> &'static str {
        text::AREA_GRAMMAR
    }

    fn questions(&self, levels: Levels) -> Vec<Entry> {
        WORDS
            .iter()
            .filter(|word| levels.contains(word.level))
            .map(entry)
            .collect()
    }
}

/// The entry of one word: its level and one question per asked form.
fn entry(word: &Word) -> Entry {
    Entry {
        levels: Levels::only(word.level),
        questions: ASKED
            .iter()
            .filter_map(|&(form, formality)| question(word, form, formality))
            .collect(),
    }
}

/// The question for one form of `word`; `None` when the rules give no answer.
fn question(word: &Word, form: Form, formality: Formality) -> Option<Question> {
    let answers = conjugation::conjugate(word.written, word.reading, word.group, form, formality);
    if answers.is_empty() {
        return None;
    }
    let mut accepted = Vec::new();
    for answer in answers {
        push_unique(&mut accepted, answer.written);
        push_unique(&mut accepted, answer.reading);
    }
    let mut shown = vec![word.written.to_string()];
    push_unique(&mut shown, word.reading.to_string());
    Some(Question {
        prompt: text::conjugation_prompt(word.written, word.reading, word.meaning, form, formality),
        accepted,
        shown,
        kind: Kind::Japanese,
    })
}

/// Appends `text` unless `list` already has it.
fn push_unique(list: &mut Vec<String>, text: String) {
    if !list.contains(&text) {
        list.push(text);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::areas;
    use crate::grading::{self, Outcome};
    use crate::kana;
    use crate::session::{Action, Screen, Session};
    use crate::ui;
    use ratatui::Terminal;
    use ratatui::backend::TestBackend;

    fn ask(written: &str, form: Form, formality: Formality) -> Question {
        let word = WORDS
            .iter()
            .find(|word| word.written == written)
            .unwrap_or_else(|| panic!("{written} is not in WORDS"));
        question(word, form, formality).unwrap()
    }

    fn strings(texts: &[&str]) -> Vec<String> {
        texts.iter().map(|text| text.to_string()).collect()
    }

    fn is_correct(question: &Question, typed: &str) -> bool {
        let accepted: Vec<&str> = question.accepted.iter().map(|s| s.as_str()).collect();
        let shown: Vec<&str> = question.shown.iter().map(|s| s.as_str()).collect();
        matches!(
            grading::check(typed, &accepted, &shown),
            Outcome::Correct { .. }
        )
    }

    #[test]
    fn grammar_is_named_and_listed() {
        assert_eq!(Grammar.name(), "Grammar");
        assert!(areas::all().iter().any(|area| area.name() == "Grammar"));
    }

    #[test]
    fn grammar_is_on_the_area_list_screen() {
        let mut session = Session::new(80, 24);
        session.apply(Action::Start);
        let mut terminal = Terminal::new(TestBackend::new(80, 24)).unwrap();
        terminal.draw(|frame| ui::draw(frame, &session)).unwrap();
        let buffer = terminal.backend().buffer().clone();
        let found = (0..24).any(|y| {
            let row: String = (0..80).map(|x| buffer[(x, y)].symbol()).collect();
            row.contains("Grammar")
        });
        assert!(found);
    }

    #[test]
    fn the_app_reaches_a_grammar_question() {
        let index = areas::all()
            .iter()
            .position(|area| area.name() == "Grammar")
            .unwrap();
        let mut session = Session::new(80, 24);
        session.apply(Action::Start);
        for _ in 0..index {
            session.apply(Action::Next);
        }
        session.apply(Action::Choose);
        assert_eq!(session.screen(), Screen::LevelPicker);
        assert_eq!(session.highlighted_area().unwrap().name(), "Grammar");
        session.apply(Action::Confirm);
        assert_eq!(session.screen(), Screen::Question);
        let prompt = &session.drill().unwrap().question().prompt;
        assert!(prompt.starts_with("What is the "), "{prompt}");
    }

    #[test]
    fn grammar_has_questions() {
        assert!(Grammar.has_questions());
    }

    #[test]
    fn every_word_gives_nine_questions() {
        let entries = Grammar.questions(Levels::ALL);
        assert_eq!(entries.len(), WORDS.len());
        for (entry, word) in entries.iter().zip(WORDS) {
            assert_eq!(entry.questions.len(), 9, "{}", word.written);
            let mut prompts: Vec<&str> =
                entry.questions.iter().map(|q| q.prompt.as_str()).collect();
            prompts.sort_unstable();
            prompts.dedup();
            assert_eq!(prompts.len(), 9, "{}", word.written);
            assert!(entry.questions.iter().all(|q| q.kind == Kind::Japanese));
            assert_eq!(entry.levels, Levels::only(word.level), "{}", word.written);
        }
    }

    #[test]
    fn entries_follow_the_chosen_levels() {
        for level in [Level::N4, Level::N5] {
            let entries = Grammar.questions(Levels::only(level));
            assert!(!entries.is_empty());
            assert!(entries.iter().all(|entry| entry.levels.contains(level)));
        }
        assert!(
            Grammar
                .questions(Levels::only(Level::N2).toggled(Level::N1))
                .is_empty()
        );
    }

    #[test]
    fn every_word_is_valid() {
        const BEFORE_RU: &str = "いきしちにひみりぎじぢびぴえけせてねへめれげぜでべぺ";
        for (i, word) in WORDS.iter().enumerate() {
            let name = word.written;
            assert!(!word.reading.is_empty(), "{name}");
            assert!(
                word.reading
                    .chars()
                    .all(|c| ('\u{3041}'..='\u{3096}').contains(&c)),
                "{name}"
            );
            assert!(!word.written.is_empty(), "{name}");
            if word.written != word.reading {
                assert!(word.written.chars().any(kana::is_kanji), "{name}");
            }
            assert!(
                !word.meaning.is_empty() && word.meaning.is_ascii(),
                "{name}"
            );
            assert!(
                WORDS[i + 1..].iter().all(|other| other.written != name),
                "{name} is listed twice"
            );
            if word.group == Group::Ichidan {
                let mut reversed = word.reading.chars().rev();
                assert_eq!(reversed.next(), Some('る'), "{name}");
                assert!(BEFORE_RU.contains(reversed.next().unwrap()), "{name}");
            }
        }
    }

    #[test]
    fn prompts_fit_80_columns() {
        for entry in Grammar.questions(Levels::ALL) {
            for question in entry.questions {
                let width = ratatui::text::Line::from(question.prompt.as_str()).width();
                assert!(width <= 80, "{} is {width} columns", question.prompt);
            }
        }
    }

    #[test]
    fn accepted_answers_are_japanese_without_duplicates() {
        for entry in Grammar.questions(Levels::ALL) {
            for question in entry.questions {
                let accepted = &question.accepted;
                for (i, text) in accepted.iter().enumerate() {
                    assert!(!text.is_empty(), "{}", question.prompt);
                    assert!(text.chars().all(kana::is_japanese), "{text}");
                    assert!(!accepted[i + 1..].contains(text), "{text} twice");
                }
                assert!(
                    accepted
                        .iter()
                        .any(|text| !text.chars().any(kana::is_kanji)),
                    "{}",
                    question.prompt
                );
            }
        }
    }

    #[test]
    fn polite_past_of_taberu() {
        let question = ask("食べる", Form::Past, Formality::Polite);
        assert_eq!(
            question.prompt,
            "What is the polite past of 食べる (たべる, to eat)?"
        );
        assert_eq!(question.accepted, strings(&["食べました", "たべました"]));
        assert_eq!(question.shown, strings(&["食べる", "たべる"]));
    }

    #[test]
    fn polite_past_negative_prompt() {
        assert_eq!(
            ask("食べる", Form::PastNegative, Formality::Polite).prompt,
            "What is the polite past negative of 食べる (たべる, to eat)?"
        );
    }

    #[test]
    fn taberu_answers_typed_every_way() {
        let question = ask("食べる", Form::Past, Formality::Polite);
        for typed in [
            "tabemashita",
            "tabemasita",
            "たべました",
            "タベマシタ",
            "食べました",
        ] {
            assert!(is_correct(&question, typed), "{typed}");
        }
        assert!(!is_correct(&question, "tabemasu"));
    }

    #[test]
    fn copying_the_shown_word_is_wrong() {
        let question = ask("食べる", Form::NonPast, Formality::Plain);
        assert!(!is_correct(&question, "食べる"));
        assert!(is_correct(&question, "たべる"));
        assert!(is_correct(&question, "taberu"));
    }
}
