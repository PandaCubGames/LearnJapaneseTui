// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 PandaCubGames

//! Romaji to hiragana, letter for letter, the way a Japanese IME reads it.

/// Every romaji spelling and its kana. Each spelling has its own pair.
const TABLE: &[(&str, &str)] = &[
    // vowels
    ("a", "あ"),
    ("i", "い"),
    ("u", "う"),
    ("e", "え"),
    ("o", "お"),
    // k
    ("ka", "か"),
    ("ki", "き"),
    ("ku", "く"),
    ("ke", "け"),
    ("ko", "こ"),
    // g
    ("ga", "が"),
    ("gi", "ぎ"),
    ("gu", "ぐ"),
    ("ge", "げ"),
    ("go", "ご"),
    // s
    ("sa", "さ"),
    ("shi", "し"),
    ("si", "し"),
    ("su", "す"),
    ("se", "せ"),
    ("so", "そ"),
    // z
    ("za", "ざ"),
    ("ji", "じ"),
    ("zi", "じ"),
    ("zu", "ず"),
    ("ze", "ぜ"),
    ("zo", "ぞ"),
    // t
    ("ta", "た"),
    ("chi", "ち"),
    ("ti", "ち"),
    ("tsu", "つ"),
    ("tu", "つ"),
    ("te", "て"),
    ("to", "と"),
    // d
    ("da", "だ"),
    ("di", "ぢ"),
    ("du", "づ"),
    ("dzu", "づ"),
    ("de", "で"),
    ("do", "ど"),
    // n
    ("na", "な"),
    ("ni", "に"),
    ("nu", "ぬ"),
    ("ne", "ね"),
    ("no", "の"),
    // h
    ("ha", "は"),
    ("hi", "ひ"),
    ("fu", "ふ"),
    ("hu", "ふ"),
    ("he", "へ"),
    ("ho", "ほ"),
    // b
    ("ba", "ば"),
    ("bi", "び"),
    ("bu", "ぶ"),
    ("be", "べ"),
    ("bo", "ぼ"),
    // p
    ("pa", "ぱ"),
    ("pi", "ぴ"),
    ("pu", "ぷ"),
    ("pe", "ぺ"),
    ("po", "ぽ"),
    // m
    ("ma", "ま"),
    ("mi", "み"),
    ("mu", "む"),
    ("me", "め"),
    ("mo", "も"),
    // y
    ("ya", "や"),
    ("yu", "ゆ"),
    ("yo", "よ"),
    // r
    ("ra", "ら"),
    ("ri", "り"),
    ("ru", "る"),
    ("re", "れ"),
    ("ro", "ろ"),
    // w
    ("wa", "わ"),
    ("wo", "を"),
    // ky, gy
    ("kya", "きゃ"),
    ("kyu", "きゅ"),
    ("kyo", "きょ"),
    ("gya", "ぎゃ"),
    ("gyu", "ぎゅ"),
    ("gyo", "ぎょ"),
    // sh
    ("sha", "しゃ"),
    ("sya", "しゃ"),
    ("shu", "しゅ"),
    ("syu", "しゅ"),
    ("sho", "しょ"),
    ("syo", "しょ"),
    ("she", "しぇ"),
    ("sye", "しぇ"),
    // j
    ("ja", "じゃ"),
    ("jya", "じゃ"),
    ("zya", "じゃ"),
    ("ju", "じゅ"),
    ("jyu", "じゅ"),
    ("zyu", "じゅ"),
    ("jo", "じょ"),
    ("jyo", "じょ"),
    ("zyo", "じょ"),
    ("je", "じぇ"),
    ("jye", "じぇ"),
    ("zye", "じぇ"),
    // ch
    ("cha", "ちゃ"),
    ("tya", "ちゃ"),
    ("chu", "ちゅ"),
    ("tyu", "ちゅ"),
    ("cho", "ちょ"),
    ("tyo", "ちょ"),
    ("che", "ちぇ"),
    ("tye", "ちぇ"),
    // dy
    ("dya", "ぢゃ"),
    ("dyu", "ぢゅ"),
    ("dyo", "ぢょ"),
    // ny, hy
    ("nya", "にゃ"),
    ("nyu", "にゅ"),
    ("nyo", "にょ"),
    ("hya", "ひゃ"),
    ("hyu", "ひゅ"),
    ("hyo", "ひょ"),
    // by, py
    ("bya", "びゃ"),
    ("byu", "びゅ"),
    ("byo", "びょ"),
    ("pya", "ぴゃ"),
    ("pyu", "ぴゅ"),
    ("pyo", "ぴょ"),
    // my, ry
    ("mya", "みゃ"),
    ("myu", "みゅ"),
    ("myo", "みょ"),
    ("rya", "りゃ"),
    ("ryu", "りゅ"),
    ("ryo", "りょ"),
    // f
    ("fa", "ふぁ"),
    ("fi", "ふぃ"),
    ("fe", "ふぇ"),
    ("fo", "ふぉ"),
    // v
    ("va", "ゔぁ"),
    ("vi", "ゔぃ"),
    ("vu", "ゔ"),
    ("ve", "ゔぇ"),
    ("vo", "ゔぉ"),
    // ts
    ("tsa", "つぁ"),
    ("tsi", "つぃ"),
    ("tse", "つぇ"),
    ("tso", "つぉ"),
    // other
    ("thi", "てぃ"),
    ("dhi", "でぃ"),
    ("wi", "うぃ"),
    ("we", "うぇ"),
    ("ye", "いぇ"),
    // small
    ("xa", "ぁ"),
    ("la", "ぁ"),
    ("xi", "ぃ"),
    ("li", "ぃ"),
    ("xu", "ぅ"),
    ("lu", "ぅ"),
    ("xe", "ぇ"),
    ("le", "ぇ"),
    ("xo", "ぉ"),
    ("lo", "ぉ"),
    ("xya", "ゃ"),
    ("lya", "ゃ"),
    ("xyu", "ゅ"),
    ("lyu", "ゅ"),
    ("xyo", "ょ"),
    ("lyo", "ょ"),
    ("xtu", "っ"),
    ("ltu", "っ"),
    ("xtsu", "っ"),
    ("ltsu", "っ"),
    ("xwa", "ゎ"),
    ("lwa", "ゎ"),
    ("xka", "ゕ"),
    ("lka", "ゕ"),
    ("xke", "ゖ"),
    ("lke", "ゖ"),
];

