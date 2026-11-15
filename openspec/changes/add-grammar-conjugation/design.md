# Design

## Context

See `proposal.md` for the motivation and the three specs for the required behaviour.

State of `main` this design starts from (all merged; read in full while writing this):

- `src/areas.rs`: `pub trait Area: Debug { fn name(&self) -> &'static str; fn questions(&self, _levels: Levels) -> Vec<Entry> { Vec::new() } fn has_questions(&self) -> bool { /* built on questions */ } }`, `const ALL: &[&dyn Area] = &[];`, `pub fn all()`, test-only `pub mod fake` (`FakeArea`) and `pub mod sample` (`src/areas/sample.rs`). Its doc comment says "There are none yet. Adding an area adds its own file under `src/areas/` and one entry here."
- `src/practice/`: `Level` (`N5`..`N1`, `Level::ALL`), `Levels` (`only`, `contains`, `toggled`, `is_empty`, `intersects`, `ALL`; the field is private and no constructor is `const`), `Entry { levels, questions }`, `Question { prompt, accepted, shown, kind }`, `question::Kind { Japanese, Meaning }` (not re-exported; used as `crate::practice::question::Kind`). The practice module picks an entry at random, then one of its questions at random.
- `src/grading.rs`: `check(typed, accepted, shown)` removes whitespace, composes voiced marks and folds katakana to hiragana on both sides, keeps kanji, and reads romaji letter for letter (`dewa` is でわ, never では). A Japanese answer equal to a normalised shown text **that contains a kanji** is wrong; kana-only shown texts never block. Romaji readings are kana only, so they can only match accepted answers written in kana: **every question must accept a kana-only form of each answer**, and accepted answers never contain romaji.
- `src/ui.rs`: the question prompt is a wrapped `Paragraph` over `PROMPT_ROWS = 3` rows; the feedback screen draws `text::expected(&question.accepted)` (`Correct answer: X` / `Correct answers: X, Y, ...`) on **one** centred row, which a long list overflows.
- `src/text.rs`: every shown text lives here; fixed texts are constants, texts with values are functions; non-Japanese wording is plain ASCII with no ambiguous-width characters, and each constant has an ASCII test.
- `src/practice/question.rs`: `enum Kind` carries `#[cfg_attr(not(test), expect(dead_code, reason = "constructed by the areas' questions, issues #11 to #13"))]` with a comment saying the first real area removes it.
- The crate is one binary: `cargo clippy --all-targets -- -D warnings` fails on any item non-test code never uses. Test baseline: **379 tests**, all passing.

Facts checked on a scratch copy of `main` (outside the repository, own `CARGO_TARGET_DIR`, cargo 1.100.0, ratatui 0.30.2) while writing this design:

1. **A real area breaks no existing test.** A stub `src/areas/grammar.rs` (one `Kind::Japanese` question at N5) registered with `mod grammar;`, `use grammar::Grammar;` and `ALL = &[&Grammar]`: `cargo clippy -- -D warnings` and `cargo clippy --all-targets -- -D warnings` clean, `cargo test` **379 passed, 0 failed** (D13).
2. **`Kind`'s expectation stays fulfilled** in that build, because `Kind::Meaning` is still never constructed outside tests. Removing the attribute makes clippy fail: `error: variant 'Meaning' is never constructed`. Moving the same attribute from the enum to the `Meaning` variant passes both clippy runs and `cargo fmt --check` (D12).
3. **Wrapping the accepted answers** (the feedback row as `Constraint::Length(4)` drawn like the prompt) keeps all 379 tests passing, and the eight answers of the worst case (簡単, polite past negative, 223 columns) fit in three rows at 80 columns (D10).
4. **The release build can be driven through `tmux`** (`new-session -x 80 -y 24`, `send-keys`, `capture-pane -p`), which reconstructs the screen: Welcome → `> Grammar` → picker → question → `tabemasita` → `Read as: たべました`, `Correct!` → Esc, Esc → exit status 0 (D17).
5. Every form of the 49 words in D8 was generated with a throwaway model of the D3 rules and read through by hand.

## Goals / Non-Goals

**Goals:**
- The first real area, reachable in the released app, with rule-generated answers for every group, form and formality, each rule tested (D2 to D5).
- No change to the practice module, `session`, `keys`, `grading` or any test (D11, D13).
- Content in the binary, checked by tests over the whole list, so later additions to the list are checked too (D6, D14).
- Every commit builds, passes fmt, clippy and tests, and has no unused code (D15).

**Non-Goals:**
- Choosing forms, formalities or groups inside Grammar; custom (hand-written) questions on an entry; descriptions; ids; a content file format (D14).
- Honorific godan verbs, 問う, ～ずる verbs, compounds of 来る and 行く other than the words listed (D4).
- Accepting non-standard or archaic variants (よい for いい, 静かだったです, 高いでした) (D5).

## Decisions

### D1. Files

| File | Contents | Tasks |
|---|---|---|
| `src/conjugation.rs` (new) | `Group`, `Form`, `Formality`, `Answer`, `pub fn conjugate`, private `inflect`, `endings`, `pick`, `GODAN`; tests | 1.1 (ichidan), 1.3, 1.4, 1.5, 2.2, 2.3 |
| `src/areas/grammar.rs` (new) | `Grammar`, `Word`, `const fn word`, `WORDS`, `ASKED`, `entry`, `question`, `push_unique`; tests | 1.1, 1.3, 1.4, 1.5, 2.2, 2.3 |
| `src/main.rs` | `mod conjugation;` after `mod areas;` (alphabetical, like the others) | 1.1 |
| `src/areas.rs` | `mod grammar;`, `use grammar::Grammar;`, `&Grammar` in `ALL`, doc comment of `ALL` | 1.1 |
| `src/text.rs` | `AREA_GRAMMAR`, `form_name`, `conjugation_prompt`; tests | 1.1 |
| `src/practice/question.rs` | the expectation moves from `Kind` to `Kind::Meaning` | 1.2 |
| `src/ui.rs` | `EXPECTED_ROWS`, the accepted answers wrap; tests | 2.1 |

**Where `conjugation` lives:** `src/conjugation.rs`, a top-level module, as the component diagram in `docs/architecture.md` §5 draws it and like `romaji` and `kana` (pure, no state, no terminal, tested directly). Alternative: `src/areas/grammar/conjugation.rs`, which would spare the `mod` line in `main.rs`; rejected because `text::conjugation_prompt` needs `Form` and `Formality`, which would then be reached through the area's private module tree, and because the architecture names it as its own component.

### D2. The conjugation API

