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

/// The most `nn` spots that give two readings; later spots read textbook only.
const MAX_FORKS: usize = 4;

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

/// Romaji to hiragana, letter for letter. Every valid reading, textbook
/// reading first: `nn` before a vowel can be read two ways. `None` when the
/// text has no valid reading; `Some(v)` has 1 to 16 readings, all different.
pub fn to_hiragana(romaji: &str) -> Option<Vec<String>> {
    if !romaji.is_ascii() {
        return None;
    }
    let text = romaji.to_ascii_lowercase();
    let mut readings = vec![String::new()];
    let mut forks = 0;
    let mut i = 0;
    while i < text.len() {
        if text.as_bytes()[i] == b'-' {
            push_all(&mut readings, "ー");
            i += 1;
            continue;
        }
        let c = text.as_bytes()[i];
        let next = text.as_bytes().get(i + 1).copied();
        if c == b'n' {
            match next {
                Some(b'\'') => {
                    push_all(&mut readings, "ん");
                    i += 2;
                    continue;
                }
                Some(b'n') => {
                    let w = match text.as_bytes().get(i + 2) {
                        Some(b'a' | b'i' | b'u' | b'e' | b'o') => 1,
                        _ if matches!(text.get(i + 2..i + 4), Some("ya" | "yu" | "yo")) => 2,
                        _ => 0,
                    };
                    if w == 0 {
                        push_all(&mut readings, "ん");
                    } else {
                        // Two readings: textbook (`nn` is ん plus the `n` of
                        // an n-row syllable) and keyboard (`nn` is ん alone).
                        let textbook = text.get(i + 1..i + 2 + w).and_then(kana_of)?;
                        let keyboard = text.get(i + 2..i + 2 + w).and_then(kana_of)?;
                        let textbook = format!("ん{textbook}");
                        let keyboard = format!("ん{keyboard}");
                        if forks < MAX_FORKS {
                            forks += 1;
                            let mut forked = Vec::with_capacity(readings.len() * 2);
                            for reading in &readings {
                                forked.push(format!("{reading}{textbook}"));
                                forked.push(format!("{reading}{keyboard}"));
                            }
                            readings = forked;
                        } else {
                            push_all(&mut readings, &textbook);
                        }
                    }
                    i += 2 + w;
                    continue;
                }
                Some(b'a' | b'i' | b'u' | b'e' | b'o' | b'y') => {}
                _ => {
                    push_all(&mut readings, "ん");
                    i += 1;
                    continue;
                }
            }
        } else if (next == Some(c) && b"bcdfghjklmpqrstvwxyz".contains(&c))
            || (c == b't' && text.get(i + 1..i + 3) == Some("ch"))
        {
            push_all(&mut readings, "っ");
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

    #[test]
    fn doubled_consonant_gives_small_tsu() {
        for (romaji, kana) in [
            ("kitte", "きって"),
            ("zasshi", "ざっし"),
            ("ippai", "いっぱい"),
            ("motto", "もっと"),
            ("gakkou", "がっこう"),
        ] {
            assert_eq!(hira(romaji), one(kana), "{romaji}");
        }
    }

    #[test]
    fn small_tsu_before_ch() {
        assert_eq!(hira("matcha"), one("まっちゃ"));
        assert_eq!(hira("maccha"), one("まっちゃ"));
    }

    #[test]
    fn doubled_consonant_needs_a_syllable() {
        for romaji in ["kk", "tt", "kkx"] {
            assert_eq!(hira(romaji), None, "{romaji}");
        }
    }

    #[test]
    fn n_before_a_consonant_and_at_the_end() {
        for (romaji, kana) in [
            ("shinbun", "しんぶん"),
            ("hon", "ほん"),
            ("kantan", "かんたん"),
            ("byouin", "びょういん"),
            ("ra-men", "らーめん"),
            ("n", "ん"),
            ("n-", "んー"),
            ("vaiorin", "ゔぁいおりん"),
        ] {
            assert_eq!(hira(romaji), one(kana), "{romaji}");
        }
    }

    #[test]
    fn n_before_a_vowel_starts_a_syllable() {
        assert_eq!(hira("kinen"), one("きねん"));
        assert_eq!(hira("konya"), one("こにゃ"));
    }

    #[test]
    fn apostrophe_gives_n() {
        assert_eq!(hira("kin'en"), one("きんえん"));
        assert_eq!(hira("kon'ya"), one("こんや"));
        assert_eq!(hira("KIN'EN"), one("きんえん"));
        assert_eq!(hira("n'"), one("ん"));
    }

    #[test]
    fn nn_without_a_vowel_after_it() {
        for (romaji, kana) in [
            ("nn", "ん"),
            ("honn", "ほん"),
            ("nnnn", "んん"),
            ("shinnbun", "しんぶん"),
            ("onnye", "おんいぇ"),
        ] {
            assert_eq!(hira(romaji), one(kana), "{romaji}");
        }
    }

    #[test]
    fn three_n_before_a_vowel() {
        assert_eq!(hira("konnnichiha"), one("こんにちは"));
        assert_eq!(hira("onnna"), one("おんな"));
        assert_eq!(hira("konnnya"), one("こんにゃ"));
    }

    #[test]
    fn apostrophe_in_the_wrong_place_after_n() {
        for romaji in ["kan''i", "nn'a"] {
            assert_eq!(hira(romaji), None, "{romaji}");
        }
    }

    #[test]
    fn m_is_not_a_syllabic_n() {
        for romaji in ["shimbun", "sempai"] {
            assert_eq!(hira(romaji), None, "{romaji}");
        }
    }

    #[test]
    fn ny_without_a_vowel() {
        for romaji in ["ny", "nyk", "konnyk", "onnx"] {
            assert_eq!(hira(romaji), None, "{romaji}");
        }
    }

    fn many(kana: &[&str]) -> Option<Vec<String>> {
        Some(kana.iter().map(|k| (*k).to_string()).collect())
    }

    #[test]
    fn nn_before_a_vowel_has_two_readings() {
        assert_eq!(hira("konnichiha"), many(&["こんにちは", "こんいちは"]));
        assert_eq!(hira("KONNICHIHA"), many(&["こんにちは", "こんいちは"]));
        assert_eq!(hira("onna"), many(&["おんな", "おんあ"]));
        assert_eq!(hira("kinnen"), many(&["きんねん", "きんえん"]));
    }

    #[test]
    fn nn_before_ya_yu_yo_has_two_readings() {
        assert_eq!(hira("konnya"), many(&["こんにゃ", "こんや"]));
        assert_eq!(hira("onnyu"), many(&["おんにゅ", "おんゆ"]));
        assert_eq!(hira("konnyo"), many(&["こんにょ", "こんよ"]));
    }

    #[test]
    fn single_n_and_apostrophe_keep_one_reading() {
        assert_eq!(hira("kinen"), one("きねん"));
        assert_eq!(hira("kin'en"), one("きんえん"));
        assert_eq!(hira("kon'ya"), one("こんや"));
        assert_eq!(hira("konnnichiha"), one("こんにちは"));
        assert_eq!(hira("onnna"), one("おんな"));
    }

    #[test]
    fn two_spots_are_ordered_textbook_first() {
        assert_eq!(
            hira("kinnennen"),
            many(&[
                "きんねんねん",
                "きんねんえん",
                "きんえんねん",
                "きんえんえん"
            ])
        );
    }

    #[test]
    fn readings_are_bounded() {
        let four = hira(&"nna".repeat(4)).unwrap();
        assert_eq!(four.len(), 16);
        assert_eq!(four[0], "んなんなんなんな");
        assert_eq!(four[15], "んあんあんあんあ");
        let five = hira(&"nna".repeat(5)).unwrap();
        assert_eq!(five.len(), 16);
        assert_eq!(five[0], "んなんなんなんなんな");
        assert_eq!(five[15], "んあんあんあんあんな");
        let thirty = hira(&"nna".repeat(30)).unwrap();
        assert_eq!(thirty.len(), 16);
        for readings in [four, five, thirty] {
            let different: std::collections::HashSet<&String> = readings.iter().collect();
            assert_eq!(different.len(), 16);
        }
    }

    #[test]
    fn nn_spot_does_not_make_text_readable() {
        assert_eq!(hira("konnichiq"), None);
    }

    #[test]
    fn nn_is_never_small_tsu() {
        for reading in hira("onna").unwrap() {
            assert!(!reading.contains('っ'), "{reading}");
        }
        assert_eq!(hira("nn"), one("ん"));
    }
}
