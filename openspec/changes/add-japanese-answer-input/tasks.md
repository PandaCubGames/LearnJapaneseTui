# Tasks

Types, signatures, tables and algorithms are in `design.md` (D-numbers point there). Every task is one commit (`AGENTS.md` §5): after each one, `cargo fmt --all -- --check`, `cargo clippy --all-targets -- -D warnings` and `cargo test` must pass. Do not change the project version (`Cargo.toml`, `VERSION`, `Cargo.lock`), and do not add dependencies: `git diff main -- Cargo.toml Cargo.lock VERSION` must stay empty.

**New files** start with the SPDX header, exactly as the existing files do, followed by a `//!` module comment:

```rust
// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 PandaCubGames
```

**Existing tests:** do not change, rename, move, skip, `#[ignore]` or delete any existing test. None of `src/app.rs`, `src/keys.rs`, `src/session.rs`, `src/text.rs` and `src/ui.rs` is edited by this change. The baseline is 51 passing tests; after every task all 51 still pass, plus the new ones. If an existing test fails, fix the new code; if that is impossible, stop and report.

**Unused-code rule (D1, approved by the owner):** every `mod` line this change adds to `src/main.rs` carries this attribute and comment, with the module name changed:

```rust
// No screen uses this module yet: the practice question screen (issue #21)
// does, and removes this attribute. Tests use every item already.
#[cfg_attr(
    not(test),
    expect(dead_code, reason = "used by the practice question screen, issue #21")
)]
mod kana;
```

Keep the `mod` declarations in alphabetical order and let `cargo fmt` format them. Because of `cfg_attr(not(test), ...)`, **every item in the new modules must be used by at least one test**; otherwise `cargo clippy --all-targets` warns in the test build. Do not use `#[allow(...)]` anywhere, and do not add any other lint attribute.

In test modules, use `use super::*;` like the existing files. Use `assert_eq!` with exact values.

## 1. Kana helpers

- [x] 1.1 Add `src/kana.rs` with `is_japanese`, `is_kanji`, `compose_voicing` and `katakana_to_hiragana` (D2 signatures, D5 rules).
  - `src/main.rs`: add `mod kana;` with the D1 attribute.
  - `pub fn is_japanese(c: char) -> bool`: `matches!` on the exact ranges of the D5 table.
  - `pub fn is_kanji(c: char) -> bool`: `matches!` on the exact ranges of the D5 `is_kanji` table: `'\u{3005}'..='\u{3007}' | '\u{3400}'..='\u{4DBF}' | '\u{4E00}'..='\u{9FFF}' | '\u{F900}'..='\u{FAFF}' | '\u{20000}'..='\u{3FFFF}'`. Doc comment as in D2.
  - `pub fn compose_voicing(text: &str) -> String`: walk the `char`s with a peekable iterator; when the next char is U+3099 or U+309A and a private `fn voiced(base: char, mark: char) -> Option<char>` returns `Some(v)`, push `v` and skip the mark; else push the char. `voiced` implements the D5 list exactly (the +1 bases, the +2 bases, and the listed special pairs). A mark that does not combine stays in the text.
  - `pub fn katakana_to_hiragana(text: &str) -> String`: map each char per D5 (U+30A1..=U+30F6 minus 0x60, `ヽ`→`ゝ`, `ヾ`→`ゞ`, all else unchanged). Use `char::from_u32(..)` and fall back to the original char if it ever returned `None`; do not `unwrap`.
  - New tests in `src/kana.rs` `mod tests`:
    - `japanese_characters_are_recognised`: `is_japanese` is true for `'あ'`, `'ア'`, `'ー'`, `'高'`, `'々'`, `'。'`, `'\u{3099}'`, `'ㇰ'` (U+31F0), `'\u{20B9F}'` and `'\u{E0100}'`.
    - `other_characters_are_not_japanese`: false for `'a'`, `'Z'`, `'1'`, `'-'`, `'\''`, `' '`, `'ō'`, `'ｔ'` (U+FF54), `'ﾀ'` (U+FF80), `'！'` (U+FF01), `'é'` and `'😀'`.
    - `katakana_folds_to_hiragana`: `タカイ`→`たかい`, `コーヒー`→`こーひー`, `ヴ`→`ゔ`, `ッ`→`っ`, `ァ`→`ぁ`, `ヵ`→`ゕ`, `ヶ`→`ゖ`, `ヽヾ`→`ゝゞ`.
    - `other_text_is_not_folded`: `高い`, `ひらがな`, `ヷ`, `ー・`, `abc` and the empty text come back unchanged.
    - `combining_marks_join_their_kana`: `"か\u{3099}"`→`が`, `"つ\u{3099}"`→`づ`, `"ほ\u{309A}"`→`ぽ`, `"ハ\u{309A}"`→`パ`, `"う\u{3099}"`→`ゔ`, `"ウ\u{3099}"`→`ヴ`, `"ワ\u{3099}"`→`ヷ`, `"ヽ\u{3099}"`→`ヾ`, `"た\u{3099}かい"`→`だかい`.
    - `marks_without_a_partner_stay`: `"あ\u{3099}"`, `"か\u{309A}"`, `"\u{3099}"` and `"a\u{3099}"` come back unchanged.
    - `composing_then_folding`: `katakana_to_hiragana(&compose_voicing("カ\u{3099}イ")) == "がい"`.
    - `kanji_are_recognised`: `is_kanji` is true for `'食'`, `'高'`, `'々'` (U+3005), `'〆'` (U+3006), `'〇'` (U+3007), `'\u{3400}'`, `'\u{4DBF}'`, `'\u{4E00}'`, `'\u{9FFF}'`, `'\u{F900}'`, `'\u{FAFF}'`, `'\u{20B9F}'` and `'\u{3FFFF}'`; and `is_japanese` is also true for each of them.
    - `other_characters_are_not_kanji`: `is_kanji` is false for `'あ'`, `'ア'`, `'ー'`, `'。'`, `'、'`, `'〜'`, `'\u{3004}'`, `'\u{3008}'`, `'\u{3099}'`, `'\u{E0100}'`, `'\u{4DC0}'` (just after Extension A), `'\u{2E80}'` (a CJK radical), `'a'`, `'ｔ'` (U+FF54), `'ﾀ'` (U+FF80) and `'😀'`.
  - Verify: the 9 new tests pass, 51 existing tests pass, fmt and clippy are clean. Commit, e.g. `feat(kana): add katakana folding and voiced-mark composition`.