```rust
// src/conjugation.rs
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
    /// いい: its forms other than the non-past are built on よ.
    Ii,
    /// な-adjectives, stored without だ or な: 静か.
    NaAdjective,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Form { NonPast, Negative, Past, PastNegative, Te }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Formality { Plain, Polite }

/// One answer, as written and as read. The same text twice for a word in kana.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Answer {
    pub written: String,
    pub reading: String,
}

/// Every standard way to say the word (`written`, read `reading`, of `group`) in
/// `form` and `formality`, the most common first. Empty when the form does not
/// exist (the polite te-form) or the word does not end the way its group needs.
pub fn conjugate(written: &str, reading: &str, group: Group, form: Form, formality: Formality) -> Vec<Answer>
```

- `conjugate` = `inflect(written, ..)` and `inflect(reading, ..)` zipped into `Answer`s; if either is empty or their lengths differ, `Vec::new()`.
- `fn inflect(text: &str, group: Group, form: Form, formality: Formality) -> Vec<String>`: `endings(..)` gives `(cut, endings)`; `text.strip_suffix(cut)` gives the stem (`None` → empty); the result is `stem + ending` for each ending, in order. For `NaAdjective`, `cut` is `""` and an empty `text` gives nothing.
- `fn endings(text: &str, group: Group, form: Form, formality: Formality) -> Option<(&'static str, Vec<String>)>`: the tables of D3. It looks at the text only where the group needs it: the last kana for `Godan`, whether it ends in `来る` or `くる` for `Kuru`.
- `fn pick<T>(form: Form, formality: Formality, plain: [T; 5], polite: [T; 4]) -> Option<T>`: the entry for the asked form, in the order non-past, negative, past, past negative, te-form (plain) and non-past, negative, past, past negative (polite); `None` for `(Te, Polite)`. This one helper is where "no polite te-form" lives for every group.
- **The rules work on the written form and on the reading separately** with the same tables, because every group's ending is kana in both scripts, except 来る: there the written ending is `来る` and the reading ending `くる`, and `endings` picks the table from the text itself (D3). No flag says "this is the reading", and a kana-only word just passes the same text twice.
- **Why a flat `Group`** rather than `Verb(Irregular(...))` nesting: one `match` per rule, one variant per rule set, and adding an irregular later (say, honorific verbs) is one variant, one table and its tests. Alternatives considered: (a) per-word override tables (stored forms), rejected because the issue requires generated answers; (b) explicit stems stored per word (`stems: ["こ", "き"]`), rejected because the four irregulars in scope each need only a fixed table.

### D3. The rule tables

"Cut" is removed from the end of the text; each ending is appended to what is left. Several endings in one cell are accepted variants, in that order (D5). `—` = no answer.

**Plain:**

| Group (text must end in) | Cut | Non-past | Negative | Past | Past negative | Te-form |
|---|---|---|---|---|---|---|
| Ichidan (`る`) | `る` | る | ない | た | なかった | て |
| Godan (D3a) | last kana *k* | *k* | *a*ない | *ta* | *a*なかった | *te* |
| Suru (`する`) | `する` | する | しない | した | しなかった | して |
| Kuru, written (`来る`) | `来る` | 来る | 来ない | 来た | 来なかった | 来て |
| Kuru, reading (`くる`) | `くる` | くる | こない | きた | こなかった | きて |
| Iku (`く`) | `く` | く | かない | った | かなかった | って |
| Aru (`ある`) | `ある` | ある | ない | あった | なかった | あって |
| IAdjective (`い`) | `い` | い | くない | かった | くなかった | くて |
| Ii (`いい`) | `いい` | いい | よくない | よかった | よくなかった | よくて |
| NaAdjective (not empty) | `""` | だ | じゃない; ではない | だった | じゃなかった; ではなかった | で |

**Polite** (te-form: `—` for every group):

| Group | Non-past | Negative | Past | Past negative |
|---|---|---|---|---|
| Ichidan | ます | ません | ました | ませんでした |
| Godan | *i*ます | *i*ません | *i*ました | *i*ませんでした |
| Suru | します | しません | しました | しませんでした |
| Kuru, written | 来ます | 来ません | 来ました | 来ませんでした |
| Kuru, reading | きます | きません | きました | きませんでした |
| Iku | きます | きません | きました | きませんでした |
| Aru | あります | ありません | ありました | ありませんでした |
| IAdjective | いです | くないです; くありません | かったです | くなかったです; くありませんでした |
| Ii | いいです | よくないです; よくありません | よかったです | よくなかったです; よくありませんでした |
| NaAdjective | です | じゃないです; じゃありません; ではないです; ではありません | でした | じゃなかったです; じゃありませんでした; ではなかったです; ではありませんでした |

For `Kuru`, `endings` checks `text.ends_with("来る")` first, then `text.ends_with("くる")`; neither → `None`. For `Iku`, the last kana must be `く`. For `Ii`, the text must end in `いい` (the word is stored in kana, D8).

**D3a. Godan.** The last kana *k* selects one row of a constant table (`const GODAN: [(&str, &str, &str, &str, &str); 9]`, columns *k*, *a* negative stem, *i* polite stem, *te*, *ta*); a last kana not in the table → `None`:

| *k* | *a* | *i* | *te* | *ta* | Example (written / reading) |
|---|---|---|---|---|---|
| う | わ | い | って | った | 買う かう: 買わない, 買います, 買って, 買った |
| く | か | き | いて | いた | 書く かく: 書かない, 書きます, 書いて, 書いた |
| ぐ | が | ぎ | いで | いだ | 泳ぐ およぐ: 泳がない, 泳ぎます, 泳いで, 泳いだ |
| す | さ | し | して | した | 話す はなす: 話さない, 話します, 話して, 話した |
| つ | た | ち | って | った | 待つ まつ: 待たない, 待ちます, 待って, 待った |
| ぬ | な | に | んで | んだ | 死ぬ しぬ: 死なない, 死にます, 死んで, 死んだ |
| ぶ | ば | び | んで | んだ | 遊ぶ あそぶ: 遊ばない, 遊びます, 遊んで, 遊んだ |
| む | ま | み | んで | んだ | 飲む のむ: 飲まない, 飲みます, 飲んで, 飲んだ |
| る | ら | り | って | った | 帰る かえる: 帰らない, 帰ります, 帰って, 帰った |

The reading of each example is its reading stem plus the same ending (かわない, かいます, かって, かった; およがない, ...). The full nine forms of the nine example verbs are in Appendix B; tests copy them from there.

### D4. Irregulars: in scope and not

