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
    /// する and noun + する: 勉強する.
    Suru,
    /// 来る: the kanji stays, the reading changes.
    Kuru,
    /// 行く: godan, but its te-form is って and its past った.
    Iku,
    /// ある: godan, but its plain negatives are ない and なかった.
    Aru,
    /// い-adjectives: 高い.
    IAdjective,
    /// いい: non-past いい or よい, other forms on よ, and each よ form also
    /// written with 良.
    Ii,
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
    if group == Group::Ii {
        let (Some(written_stem), Some(reading_stem)) =
            (written.strip_suffix("いい"), reading.strip_suffix("いい"))
        else {
            return Vec::new();
        };
        return ii_endings(form, formality)
            .unwrap_or_default()
            .into_iter()
            .map(|(written_ending, reading_ending)| Answer {
                written: format!("{written_stem}{written_ending}"),
                reading: format!("{reading_stem}{reading_ending}"),
            })
            .collect();
    }
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

/// いい's endings after its stem, as (written, reading) pairs: each variant in kana, then with 良.
fn ii_endings(form: Form, formality: Formality) -> Option<Vec<(&'static str, &'static str)>> {
    pick(
        form,
        formality,
        [
            vec![("いい", "いい"), ("よい", "よい"), ("良い", "よい")],
            vec![("よくない", "よくない"), ("良くない", "よくない")],
            vec![("よかった", "よかった"), ("良かった", "よかった")],
            vec![
                ("よくなかった", "よくなかった"),
                ("良くなかった", "よくなかった"),
            ],
            vec![("よくて", "よくて"), ("良くて", "よくて")],
        ],
        [
            vec![
                ("いいです", "いいです"),
                ("よいです", "よいです"),
                ("良いです", "よいです"),
            ],
            vec![
                ("よくないです", "よくないです"),
                ("良くないです", "よくないです"),
                ("よくありません", "よくありません"),
                ("良くありません", "よくありません"),
            ],
            vec![
                ("よかったです", "よかったです"),
                ("良かったです", "よかったです"),
            ],
            vec![
                ("よくなかったです", "よくなかったです"),
                ("良くなかったです", "よくなかったです"),
                ("よくありませんでした", "よくありませんでした"),
                ("良くありませんでした", "よくありませんでした"),
            ],
        ],
    )
}

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
        Group::Suru => {
            let ending = pick(
                form,
                formality,
                ["する", "しない", "した", "しなかった", "して"],
                ["します", "しません", "しました", "しませんでした"],
            )?;
            Some(("する", vec![ending.to_string()]))
        }
        Group::Kuru => {
            let (cut, plain, polite) = if text.ends_with("来る") {
                (
                    "来る",
                    ["来る", "来ない", "来た", "来なかった", "来て"],
                    ["来ます", "来ません", "来ました", "来ませんでした"],
                )
            } else if text.ends_with("くる") {
                (
                    "くる",
                    ["くる", "こない", "きた", "こなかった", "きて"],
                    ["きます", "きません", "きました", "きませんでした"],
                )
            } else {
                return None;
            };
            let ending = pick(form, formality, plain, polite)?;
            Some((cut, vec![ending.to_string()]))
        }
        Group::Iku => {
            if !text.ends_with('く') {
                return None;
            }
            let ending = pick(
                form,
                formality,
                ["く", "かない", "った", "かなかった", "って"],
                ["きます", "きません", "きました", "きませんでした"],
            )?;
            Some(("く", vec![ending.to_string()]))
        }
        Group::Aru => {
            let ending = pick(
                form,
                formality,
                ["ある", "ない", "あった", "なかった", "あって"],
                ["あります", "ありません", "ありました", "ありませんでした"],
            )?;
            Some(("ある", vec![ending.to_string()]))
        }
        Group::IAdjective => {
            let endings = pick(
                form,
                formality,
                [
                    vec!["い"],
                    vec!["くない"],
                    vec!["かった"],
                    vec!["くなかった"],
                    vec!["くて"],
                ],
                [
                    vec!["いです"],
                    vec!["くないです", "くありません"],
                    vec!["かったです"],
                    vec!["くなかったです", "くありませんでした"],
                ],
            )?;
            Some((
                "い",
                endings
                    .into_iter()
                    .map(|ending| ending.to_string())
                    .collect(),
            ))
        }
        // conjugate builds いい's answers from ii_endings
        Group::Ii => None,
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

    #[test]
    fn suru_nine_forms() {
        assert_eq!(
            nine("する", "する", Group::Suru),
            single([
                ("する", "する"),
                ("しない", "しない"),
                ("した", "した"),
                ("しなかった", "しなかった"),
                ("して", "して"),
                ("します", "します"),
                ("しません", "しません"),
                ("しました", "しました"),
                ("しませんでした", "しませんでした"),
            ])
        );
    }

    #[test]
    fn noun_suru_keeps_the_noun() {
        assert_eq!(
            nine("勉強する", "べんきょうする", Group::Suru),
            single([
                ("勉強する", "べんきょうする"),
                ("勉強しない", "べんきょうしない"),
                ("勉強した", "べんきょうした"),
                ("勉強しなかった", "べんきょうしなかった"),
                ("勉強して", "べんきょうして"),
                ("勉強します", "べんきょうします"),
                ("勉強しません", "べんきょうしません"),
                ("勉強しました", "べんきょうしました"),
                ("勉強しませんでした", "べんきょうしませんでした"),
            ])
        );
    }

    #[test]
    fn kuru_keeps_its_kanji_and_changes_its_reading() {
        assert_eq!(
            nine("来る", "くる", Group::Kuru),
            single([
                ("来る", "くる"),
                ("来ない", "こない"),
                ("来た", "きた"),
                ("来なかった", "こなかった"),
                ("来て", "きて"),
                ("来ます", "きます"),
                ("来ません", "きません"),
                ("来ました", "きました"),
                ("来ませんでした", "きませんでした"),
            ])
        );
    }

    #[test]
    fn kuru_in_kana() {
        assert_eq!(
            nine("くる", "くる", Group::Kuru),
            single([
                ("くる", "くる"),
                ("こない", "こない"),
                ("きた", "きた"),
                ("こなかった", "こなかった"),
                ("きて", "きて"),
                ("きます", "きます"),
                ("きません", "きません"),
                ("きました", "きました"),
                ("きませんでした", "きませんでした"),
            ])
        );
    }

    #[test]
    fn suru_and_kuru_have_no_polite_te_form() {
        assert!(conjugate("する", "する", Group::Suru, Form::Te, Formality::Polite).is_empty());
        assert!(conjugate("来る", "くる", Group::Kuru, Form::Te, Formality::Polite).is_empty());
    }

    #[test]
    fn suru_and_kuru_need_their_ending() {
        assert!(
            conjugate(
                "食べる",
                "たべる",
                Group::Suru,
                Form::Negative,
                Formality::Plain
            )
            .is_empty()
        );
        assert!(
            conjugate(
                "見る",
                "みる",
                Group::Kuru,
                Form::Negative,
                Formality::Plain
            )
            .is_empty()
        );
    }

    #[test]
    fn iku_nine_forms() {
        assert_eq!(
            nine("行く", "いく", Group::Iku),
            single([
                ("行く", "いく"),
                ("行かない", "いかない"),
                ("行った", "いった"),
                ("行かなかった", "いかなかった"),
                ("行って", "いって"),
                ("行きます", "いきます"),
                ("行きません", "いきません"),
                ("行きました", "いきました"),
                ("行きませんでした", "いきませんでした"),
            ])
        );
    }

    #[test]
    fn iku_differs_from_godan_only_in_te_form_and_past() {
        let iku = nine("行く", "いく", Group::Iku);
        let godan = nine("行く", "いく", Group::Godan);
        for index in 0..9 {
            if index == 2 || index == 4 {
                assert_ne!(iku[index], godan[index], "{index}");
            } else {
                assert_eq!(iku[index], godan[index], "{index}");
            }
        }
    }

    #[test]
    fn aru_nine_forms() {
        assert_eq!(
            nine("ある", "ある", Group::Aru),
            single([
                ("ある", "ある"),
                ("ない", "ない"),
                ("あった", "あった"),
                ("なかった", "なかった"),
                ("あって", "あって"),
                ("あります", "あります"),
                ("ありません", "ありません"),
                ("ありました", "ありました"),
                ("ありませんでした", "ありませんでした"),
            ])
        );
    }

    #[test]
    fn iku_and_aru_have_no_polite_te_form() {
        assert!(conjugate("行く", "いく", Group::Iku, Form::Te, Formality::Polite).is_empty());
        assert!(conjugate("ある", "ある", Group::Aru, Form::Te, Formality::Polite).is_empty());
    }

    #[test]
    fn iku_and_aru_need_their_ending() {
        assert!(conjugate("飲む", "のむ", Group::Iku, Form::Te, Formality::Plain).is_empty());
        assert!(conjugate("いる", "いる", Group::Aru, Form::Negative, Formality::Plain).is_empty());
    }

    #[test]
    fn i_adjective_plain_forms() {
        assert_eq!(
            nine("高い", "たかい", Group::IAdjective)[..5],
            single5([
                ("高い", "たかい"),
                ("高くない", "たかくない"),
                ("高かった", "たかかった"),
                ("高くなかった", "たかくなかった"),
                ("高くて", "たかくて"),
            ])
        );
    }

    /// One answer per form, for the first five forms.
    fn single5(pairs: [(&str, &str); 5]) -> Vec<Vec<Answer>> {
        pairs
            .iter()
            .map(|&(written, reading)| vec![a(written, reading)])
            .collect()
    }

    #[test]
    fn i_adjective_polite_forms_and_variants() {
        assert_eq!(
            nine("高い", "たかい", Group::IAdjective)[5..],
            [
                vec![a("高いです", "たかいです")],
                vec![
                    a("高くないです", "たかくないです"),
                    a("高くありません", "たかくありません"),
                ],
                vec![a("高かったです", "たかかったです")],
                vec![
                    a("高くなかったです", "たかくなかったです"),
                    a("高くありませんでした", "たかくありませんでした"),
                ],
            ]
        );
    }

    #[test]
    fn i_adjective_polite_past_is_katta_desu() {
        assert_eq!(
            conjugate(
                "高い",
                "たかい",
                Group::IAdjective,
                Form::Past,
                Formality::Polite
            ),
            vec![a("高かったです", "たかかったです")]
        );
    }

    #[test]
    fn ii_forms() {
        assert_eq!(
            nine("いい", "いい", Group::Ii),
            vec![
                vec![a("いい", "いい"), a("よい", "よい"), a("良い", "よい")],
                vec![a("よくない", "よくない"), a("良くない", "よくない")],
                vec![a("よかった", "よかった"), a("良かった", "よかった")],
                vec![
                    a("よくなかった", "よくなかった"),
                    a("良くなかった", "よくなかった"),
                ],
                vec![a("よくて", "よくて"), a("良くて", "よくて")],
                vec![
                    a("いいです", "いいです"),
                    a("よいです", "よいです"),
                    a("良いです", "よいです"),
                ],
                vec![
                    a("よくないです", "よくないです"),
                    a("良くないです", "よくないです"),
                    a("よくありません", "よくありません"),
                    a("良くありません", "よくありません"),
                ],
                vec![
                    a("よかったです", "よかったです"),
                    a("良かったです", "よかったです"),
                ],
                vec![
                    a("よくなかったです", "よくなかったです"),
                    a("良くなかったです", "よくなかったです"),
                    a("よくありませんでした", "よくありませんでした"),
                    a("良くありませんでした", "よくありませんでした"),
                ],
            ]
        );
    }

    #[test]
    fn ii_non_past_starts_with_ii() {
        let plain = conjugate("いい", "いい", Group::Ii, Form::NonPast, Formality::Plain);
        assert_eq!(plain[0], a("いい", "いい"));
        let polite = conjugate("いい", "いい", Group::Ii, Form::NonPast, Formality::Polite);
        assert_eq!(polite[0], a("いいです", "いいです"));
    }

    #[test]
    fn ii_keeps_what_comes_before_it() {
        assert_eq!(
            conjugate(
                "かっこいい",
                "かっこいい",
                Group::Ii,
                Form::Negative,
                Formality::Plain
            ),
            vec![
                a("かっこよくない", "かっこよくない"),
                a("かっこ良くない", "かっこよくない"),
            ]
        );
    }

    #[test]
    fn adjectives_have_no_polite_te_form() {
        assert!(
            conjugate(
                "高い",
                "たかい",
                Group::IAdjective,
                Form::Te,
                Formality::Polite
            )
            .is_empty()
        );
        assert!(conjugate("いい", "いい", Group::Ii, Form::Te, Formality::Polite).is_empty());
    }

    #[test]
    fn i_adjective_needs_its_ending() {
        assert!(
            conjugate(
                "静か",
                "しずか",
                Group::IAdjective,
                Form::Negative,
                Formality::Plain
            )
            .is_empty()
        );
        assert!(
            conjugate(
                "高い",
                "たかい",
                Group::Ii,
                Form::Negative,
                Formality::Plain
            )
            .is_empty()
        );
    }
}
