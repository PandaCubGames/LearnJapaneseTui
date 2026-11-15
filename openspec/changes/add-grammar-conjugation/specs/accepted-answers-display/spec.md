# Spec Delta

## Purpose

When an answer is wrong, the feedback screen lists the accepted answers; this capability keeps that list readable when it is wider than the terminal, as it is for adjective forms with several standard variants.

## ADDED Requirements

### Requirement: Accepted answers wrap
On the feedback screen, the line of accepted answers (`Correct answer: X` or `Correct answers: X, Y`) SHALL wrap onto up to four rows when it is wider than the terminal, breaking between words, each row centred by display width. A line that fits SHALL stay on one row. The rows below it (score and key hint) SHALL stay visible.

#### Scenario: Eight long answers in 80 columns
- **WHEN** a question with the eight accepted answers 簡単じゃなかったです, かんたんじゃなかったです, 簡単じゃありませんでした, かんたんじゃありませんでした, 簡単ではなかったです, かんたんではなかったです, 簡単ではありませんでした, かんたんではありませんでした is answered wrongly in a terminal of 80 columns by 24 rows
- **THEN** every one of the eight answers is shown in full on the screen
- **AND** the `Correct answers:` text uses more than one row
- **AND** `Score: 0 of 1` and `Press Enter for the next question, Esc to stop` are shown below it

#### Scenario: A short list stays on one row
- **WHEN** the polite past of 食べる is answered with `ねこ` in a terminal 80 columns wide
- **THEN** `Correct answers: 食べました, たべました` is shown on one row

#### Scenario: Small terminals
- **WHEN** the feedback screen with the eight long answers is drawn at 1x1, 5x2, 20x3 and 200x60
- **THEN** drawing finishes without a panic
