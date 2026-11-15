// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 PandaCubGames

//! Kana helpers: which characters are Japanese, joining combining voiced
//! marks, and folding katakana to hiragana.

/// Japanese characters: kana, kanji, CJK punctuation, variation selectors.
///
/// Full-width Latin letters and half-width katakana (U+FF00..=U+FFEF) are not
/// Japanese here, on purpose.
pub fn is_japanese(c: char) -> bool {
    matches!(
        c,
        '\u{3000}'..='\u{303F}'
            | '\u{3040}'..='\u{309F}'
            | '\u{30A0}'..='\u{30FF}'
            | '\u{31F0}'..='\u{31FF}'
            | '\u{3400}'..='\u{4DBF}'
            | '\u{4E00}'..='\u{9FFF}'
            | '\u{F900}'..='\u{FAFF}'
            | '\u{20000}'..='\u{3FFFF}'
            | '\u{FE00}'..='\u{FE0F}'
            | '\u{E0100}'..='\u{E01EF}'
    )
}

/// Kanji: CJK ideographs and the marks 々 〆 〇. Every kanji is also Japanese.
pub fn is_kanji(c: char) -> bool {
    matches!(
        c,
        '\u{3005}'..='\u{3007}'
            | '\u{3400}'..='\u{4DBF}'
            | '\u{4E00}'..='\u{9FFF}'
            | '\u{F900}'..='\u{FAFF}'
            | '\u{20000}'..='\u{3FFFF}'
    )
}

/// The precomposed kana for `base` followed by the combining `mark`, if one
/// exists.
fn voiced(base: char, mark: char) -> Option<char> {
    const PLUS_ONE: &str =
        "かきくけこさしすせそたちつてとはひふへほカキクケコサシスセソタチツテトハヒフヘホ";
    const PLUS_TWO: &str = "はひふへほハヒフヘホ";
    match mark {
        '\u{3099}' => {
            let special = match base {
                'う' => Some('ゔ'),
                'ウ' => Some('ヴ'),
                'ゝ' => Some('ゞ'),
                'ヽ' => Some('ヾ'),
                'ワ' => Some('ヷ'),
                'ヰ' => Some('ヸ'),
                'ヱ' => Some('ヹ'),
                'ヲ' => Some('ヺ'),
                _ => None,
            };
            if special.is_some() {
                special
            } else if PLUS_ONE.contains(base) {
                char::from_u32(u32::from(base) + 1)
            } else {
                None
            }
        }
        '\u{309A}' if PLUS_TWO.contains(base) => char::from_u32(u32::from(base) + 2),
        _ => None,
    }
}

/// Joins a kana and a following combining voiced (U+3099) or semi-voiced
/// (U+309A) mark. A mark that has no partner stays in the text.
pub fn compose_voicing(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut chars = text.chars().peekable();
    while let Some(c) = chars.next() {
        if let Some(v) = chars.peek().and_then(|&mark| voiced(c, mark)) {
            out.push(v);
            chars.next();
            continue;
        }
        out.push(c);
    }
    out
}

/// Katakana with a hiragana counterpart becomes that hiragana; all else is
/// unchanged.
pub fn katakana_to_hiragana(text: &str) -> String {
    text.chars()
        .map(|c| match c {
            '\u{30A1}'..='\u{30F6}' => char::from_u32(u32::from(c) - 0x60).unwrap_or(c),
            'ヽ' => 'ゝ',
            'ヾ' => 'ゞ',
            _ => c,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn japanese_characters_are_recognised() {
        for c in [
            'あ',
            'ア',
            'ー',
            '高',
            '々',
            '。',
            '\u{3099}',
            '\u{31F0}',
            '\u{20B9F}',
            '\u{E0100}',
        ] {
            assert!(is_japanese(c), "{c:?}");
        }
    }

    #[test]
    fn other_characters_are_not_japanese() {
        for c in [
            'a', 'Z', '1', '-', '\'', ' ', 'ō', '\u{FF54}', '\u{FF80}', '\u{FF01}', 'é', '😀',
        ] {
            assert!(!is_japanese(c), "{c:?}");
        }
    }

    #[test]
    fn katakana_folds_to_hiragana() {
        assert_eq!(katakana_to_hiragana("タカイ"), "たかい");
        assert_eq!(katakana_to_hiragana("コーヒー"), "こーひー");
        assert_eq!(katakana_to_hiragana("ヴ"), "ゔ");
        assert_eq!(katakana_to_hiragana("ッ"), "っ");
        assert_eq!(katakana_to_hiragana("ァ"), "ぁ");
        assert_eq!(katakana_to_hiragana("ヵ"), "ゕ");
        assert_eq!(katakana_to_hiragana("ヶ"), "ゖ");
        assert_eq!(katakana_to_hiragana("ヽヾ"), "ゝゞ");
    }

    #[test]
    fn other_text_is_not_folded() {
        for text in ["高い", "ひらがな", "ヷ", "ー・", "abc", ""] {
            assert_eq!(katakana_to_hiragana(text), text);
        }
    }

    #[test]
    fn combining_marks_join_their_kana() {
        assert_eq!(compose_voicing("か\u{3099}"), "が");
        assert_eq!(compose_voicing("つ\u{3099}"), "づ");
        assert_eq!(compose_voicing("ほ\u{309A}"), "ぽ");
        assert_eq!(compose_voicing("ハ\u{309A}"), "パ");
        assert_eq!(compose_voicing("う\u{3099}"), "ゔ");
        assert_eq!(compose_voicing("ウ\u{3099}"), "ヴ");
        assert_eq!(compose_voicing("ワ\u{3099}"), "ヷ");
        assert_eq!(compose_voicing("ヽ\u{3099}"), "ヾ");
        assert_eq!(compose_voicing("た\u{3099}かい"), "だかい");
    }

    #[test]
    fn marks_without_a_partner_stay() {
        for text in ["あ\u{3099}", "か\u{309A}", "\u{3099}", "a\u{3099}"] {
            assert_eq!(compose_voicing(text), text);
        }
    }

    #[test]
    fn composing_then_folding() {
        assert_eq!(
            katakana_to_hiragana(&compose_voicing("カ\u{3099}イ")),
            "がい"
        );
    }

    #[test]
    fn kanji_are_recognised() {
        for c in [
            '食',
            '高',
            '々',
            '〆',
            '〇',
            '\u{3400}',
            '\u{4DBF}',
            '\u{4E00}',
            '\u{9FFF}',
            '\u{F900}',
            '\u{FAFF}',
            '\u{20B9F}',
            '\u{3FFFF}',
        ] {
            assert!(is_kanji(c), "{c:?}");
            assert!(is_japanese(c), "{c:?}");
        }
    }

    #[test]
    fn other_characters_are_not_kanji() {
        for c in [
            'あ',
            'ア',
            'ー',
            '。',
            '、',
            '〜',
            '\u{3004}',
            '\u{3008}',
            '\u{3099}',
            '\u{E0100}',
            '\u{4DC0}',
            '\u{2E80}',
            'a',
            '\u{FF54}',
            '\u{FF80}',
            '😀',
        ] {
            assert!(!is_kanji(c), "{c:?}");
        }
    }
}
