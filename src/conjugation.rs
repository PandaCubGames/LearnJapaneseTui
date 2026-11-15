// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 PandaCubGames

//! Conjugation rules: a word, its group, a form and a formality give the
//! answers. No state and no terminal, so every rule is tested directly.

/// How a word conjugates. Maps onto the categories of docs/architecture.md:
/// Verb(Ichidan) = Ichidan, Verb(Godan) = Godan, Verb(Irregular) = Suru, Kuru,
/// Iku, Aru; Adjective(I) = IAdjective, Ii; Adjective(Na) = NaAdjective.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Group {
    /// る-verbs: 食べる, 見る.
    Ichidan,
}

/// The five forms asked of a word.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Form {
    NonPast,
    Negative,
    Past,
    PastNegative,
    Te,
}

/// Plain or polite (です/ます) style.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Formality {
    Plain,
    Polite,
}

/// One answer, as written and as read. The same text twice for a word in kana.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Answer {
    pub written: String,
    pub reading: String,
}

/// Every standard way to say the word (`written`, read `reading`, of `group`) in
/// `form` and `formality`, the most common first. Empty when the form does not
/// exist (the polite te-form) or the word does not end the way its group needs.
pub fn conjugate(
    written: &str,
    reading: &str,
    group: Group,
    form: Form,
    formality: Formality,
) -> Vec<Answer> {
    let written = inflect(written, group, form, formality);
    let reading = inflect(reading, group, form, formality);
    if written.is_empty() || written.len() != reading.len() {
        return Vec::new();
    }
    written
        .into_iter()
        .zip(reading)
        .map(|(written, reading)| Answer { written, reading })
        .collect()
}

/// The word with its ending replaced by each ending of the asked form, in order.
fn inflect(text: &str, group: Group, form: Form, formality: Formality) -> Vec<String> {
    let Some((cut, endings)) = endings(group, form, formality) else {
        return Vec::new();
    };
    let Some(stem) = text.strip_suffix(cut) else {
        return Vec::new();
    };
    endings
        .into_iter()
        .map(|ending| format!("{stem}{ending}"))
        .collect()
}

/// What to cut from the end of the word and the endings to put there instead.
fn endings(group: Group, form: Form, formality: Formality) -> Option<(&'static str, Vec<String>)> {
    match group {
        Group::Ichidan => {
            let ending = pick(
                form,
                formality,
                ["る", "ない", "た", "なかった", "て"],
                ["ます", "ません", "ました", "ませんでした"],
            )?;
            Some(("る", vec![ending.to_string()]))
        }
    }
}

/// The entry for the asked form, in the order non-past, negative, past, past
/// negative, te-form (plain) and non-past, negative, past, past negative
/// (polite). `None` for the polite te-form, which does not exist.
fn pick<T>(form: Form, formality: Formality, plain: [T; 5], polite: [T; 4]) -> Option<T> {
    match formality {
        Formality::Plain => {
            let [non_past, negative, past, past_negative, te] = plain;
            Some(match form {
                Form::NonPast => non_past,
                Form::Negative => negative,
                Form::Past => past,
                Form::PastNegative => past_negative,
                Form::Te => te,
            })
        }
        Formality::Polite => {
            let [non_past, negative, past, past_negative] = polite;
            match form {
                Form::NonPast => Some(non_past),
                Form::Negative => Some(negative),
                Form::Past => Some(past),
                Form::PastNegative => Some(past_negative),
                Form::Te => None,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const NINE: [(Form, Formality); 9] = [
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

    fn a(written: &str, reading: &str) -> Answer {
        Answer {
            written: written.to_string(),
            reading: reading.to_string(),
        }
    }

    fn nine(written: &str, reading: &str, group: Group) -> Vec<Vec<Answer>> {
        NINE.iter()
            .map(|&(form, formality)| conjugate(written, reading, group, form, formality))
            .collect()
    }

    /// One answer per form, from the "written/reading" pairs of design.md Appendix B.
    fn single(pairs: [(&str, &str); 9]) -> Vec<Vec<Answer>> {
        pairs
            .iter()
            .map(|&(written, reading)| vec![a(written, reading)])
            .collect()
    }

    #[test]
    fn ichidan_nine_forms_of_taberu() {
        assert_eq!(
            nine("食べる", "たべる", Group::Ichidan),
            single([
                ("食べる", "たべる"),
                ("食べない", "たべない"),
                ("食べた", "たべた"),
                ("食べなかった", "たべなかった"),
                ("食べて", "たべて"),
                ("食べます", "たべます"),
                ("食べません", "たべません"),
                ("食べました", "たべました"),
                ("食べませんでした", "たべませんでした"),
            ])
        );
    }

    #[test]
    fn ichidan_nine_forms_of_miru() {
        assert_eq!(
            nine("見る", "みる", Group::Ichidan),
            single([
                ("見る", "みる"),
                ("見ない", "みない"),
                ("見た", "みた"),
                ("見なかった", "みなかった"),
                ("見て", "みて"),
                ("見ます", "みます"),
                ("見ません", "みません"),
                ("見ました", "みました"),
                ("見ませんでした", "みませんでした"),
            ])
        );
    }

    #[test]
    fn ichidan_has_no_polite_te_form() {
        assert!(
            conjugate(
                "食べる",
                "たべる",
                Group::Ichidan,
                Form::Te,
                Formality::Polite
            )
            .is_empty()
        );
    }

    #[test]
    fn a_word_that_does_not_fit_its_group_gives_nothing() {
        assert!(
            conjugate(
                "静か",
                "しずか",
                Group::Ichidan,
                Form::Negative,
                Formality::Plain
            )
            .is_empty()
        );
        assert!(conjugate("", "", Group::Ichidan, Form::NonPast, Formality::Plain).is_empty());
    }

    #[test]
    fn written_form_and_reading_must_both_fit() {
        assert!(
            conjugate(
                "食べる",
                "たべ",
                Group::Ichidan,
                Form::Negative,
                Formality::Plain
            )
            .is_empty()
        );
    }
}
