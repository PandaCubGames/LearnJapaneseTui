# Design

## Context

See `proposal.md` for the motivation and the three specs (`romaji-conversion`, `answer-checking`, `answer-input`) for the required behaviour.

- The crate is a single binary (`src/main.rs` declares `app`, `keys`, `session`, `text`, `ui`). `cargo clippy --all-targets -- -D warnings` fails on any item that non-test code never reaches. This was checked in a scratch crate: unused functions, methods and types are errors.
- `keys::action_for` already turns `KeyCode::Char(c)` with no modifiers or only `SHIFT` into `Action::Typed(c)` on the Welcome screen. Text committed by an IME reaches the app the same way: the terminal sends UTF-8 and crossterm reports one `KeyCode::Char` per character (on Windows, one key event per character from the console input records). Backspace arrives as `KeyCode::Backspace` (byte `0x7F` on Unix, `VK_BACK` on Windows); byte `0x08` arrives as `Ctrl+H`.
- crossterm 0.29 is built with its `bracketed-paste` feature, so `Event::Paste(String)` exists, but `ratatui::try_init` does not enable bracketed paste. Pasted text therefore arrives as ordinary key events, one per character.
- `Cargo.lock` already contains `unicode-width` 0.2.3 and `unicode-segmentation` 1.13.3 (both `MIT OR Apache-2.0`), pulled in by `ratatui-core`. They are not direct dependencies, but `ratatui::text::Span` exposes both through `Span::width()` and `Span::styled_graphemes()`. `src/ui.rs` already relies on `Line::width()` for display width.
- No question screen exists. The generic practice module (#21) will build it and consume this change.
- Baseline: 51 passing tests (`cargo test`, checked while writing this design).

## Goals / Non-Goals

**Goals:**
- #21 can take a Japanese answer, check it and give feedback by calling a small, fixed API, without knowing any romaji or kana rule.
- Every owner decision in issue #18 is pinned by a unit test that needs no terminal.
- No existing test, key mapping, screen or dependency changes.

**Non-Goals:**
- The question screen, its wording (including the typo warning text), its key arm in `keys.rs`, new `Action` variants, counting answers, and showing feedback. All of that is #21.
- Cursor movement inside the text, selection, delete-forward, word deletion, undo, and editing in the middle of the text. The field only appends and deletes at the end.
- Enabling bracketed paste (Open Question 4).
- Kana to kanji conversion, live conversion while typing, English answers, half-width katakana, full-width Latin letters.

## Decisions

### D1. Landing code that no screen uses yet: a scoped, self-removing lint expectation

Each new module is declared in `src/main.rs` with:

```rust
// No screen uses this module yet: the practice question screen (issue #21)
// does, and removes this attribute. Tests use every item already.
#[cfg_attr(
    not(test),
    expect(dead_code, reason = "used by the practice question screen, issue #21")
)]
mod grading;
```

- **Why `expect` and not `allow`:** an `expect` that suppresses nothing is itself a warning (`unfulfilled_lint_expectations`), which `-D warnings` turns into an error. So when #21 makes the module fully used, clippy forces #21 to delete the attribute; it cannot be forgotten. Verified in a scratch crate with Rust 1.100: passes while unused, fails with "this lint expectation is unfulfilled" once everything is used. `#[expect]` is stable since Rust 1.81; the project's `rust-version` is 1.88.
- **Why `cfg_attr(not(test), ...)`:** in the test build the tests use every item, so `dead_code` does not fire and a plain `expect` would be unfulfilled there. Consequence for the implementer: **every item in these modules must be used by a test**, or the test build warns.
- **Scope:** the attribute sits on the four `mod` lines only. Nothing in the existing modules is touched.

Alternatives considered:
- **A demo use on an existing screen** (for example an answer field on the area selection placeholder or on Welcome). Rejected: it contorts the app, conflicts with the other change in flight for area selection (#20), and the tests that pin the demo would later have to be changed or deleted, which `AGENTS.md` §7 forbids without the owner.
- **Lib plus bin crate** (`src/lib.rs` with `pub mod`, `main.rs` using it). `pub` items of a library are never dead code. Rejected for now: it moves every existing module or splits the crate in two, changes how existing tests are compiled, and is a structural decision for the owner, not a side effect of #18.
- **Defer the field to #21.** Does not solve the problem: the romaji and grading functions are just as unused. It also splits issue #18 across two changes.
- **Plain `#[allow(dead_code)]`.** Works, but nothing removes it later; dead code could hide behind it for good.

**Owner decision (approved):** this is the first lint suppression in the code base, and the owner has approved it for now, as described above: the scoped `#[cfg_attr(not(test), expect(dead_code, reason = ...))]` on each new `mod` line in `src/main.rs`, nowhere else. Issue #21 removes each attribute when it first uses that module (clippy's `unfulfilled_lint_expectations` makes a forgotten one fail the build).

### D2. The public API for #21

Four new files. Every item below is `pub`; everything else is private.

```rust
// src/romaji.rs
/// Romaji to hiragana, letter for letter. Every valid reading, textbook
/// reading first (D4): `nn` before a vowel can be read two ways. `None`
/// when the text has no valid reading; `Some(v)` has 1 to 16 readings,
/// all different.
pub fn to_hiragana(romaji: &str) -> Option<Vec<String>>;

// src/kana.rs
/// Japanese characters: kana, kanji, CJK punctuation, variation selectors (D5).
pub fn is_japanese(c: char) -> bool;
/// Kanji: CJK ideographs and the marks 々 〆 〇 (D5). Every kanji is also Japanese.
pub fn is_kanji(c: char) -> bool;
/// Joins a kana and a following combining voiced (U+3099) or semi-voiced (U+309A) mark.
pub fn compose_voicing(text: &str) -> String;
/// Katakana with a hiragana counterpart becomes that hiragana; all else unchanged.
pub fn katakana_to_hiragana(text: &str) -> String;

// src/grading.rs
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Outcome {
    /// Empty or only whitespace. Nothing to check; not counted.
    Empty,
    /// Cannot be read as Japanese. From `check`: always. From `TypoGuard`: first time only.
    LooksLikeTypo,
    /// Matches an accepted answer. `read_as` is the reading that matched (D6).
    Correct { read_as: String },
    /// Counted wrong answer. `read_as` is `None` for mixed, macron or unreadable
    /// input; otherwise the reading to show in feedback (D6).
    Wrong { read_as: Option<String> },
}

/// Checks one answer. Pure: same input, same result. Never remembers anything.
pub fn check(typed: &str, accepted: &[&str], shown: &[&str]) -> Outcome;

/// The typo memory of one question.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct TypoGuard { warned: Option<String> }

impl TypoGuard {
    /// Like `check`, but an unreadable text submitted again unchanged is `Wrong { read_as: None }`.
    pub fn submit(&mut self, typed: &str, accepted: &[&str], shown: &[&str]) -> Outcome;
}

// src/answer_field.rs
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Edit { Insert(char), DeleteBack }

/// The edit a terminal event makes to an answer field, if any (D8).
pub fn edit_for(event: &Event) -> Option<Edit>;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct AnswerField { text: String }

impl AnswerField {
    pub fn text(&self) -> &str;
    pub fn apply(&mut self, edit: Edit);
    /// Draws on the first row of `area`; returns where the cursor goes, or
    /// `None` when `area` has no columns or no rows.
    pub fn render(&self, area: Rect, buf: &mut Buffer) -> Option<Position>;
}
```

(`Event` is `ratatui::crossterm::event::Event`; `Rect` and `Position` are from `ratatui::layout`; `Buffer` is `ratatui::buffer::Buffer`.)

**What #21 supplies:**
- `accepted`: every accepted answer of the question, written in Japanese (kana and/or kanji), for example `["高い", "たかい"]`. Romaji is not allowed in content; an accepted answer written in romaji is compared as text and will never match a romaji answer. Validating content is the content change's job.
- `shown`: the Japanese text the question already displays, for example `["食"]` for "How do you read 食?", or `["食べる", "たべる"]` for "What is the polite past of 食べる?". `&[]` when the prompt shows no Japanese. Only a shown text that contains a kanji has an effect: an answer typed in Japanese that equals it is wrong, so the user cannot copy the kanji back (D6). A shown text in kana only never makes an answer wrong, so #21 may list it or leave it out with the same result.
- One `TypoGuard::default()` and one `AnswerField::default()` per question, replaced when the next question starts.
- The key arm of its question screen: Enter submits `field.text()` to `guard.submit(...)`, Esc leaves, and otherwise `answer_field::edit_for(event)` gives the edit. `Edit` is `Copy`, so #21 can wrap it in an `Action` variant without breaking `#[derive(Clone, Copy)]` on `Action`.
- Drawing: `if let Some(p) = field.render(area, frame.buffer_mut()) { frame.set_cursor_position(p) }`.
- All wording (the typo warning, "correct", "wrong", the expected answers) in `src/text.rs`.

**What #21 gets back:** an `Outcome`. It counts `Correct` and `Wrong`, never `Empty` or `LooksLikeTypo`. On `LooksLikeTypo` it shows the warning and keeps the field's text so the user can correct it. On `Empty` it does nothing. `read_as` is shown in feedback ("read as ときょ") when present; the expected answers come from #21's own `accepted` list. When romaji has two readings (`nn` before a vowel, D4), `read_as` is the reading that matched, or the textbook reading for a wrong answer (D6); #21 never sees the list.

**The two-readings rule (D4) changes no part of this API:** `check`, `TypoGuard::submit` and `Outcome` keep the signatures and variants above. Only `romaji::to_hiragana`, which #21 does not call, returns the list; `grading` consumes it internally.

**Why the memory is a separate `TypoGuard`:** `check` stays a pure function that is easy to test exhaustively, and the only state the rule needs (the last warned text) is one `Option<String>` that #21 owns per question. #21 does not have to re-implement the rule.

### D3. Module layout

| File | Contents | Uses |
|---|---|---|
| `src/kana.rs` | `is_japanese`, `is_kanji`, `compose_voicing`, `katakana_to_hiragana` | nothing |
| `src/romaji.rs` | `to_hiragana` and its private table | nothing |
| `src/grading.rs` | `Outcome`, `check`, `TypoGuard` and private helpers | `kana`, `romaji` |
| `src/answer_field.rs` | `Edit`, `edit_for`, `AnswerField` | `ratatui` |

`docs/architecture.md` §5 already names `grading` and `romaji`. `kana` and `answer_field` are added to its component diagram (tasks 3.2 and 4.2). Every new file starts with the SPDX header and a `//!` module comment, like the existing files.

### D4. Romaji table and algorithm

**Table** (romaji → hiragana). Spellings separated by `/` give the same kana. 175 entries; the longest key has 4 letters.

| Row | Entries |
|---|---|
| vowels | `a` あ, `i` い, `u` う, `e` え, `o` お |
| k | `ka` か, `ki` き, `ku` く, `ke` け, `ko` こ |
| g | `ga` が, `gi` ぎ, `gu` ぐ, `ge` げ, `go` ご |
| s | `sa` さ, `shi`/`si` し, `su` す, `se` せ, `so` そ |
| z | `za` ざ, `ji`/`zi` じ, `zu` ず, `ze` ぜ, `zo` ぞ |
| t | `ta` た, `chi`/`ti` ち, `tsu`/`tu` つ, `te` て, `to` と |
| d | `da` だ, `di` ぢ, `du`/`dzu` づ, `de` で, `do` ど |
| n | `na` な, `ni` に, `nu` ぬ, `ne` ね, `no` の |
| h | `ha` は, `hi` ひ, `fu`/`hu` ふ, `he` へ, `ho` ほ |
| b | `ba` ば, `bi` び, `bu` ぶ, `be` べ, `bo` ぼ |
| p | `pa` ぱ, `pi` ぴ, `pu` ぷ, `pe` ぺ, `po` ぽ |
| m | `ma` ま, `mi` み, `mu` む, `me` め, `mo` も |
| y | `ya` や, `yu` ゆ, `yo` よ |
| r | `ra` ら, `ri` り, `ru` る, `re` れ, `ro` ろ |
| w | `wa` わ, `wo` を |
| ky, gy | `kya` きゃ, `kyu` きゅ, `kyo` きょ, `gya` ぎゃ, `gyu` ぎゅ, `gyo` ぎょ |
| sh | `sha`/`sya` しゃ, `shu`/`syu` しゅ, `sho`/`syo` しょ, `she`/`sye` しぇ |
| j | `ja`/`jya`/`zya` じゃ, `ju`/`jyu`/`zyu` じゅ, `jo`/`jyo`/`zyo` じょ, `je`/`jye`/`zye` じぇ |
| ch | `cha`/`tya` ちゃ, `chu`/`tyu` ちゅ, `cho`/`tyo` ちょ, `che`/`tye` ちぇ |
| dy | `dya` ぢゃ, `dyu` ぢゅ, `dyo` ぢょ |
| ny, hy | `nya` にゃ, `nyu` にゅ, `nyo` にょ, `hya` ひゃ, `hyu` ひゅ, `hyo` ひょ |
| by, py | `bya` びゃ, `byu` びゅ, `byo` びょ, `pya` ぴゃ, `pyu` ぴゅ, `pyo` ぴょ |
| my, ry | `mya` みゃ, `myu` みゅ, `myo` みょ, `rya` りゃ, `ryu` りゅ, `ryo` りょ |
| f | `fa` ふぁ, `fi` ふぃ, `fe` ふぇ, `fo` ふぉ |
| v | `va` ゔぁ, `vi` ゔぃ, `vu` ゔ, `ve` ゔぇ, `vo` ゔぉ |
| ts | `tsa` つぁ, `tsi` つぃ, `tse` つぇ, `tso` つぉ |
| other | `thi` てぃ, `dhi` でぃ, `wi` うぃ, `we` うぇ, `ye` いぇ |
| small | `xa`/`la` ぁ, `xi`/`li` ぃ, `xu`/`lu` ぅ, `xe`/`le` ぇ, `xo`/`lo` ぉ, `xya`/`lya` ゃ, `xyu`/`lyu` ゅ, `xyo`/`lyo` ょ, `xtu`/`ltu`/`xtsu`/`ltsu` っ, `xwa`/`lwa` ゎ, `xka`/`lka` ゕ, `xke`/`lke` ゖ |

There is deliberately **no** entry for a lone `n`, for `c` other than in `ch`, `cc` and `tch`, for `q`, for `wu`, `yi`, `ca`/`ci`/`cu`, or for macron vowels.

**Implementation shape:** a private `const TABLE: &[(&str, &str)]` with one pair per spelling (175 pairs), searched by key. A linear search is fine (answers are a few letters; 175 comparisons per position). A test checks that no key appears twice.

**Algorithm**, `to_hiragana(romaji)`:

1. If any character is not ASCII, return `None` at once (all valid input is ASCII). Lower-case the letters (`str::to_ascii_lowercase`).
2. Start with `readings = vec![String::new()]` (one empty reading) and `forks = 0`. "Push `k`" below means: append `k` to **every** reading in `readings`.
3. Walk the text left to right with a byte index `i`. At each position, with `c` the current letter and `next` the following one (or none):
   1. `c == '-'` → push `ー`, advance 1.
   2. `c == 'n'`:
      - `next` is `'` → push `ん`, advance 2. (`n'` never forks.)
      - `next` is `n`: look at the text after the two `n`s. Let `w` = 1 if it starts with a vowel (`a i u e o`), `w` = 2 if it starts with `ya`, `yu` or `yo`, else `w` = 0.
        - `w > 0` → **a two-reading spot** (see *Two readings of `nn`* below): `textbook` = `ん` + the kana of the table key `text[i+1 .. i+2+w]` (`na`…`no`, `nya`, `nyu`, `nyo`); `keyboard` = `ん` + the kana of the table key `text[i+2 .. i+2+w]` (`a`…`o`, `ya`, `yu`, `yo`). If `forks < MAX_FORKS`: `forks += 1` and replace `readings` by a new list that holds, for each old reading `r` in order, `r + textbook` and then `r + keyboard`. Otherwise push `textbook` only. Advance `2 + w`.
        - `w == 0` → push `ん`, advance 2.
      - `next` is none, or is neither a vowel nor `y` → push `ん`, advance 1.
      - otherwise fall through to the table lookup (step 3.5), which finds `na`, `nya`, ... or fails.
   3. `c` is a consonant other than `n` (one of `b c d f g h j k l m p q r s t v w x y z`) and `next == c` → push `っ`, advance 1.
   4. `c == 't'` and the text continues with `ch` → push `っ`, advance 1.
   5. Otherwise look up the longest table key that the text continues with, trying 4, 3, 2 and then 1 letters. Found → push its kana and advance by the key length. Not found → return `None`.
4. Return `Some(readings)`. The empty text gives `Some(vec![String::new()])`: one reading, empty.

`MAX_FORKS` is a private `const MAX_FORKS: usize = 4;` in `src/romaji.rs`.

**Two readings of `nn`** (owner decision). Learners type `nn` before a vowel in two ways: keyboard style, as a Japanese IME reads it (`nn` = ん, then the vowel: `konnichiha` → こんいちは), and textbook style, where `nn` is the ん plus the `n` of an `n`-row syllable (`konnichiha` → こんにちは). Both are accepted, so `konnichiha` and `konnnichiha` are both correct for こんにちは, and `onna` and `onnna` both correct for おんな.

- **When a spot forks.** Exactly when `nn` is directly followed by a vowel or by `ya`, `yu` or `yo`, and the first of the two `n`s is not itself the second letter of an `nn` already read: the walk reaches a spot only at its first `n`. These are the only cases in which *both* readings exist (the table has `na`…`no` and `nya`/`nyu`/`nyo`, but no `nyi` or `nye`).
- **No fork:** `nn` at the end or before a consonant, `-` or `'` (ん, both letters consumed); `nn` before `y` followed by anything other than `a`, `u`, `o` (ん, then the walk goes on as before: `onnye` おんいぇ, `konnyk` unreadable); `n'` (always ん); a single `n` (unchanged rules).
- **Three `n`s** (`konnnichiha`): at the first `n` the walk sees `nn` followed by `n`, which is not a vowel, so it pushes ん and advances 2; the third `n` then starts `ni`. One reading, こんにちは. There is no こんんいちは-style reading: that would need the walk to stand on the second `n`, which it never does, because the first two `n`s are consumed together. This is also how IMEs read `nnn` + vowel, and two ん in a row before a vowel does not occur in real words. `onnna` gives おんな only; `nnnn` gives んん.
- **Doubled consonants:** `n` is excluded from the っ rule (step 3.3), so `nn` is never read as っ + `n`-row.
- **Capitals:** lower-cased first, so `KONNICHIHA` has the same two readings as `konnichiha`.
- **Both readings consume the same letters** (`nn` + vowel = 3 letters, `nn` + `ya`/`yu`/`yo` = 4) and so continue from the same position: everything after a spot is read identically in both. Consequences: a spot never makes a text readable or unreadable (text is unreadable exactly when it has no reading at all, as before); the readings are all different (they differ in the kana right after the spot's ん); and with `k` forks there are exactly `2^k` readings.
- **Bound.** At most `MAX_FORKS = 4` spots fork, so at most 16 readings. From the fifth spot on, only the textbook reading is pushed. Real answers have 0 to 2 spots; the cap exists because the answer is user text of any length, and `nna` repeated 20 times would otherwise produce 2^20 strings and freeze the app. Pushing the textbook reading beyond the cap keeps the first reading textbook-style at every spot.
- **Order** (deterministic): by the choice made at each spot, the first spot most significant, textbook before keyboard. So `readings[0]` is the **textbook reading** (textbook at every spot), and with two spots the order is textbook-textbook, textbook-keyboard, keyboard-textbook, keyboard-keyboard. `kinnennen` → `きんねんねん`, `きんねんえん`, `きんえんねん`, `きんえんえん`.
- **Signature:** `Option<Vec<String>>`. `None` = cannot be read; `Some(v)` = readable, with `v` non-empty, at most 16 entries, no duplicates, `v[0]` the textbook reading. A small newtype was considered and rejected: `grading` is the only caller, and every `pub` method of a newtype would need its own test under the D1 rule. The function keeps its name because it still converts romaji to hiragana.

Prototype check (a throwaway Python version of this algorithm and table, outside the repository): `konnichiha` → [こんにちは, こんいちは]; `konnnichiha` → [こんにちは]; `onna` → [おんな, おんあ]; `onnna` → [おんな]; `kinnen` → [きんねん, きんえん]; `konnya` → [こんにゃ, こんや]; `kinen` → [きねん]; `kin'en` → [きんえん]; `nn` → [ん]; `honn` → [ほん]; `konnyk` → none; `nna` × 5 → 16 readings, first `んなんなんなんなんな`, last `んあんあんあんあんな`.

Decisions taken by this algorithm (the issue did not decide them; the ones a user would notice are Open Question 2):

| Case | Decision | Example |
|---|---|---|
| `nn` before a vowel or `ya`/`yu`/`yo` | Two readings, textbook first, keyboard second (owner decision, above). | `konnichiha` こんにちは / こんいちは, `kinnen` きんねん / きんえん, `konnya` こんにゃ / こんや |
| `nn` elsewhere | `ん`, consuming both letters, as Japanese IMEs do. One reading. | `honn` ほん, `nn` ん, `konnnichiha` こんにちは, `onnna` おんな, `onnye` おんいぇ |
| `n'` | Always `ん`, one reading. The apostrophe is valid only right after a single `n`. | `kin'en` きんえん; `ta'kai`, `'takai`, `kan''i`, `nn'a` unreadable |
| `n` before `y` | Starts `nya`/`nyu`/`nyo`; `ny` + anything else is unreadable. | `konya` こにゃ, `kon'ya` こんや |
| `m` before `b`, `p`, `m` | Not `ん`. `mb` and `mp` spell nothing, so the text is unreadable; `mm` is a doubled consonant. | `shimbun` unreadable, `shinbun` しんぶん |
| っ before `ch` | Both `tch` and `cch`. | `matcha`, `maccha` まっちゃ |
| `wo` | を. `o` is お. | |
| `di`, `du`, `dzu` | `di` ぢ, `du` and `dzu` づ. `ji` and `zi` are always じ, `zu` always ず. | |
| `wi`, `we` | うぃ, うぇ (as IMEs type them), not the archaic ゐ, ゑ. | |
| `ha`, `he`, `wa`, `o` for particles | No special case: `ha` は, `wa` わ. Romaji is exact, so は is typed `ha`. | |
| `-` | `ー` anywhere, also at the start. | `ko-hi-` こーひー |
| Spaces | Not romaji: `to_hiragana` rejects them. `grading` removes whitespace before calling it (D6). | |
| Other punctuation (`.`, `,`, `!`, `?`) | Not romaji: unreadable. | `taka.` unreadable |
| Digits | Unreadable. | `taka1` |
| Trailing lone consonant | Unreadable (no table key matches). | `tabek`, `kk`, `ky` |
| Upper case | Lower-cased first. | `KIN'EN` きんえん, `KONNICHIHA` こんにちは / こんいちは |

### D5. Japanese characters, voicing and katakana folding (`kana.rs`)

**`is_japanese(c)`** is true exactly for these code point ranges (inclusive):

| Range | What |
|---|---|
| U+3000–U+303F | CJK symbols and punctuation (`、`, `。`, `「`, `」`, `々`, `〆`, `〇`, `〜`, ...) |
| U+3040–U+309F | Hiragana, including combining U+3099/U+309A, `゛`, `゜`, `ゝ`, `ゞ` |
| U+30A0–U+30FF | Katakana, including `ー`, `・`, `ヽ`, `ヾ` |
| U+31F0–U+31FF | Katakana phonetic extensions |
| U+3400–U+4DBF | CJK Extension A |
| U+4E00–U+9FFF | CJK Unified Ideographs |
| U+F900–U+FAFF | CJK Compatibility Ideographs |
| U+20000–U+3FFFF | CJK Extensions B and later (supplementary ideographic planes) |
| U+FE00–U+FE0F, U+E0100–U+E01EF | Variation selectors (kanji variants) |

Not Japanese, on purpose: U+FF00–U+FFEF (full-width Latin, full-width punctuation and half-width katakana, all out of scope), and everything else.

**`is_kanji(c)`** is true exactly for these code point ranges (inclusive). It is used only for the repeat rule (D6): a shown text "contains a kanji" when at least one of its characters is a kanji.

| Range | What |
|---|---|
| U+3005–U+3007 | `々` (iteration mark), `〆` (closing mark), `〇` (ideographic zero) |
| U+3400–U+4DBF | CJK Extension A |
| U+4E00–U+9FFF | CJK Unified Ideographs |
| U+F900–U+FAFF | CJK Compatibility Ideographs |
| U+20000–U+3FFFF | CJK Extensions B and later (supplementary ideographic planes) |

- **Every kanji is Japanese:** each range lies inside an `is_japanese` range, so a text with a kanji is never classified as romaji or as unreadable for that reason.
- **`々`, `〆` and `〇` count as kanji.** They sit in the CJK symbols block, not among the ideographs, but they are written in place of kanji: `々` repeats the kanji before it (`人々`, `時々`), `〆` is read しめ, `〇` is a kanji numeral (`二〇二六`). Like a kanji, each is something the learner has to supply the reading for, so copying it back must not count. In a real word `々` always follows a kanji, so the choice only changes the result when a question shows `々` (or `〆`, `〇`) on its own, for example "How do you read 々?"; then typing `々` back is wrong, as typing `食` back is.
- **Not kanji:** kana, `ー`, punctuation (`、`, `。`, `「`), the other CJK symbols (U+3000–U+3004, U+3008–U+303F), combining marks, variation selectors on their own, CJK radicals (U+2E80–U+2FDF, which `is_japanese` does not accept either), and everything outside the table.

**`compose_voicing(text)`**: for each character followed by U+3099 (combining voiced mark) or U+309A (combining semi-voiced mark), replace the pair by the precomposed kana when one exists; otherwise leave both characters as they are. All other characters are copied unchanged.

- U+3099 after a kana of か–ご, さ–ぞ, た–ど, は–ぽ rows: the voiced kana is the base code point **+ 1**, for these bases only: hiragana `かきくけこさしすせそたちつてとはひふへほ`, katakana `カキクケコサシスセソタチツテトハヒフヘホ`.
- U+309A after `はひふへほ` / `ハヒフヘホ`: base **+ 2**.
- U+3099 after `う` → `ゔ` (U+3094), `ウ` → `ヴ` (U+30F4), `ゝ` → `ゞ`, `ヽ` → `ヾ`, `ワ` → `ヷ` (U+30F7), `ヰ` → `ヸ`, `ヱ` → `ヹ`, `ヲ` → `ヺ` (U+30F8–U+30FA).

This is what Unicode NFC does for these characters, without a normalisation crate. It matters for text pasted from places that store decomposed kana (for example macOS file names).

**`katakana_to_hiragana(text)`**: U+30A1–U+30F6 (`ァ` to `ヶ`) become the code point **− 0x60** (`ァ`→`ぁ`, `ヴ`→`ゔ`, `ヵ`→`ゕ`, `ヶ`→`ゖ`); `ヽ` (U+30FD) → `ゝ`, `ヾ` (U+30FE) → `ゞ`. Everything else, including `ー`, `・`, `ヷ`–`ヺ`, the phonetic extensions and kanji, is unchanged.

### D6. Classifying and checking an answer (`grading.rs`)

Private helper `normalise_japanese(text) -> String` = remove every character for which `char::is_whitespace` is true (this includes U+3000), then `kana::compose_voicing`, then `kana::katakana_to_hiragana`. It is applied to every accepted answer, every shown text and every Japanese answer, so typed text and shown text are compared in the same form.

Private helper `is_marked_vowel(c) -> bool`: true for `ā ī ū ē ō Ā Ī Ū Ē Ō â î û ê ô Â Î Û Ê Ô` (precomposed only).

A romaji character is an ASCII letter, `'`, `-` or a marked vowel. A Latin letter is an ASCII letter or a marked vowel.

**The repeat rule** (owner decision, narrowed after review): an answer is wrong as a repeat only when it equals Japanese text the question shows **and** that shown text contains at least one kanji (`kana::is_kanji`, D5). A shown text written only in kana never makes an answer wrong. The rule exists for one reason: when a question shows a word written with kanji (`食べる`), the user must not be able to answer by copying and pasting that text back. Typing its reading is a real answer: with `食べる` shown, `食べる` is wrong but `たべる` and `taberu` are correct; with `する` shown, `する` and `suru` are correct.

Private helper `blocked(shown) -> Vec<String>`: `normalise_japanese(s)` for every `s` in `shown` whose normalised text contains at least one kanji (`.chars().any(kana::is_kanji)`), in order; the others are dropped. (Normalising neither adds nor removes kanji, so testing before or after normalising gives the same result; testing the normalised text keeps one value per shown text.)

`check(typed, accepted, shown)`:

1. `text` = `typed` with every whitespace character removed (anywhere). If `text` is empty → `Outcome::Empty`.
2. Classify `text`, in this order:
   1. **Mixed**: it contains at least one Japanese character (`kana::is_japanese`) **and** at least one Latin letter → `Wrong { read_as: None }`. (Owner decision: `食beru`, `たberu`.)
   2. **Japanese**: every character is Japanese → `r = normalise_japanese(text)`. If `r` equals a text in `blocked(shown)` → `Wrong { read_as: Some(r) }` (the repeat rule). Otherwise `readings = vec![r]` (always exactly one).
   3. **Romaji**: every character is a romaji character:
      - it contains a marked vowel → `Wrong { read_as: None }`. (Owner decision: `tōkyō` is wrong. It is not a typo warning, and it is never read with short vowels, because that would make `kōkō` correct for ここ.)
      - otherwise `romaji::to_hiragana(text)`: `Some(rs)` with `rs` non-empty → `readings = rs`; `None` → `LooksLikeTypo`. (`to_hiragana` never returns an empty list; if it ever did, treat it like `None`, so nothing indexes an empty list.)
   4. **Anything else** (a digit, full-width Latin, half-width katakana, `@`, a Japanese text with `-` or `'` in it, ...) → `LooksLikeTypo`.
3. With `readings` (non-empty, in the order `to_hiragana` gives, so `readings[0]` is the textbook reading):
   - if any reading equals `normalise_japanese(a)` for any `a` in `accepted` → `Correct { read_as: r }`, where `r` is the first reading, in list order, that equals an accepted answer;
   - else → `Wrong { read_as: Some(readings[0]) }`: the textbook reading.

**Why the repeat check is only in the Japanese branch:** a romaji answer's readings are made of hiragana and `ー` only (the D4 table has nothing else), and every blocked text contains a kanji, so a romaji reading can never equal a blocked text. Only an answer typed in Japanese, with a kanji in it, can be a repeat. Checking romaji readings against `blocked` would be code that can never fire, so there is none. Consequence: a romaji answer is never wrong because of `shown`, and no rule has to choose which of several readings "repeats" the shown text.

**`read_as`**, complete:
- **Correct:** the reading that matched. For romaji with several readings, the first in list order that equals an accepted answer: `konnya` against `["今夜", "こんや"]` is correct, read as こんや (its keyboard reading); `konnichiha` against `["こんにちは"]` is read as こんにちは.
- **Wrong (readable):** the textbook reading, `readings[0]` (owner preference), so a learner who types `konnichiwa` for こんばんは sees `こんにちわ`, the kana they expect, not `こんいちわ`. For romaji with one reading, that reading.
- **A Japanese answer has exactly one reading**, its normalised text (katakana folded, kanji kept). It is the `read_as` whether the answer is correct, wrong, or wrong as a repeat: `食ベル` with `食べる` shown is wrong, read as `食べる`.
- **No reading** (`Wrong { read_as: None }`): mixed and macron answers, and from `TypoGuard` an unreadable text submitted twice (D7).

Notes:
- Comparison is exact string equality after normalisation: `normalise_japanese` on both sides (whitespace anywhere removed, voiced marks composed, katakana folded to hiragana; kanji kept). So `食 べる`, `食ベル` and `食べる` all repeat a shown `食べる`. "Repeats what the question shows" means the whole normalised answer equals a whole normalised shown text that contains a kanji; containing it is not enough (`食べる` is not rejected for showing `食`).
- A shown text that is also an accepted answer is still wrong when typed back, if it contains a kanji: with `食` shown and `食` accepted, `食` is wrong. Its kana form, if accepted, stays correct.
- Whitespace is removed everywhere, not only at the ends, so `tabete kudasai` matches `たべてください`. Japanese answers have no meaningful spaces.
- Japanese punctuation is not removed: `たかい。` does not match `たかい`. Content does not put punctuation in accepted answers.
- `read_as` for a Japanese answer keeps kanji: `高い` reads as `高い`, `タカイ` as `たかい`.

### D7. Typo memory (`TypoGuard`)

```text
submit(typed, accepted, shown):
    outcome = check(typed, accepted, shown)
    match outcome:
        Empty         -> return Empty                     (memory unchanged)
        LooksLikeTypo -> if warned == Some(typed):        (exact, byte for byte, untrimmed)
                             warned = None; return Wrong { read_as: None }
                         else:
                             warned = Some(typed); return LooksLikeTypo
        other         -> warned = None; return other
```

- The guard compares the **typed text**, never readings, so the two readings of `nn` (D4) change nothing here. A text is unreadable, and can get a warning, only when it has no reading at all (`to_hiragana` returns `None`).
- "Same text" is the raw field text, compared exactly. `tabxru` and `tabxru ` are different texts, so the second gets its own warning. This is the literal rule of the issue and the simplest to explain.
- **Undecided in the issue, default here:** a *different* unreadable text after a warning (`tabxru`, then `tabwru`) gets **another warning**, as the issue's rule "text changed: checked from the start" says. The user can always give up by submitting the same text twice. Open Question 1.
- An empty submit changes nothing, so `tabxru`, empty, `tabxru` is a counted wrong answer.
- A readable submit clears the memory, but it ends the question anyway in #21's flow (it goes to feedback).

### D8. The answer field (`answer_field.rs`)

**Keys, `edit_for(event)`:**

1. Not an `Event::Key` (paste, resize, focus, mouse) → `None`.
2. `key.kind == KeyEventKind::Release` → `None`. `Press` and `Repeat` both count, so held keys repeat.
3. `KeyCode::Char('h')` with modifiers exactly `CONTROL` → `Some(Edit::DeleteBack)` (some terminals send `0x08` for Backspace, which crossterm reports as `Ctrl+H`).
4. `KeyCode::Backspace` with modifiers empty or only `SHIFT` → `Some(Edit::DeleteBack)`.
5. `KeyCode::Char(c)` with modifiers empty or only `SHIFT` (`(key.modifiers - KeyModifiers::SHIFT).is_empty()`, the test `keys.rs` already uses) and `!c.is_control()` → `Some(Edit::Insert(c))`.
6. Anything else → `None`. This includes Enter, Esc, arrows, Tab, Delete, Home, End and `Ctrl`/`Alt` combinations. Enter and Esc belong to #21's screen. `Ctrl+C` is handled by `keys::action_for` before any screen arm, as today.

`edit_for` lives with the field, not in `keys.rs`, so every future screen with a field types the same way, and `keys.rs` (with its existing tests) is not touched now. #21 calls it from its screen arm in `keys::action_for`.

**Editing, `apply(edit)`:**
- `Insert(c)`: if `c.is_control()` do nothing; else `text.push(c)`.
- `DeleteBack`: remove the last grapheme cluster:
  ```rust
  let span = Span::raw(self.text.as_str());
  let last = span.styled_graphemes(Style::default()).last().map(|g| g.symbol.len());
  if let Some(len) = last { let keep = self.text.len() - len; self.text.truncate(keep); }
  ```
  (`Span` from `ratatui::text`, `Style` from `ratatui::style`.) Compute `last` in its own statement so the borrow of `self.text` ends before `truncate`.

**Grapheme clusters, not `char`:** a user-perceived character can be several `char`s: a kana with a combining voiced mark, a kanji with a variation selector, an emoji sequence. Removing one `char` would leave a stray mark or selector the user cannot see. ratatui draws by grapheme cluster (`styled_graphemes`), so deleting by the same unit keeps what is deleted equal to what is seen. The segmentation comes from `unicode-segmentation` through ratatui; no new dependency.

**Drawing, `render(area, buf)`:**
1. `area.width == 0 || area.height == 0` → return `None`.
2. `room = area.width - 1` columns (one column is kept for the cursor).
3. Walk the grapheme clusters of `text` from the end. Width of one cluster: `Span::raw(symbol).width()`. Keep adding clusters while the total width stays `<= room`; stop at the first that does not fit (so a wide character is never cut in half).
4. `visible` = the suffix of `text` made of the kept clusters (its byte start is `text.len()` minus the sum of their `symbol.len()`).
5. `buf.set_stringn(area.x, area.y, visible, usize::from(room), Style::default())`.
6. Return `Some(Position::new(area.x + w, area.y))`, where `w` is the visible width converted with `u16::try_from(..).unwrap_or(u16::MAX)` (the project avoids `as` casts).

Checked against ratatui 0.30.2 in a scratch crate: `たかい` in 10 columns puts `た`, `か`, `い` at 0, 2, 4 and the cursor at 6; `abcdefたか` in 6 columns shows `fたか` with the cursor at 5; `あいう` in 4 columns shows `う` with the cursor at 2; `食べる` in 2 columns shows nothing with the cursor at 0; a 0x0 area gives `None`.

No colour or style is used. Only the first row is drawn. The field draws nothing around itself (no border, no prompt); #21 lays those out.

**IME:** the app never sees the IME's composition, only the committed characters, which arrive as `KeyCode::Char` key events like typed letters. While the IME composes, the terminal (not the app) shows the preedit text; its position is wherever the terminal's cursor is, which is why `render` returns a cursor position for #21 to set.

### D9. No new dependency

`Span::width()` (backed by `unicode-width`) and `Span::styled_graphemes()` (backed by `unicode-segmentation`) come from `ratatui`, already a dependency; katakana folding, voicing and classification use code point arithmetic and ranges. So `Cargo.toml` and `Cargo.lock` do not change. If a future change needs grapheme or width functions without a `Span`, adding `unicode-segmentation = "1"` and `unicode-width = "0.2"` as direct dependencies would add no new crate to the build (both are already in `Cargo.lock`, `MIT OR Apache-2.0`, compatible with GPL-3.0-or-later).

### D10. Task order and commits

Each task is one commit that builds and passes `cargo fmt --all -- --check`, `cargo clippy --all-targets -- -D warnings` and `cargo test` on its own:

- 1.1 `kana.rs` (no dependencies on other new modules).
- 2.1 `romaji.rs`: table and basic syllables, `-`, upper case, unreadable input. Already returns `Option<Vec<String>>` (always one reading at this point).
- 2.2 `romaji.rs`: っ and ん rules, with `nn` always read as ん (one reading).
- 2.3 `romaji.rs`: the two readings of `nn` before a vowel or `ya`/`yu`/`yo`, their order and the `MAX_FORKS` bound.
- 3.1 `grading.rs`: classification and `check`.
- 3.2 `grading.rs`: `TypoGuard`, plus the `CHANGELOG.md` entry and the `docs/architecture.md` rules for checking.
- 4.1 `answer_field.rs`: `Edit`, `edit_for`, `AnswerField` with `text` and `apply`.
- 4.2 `answer_field.rs`: `render`, plus the docs for the field and the status note.
- 5.1 Pseudo-terminal check that committed Japanese text arrives as typed characters (no file changes).

Each module's `mod` line with the D1 attribute is added in the task that creates the file. Tasks 2.1, 2.2 and 2.3 split `romaji` so each commit is reviewable; the っ and ん tests are new tests in 2.2 and the two-readings tests are new tests in 2.3, never edits of earlier tests. To make that possible, 2.1 already uses the final signature, and 2.2's tests use only inputs whose result 2.3 does not change (no `nn` directly before a vowel or `ya`/`yu`/`yo`). Documentation lands with the group whose behaviour it describes.

### D11. Existing tests

None is changed, and none can be affected:

- The four new modules are new files; existing modules do not call them.
- `src/main.rs` gains four `mod` lines with attributes. No function in it changes.
- `src/keys.rs`, `src/session.rs`, `src/ui.rs`, `src/text.rs` and `src/app.rs` are not touched, so all 51 existing tests (`app` 4, `keys` 15, `session` 9, `text` 7, `ui` 16) run against unchanged code.
- `Cargo.toml` and `Cargo.lock` do not change, so no dependency version moves under the existing tests.

## Risks / Trade-offs

- [**Known cost of the two readings of `nn`:** `onna` is accepted for both おんな and おんあ, `kinnen` for both きんねん and きんえん, `konnya` for both こんにゃ and こんや] → Owner decision. It only matters if both readings were accepted answers to one question, which real content does not do. (A shown text cannot interfere: only shown text with a kanji blocks an answer, and romaji readings are kana, D6.) Romaji stays exact everywhere else: `kinen` is only きねん and `kin'en` only きんえん.
- [Beyond four `nn` spots in one answer, only the textbook reading is read] → Needed to bound the work on arbitrary user text (D4). No real answer has more than two spots.
- [Strict romaji (`shimbun`, `tokyo`) frustrates users who learned other spellings] → Owner decision for long vowels. `shimbun` gets a typo warning first, so the user sees it was not read. Open Question 2.
- [The lint expectation hides dead code until #21] → Approved by the owner (D1). It is scoped to four modules, every item has tests, and it removes itself through `unfulfilled_lint_expectations`.
- [A paste that contains a newline arrives as Enter and would submit in #21] → Bracketed paste is not enabled; Open Question 4.
- [On Windows with some keyboard layouts, `AltGr` characters arrive with `CONTROL | ALT` modifiers and are not typed] → Romaji needs only `a`–`z`, `'` and `-`, which no common layout puts on `AltGr`; IME text arrives without modifiers. Checked by hand in Open Question 3.
- [A question that shows a kana-only word which is also its answer is easy: typing the shown kana back is correct (for example "What is the plain non-past of する?")] → Owner decision: the repeat rule is only there to stop copying kanji (D6). Whether to ask such a question is the content's decision (#13), not this module's.
- [A question that shows the kanji word and accepts only that kanji form can never be answered correctly] → Content error, not a checking rule: content that shows a word with kanji also accepts its kana form (`食べる` shown, `たべる` accepted). Validating content is the content change's job, as for romaji in `accepted`.
- [Answers with typos in content (romaji in `accepted`) never match] → Documented contract in D2; content checks belong to the content changes.
- [ambiguous-width characters] → The field only displays what the user typed; it adds no text of its own, so `src/text.rs`'s ambiguous-width rule is not affected.

## Open Questions

For the owner:

Already decided by the owner, recorded here so they are not asked again: the scoped unused-code expectation (D1, approved), both readings of `nn` before a vowel (D4), and the repeat rule applying only to shown text that contains a kanji (D6).

1. **A different unreadable text after a warning** (`tabxru`, then `tabwru`). Default: another warning, as the issue's rule is written (D7). Alternative: any second unreadable submit in a row is a counted wrong answer. Changing this changes only `TypoGuard::submit` and the scenario *A different unreadable text*.
2. **Romaji details not decided in the issue** (D4). Defaults: `m` before `b`/`p` is not ん (`shimbun` unreadable); both `tch` and `cch` give っち; `wi`/`we` give うぃ/うぇ; `di` ぢ and `du`/`dzu` づ; whitespace is ignored anywhere; punctuation is not ignored (`takai.` unreadable, `たかい。` wrong); macron vowels are always wrong; an empty submit is ignored (`Outcome::Empty`, not counted). Please confirm or name the ones to change.
3. **Checking IME input on real terminals.** It cannot be automated. Task 5.1 checks in a Linux pseudo-terminal that committed Japanese text (`たかい高い`, written as UTF-8 bytes) arrives as typed characters on today's Welcome screen. The real check (an actual IME on Linux, macOS and Windows terminals, and over SSH) needs a person and the answer field on screen, so it belongs to #21: the owner, or a contributor with that platform, types `たかい`, `タカイ`, `高い`, `takai` and Backspace into the question screen and records the terminal, OS and result in #21's PR. Default: done by the owner in #21; please confirm who.
4. **Bracketed paste.** Not enabled here: today a paste arrives as typed characters (works), but a newline in it arrives as Enter. Enabling it needs `EnableBracketedPaste` in `src/app.rs` and handling `Event::Paste` in #21. Default: leave to #21.
5. **Existing tests.** None is affected (D11); no owner decision is needed.
