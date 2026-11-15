# Spec Delta

## Purpose

Question picking decides which entries of an area are practised at the chosen levels and in which order their questions are asked, so that practice is varied, every entry gets its turn, and tests can fix the order.

## ADDED Requirements

### Requirement: Entries practised at the chosen levels
An entry SHALL be practised when it has any of the chosen levels and at least one question. Entries without any chosen level and entries without questions MUST NOT be asked, even when an area supplies them.

#### Scenario: Only N3 chosen
- **WHEN** only N3 is ticked for the Sample area and twenty questions are answered
- **THEN** every question asked is the Sample area's N3 question

#### Scenario: An area that ignores the levels
- **WHEN** an area supplies an N1 entry while only N5 is chosen
- **THEN** the N1 entry's question is never asked

#### Scenario: An entry with no questions
- **WHEN** an area supplies an entry at a chosen level with no questions, next to one entry with a question
- **THEN** only the entry with a question is ever asked

### Requirement: Random entry, then random question
The next question SHALL be chosen by first picking one of the practised entries at random and then one of that entry's questions at random.

#### Scenario: Both questions of an entry are asked
- **WHEN** the only practised entry has two questions and forty questions are asked with a fixed seed
- **THEN** both questions are asked at least once

### Requirement: No entry repeats until all have been seen
Within a run of practice, an entry MUST NOT be picked again until every practised entry has been picked once. When a new round starts and there are at least two entries, its first entry MUST NOT be the entry asked just before.

#### Scenario: Rounds
- **WHEN** there are three practised entries and nine questions are asked
- **THEN** questions 1 to 3, 4 to 6 and 7 to 9 each come from three different entries

#### Scenario: No entry twice in a row
- **WHEN** there are two or more practised entries and fifty questions are asked, for each of twenty different seeds
- **THEN** no two questions in a row come from the same entry

#### Scenario: One entry
- **WHEN** there is only one practised entry
- **THEN** every question comes from it

### Requirement: Reproducible order
The order of questions SHALL depend only on the seed of the random number generator and the entries supplied. The same seed and the same entries MUST give the same order. When the app runs, the seed SHALL come from the current time, so the order differs between runs.

#### Scenario: Same seed, same order
- **WHEN** two practice runs on the Sample area with all levels ticked use the same seed and ten questions are answered in each
- **THEN** both runs ask the same ten prompts in the same order

#### Scenario: The clock gives the seed
- **WHEN** the seed is taken from two different times
- **THEN** the two seeds differ
