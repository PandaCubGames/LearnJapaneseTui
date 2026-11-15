# Proposal

> **Prerequisite:** this change is implemented **after** two other changes are merged to `main`:
> `add-japanese-answer-input` (issue #18) and `add-area-selection-screen` (issue #20).
> It uses from #20: the trait `areas::Area` (`name`, `has_questions`), `areas::all()`, the real
> area files `src/areas/{vocabulary,kanji,grammar}.rs`, the test-only `areas::fake::FakeArea`,
> `Session::with_areas`, `Session::highlighted_area`, `Screen::{AreaSelection, NoQuestions}`,
> `Action::{Previous, Next, Choose, Back}`, `ui::draw_centred` and `text::{AREA_MARKER, AREA_NO_MARKER}`.
> It uses from #18: `grading::TypoGuard::submit`, `grading::Outcome`, `answer_field::{AnswerField, Edit, edit_for}`,
> and it removes the four `expect(dead_code)` attributes #18 puts on `mod` lines in `src/main.rs`.

## Why

Vocabulary (#11), Kanji (#12) and Grammar (#13) all practise the same way: choose JLPT levels, get a question, type an answer, see whether it was right, go on. Without one shared module each area would rebuild that loop. This change builds it once, so an area only has to supply questions. Tracked in GitHub issue #21.

Today an area that has questions can be chosen on area selection, but nothing happens (#20 leaves a marked branch for this issue).

## What Changes

- **The area contract.** `areas::Area` gains `questions(levels)`: the area's entries that have any of the given JLPT levels, each entry with its questions. A question carries a prompt, its accepted answers, the Japanese text it already shows, and its kind (`Japanese` or `Meaning`). Generated and hand-written questions are carried the same way. `has_questions` gets a default body built on `questions`, so an area only implements `name` and `questions`. The practice module never sees an area's own model.
- **The level picker** (new screen): after choosing an area that has questions. N5, N4, N3, N2, N1, each with a tick box; Up/Down move, Space ticks or unticks, Enter starts, Esc goes back to area selection. Only N5 is ticked when the app starts; the ticks are kept while the app runs. With nothing ticked it says `Tick at least one level` and Enter does nothing.
- **No questions at these levels** (new screen): when no entry has a chosen level, it says `No questions at these levels yet`; Enter goes back to the picker, Esc to area selection.
- **Picking questions:** an entry at random, then one of its questions at random; no entry repeats until every entry has been seen, and a new round never starts with the entry just asked. The random numbers come from a small built-in generator (SplitMix64, no new dependency), seeded from the clock in `app`, and fixed by a seed in tests.
- **Question screen** (new): the area name, the prompt, an answer field (#18) with the cursor in it, a line saying how to answer (`Answer in Japanese or romaji` or `Answer in English`), the score and the keys. Enter submits; an empty answer is ignored; a Japanese answer that cannot be read gets #18's typo warning and is not counted.
- **Feedback screen** (new): the answer, its reading when it differs, `Correct!` or `Not quite.`, and the accepted answers when wrong. Enter goes to the next question.
- **Counting:** each submitted answer is counted once; continuing from feedback, typo warnings, empty submits and leaving with Esc are not counted. The score (`Score: 2 of 3`) is shown on the question and feedback screens until the results screen exists.
- **Esc** on the question or the feedback screen goes back to area selection; an unfinished answer is dropped.
- **English (meaning) answers** are checked here: case, extra spaces and a leading `to`, `a`, `an` or `the` are ignored.
- A test-only area (`areas::sample::Sample`) with fixed questions at N5, N4 and N3 proves the whole loop end to end. It is listed only in tests; the practice module did not change to accept it.
- Until the results screen exists, practice goes on until Esc.

Not in scope: real areas or content (#11, #12, #13); the question count and the results screen; multiple choice; bracketed paste; kana-to-kanji conversion.

## Capabilities

### New Capabilities
- `area-contract`: what an area supplies to practice (entries of questions for chosen levels, question fields and kinds), how an area is reported as empty, and that adding an area only means listing it.
- `level-picker`: the JLPT level picker screen and the "no questions at these levels" screen: content, ticks, keys, layout.
- `question-picking`: which entries are practised and in which order: the any-level rule, random entry then random question, no repeat until all are seen, reproducible order from a seed.
- `practice-loop`: the question and feedback screens: answering, checking, the typo warning, counting, the score, continuing and leaving.
- `meaning-checking`: how an English (meaning) answer is compared with the accepted meanings.

### Modified Capabilities

None. `area-selection` (#20) already says that what Enter shows for an area with questions "is decided by the practice screens, which are not part of this capability"; this change adds those screens as new capabilities. The capabilities of #18 (`answer-checking`, `answer-input`, `romaji-conversion`) are used as they are. So this change archives independently of #18 and #20, as long as both are archived (they are merged first).

## Impact

- **Code:** new `src/practice.rs`, `src/practice/level.rs`, `src/practice/question.rs`, `src/practice/rng.rs`, `src/practice/pool.rs`, `src/practice/meaning.rs`, `src/areas/sample.rs` (test only). Changed: `src/main.rs` (one `mod` line, #18's four attributes removed), `src/areas.rs` (trait methods, one `#[cfg(test)]` line), the three real area files (their `has_questions` override removed), `src/session.rs`, `src/keys.rs`, `src/ui.rs`, `src/text.rs`, `src/app.rs` (the clock seed).
- **Tests:** new tests in every file above. No existing test (on `main` today, or added by #18 or #20) changes; `design.md` D14 checks them one by one.
- **Docs:** `CHANGELOG.md` (*Unreleased* / *Added*), `docs/architecture.md` (status note, Levels, Questions, Answering, Area contract, Still open).
- **Not affected:** dependencies (`Cargo.toml`, `Cargo.lock`), the project version, CI, learning content, `ATTRIBUTION.md`.
