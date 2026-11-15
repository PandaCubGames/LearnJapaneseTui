# Spec Delta

## Purpose

The practice loop is the part every area shares once levels are chosen: show a question, take a typed answer, check it, give feedback, count it, and go on to the next question until the user leaves.

## ADDED Requirements

### Requirement: Question screen
The question screen SHALL show the area's name, the question's prompt, the label `Answer: ` followed by the answer field, a line saying how to answer (`Answer in Japanese or romaji` for a Japanese question, `Answer in English` for a meaning question), the score, and the hint `Enter to answer, Esc to stop`. The terminal cursor SHALL be placed in the answer field, after the typed text.

#### Scenario: A Japanese question
- **WHEN** the question `How do you read 高い?` is shown
- **THEN** the screen shows the area name, `How do you read 高い?`, `Answer: `, `Answer in Japanese or romaji`, `Score: 0 of 0` and `Enter to answer, Esc to stop`

#### Scenario: A meaning question
- **WHEN** a meaning question is shown
- **THEN** the screen shows `Answer in English`

#### Scenario: Cursor in the field
- **WHEN** `たか` has been typed on the question screen in a terminal 80 columns wide
- **THEN** the cursor is on the row of `Answer: `, at column 33 (the field starts at column 29 and `たか` takes 4 columns)

### Requirement: Typing an answer
On the question screen, the keys of the answer field (characters, Backspace) SHALL edit the answer, including `q` and Space, which type themselves there. A held key SHALL keep typing.

#### Scenario: Typing and deleting
- **WHEN** the user types `t`, `a`, `k`, `a`, `x`, presses Backspace, and types `i`
- **THEN** the field holds `takai`

#### Scenario: q types
- **WHEN** the user presses `q` on the question screen
- **THEN** the field holds `q` and the app keeps running

### Requirement: Submitting an answer
Enter without modifiers, not a repeat, SHALL submit the answer. An empty answer or one made only of spaces MUST be ignored. A Japanese answer SHALL be checked with the shared answer checking (romaji, kana and kanji accepted; an answer that is a shown text containing a kanji is wrong, while shown text in kana only never makes an answer wrong). A meaning answer SHALL be checked with meaning checking. A counted answer SHALL show the feedback screen.

#### Scenario: Correct romaji
- **WHEN** the question accepts `たかい` and the user types `takai` and presses Enter
- **THEN** the feedback screen says `Correct!`

#### Scenario: Shown kanji copied back
- **WHEN** the question shows `高い`, accepts `高い` and `たかい`, and the user submits `高い`
- **THEN** the feedback screen says `Not quite.`, while submitting `takai` says `Correct!`

#### Scenario: Shown kana typed back
- **WHEN** the question shows `する`, accepts `する`, and the user submits `する` or `suru`
- **THEN** the feedback screen says `Correct!`

#### Scenario: Empty answer
- **WHEN** the field is empty or holds only spaces and the user presses Enter
- **THEN** the question screen stays and nothing is counted

### Requirement: Typo warning
When a Japanese answer cannot be read as Japanese, the question screen SHALL stay, keep the typed text, and show `This does not read as Japanese. Press Enter again to submit it anyway.` The warning MUST NOT be counted. It SHALL disappear when the answer is edited. Submitting the same text again SHALL count it as a wrong answer.

#### Scenario: Warned, then counted
- **WHEN** the user submits `tabxru` to a Japanese question, then presses Enter again
- **THEN** the first Enter shows the warning, keeps `tabxru` and counts nothing, and the second shows the feedback screen with `Not quite.` and the score `Score: 0 of 1`

#### Scenario: Warning cleared by editing
- **WHEN** the warning is shown and the user presses Backspace
- **THEN** the warning is no longer shown

### Requirement: Feedback screen
The feedback screen SHALL show the area's name, the prompt, `Your answer: ` with the answer as typed (spaces at both ends removed), `Read as: ` with the reading when the answer was read as Japanese and the reading differs from it, `Correct!` or `Not quite.`, and when wrong the accepted answers in the question's order (`Correct answer: X` for one, `Correct answers: X, Y` for several), the score, and the hint `Press Enter for the next question, Esc to stop`.

