# Tasks

> **Starting point:** a branch from an up-to-date `main` that has `add-area-selection-screen` (#20), `add-japanese-answer-input` (#18) and `add-generic-practice` (#21) merged. `cargo test` on it passes **379** tests.

Types, signatures, rule tables, the word list and the expected forms are in `design.md` (D-numbers and Appendices A and B). Rules for every task:

- **One task, one commit** (`AGENTS.md` §5). After each task, `cargo fmt --all -- --check`, `cargo clippy --all-targets -- -D warnings` and `cargo test` pass. Tick the task's checkbox in this file in the same commit.
- Do not change the project version and do not add a dependency: `git diff main -- Cargo.toml Cargo.lock VERSION` stays empty.
- Every new source file starts with the SPDX header, then a `//!` module comment:
  ```rust
  // SPDX-License-Identifier: GPL-3.0-or-later
  // Copyright (C) 2026 PandaCubGames
  ```
- **Existing tests:** do not change, rename, move, skip, `#[ignore]` or delete any test that is on `main`, and do not edit the test helpers `areas::fake` and `areas::sample`. Adding new tests, test helpers, test-only types and `use` lines inside existing `mod tests` blocks is allowed. No existing test is expected to fail (design.md D13); if one does, the code is wrong: fix the code, or stop and report.
- **Tests added earlier in this branch** are not edited by later tasks either: each task adds its own new tests.
- **Unused code** (design.md D15): a `Group` variant, function or field arrives only in the task that uses it outside tests. Never add `#[allow(...)]` or a new `#[expect(...)]`. If clippy reports dead code or an unfulfilled expectation that this file does not mention, stop and report.
- **Japanese text:** copy every expected form from design.md Appendix B, never type it from memory.
- **Wording** comes from `src/text.rs` only.
- Test helpers used below, to add in the first task that needs them, inside the file's `mod tests`:
  - `src/conjugation.rs`: `const NINE: [(Form, Formality); 9]` (the order of design.md D6 `ASKED`); `fn a(written: &str, reading: &str) -> Answer`; `fn nine(written: &str, reading: &str, group: Group) -> Vec<Vec<Answer>>` (`conjugate` for each of `NINE`, in order).
  - `src/areas/grammar.rs`: `fn ask(written: &str, form: Form, formality: Formality) -> Question` (finds the word in `WORDS` by `written`, panics if missing, returns `question(word, form, formality).unwrap()`); `fn strings(texts: &[&str]) -> Vec<String>`; `fn is_correct(question: &Question, typed: &str) -> bool` (`matches!(grading::check(typed, &accepted, &shown), Outcome::Correct { .. })` with `&str` views of `accepted` and `shown`); from task 2.4, `fn outcome(question: &Question, typed: &str) -> Outcome` (`grading::check(typed, &accepted, &shown)` with the same views).

## 1. Grammar area with the verbs

- [x] 1.1 Register the Grammar area with the ichidan rule and the ichidan verbs.
  - **First, check:** `src/areas.rs` has `trait Area` with `name`, `questions(&self, levels: Levels) -> Vec<Entry>` and a default `has_questions`, and `const ALL: &[&dyn Area] = &[];`; `practice::{Entry, Level, Levels}` and `practice::question::{Kind, Question}` exist as in design.md Context; `grading::check(&str, &[&str], &[&str]) -> Outcome` is `pub`. If anything differs, stop and report.
  - `src/conjugation.rs` (new): module doc as in D2. `Group` with the doc comment of D2 and **only** the variant `Ichidan` (with its doc). `Form`, `Formality`, `Answer` and `pub fn conjugate` exactly as D2. Private `fn inflect(text: &str, group: Group, form: Form, formality: Formality) -> Vec<String>` and `fn pick<T>(form: Form, formality: Formality, plain: [T; 5], polite: [T; 4]) -> Option<T>` as D2. Private `fn endings(group: Group, form: Form, formality: Formality) -> Option<(&'static str, Vec<String>)>` (no `text` parameter yet; task 1.3 adds it) with the Ichidan rows of D3: cut `る`; plain `る`, `ない`, `た`, `なかった`, `て`; polite `ます`, `ません`, `ました`, `ませんでした`; `(Te, Polite)` → `None` through `pick`.
  - `src/main.rs`: add `mod conjugation;` right after `mod areas;`. Nothing else.
  - `src/text.rs`: add `use crate::conjugation::{Form, Formality};`, and after `NO_QUESTIONS_HINT` the constant `AREA_GRAMMAR` and the functions `form_name` and `conjugation_prompt`, exactly as design.md D7, each with its doc comment.
  - `src/areas/grammar.rs` (new): everything in design.md D6 (`ASKED`, `Word`, `const fn word`, `WORDS`, `Grammar`, `entry`, `question`, `push_unique`) with `WORDS` holding words 1 to 7 of Appendix A, in that order.
  - `src/areas.rs`: add `mod grammar;` above `#[cfg(test)] pub mod sample;`, `use grammar::Grammar;` below the existing `use` line, `const ALL: &[&dyn Area] = &[&Grammar];`, and change its doc comment to `/// Every area, in the order the list shows them. Adding an area adds its own file under \`src/areas/\` and one entry here.` (without the backslashes).
  - New tests, `src/conjugation.rs`:
    - `ichidan_nine_forms_of_taberu`: `nine("食べる", "たべる", Group::Ichidan)` equals Appendix B's nine pairs, one `Answer` each.
    - `ichidan_nine_forms_of_miru`: same for `見る` / `みる`.
    - `ichidan_has_no_polite_te_form`: `conjugate("食べる", "たべる", Group::Ichidan, Form::Te, Formality::Polite)` is empty.
    - `a_word_that_does_not_fit_its_group_gives_nothing`: `conjugate("静か", "しずか", Group::Ichidan, Form::Negative, Formality::Plain)` is empty; so is `conjugate("", "", Group::Ichidan, Form::NonPast, Formality::Plain)`.
    - `written_form_and_reading_must_both_fit`: `conjugate("食べる", "たべ", Group::Ichidan, Form::Negative, Formality::Plain)` is empty.
  - New tests, `src/text.rs`:
    - `grammar_area_name_is_plain_ascii_text`: `AREA_GRAMMAR == "Grammar"` and is ASCII.
    - `form_names`: the nine pairs of `ASKED`'s order give `"plain non-past"`, `"plain negative"`, `"plain past"`, `"plain past negative"`, `"te-form"`, `"polite non-past"`, `"polite negative"`, `"polite past"`, `"polite past negative"`; `form_name(Form::Te, Formality::Polite) == "te-form"`; every name is ASCII.
    - `conjugation_prompt_for_a_word_with_kanji`: `conjugation_prompt("食べる", "たべる", "to eat", Form::PastNegative, Formality::Polite) == "What is the polite past negative of 食べる (たべる, to eat)?"`.
    - `conjugation_prompt_for_a_word_in_kana`: `conjugation_prompt("する", "する", "to do", Form::Negative, Formality::Plain) == "What is the plain negative of する (to do)?"`.
    - `conjugation_prompt_for_the_te_form`: `conjugation_prompt("行く", "いく", "to go", Form::Te, Formality::Plain) == "What is the te-form of 行く (いく, to go)?"`.
  - New tests, `src/areas/grammar.rs`:
    - `grammar_is_named_and_listed`: `Grammar.name() == "Grammar"`; `areas::all()` has an area named `"Grammar"` (do not assert its position or the list's length).
    - `grammar_is_on_the_area_list_screen`: `Session::new(80, 24)`, `Action::Start`; draw it with `crate::ui::draw` into a `ratatui::Terminal` on `TestBackend::new(80, 24)`; some row of the buffer contains `Grammar`.
    - `the_app_reaches_a_grammar_question`: `Session::new(80, 24)`, `Start`, then `Next` as many times as Grammar's index in `areas::all()`, `Choose` → screen `LevelPicker` and `highlighted_area()` named `Grammar`; `Confirm` → screen `Question`, and the drill's question prompt starts with `"What is the "`.
    - `grammar_has_questions`: `Grammar.has_questions()`.
    - `every_word_gives_nine_questions`: `Grammar.questions(Levels::ALL)` has `WORDS.len()` entries; each has 9 questions with 9 different prompts, all `Kind::Japanese`, and `levels == Levels::only(word.level)` for its word (same order as `WORDS`).
    - `entries_follow_the_chosen_levels`: at `Levels::only(Level::N4)` the result is not empty and every entry's levels contain N4; same for N5; at `Levels::only(Level::N2).toggled(Level::N1)` it is empty.
    - `every_word_is_valid` (spec *Word list entries are valid*): for each word: `reading` is not empty and every char is in `'\u{3041}'..='\u{3096}'` (hiragana); `written` is not empty, and when `written != reading` it has a char with `kana::is_kanji`; `meaning` is not empty and ASCII; no other word has the same `written`; for `Group::Ichidan`, the reading ends in `る` and the char before it is one of `いきしちにひみりぎじぢびぴえけせてねへめれげぜでべぺ`.
    - `prompts_fit_80_columns`: every prompt of `Grammar.questions(Levels::ALL)` has `ratatui::text::Line::from(prompt.as_str()).width() <= 80`.
    - `accepted_answers_are_japanese_without_duplicates`: for every question: each accepted text is not empty and every char satisfies `kana::is_japanese`; no text appears twice; at least one text has no char with `kana::is_kanji`.
    - `polite_past_of_taberu`: `ask("食べる", Form::Past, Formality::Polite)` has prompt `"What is the polite past of 食べる (たべる, to eat)?"`, accepted `strings(&["食べました", "たべました"])`, shown `strings(&["食べる", "たべる"])`.
    - `polite_past_negative_prompt`: `ask("食べる", Form::PastNegative, Formality::Polite).prompt == "What is the polite past negative of 食べる (たべる, to eat)?"`.
    - `taberu_answers_typed_every_way`: for the polite past of 食べる, `is_correct` for `tabemashita`, `tabemasita`, `たべました`, `タベマシタ`, `食べました`; not for `tabemasu`.
    - `copying_the_shown_word_is_wrong`: for `ask("食べる", Form::NonPast, Formality::Plain)`: `食べる` is not correct; `たべる` and `taberu` are.
  - Verify: the new tests pass; the 379 existing tests pass unchanged; fmt and both clippy runs (`cargo clippy -- -D warnings` and `cargo clippy --all-targets -- -D warnings`) clean. Commit, e.g. `feat(grammar): add the Grammar area with ichidan verbs`.

- [x] 1.2 Narrow the unused-code expectation on `Kind` to `Kind::Meaning` (design.md D12).
  - `src/practice/question.rs`: delete the two comment lines (`// No real area builds a question yet ...` and `// does removes this attribute. ...`) and the `#[cfg_attr(not(test), expect(dead_code, ...))]` attribute above `pub enum Kind`; put the comment and attribute of D12 above `Meaning`, after its doc comment, exactly as written there. Change nothing else in the file.
  - No new test: this changes no behaviour.
  - Verify: `cargo clippy -- -D warnings` and `cargo clippy --all-targets -- -D warnings` clean, `cargo fmt --all -- --check` clean, `cargo test` passes. If clippy reports the moved expectation as unfulfilled, or reports `Japanese` as never constructed, stop and report. Commit, e.g. `refactor(practice): expect only the meaning kind to be unused`.

- [x] 1.3 Add godan verbs: all nine endings.
  - `src/conjugation.rs`: add `Group::Godan` (doc as D2); the constant `GODAN` of D3a (doc: `/// A godan verb's last kana, its negative stem, its polite stem, its te-form and past endings.`); `endings` gets a first parameter `text: &str` (update its one caller in `inflect`), and the Godan rows: the last char of `text` selects the `GODAN` row (none → `None`); cut that kana; plain *k*, *a*`ない`, *ta*, *a*`なかった`, *te*; polite *i*`ます`, *i*`ません`, *i*`ました`, *i*`ませんでした`.
  - `src/areas/grammar.rs`: append words 8 to 28 of Appendix A to `WORDS`.
  - New tests, `src/conjugation.rs`, one per ending, each asserting all nine forms (`nine(..)` against Appendix B; the reading of each form is the reading stem plus the same ending): `godan_u_kau` (also the full readings given in Appendix B), `godan_ku_kaku`, `godan_gu_oyogu`, `godan_su_hanasu`, `godan_tsu_matsu`, `godan_nu_shinu` (also the full readings given in Appendix B), `godan_bu_asobu`, `godan_mu_nomu`, `godan_ru_kaeru`. Plus `godan_has_no_polite_te_form` (書く) and `a_verb_without_a_godan_ending_gives_nothing` (`conjugate("高い", "たかい", Group::Godan, Form::Negative, Formality::Plain)` and the same for `静か` / `しずか` are empty).
  - New tests, `src/areas/grammar.rs`: `godan_words_cover_every_ending` (the set of last chars of the readings of the `Group::Godan` words contains each of `う く ぐ す つ ぬ ぶ む る`); `kaeru_is_godan` (`ask("帰る", Form::Negative, Formality::Plain).accepted == strings(&["帰らない", "かえらない"])`).
  - Verify as in 1.1. Commit, e.g. `feat(grammar): conjugate godan verbs`.

- [x] 1.4 Add する, noun + する and 来る.
  - `src/conjugation.rs`: add `Group::Suru` and `Group::Kuru` (docs as D2) and their rows of D3: Suru cuts `する`; Kuru cuts `来る` when `text` ends in `来る` (keeping 来 in every ending), else `くる` when it ends in `くる` (endings with こ/き), else `None`.
  - `src/areas/grammar.rs`: append words 29 to 32 of Appendix A.
  - New tests, `src/conjugation.rs`: `suru_nine_forms` (する); `noun_suru_keeps_the_noun` (勉強する / べんきょうする, all nine pairs); `kuru_keeps_its_kanji_and_changes_its_reading` (来る / くる, all nine pairs); `kuru_in_kana` (`nine("くる", "くる", Group::Kuru)`, all nine); `suru_and_kuru_have_no_polite_te_form`; `suru_and_kuru_need_their_ending` (`conjugate("食べる", "たべる", Group::Suru, Form::Negative, Formality::Plain)` and `conjugate("見る", "みる", Group::Kuru, Form::Negative, Formality::Plain)` are empty).
  - New tests, `src/areas/grammar.rs`:
    - `a_word_in_kana_shows_only_its_meaning`: `ask("する", Form::Negative, Formality::Plain)` has prompt `"What is the plain negative of する (to do)?"`, accepted `strings(&["しない"])`, shown `strings(&["する"])`; `ask("する", Form::NonPast, Formality::Polite).accepted == strings(&["します"])`.
    - `the_plain_non_past_of_suru_can_be_typed_back`: for `ask("する", Form::NonPast, Formality::Plain)`, `is_correct` for `する` and `suru`.
    - `kuru_answers_use_its_reading`: `ask("来る", Form::Negative, Formality::Plain).accepted == strings(&["来ない", "こない"])`; `is_correct` for `konai`, `こない` and `来ない`; not for `kinai`.
  - Verify as in 1.1. Commit, e.g. `feat(grammar): conjugate suru and kuru`.

- [ ] 1.5 Add 行く and ある.
  - `src/conjugation.rs`: add `Group::Iku` and `Group::Aru` (docs as D2) and their rows of D3: Iku needs a last kana `く` (else `None`), cuts it, and uses the godan く row except te `って` and past `った`; Aru cuts `ある`.
  - `src/areas/grammar.rs`: append words 33 and 34 of Appendix A.
  - New tests, `src/conjugation.rs`: `iku_nine_forms` (行く / いく, all nine pairs); `iku_differs_from_godan_only_in_te_form_and_past` (`nine("行く", "いく", Group::Iku)` and `nine("行く", "いく", Group::Godan)` are equal except at index 2, plain past, and index 4, te-form); `aru_nine_forms` (ある, all nine); `iku_and_aru_have_no_polite_te_form`; `iku_and_aru_need_their_ending` (`conjugate("飲む", "のむ", Group::Iku, Form::Te, Formality::Plain)` and `conjugate("いる", "いる", Group::Aru, Form::Negative, Formality::Plain)` are empty).
  - New tests, `src/areas/grammar.rs`: `te_form_of_iku` (`ask("行く", Form::Te, Formality::Plain)` has prompt `"What is the te-form of 行く (いく, to go)?"` and accepted `strings(&["行って", "いって"])`); `aru_negative_is_nai` (`ask("ある", Form::Negative, Formality::Plain).accepted == strings(&["ない"])`; `is_correct` for `nai`, not for `aranai`).
  - Verify as in 1.1. Commit, e.g. `feat(grammar): conjugate iku and aru`.

## 2. Adjectives

- [ ] 2.1 Wrap long lists of accepted answers on the feedback screen (design.md D10).
  - `src/ui.rs`: add `const EXPECTED_ROWS: u16 = 4;` with the doc comment of D10, next to `PROMPT_ROWS`. In `draw_feedback`, the constraint of `expected_row` (the ninth, between `verdict_row` and the empty row before `score_row`) becomes `Constraint::Length(EXPECTED_ROWS)`, and the accepted answers are drawn with `draw_prompt(frame, expected_row, &text::expected(&question.accepted))` instead of `centred(..)`. Change `draw_prompt`'s doc comment as in D10. Nothing else changes.
  - New tests, `src/ui.rs` (a test-only `struct ManyAnswers` implementing `Area` with name `"Many"` and one N5 entry with one `Kind::Japanese` question: prompt `"What is the polite past negative of 簡単 (かんたん, easy)?"`, accepted the eight answers of the spec scenario *Eight long answers in 80 columns*, in that order, shown empty; `const MANY: &[&dyn Area] = &[&ManyAnswers];`):
    - `many_accepted_answers_wrap_and_stay_whole`: `practising(MANY, 80, 24)`, `submit(&mut session, "neko")`; in `squashed(&render(..))` every one of the eight answers appears; the row with `Correct answers:` does not contain the last answer; the rows of `Score: 0 of 1` and of `text::FEEDBACK_HINT` come after the row with the last answer.
    - `many_accepted_answers_draw_at_any_size_without_panicking`: the same at 1x1, 5x2, 20x3 and 200x60.
  - Verify: the new tests pass; every existing `ui::tests` test passes unchanged, in particular `feedback_for_a_wrong_answer`, `feedback_lines_are_in_order` and `feedback_hint_is_centred`; fmt and clippy clean. Commit, e.g. `feat(practice): wrap long lists of accepted answers`.

- [ ] 2.2 Add i-adjectives and いい.
  - `src/conjugation.rs`: add `Group::IAdjective` (doc as D2) and its rows of D3, with two endings (in D3's order) where D3 lists two. Add `Group::Ii` (doc as D2, the new two-line text) **without** rows in `endings`: its arm there is `Group::Ii => None,` preceded by the comment `// conjugate builds いい's answers from ii_endings`. Add the private `fn ii_endings(form: Form, formality: Formality) -> Option<Vec<(&'static str, &'static str)>>` with the doc comment `/// いい's endings after its stem, as (written, reading) pairs: each variant in kana, then with 良.` returning `pick(form, formality, plain, polite)` over the nine rows of design.md D3b, copied exactly and in that order (plain non-past, negative, past, past negative, te-form; polite non-past, negative, past, past negative). In `conjugate`, before the existing code: `if group == Group::Ii { .. }` as design.md D2 describes (both texts must end in `いい`, else `Vec::new()`; the stems are put in front of each pair's written and reading ending; `ii_endings` giving `None` gives `Vec::new()`).
  - `src/areas/grammar.rs`: append words 35 to 42 of Appendix A.
  - New tests, `src/conjugation.rs`: `i_adjective_plain_forms` (高い / たかい, the five plain forms of Appendix B); `i_adjective_polite_forms_and_variants` (the four polite forms, two answers for the negative and past negative, in Appendix B's order); `i_adjective_polite_past_is_katta_desu` (the polite past of 高い is exactly `[a("高かったです", "たかかったです")]`, so no `高いでした`); `ii_forms` (`nine("いい", "いい", Group::Ii)` equals the nine lines of `a(..)` answers for いい in Appendix B, exactly and in order); `ii_non_past_starts_with_ii` (the first answer of the plain non-past is `a("いい", "いい")` and of the polite non-past `a("いいです", "いいです")`); `ii_keeps_what_comes_before_it` (`conjugate("かっこいい", "かっこいい", Group::Ii, Form::Negative, Formality::Plain) == vec![a("かっこよくない", "かっこよくない"), a("かっこ良くない", "かっこよくない")]`); `adjectives_have_no_polite_te_form` (高い, いい); `i_adjective_needs_its_ending` (`conjugate("静か", "しずか", Group::IAdjective, ..)` and `conjugate("高い", "たかい", Group::Ii, ..)` for the plain negative are empty).
  - New tests, `src/areas/grammar.rs`:
    - `i_adjective_polite_negative_accepts_both_variants`: `ask("高い", Form::Negative, Formality::Polite).accepted == strings(&["高くないです", "たかくないです", "高くありません", "たかくありません"])`; `is_correct` for `takakunaidesu`, `takakuarimasen` and `高くありません`; not for `takainaidesu`.
    - `ii_uses_yo`: `ask("いい", Form::Past, Formality::Plain).accepted == strings(&["よかった", "良かった"])`; `is_correct` for `yokatta`, `よかった` and `良かった`; not for `iikatta`.
    - `ii_accepts_yoi`: `ask("いい", Form::NonPast, Formality::Plain)` has prompt `"What is the plain non-past of いい (good)?"`, shown `strings(&["いい"])` and accepted `strings(&["いい", "よい", "良い"])`; `is_correct` for `ii`, `yoi`, `いい`, `よい`, `イイ` and `良い`; not for `yoku`. `ask("いい", Form::NonPast, Formality::Polite).accepted == strings(&["いいです", "よいです", "良いです"])`; `is_correct` for `iidesu`, `yoidesu` and `良いです`.
    - `ii_accepted_answers_of_every_form`: for each of the nine `ASKED` forms, `ask("いい", form, formality).accepted` equals the list for that form in design.md Appendix B, *Accepted answers of a question*, いい (copy the nine lists exactly).
  - Verify as in 1.1. Commit, e.g. `feat(grammar): conjugate i-adjectives`.

- [ ] 2.3 Accept the romaji `dewa` for the では of an accepted answer (design.md D18, owner decision).
  - **First, check:** `src/grading.rs` has `fn is_romaji_char`, and `check` ends with the `if let Some(read_as) = readings.iter().find(|r| accepted.contains(r)) { .. }` block followed by `Outcome::Wrong { read_as: readings.into_iter().next() }`, as in design.md D18. If not, stop and report.
  - `src/grading.rs`: add the private `fn dewa_for_deha` exactly as design.md D18 (with its doc comment), right above `pub fn check`. In `check`, add `let from_romaji = text.chars().all(is_romaji_char);` right after the `if text.is_empty() { return Outcome::Empty; }` block, and the `if from_romaji { .. }` block of D18 between the exact-match block and the final `Outcome::Wrong { .. }`. Change nothing else in the file, including `TypoGuard` and every existing test.
  - New tests, `src/grading.rs`, appended at the end of the existing `mod tests` (use the existing `correct`, `wrong` and `NOTHING`; do not edit them):
    - `dewa_is_accepted_for_deha_in_romaji`: accepted `["静かではない", "しずかではない"]`, shown `["静か", "しずか"]`: `shizukadewanai`, `shizukadehanai`, `SHIZUKADEWANAI` and `shizuka dewa nai` each give `correct("しずかではない")`.
    - `dewa_in_a_polite_form`: accepted `["静かではありません", "しずかではありません"]`: `shizukadewaarimasen` gives `correct("しずかではありません")`.
    - `kana_dewa_stays_wrong`: accepted `["静かではない", "しずかではない"]`, shown `["静か", "しずか"]`: `しずかでわない` and `シズカデワナイ` give `wrong("しずかでわない")`; `静かではない` gives `correct("静かではない")`; `しずかではない` gives `correct("しずかではない")`.
    - `dewa_composes_with_nn`: accepted `["今日ではない", "こんにちではない"]`: `konnichidewanai`, `konnnichidewanai` and `konnichidehanai` give `correct("こんにちではない")`; accepted `["禁煙ではない", "きんえんではない"]`: `kinnendewanai` gives `correct("きんえんではない")`.
    - `each_deha_may_be_typed_either_way`: accepted `["ではでは"]`: `dewadewa`, `dehadewa` and `dewadeha` give `correct("ではでは")`.
    - `other_ha_are_still_typed_ha`: `konnichiwa` against `["こんにちは"]` gives `wrong("こんにちわ")`; `watashiwa` against `["わたしは"]` gives `wrong("わたしわ")`.
    - `dewa_does_not_work_the_other_way_round`: `tedehakeru` against `["てでわける"]` gives `wrong("てではける")`.
    - `a_wrong_dewa_answer_keeps_its_reading`: `shizukadewaarimasu` against `["しずかではありません"]` gives `wrong("しずかでわあります")`.
  - New tests, `src/ui.rs`, in its `mod tests` (a test-only `struct NotQuiet` implementing `Area` with name `"NotQuiet"` and one N5 entry with one `Kind::Japanese` question: prompt `"What is the plain negative of 静か (しずか, quiet)?"`, accepted `["静かじゃない", "しずかじゃない", "静かではない", "しずかではない"]`, shown `["静か", "しずか"]`; `const NOT_QUIET: &[&dyn Area] = &[&NotQuiet];`; build it like the existing `LongPrompt`):
    - `dewa_reads_as_deha_on_the_feedback_screen`: `practising(NOT_QUIET, 80, 24)`, `submit(&mut session, "shizukadewanai")`; `squashed(&render(&session))` contains `Read as: しずかではない` and `text::CORRECT`, and does not contain `でわ`.
    - `kana_dewa_is_wrong_on_the_feedback_screen`: the same with `しずかでわない`: the squashed screen contains `Your answer: しずかでわない`, `text::WRONG` and `Correct answers: 静かじゃない, しずかじゃない, 静かではない, しずかではない` (on one line of the squashed text), and does not contain `Read as:`; exactly one line of the squashed text contains `でわ`, and it is the one with `Your answer:`.
  - Verify: the new tests pass; the 379 tests that were on `main` pass unchanged (design.md D13, D18), together with the tests added earlier in this branch; fmt and both clippy runs clean. Commit, e.g. `feat(grading): accept dewa for では in romaji answers`.

- [ ] 2.4 Add na-adjectives, and check that the word list covers every group.
  - `src/conjugation.rs`: add `Group::NaAdjective` (doc as D2) and its rows of D3 (cut `""`; an empty `text` gives `None`), with every variant in D3's order.
  - `src/areas/grammar.rs`: append words 43 to 49 of Appendix A.
  - New tests, `src/conjugation.rs`: `na_adjective_never_gives_dewa` (no `written` or `reading` of any answer of `nine("静か", "しずか", Group::NaAdjective)` contains `でわ`); `na_adjective_plain_forms` (静か / しずか, the five plain forms of Appendix B, two answers for the negatives); `na_adjective_polite_forms_and_variants` (the four polite forms, four answers for the negative and past negative, in Appendix B's order); `na_adjective_plain_non_past_has_da` (exactly `[a("静かだ", "しずかだ")]`); `na_adjective_ending_in_i_is_not_an_i_adjective` (`nine("きれい", "きれい", Group::NaAdjective)` equals Appendix B's きれい); `na_adjective_has_no_polite_te_form`; `empty_na_adjective_gives_nothing` (`conjugate("", "", Group::NaAdjective, Form::NonPast, Formality::Plain)` is empty).
  - New tests, `src/areas/grammar.rs`:
    - `na_adjective_polite_negative_accepts_four_variants`: `ask("静か", Form::Negative, Formality::Polite).accepted` equals Appendix B's eight strings for 静か, polite negative, in order; `is_correct` for `shizukadehaarimasen`, `shizukadewaarimasen`, `shizukajaarimasen`, `shizukajanaidesu` and `しずかではありません`; not for `しずかでわありません`.
    - `the_plain_negative_of_shizuka_typed_six_ways` (design.md D18 table): for `ask("静か", Form::Negative, Formality::Plain)` (accepted equal to Appendix B's `["静かじゃない", "しずかじゃない", "静かではない", "しずかではない"]`), `outcome(..)` is `Outcome::Correct { read_as: "しずかではない".to_string() }` for `shizukadehanai`, `shizukadewanai` and `しずかではない`; `Correct { read_as: "静かではない" }` for `静かではない`; `Outcome::Wrong { read_as: Some("しずかでわない".to_string()) }` for `しずかでわない`; `Correct { read_as: "しずかじゃない" }` for `shizukajanai`.
    - `dewa_in_the_polite_past_negative`: for `ask("静か", Form::PastNegative, Formality::Polite)`, `outcome(..)` is `Correct { read_as: "しずかではありませんでした" }` for `shizukadewaarimasendeshita` and `Correct { read_as: "しずかではなかったです" }` for `shizukadewanakattadesu`.
    - `no_accepted_answer_spells_dewa`: no accepted text of any question of `Grammar.questions(Levels::ALL)` contains `でわ`.
    - `na_adjective_plain_non_past_needs_da`: for `ask("静か", Form::NonPast, Formality::Plain)`, `is_correct` for `shizukada` and `しずかだ`; not for `しずか` or `shizuka`.
    - `the_word_list_covers_every_group` (spec *The starting word list*): for each of the nine `Group` variants, at least one word has it; `WORDS` has the words written `する`, `来る`, `行く`, `ある` and `いい`; at least one `Group::Suru` word has `written != reading`; at least two different `level` values are used.
  - Verify as in 1.1. Commit, e.g. `feat(grammar): conjugate na-adjectives`.

## 3. Documentation

- [ ] 3.1 Update `docs/architecture.md` for the Grammar area.
  - Status note: replace `Area selection lists the registered areas; none is registered yet, so it says "Nothing to practise yet".` with `Area selection lists the registered areas; the only one so far is Grammar (conjugation practice, \`areas::grammar\` with the rules in \`conjugation\`).` and replace `only a test area has questions so far.` with `Grammar is the first area with questions.` (without the backslashes).
  - §3 *Grammar*: after the example and the sentence "The template questions are the conjugations in the next section.", add a paragraph starting `**In this first version:**` followed by these bullets, in this wording:
    - `Each word has one written form (the same as its reading for a word written in kana), one reading, one meaning and one JLPT level. There is no id, description or custom question yet.`
    - `The groups are Ichidan, Godan, the irregular verbs Suru (する and noun + する), Kuru (来る), Iku (行く) and Aru (ある), and the adjectives IAdjective, Ii (いい) and NaAdjective. Na-adjectives are stored without だ or な.`
    - `The words are Rust constant data in src/areas/grammar.rs. Their JLPT levels are the project's own approximate judgment; the JLPT publishes no official vocabulary list.`
  - §4 *Conjugation*: after its bullet list, add the sentence `The rules of the first version, by group (cut = what is removed from the end of the word):` and copy both tables of design.md D3 (plain and polite) as they are, then the godan table of D3a without its example column, followed by: `Where a form has several standard variants, all are accepted: 高くないです and 高くありません; 静かじゃないです, 静かじゃありません, 静かではないです and 静かではありません. いい also accepts よい (よい, よいです), and its forms on よ are also accepted written with 良 (良い, 良くない, 良かった). The plain non-past of a na-adjective needs だ. The polite te-form does not exist.` Then copy the D3b table of design.md (form and pairs columns) under the sentence `いい's answers, as (written, reading) pairs:`.
  - §2 *Decisions*, *Checking answers*, the bullet `**Romaji is exact.**`: append, at its end, the sentence ``One exception: where an accepted answer has では, romaji may spell it `dewa`, as it is pronounced, so `shizukadewanai` is correct and shown as read しずかではない. でわ typed in kana stays wrong, and は elsewhere is still typed `ha` (`konnichiwa` is wrong for こんにちは).`` (keep the bullet's line wrapping style).
  - §5 *Components*, the *Area contract* bullet: replace `none is registered yet, and each area adds` with `Grammar is registered, and each area adds`.
  - §6 *Still open*: delete the bullet `**Conjugation details:** ...`; change the bullet `**The content file format**, and the step that adds an entry and assigns its id.` to `**The content file format**, and the step that adds an entry and assigns its id. Grammar keeps its words as Rust constant data, without ids, until then.`
  - Verify: `git diff --stat` shows only `docs/architecture.md` and `tasks.md`; the Mermaid blocks are unchanged. Commit, e.g. `docs(architecture): describe the Grammar area and its conjugation rules`.

- [ ] 3.2 Update `CHANGELOG.md`, `README.md` and `ATTRIBUTION.md`.
  - `CHANGELOG.md`, *Unreleased* / *Added*, as the **last** item of the list: `- Grammar area (issue #13): the first area with questions. It practises conjugation: a verb or adjective is shown with its reading and meaning, and the question asks for one of nine forms (plain non-past, negative, past, past negative and te-form; polite non-past, negative, past and past negative), picked at random. Answers are generated by rules for ichidan and godan verbs, する, 来る, 行く, ある, i-adjectives (including いい) and na-adjectives, and are accepted with kanji, in kana or in romaji; every standard polite variant of an adjective is accepted, いい also accepts よい and the 良 spellings (良くない), and the では of a na-adjective may be typed `dewa` in romaji (でわ typed in kana stays wrong). 49 common words at N5 and N4 are built in. Long lists of accepted answers now wrap on the feedback screen.`
  - `README.md`, the status note: replace `There's no usable learning content yet, and the features below are the plan, not what exists today.` with `The first learning module, Grammar (conjugation practice for common verbs and adjectives), works; the rest of the features below are the plan, not what exists today.`
  - `ATTRIBUTION.md`: below the table, before `When adding content:`, add the paragraph of design.md D9 (from `Content written for this project` to `none of the unofficial lists was used.`). Keep the table's `_None yet_` row.
  - Verify: `git diff --stat` shows only these three files and `tasks.md`; the project version is unchanged. Commit, e.g. `docs: record the Grammar area in the changelog, readme and attribution`.

## 4. Integration check

- [ ] 4.1 Drive the release binary through a terminal (design.md D17, no person needed).
  - `cargo build --release`, then run the steps of D17 with `tmux` at 80x24 against `target/release/tui-learnjapanese`. Keep the script outside the repository. After every key, wait about 0.3 s before `tmux capture-pane -p`.
  - Confirm in order: `Press Enter to start`; `What do you want to practise?` with `> Grammar`; the level picker with `Grammar` and `> [x] N5`; a question starting `What is the ` with `Answer in Japanese or romaji` and `Score: 0 of 0`; `neko` gives `Your answer: neko`, `Read as: ねこ`, `Not quite.` and a `Correct answer` line; a repeated prompt answered in romaji gives `Correct!` and `Read as:` with the kana; Esc shows `What do you want to practise?`; Esc exits with status 0.
  - Record each result (and how many questions it took to see a repeated prompt) in the report. If a step fails, stop and report; do not change code in this task.
  - This task changes no files: commit only the ticked checkbox, e.g. `chore(openspec): record the Grammar terminal check`.

## Workflow follow-up

- **The real IME check (needs a person, design.md D17 and Open Question 8):** the owner, or a contributor with that platform, opens Grammar in the release build on Linux, macOS and Windows terminals and over SSH, and on a question types the answer with the system IME in hiragana, in katakana and converted to kanji, then in romaji, and uses Backspace on IME text. Record the terminal, OS and result for each in the pull request for #13.
- **Open questions** in design.md have defaults that the tasks above follow; if the owner chooses otherwise before implementation, revise the affected tasks first (for example, Open Question 3 removes task 2.1).
- **Archive:** this change adds new capabilities only, so it archives on its own; archiving it after `add-generic-practice` is recommended (design.md D16).
