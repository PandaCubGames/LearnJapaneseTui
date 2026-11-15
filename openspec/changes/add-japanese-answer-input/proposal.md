# Proposal

## Why

Vocabulary (#11), Kanji (#12) and Grammar (#13) all ask for Japanese answers, through the generic practice module (#21). Today the app has no text input beyond reporting a single key, no romaji conversion and no answer checking. Each area would otherwise invent its own, and a user typing `takai` would be marked differently from one typing `たかい` through an IME. This change builds one shared way to take a Japanese answer and check it. Tracked in GitHub issue #18.

The app cannot see the IME; it only receives finished UTF-8 text. So the approach is: take the text as typed, convert nothing while typing, normalise only when checking, compare, and give back the kana the answer was read as, for feedback.

## What Changes

- **Answer field:** a text input field that accepts any UTF-8 text, including text committed by an IME or pasted. Backspace removes one whole character (one grapheme cluster), never a byte. The field is drawn and its cursor placed by terminal display width, so kana and kanji take two columns. A long answer scrolls so its end and the cursor stay visible.
- **Romaji to hiragana:** a pure function, no state and no terminal. Hepburn plus the common typed variants that give the same kana (`shi`/`si`, `chi`/`ti`, `tsu`/`tu`, `fu`/`hu`, `ji`/`zi`, `sha`/`sya`, `cha`/`tya`, `ja`/`jya`/`zya`, and more listed in `design.md`). A doubled consonant gives っ (`kitte` → `きって`). `n` gives ん before a consonant and at the end; `nn` and `n'` give ん (`kinen` → `きねん`, `kin'en` → `きんえん`). `-` gives `ー`. Capitals count as lower case. Text that cannot be read reports that it cannot be converted; the function never guesses.
- **Katakana folding:** katakana is turned into hiragana before comparing (`タカイ` matches `たかい`).
- **Answer checking:** one function checks a typed answer against the question's accepted answers. Romaji, hiragana, katakana and the written form with kanji are all accepted. An answer that repeats what the question already shows is never accepted. The owner's decisions are binding:
  - Romaji is exact, letter for letter. Long vowels are typed out as in kana: `とうきょう` is `toukyou`; `tokyo` and `tōkyō` are wrong answers.
  - Mixed input is not accepted: `食beru` and `たberu` are plain wrong answers, with no warning.
  - Input that cannot be read as Japanese at all (`tabxru`) gets a "looks like a typo" warning on the first submit and is not counted; the same text submitted again is a counted wrong answer; changed text is checked from the start. A readable but wrong word (`tokyo`, read as `ときょ`) is a normal wrong answer.
- **No screen uses this yet.** #21 builds the question screen that consumes it. Until then the new modules are reached only by their tests, and a scoped, self-removing lint expectation keeps `cargo clippy -D warnings` passing (`design.md` D1).
- No new dependency. Width and grapheme handling come from `ratatui`, already used.

Not in scope: kana to kanji conversion; live conversion while typing; checking English (meaning) answers; half-width katakana and full-width Latin letters; the question screen, its wording and its keys (#21); enabling bracketed paste in the terminal.

## Capabilities

### New Capabilities
- `romaji-conversion`: turning typed romaji into hiragana: the table, the っ and ん rules, the long-vowel mark, upper case, and when text cannot be read.
- `answer-checking`: deciding whether a typed Japanese answer is correct: how input is classified (romaji, Japanese, mixed, unreadable, empty), katakana folding and other normalisation, comparison with the accepted answers and with what the question shows, and the typo warning that is given once per text.
- `answer-input`: the text field an answer is typed into: which key events edit it, how Backspace removes a character, and how it is drawn and where its cursor goes, by display width.

### Modified Capabilities

None. `openspec/specs/` is empty, and these are new behaviours. Nothing in the unarchived changes (`add-main-application-loop`, `add-welcome-screen`) is contradicted: no existing key, screen or action changes.

## Impact

- **Code:** new files `src/kana.rs`, `src/romaji.rs`, `src/grading.rs`, `src/answer_field.rs`; four `mod` lines in `src/main.rs`. No existing function, type, key mapping or screen changes.
- **Public API for #21:** `grading::check`, `grading::TypoGuard`, `grading::Outcome`, `answer_field::AnswerField`, `answer_field::Edit`, `answer_field::edit_for` (`design.md` D2).
- **Tests:** new unit tests in each new file. No existing test is changed; the baseline is 51 passing tests.
- **Docs:** `docs/architecture.md` (status note, Components), `CHANGELOG.md` (*Unreleased* / *Added*).
- **Not affected:** `Cargo.toml` and `Cargo.lock` (no dependency change), the project version, CI, learning content.
