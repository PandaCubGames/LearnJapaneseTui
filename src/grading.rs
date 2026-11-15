// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 PandaCubGames

//! Checks a typed answer against the accepted answers, whether it is typed in
//! romaji, hiragana, katakana or kanji.

use crate::{kana, romaji};

/// The result of checking one answer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Outcome {
    /// Empty or only whitespace. Nothing to check; not counted.
    Empty,
    /// Cannot be read as Japanese. From `check`: always. From `TypoGuard`: first time only.
    LooksLikeTypo,
    /// Matches an accepted answer. `read_as` is the reading that matched.
    Correct { read_as: String },
    /// Counted wrong answer. `read_as` is `None` for mixed, macron or unreadable
    /// input; otherwise the reading to show in feedback.
    Wrong { read_as: Option<String> },
}

/// Removes whitespace anywhere, joins combining voiced marks and folds
/// katakana to hiragana. Kanji stay as they are.
fn normalise_japanese(text: &str) -> String {
    let compact: String = text.chars().filter(|c| !c.is_whitespace()).collect();
    kana::katakana_to_hiragana(&kana::compose_voicing(&compact))
}

/// The normalised shown texts that contain a kanji: copying one of them back
/// is not an answer. Texts in kana only never block anything.
fn blocked(shown: &[&str]) -> Vec<String> {
    shown
        .iter()
        .map(|s| normalise_japanese(s))
        .filter(|s| s.chars().any(kana::is_kanji))
        .collect()
}

/// A vowel with a macron or a circumflex, which romaji answers must not use.
fn is_marked_vowel(c: char) -> bool {
    matches!(
        c,
        'ā' | 'ī'
            | 'ū'
            | 'ē'
            | 'ō'
            | 'Ā'
            | 'Ī'
            | 'Ū'
            | 'Ē'
            | 'Ō'
            | 'â'
            | 'î'
            | 'û'
            | 'ê'
            | 'ô'
            | 'Â'
            | 'Î'
            | 'Û'
            | 'Ê'
            | 'Ô'
    )
}

/// A Latin letter: an ASCII letter or a marked vowel.
fn is_latin_letter(c: char) -> bool {
    c.is_ascii_alphabetic() || is_marked_vowel(c)
}

/// A character romaji answers are made of.
fn is_romaji_char(c: char) -> bool {
    is_latin_letter(c) || c == '\'' || c == '-'
}

/// Whether the romaji reading `reading` is the accepted answer `answer` with
/// some of its では typed as `dewa` (read でわ), the way では is pronounced.
/// Only a は right after で may be read わ; everything else must be equal.
fn dewa_for_deha(reading: &str, answer: &str) -> bool {
    let reading: Vec<char> = reading.chars().collect();
    let answer: Vec<char> = answer.chars().collect();
    reading.len() == answer.len()
        && reading
            .iter()
            .zip(&answer)
            .enumerate()
            .all(|(i, (&r, &a))| {
                r == a || (a == 'は' && r == 'わ' && i > 0 && answer[i - 1] == 'で')
            })
}

/// Checks one answer. Pure: same input, same result. Never remembers anything.
pub fn check(typed: &str, accepted: &[&str], shown: &[&str]) -> Outcome {
    let text: String = typed.chars().filter(|c| !c.is_whitespace()).collect();
    if text.is_empty() {
        return Outcome::Empty;
    }
    let from_romaji = text.chars().all(is_romaji_char);

    let readings: Vec<String> =
        if text.chars().any(kana::is_japanese) && text.chars().any(is_latin_letter) {
            return Outcome::Wrong { read_as: None };
        } else if text.chars().all(kana::is_japanese) {
            let reading = normalise_japanese(&text);
            if blocked(shown).contains(&reading) {
                return Outcome::Wrong {
                    read_as: Some(reading),
                };
            }
            vec![reading]
        } else if text.chars().all(is_romaji_char) {
            if text.chars().any(is_marked_vowel) {
                return Outcome::Wrong { read_as: None };
            }
            match romaji::to_hiragana(&text) {
                Some(readings) if !readings.is_empty() => readings,
                _ => return Outcome::LooksLikeTypo,
            }
        } else {
            return Outcome::LooksLikeTypo;
        };

    let accepted: Vec<String> = accepted.iter().map(|a| normalise_japanese(a)).collect();
    if let Some(read_as) = readings.iter().find(|r| accepted.contains(r)) {
        return Outcome::Correct {
            read_as: read_as.clone(),
        };
    }
    if from_romaji {
        for reading in &readings {
            if let Some(answer) = accepted.iter().find(|a| dewa_for_deha(reading, a)) {
                return Outcome::Correct {
                    read_as: answer.clone(),
                };
            }
        }
    }
    Outcome::Wrong {
        read_as: readings.into_iter().next(),
    }
}

