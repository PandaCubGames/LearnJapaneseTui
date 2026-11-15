# Proposal

> **Builds on merged work only:** `add-area-selection-screen` (#20), `add-japanese-answer-input` (#18) and `add-generic-practice` (#21) are all on `main`. This change uses, unchanged: the `Area` trait (`name`, `questions(Levels) -> Vec<Entry>`, the defaulted `has_questions`) and the list `ALL` in `src/areas.rs`; `practice::{Entry, Level, Levels}` and `practice::question::{Question, Kind}`; `grading::check`; `text.rs` as the home of all wording.

## Why

The app has the whole practice loop (area list, level picker, question and feedback screens, romaji and IME answers), but no area is registered, so the released app stops at "Nothing to practise yet". GitHub issue #13 asks for the first real area: **Grammar**, whose first version is conjugation practice. It is also the first content that reaches the question screen outside tests, so the deferred real-terminal IME check can finally be done.

## What Changes

- **A `Grammar` area** in the area list (its own file `src/areas/grammar.rs`, one `mod`/`use` line and one entry in `ALL`). It is the first registered area, so the app now goes Welcome → area list → level picker → questions.
- **Conjugation practice:** each word (a verb or an adjective) is one entry with nine questions: plain non-past, negative, past, past negative and te-form, and polite non-past, negative, past and past negative. Questions are picked at random by the existing practice module, so words, forms and formalities arrive mixed. There is no step to choose a topic, form or formality.
- **Conjugation rules** in a new pure module, `src/conjugation.rs` (no state, no terminal): correct answers are generated from the word's reading and written form, never stored per word. Groups: ichidan; godan (all nine endings う く ぐ す つ ぬ ぶ む る); the irregular verbs する (and noun + する), 来る, 行く and ある; i-adjectives (and the irregular いい); na-adjectives. Every group, form and formality is tested.
- **Accepted answers:** each form is accepted with kanji and in kana (and so in romaji). Adjective forms with more than one standard polite or negative variant accept all of them (for example 高くないです and 高くありません; 静かじゃありません and 静かではありません and their ないです forms).
- **The prompt** names the word with its reading and English meaning, and the form: `What is the polite past negative of 食べる (たべる, to eat)?`. The word as written is the question's shown text, so copying 食べる back is not an answer, but たべる and `taberu` are (owner decision on the repeat rule).
- **A starting word list** of 49 common words written for this project (7 ichidan, 21 godan, 6 irregular, 8 i-adjectives, 7 na-adjectives) at N5 and N4, embedded in the binary as Rust constant data. JLPT levels are the project's own approximate judgment (the JLPT publishes no official list); no published list or textbook is copied.
- **Feedback with many accepted answers:** the "Correct answers" line on the feedback screen wraps onto up to four rows instead of being cut off, because a na-adjective form can have eight accepted answers.
- **Docs:** `docs/architecture.md` (status, Grammar, Conjugation, Still open), `CHANGELOG.md`, `ATTRIBUTION.md` (a note on content written for this project and on the levels), `README.md` (status).

Not in scope (issue #13): particle questions; grammar patterns without a word of their own (てもいい); further forms (potential, volitional, conditional, imperative, passive, causative); choosing a topic, form, formality or difficulty inside Grammar; ids and a content file format (both stay open in `docs/architecture.md`); custom (hand-written) questions on a Grammar entry.

## Capabilities

### New Capabilities
- `conjugation`: the rules that turn a word (reading, written form, group) into a form and formality: the groups, the nine endings of godan verbs, the irregular verbs, both adjective types, and the accepted variants.
- `grammar-area`: the Grammar area: its place in the area list, its word list and levels, one entry per word with nine questions, the prompt, the shown text and the accepted answers.
- `accepted-answers-display`: how the accepted answers are laid out on the feedback screen when they do not fit on one line.

### Modified Capabilities

None. The capabilities of the earlier changes (`area-selection`, `area-contract`, `level-picker`, `question-picking`, `practice-loop`, `answer-checking`, `romaji-conversion`, `answer-input`) are used as they are. `accepted-answers-display` adds a layout rule next to `practice-loop`'s *Feedback screen* requirement without changing what it says. Because all three capabilities are new, this change archives on its own; archiving it after `add-generic-practice` is recommended so the specs read in the order they were built (design.md D16).

## Impact

- **New code:** `src/conjugation.rs`, `src/areas/grammar.rs`.
- **Changed code (the minimum, design.md D11):** `src/main.rs` (`mod conjugation;`), `src/areas.rs` (`mod grammar;`, `use grammar::Grammar;`, `&Grammar` in `ALL`, its doc comment), `src/text.rs` (the area's name and the prompt wording, as the file's rule requires), `src/practice/question.rs` (the unused-code expectation on `Kind` moves from the enum to the still-unused `Meaning` variant, design.md D12), `src/ui.rs` (the accepted answers wrap, design.md D10).
- **Tests:** new tests in each of these files. No existing test changes: all 379 tests on `main` were run with a Grammar-like area registered and pass (design.md D13).
- **Docs:** `docs/architecture.md`, `CHANGELOG.md`, `ATTRIBUTION.md`, `README.md`.
- **Not affected:** dependencies (`Cargo.toml`, `Cargo.lock`), the project version, CI, `session`, `keys`, `practice` (except the one attribute), `grading`, `romaji`, `kana`, `answer_field`.
