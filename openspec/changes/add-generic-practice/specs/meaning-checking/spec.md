# Spec Delta

## Purpose

Meaning checking decides whether an answer typed in English matches one of a question's accepted meanings, forgiving capital letters, extra spaces and a leading "to", "a", "an" or "the", and never converting the text to kana.

## ADDED Requirements

### Requirement: Meanings compared without case and extra spaces
A meaning answer SHALL be correct when, after normalising, it equals one accepted meaning after the same normalising. Normalising SHALL turn letters to lower case, remove spaces at both ends, and turn every run of whitespace inside into one space.

#### Scenario: Capitals and spaces
- **WHEN** the accepted meanings are `to eat` and the answer is `  To   EAT `
- **THEN** the answer is correct

#### Scenario: A different word
- **WHEN** the accepted meanings are `high`, `tall` and `expensive` and the answer is `cheap`
- **THEN** the answer is wrong

### Requirement: Leading article or "to" ignored
After normalising, one leading `to `, `a `, `an ` or `the ` SHALL be removed from both the answer and every accepted meaning before comparing. Only a whole leading word is removed: a word that merely starts with those letters MUST stay as it is.

#### Scenario: With and without "to"
- **WHEN** the accepted meaning is `to eat` and the answer is `eat`, or the accepted meaning is `eat` and the answer is `to eat`
- **THEN** both answers are correct

#### Scenario: Articles
- **WHEN** the accepted meaning is `mountain` and the answer is `a mountain`, `the mountain` or `An mountain`
- **THEN** each answer is correct

#### Scenario: Not a whole word
- **WHEN** the accepted meaning is `day` and the answer is `today`
- **THEN** the answer is wrong

### Requirement: Meaning answers are plain text
A meaning answer MUST NOT be converted to kana and MUST NOT get a typo warning. An empty answer, or one made only of whitespace, SHALL be reported as empty and not checked.

#### Scenario: Romaji is not converted
- **WHEN** the accepted meaning is `tall` and the answer is `takai`
- **THEN** the answer is wrong, with no typo warning

#### Scenario: Unreadable letters
- **WHEN** the accepted meaning is `tall` and the answer is `xqzt`
- **THEN** the answer is wrong, with no typo warning

#### Scenario: Empty
- **WHEN** the answer is empty or `   `
- **THEN** it is reported as empty
