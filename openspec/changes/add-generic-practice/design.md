# Design

## Context

See `proposal.md` for the motivation (including the prerequisite changes #18 and #20) and the five specs for the required behaviour.

State of the code this design starts from, **after #18 and #20 are merged** (their designs are the source; neither is implemented yet):

- `keys::action_for(screen, event)` turns an event into an `Action` per `Screen`; `Session::apply` is the only place state changes; `ui::draw` matches on `Session::screen()`. `Screen` and `Action` derive `Debug, Clone, Copy, PartialEq, Eq`; `Session` derives `Debug`.
- From #20: `src/areas.rs` with `pub trait Area: Debug { fn name(&self) -> &'static str; fn has_questions(&self) -> bool; }`, `areas::all()`, three unit structs in `src/areas/{vocabulary,kanji,grammar}.rs` whose `has_questions` returns `false`, `#[cfg(test)] pub mod fake` with `FakeArea { name, has_questions }`, `Session::with_areas(width, height, areas)`, `highlighted_area()`, `Screen::{AreaSelection, NoQuestions}`, `Action::{Previous, Next, Choose, Back}`, `ui::draw_centred`, `text::{AREA_MARKER, AREA_NO_MARKER}`. Its `Session::apply` has the branch `// An area with questions starts practice here (issue #21).` in the `Choose` arm.
- From #18: `src/{kana,romaji,grading,answer_field}.rs`, each declared in `src/main.rs` with `#[cfg_attr(not(test), expect(dead_code, reason = "..."))]` and a two-line comment. API used here: `grading::TypoGuard` (`Default`, `submit(&mut self, typed: &str, accepted: &[&str], shown: &[&str]) -> Outcome`), `grading::Outcome::{Empty, LooksLikeTypo, Correct { read_as: String }, Wrong { read_as: Option<String> }}`, `answer_field::Edit` (`Copy`), `answer_field::edit_for(&Event) -> Option<Edit>`, `answer_field::AnswerField` (`Default`, `text()`, `apply(Edit)`, `render(Rect, &mut Buffer) -> Option<Position>`).
- The crate is a single binary: `cargo clippy --all-targets -- -D warnings` fails on dead code. Facts checked in a scratch crate with the project's toolchain (rustc 1.100, ratatui 0.30.2) while writing this design:
  1. A struct or enum that appears in the signature of a live trait method (here `Area::questions`) gets **no** dead-code warning in the non-test build, even though no non-test code constructs it, and its fields get no "never read" warning either. So `Entry`, `Question` and `Kind` need no lint attribute (D13).
  2. A `pub` method that only tests call **is** dead code (`method ... is never used`). A field read only through a derived `Debug` is "never read". So every method added here must have a non-test caller in the same commit, and every field must be read.
  3. `pub use` re-exports inside `practice.rs` produce no warning when the items are used through them.
  4. A derived `Default` counts as constructing the type.
  5. SplitMix64 from seed 0 gives `0xe220a8397b1dcdaf`, `0x6e789e6aa1b965f4`, `0x06c45d188009454f`.
  6. In 80 columns, `Layout::horizontal([Length(8), Length(30)]).flex(Flex::Center)` places the label at x = 21 and the field at x = 29; in 100 columns at 31 and 39. A centred `Paragraph` keeps leading spaces, so `> [x] N5` and `  [ ] N4` put `[` in the same column (38 in 80 columns). `Paragraph::wrap` breaks a long CJK text without spaces at the width.

## Goals / Non-Goals

**Goals:**
- An area implements two methods (`name`, `questions`) and gets the level picker, question picking, the question and feedback screens, checking and counting for free (D2).
- Everything is testable without a terminal, including the random order (D6) and the whole loop through the key table with a test-only area (D12).
- No existing test, and no test added by #18 or #20, changes (D14).
- No new dependency.

**Non-Goals:**
- The question count and the results screen (practice goes on until Esc; the score line stands in until then).
- Any real content; multiple choice; bracketed paste (Open Question 7).
- Kana-to-kanji conversion or live conversion (as in #18).

## Decisions

### D1. Module layout

| File | Contents | Task |
|---|---|---|
| `src/practice.rs` | module doc; `mod level; mod meaning; mod pool; mod question; mod rng;`; `pub use level::{Level, Levels}; pub use question::{Entry, Kind, Question}; pub use rng::Rng;`; `pub fn selection`; `Drill`; `Feedback` | 1.1 (created), grows |
| `src/practice/level.rs` | `Level`, `Levels` | 1.1, 1.2, 2.1 |
| `src/practice/question.rs` | `Kind`, `Question`, `Entry` | 2.1 |
| `src/practice/rng.rs` | `Rng` (SplitMix64) | 3.1 |
| `src/practice/pool.rs` | `Pool` (entry order) | 3.1 |
| `src/practice/meaning.rs` | `Outcome`, `check` (English answers) | 4.1, 4.3 |
| `src/areas/sample.rs` | test-only `Sample`, `Only`, `READING`, `MEANING`, `POLITE_PAST`, `entries()` | 2.1 |

- `practice` is the generic module the issue asks for. It never names an area; `session` connects the chosen area to it. `areas` depends on `practice` for the contract types, not the other way round.
- `src/main.rs` gets `mod practice;` after `mod keys;`. The `pub use` lines are added in the task that adds each submodule.

### D2. The contract

```rust
// src/practice/level.rs
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Level { N5, N4, N3, N2, N1 }

impl Level {
    /// Every level, in the order the level picker lists them.
    pub const ALL: [Level; 5] = [Level::N5, Level::N4, Level::N3, Level::N2, Level::N1];
    fn bit(self) -> u8 { match self { Level::N5 => 1, Level::N4 => 2, Level::N3 => 4, Level::N2 => 8, Level::N1 => 16 } }
}

/// A set of JLPT levels.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Levels(u8);

impl Levels {
    pub const ALL: Levels = Levels(0b1_1111);                                       // 2.1
    pub fn only(level: Level) -> Levels { Levels(level.bit()) }                     // 1.1
    pub fn contains(self, level: Level) -> bool { self.0 & level.bit() != 0 }       // 1.1
    pub fn toggled(self, level: Level) -> Levels { Levels(self.0 ^ level.bit()) }   // 1.2
    pub fn is_empty(self) -> bool { self.0 == 0 }                                   // 1.2
    pub fn intersects(self, other: Levels) -> bool { self.0 & other.0 != 0 }        // 2.1
}

// src/practice/question.rs
/// How an answer is typed and checked.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    /// A Japanese answer: romaji, kana or kanji, checked by `grading`.
    Japanese,
    /// A meaning in plain English, checked by `practice::meaning`.
    Meaning,
}

/// One question, as the practice module sees it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Question {
    /// What the user is asked, ready to show.
    pub prompt: String,
    /// Every accepted answer. Japanese answers are written in kana or kanji, never romaji.
    pub accepted: Vec<String>,
    /// The Japanese texts the prompt already shows. An answer typed back as one of them
    /// is wrong when that text contains a kanji, so kanji cannot be copied and pasted;
    /// kana-only texts never make an answer wrong. Empty when the prompt shows no
    /// Japanese. Ignored for `Meaning`.
    pub shown: Vec<String>,
    pub kind: Kind,
}

/// One entry of an area (a word, a kanji, a verb), with its questions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    pub levels: Levels,
    pub questions: Vec<Question>,
}

// src/areas.rs (the trait after this change)
pub trait Area: std::fmt::Debug {
    /// The name shown in the list of areas.
    fn name(&self) -> &'static str;
    /// The area's entries that have at least one of `levels`, each with all its
    /// questions, generated and hand-written alike. The practice module drops
    /// any entry without a chosen level or without questions. No entries by default.
    fn questions(&self, _levels: Levels) -> Vec<Entry> { Vec::new() }
    /// Whether the area has any question at all. Built on `questions`; areas do not
    /// override it.
    fn has_questions(&self) -> bool {
        !practice::selection(self.questions(Levels::ALL), Levels::ALL).is_empty()
    }
}
```

- **Grouped by entry** because the picking rule (D7) works on entries. An entry's template and custom questions are simply all in its `questions`.
- **The area filters by level, and the practice module filters again** (`practice::selection`, D7). The area can skip work for entries it will not need; the any-level rule and "no empty entries" still live in one place and cannot be got wrong by an area. A test with an area that ignores the levels pins this.
- **`Levels` is a 5-bit set**, `Copy`, so `Session` and tests pass it around freely. Its field is private; outside the module, sets are built with `only` and `toggled` (an empty set is `Levels::only(Level::N5).toggled(Level::N5)`). Methods are added in the task that first uses them (Context fact 2).
- **`shown` is part of the question** because #18's repeat rule needs it, and only the area knows what its prompt shows. The rule exists for one reason: when the prompt shows a word written with kanji (`食べる`), the user must not be able to answer by copying that text back. So an answer equal to a shown text is wrong only when that text contains a kanji; a kana-only shown text never makes an answer wrong (#18, design D6). Areas list everything Japanese their prompt shows and need not decide which entries matter.
- **`has_questions` keeps its name and signature** (as #20's D2 allows) and gets a default body, so it can never disagree with `questions`. The three real areas lose their `fn has_questions(&self) -> bool { false }` (task 2.1): with the default they still have no questions, and #11, #12 and #13 then only add `questions`, instead of having to remember to delete an override that would otherwise hide their content.
- **`questions` has a default body** (no entries) so #20's `FakeArea`, which only implements `name` and `has_questions`, compiles unchanged and keeps its meaning (D14). An area that forgets `questions` shows "No questions in this area yet", which is honest. Alternative considered: a required `questions`; rejected because it would force an edit of #20's `FakeArea` test helper and of the three real area files for no behaviour.

### D3. What a real area implements (Grammar, #13, sketch)

```rust
impl Area for Grammar {
    fn name(&self) -> &'static str { text::AREA_GRAMMAR }
    fn questions(&self, levels: Levels) -> Vec<Entry> {
        words()                                            // parsed from include_str!
            .filter(|word| word.levels.intersects(levels))
            .map(|word| Entry {
                levels: word.levels,
                questions: FORMS.iter()
                    .filter_map(|&(form, formality)| {
                        let answer = conjugation::conjugate(&word, form, formality)?;
                        Some(Question {
                            kind: Kind::Japanese,
                            prompt: text::conjugation_prompt(word.written(), form, formality),
                            accepted: vec![answer.written, answer.reading],
                            shown: vec![word.written().to_string(), word.reading.clone()],
                        })
                    })
                    .chain(word.special_questions.iter().map(Question::from))
                    .collect(),
            })
            .collect()
    }
}
```

So the contract is enough for conjugation: one entry per word, nine generated questions plus any custom ones, accepted answers in kanji and kana, and the dictionary form as `shown`. **Note for #13:** for the plain non-past form the answer *is* the dictionary form, and that needs no special handling. With `食べる` shown (and `shown = ["食べる", "たべる"]`), typing the kanji form `食べる` back is wrong, but `たべる` and `taberu` are correct, because only shown text with a kanji blocks an answer. For a kana-only word such as `する`, nothing is blocked: `する` and `suru` are correct. So #13 can ask all nine forms with `shown` filled as in the sketch, and the contract does not change. The remaining wrinkle is that this one question is easy: its answer is the reading of the word the prompt shows (and for a kana-only word, the shown text itself). Whether to ask it anyway is #13's content decision.

### D4. Owned `String`s, not `&'static str` or `Cow`

Grammar answers are generated at run time and prompts are formatted (`What does 高い mean?`), so they cannot be `&'static str`. `Cow<'static, str>` would save copying hand-written text from `include_str!` content, but the whole Vocabulary list is a few thousand short strings built once when practice starts; the cost is negligible and `String` is simpler for every area. `grading::TypoGuard::submit` takes `&[&str]`, so `Drill::submit` builds `Vec<&str>` views (`question.accepted.iter().map(String::as_str).collect()`).

### D5. Test-only areas: `src/areas/sample.rs`

`src/areas.rs` gets `#[cfg(test)] pub mod sample;` next to its other `mod` lines (the only change to `areas.rs` besides the trait). The file holds:

```rust
#[derive(Debug)]
pub struct Sample;                                 // name "Sample", all of entries(), filtered by level

/// One question of the Sample area on its own, for tests that need a known question.
#[derive(Debug)]
pub struct Only { pub entry: usize, pub question: usize }
pub const READING: Only = Only { entry: 0, question: 0 };      // How do you read 高い?
pub const MEANING: Only = Only { entry: 0, question: 1 };      // What does 高い mean?
pub const POLITE_PAST: Only = Only { entry: 1, question: 0 };  // polite past of 食べる

pub fn entries() -> Vec<Entry>
```

`entries()`, in this order:

| # | Levels | Questions (kind, prompt, accepted, shown) |
|---|---|---|
| 0 | N5 | Japanese, `How do you read 高い?`, [`たかい`], [`高い`]; Meaning, `What does 高い mean?`, [`high`, `tall`, `expensive`], [] |
| 1 | N5, N4 | Japanese, `What is the polite past of 食べる?`, [`食べました`, `たべました`], [`食べる`, `たべる`] |
| 2 | N3 | Japanese, `How do you read 山?`, [`やま`], [`山`] |

No entry has N2 or N1. `Sample::questions(levels)` returns the entries of `entries()` whose levels intersect `levels`. `Only::questions(levels)` returns entry `self.entry` with only its question `self.question`, if that entry intersects `levels`, else nothing. Both are named `Sample`.

- The Sample area is listed only through `Session::with_areas` in tests, never in `areas::all()`. That it can be practised start to finish while `practice.rs`, the screens and the keys know nothing about it is the "adding an area means listing it" proof the issue asks for.
- Its name is a literal, not a `text.rs` constant: it is never shown to users.

### D6. Randomness: a built-in SplitMix64, seeded by `app`

```rust
// src/practice/rng.rs
/// SplitMix64: small and fast, good enough to shuffle questions. Not for cryptography.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rng { state: u64 }

impl Rng {
    pub fn new(seed: u64) -> Rng { Rng { state: seed } }
    pub fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.state;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }
    /// A number in `0..n`; 0 when `n` is 0.
    pub fn below(&mut self, n: usize) -> usize {
        let n = u64::try_from(n).unwrap_or(u64::MAX);
        let wide = u128::from(self.next_u64()) * u128::from(n);
        usize::try_from(wide >> 64).unwrap_or(0)
    }
}
```

- **Why not the `rand` crate:** it (and `fastrand`, `oorandom`) is MIT/Apache-2.0 and would be allowed by `AGENTS.md` §6, but it adds a dependency (and for `rand`, several) for 15 lines of well-known public-domain arithmetic, and its output for a given seed is not promised to stay the same across versions, which would make seeded tests fragile. SplitMix64 has published reference values (Context fact 5), which a test pins.
- `below` uses the multiply-shift reduction (Lemire). Its bias is at most `n / 2^64`, irrelevant here. No `as` casts.
- **Seeding:** `Session::with_areas` starts with `Rng::new(0)` (constant `DEFAULT_SEED: u64 = 0` in `session.rs`), so every test is deterministic by default. A new builder `pub fn with_seed(self, seed: u64) -> Session` replaces it. `app::run_loop` builds `Session::new(w, h).with_seed(seed_from(SystemTime::now()))`, where

  ```rust
  // src/app.rs
  /// A seed for the order of questions, taken from the clock so each run differs.
  pub fn seed_from(now: SystemTime) -> u64 {
      let since = now.duration_since(UNIX_EPOCH).unwrap_or_default();
      since.as_secs().rotate_left(32) ^ u64::from(since.subsec_nanos())
  }
  ```

  The clock stays in `app`, which already owns time (`Ticker`), and `session` stays free of side effects. `with_areas` keeps #20's signature.
- **One `Rng` per session**, kept across practice runs, so choosing the same area twice gives a different order.

### D7. Picking: `selection` and `Pool`

```rust
// src/practice.rs
/// The entries practised at `levels`: those with any of `levels` and at least one
/// question, in the order given.
pub fn selection(entries: Vec<Entry>, levels: Levels) -> Vec<Entry> {
    entries.into_iter().filter(|e| e.levels.intersects(levels) && !e.questions.is_empty()).collect()
}

// src/practice/pool.rs
/// Picks entries in random order, each once per round.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pool { len: usize, left: Vec<usize>, last: Option<usize> }

impl Pool {
    pub fn new(len: usize) -> Pool { Pool { len, left: Vec::new(), last: None } }
    /// The next entry, an index in `0..len`; `None` when `len` is 0.
    pub fn next(&mut self, rng: &mut Rng) -> Option<usize> {
        if self.len == 0 { return None; }
        if self.left.is_empty() { self.left = (0..self.len).collect(); }
        let mut i = rng.below(self.left.len());
        if self.left.len() > 1 && Some(self.left[i]) == self.last {
            let j = rng.below(self.left.len() - 1);
            i = if j >= i { j + 1 } else { j };
        }
        let entry = self.left.swap_remove(i);
        self.last = Some(entry);
        Some(entry)
    }
}
```

- An entry is removed from `left` when picked, so it cannot come back until `left` is refilled, i.e. until every entry has been picked: the issue's rule.
- `Some(self.left[i]) == self.last` can only be true right after a refill (otherwise `last` is no longer in `left`). Then another entry is drawn uniformly among the others, so a round never starts with the entry just asked. This avoids the same entry twice in a row across rounds, which the issue's rule alone allows; with one entry it is unavoidable and allowed.
- Then a question of the entry is drawn with `rng.below(questions.len())`. Questions of the same entry may repeat across rounds; within a round an entry is asked once.

### D8. State

Practice state that lives only while practising is one value, `Option<Drill>` in `Session`; the level picker's state lives directly in `Session` because it outlives a run.

```rust
// src/session.rs, new fields
levels: Levels,               // ticked levels; Levels::only(Level::N5) at start, never reset
level_highlighted: usize,     // index into Level::ALL; 0 each time the picker opens
rng: Rng,                     // Rng::new(DEFAULT_SEED), replaced by with_seed
drill: Option<Drill>,         // Some while on Question or Feedback

// src/practice.rs
#[derive(Debug)]
pub struct Drill {
    entries: Vec<Entry>,          // the selection; every entry has a question       (3.1)
    pool: Pool,                   // kept from 4.1; in 3.1 `start` uses a local Pool   (4.1)
    entry: usize,                 // current entry index                               (3.1)
    question: usize,              // current question index in that entry              (3.1)
    field: AnswerField,           //                                                   (3.2)
    guard: TypoGuard,             // one per question                                  (4.1)
    typo_warning: bool,           // the warning is on screen                          (4.1)
    feedback: Option<Feedback>,   // Some on the feedback screen                       (4.1)
    answered: u32,                //                                                   (4.2)
    correct: u32,                 //                                                   (4.2)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Feedback {
    pub correct: bool,
    /// The answer as typed, without spaces at both ends.
    pub answer: String,
    /// The reading, when the answer was read as Japanese and it differs from `answer`.
    pub read_as: Option<String>,
}
```

`Drill` methods (each added in the task in brackets):

| Method | Behaviour |
|---|---|
| `pub fn start(entries: Vec<Entry>, levels: Levels, rng: &mut Rng) -> Option<Drill>` (3.1) | `selection(entries, levels)`; `Pool::new(len)`; first `pool.next(rng)?`; question `rng.below(..)`; empty field, default guard, no warning, no feedback, counts 0. `None` when the selection is empty. |
| `pub fn question(&self) -> &Question` (3.1) | `&self.entries[self.entry].questions[self.question]` |
| `pub fn field(&self) -> &AnswerField` (3.2) | |
| `pub fn edit(&mut self, edit: Edit)` (3.2; 4.1 adds the warning) | Does nothing while `feedback` is `Some`; else `field.apply(edit)` and `typo_warning = false`. |
| `pub fn submit(&mut self) -> bool` (4.1; 4.2 counts) | D9 below. `true` when the answer was counted and feedback is now shown. |
| `pub fn next(&mut self, rng: &mut Rng)` (4.1) | Next entry from the pool and a random question of it; new `AnswerField::default()` and `TypoGuard::default()`; `typo_warning = false`; `feedback = None`. Counts unchanged. |
| `pub fn typo_warning(&self) -> bool`, `pub fn feedback(&self) -> Option<&Feedback>` (4.1) | |
| `pub fn answered(&self) -> u32`, `pub fn correct(&self) -> u32` (4.2) | |

- `Action` stays `Copy`: the new actions carry only `answer_field::Edit`, which is `Copy`. `Screen` stays `Copy`: the typo warning and the feedback are state in `Drill`, not screen variants.
- `Session` stays `#[derive(Debug)]`: `Levels`, `Rng`, `Drill` (and through it `AnswerField`, `TypoGuard`, `Entry`) all derive `Debug`.
- **Leaving practice** drops the `Drill`. That is how "an unfinished answer is dropped and not counted" and "the counts start at zero" hold without extra code.

### D9. Checking an answer (`Drill::submit`)

```rust
pub fn submit(&mut self) -> bool {
    if self.feedback.is_some() { return false; }
    let typed = self.field.text().to_string();
    let question = &self.entries[self.entry].questions[self.question];
    let accepted: Vec<&str> = question.accepted.iter().map(String::as_str).collect();
    let (correct, read_as) = match question.kind {
        Kind::Japanese => {
            let shown: Vec<&str> = question.shown.iter().map(String::as_str).collect();
            match self.guard.submit(&typed, &accepted, &shown) {
                Outcome::Empty => return false,
                Outcome::LooksLikeTypo => { self.typo_warning = true; return false; }
                Outcome::Correct { read_as } => (true, Some(read_as)),
                Outcome::Wrong { read_as } => (false, read_as),
            }
        }
        Kind::Meaning => match meaning::check(&typed, &accepted) {
            meaning::Outcome::Empty => return false,
            meaning::Outcome::Correct => (true, None),
            meaning::Outcome::Wrong => (false, None),
        },
    };
    let answer = typed.trim().to_string();
    let read_as = read_as.filter(|reading| *reading != answer);
    self.feedback = Some(Feedback { correct, answer, read_as });
    // 4.2: self.answered = self.answered.saturating_add(1); if correct { self.correct = self.correct.saturating_add(1); }
    true
}
```

- Counting happens only on the path that sets `feedback`, so a typo warning, an empty submit and a submit while feedback is shown are never counted, and continuing (`next`) never counts.
- `self.guard` and `self.entries` are different fields, so borrowing `question` while calling `self.guard.submit` compiles; `typed` is cloned to keep the borrow checker out of the way.

### D10. Screens, actions and `Session::apply`

```rust
pub enum Screen { Welcome, AreaSelection, NoQuestions, LevelPicker, NoQuestionsAtLevels, Question, Feedback }

pub enum Action {
    Quit, Typed(char), Resized { width: u16, height: u16 }, Tick, Start,   // unchanged
    /// Move the highlight up (area selection, level picker).
    Previous,
    /// Move the highlight down (area selection, level picker).
    Next,
    /// Choose the highlighted area.
    Choose,
    /// Go back to area selection.
    Back,
    /// Tick or untick the highlighted level.                    (1.2)
    Toggle,
    /// Start practice with the ticked levels.                   (2.2)
    Confirm,
    /// Go on: back to the level picker, or to the next question. (2.2; 4.1)
    Continue,
    /// Edit the answer.                                         (3.2)
    Edit(Edit),
    /// Submit the answer.                                       (4.1)
    Submit,
}
```

`Previous`, `Next`, `Choose` and `Back` are #20's; this change rewords their doc comments and extends their arms to the new screens. Arms after the last task (each checks the screen first, so an action on the wrong screen does nothing):

| Action | Screen | Effect |
|---|---|---|
| `Previous` / `Next` | `AreaSelection` | #20's behaviour, unchanged |
| `Previous` / `Next` | `LevelPicker` | `level_highlighted` −1 / +1, stopping at 0 and 4 |
| `Choose` | `AreaSelection`, an area is highlighted | `has_questions()` → `LevelPicker`, `level_highlighted = 0`; otherwise `NoQuestions` (#20) |
| `Back` | `NoQuestions`, `LevelPicker`, `NoQuestionsAtLevels`, `Question`, `Feedback` | `AreaSelection`; `drill = None` |
| `Toggle` | `LevelPicker` | `levels = levels.toggled(Level::ALL[level_highlighted])` |
| `Confirm` | `LevelPicker`, `levels` not empty, an area is highlighted | `Drill::start(area.questions(levels), levels, &mut rng)`: `Some` → store it, `Question`; `None` → `NoQuestionsAtLevels` |
| `Continue` | `NoQuestionsAtLevels` | `LevelPicker` (ticks and highlight kept) |
| `Continue` | `Feedback` | `drill.next(&mut rng)`, `Question` |
| `Edit(e)` | `Question` | `drill.edit(e)` |
| `Submit` | `Question` | `drill.submit()` true → `Feedback` |

New getters: `pub fn levels(&self) -> Levels`, `pub fn highlighted_level(&self) -> Level` (`Level::ALL.get(self.level_highlighted).copied().unwrap_or(Level::N5)`), `pub fn drill(&self) -> Option<&Drill>`; builder `pub fn with_seed(self, seed: u64) -> Session`.

- The area practised is `highlighted_area()`: the highlight cannot move while practising, so no extra field is needed, and Esc returns to the list with that area still highlighted (#20's rule for Back).
- In task 2.2 `Confirm` with a non-empty selection does nothing yet (a `// Practice starts here (task 3.1).` comment); no test pins that, only that it does not show `NoQuestionsAtLevels`.

### D11. Key table

Rules before the per-screen arms (resize, ignore releases, `Ctrl+C` quits) and the `Welcome`, `AreaSelection` and `NoQuestions` arms do not change. "Pressed" below means `key.modifiers.is_empty() && key.kind == KeyEventKind::Press`.

| Screen | Key | Action |
|---|---|---|
| `LevelPicker` | Up / Down, no modifiers (press or repeat) | `Previous` / `Next` |
| | Space (`KeyCode::Char(' ')`), pressed | `Toggle` |
| | Enter, pressed | `Confirm` |
| | Esc, pressed | `Back` |
| | anything else | `None` |
| `NoQuestionsAtLevels` | Enter, pressed / Esc, pressed | `Continue` / `Back` |
| | anything else | `None` |
| `Question` | Enter, pressed | `Submit` |
| | Esc, pressed | `Back` |
| | anything else | `answer_field::edit_for(event).map(Action::Edit)` |
| `Feedback` | Enter, pressed / Esc, pressed | `Continue` / `Back` |
| | anything else | `None` |

- Arrows repeat, as on area selection. Space, Enter and Esc act on presses only, so a held key cannot tick and untick in a flicker or run through several screens, where the terminal reports repeats (#20's rule; see Risks).
- On the question screen `q`, Space, letters and kana type, because `edit_for` claims them; only Enter, Esc and `Ctrl+C` are commands there. Enter with a modifier, or repeated, falls through to `edit_for`, which returns `None` for Enter.
- **Space ticks** (Open Question 3): it is the usual key for check boxes in terminal UIs, and Enter is taken by "start".

### D12. Drawing

All new screens are drawn in `ui.rs`, use no colour or style, and take all wording from `text.rs`. The area name on each screen is `session.highlighted_area().map(|area| area.name()).unwrap_or("")`.

**Level picker** (`draw_level_picker`), through #20's `draw_centred`:

```text
Sample
(empty)
Which JLPT levels do you want to practise?
(empty)
> [x] N5          three spans: AREA_MARKER or AREA_NO_MARKER, LEVEL_TICKED or LEVEL_UNTICKED, text::level_name(level)
  [ ] N4
  [ ] N3
  [ ] N2
  [ ] N1
(empty)
Up/Down to move, Space to tick, Enter to start, Esc to go back
Tick at least one level        (task 1.2; an empty line while a level is ticked)
```

All level lines are 8 columns wide, so no padding is needed for them to line up: in 80 columns the marker is at 36 and `[` at 38; in 100 columns at 46 and 48.

**No questions at these levels** (`draw_no_questions_at_levels`), through `draw_centred`: area name, empty, `No questions at these levels yet`, empty, `Press Enter to choose other levels`. In 80 columns the message starts at column 24.

**Question** (`draw_question`) and **Feedback** (`draw_feedback`) need a wrapped prompt and an answer field, so they use a vertical `Layout` of fixed rows with `Flex::Center` instead of `draw_centred`. Each text row is a `Paragraph::new(line).alignment(Alignment::Center)`; the prompt row is `Paragraph::new(prompt).alignment(Alignment::Center).wrap(Wrap { trim: true })` over `PROMPT_ROWS = 3` rows. A private helper `fn centred(frame: &mut Frame, area: Rect, line: Line)` draws one centred row.

Question rows (13): name 1, empty 1, prompt 3, empty 1, field 1, how-to-answer 1, empty 1, typo warning 1, empty 1, score 1, hint 1. Rows not yet used by an earlier task stay empty.

- Field row: `Layout::horizontal([Constraint::Length(ANSWER_LABEL_WIDTH), Constraint::Length(FIELD_WIDTH)]).flex(Flex::Center)` with `FIELD_WIDTH = 30` and `ANSWER_LABEL_WIDTH = 8` (a test pins that it equals the width of `text::ANSWER_LABEL`). The label is a plain `Paragraph::new(text::ANSWER_LABEL)`; the field is drawn with `if let Some(p) = drill.field().render(field_area, frame.buffer_mut()) { frame.set_cursor_position(p); }`. In 80 columns: label at 21, field at 29.
- How-to-answer row: `text::ANSWER_IN_JAPANESE` for `Kind::Japanese`, `text::ANSWER_IN_ENGLISH` for `Kind::Meaning`.
- Typo row: `text::TYPO_WARNING` when `drill.typo_warning()`, else empty (4.1).
- Score row: `text::score(drill.correct(), drill.answered())` (4.2).
- Hint row: `text::QUESTION_HINT`.

Feedback rows (14): name 1, empty 1, prompt 3, empty 1, `text::your_answer(&feedback.answer)` 1, `text::read_as(r)` or empty 1, empty 1, `text::CORRECT` or `text::WRONG` 1, `text::expected(&question.accepted)` when wrong or empty 1, empty 1, score 1, `text::FEEDBACK_HINT` 1.

- The cursor is set only on the question screen; ratatui hides it on every other frame.
- At small sizes `Layout` shrinks rows to height 0 and `render` returns `None` for an empty area, so nothing panics (Context fact 6; spec scenarios at 1x1).
- If the drill is missing on these screens (cannot happen through `apply`), the functions draw nothing.

### D13. Unused code

- **Reachable from real code through the contract.** Every new function is called on a path from `main`: `Session::apply` → `Choose` → `Area::has_questions` (default) → `Area::questions` and `practice::selection`; `Confirm` → `Drill::start` → `Pool`, `Rng`; and so on. Dead-code analysis is static, so it does not matter that the real areas return no entries and the path is only *taken* in tests. No lint attribute is added anywhere.
- **The contract types** are only constructed by the test-only Sample area, but appear in the live signature of `Area::questions`, so they get no warning (Context fact 1). When #11 to #13 construct them for real, nothing has to be removed.
- **Methods and fields are added only with their first non-test use** (Context fact 2), which is why `Levels`, `Drill` and the `Action`s grow task by task (D2, D8, D10). In the test build every field must also be read somewhere; for the contract types the tests of `src/areas/sample.rs` read every field.
- **#18's four attributes** (`#[cfg_attr(not(test), expect(dead_code, ...))]` and their two comment lines on `mod answer_field;`, `mod grading;`, `mod kana;`, `mod romaji;`) are removed in the task in which clippy reports `unfulfilled_lint_expectations` for them. Expected: all four in task 4.1, when `Drill::submit` calls `TypoGuard::submit` (which uses `check`, `kana` and `romaji`) and `AnswerField::text`. If #18's `render` happens to call `text()` itself, `answer_field`'s attribute already becomes unfulfilled in task 3.2 and is removed there. The rule for the implementer is the clippy message, not a guess.

### D14. Existing tests

Baseline: 51 tests on `main` today, plus the tests #18's tasks add (all inside `kana.rs`, `romaji.rs`, `grading.rs`, `answer_field.rs`) and the tests #20's tasks add (in `areas.rs`, the three area files, `session.rs`, `keys.rs`, `ui.rs`, `text.rs`). The exact count after #18 and #20 is not known until they are implemented.

| Tests | Effect |
|---|---|
| `app::tests::*` (4) | Unaffected: `Ticker` does not change; `seed_from` is new. |
| `keys::tests` on Welcome (`enter_starts_on_welcome`, `characters_are_typed`, `other_keys_and_events_do_nothing`, ...) | Unaffected: the Welcome arm does not change. |
| `keys::tests` on area selection and "no questions" from `main` and #20 (`other_keys_do_nothing_on_area_selection` as edited by #20, `enter_chooses_on_area_selection`, `arrows_move_on_area_selection`, `unbound_keys_do_nothing_on_area_selection`, `other_keys_do_nothing_on_no_questions`, `esc_through_the_key_table`, `enter_through_the_key_table_with_an_empty_area`, `enter_then_ctrl_c_through_the_key_table`, ...) | Unaffected: those two arms do not change. The key-table tests use `FakeArea` without questions or the real (empty) areas, so `Choose` still leads to `NoQuestions`. |
| `session::tests` from `main` (9) | Unaffected: `new` keeps its signature and starts on Welcome; `Start`, `Typed`, `Tick`, `Resized`, `Quit` arms do not change. |
| `session::tests` from #20: `choosing_an_area_with_questions_does_not_say_it_is_empty` | Still passes: `FakeArea { has_questions: true }` now leads to `LevelPicker`, which is neither `NoQuestions` nor quitting. This is exactly the case #20's D11 kept open for this change. |
| `session::tests` from #20: `choose_and_back_act_only_on_their_own_screens`, `moving_only_works_on_area_selection`, `back_keeps_the_moved_highlight`, `choosing_with_no_areas_does_nothing`, the others | Still pass: `Back` on Welcome or area selection still does nothing; `Next` on `NoQuestions` still does nothing; the picker has its own highlight field. |
| `areas::tests` and the three area files' `is_named_*` tests (#20) | Still pass: names and the list do not change. Removing the real areas' `has_questions` override changes no result (still `false`), and no #20 test calls it on a real area (#20 D11). |
| `areas::fake::FakeArea` (#20 helper, not a test) | Not edited; it compiles because `questions` has a default (D2). |
| `ui::tests` from `main` and #20 | Unaffected: `draw_welcome`, `draw_area_selection`, `draw_no_questions` and `draw_centred` do not change. |
| `text::tests` from `main` and #20 | Unaffected: no existing constant changes. |
| #18's tests | Unaffected: their modules are not edited; only the attributes on their `mod` lines in `main.rs` go. |

No existing test needs to change, so no task waits for the owner's approval of a test edit. If #18 or #20 change while being implemented, re-check this table before starting (Open Question 1).

### D15. English (meaning) answers

```rust
// src/practice/meaning.rs
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Outcome { Empty, Correct, Wrong }

/// Checks an English answer against the accepted meanings.
pub fn check(typed: &str, accepted: &[&str]) -> Outcome

fn normalise(text: &str) -> String
```

`normalise`: lower-case (`str::to_lowercase`), then `split_whitespace().collect::<Vec<_>>().join(" ")` (removes spaces at both ends, one space inside), then (task 4.3) strip **one** leading `"to "`, `"a "`, `"an "` or `"the "` (`strip_prefix`, checked in that order, first match only). `check`: `normalise(typed)` empty → `Empty`; equals `normalise(a)` for any `a` in `accepted` → `Correct`; else `Wrong`.

- **Implemented here** (Open Question 5) rather than only defined: `Kind::Meaning` is part of the contract, Vocabulary and Kanji need it for their first template question ("What does 高い mean?", "What does 食 mean?"), and without a checker the kind could not be practised at all. It is 20 lines and its own capability, `meaning-checking`, so it can move later without touching the loop. No typo warning and no kana conversion, as `docs/architecture.md` says.
- Stripping is done on the normalised text, so "To  Eat" becomes "eat". A bare `to` stays `to`. `today` is not touched (`"to "` needs the space).
- The cross-entry rule ("any entry of the area whose meaning matches is accepted") is about English **prompts** with Japanese answers and is the area's job when it fills `accepted` (#11); the practice module cannot see other entries' meanings.
- Task 4.1 lands the case and space rules (the loop needs a checker for `Meaning` to compile); task 4.3 adds the leading-word rule as a separate commit with its own tests. No 4.1 test asserts that `to eat` and `eat` differ.

### D16. Wording (`src/text.rs`)

All plain ASCII (no ambiguous widths), each with a doc comment saying where it is shown:

```rust
pub const LEVEL_PICKER_TITLE: &str = "Which JLPT levels do you want to practise?";   // 1.1
pub const LEVEL_PICKER_HINT: &str = "Up/Down to move, Space to tick, Enter to start, Esc to go back"; // 1.1
pub const LEVEL_TICKED: &str = "[x] ";                                               // 1.1
pub const LEVEL_UNTICKED: &str = "[ ] ";                                             // 1.1
pub fn level_name(level: Level) -> &'static str   // "N5", "N4", "N3", "N2", "N1"  // 1.1
pub const NO_LEVEL_TICKED: &str = "Tick at least one level";                         // 1.2
pub const NO_QUESTIONS_AT_LEVELS: &str = "No questions at these levels yet";          // 2.2
pub const NO_QUESTIONS_AT_LEVELS_HINT: &str = "Press Enter to choose other levels";  // 2.2
pub const ANSWER_IN_JAPANESE: &str = "Answer in Japanese or romaji";                 // 3.1
pub const ANSWER_IN_ENGLISH: &str = "Answer in English";                             // 3.1
pub const QUESTION_HINT: &str = "Enter to answer, Esc to stop";                      // 3.1
pub const ANSWER_LABEL: &str = "Answer: ";                                           // 3.2
pub const TYPO_WARNING: &str = "This does not read as Japanese. Press Enter again to submit it anyway."; // 4.1
pub const CORRECT: &str = "Correct!";                                                // 4.1
pub const WRONG: &str = "Not quite.";                                                // 4.1
pub const FEEDBACK_HINT: &str = "Press Enter for the next question, Esc to stop";    // 4.1
pub fn your_answer(answer: &str) -> String      // "Your answer: {answer}"           // 4.1
pub fn read_as(reading: &str) -> String         // "Read as: {reading}"              // 4.1
pub fn expected(accepted: &[String]) -> String  // 1 → "Correct answer: X"; else "Correct answers: X, Y" // 4.1
pub fn score(correct: u32, answered: u32) -> String  // "Score: {correct} of {answered}" // 4.2
```

Widths (columns): title 42, picker hint 62, no-level line 23, no-questions message 32, typo warning 70, feedback hint 46. All fit 80 columns. Level names are not localised, but still go through `text.rs` like every shown text.

### D17. Task order and commits

Each task is one commit that builds and passes fmt, clippy and tests (`AGENTS.md` §5):

1. **Level picker:** 1.1 open it, show it, move, Esc back; 1.2 Space ticks, "Tick at least one level".
2. **Contract:** 2.1 the contract types, `Area::questions`, the default `has_questions`, `selection`, the Sample area; 2.2 Enter on the picker and "No questions at these levels yet".
3. **Question screen:** 3.1 `Rng`, `Pool`, `Drill::start`, the clock seed and the question screen without a field; 3.2 the answer field and typing.
4. **Answering:** 4.1 submit, Japanese and basic meaning checking, typo warning, feedback, next question, the end-to-end test, removal of #18's attributes; 4.2 counting and the score; 4.3 the leading-word rule for meanings.
5. **Integration check:** 5.1 scripted check of the release binary.

`CHANGELOG.md` gets one entry in 1.1, as the last item of *Unreleased* / *Added*, and each later feature task appends a sentence to it. `docs/architecture.md` is edited in the task whose behaviour it describes (1.1, 1.2, 2.1, 3.1, 4.1, 4.2), not in a final docs task; group 5 is only the integration check.

**Size:** 10 tasks, the largest change so far. It could be split in two OpenSpec changes at the boundary after group 2 (picker and contract; the app's behaviour is unchanged there, since no real area has questions) and groups 3 to 5 (the loop). Recommended: keep one change and one PR, because the five specs are written together and group 2 alone would leave the picker's Enter half-done; the task groups already give reviewable commits. Open Question 8.

## Risks / Trade-offs

- [#18 or #20 change while being implemented, and names used here (`TypoGuard::submit`, `Outcome`, `edit_for`, `with_areas`, `highlighted_area`, `FakeArea`, `Back`) no longer match] → Prerequisite stated at the top of `proposal.md` and `tasks.md`; the first task starts with a check that each named item exists with the expected signature, and stops if not (Open Question 1).
- [The practice loop cannot be reached in the released app until #11 to #13 add content] → Intended by the issue; everything is tested with the Sample area, and the scripted binary check (5.1) confirms that the real, empty areas still show "No questions in this area yet". The real IME check with the answer field on screen therefore moves to the first content issue (Open Question 6).
- [Most Unix terminals report a held key as repeated presses, so a held Enter can go question → feedback → next question] → Same limitation as #20; the press-only guard helps where repeats are reported (Windows, kitty protocol). Nothing is lost: the answer was counted once.
- [A paste containing a newline arrives as Enter and submits the part before it] → Bracketed paste stays off (Open Question 7). Rare for one-word answers.
- [A prompt longer than three rows is clipped] → Prompts are a sentence at most; content tests in #11 to #13 can bound them.
- [Seeds from the clock are predictable] → Irrelevant for a learning app; the order only needs to vary.
- [The score line is shown before the results screen exists] → It reads the counts the issue requires, keeps them live code, and is useful; the results change may move or remove it (no test pins its row).
- [Area content could put romaji in `accepted`, or accept only the kanji form of a word its prompt shows, which can then never be answered correctly (accept the kana form too, D3)] → The contract documents both; content changes test their own data.

## Open Questions

For the owner:

1. **Dependencies on #18 and #20 (most load-bearing first).** This design depends on: #20's `Area` trait being extendable with defaulted methods and `FakeArea` not implementing more than `name` and `has_questions`; `Session::with_areas(width, height, &'static [&'static dyn Area])` and `highlighted_area()`; the `Choose` branch for areas with questions; `Back` being free to reuse as "back to area selection"; `draw_centred` and `AREA_MARKER`/`AREA_NO_MARKER`. From #18: `TypoGuard::submit(&mut self, &str, &[&str], &[&str]) -> Outcome` with the four `Outcome` variants; `Edit` being `Copy`; `edit_for(&Event)`; `AnswerField::{default, text, apply, render}` with `render` returning the cursor position; and the self-removing `expect(dead_code)` attributes (if you choose a different approach for #18's Open Question 1, task 4.1's removal step changes accordingly). If any of these change, this change's tasks must be revised before implementation.
2. **Level picker's starting selection.** Default: **only N5** when the app starts, then **remembered** for the whole run (across areas and after Esc). Alternative: all five; or reset to N5 every time the picker opens.
3. **Key that ticks a level.** Default: **Space** (press only). Alternatives: also `x`, or Enter ticks and another key starts.
4. **Enter with no level ticked.** Default: **does nothing**, and the picker shows `Tick at least one level` whenever nothing is ticked. Alternative: forbid unticking the last level, so the case cannot happen.
5. **English (meaning) answers.** Default: **implemented here** as its own capability (D15). Alternative: only define `Kind::Meaning` here and implement checking in a later issue; then Vocabulary's "What does 高い mean?" and Kanji's "What does 食 mean?" (the first template of each) could not be practised, and `Drill::submit` would need a placeholder for that kind.
6. **The real IME check with the field on screen** (#18's Open Question 4). The release binary cannot reach the question screen until an area has content, so the owner cannot run it in this change. Default: do it in the first content issue to land (#11, #12 or #13), with `たかい`, `タカイ`, `高い`, `takai` and Backspace, on Linux, macOS, Windows and over SSH. Alternative: the owner temporarily adds the Sample area to `areas::all()` in a local, uncommitted build.
7. **Bracketed paste** (#18's Open Question 5). Default: **not in this change**; pasted text keeps arriving as typed characters, and a newline in it submits. Enabling it needs `EnableBracketedPaste` in `app.rs` and a way to deliver an `Event::Paste(String)` to the field, which does not fit an `Action` that is `Copy`. Suggested as its own issue.
8. **Splitting the change** (D17). Default: one change, one PR. Alternative: two changes, `add-level-picker` (groups 1 and 2) and `add-practice-loop` (groups 3 to 5).
9. **Question count and results screen.** Default: **not in this change**; practice goes on until Esc and the score line shows the counts.
10. **Esc on "No questions at these levels yet".** Default: back to **area selection**, as Esc does on every screen after it (`docs/architecture.md` §2 *Keys*); Enter goes back to the picker. Alternative: Esc also goes back to the picker.
11. **No entry twice in a row across rounds** (D7). Default: a new round never starts with the entry just asked. This goes slightly beyond the architecture's rule; say if you prefer the plain rule.
