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
    word("買う", "かう", "to buy", Group::Godan, Level::N5),
    word("会う", "あう", "to meet", Group::Godan, Level::N5),
    word("手伝う", "てつだう", "to help", Group::Godan, Level::N4),
    word("書く", "かく", "to write", Group::Godan, Level::N5),
    word("聞く", "きく", "to listen", Group::Godan, Level::N5),
    word("働く", "はたらく", "to work", Group::Godan, Level::N4),
    word("泳ぐ", "およぐ", "to swim", Group::Godan, Level::N5),
    word("急ぐ", "いそぐ", "to hurry", Group::Godan, Level::N4),
    word("話す", "はなす", "to speak", Group::Godan, Level::N5),
    word("貸す", "かす", "to lend", Group::Godan, Level::N5),
    word("待つ", "まつ", "to wait", Group::Godan, Level::N5),
    word("持つ", "もつ", "to hold", Group::Godan, Level::N5),
    word("死ぬ", "しぬ", "to die", Group::Godan, Level::N5),
    word("遊ぶ", "あそぶ", "to play", Group::Godan, Level::N5),
    word("運ぶ", "はこぶ", "to carry", Group::Godan, Level::N4),
    word("飲む", "のむ", "to drink", Group::Godan, Level::N5),
    word("読む", "よむ", "to read", Group::Godan, Level::N5),
    word("住む", "すむ", "to live", Group::Godan, Level::N4),
    word("帰る", "かえる", "to go home", Group::Godan, Level::N5),
    word("分かる", "わかる", "to understand", Group::Godan, Level::N5),
    word("送る", "おくる", "to send", Group::Godan, Level::N4),
    word("する", "する", "to do", Group::Suru, Level::N5),
    word(
        "勉強する",
        "べんきょうする",
        "to study",
        Group::Suru,
        Level::N5,
    ),
    word(
        "運転する",
        "うんてんする",
        "to drive",
        Group::Suru,
        Level::N4,
    ),
    word("来る", "くる", "to come", Group::Kuru, Level::N5),
    word("行く", "いく", "to go", Group::Iku, Level::N5),
    word("ある", "ある", "to exist", Group::Aru, Level::N5),
    word("高い", "たかい", "expensive", Group::IAdjective, Level::N5),
    word("安い", "やすい", "cheap", Group::IAdjective, Level::N5),
    word("大きい", "おおきい", "big", Group::IAdjective, Level::N5),
    word("新しい", "あたらしい", "new", Group::IAdjective, Level::N5),
    word("寒い", "さむい", "cold", Group::IAdjective, Level::N5),
    word("いい", "いい", "good", Group::Ii, Level::N5),
    word("優しい", "やさしい", "kind", Group::IAdjective, Level::N4),
    word("悲しい", "かなしい", "sad", Group::IAdjective, Level::N4),
    word("静か", "しずか", "quiet", Group::NaAdjective, Level::N5),
    word("きれい", "きれい", "pretty", Group::NaAdjective, Level::N5),
    word("元気", "げんき", "healthy", Group::NaAdjective, Level::N5),
    word("好き", "すき", "liked", Group::NaAdjective, Level::N5),
    word(
        "便利",
        "べんり",
        "convenient",
        Group::NaAdjective,
        Level::N4,
    ),
    word("簡単", "かんたん", "easy", Group::NaAdjective, Level::N4),
    word(
        "大切",
        "たいせつ",
        "important",
        Group::NaAdjective,
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

    fn outcome(question: &Question, typed: &str) -> Outcome {
        let accepted: Vec<&str> = question.accepted.iter().map(|s| s.as_str()).collect();
        let shown: Vec<&str> = question.shown.iter().map(|s| s.as_str()).collect();
        grading::check(typed, &accepted, &shown)
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

    #[test]
    fn godan_words_cover_every_ending() {
        let last: Vec<char> = WORDS
            .iter()
            .filter(|word| word.group == Group::Godan)
            .filter_map(|word| word.reading.chars().last())
            .collect();
        for ending in ['う', 'く', 'ぐ', 'す', 'つ', 'ぬ', 'ぶ', 'む', 'る'] {
            assert!(last.contains(&ending), "{ending}");
        }
    }

    #[test]
    fn kaeru_is_godan() {
        assert_eq!(
            ask("帰る", Form::Negative, Formality::Plain).accepted,
            strings(&["帰らない", "かえらない"])
        );
    }

    #[test]
    fn a_word_in_kana_shows_only_its_meaning() {
        let question = ask("する", Form::Negative, Formality::Plain);
        assert_eq!(
            question.prompt,
            "What is the plain negative of する (to do)?"
        );
        assert_eq!(question.accepted, strings(&["しない"]));
        assert_eq!(question.shown, strings(&["する"]));
        assert_eq!(
            ask("する", Form::NonPast, Formality::Polite).accepted,
            strings(&["します"])
        );
    }

    #[test]
    fn the_plain_non_past_of_suru_can_be_typed_back() {
        let question = ask("する", Form::NonPast, Formality::Plain);
        assert!(is_correct(&question, "する"));
        assert!(is_correct(&question, "suru"));
    }

    #[test]
    fn kuru_answers_use_its_reading() {
        let question = ask("来る", Form::Negative, Formality::Plain);
        assert_eq!(question.accepted, strings(&["来ない", "こない"]));
        for typed in ["konai", "こない", "来ない"] {
            assert!(is_correct(&question, typed), "{typed}");
        }
        assert!(!is_correct(&question, "kinai"));
    }

    #[test]
    fn te_form_of_iku() {
        let question = ask("行く", Form::Te, Formality::Plain);
        assert_eq!(
            question.prompt,
            "What is the te-form of 行く (いく, to go)?"
        );
        assert_eq!(question.accepted, strings(&["行って", "いって"]));
    }

    #[test]
    fn aru_negative_is_nai() {
        let question = ask("ある", Form::Negative, Formality::Plain);
        assert_eq!(question.accepted, strings(&["ない"]));
        assert!(is_correct(&question, "nai"));
        assert!(!is_correct(&question, "aranai"));
    }

    #[test]
    fn i_adjective_polite_negative_accepts_both_variants() {
        let question = ask("高い", Form::Negative, Formality::Polite);
        assert_eq!(
            question.accepted,
            strings(&[
                "高くないです",
                "たかくないです",
                "高くありません",
                "たかくありません"
            ])
        );
        for typed in ["takakunaidesu", "takakuarimasen", "高くありません"] {
            assert!(is_correct(&question, typed), "{typed}");
        }
        assert!(!is_correct(&question, "takainaidesu"));
    }

    #[test]
    fn ii_uses_yo() {
        let question = ask("いい", Form::Past, Formality::Plain);
        assert_eq!(question.accepted, strings(&["よかった", "良かった"]));
        for typed in ["yokatta", "よかった", "良かった"] {
            assert!(is_correct(&question, typed), "{typed}");
        }
        assert!(!is_correct(&question, "iikatta"));
    }

    #[test]
    fn ii_accepts_yoi() {
        let question = ask("いい", Form::NonPast, Formality::Plain);
        assert_eq!(
            question.prompt,
            "What is the plain non-past of いい (good)?"
        );
        assert_eq!(question.shown, strings(&["いい"]));
        assert_eq!(question.accepted, strings(&["いい", "よい", "良い"]));
        for typed in ["ii", "yoi", "いい", "よい", "イイ", "良い"] {
            assert!(is_correct(&question, typed), "{typed}");
        }
        assert!(!is_correct(&question, "yoku"));
        let polite = ask("いい", Form::NonPast, Formality::Polite);
        assert_eq!(
            polite.accepted,
            strings(&["いいです", "よいです", "良いです"])
        );
        for typed in ["iidesu", "yoidesu", "良いです"] {
            assert!(is_correct(&polite, typed), "{typed}");
        }
    }

    #[test]
    fn ii_accepted_answers_of_every_form() {
        let expected: [&[&str]; 9] = [
            &["いい", "よい", "良い"],
            &["よくない", "良くない"],
            &["よかった", "良かった"],
            &["よくなかった", "良くなかった"],
            &["よくて", "良くて"],
            &["いいです", "よいです", "良いです"],
            &[
                "よくないです",
                "良くないです",
                "よくありません",
                "良くありません",
            ],
            &["よかったです", "良かったです"],
            &[
                "よくなかったです",
                "良くなかったです",
                "よくありませんでした",
                "良くありませんでした",
            ],
        ];
        for (&(form, formality), list) in ASKED.iter().zip(expected) {
            assert_eq!(ask("いい", form, formality).accepted, strings(list));
        }
    }

    #[test]
    fn na_adjective_polite_negative_accepts_four_variants() {
        let question = ask("静か", Form::Negative, Formality::Polite);
        assert_eq!(
            question.accepted,
            strings(&[
                "静かじゃないです",
                "しずかじゃないです",
                "静かじゃありません",
                "しずかじゃありません",
                "静かではないです",
                "しずかではないです",
                "静かではありません",
                "しずかではありません",
            ])
        );
        for typed in [
            "shizukadehaarimasen",
            "shizukadewaarimasen",
            "shizukajaarimasen",
            "shizukajanaidesu",
            "しずかではありません",
        ] {
            assert!(is_correct(&question, typed), "{typed}");
        }
        assert!(!is_correct(&question, "しずかでわありません"));
    }

    #[test]
    fn the_plain_negative_of_shizuka_typed_six_ways() {
        let question = ask("静か", Form::Negative, Formality::Plain);
        assert_eq!(
            question.accepted,
            strings(&[
                "静かじゃない",
                "しずかじゃない",
                "静かではない",
                "しずかではない"
            ])
        );
        let correct = |read_as: &str| Outcome::Correct {
            read_as: read_as.to_string(),
        };
        for typed in ["shizukadehanai", "shizukadewanai", "しずかではない"] {
            assert_eq!(
                outcome(&question, typed),
                correct("しずかではない"),
                "{typed}"
            );
        }
        assert_eq!(outcome(&question, "静かではない"), correct("静かではない"));
        assert_eq!(
            outcome(&question, "しずかでわない"),
            Outcome::Wrong {
                read_as: Some("しずかでわない".to_string())
            }
        );
        assert_eq!(
            outcome(&question, "shizukajanai"),
            correct("しずかじゃない")
        );
    }

    #[test]
    fn dewa_in_the_polite_past_negative() {
        let question = ask("静か", Form::PastNegative, Formality::Polite);
        assert_eq!(
            outcome(&question, "shizukadewaarimasendeshita"),
            Outcome::Correct {
                read_as: "しずかではありませんでした".to_string()
            }
        );
        assert_eq!(
            outcome(&question, "shizukadewanakattadesu"),
            Outcome::Correct {
                read_as: "しずかではなかったです".to_string()
            }
        );
    }

    #[test]
    fn no_accepted_answer_spells_dewa() {
        for entry in Grammar.questions(Levels::ALL) {
            for question in entry.questions {
                for text in &question.accepted {
                    assert!(!text.contains("でわ"), "{text}");
                }
            }
        }
    }

    #[test]
    fn na_adjective_plain_non_past_needs_da() {
        let question = ask("静か", Form::NonPast, Formality::Plain);
        assert!(is_correct(&question, "shizukada"));
        assert!(is_correct(&question, "しずかだ"));
        assert!(!is_correct(&question, "しずか"));
        assert!(!is_correct(&question, "shizuka"));
    }

    #[test]
    fn the_word_list_covers_every_group() {
        for group in [
            Group::Ichidan,
            Group::Godan,
            Group::Suru,
            Group::Kuru,
            Group::Iku,
            Group::Aru,
            Group::IAdjective,
            Group::Ii,
            Group::NaAdjective,
        ] {
            assert!(WORDS.iter().any(|word| word.group == group), "{group:?}");
        }
        for written in ["する", "来る", "行く", "ある", "いい"] {
            assert!(
                WORDS.iter().any(|word| word.written == written),
                "{written}"
            );
        }
        assert!(
            WORDS
                .iter()
                .any(|word| word.group == Group::Suru && word.written != word.reading)
        );
        assert!(WORDS.iter().any(|word| word.level == Level::N5));
        assert!(WORDS.iter().any(|word| word.level == Level::N4));
    }
}