## 2. Romaji to hiragana

- [x] 2.1 Add `src/romaji.rs` with the table and the basic conversion (D4 table; D4 algorithm steps 1, 2, 3.1, 3.5 and 4 only).
  - `src/main.rs`: add `mod romaji;` with the D1 attribute.
  - `const TABLE: &[(&str, &str)]`: every spelling of the D4 table as its own pair, 175 pairs, in the order of the D4 table. Keys are lower-case ASCII.
  - Private `fn kana_of(key: &str) -> Option<&'static str>`: the kana of the table entry whose key equals `key` exactly, or `None`. The longest-match lookup below calls it; task 2.3 calls it too.
  - Private `fn push_all(readings: &mut [String], kana: &str)`: appends `kana` to every reading.
  - `pub fn to_hiragana(romaji: &str) -> Option<Vec<String>>` (the final signature, D2; the list of readings): return `None` if `!romaji.is_ascii()`; lower-case with `to_ascii_lowercase`; start with `let mut readings = vec![String::new()];`; walk with a byte index; `-` pushes `ー` (with `push_all`); otherwise try table keys of length 4, 3, 2, 1 at the current position (use `str::get(i..i + len)` so a short tail never panics, then `kana_of`) and push the first match with `push_all`; no match returns `None`; at the end return `Some(readings)`. In this task the list always has exactly one reading. There is **no** special `n` rule and **no** っ rule yet (task 2.2): a lone `n` and a doubled consonant simply find no key.
  - New tests in `src/romaji.rs` `mod tests` (helpers `fn hira(romaji: &str) -> Option<Vec<String>>` that calls `to_hiragana`, and `fn one(kana: &str) -> Option<Vec<String>> { Some(vec![kana.to_string()]) }`; below, `x`→`か` means `assert_eq!(hira("x"), one("か"))`, and `None` means `assert_eq!(hira("x"), None)`):
    - `table_has_every_spelling_once`: `TABLE.len() == 175`, and no key appears twice (collect keys into a `std::collections::HashSet` and compare its length with 175).
    - `plain_syllables`: `takai`→`たかい`, `taberu`→`たべる`, `sakana`→`さかな`, `hiragana`→`ひらがな`.
    - `every_row`: `aiueo`→`あいうえお`, `kakikukeko`→`かきくけこ`, `gagigugego`→`がぎぐげご`, `sashisuseso`→`さしすせそ`, `zajizuzezo`→`ざじずぜぞ`, `tachitsuteto`→`たちつてと`, `dadidudedo`→`だぢづでど`, `naninuneno`→`なにぬねの`, `hahifuheho`→`はひふへほ`, `babibubebo`→`ばびぶべぼ`, `papipupepo`→`ぱぴぷぺぽ`, `mamimumemo`→`まみむめも`, `yayuyo`→`やゆよ`, `rarirurero`→`らりるれろ`, `wawo`→`わを`.
    - `combinations_with_small_kana`: `kyou`→`きょう`, `ryokou`→`りょこう`, `nyuugaku`→`にゅうがく`, `myaku`→`みゃく`, `hyaku`→`ひゃく`, `gyuu`→`ぎゅう`, `byou`→`びょう`, `pyuu`→`ぴゅう`, `dya`→`ぢゃ`.
    - `typed_variants_give_the_same_kana`: for each pair (`shi`,`si`)→`し`, (`chi`,`ti`)→`ち`, (`tsu`,`tu`)→`つ`, (`fu`,`hu`)→`ふ`, (`ji`,`zi`)→`じ`, (`sha`,`sya`)→`しゃ`, (`cha`,`tya`)→`ちゃ`, (`shu`,`syu`)→`しゅ`, (`cho`,`tyo`)→`ちょ`, both spellings give the kana.
    - `three_spellings_of_ja`: `ja`, `jya` and `zya` → `じゃ`; `ju`, `jyu`, `zyu` → `じゅ`; `jo`, `jyo`, `zyo` → `じょ`.
    - `kana_that_are_easy_to_confuse`: `wo`→`を`, `o`→`お`, `di`→`ぢ`, `du`→`づ`, `dzu`→`づ`, `ji`→`じ`, `zu`→`ず`, `ha`→`は`, `wa`→`わ`.
    - `loanword_syllables`: `fairu`→`ふぁいる`, `vu`→`ゔ`, `thi`→`てぃ`, `dhi`→`でぃ`, `tsa`→`つぁ`, `wi`→`うぃ`, `we`→`うぇ`, `ye`→`いぇ`, `she`→`しぇ`, `je`→`じぇ`, `che`→`ちぇ`.
    - `small_kana_spelled_out`: `xa`→`ぁ`, `la`→`ぁ`, `xtu`→`っ`, `ltsu`→`っ`, `xya`→`ゃ`, `lyo`→`ょ`, `xwa`→`ゎ`, `xka`→`ゕ`, `lke`→`ゖ`.
    - `hyphen_is_the_long_vowel_mark`: `ko-hi-`→`こーひー`, `-`→`ー`.
    - `capitals_are_read_as_lower_case`: `TAKAI`→`たかい`, `Takai`→`たかい`, `ShI`→`し`.
    - `empty_text_is_empty`: `""` → `Some(vec![String::new()])` (one reading, empty).
    - `letters_that_spell_no_kana`: `tabxru`, `tabwru`, `ca`, `qa`, `wu`, `yi`, `x`, `l` → `None`.
    - `trailing_lone_consonant`: `tabek`, `ky`, `k` → `None`.
    - `characters_that_are_not_romaji`: `taka1`, `taka i`, `taka.`, `tōkyō`, `たかい`, `ｔａｋａｉ` → `None`.
    - `apostrophe_not_after_n`: `'takai`, `ta'kai`, `'` → `None`.
  - Verify: the 16 new tests pass, all earlier tests pass, fmt and clippy are clean. Commit, e.g. `feat(romaji): convert romaji syllables to hiragana`.