/// The kana of the table entry whose key is exactly `key`.
fn kana_of(key: &str) -> Option<&'static str> {
    TABLE
        .iter()
        .find(|(romaji, _)| *romaji == key)
        .map(|(_, kana)| *kana)
}

/// Appends `kana` to every reading.
fn push_all(readings: &mut [String], kana: &str) {
    for reading in readings.iter_mut() {
        reading.push_str(kana);
    }
}

/// Romaji to hiragana, letter for letter. `None` when the text has no valid
/// reading.
pub fn to_hiragana(romaji: &str) -> Option<Vec<String>> {
    if !romaji.is_ascii() {
        return None;
    }
    let text = romaji.to_ascii_lowercase();
    let mut readings = vec![String::new()];
    let mut i = 0;
    while i < text.len() {
        if text.as_bytes()[i] == b'-' {
            push_all(&mut readings, "ー");
            i += 1;
            continue;
        }
        let mut found = None;
        for len in (1..=4).rev() {
            if let Some(kana) = text.get(i..i + len).and_then(kana_of) {
                found = Some((len, kana));
                break;
            }
        }
        let (len, kana) = found?;
        push_all(&mut readings, kana);
        i += len;
    }
    Some(readings)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn hira(romaji: &str) -> Option<Vec<String>> {
        to_hiragana(romaji)
    }

    fn one(kana: &str) -> Option<Vec<String>> {
        Some(vec![kana.to_string()])
    }

    #[test]
    fn table_has_every_spelling_once() {
        assert_eq!(TABLE.len(), 175);
        let keys: std::collections::HashSet<&str> = TABLE.iter().map(|(key, _)| *key).collect();
        assert_eq!(keys.len(), 175);
    }

    #[test]
    fn plain_syllables() {
        assert_eq!(hira("takai"), one("たかい"));
        assert_eq!(hira("taberu"), one("たべる"));
        assert_eq!(hira("sakana"), one("さかな"));
        assert_eq!(hira("hiragana"), one("ひらがな"));
    }

    #[test]
    fn every_row() {
        for (romaji, kana) in [
            ("aiueo", "あいうえお"),
            ("kakikukeko", "かきくけこ"),
            ("gagigugego", "がぎぐげご"),
            ("sashisuseso", "さしすせそ"),
            ("zajizuzezo", "ざじずぜぞ"),
            ("tachitsuteto", "たちつてと"),
            ("dadidudedo", "だぢづでど"),
            ("naninuneno", "なにぬねの"),
            ("hahifuheho", "はひふへほ"),
            ("babibubebo", "ばびぶべぼ"),
            ("papipupepo", "ぱぴぷぺぽ"),
            ("mamimumemo", "まみむめも"),
            ("yayuyo", "やゆよ"),
            ("rarirurero", "らりるれろ"),
            ("wawo", "わを"),
        ] {
            assert_eq!(hira(romaji), one(kana), "{romaji}");
        }
    }

    #[test]
    fn combinations_with_small_kana() {
        for (romaji, kana) in [
            ("kyou", "きょう"),
            ("ryokou", "りょこう"),
            ("nyuugaku", "にゅうがく"),
            ("myaku", "みゃく"),
            ("hyaku", "ひゃく"),
            ("gyuu", "ぎゅう"),
            ("byou", "びょう"),
            ("pyuu", "ぴゅう"),
            ("dya", "ぢゃ"),
        ] {
            assert_eq!(hira(romaji), one(kana), "{romaji}");
        }
    }

    #[test]
    fn typed_variants_give_the_same_kana() {
        for (a, b, kana) in [
            ("shi", "si", "し"),
            ("chi", "ti", "ち"),
            ("tsu", "tu", "つ"),
            ("fu", "hu", "ふ"),
            ("ji", "zi", "じ"),
            ("sha", "sya", "しゃ"),
            ("cha", "tya", "ちゃ"),
            ("shu", "syu", "しゅ"),
            ("cho", "tyo", "ちょ"),
        ] {
            assert_eq!(hira(a), one(kana), "{a}");
            assert_eq!(hira(b), one(kana), "{b}");
        }
    }

    #[test]
    fn three_spellings_of_ja() {
        for romaji in ["ja", "jya", "zya"] {
            assert_eq!(hira(romaji), one("じゃ"), "{romaji}");
        }
        for romaji in ["ju", "jyu", "zyu"] {
            assert_eq!(hira(romaji), one("じゅ"), "{romaji}");
        }
        for romaji in ["jo", "jyo", "zyo"] {
            assert_eq!(hira(romaji), one("じょ"), "{romaji}");
        }
    }

    #[test]
    fn kana_that_are_easy_to_confuse() {
        for (romaji, kana) in [
            ("wo", "を"),
            ("o", "お"),
            ("di", "ぢ"),
            ("du", "づ"),
            ("dzu", "づ"),
            ("ji", "じ"),
            ("zu", "ず"),
            ("ha", "は"),
            ("wa", "わ"),
        ] {
            assert_eq!(hira(romaji), one(kana), "{romaji}");
        }
    }

    #[test]
    fn loanword_syllables() {
        for (romaji, kana) in [
            ("fairu", "ふぁいる"),
            ("vu", "ゔ"),
            ("thi", "てぃ"),
            ("dhi", "でぃ"),
            ("tsa", "つぁ"),
            ("wi", "うぃ"),
            ("we", "うぇ"),
            ("ye", "いぇ"),
            ("she", "しぇ"),
            ("je", "じぇ"),
            ("che", "ちぇ"),
        ] {
            assert_eq!(hira(romaji), one(kana), "{romaji}");
        }
    }

    #[test]
    fn small_kana_spelled_out() {
        for (romaji, kana) in [
            ("xa", "ぁ"),
            ("la", "ぁ"),
            ("xtu", "っ"),
            ("ltsu", "っ"),
            ("xya", "ゃ"),
            ("lyo", "ょ"),
            ("xwa", "ゎ"),
            ("xka", "ゕ"),
            ("lke", "ゖ"),
        ] {
            assert_eq!(hira(romaji), one(kana), "{romaji}");
        }
    }

    #[test]
    fn hyphen_is_the_long_vowel_mark() {
        assert_eq!(hira("ko-hi-"), one("こーひー"));
        assert_eq!(hira("-"), one("ー"));
    }

    #[test]
    fn capitals_are_read_as_lower_case() {
        assert_eq!(hira("TAKAI"), one("たかい"));
        assert_eq!(hira("Takai"), one("たかい"));
        assert_eq!(hira("ShI"), one("し"));
    }

    #[test]
    fn empty_text_is_empty() {
        assert_eq!(hira(""), Some(vec![String::new()]));
    }

    #[test]
    fn letters_that_spell_no_kana() {
        for romaji in ["tabxru", "tabwru", "ca", "qa", "wu", "yi", "x", "l"] {
            assert_eq!(hira(romaji), None, "{romaji}");
        }
    }

    #[test]
    fn trailing_lone_consonant() {
        for romaji in ["tabek", "ky", "k"] {
            assert_eq!(hira(romaji), None, "{romaji}");
        }
    }

    #[test]
    fn characters_that_are_not_romaji() {
        for romaji in ["taka1", "taka i", "taka.", "tōkyō", "たかい", "ｔａｋａｉ"] {
            assert_eq!(hira(romaji), None, "{romaji}");
        }
    }

    #[test]
    fn apostrophe_not_after_n() {
        for romaji in ["'takai", "ta'kai", "'"] {
            assert_eq!(hira(romaji), None, "{romaji}");
        }
    }
}