#### Scenario: Correct in romaji
- **WHEN** the question accepts `たかい` and the user submits `takai`
- **THEN** the feedback shows `Your answer: takai`, `Read as: たかい` and `Correct!`, and no `Correct answer`

#### Scenario: Correct in kana
- **WHEN** the question accepts `たかい` and the user submits `たかい`
- **THEN** the feedback shows `Correct!` and no `Read as:` line

#### Scenario: Wrong
- **WHEN** the question accepts `食べました` and `たべました` and the user submits `ねこ`
- **THEN** the feedback shows `Your answer: ねこ`, `Not quite.` and `Correct answers: 食べました, たべました`

### Requirement: Counting answers
Each submitted answer that is checked as correct or wrong SHALL be counted exactly once, and a correct one SHALL also be counted as correct. Continuing from feedback, an empty submit, a typo warning and leaving with Esc MUST NOT change the counts. The question and feedback screens SHALL show `Score: <correct> of <answered>`. The counts SHALL start at zero each time practice starts.

#### Scenario: Two answers
- **WHEN** the user answers one question correctly, continues, answers the next one wrongly and continues
- **THEN** the question screen shows `Score: 1 of 2`

#### Scenario: Continue is not counted
- **WHEN** the feedback screen is shown after the first correct answer and the user presses Enter
- **THEN** the next question screen shows `Score: 1 of 1`

### Requirement: Next question
On the feedback screen, Enter without modifiers, not a repeat, SHALL show the next question with an empty answer field and no typo warning. Practice SHALL go on until the user leaves. Every other key except Esc and `Ctrl+C` MUST do nothing there.

#### Scenario: Enter on feedback
- **WHEN** the feedback screen is shown and the user presses Enter
- **THEN** a question screen is shown with an empty answer field

#### Scenario: Other keys on feedback
- **WHEN** the feedback screen is shown and the user presses `a`, Space, Backspace or Up
- **THEN** the feedback screen stays

### Requirement: Leaving practice
Esc without modifiers on the question screen or the feedback screen SHALL go back to area selection with the same area highlighted. An unfinished answer SHALL be dropped and not counted. `Ctrl+C` SHALL quit on both screens.

#### Scenario: Esc while answering
- **WHEN** the user has typed `たか` without submitting and presses Esc
- **THEN** area selection is shown with the practised area highlighted, and when practice starts again the score is `Score: 0 of 0` and the field is empty

#### Scenario: Esc on feedback
- **WHEN** the feedback screen is shown and the user presses Esc
- **THEN** area selection is shown

#### Scenario: Ctrl+C while answering
- **WHEN** the question screen is shown and the user presses `Ctrl+C`
- **THEN** the app quits

### Requirement: Practice screens layout
Every text line of the question and feedback screens SHALL be centred horizontally by terminal display width, a prompt wider than the terminal SHALL wrap onto more lines, both screens SHALL be centred vertically, and they MUST stay centred after a resize. The answer label and a 30-column answer field SHALL be centred together as one 38-column block.

#### Scenario: Field position
- **WHEN** the question screen is shown 80 columns wide
- **THEN** `Answer: ` starts at column 21 and the empty field's cursor is at column 29

#### Scenario: Field position after a resize
- **WHEN** the terminal is resized to 100 columns while the question screen is shown
- **THEN** `Answer: ` starts at column 31

#### Scenario: Long prompt
- **WHEN** a prompt of 100 columns is shown in a terminal 80 columns wide
- **THEN** its words are drawn on two rows and nothing panics

### Requirement: Practice screens at any size and without colour
The question and feedback screens MUST draw without panicking at any terminal size down to one column by one row, and MUST NOT use colour.

#### Scenario: Sizes
- **WHEN** each screen is drawn at 1x1, 5x2, 20x3 and 200x60
- **THEN** drawing finishes without a panic

#### Scenario: No colour
- **WHEN** each screen is drawn at 80x24 with an answer typed
- **THEN** every cell has the terminal's default foreground and background colour