- [x] 2.2 Add the っ and ん rules to `to_hiragana` (D4 algorithm steps 3.2 **without** the two-reading spot, 3.3 and 3.4, checked in that order before the table lookup). Every result still has exactly one reading.
  - `n`: next is `'` → push `ん`, advance 2; next is `n` → push `ん`, advance 2 (in this task always; task 2.3 adds the two-reading spot); next is none, or not one of `a i u e o y` → push `ん`, advance 1; otherwise fall through to the table lookup.
  - Doubled consonant: current is one of `bcdfghjklmpqrstvwxyz` (not `n`, not a vowel) and next is the same letter → push `っ`, advance 1.
  - `t` followed by `ch` → push `っ`, advance 1.
  - Do not edit any test from task 2.1; they must all still pass.
  - New tests in `src/romaji.rs` `mod tests`, same notation as 2.1 (`x`→`か` means `hira("x") == one("か")`). On purpose, none of them uses `nn` directly before a vowel or `ya`/`yu`/`yo`: task 2.3 gives those two readings, and a test must never need changing.
    - `doubled_consonant_gives_small_tsu`: `kitte`→`きって`, `zasshi`→`ざっし`, `ippai`→`いっぱい`, `motto`→`もっと`, `gakkou`→`がっこう`.
    - `small_tsu_before_ch`: `matcha`→`まっちゃ`, `maccha`→`まっちゃ`.
    - `doubled_consonant_needs_a_syllable`: `kk`, `tt`, `kkx` → `None`.
    - `n_before_a_consonant_and_at_the_end`: `shinbun`→`しんぶん`, `hon`→`ほん`, `kantan`→`かんたん`, `byouin`→`びょういん`, `ra-men`→`らーめん`, `n`→`ん`, `n-`→`んー`, `vaiorin`→`ゔぁいおりん`.
    - `n_before_a_vowel_starts_a_syllable`: `kinen`→`きねん`, `konya`→`こにゃ`.
    - `apostrophe_gives_n`: `kin'en`→`きんえん`, `kon'ya`→`こんや`, `KIN'EN`→`きんえん`, `n'`→`ん`.
    - `nn_without_a_vowel_after_it`: `nn`→`ん`, `honn`→`ほん`, `nnnn`→`んん`, `shinnbun`→`しんぶん`, `onnye`→`おんいぇ`.
    - `three_n_before_a_vowel`: `konnnichiha`→`こんにちは`, `onnna`→`おんな`, `konnnya`→`こんにゃ`.
    - `apostrophe_in_the_wrong_place_after_n`: `kan''i`, `nn'a` → `None`.
    - `m_is_not_a_syllabic_n`: `shimbun`, `sempai` → `None`.
    - `ny_without_a_vowel`: `ny`, `nyk`, `konnyk`, `onnx` → `None`.
  - Verify: the 11 new tests pass, all earlier tests (including every test of 2.1, unchanged) pass, fmt and clippy are clean. Commit, e.g. `feat(romaji): read small tsu and syllabic n`.