| Verb | In scope | How |
|---|---|---|
| する, and noun + する (勉強する, 運転する) | yes | `Group::Suru` |
| 来る | yes | `Group::Kuru`; written keeps 来, reading changes |
| 行く | yes | `Group::Iku` |
| ある | yes | `Group::Aru` |
| いい (adjective) | yes | `Group::Ii` |
| くれる, いる (to be) | not irregular in these nine forms | would be `Ichidan` |
| Honorific -aru verbs (いらっしゃる, くださる, なさる, おっしゃる, ござる): polite stem い (いらっしゃいます) | **no** | a later `Group` variant; none is in the list |
| 問う, 請う (te-form とうて) | **no** | rare; not in the list |
| ～ずる / ～じる verbs (信ずる), 愛する (愛さない) | **no** | not in the list |
| Compounds of 来る / 行く (持って来る, 出て行く) | **no** | `Kuru`/`Iku` would handle them if written ending in 来る/行く, but none is listed and none is tested |
| よい / 良い as variants of いい | **no** | D5 |

Adding an irregular later is one `Group` variant, one row or table in `endings`, its words, and its tests; no other code changes.

### D5. Adjectives: what "polite" means, which variants are accepted, which come first

- **"Polite" means the です/ます style** (丁寧語), the only polite style in scope.
- **Every standard variant is accepted** (owner's recommendation), because a learner taught either textbook style must not be marked wrong:
  - i-adjectives: polite negative 高くないです **and** 高くありません; polite past negative 高くなかったです **and** 高くありませんでした. Polite non-past only 高いです; polite past only 高かったです (高いでした is a common learner error, not a variant).
  - na-adjectives: plain negative じゃない **and** ではない (same for past negative); polite negative じゃないです, じゃありません, ではないです, ではありません; polite past negative じゃなかったです, じゃありませんでした, ではなかったです, ではありませんでした. Not accepted: 静かだったです and 静かでしたです (non-standard), でわ spellings (non-standard).
- **Order (what feedback lists first):** the `-ないです` form before the `-ありません` form, and じゃ before では. Reason: the `-ないです` forms are built from the plain negative the learner has just practised (高くない + です), which is how modern textbooks introduce them first; じゃ is the spoken form. The order matters only for reading feedback; all variants are equally correct.
- **"Which is shown as the correct answer":** the feedback screen lists **all** accepted answers in order (the existing `text::expected`); there is no separate "shown answer". The first item is the written form of the first variant. D10 makes the long lists fit.
- **Na-adjectives are stored without だ or な** (`静か`), the dictionary form, and shown that way in the prompt. **The plain non-past answer is the word with だ** (`静かだ`, `しずかだ`); the bare word is not accepted. Reason: the question is about the predicate form, and だ is what distinguishes it from the stem; accepting the bare word would also make the question "type the shown word". Open Question 3.
- **いい:** non-past いい / いいです; every other form on よ. Stored in kana (いい is rarely written 良い, which is read よい). よい/良い forms are not accepted (Open Question 9).

### D6. The Grammar area

```rust
// src/areas/grammar.rs
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
const fn word(written: &'static str, reading: &'static str, meaning: &'static str, group: Group, level: Level) -> Word

/// Every word, by group, in the order of design.md Appendix A.
const WORDS: &[Word] = &[
    word("食べる", "たべる", "to eat", Group::Ichidan, Level::N5),
    // ...
];

/// The Grammar area.
#[derive(Debug)]
pub struct Grammar;

impl Area for Grammar {
    fn name(&self) -> &'static str { text::AREA_GRAMMAR }
    fn questions(&self, levels: Levels) -> Vec<Entry> {
        WORDS.iter().filter(|word| levels.contains(word.level)).map(entry).collect()
    }
}

/// The entry of one word: its level and one question per asked form.
fn entry(word: &Word) -> Entry {
    Entry {
        levels: Levels::only(word.level),
        questions: ASKED.iter().filter_map(|&(form, formality)| question(word, form, formality)).collect(),
    }
}

/// The question for one form of `word`; `None` when the rules give no answer.
fn question(word: &Word, form: Form, formality: Formality) -> Option<Question> {
    let answers = conjugation::conjugate(word.written, word.reading, word.group, form, formality);
    if answers.is_empty() { return None; }
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
fn push_unique(list: &mut Vec<String>, text: String)
```

- **One level per word** (`level: Level`), not a list: every word of the starting list has one level, `Levels` has no `const` constructor, and the field is private to the area, so a list can replace it when a word needs two. The architecture's "one or more levels" still holds for `Entry`.
- **`shown`** is the written form and, when different, the reading: exactly what the prompt shows in Japanese. Only the written form with kanji blocks a copied answer (#18's rule); the reading never does.
- **`accepted`** is, for each answer of `conjugate` in order, its written form then its reading, without duplicates; for a kana word just one string per variant. So kanji, kana (and katakana, folded by `grading`) and romaji all work, and nothing in it is romaji.
- **Filtering:** `questions` keeps the words with a chosen level; `practice::selection` filters again, as the contract allows.
- **Every word gives nine questions.** `filter_map` drops a form the rules cannot give, but the content test (task 1.1, `every_word_gives_nine_questions`) fails for any word that loses one, so a misfiled word cannot silently ship with fewer questions.

### D7. The prompt and the wording

```rust
// src/text.rs (task 1.1)
use crate::conjugation::{Form, Formality};

/// Area selection: the name of the Grammar area.
pub const AREA_GRAMMAR: &str = "Grammar";

/// Grammar: how a form is named in a question.
pub fn form_name(form: Form, formality: Formality) -> &'static str {
    match (form, formality) {
        (Form::NonPast, Formality::Plain) => "plain non-past",
        (Form::Negative, Formality::Plain) => "plain negative",
        (Form::Past, Formality::Plain) => "plain past",
        (Form::PastNegative, Formality::Plain) => "plain past negative",
        (Form::Te, _) => "te-form",
        (Form::NonPast, Formality::Polite) => "polite non-past",
        (Form::Negative, Formality::Polite) => "polite negative",
        (Form::Past, Formality::Polite) => "polite past",
        (Form::PastNegative, Formality::Polite) => "polite past negative",
    }
}

/// Grammar: the question for one form of a word. A word written in kana shows
/// only its meaning in brackets; any other word shows its reading too.
pub fn conjugation_prompt(written: &str, reading: &str, meaning: &str, form: Form, formality: Formality) -> String {
    let name = form_name(form, formality);
    if written == reading {
        format!("What is the {name} of {written} ({meaning})?")
    } else {
        format!("What is the {name} of {written} ({reading}, {meaning})?")
    }
}
```

Examples (spec *The prompt*): `What is the polite past negative of 食べる (たべる, to eat)?`, `What is the plain negative of する (to do)?`, `What is the te-form of 行く (いく, to go)?`.

- **The written form, its reading and the meaning are all shown.** The area practises conjugation, not kanji reading: a learner answering in romaji must know the reading (勉強する, 来る), and the meaning tells apart words with the same reading. Consequence: the plain non-past question of a verb or i-adjective is answered by typing the shown reading; the owner has decided all nine forms are asked and that typing the reading is correct. Open Question 1 offers hiding the reading.
- **Te-form has no formality** in the prompt: there is only the plain one.
- **Wording** is plain ASCII around the Japanese, with ASCII brackets and commas (no full-width or ambiguous-width characters). The longest prompt in the list is 72 columns (`What is the polite past negative of 勉強する (べんきょうする, to study)?`); a test keeps every prompt within 80 columns, so it fits on one row of an 80-column terminal; narrower terminals wrap it over the `PROMPT_ROWS = 3` rows.
- **The area name lives in `src/text.rs`**, following that file's rule that every shown text lives there (#20's Open Question 5 left this to the first area). Alternative: a literal in `grammar.rs`, which would make adding an area touch one file fewer; rejected for consistency (Open Question 10).

### D8. The word list

49 words written for this project (Appendix A has the complete list with readings, meanings, groups and levels). Coverage:

| Group | Words | Count |
|---|---|---|
| Ichidan | 食べる 見る 起きる 寝る 教える 決める 調べる | 7 |
| Godan う | 買う 会う 手伝う | 3 |
| Godan く | 書く 聞く 働く | 3 |
| Godan ぐ | 泳ぐ 急ぐ | 2 |
| Godan す | 話す 貸す | 2 |
| Godan つ | 待つ 持つ | 2 |
| Godan ぬ | 死ぬ (the only common ぬ verb) | 1 |
| Godan ぶ | 遊ぶ 運ぶ | 2 |
| Godan む | 飲む 読む 住む | 3 |
| Godan る | 帰る 分かる 送る (帰る looks ichidan but is godan) | 3 |
| Suru | する 勉強する 運転する | 3 |
| Kuru, Iku, Aru | 来る 行く ある | 3 |
| IAdjective | 高い 安い 大きい 新しい 寒い 優しい 悲しい | 7 |
| Ii | いい | 1 |
| NaAdjective | 静か きれい 元気 好き 便利 簡単 大切 (きれい ends in い but is a na-adjective) | 7 |

Levels: 35 words at N5, 14 at N4. Ticking only N4 practises the 14; N3, N2 and N1 have no Grammar words (the picker then says `No questions at these levels yet`).

- **Written forms** use the kanji these words are usually written with; ある, する, いい and きれい are kept in kana, as they usually are.
- **Meanings** are short English glosses written for this project, plain ASCII, without brackets (so the prompt has one pair).

### D9. JLPT levels and licensing

- The **words** are common, basic vocabulary; their spelling, readings and meanings are written for this project. The **forms** are generated by the rules. Neither is copied from any list, dictionary or textbook, so the content is GPL-3.0-or-later like the code and needs no third-party row in `ATTRIBUTION.md`.
- The **JLPT levels**: the JLPT has published no official vocabulary list since 2010. The levels here are **the project's own approximate judgment** (N5 for the most basic everyday words, N4 for the next ones), stated as approximate in `ATTRIBUTION.md` and `docs/architecture.md`. No published unofficial list was consulted or copied. If the owner prefers levels from a licensed source, a GPL-compatible one would have to be found and recorded first (Open Question 7); none is needed for this change.
- **`ATTRIBUTION.md`** gets a paragraph below the table (task 3.2): "Content written for this project is licensed under GPL-3.0-or-later, like the code, and is not listed in the table. This includes the Grammar word list in `src/areas/grammar.rs` (words, readings, English meanings). Its JLPT levels are the project's own approximate judgment; the JLPT publishes no official vocabulary list, and none of the unofficial lists was used." The table keeps `_None yet_`.

### D10. Long lists of accepted answers on the feedback screen

A na-adjective's polite past negative has four variants, eight accepted strings (Appendix B: 簡単 gives 223 columns). On one centred row most of it is cut off, and the kana forms, which are what a romaji learner needs, come last.

**Decision:** in `src/ui.rs`, the accepted-answers row of `draw_feedback` becomes `Constraint::Length(EXPECTED_ROWS)` with

```rust
/// Rows the accepted answers may use on the feedback screen.
const EXPECTED_ROWS: u16 = 4;
```

and is drawn with the existing `draw_prompt(frame, expected_row, &text::expected(&question.accepted))` (a centred, word-wrapped `Paragraph`) instead of `centred(..)`. `draw_prompt`'s doc comment becomes "Draws a text that may need several rows over `area`, wrapped between words, each line centred." The layout grows from 14 to 17 rows and still fits 24 rows. When the answer is correct the rows stay empty, as the one row did before.

- **Why `ui.rs` changes in an area's change:** the feedback screen is the shared practice module, not another area, and the change is generic (Vocabulary's lists of meanings need it too). No area-specific code enters `ui.rs`.
- **Why 4 rows:** the worst case in the list needs 3 rows at 80 columns, with the third row exactly full (fact 3); 4 leaves room for longer na-adjectives.
- **Alternatives:** accept only one variant per form (no UI change, but marks correct textbook answers wrong, against the issue's intent); show only the first answer (changes `text::expected` and its existing test `text::tests::expected_answers` and `ui::tests::feedback_for_a_wrong_answer`, which is not allowed). Open Question 5.

### D11. What the change touches outside its own files, and why

"Adding this module must not require changes to other modules": no other **area** is touched (there is none), and no part of the practice module, `session`, `keys`, `grading`, `romaji`, `kana` or `answer_field` changes. The edits outside the two new files are:

| File | Change | Why it is unavoidable |
|---|---|---|
| `src/areas.rs` | `mod grammar;`, `use grammar::Grammar;`, `&Grammar` in `ALL`, `ALL`'s doc comment ("There are none yet." becomes "Adding an area adds its own file under `src/areas/` and one entry here.") | The registration step #20 designed (its D2) |
| `src/main.rs` | `mod conjugation;` | Declares the new component (D1) |
| `src/text.rs` | `AREA_GRAMMAR`, `form_name`, `conjugation_prompt` | The rule that all wording lives in `text.rs` |
| `src/practice/question.rs` | the expectation moves to `Kind::Meaning` | Its own comment asks the first real area to deal with it (D12) |
| `src/ui.rs` | accepted answers wrap | D10; can be dropped if the owner prefers one variant per form |

### D12. The unused-code marker on `Kind`

What clippy does (checked, fact 2): with Grammar constructing `Kind::Japanese` and nothing outside tests constructing `Kind::Meaning`, the enum-level `expect(dead_code)` is **still fulfilled** (the lint still fires, for `Meaning`), so clippy stays clean and does **not** report it as unfulfilled. Removing the attribute, as its comment suggests, fails clippy (`variant 'Meaning' is never constructed`).

**Decision (task 1.2):** keep the expectation and narrow it to the variant that is still unused. In `src/practice/question.rs`, delete the two comment lines and the attribute above `pub enum Kind` and put this above `Meaning`, after its doc comment:

```rust
    /// A meaning in plain English, checked by `practice::meaning`.
    // No real area asks for a meaning yet (Vocabulary and Kanji, issues #11 and
    // #12): the first one that does removes this attribute. Tests construct it already.
    #[cfg_attr(
        not(test),
        expect(
            dead_code,
            reason = "constructed by the Vocabulary and Kanji questions, issues #11 and #12"
        )
    )]
    Meaning,
```

- This is the same expectation the owner approved, with a narrower scope and an updated reason, not a new suppression: nothing else is suppressed, `Japanese` is now checked by the lint again, and clippy will report the expectation as unfulfilled as soon as #11 or #12 constructs `Meaning`, which tells that change to remove it.
- Verified: `cargo clippy -- -D warnings` and `cargo clippy --all-targets -- -D warnings` clean, `cargo fmt --all -- --check` clean.
- Alternative: leave the attribute on the enum and only update its comment and reason. Equivalent for clippy; less precise. Open Question 6.
- **If clippy reports anything else** (any unfulfilled expectation, any dead code), the implementer stops and reports; no `#[allow]` or new `expect` may be added.

### D13. Existing tests

Checked one by one against the code, and confirmed by fact 1 (all 379 pass with a real area registered) and fact 3 (all pass with the wrapped feedback row). Registering Grammar changes only what the real list contains; the tests below that use the real list (`Session::new`, `areas::all()`) assert nothing a non-empty list breaks.

| Tests | Uses the real list? | Effect |
|---|---|---|
| `answer_field::tests` (22), `grading::tests` (34), `kana::tests` (9), `romaji::tests` (34) | no | Unaffected: modules not edited. |
| `app::tests` (8: `Ticker`, `seed_from`) | no | Unaffected. |
| `practice::tests` (24), `practice::level::tests` (6), `practice::meaning::tests` (10), `practice::pool::tests` (5), `practice::rng::tests` (6) | no | Unaffected: `practice` code unchanged; `question.rs` changes only an attribute. |
| `areas::sample::tests` (9) | no | Unaffected. |
| `areas::tests::area_names_are_unique_and_not_empty` | yes (`all()`) | Passes: one name, `Grammar`, not empty. |
| `areas::tests::every_listed_area_agrees_with_its_questions` | yes | Passes, and is no longer vacuous: Grammar uses the default `has_questions`. |
| `areas::tests::an_area_without_entries_has_no_questions`, `entries_without_questions_do_not_count`, `entries_without_a_level_do_not_count` | no | Unaffected. |
| `session::tests::new_session_starts_on_welcome_with_nothing_recorded`, `quit_sets_should_quit`, `typed_keeps_the_last_character`, `resized_updates_the_size`, `ticks_advance_elapsed_time`, `start_moves_from_welcome_to_area_selection`, `start_is_one_way`, `start_does_not_quit`, `only_quit_sets_should_quit`, `with_seed_keeps_the_welcome_screen` | yes (`Session::new`) | Pass: none of them chooses an area or checks the list. |
| `session::tests::new_session_lists_the_apps_areas` | yes | Passes: compares `areas().len()` with `areas::all().len()` (1 = 1). |
| `session::tests::an_empty_list_highlights_nothing`, `choosing_with_no_areas_does_nothing`, `moving_with_no_areas_does_nothing` | no (`with_areas(&[])`) | Unaffected. |
| all other `session::tests` (45) | no (`with_areas` with `FakeArea` or Sample lists) | Unaffected. |
| `keys::tests::enter_then_ctrl_c_through_the_key_table` | yes (`Session::new`) | Passes: Enter → area selection, `q` does nothing there, `Ctrl+C` quits; none depends on the list. |
| `keys::tests::only_esc_leaves_an_empty_list`, `ctrl_c_quits_an_empty_list` | no (`with_areas(&[])`) | Unaffected. |
| all other `keys::tests` (53) | no (key table only, or `with_areas`) | Unaffected: `keys.rs` not edited. |
| `ui::tests` on Welcome: `greeting_is_24_columns_but_12_characters`, `greeting_is_centred_by_display_width`, `greeting_starts_at_the_same_column_in_an_odd_width`, `a_fresh_session_shows_every_line`, `typed_character_and_elapsed_time_are_shown`, `fresh_welcome_shows_the_start_hint`, `welcome_lines_are_in_order`, `start_hint_is_centred_by_display_width`, `welcome_stays_centred_after_a_resize`, `welcome_draws_at_more_sizes_without_panicking`, `welcome_uses_no_colour`, `any_size_draws_without_panicking` | yes (`Session::new`, Welcome only) | Pass: the Welcome screen does not show the list. |
| `ui::tests::area_selection_placeholder_is_shown` | yes (`started`) | Passes: checks the heading and hint, which every list shows, and the absence of `日`, `Press Enter to start` and `Last key:`; `Grammar` contains none of them. |
| `ui::tests::area_selection_follows_a_resize` | yes | Passes: the hint is centred on its own row (column 27 at 100 columns) whatever the list holds. |
| `ui::tests::area_selection_draws_at_any_size_without_panicking`, `area_selection_uses_no_colour` | yes | Pass: one more plain line. |
| `ui::tests::empty_area_list_says_there_is_nothing_to_practise`, `empty_area_list_is_centred`, `empty_area_list_draws_at_any_size_without_panicking`, `empty_area_list_uses_no_colour`, `area_selection_with_no_areas_still_draws` | no (`started_with(&[])`) | Unaffected: the empty list is still drawn the same way. |
| `ui::tests` on the feedback screen: `feedback_for_a_correct_romaji_answer`, `feedback_for_a_correct_kana_answer`, `feedback_for_a_wrong_answer`, `feedback_lines_are_in_order`, `feedback_hint_is_centred`, `feedback_draws_at_any_size_without_panicking`, `feedback_uses_no_colour`, `score_after_two_answers`, `continue_is_not_counted`, `typo_warning_is_not_counted` | no | Pass with D10 (fact 3): rows are found by content, columns are horizontal only, and a short list still fits one row. |
| all other `ui::tests` | no | Unaffected. |
| `text::tests` (21) | no | Unaffected: no existing constant or function changes. |

**No existing test is in conflict**, so no task edits a test and nothing waits for the owner's approval of a test edit. If the implementer finds an existing test failing, the code is wrong: stop and report (AGENTS.md §7).

### D14. Content format and ids

- **Rust constant data in the area's file** (`const WORDS: &[Word]`), not an embedded text file with `include_str!` and a parser. Reasons: the compiler checks every field and every group name; there is no parser to write and test; no new dependency (a TOML/serde parser would need a licence check and adds code to the single binary); 49 one-line entries read well as code. Cross-field rules the compiler cannot check (hiragana readings, ASCII meanings, kanji in written forms, ichidan endings, nine questions per word, no duplicates) are checked by one test over the whole list (spec *Word list entries are valid*), so every word added later is checked too.
- **No ids in this change.** Nothing stores, references or deduplicates entries across runs (no saved history, no custom questions, no editing step); an id would be data with no reader, and clippy would flag a field never read. The architecture keeps ids and "the step that adds an entry and assigns its id" under *Still open*, now noting that Grammar uses constant data until a content file format is chosen.
- **What #11 and #12 reuse:** the pattern, not code: their own `Word`-like struct and `const` table in their own area file, a `const fn` row builder, and a validity test over the whole table. If they need content too large for code, the file format is decided there, and Grammar's table can move to it without changing its tests' expectations.

### D15. Task order, commits and unused code

Each task is one commit that builds and passes `cargo fmt --all -- --check`, `cargo clippy --all-targets -- -D warnings` and `cargo test` (AGENTS.md §5). Nothing may be unused in the non-test build at any commit, so a rule lands **together with the words that use it**, group by group, behind the already registered area:

1. **Verbs.** 1.1: the area, registered, with the ichidan rule and the 7 ichidan words, the wording, the content tests. `Group` has only `Ichidan` here, `Form` and `Formality` are fully used through `ASKED`. 1.2: the `Kind` marker (D12). 1.3: godan (`Group::Godan`, `GODAN`) and 21 words. 1.4: `Suru` and `Kuru` and 4 words. 1.5: `Iku` and `Aru` and 2 words.
2. **Adjectives.** 2.1: the feedback wrap (D10), before any question has several variants. 2.2: `IAdjective`, `Ii` and 8 words. 2.3: `NaAdjective`, 7 words, and the coverage test over every group.
3. **Docs.** 3.1: `docs/architecture.md`. 3.2: `CHANGELOG.md`, `README.md`, `ATTRIBUTION.md`.
4. **Check.** 4.1: the scripted terminal check (no file changes).

- A `Group` variant with no word would be "never constructed": that is why each variant arrives with its words.
- Tests that hold for any list (validity, nine questions, prompt width, levels) are added in 1.1 and automatically cover later words. Tests that need the whole list (every group present) come with the last group (2.3), so no test is ever edited later in the branch.
- No test asserts the exact number of words or entries, so adding words later never breaks a test.

### D16. Archive order

All three capabilities are new (ADDED only), so `openspec archive` of this change does not depend on another change. Recommended: archive it after `add-generic-practice`, whose `practice-loop` *Feedback screen* requirement `accepted-answers-display` complements, so the main specs are created in build order. `area-selection`'s *Empty list of areas* requirement ("When the app has registered no area ...") stays true as written and is not modified.

### D17. End-to-end check, and what still needs a person

**Scripted check (task 4.1, no person needed).** The release binary is driven in an 80x24 `tmux` session; `tmux capture-pane -p` returns the reconstructed screen, so ratatui's partial redraws do not matter. The script lives outside the repository (for example in the agent's scratch directory). Because questions are random, the script never predicts an answer: it learns answers from feedback and waits for a prompt to come round again (each N5 round asks all 35 words, one random form each, so prompts repeat within a few dozen questions):