/// The typo memory of one question.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct TypoGuard {
    warned: Option<String>,
}

impl TypoGuard {
    /// Like `check`, but an unreadable text submitted again unchanged is
    /// `Wrong { read_as: None }`.
    pub fn submit(&mut self, typed: &str, accepted: &[&str], shown: &[&str]) -> Outcome {
        match check(typed, accepted, shown) {
            Outcome::Empty => Outcome::Empty,
            Outcome::LooksLikeTypo => {
                if self.warned.as_deref() == Some(typed) {
                    self.warned = None;
                    Outcome::Wrong { read_as: None }
                } else {
                    self.warned = Some(typed.to_string());
                    Outcome::LooksLikeTypo
                }
            }
            other => {
                self.warned = None;
                other
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const NOTHING: &[&str] = &[];

    fn correct(r: &str) -> Outcome {
        Outcome::Correct {
            read_as: r.to_string(),
        }
    }

    fn wrong(r: &str) -> Outcome {
        Outcome::Wrong {
            read_as: Some(r.to_string()),
        }
    }

    fn unread() -> Outcome {
        Outcome::Wrong { read_as: None }
    }

    #[test]
    fn four_ways_to_answer_takai() {
        let accepted = ["高い", "たかい"];
        assert_eq!(check("takai", &accepted, NOTHING), correct("たかい"));
        assert_eq!(check("たかい", &accepted, NOTHING), correct("たかい"));
        assert_eq!(check("タカイ", &accepted, NOTHING), correct("たかい"));
        assert_eq!(check("高い", &accepted, NOTHING), correct("高い"));
    }

    #[test]
    fn katakana_accepted_answer() {
        for typed in ["しょく", "ショク", "shoku"] {
            assert_eq!(check(typed, &["ショク"], NOTHING), correct("しょく"));
        }
    }

    #[test]
    fn capital_romaji_is_correct() {
        assert_eq!(check("TAKAI", &["たかい"], NOTHING), correct("たかい"));
    }

    #[test]
    fn long_vowel_typed_out_is_correct() {
        assert_eq!(
            check("toukyou", &["東京", "とうきょう"], NOTHING),
            correct("とうきょう")
        );
    }

    #[test]
    fn tokyo_without_long_vowels_is_wrong() {
        assert_eq!(
            check("tokyo", &["東京", "とうきょう"], NOTHING),
            wrong("ときょ")
        );
    }

    #[test]
    fn macron_romaji_is_wrong() {
        for typed in ["tōkyō", "TŌKYŌ", "tôkyô"] {
            assert_eq!(check(typed, &["東京", "とうきょう"], NOTHING), unread());
        }
        assert_eq!(check("kōkō", &["ここ"], NOTHING), unread());
    }

    #[test]
    fn mixed_scripts_are_wrong() {
        for typed in ["食beru", "たberu", "食beru1"] {
            assert_eq!(check(typed, &["食べる", "たべる"], NOTHING), unread());
        }
    }

    #[test]
    fn shown_kanji_is_never_accepted() {
        let accepted = ["食", "しょく", "たべる"];
        let shown = ["食"];
        assert_eq!(check("食", &accepted, &shown), wrong("食"));
        assert_eq!(check("しょく", &accepted, &shown), correct("しょく"));
        assert_eq!(check("たべる", &accepted, &shown), correct("たべる"));
        assert_eq!(check("taberu", &accepted, &shown), correct("たべる"));
    }

    #[test]
    fn copying_a_shown_word_with_kanji_is_wrong() {
        let accepted = ["食べる", "たべる"];
        let shown = ["食べる"];
        for typed in ["食べる", "食 べる", "食ベル"] {
            assert_eq!(check(typed, &accepted, &shown), wrong("食べる"), "{typed}");
        }
        for typed in ["たべる", "タベル", "taberu"] {
            assert_eq!(
                check(typed, &accepted, &shown),
                correct("たべる"),
                "{typed}"
            );
        }
    }

    #[test]
    fn kana_shown_text_never_makes_an_answer_wrong() {
        for typed in ["する", "スル", "suru"] {
            assert_eq!(check(typed, &["する"], &["する"]), correct("する"));
        }
        let accepted = ["高い", "たかい"];
        let shown = ["たかい"];
        for typed in ["takai", "タカイ", "たかい"] {
            assert_eq!(check(typed, &accepted, &shown), correct("たかい"));
        }
        assert_eq!(check("高い", &accepted, &shown), correct("高い"));
    }

    #[test]
    fn romaji_is_never_a_repeat() {
        let accepted = ["今夜", "こんや"];
        let shown = ["今夜", "こんや"];
        assert_eq!(check("konnya", &accepted, &shown), correct("こんや"));
        assert_eq!(check("kon'ya", &accepted, &shown), correct("こんや"));
        assert_eq!(check("今夜", &accepted, &shown), wrong("今夜"));
    }

    #[test]
    fn iteration_mark_counts_as_kanji() {
        assert_eq!(check("々", &["々"], &["々"]), wrong("々"));
    }

    #[test]
    fn containing_the_shown_text_is_not_repeating_it() {
        assert_eq!(check("食べる", &["食べる"], &["食"]), correct("食べる"));
    }

    #[test]
    fn spaces_are_ignored() {
        let accepted = ["たべてください"];
        for typed in [" tabete kudasai ", "たべて\u{3000}ください"] {
            assert_eq!(check(typed, &accepted, NOTHING), correct("たべてください"));
        }
        assert_eq!(
            check("tabetekudasai", &["たべて ください"], NOTHING),
            correct("たべてください")
        );
    }

    #[test]
    fn empty_answers_are_not_checked() {
        for typed in ["", "   ", "\u{3000}", "\t"] {
            assert_eq!(check(typed, &["たべる"], NOTHING), Outcome::Empty);
        }
    }

    #[test]
    fn unreadable_answers_look_like_a_typo() {
        for typed in [
            "tabxru",
            "tabek",
            "taka1",
            "たか1",
            "ｔａｋａｉ",
            "ﾀｶｲ",
            "@",
            "たか-",
        ] {
            assert_eq!(
                check(typed, &["たべる"], NOTHING),
                Outcome::LooksLikeTypo,
                "{typed}"
            );
        }
    }

    #[test]
    fn readings_for_feedback() {
        let accepted = ["たかい"];
        assert_eq!(check("tokyo", &accepted, NOTHING), wrong("ときょ"));
        assert_eq!(check("タカイ", &accepted, NOTHING), correct("たかい"));
        assert_eq!(check("高い", &accepted, NOTHING), wrong("高い"));
        assert_eq!(check("yasui", &accepted, NOTHING), wrong("やすい"));
    }

    #[test]
    fn combining_marks_are_composed() {
        assert_eq!(check("か\u{3099}い", &["がい"], NOTHING), correct("がい"));
        assert_eq!(check("ハ\u{309A}", &["ぱ"], NOTHING), correct("ぱ"));
    }

    #[test]
    fn check_remembers_nothing() {
        for _ in 0..2 {
            assert_eq!(
                check("tabxru", &["たべる"], NOTHING),
                Outcome::LooksLikeTypo
            );
        }
    }

    #[test]
    fn both_spellings_of_konnichiha_are_correct() {
        for typed in ["konnichiha", "konnnichiha", "KONNICHIHA"] {
            assert_eq!(
                check(typed, &["こんにちは"], NOTHING),
                correct("こんにちは"),
                "{typed}"
            );
        }
    }

    #[test]
    fn both_spellings_of_onna_are_correct() {
        for typed in ["onna", "onnna"] {
            assert_eq!(
                check(typed, &["女", "おんな"], NOTHING),
                correct("おんな"),
                "{typed}"
            );
        }
    }

    #[test]
    fn keyboard_reading_can_be_correct() {
        let accepted = ["今夜", "こんや"];
        assert_eq!(check("konnya", &accepted, NOTHING), correct("こんや"));
        assert_eq!(check("kon'ya", &accepted, NOTHING), correct("こんや"));
        assert_eq!(check("konya", &accepted, NOTHING), wrong("こにゃ"));
        assert_eq!(check("kinnen", &["きんえん"], NOTHING), correct("きんえん"));
        assert_eq!(check("onna", &["おんあ"], NOTHING), correct("おんあ"));
    }

    #[test]
    fn single_readings_stay_exact() {
        assert_eq!(check("kinen", &["きんえん"], NOTHING), wrong("きねん"));
        assert_eq!(check("kin'en", &["きんえん"], NOTHING), correct("きんえん"));
    }

    #[test]
    fn wrong_answer_shows_the_textbook_reading() {
        let accepted = ["こんばんは"];
        assert_eq!(check("konnichiwa", &accepted, NOTHING), wrong("こんにちわ"));
        assert_eq!(check("konnichiha", &accepted, NOTHING), wrong("こんにちは"));
        assert_eq!(check("onna", &["おとこ"], NOTHING), wrong("おんな"));
    }

    #[test]
    fn unreadable_text_with_an_nn_spot_is_a_typo() {
        assert_eq!(
            check("konnichiq", &["こんにちは"], NOTHING),
            Outcome::LooksLikeTypo
        );
    }

    fn submit(guard: &mut TypoGuard, typed: &str) -> Outcome {
        guard.submit(typed, &["たべる"], NOTHING)
    }

    #[test]
    fn same_unreadable_text_twice_is_wrong() {
        let mut guard = TypoGuard::default();
        assert_eq!(submit(&mut guard, "tabxru"), Outcome::LooksLikeTypo);
        assert_eq!(submit(&mut guard, "tabxru"), unread());
    }

    #[test]
    fn corrected_text_is_checked_normally() {
        let mut guard = TypoGuard::default();
        assert_eq!(submit(&mut guard, "tabxru"), Outcome::LooksLikeTypo);
        assert_eq!(submit(&mut guard, "taberu"), correct("たべる"));
    }

    #[test]
    fn different_unreadable_text_gets_its_own_warning() {
        let mut guard = TypoGuard::default();
        assert_eq!(submit(&mut guard, "tabxru"), Outcome::LooksLikeTypo);
        assert_eq!(submit(&mut guard, "tabwru"), Outcome::LooksLikeTypo);
        assert_eq!(submit(&mut guard, "tabwru"), unread());
    }

    #[test]
    fn empty_submit_keeps_the_memory() {
        let mut guard = TypoGuard::default();
        assert_eq!(submit(&mut guard, "tabxru"), Outcome::LooksLikeTypo);
        assert_eq!(submit(&mut guard, ""), Outcome::Empty);
        assert_eq!(submit(&mut guard, "tabxru"), unread());
    }

    #[test]
    fn readable_wrong_answer_is_never_a_warning() {
        let mut guard = TypoGuard::default();
        assert_eq!(
            guard.submit("tokyo", &["とうきょう"], NOTHING),
            wrong("ときょ")
        );
    }

    #[test]
    fn same_text_means_exactly_the_same() {
        let mut guard = TypoGuard::default();
        assert_eq!(submit(&mut guard, "tabxru"), Outcome::LooksLikeTypo);
        assert_eq!(submit(&mut guard, "tabxru "), Outcome::LooksLikeTypo);
        assert_eq!(submit(&mut guard, "tabxru "), unread());
    }

    #[test]
    fn a_readable_submit_clears_the_memory() {
        let mut guard = TypoGuard::default();
        assert_eq!(submit(&mut guard, "tabxru"), Outcome::LooksLikeTypo);
        assert_eq!(submit(&mut guard, "tokyo"), wrong("ときょ"));
        assert_eq!(submit(&mut guard, "tabxru"), Outcome::LooksLikeTypo);
    }

    #[test]
    fn mixed_answer_is_wrong_at_once() {
        let mut guard = TypoGuard::default();
        assert_eq!(guard.submit("食beru", &["食べる"], NOTHING), unread());
    }

    #[test]
    fn two_readings_are_never_a_warning() {
        let mut guard = TypoGuard::default();
        let accepted = ["こんばんは"];
        assert_eq!(
            guard.submit("konnichiha", &accepted, NOTHING),
            wrong("こんにちは")
        );
        assert_eq!(
            guard.submit("konnichiq", &accepted, NOTHING),
            Outcome::LooksLikeTypo
        );
        assert_eq!(guard.submit("konnichiq", &accepted, NOTHING), unread());
    }

    #[test]
    fn dewa_is_accepted_for_deha_in_romaji() {
        let accepted = ["静かではない", "しずかではない"];
        let shown = ["静か", "しずか"];
        for typed in [
            "shizukadewanai",
            "shizukadehanai",
            "SHIZUKADEWANAI",
            "shizuka dewa nai",
        ] {
            assert_eq!(
                check(typed, &accepted, &shown),
                correct("しずかではない"),
                "{typed}"
            );
        }
    }

    #[test]
    fn dewa_in_a_polite_form() {
        assert_eq!(
            check(
                "shizukadewaarimasen",
                &["静かではありません", "しずかではありません"],
                &["静か", "しずか"]
            ),
            correct("しずかではありません")
        );
    }

    #[test]
    fn kana_dewa_stays_wrong() {
        let accepted = ["静かではない", "しずかではない"];
        let shown = ["静か", "しずか"];
        for typed in ["しずかでわない", "シズカデワナイ"] {
            assert_eq!(
                check(typed, &accepted, &shown),
                wrong("しずかでわない"),
                "{typed}"
            );
        }
        assert_eq!(
            check("静かではない", &accepted, &shown),
            correct("静かではない")
        );
        assert_eq!(
            check("しずかではない", &accepted, &shown),
            correct("しずかではない")
        );
    }

    #[test]
    fn dewa_composes_with_nn() {
        let accepted = ["今日ではない", "こんにちではない"];
        for typed in ["konnichidewanai", "konnnichidewanai", "konnichidehanai"] {
            assert_eq!(
                check(typed, &accepted, NOTHING),
                correct("こんにちではない"),
                "{typed}"
            );
        }
        assert_eq!(
            check(
                "kinnendewanai",
                &["禁煙ではない", "きんえんではない"],
                NOTHING
            ),
            correct("きんえんではない")
        );
    }

    #[test]
    fn each_deha_may_be_typed_either_way() {
        for typed in ["dewadewa", "dehadewa", "dewadeha"] {
            assert_eq!(
                check(typed, &["ではでは"], NOTHING),
                correct("ではでは"),
                "{typed}"
            );
        }
    }

    #[test]
    fn other_ha_are_still_typed_ha() {
        assert_eq!(
            check("konnichiwa", &["こんにちは"], NOTHING),
            wrong("こんにちわ")
        );
        assert_eq!(
            check("watashiwa", &["わたしは"], NOTHING),
            wrong("わたしわ")
        );
    }

    #[test]
    fn dewa_does_not_work_the_other_way_round() {
        assert_eq!(
            check("tedehakeru", &["てでわける"], NOTHING),
            wrong("てではける")
        );
    }

    #[test]
    fn a_wrong_dewa_answer_keeps_its_reading() {
        assert_eq!(
            check("shizukadewaarimasu", &["しずかではありません"], NOTHING),
            wrong("しずかでわあります")
        );
    }
}
