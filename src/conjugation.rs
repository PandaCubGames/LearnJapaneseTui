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
    /// う-verbs, ending in う く ぐ す つ ぬ ぶ む る.
    Godan,
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
    let Some((cut, endings)) = endings(text, group, form, formality) else {
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

/// A godan verb's last kana, its negative stem, its polite stem, its te-form and past endings.
const GODAN: [(&str, &str, &str, &str, &str); 9] = [
    ("う", "わ", "い", "って", "った"),
    ("く", "か", "き", "いて", "いた"),
    ("ぐ", "が", "ぎ", "いで", "いだ"),
    ("す", "さ", "し", "して", "した"),
    ("つ", "た", "ち", "って", "った"),
    ("ぬ", "な", "に", "んで", "んだ"),
    ("ぶ", "ば", "び", "んで", "んだ"),
    ("む", "ま", "み", "んで", "んだ"),
    ("る", "ら", "り", "って", "った"),
];

/// What to cut from the end of `text` and the endings to put there instead.
/// Looks at the text only where the group needs it.
fn endings(
    text: &str,
    group: Group,
    form: Form,
    formality: Formality,
) -> Option<(&'static str, Vec<String>)> {
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
        Group::Godan => {
            let last = text.chars().last()?;
            let &(k, a, i, te, ta) = GODAN.iter().find(|row| row.0.starts_with(last))?;
            let ending = pick(
                form,
                formality,
                [
                    k.to_string(),
                    format!("{a}ない"),
                    ta.to_string(),
                    format!("{a}なかった"),
                    te.to_string(),
                ],
                [
                    format!("{i}ます"),
                    format!("{i}ません"),
                    format!("{i}ました"),
                    format!("{i}ませんでした"),
                ],
            )?;
            Some((k, vec![ending]))
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

    /// The nine written forms of Appendix B; each reading is the reading stem
    /// plus the same ending.
    fn godan_forms(written: [&str; 9], written_stem: &str, reading_stem: &str) -> Vec<Vec<Answer>> {
        written
            .iter()
            .map(|form| {
                let ending = form.strip_prefix(written_stem).unwrap();
                vec![a(form, &format!("{reading_stem}{ending}"))]
            })
            .collect()
    }

    #[test]
    fn godan_u_kau() {
        let forms = [
            "買う",
            "買わない",
            "買った",
            "買わなかった",
            "買って",
            "買います",
            "買いません",
            "買いました",
            "買いませんでした",
        ];
        assert_eq!(
            nine("買う", "かう", Group::Godan),
            godan_forms(forms, "買", "か")
        );
        let readings = [
            "かう",
            "かわない",
            "かった",
            "かわなかった",
            "かって",
            "かいます",
            "かいません",
            "かいました",
            "かいませんでした",
        ];
        for (answers, reading) in nine("買う", "かう", Group::Godan).iter().zip(readings) {
            assert_eq!(answers[0].reading, reading);
        }
    }

    #[test]
    fn godan_ku_kaku() {
        let forms = [
            "書く",
            "書かない",
            "書いた",
            "書かなかった",
            "書いて",
            "書きます",
            "書きません",
            "書きました",
            "書きませんでした",
        ];
        assert_eq!(
            nine("書く", "かく", Group::Godan),
            godan_forms(forms, "書", "か")
        );
    }

    #[test]
    fn godan_gu_oyogu() {
        let forms = [
            "泳ぐ",
            "泳がない",
            "泳いだ",
            "泳がなかった",
            "泳いで",
            "泳ぎます",
            "泳ぎません",
            "泳ぎました",
            "泳ぎませんでした",
        ];
        assert_eq!(
            nine("泳ぐ", "およぐ", Group::Godan),
            godan_forms(forms, "泳", "およ")
        );
    }

    #[test]
    fn godan_su_hanasu() {
        let forms = [
            "話す",
            "話さない",
            "話した",
            "話さなかった",
            "話して",
            "話します",
            "話しません",
            "話しました",
            "話しませんでした",
        ];
        assert_eq!(
            nine("話す", "はなす", Group::Godan),
            godan_forms(forms, "話", "はな")
        );
    }

    #[test]
    fn godan_tsu_matsu() {
        let forms = [
            "待つ",
            "待たない",
            "待った",
            "待たなかった",
            "待って",
            "待ちます",
            "待ちません",
            "待ちました",
            "待ちませんでした",
        ];
        assert_eq!(
            nine("待つ", "まつ", Group::Godan),
            godan_forms(forms, "待", "ま")
        );
    }

    #[test]
    fn godan_nu_shinu() {
        let forms = [
            "死ぬ",
            "死なない",
            "死んだ",
            "死ななかった",
            "死んで",
            "死にます",
            "死にません",
            "死にました",
            "死にませんでした",
        ];
        assert_eq!(
            nine("死ぬ", "しぬ", Group::Godan),
            godan_forms(forms, "死", "し")
        );
        let readings = [
            "しぬ",
            "しなない",
            "しんだ",
            "しななかった",
            "しんで",
            "しにます",
            "しにません",
            "しにました",
            "しにませんでした",
        ];
        for (answers, reading) in nine("死ぬ", "しぬ", Group::Godan).iter().zip(readings) {
            assert_eq!(answers[0].reading, reading);
        }
    }

    #[test]
    fn godan_bu_asobu() {
        let forms = [
            "遊ぶ",
            "遊ばない",
            "遊んだ",
            "遊ばなかった",
            "遊んで",
            "遊びます",
            "遊びません",
            "遊びました",
            "遊びませんでした",
        ];
        assert_eq!(
            nine("遊ぶ", "あそぶ", Group::Godan),
            godan_forms(forms, "遊", "あそ")
        );
    }

    #[test]
    fn godan_mu_nomu() {
        let forms = [
            "飲む",
            "飲まない",
            "飲んだ",
            "飲まなかった",
            "飲んで",
            "飲みます",
            "飲みません",
            "飲みました",
            "飲みませんでした",
        ];
        assert_eq!(
            nine("飲む", "のむ", Group::Godan),
            godan_forms(forms, "飲", "の")
        );
    }

    #[test]
    fn godan_ru_kaeru() {
        let forms = [
            "帰る",
            "帰らない",
            "帰った",
            "帰らなかった",
            "帰って",
            "帰ります",
            "帰りません",
            "帰りました",
            "帰りませんでした",
        ];
        assert_eq!(
            nine("帰る", "かえる", Group::Godan),
            godan_forms(forms, "帰", "かえ")
        );
    }

    #[test]
    fn godan_has_no_polite_te_form() {
        assert!(conjugate("書く", "かく", Group::Godan, Form::Te, Formality::Polite).is_empty());
    }

    #[test]
    fn a_verb_without_a_godan_ending_gives_nothing() {
        for (written, reading) in [("高い", "たかい"), ("静か", "しずか")] {
            assert!(
                conjugate(
                    written,
                    reading,
                    Group::Godan,
                    Form::Negative,
                    Formality::Plain
                )
                .is_empty(),
                "{written}"
            );
        }
    }
}