1. Start `tmux new-session -d -s grammar -x 80 -y 24 "target/release/tui-learnjapanese; echo EXIT=\$? > <scratch>/exit.txt"`; after 0.5 s the screen contains `Press Enter to start`.
2. Enter → the screen contains `What do you want to practise?` and `> Grammar`.
3. Enter → `Grammar`, `Which JLPT levels do you want to practise?`, `> [x] N5`.
4. Enter → `Grammar`, a line starting with `What is the `, `Answer:`, `Answer in Japanese or romaji`, `Score: 0 of 0`.
5. Loop, at most 300 times: read the prompt row (the row containing `What is the `). If the script has an answer recorded for exactly this prompt, go to step 6. Otherwise type `neko` and Enter; the screen must contain `Your answer: neko`, `Read as: ねこ`, `Not quite.` and `Correct answer`; join the rows from the one containing `Correct answer` up to the row before `Score:`, take the text after `: `, split on `, `, and record for this prompt the first item that contains only hiragana; Enter; the screen must show a new question.
6. Convert the recorded kana to romaji (table below), type it and Enter: the screen must contain `Your answer: <romaji>`, `Read as: <kana>` and `Correct!`.
7. Esc → `What do you want to practise?`; Esc → within 2 s `exit.txt` contains `EXIT=0`.