- [x] 2.3 Give `nn` before a vowel or `ya`/`yu`/`yo` both readings (owner decision; D4 algorithm step 3.2, the two-reading spot, *Two readings of `nn`*, order and bound).
  - `src/romaji.rs`: add `const MAX_FORKS: usize = 4;` and a `let mut forks = 0;` counter in `to_hiragana`.
  - In the `n` + `n` branch from 2.2, before pushing `ん`: let `w` be 1 if the byte at `i + 2` is one of `a i u e o`, 2 if `text.get(i + 2..i + 4)` is `Some("ya")`, `Some("yu")` or `Some("yo")`, else 0. If `w == 0`, keep 2.2's behaviour (push `ん`, advance 2). If `w > 0`:
    - `textbook` = `"ん"` followed by `kana_of(&text[i + 1..i + 2 + w])`; `keyboard` = `"ん"` followed by `kana_of(&text[i + 2..i + 2 + w])`. Both keys are always in the table; if `kana_of` ever returns `None`, return `None` (no `unwrap`, no panic).
    - If `forks < MAX_FORKS`: `forks += 1`, and build a new `Vec<String>` with capacity `readings.len() * 2` that holds, for each old reading `r` in order, `r + textbook` then `r + keyboard`; it replaces `readings`. Otherwise `push_all(&mut readings, &textbook)` only.
    - Advance `2 + w`.
  - Update the doc comment of `to_hiragana` to the D2 text.
  - Do not edit any test from tasks 2.1 and 2.2; they must all still pass.
  - New tests in `src/romaji.rs` `mod tests` (helper `fn many(kana: &[&str]) -> Option<Vec<String>>` that maps each to `String` and wraps in `Some`; compare with `assert_eq!`):
    - `nn_before_a_vowel_has_two_readings`: `konnichiha` → `many(&["こんにちは", "こんいちは"])`; `KONNICHIHA` → the same; `onna` → `many(&["おんな", "おんあ"])`; `kinnen` → `many(&["きんねん", "きんえん"])`.
    - `nn_before_ya_yu_yo_has_two_readings`: `konnya` → `many(&["こんにゃ", "こんや"])`; `onnyu` → `many(&["おんにゅ", "おんゆ"])`; `konnyo` → `many(&["こんにょ", "こんよ"])`.
    - `single_n_and_apostrophe_keep_one_reading`: `kinen` → `one("きねん")`; `kin'en` → `one("きんえん")`; `kon'ya` → `one("こんや")`; `konnnichiha` → `one("こんにちは")`; `onnna` → `one("おんな")`.
    - `two_spots_are_ordered_textbook_first`: `kinnennen` → `many(&["きんねんねん", "きんねんえん", "きんえんねん", "きんえんえん"])`.
    - `readings_are_bounded`: for `"nna".repeat(4)`: 16 readings, the first `んなんなんなんな`, the last `んあんあんあんあ`; for `"nna".repeat(5)`: 16 readings, the first `んなんなんなんなんな`, the last `んあんあんあんあんな`; for `"nna".repeat(30)`: 16 readings. In each case, collecting the readings into a `HashSet` gives 16 (all different).
    - `nn_spot_does_not_make_text_readable`: `konnichiq` → `None`.
    - `nn_is_never_small_tsu`: `onna` → readings contain no `っ` (`!r.contains('っ')` for every reading), and `nn` → `one("ん")`.
  - Verify: the 7 new tests pass (at least `nn_before_a_vowel_has_two_readings`, `nn_before_ya_yu_yo_has_two_readings`, `two_spots_are_ordered_textbook_first` and `readings_are_bounded` fail without this task's code), all earlier tests pass unchanged, fmt and clippy are clean. Commit, e.g. `feat(romaji): accept both readings of nn before a vowel`.

## 3. Answer checking

- [x] 3.1 Add `src/grading.rs` with `Outcome` and `check` (D2, D6).
  - `src/main.rs`: add `mod grading;` with the D1 attribute.
  - `Outcome` exactly as in D2, with its derives and doc comments.
  - Private helpers: `fn normalise_japanese(text: &str) -> String`, `fn blocked(shown: &[&str]) -> Vec<String>` and `fn is_marked_vowel(c: char) -> bool` as in D6. Remove whitespace with `text.chars().filter(|c| !c.is_whitespace())`. `blocked` keeps `normalise_japanese(s)` for each `s` whose normalised text satisfies `.chars().any(kana::is_kanji)`, in order.
  - `pub fn check(typed: &str, accepted: &[&str], shown: &[&str]) -> Outcome`: steps 1 to 3 of D6, in that order. Use `crate::kana` and `crate::romaji`. The repeat rule is checked **only** in the Japanese branch (D6 step 2.2): if the normalised answer equals a text of `blocked(shown)`, return `Wrong { read_as: Some(normalised answer) }`. Do not compare romaji readings with `shown` at all (they are kana and can never equal a text with a kanji). Then work with a `Vec<String>` of readings: one for a Japanese answer, the list from `romaji::to_hiragana` for romaji (an empty list is treated like `None`). Step 3 checks `accepted` (correct, `read_as` = the first reading in list order that matches), and otherwise returns `Wrong { read_as: Some(first reading) }`, the textbook reading. Take the first reading with `readings.first()` or `into_iter().next()`, never by indexing.
  - New tests in `src/grading.rs` `mod tests` (define `const NOTHING: &[&str] = &[];` and helpers `fn correct(r: &str) -> Outcome { Outcome::Correct { read_as: r.to_string() } }` and `fn wrong(r: &str) -> Outcome { Outcome::Wrong { read_as: Some(r.to_string()) } }`; `const UNREAD: Outcome = Outcome::Wrong { read_as: None };` will not compile because of `String`, so write `Outcome::Wrong { read_as: None }` in full):
    - `four_ways_to_answer_takai`: accepted `["高い", "たかい"]`, shown `NOTHING`: `takai`→`correct("たかい")`, `たかい`→`correct("たかい")`, `タカイ`→`correct("たかい")`, `高い`→`correct("高い")`.
    - `katakana_accepted_answer`: accepted `["ショク"]`: `しょく`, `ショク`, `shoku` → `correct("しょく")`.
    - `capital_romaji_is_correct`: accepted `["たかい"]`: `TAKAI` → `correct("たかい")`.
    - `long_vowel_typed_out_is_correct`: accepted `["東京", "とうきょう"]`: `toukyou` → `correct("とうきょう")`.
    - `tokyo_without_long_vowels_is_wrong`: same accepted: `tokyo` → `wrong("ときょ")`.
    - `macron_romaji_is_wrong`: same accepted: `tōkyō`, `TŌKYŌ`, `tôkyô` → `Outcome::Wrong { read_as: None }`; accepted `["ここ"]`: `kōkō` → `Outcome::Wrong { read_as: None }`.
    - `mixed_scripts_are_wrong`: accepted `["食べる", "たべる"]`: `食beru`, `たberu`, `食beru1` → `Outcome::Wrong { read_as: None }`.
    - `shown_kanji_is_never_accepted`: shown `["食"]`, accepted `["食", "しょく", "たべる"]`: `食` → `wrong("食")`; `しょく` → `correct("しょく")`; `たべる` → `correct("たべる")`; `taberu` → `correct("たべる")`.
    - `copying_a_shown_word_with_kanji_is_wrong`: shown `["食べる"]`, accepted `["食べる", "たべる"]`: `食べる`, `"食 べる"` and `食ベル` → `wrong("食べる")`; `たべる`, `タベル` and `taberu` → `correct("たべる")`.
    - `kana_shown_text_never_makes_an_answer_wrong`: shown `["する"]`, accepted `["する"]`: `する`, `スル` and `suru` → `correct("する")`; shown `["たかい"]`, accepted `["高い", "たかい"]`: `takai`, `タカイ` and `たかい` → `correct("たかい")`, `高い` → `correct("高い")`.
    - `romaji_is_never_a_repeat`: shown `["今夜", "こんや"]`, accepted `["今夜", "こんや"]`: `konnya` and `kon'ya` → `correct("こんや")`; `今夜` → `wrong("今夜")`.
    - `iteration_mark_counts_as_kanji`: shown `["々"]`, accepted `["々"]`: `々` → `wrong("々")`.
    - `containing_the_shown_text_is_not_repeating_it`: shown `["食"]`, accepted `["食べる"]`: `食べる` → `correct("食べる")`.
    - `spaces_are_ignored`: accepted `["たべてください"]`: `" tabete kudasai "` and `"たべて\u{3000}ください"` → `correct("たべてください")`; accepted `["たべて ください"]`: `tabetekudasai` → `correct("たべてください")`.
    - `empty_answers_are_not_checked`: `""`, `"   "`, `"\u{3000}"`, `"\t"` → `Outcome::Empty`.
    - `unreadable_answers_look_like_a_typo`: accepted `["たべる"]`: `tabxru`, `tabek`, `taka1`, `たか1`, `ｔａｋａｉ`, `ﾀｶｲ`, `@`, `たか-` → `Outcome::LooksLikeTypo`.
    - `readings_for_feedback`: accepted `["たかい"]`: `tokyo` → `wrong("ときょ")`, `タカイ` → `correct("たかい")`, `高い` → `wrong("高い")`, `yasui` → `wrong("やすい")`.
    - `combining_marks_are_composed`: accepted `["がい"]`: `"か\u{3099}い"` → `correct("がい")`; accepted `["ぱ"]`: `"ハ\u{309A}"` → `correct("ぱ")`.
    - `check_remembers_nothing`: `check("tabxru", &["たべる"], NOTHING)` twice → `Outcome::LooksLikeTypo` both times.
    - `both_spellings_of_konnichiha_are_correct`: accepted `["こんにちは"]`: `konnichiha`, `konnnichiha` and `KONNICHIHA` → `correct("こんにちは")`.
    - `both_spellings_of_onna_are_correct`: accepted `["女", "おんな"]`: `onna` and `onnna` → `correct("おんな")`.
    - `keyboard_reading_can_be_correct`: accepted `["今夜", "こんや"]`: `konnya` and `kon'ya` → `correct("こんや")`, `konya` → `wrong("こにゃ")`; accepted `["きんえん"]`: `kinnen` → `correct("きんえん")`; accepted `["おんあ"]`: `onna` → `correct("おんあ")` (the known cost in design *Risks*).
    - `single_readings_stay_exact`: accepted `["きんえん"]`: `kinen` → `wrong("きねん")`, `kin'en` → `correct("きんえん")`.
    - `wrong_answer_shows_the_textbook_reading`: accepted `["こんばんは"]`: `konnichiwa` → `wrong("こんにちわ")`, `konnichiha` → `wrong("こんにちは")`; accepted `["おとこ"]`: `onna` → `wrong("おんな")`.
    - `unreadable_text_with_an_nn_spot_is_a_typo`: accepted `["こんにちは"]`: `konnichiq` → `Outcome::LooksLikeTypo`.
  - Verify: the 25 new tests pass (at least `shown_kanji_is_never_accepted`, `copying_a_shown_word_with_kanji_is_wrong` and `iteration_mark_counts_as_kanji` fail without the repeat rule, and `kana_shown_text_never_makes_an_answer_wrong` fails if `blocked` keeps kana-only shown text), all earlier tests pass, fmt and clippy are clean. Commit, e.g. `feat(grading): check Japanese answers in any script`.

- [x] 3.2 Add `TypoGuard` (D2, D7) and document the checking rules.
  - `src/grading.rs`: `TypoGuard` with its derives and `submit` exactly as the D7 pseudocode. Compare `self.warned.as_deref() == Some(typed)`.
  - New tests in `src/grading.rs` `mod tests` (each starts from `let mut guard = TypoGuard::default();`, accepted `["たべる"]` unless stated, shown `NOTHING`):
    - `same_unreadable_text_twice_is_wrong`: `tabxru` → `LooksLikeTypo`; `tabxru` → `Wrong { read_as: None }`.
    - `corrected_text_is_checked_normally`: `tabxru` → `LooksLikeTypo`; `taberu` → `correct("たべる")`.
    - `different_unreadable_text_gets_its_own_warning`: `tabxru` → `LooksLikeTypo`; `tabwru` → `LooksLikeTypo`; `tabwru` → `Wrong { read_as: None }`.
    - `empty_submit_keeps_the_memory`: `tabxru` → `LooksLikeTypo`; `""` → `Empty`; `tabxru` → `Wrong { read_as: None }`.
    - `readable_wrong_answer_is_never_a_warning`: accepted `["とうきょう"]`: `tokyo` → `wrong("ときょ")` on the first submit.
    - `same_text_means_exactly_the_same`: `tabxru` → `LooksLikeTypo`; `"tabxru "` → `LooksLikeTypo`; `"tabxru "` → `Wrong { read_as: None }`.
    - `a_readable_submit_clears_the_memory`: `tabxru` → `LooksLikeTypo`; `tokyo` → `wrong("ときょ")`; `tabxru` → `LooksLikeTypo`.
    - `mixed_answer_is_wrong_at_once`: accepted `["食べる"]`: `食beru` → `Wrong { read_as: None }` on the first submit.
    - `two_readings_are_never_a_warning`: accepted `["こんばんは"]`: `konnichiha` → `wrong("こんにちは")` on the first submit; then `konnichiq` → `LooksLikeTypo`; then `konnichiq` → `Wrong { read_as: None }`.
  - `CHANGELOG.md`, *Unreleased* / *Added*: add this entry as the **last** item of the *Added* list (after whatever is last there now):
    `- Japanese answer checking (issue #18): romaji is read letter for letter into hiragana (\`shi\`/\`si\`, \`kitte\` → きって, \`kin'en\` → きんえん, \`-\` → ー; \`nn\` before a vowel is accepted both ways, so \`konnichiha\` and \`konnnichiha\` both match こんにちは) and katakana is folded to hiragana, so \`takai\`, \`たかい\`, \`タカイ\` and \`高い\` all match \`高い\`. Long vowels must be typed out (\`toukyou\`), mixed romaji and Japanese is wrong, and text that cannot be read gets one typo warning before it counts as wrong. No screen uses it yet; the practice screen is issue #21.`
    (without the backslashes; they only escape the backticks here).
  - `docs/architecture.md`, section *Checking answers*: replace the whole bullet that starts `- **An answer that repeats what the question already shows is never accepted.**` (two lines, ending with `"How do you read 食?" does not accept \`食\`.`) by this bullet, which states the narrower rule (design D6), wrapped like the surrounding text:
    - `- **Kanji the question shows can't be copied back as the answer.** An answer that is exactly a text the question shows is wrong when that text contains a kanji, so it can't just be copied and pasted: "How do you read 食?" does not accept \`食\`, and a question that shows \`食べる\` does not accept \`食べる\`, but \`たべる\` and \`taberu\` are correct. Text the question shows in kana only never makes an answer wrong.`
  - Then, right after that bullet, insert three bullets, wrapped like the surrounding text:
    - `- **Romaji is exact.** It is read letter for letter, the way it is typed into a Japanese IME: long vowels are typed out (\`とうきょう\` is \`toukyou\`; \`tokyo\` and \`tōkyō\` are wrong), \`n'\` gives ん before a vowel, and a doubled consonant gives っ. \`nn\` before a vowel is accepted both ways, as an IME reads it and as textbooks write it: \`konnichiha\` and \`konnnichiha\` both give こんにちは, and wrong answers are shown with the textbook reading.`
    - `- **One script per answer.** An answer is all romaji or all Japanese. \`食beru\` is wrong.`
    - `- **Typos.** Text that cannot be read as Japanese at all (\`tabxru\`) gets a warning and is not counted. Submitting the same text again counts it as wrong. A readable but wrong word, such as \`tokyo\`, is simply wrong.`
  - `docs/architecture.md`, section *5. Components*: in the Mermaid diagram add the node `kana[kana<br/>katakana → hiragana]` inside `subgraph core` right after the `romaji` node, and the edge `grading --> kana` right after `grading --> romaji`. Below the diagram, after the bullet about **`grading`**, add: `- **\`romaji\`** and **\`kana\`** have no state and no terminal access, like \`conjugation\`, so their rules are tested directly.`
  - Change nothing else in either file.
  - Verify: the 9 new tests pass, all earlier tests pass, fmt and clippy are clean; the Mermaid block still has matching `subgraph`/`end` lines; `grep -n "repeats what the question already shows" docs/architecture.md` finds nothing. Commit, e.g. `feat(grading): warn once about answers that look like typos`.

## 4. Answer field

- [ ] 4.1 Add `src/answer_field.rs` with `Edit`, `edit_for` and `AnswerField` (`text`, `apply`) (D2, D8 keys and editing).
  - `src/main.rs`: add `mod answer_field;` with the D1 attribute.
  - `Edit` and `AnswerField` with the derives of D2. `edit_for` follows the six rules of D8 in order. `apply` follows D8 *Editing*, using `ratatui::text::Span::styled_graphemes` for `DeleteBack`.
  - Imports: `ratatui::crossterm::event::{Event, KeyCode, KeyEventKind, KeyModifiers}`, `ratatui::style::Style`, `ratatui::text::Span`.
  - New tests in `src/answer_field.rs` `mod tests` (helpers like those in `src/keys.rs`: `fn press(code: KeyCode, modifiers: KeyModifiers) -> Event`, `fn with_kind(code: KeyCode, kind: KeyEventKind) -> Event`, using `ratatui::crossterm::event::KeyEvent`; and `fn field(text: &str) -> AnswerField` that applies `Edit::Insert` for each char of `text` to a default field):
    - `new_field_is_empty`: `AnswerField::default().text() == ""`.
    - `typed_text_is_kept_as_typed`: inserting `t`, `a`, `k`, `a`, `i` gives `takai` (no conversion).
    - `committed_ime_text_is_appended`: `field("高いタカイ").text() == "高いタカイ"`.
    - `control_characters_are_ignored`: on `field("a")`, inserting `'\t'`, `'\n'` and `'\u{7f}'` leaves `a`.
    - `backspace_removes_a_whole_kanji`: `field("食べる")`, `DeleteBack` twice → `食`.
    - `backspace_removes_a_kana_with_its_mark`: `field("たか\u{3099}")`, `DeleteBack` once → `た`.
    - `backspace_removes_a_kanji_with_its_variation_selector`: `field("a葛\u{E0100}")`, `DeleteBack` once → `a`.
    - `backspace_on_empty_does_nothing`: default field, `DeleteBack` → `""`.
    - `characters_type`: `edit_for` of `a` with `NONE` → `Some(Edit::Insert('a'))`; `A` with `SHIFT` → `Insert('A')`; `あ` with `NONE` → `Insert('あ')`; `a` with `KeyEventKind::Repeat` → `Insert('a')`.
    - `backspace_deletes`: `Backspace` with `NONE`, `Backspace` with `SHIFT`, `Char('h')` with `CONTROL`, and `Backspace` with `KeyEventKind::Repeat` → `Some(Edit::DeleteBack)`.
    - `other_keys_do_not_edit`: `Enter`, `Esc`, `Up`, `Left`, `Tab`, `Delete`, `Home`, `End` with `NONE`; `Char('a')` with `CONTROL`; `Char('a')` with `ALT`; `Char('c')` with `CONTROL`; `Backspace` with `CONTROL`; `Char('a')` and `Backspace` with `KeyEventKind::Release`; `Event::Paste("たかい".to_string())`; `Event::FocusGained`; `Event::Resize(80, 24)` → all `None`.
    - `typing_through_the_key_mapping`: feed `t`, `a`, `k`, `a`, `x`, `Backspace`, `i` through `edit_for` and `apply` each `Some` result → `takai`.
  - Verify: the 12 new tests pass, all earlier tests pass, fmt and clippy are clean. Commit, e.g. `feat(input): add the answer field and its keys`.

- [ ] 4.2 Add `AnswerField::render` (D2, D8 *Drawing*) and document the field.
  - `src/answer_field.rs`: `pub fn render(&self, area: Rect, buf: &mut Buffer) -> Option<Position>` following the six drawing steps of D8. Imports: `ratatui::buffer::Buffer`, `ratatui::layout::{Position, Rect}`. No `as` casts; no `unwrap` outside tests.
  - New tests in `src/answer_field.rs` `mod tests` (helper `fn draw(field: &AnswerField, area: Rect) -> (Buffer, Option<Position>)` that makes `Buffer::empty(Rect::new(0, 0, 20, 5))` and calls `render`; read cells with `buffer[(x, y)].symbol()`):
    - `kana_take_two_columns`: `field("たかい")` in `Rect::new(0, 0, 10, 1)`: cells 0, 2, 4 of row 0 are `た`, `か`, `い`; cursor `Some(Position::new(6, 0))`.
    - `latin_letters_take_one_column`: `field("takai")` in `Rect::new(0, 0, 10, 1)`: cells 0..5 are `t`, `a`, `k`, `a`, `i`; cursor `(5, 0)`.
    - `empty_field_puts_the_cursor_at_the_start`: default field in `Rect::new(3, 2, 10, 1)` → cursor `Some(Position::new(3, 2))`, and cells 3..13 of row 2 are `" "`.
    - `field_is_drawn_inside_its_area`: `field("たかい")` in `Rect::new(3, 2, 10, 1)`: cell (3, 2) is `た`, cell (7, 2) is `い`, cursor `(9, 2)`; every cell of rows 0, 1, 3 and 4 is `" "`.
    - `field_uses_no_colour`: `field("takaiたかい")` in `Rect::new(0, 0, 20, 5)`: every cell has `fg == Color::Reset` and `bg == Color::Reset` (`ratatui::style::Color`).
    - `long_text_keeps_its_end_visible`: `field("abcdefたか")` in `Rect::new(0, 0, 6, 1)`: cell 0 is `f`, cell 1 is `た`, cell 3 is `か`, cursor `(5, 0)`.
    - `wide_character_is_not_cut_in_half`: `field("あいう")` in `Rect::new(0, 0, 4, 1)`: cell 0 is `う`, cells 2 and 3 are `" "`, cursor `(2, 0)`.
    - `field_at_any_size`: `field("食べる")`: areas `Rect::new(0, 0, 0, 0)`, `(0, 0, 0, 1)` and `(0, 0, 1, 0)` → `None`; `(0, 0, 1, 1)` and `(0, 0, 2, 1)` → cursor `(0, 0)` and cells 0 (and 1) are `" "`; `(0, 0, 3, 1)` → cell 0 is `る`, cursor `(2, 0)`. None of them panics.
    - `only_the_first_row_is_drawn`: `field("takai")` in `Rect::new(0, 0, 10, 3)`: rows 1 and 2 are all `" "`.
    - `cursor_reaches_the_terminal`: with `Terminal::new(TestBackend::new(20, 3))` (`ratatui::Terminal`, `ratatui::backend::TestBackend`), draw `field("たかい")` with `if let Some(p) = field.render(frame.area(), frame.buffer_mut()) { frame.set_cursor_position(p); }`; then `terminal.get_cursor_position().unwrap() == Position::new(6, 0)`. This is how #21 will use the field.
  - `CHANGELOG.md`: in the entry added in 3.2, change its start from `- Japanese answer checking (issue #18):` to `- Japanese answer input and checking (issue #18):`, and insert this sentence right before `No screen uses it yet;`: `Answers are typed into a text field that keeps IME text as typed, deletes whole characters with Backspace and is drawn by display width.` Change nothing else in the file.
  - `docs/architecture.md`:
    - Status note at the top: add the sentence `Answer input and checking (\`answer_field\`, \`grading\`, \`kana\`, \`romaji\`) exist, but no screen uses them yet.` right after the sentence that says which modules exist in code (keep every other sentence of the note as it is when you edit it; another change may have edited the note meanwhile).
    - *5. Components*: add the node `field[answer_field<br/>typed answer,<br/>display width]` inside `subgraph core` after the `kana` node, and the edges `keys --> field` and `ui --> field` after `grading --> kana`. After the bullet added in 3.2, add: `- **\`answer_field\`** keeps the answer exactly as typed, romaji or IME text, and converts nothing while typing. \`keys\` asks it which keys edit it and \`ui\` asks it to draw itself, by display width.`
    - Change nothing else.
  - Verify: the 10 new tests pass, all earlier tests pass, fmt and clippy are clean (no `unfulfilled_lint_expectations`: nothing outside tests uses the new modules yet), and `git diff main -- Cargo.toml Cargo.lock VERSION` is empty. Commit, e.g. `feat(input): draw the answer field by display width`.

## 5. Integration check

- [ ] 5.1 Check in a pseudo-terminal that committed Japanese text reaches the app as typed characters (no person needed; Linux). Build with `cargo build --release`, run `target/release/tui-learnjapanese` under a pty of 80x24 (for example with Python's `pty` module; keep the script outside the repository, e.g. in a temporary directory). On the Welcome screen, send the UTF-8 bytes of each character of `たかい高い` one at a time, as an IME commit would, and after each confirm the output contains `Last key: <that character>`; then send `\x7f` (Backspace) and confirm the app is still running; then send `\x03` (Ctrl+C) and confirm it exits with status 0. Do not send `q`. Record each result in the report. This task changes no files: commit only the ticked checkbox, e.g. `chore(openspec): record the Japanese text terminal check`. If a character does not arrive, stop and report; do not change `src/keys.rs`.

## Workflow follow-up

- The real IME check (an actual IME on Linux, macOS and Windows terminals, and over SSH) needs the answer field on a screen, so it is done in #21 (design Open Question 3).
- #21 must remove the four `expect(dead_code)` attributes from `src/main.rs` as it starts using each module; clippy reports `unfulfilled_lint_expectations` for any it forgets.
- Archive order: this change uses ADDED requirements only, for three new capabilities, so it archives on its own, before or after the other open changes.
- `CHANGELOG.md` and the status note in `docs/architecture.md` are also edited by the area selection change (#20); whichever branch merges second resolves the text conflict by keeping both entries.
