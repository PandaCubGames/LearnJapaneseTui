# Spec Delta

## Purpose

Answer checking (created by `add-japanese-answer-input`) gains one narrow exception to exact romaji: the では of an accepted answer may be typed `dewa`, the way it is pronounced, as in the na-adjective negatives of the Grammar area (静かではない). Nothing else about checking changes: は elsewhere is still typed `ha`, and でわ typed in Japanese is still wrong.

## ADDED Requirements

### Requirement: では may be typed dewa
Where an accepted answer contains では, a romaji answer SHALL also be correct when one of its readings equals that accepted answer except that one or more of its では are read でわ (typed `dewa`). Every other character MUST match exactly. The rule SHALL apply to every reading of the romaji (both readings of `nn` before a vowel) and after whitespace is removed, as all checking does.

#### Scenario: Plain form typed with dewa
- **WHEN** the question shows `静か` and `しずか`, the accepted answers are `静かではない` and `しずかではない`, and the answer is `shizukadewanai`, `shizukadehanai`, `SHIZUKADEWANAI` or `shizuka dewa nai`
- **THEN** each answer is correct, read as `しずかではない`

#### Scenario: Polite form typed with dewa
- **WHEN** the accepted answers are `静かではありません` and `しずかではありません` and the answer is `shizukadewaarimasen`
- **THEN** the answer is correct, read as `しずかではありません`

#### Scenario: Together with both readings of nn
- **WHEN** the accepted answers are `今日ではない` and `こんにちではない` and the answer is `konnichidewanai`, `konnnichidewanai` or `konnichidehanai`
- **THEN** each answer is correct, read as `こんにちではない`
- **AND** with the accepted answers `禁煙ではない` and `きんえんではない`, the answer `kinnendewanai` is correct through its keyboard reading, read as `きんえんではない`

#### Scenario: Each では may be typed either way
- **WHEN** the accepted answer is `ではでは` and the answer is `dewadewa`, `dehadewa` or `dewadeha`
- **THEN** each answer is correct, read as `ではでは`

### Requirement: The dewa rule is narrow
The requirement *では may be typed dewa* MUST NOT apply to answers typed in Japanese: でわ typed in hiragana or katakana stays a wrong answer. It MUST NOT apply to a は that does not directly follow で (`konnichiwa` stays wrong for こんにちは), nor the other way round (`deha` does not match an accepted でわ). It never changes the accepted answers, so でわ is never listed as a correct answer, and a wrong answer keeps its own reading.

#### Scenario: でわ typed in Japanese stays wrong
- **WHEN** the question shows `静か` and `しずか`, the accepted answers are `静かではない` and `しずかではない`, and the answer is `しずかでわない` or `シズカデワナイ`
- **THEN** each answer is wrong, read as `しずかでわない`
- **AND** `しずかではない` and `静かではない` are correct, each read as itself

#### Scenario: Other は are still typed ha
- **WHEN** the accepted answer is `こんにちは` and the answer is `konnichiwa`, or the accepted answer is `わたしは` and the answer is `watashiwa`
- **THEN** each answer is wrong, read as `こんにちわ` and `わたしわ`

#### Scenario: Not the other way round
- **WHEN** the accepted answer is `てでわける` and the answer is `tedehakeru`
- **THEN** the answer is wrong, read as `てではける`

#### Scenario: A wrong answer keeps its own reading
- **WHEN** the accepted answer is `しずかではありません` and the answer is `shizukadewaarimasu`
- **THEN** the answer is wrong, read as `しずかでわあります`

## MODIFIED Requirements

### Requirement: Romaji is exact
A romaji answer SHALL be correct only when one of its readings, read letter for letter, equals an accepted answer, or matches one through the requirement *では may be typed dewa*. Long vowels MUST be typed out as in kana. A romaji answer that contains a vowel with a macron or circumflex (`ō`, `ô` and the like) MUST be a wrong answer, never correct and never a typo warning.

#### Scenario: Long vowel typed out
- **WHEN** the accepted answers are `東京` and `とうきょう` and the answer is `toukyou`
- **THEN** the answer is correct

#### Scenario: Long vowel left out
- **WHEN** the accepted answers are `東京` and `とうきょう` and the answer is `tokyo`
- **THEN** the answer is wrong, without a typo warning, and it was read as `ときょ`

#### Scenario: Macron vowels
- **WHEN** the accepted answers are `東京` and `とうきょう` and the answer is `tōkyō`
- **THEN** the answer is wrong, without a typo warning

### Requirement: Which reading is reported when romaji has several
When a romaji answer has several readings, a correct answer SHALL come with the reading that matched, and a wrong answer with its textbook reading (`nn` before a vowel read as `ん` plus an `n`-row syllable). A romaji answer that is correct only through the requirement *では may be typed dewa* SHALL come with the accepted answer it matched, in kana and spelled with では, never with でわ. A Japanese answer has exactly one reading, which SHALL be reported whether it is correct or wrong.

#### Scenario: Wrong answer with two readings
- **WHEN** the accepted answer is `こんばんは` and the answer is `konnichiwa`
- **THEN** the answer is wrong, read as `こんにちわ`

#### Scenario: Correct through the keyboard reading
- **WHEN** the accepted answer is `こんや` and the answer is `konnya`
- **THEN** the answer is correct, read as `こんや`

#### Scenario: Correct through dewa
- **WHEN** the accepted answers are `静かではない` and `しずかではない` and the answer is `shizukadewanai`
- **THEN** the answer is correct, read as `しずかではない`, not `しずかでわない`