Kana to romaji for step 6, by the app's own table: every kana by its first spelling there (し `shi`, ち `chi`, つ `tsu`, ふ `fu`, じ `ji`, を `wo`, ぢ `di`, づ `du`); a kana followed by small ゃ/ゅ/ょ as one syllable (きゃ `kya`, しゃ `sha`, じゃ `ja`, ちゃ `cha`, ...); っ as the first letter of the next syllable's romaji doubled (っち → `cchi`); ん always as `n'`. Any other character stops the script with an error. Record every step's result in the implementer's report; if a step fails, stop and report.

**What still needs a person.** The real IME check, deferred by #18 and #21 to the first area with content: on Linux, macOS and Windows terminals and over SSH, the owner (or a contributor with that platform) opens Grammar, and on the question screen types the answer with the system IME in hiragana, in katakana and converted to kanji, types it in romaji, and uses Backspace on IME text. Expected: each is accepted as described in `answer-checking`, Backspace removes one whole character, and the field draws CJK text by width. It is recorded (terminal, OS, result) in the pull request for #13, as the "Workflow follow-up" of `tasks.md` says. Who does it: the owner by default (Open Question 11).

## Risks / Trade-offs

- [Typing the reading answers the plain non-past of a verb or i-adjective] → Owner decision (all nine forms asked, only kanji blocks a copy). It still checks that the learner knows the dictionary form is the plain non-past. Open Question 1 offers hiding the reading.
- [では must be typed `deha` in romaji; `dewa` gives でわ and is wrong] → Same exact-romaji rule as everywhere (#18): it is how an IME reads it. A learner who types `dewa` sees `Read as: でわ` and the accepted forms. Open Question 4.
- [Feedback for a na-adjective lists up to eight answers] → D10 wraps them; they are grouped written/reading per variant so the pair is easy to spot.
- [In terminals narrower than about 60 columns the longest lists need more than four rows and the last answers are clipped] → Accepted; nothing panics (spec scenario at small sizes). The question itself and the score stay visible.
- [JLPT levels are approximate] → Stated as the project's judgment in `ATTRIBUTION.md` and the architecture; the picker still separates basic from less basic words.
- [A word misfiled under the wrong group yields wrong or missing forms] → The validity test requires nine questions per word and the ichidan ending rule; godan and adjective endings are checked by the rules themselves (no answer → test fails). A godan verb misfiled as ichidan whose reading ends in e/i-row + る (帰る as ichidan) cannot be caught by a rule; the word list in Appendix A was checked by hand, and Appendix B pins the forms of the risky ones (帰る).
- [Homophones across entries (会って and ある's あって)] → Different entries, never in the same question; no effect on checking.
- [The level picker starts with only N5 ticked, so N4 words are only seen after ticking N4] → Existing picker behaviour (#21), unchanged.

## Open Questions

For the owner (each has a default that the specs and tasks already follow):

1. **Show the reading in the prompt** (D7). Default: **yes**, `食べる (たべる, to eat)`. Alternative: only the written form and meaning; then romaji learners who cannot read 勉強 or 来 cannot answer, but the plain non-past is no longer "type the shown reading".
2. **Adjective variants and their order** (D5). Default: all standard variants accepted; `-ないです` before `-ありません`, じゃ before では.
3. **Na-adjective plain non-past** (D5). Default: **with だ only** (`静かだ`); the bare `静か` is not accepted.
4. **`dewa` for では** (risks). Default: **not accepted**, consistent with exact romaji. Accepting it would mean adding でわ spellings to `accepted`, which the content rules (no non-standard spellings) reject, or changing `grading`, which is out of scope.
5. **Wrapping the accepted answers in `ui.rs`** (D10). Default: **yes, up to 4 rows**. Alternative: accept only one variant per adjective form and leave `ui.rs` alone (task 2.1 is then dropped and D5's variant lists shrink to their first item).
6. **The `Kind` marker** (D12). Default: **moved to `Kind::Meaning`** with an updated reason. Alternative: left on the enum, comment and reason updated only.
7. **JLPT levels** (D9). Default: the project's own approximate judgment, stated as such. Alternative: a licensed source, to be found and recorded first.
8. **The word list** (Appendix A). Please review the 49 words, their meanings and levels; any word can be swapped for another of the same group without changing the design or the tests, except the words the tests name (食べる, 見る, the nine godan examples, する, 勉強する, 来る, 行く, ある, 高い, いい, 静か, きれい).
9. **よい / 良い for いい** (D5). Default: **not accepted**; いい is shown and its forms are asked.
10. **Where area names live** (D7). Default: **`src/text.rs`** (`AREA_GRAMMAR`), like all other wording.
11. **The real IME check** (D17). Default: **the owner**, on Linux, macOS, Windows and over SSH, recorded in the pull request for #13.

**Existing tests:** none is in conflict (D13), so no test edit needs approval.

## Appendix A: the word list

In this order in `WORDS` (`word(written, reading, meaning, group, level)`):

| # | Written | Reading | Meaning | Group | Level | Task |
|---|---|---|---|---|---|---|
| 1 | 食べる | たべる | to eat | Ichidan | N5 | 1.1 |
| 2 | 見る | みる | to see | Ichidan | N5 | 1.1 |
| 3 | 起きる | おきる | to get up | Ichidan | N5 | 1.1 |
| 4 | 寝る | ねる | to sleep | Ichidan | N5 | 1.1 |
| 5 | 教える | おしえる | to teach | Ichidan | N5 | 1.1 |
| 6 | 決める | きめる | to decide | Ichidan | N4 | 1.1 |
| 7 | 調べる | しらべる | to look up | Ichidan | N4 | 1.1 |
| 8 | 買う | かう | to buy | Godan | N5 | 1.3 |
| 9 | 会う | あう | to meet | Godan | N5 | 1.3 |
| 10 | 手伝う | てつだう | to help | Godan | N4 | 1.3 |
| 11 | 書く | かく | to write | Godan | N5 | 1.3 |
| 12 | 聞く | きく | to listen | Godan | N5 | 1.3 |
| 13 | 働く | はたらく | to work | Godan | N4 | 1.3 |
| 14 | 泳ぐ | およぐ | to swim | Godan | N5 | 1.3 |
| 15 | 急ぐ | いそぐ | to hurry | Godan | N4 | 1.3 |
| 16 | 話す | はなす | to speak | Godan | N5 | 1.3 |
| 17 | 貸す | かす | to lend | Godan | N5 | 1.3 |
| 18 | 待つ | まつ | to wait | Godan | N5 | 1.3 |
| 19 | 持つ | もつ | to hold | Godan | N5 | 1.3 |
| 20 | 死ぬ | しぬ | to die | Godan | N5 | 1.3 |
| 21 | 遊ぶ | あそぶ | to play | Godan | N5 | 1.3 |
| 22 | 運ぶ | はこぶ | to carry | Godan | N4 | 1.3 |
| 23 | 飲む | のむ | to drink | Godan | N5 | 1.3 |
| 24 | 読む | よむ | to read | Godan | N5 | 1.3 |
| 25 | 住む | すむ | to live | Godan | N4 | 1.3 |
| 26 | 帰る | かえる | to go home | Godan | N5 | 1.3 |
| 27 | 分かる | わかる | to understand | Godan | N5 | 1.3 |
| 28 | 送る | おくる | to send | Godan | N4 | 1.3 |
| 29 | する | する | to do | Suru | N5 | 1.4 |
| 30 | 勉強する | べんきょうする | to study | Suru | N5 | 1.4 |
| 31 | 運転する | うんてんする | to drive | Suru | N4 | 1.4 |
| 32 | 来る | くる | to come | Kuru | N5 | 1.4 |
| 33 | 行く | いく | to go | Iku | N5 | 1.5 |
| 34 | ある | ある | to exist | Aru | N5 | 1.5 |
| 35 | 高い | たかい | expensive | IAdjective | N5 | 2.2 |
| 36 | 安い | やすい | cheap | IAdjective | N5 | 2.2 |
| 37 | 大きい | おおきい | big | IAdjective | N5 | 2.2 |
| 38 | 新しい | あたらしい | new | IAdjective | N5 | 2.2 |
| 39 | 寒い | さむい | cold | IAdjective | N5 | 2.2 |
| 40 | いい | いい | good | Ii | N5 | 2.2 |
| 41 | 優しい | やさしい | kind | IAdjective | N4 | 2.2 |
| 42 | 悲しい | かなしい | sad | IAdjective | N4 | 2.2 |
| 43 | 静か | しずか | quiet | NaAdjective | N5 | 2.3 |
| 44 | きれい | きれい | pretty | NaAdjective | N5 | 2.3 |
| 45 | 元気 | げんき | healthy | NaAdjective | N5 | 2.3 |
| 46 | 好き | すき | liked | NaAdjective | N5 | 2.3 |
| 47 | 便利 | べんり | convenient | NaAdjective | N4 | 2.3 |
| 48 | 簡単 | かんたん | easy | NaAdjective | N4 | 2.3 |
| 49 | 大切 | たいせつ | important | NaAdjective | N4 | 2.3 |

## Appendix B: expected forms for the tests

Order of the nine forms everywhere: plain non-past, plain negative, plain past, plain past negative, te-form, polite non-past, polite negative, polite past, polite past negative. "Written / reading" pairs; a single text means written and reading are the same. Variants of one form are separated by `;` and listed in the rules' order.

**Ichidan**
- 食べる / たべる: 食べる/たべる, 食べない/たべない, 食べた/たべた, 食べなかった/たべなかった, 食べて/たべて, 食べます/たべます, 食べません/たべません, 食べました/たべました, 食べませんでした/たべませんでした
- 見る / みる: 見る/みる, 見ない/みない, 見た/みた, 見なかった/みなかった, 見て/みて, 見ます/みます, 見ません/みません, 見ました/みました, 見ませんでした/みませんでした

**Godan** (the reading of each form is the reading stem plus the same ending: 買わない/かわない)
- 買う / かう: 買う, 買わない, 買った, 買わなかった, 買って, 買います, 買いません, 買いました, 買いませんでした
- 書く / かく: 書く, 書かない, 書いた, 書かなかった, 書いて, 書きます, 書きません, 書きました, 書きませんでした
- 泳ぐ / およぐ: 泳ぐ, 泳がない, 泳いだ, 泳がなかった, 泳いで, 泳ぎます, 泳ぎません, 泳ぎました, 泳ぎませんでした
- 話す / はなす: 話す, 話さない, 話した, 話さなかった, 話して, 話します, 話しません, 話しました, 話しませんでした
- 待つ / まつ: 待つ, 待たない, 待った, 待たなかった, 待って, 待ちます, 待ちません, 待ちました, 待ちませんでした
- 死ぬ / しぬ: 死ぬ, 死なない, 死んだ, 死ななかった, 死んで, 死にます, 死にません, 死にました, 死にませんでした
- 遊ぶ / あそぶ: 遊ぶ, 遊ばない, 遊んだ, 遊ばなかった, 遊んで, 遊びます, 遊びません, 遊びました, 遊びませんでした
- 飲む / のむ: 飲む, 飲まない, 飲んだ, 飲まなかった, 飲んで, 飲みます, 飲みません, 飲みました, 飲みませんでした
- 帰る / かえる: 帰る, 帰らない, 帰った, 帰らなかった, 帰って, 帰ります, 帰りません, 帰りました, 帰りませんでした

Readings in full for two of them, as a check of the rule: かう, かわない, かった, かわなかった, かって, かいます, かいません, かいました, かいませんでした; しぬ, しなない, しんだ, しななかった, しんで, しにます, しにません, しにました, しにませんでした.

**Irregular**
- する: する, しない, した, しなかった, して, します, しません, しました, しませんでした
- 勉強する / べんきょうする: 勉強する/べんきょうする, 勉強しない/べんきょうしない, 勉強した/べんきょうした, 勉強しなかった/べんきょうしなかった, 勉強して/べんきょうして, 勉強します/べんきょうします, 勉強しません/べんきょうしません, 勉強しました/べんきょうしました, 勉強しませんでした/べんきょうしませんでした
- 来る / くる: 来る/くる, 来ない/こない, 来た/きた, 来なかった/こなかった, 来て/きて, 来ます/きます, 来ません/きません, 来ました/きました, 来ませんでした/きませんでした
- くる (written in kana, test of the reading table alone): くる, こない, きた, こなかった, きて, きます, きません, きました, きませんでした
- 行く / いく: 行く/いく, 行かない/いかない, 行った/いった, 行かなかった/いかなかった, 行って/いって, 行きます/いきます, 行きません/いきません, 行きました/いきました, 行きませんでした/いきませんでした
- ある: ある, ない, あった, なかった, あって, あります, ありません, ありました, ありませんでした

**I-adjectives**
- 高い / たかい: 高い/たかい, 高くない/たかくない, 高かった/たかかった, 高くなかった/たかくなかった, 高くて/たかくて, 高いです/たかいです, 高くないです/たかくないです; 高くありません/たかくありません, 高かったです/たかかったです, 高くなかったです/たかくなかったです; 高くありませんでした/たかくありませんでした
- いい: いい, よくない, よかった, よくなかった, よくて, いいです, よくないです; よくありません, よかったです, よくなかったです; よくありませんでした

**Na-adjectives**
- 静か / しずか: 静かだ/しずかだ, 静かじゃない/しずかじゃない; 静かではない/しずかではない, 静かだった/しずかだった, 静かじゃなかった/しずかじゃなかった; 静かではなかった/しずかではなかった, 静かで/しずかで, 静かです/しずかです, 静かじゃないです/しずかじゃないです; 静かじゃありません/しずかじゃありません; 静かではないです/しずかではないです; 静かではありません/しずかではありません, 静かでした/しずかでした, 静かじゃなかったです/しずかじゃなかったです; 静かじゃありませんでした/しずかじゃありませんでした; 静かではなかったです/しずかではなかったです; 静かではありませんでした/しずかではありませんでした
- きれい: きれいだ, きれいじゃない; きれいではない, きれいだった, きれいじゃなかった; きれいではなかった, きれいで, きれいです, きれいじゃないです; きれいじゃありません; きれいではないです; きれいではありません, きれいでした, きれいじゃなかったです; きれいじゃありませんでした; きれいではなかったです; きれいではありませんでした

**Accepted answers of a question** (written then reading per variant, no duplicates):
- 食べる, polite past: `["食べました", "たべました"]`
- 高い, polite negative: `["高くないです", "たかくないです", "高くありません", "たかくありません"]`
- 静か, polite negative: `["静かじゃないです", "しずかじゃないです", "静かじゃありません", "しずかじゃありません", "静かではないです", "しずかではないです", "静かではありません", "しずかではありません"]`
- する, polite non-past: `["します"]`; する, plain negative: `["しない"]`
- 来る, plain negative: `["来ない", "こない"]`
- 行く, te-form: `["行って", "いって"]`
