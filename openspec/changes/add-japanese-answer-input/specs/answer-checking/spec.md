# Spec Delta

## Purpose

Answer checking decides whether a typed Japanese answer is correct, whatever script it was typed in, and tells a likely typo apart from a wrong answer. Every area that asks for a Japanese answer uses it, so the same answer is judged the same way everywhere.

## ADDED Requirements

### Requirement: Any script is accepted
A Japanese answer SHALL be correct when it matches one of the question's accepted answers, whether it is typed in romaji, hiragana, katakana or the written form with kanji. Romaji is converted to hiragana and katakana is folded to hiragana before comparing; kanji are compared as written.

#### Scenario: Four ways to answer 高い
- **WHEN** the accepted answers are `高い` and `たかい`, the question shows no Japanese, and the answer is `takai`, `たかい`, `タカイ` or `高い`
- **THEN** each answer is correct

#### Scenario: Katakana accepted answer
- **WHEN** the accepted answer is `ショク` and the answer is `しょく`, `ショク` or `shoku`
- **THEN** each answer is correct

### Requirement: Romaji is exact
A romaji answer SHALL be correct only when one of its readings, read letter for letter, equals an accepted answer. Long vowels MUST be typed out as in kana. A romaji answer that contains a vowel with a macron or circumflex (`ō`, `ô` and the like) MUST be a wrong answer, never correct and never a typo warning.

#### Scenario: Long vowel typed out
- **WHEN** the accepted answers are `東京` and `とうきょう` and the answer is `toukyou`
- **THEN** the answer is correct

#### Scenario: Long vowel left out
- **WHEN** the accepted answers are `東京` and `とうきょう` and the answer is `tokyo`
- **THEN** the answer is wrong, without a typo warning, and it was read as `ときょ`

#### Scenario: Macron vowels
- **WHEN** the accepted answers are `東京` and `とうきょう` and the answer is `tōkyō`
- **THEN** the answer is wrong, without a typo warning

### Requirement: Both readings of nn are accepted
A romaji answer with `nn` before a vowel or `ya`/`yu`/`yo` SHALL be correct when either its textbook or its keyboard reading equals an accepted answer. Romaji without such an `nn` keeps its single reading.

#### Scenario: Two spellings of こんにちは
- **WHEN** the accepted answer is `こんにちは` and the answer is `konnichiha` or `konnnichiha`
- **THEN** each answer is correct, read as `こんにちは`

#### Scenario: Two spellings of おんな
- **WHEN** the accepted answers are `女` and `おんな` and the answer is `onna` or `onnna`
- **THEN** each answer is correct, read as `おんな`

#### Scenario: Keyboard reading matches
- **WHEN** the accepted answers are `今夜` and `こんや` and the answer is `konnya` or `kon'ya`
- **THEN** each answer is correct, read as `こんや`, while `konya` is wrong, read as `こにゃ`

#### Scenario: Single readings stay exact
- **WHEN** the accepted answer is `きんえん` and the answer is `kinen`
- **THEN** the answer is wrong, read as `きねん`, while `kin'en` is correct

### Requirement: Mixed scripts are wrong
An answer that contains both Japanese characters and Latin letters SHALL be a wrong answer, with no typo warning.

#### Scenario: Kanji with romaji
- **WHEN** the accepted answers are `食べる` and `たべる` and the answer is `食beru`
- **THEN** the answer is wrong, without a typo warning

#### Scenario: Kana with romaji
- **WHEN** the accepted answers are `食べる` and `たべる` and the answer is `たberu`
- **THEN** the answer is wrong, without a typo warning

### Requirement: Shown text with kanji is never accepted
An answer MUST be wrong when, after normalisation, it equals Japanese text the question shows that contains at least one kanji (a CJK ideograph, `々`, `〆` or `〇`), even when that text is also accepted. Shown text written only in kana MUST NOT make an answer wrong. Containing a shown text is not repeating it. So a romaji answer, read as kana, is never a repeat.

#### Scenario: Repeating the kanji asked about
- **WHEN** the question shows `食`, the accepted answers are `食`, `しょく` and `たべる`, and the answer is `食`
- **THEN** the answer is wrong, read as `食`, while `しょく`, `たべる` and `taberu` are correct

#### Scenario: Copying a shown word written with kanji
- **WHEN** the question shows `食べる`, the accepted answers are `食べる` and `たべる`, and the answer is `食べる`, `食 べる` or `食ベル`
- **THEN** each answer is wrong, read as `食べる`, while `たべる`, `タベル` and `taberu` are correct

#### Scenario: Shown text in kana only blocks nothing
- **WHEN** the question shows `する`, the accepted answer is `する`, and the answer is `する`, `スル` or `suru`
- **THEN** each answer is correct, read as `する`

#### Scenario: The shown reading may be typed in any script
- **WHEN** the question shows `たかい`, the accepted answers are `高い` and `たかい`, and the answer is `takai`, `タカイ`, `たかい` or `高い`
- **THEN** each answer is correct

#### Scenario: Romaji is never a repeat
- **WHEN** the question shows `今夜` and `こんや`, the accepted answers are `今夜` and `こんや`, and the answer is `konnya`
- **THEN** the answer is correct, read as `こんや`, while `今夜` is wrong

#### Scenario: The iteration mark counts as kanji
- **WHEN** the question shows `々`, the accepted answer is `々`, and the answer is `々`
- **THEN** the answer is wrong

#### Scenario: Containing the shown kanji is not repeating it
- **WHEN** the question shows `食`, the accepted answer is `食べる`, and the answer is `食べる`
- **THEN** the answer is correct

### Requirement: Spaces are ignored
Whitespace anywhere in an answer or an accepted answer, including the ideographic space, SHALL be ignored when checking.

#### Scenario: Spaces around and inside
- **WHEN** the accepted answer is `たべてください` and the answer is ` tabete kudasai `, or `たべて　ください` with an ideographic space
- **THEN** both answers are correct

### Requirement: Empty answers are not checked
An answer that is empty or only whitespace SHALL be reported as empty: neither correct nor wrong, and not a typo.

#### Scenario: Nothing typed
- **WHEN** the answer is the empty text, `   ` or an ideographic space alone
- **THEN** the result is empty

### Requirement: Unreadable answers look like a typo
An answer that cannot be read as Japanese at all SHALL be reported as looking like a typo: romaji that has no valid reading, or an answer that is not mixed (see above) but contains a character that is neither romaji (Latin letters, `'`, `-`) nor Japanese, such as a digit, full-width Latin or half-width katakana.

#### Scenario: Letters that spell no kana
- **WHEN** the accepted answer is `たべる` and the answer is `tabxru`
- **THEN** the result is a typo warning

#### Scenario: Digits and out-of-scope scripts
- **WHEN** the answer is `taka1`, `たか1`, `ｔａｋａｉ` or `ﾀｶｲ`
- **THEN** each result is a typo warning

#### Scenario: Trailing lone consonant
- **WHEN** the answer is `tabek`
- **THEN** the result is a typo warning

#### Scenario: An nn spot does not make text readable
- **WHEN** the accepted answer is `こんにちは` and the answer is `konnichiq`
- **THEN** the result is a typo warning

### Requirement: A typo warning is given once per text
Within one question, the first submit of an unreadable text SHALL give a typo warning that is not counted. When the next non-empty submit is exactly the same text, it SHALL be a counted wrong answer. A different text SHALL be checked from the start, so a different unreadable text gets its own warning. "The same text" compares the typed text exactly, never its readings.

#### Scenario: Same text twice
- **WHEN** `tabxru` is submitted, then `tabxru` is submitted again
- **THEN** the first result is a typo warning and the second is a wrong answer

#### Scenario: Text corrected after the warning
- **WHEN** `tabxru` is submitted, then `taberu` is submitted, and the accepted answer is `たべる`
- **THEN** the first result is a typo warning and the second is correct

#### Scenario: A different unreadable text
- **WHEN** `tabxru` is submitted, then `tabwru`, then `tabwru` again
- **THEN** the results are a typo warning, a typo warning and a wrong answer

#### Scenario: An empty submit in between
- **WHEN** `tabxru` is submitted, then the empty text, then `tabxru`
- **THEN** the results are a typo warning, empty and a wrong answer

#### Scenario: Wrong but readable is never a warning
- **WHEN** the accepted answer is `とうきょう` and `tokyo` is submitted
- **THEN** the result is a wrong answer straight away

### Requirement: The reading is reported for feedback
A correct or wrong answer that could be read SHALL come with its reading: romaji as hiragana, Japanese with katakana folded to hiragana and kanji kept. Mixed answers, macron romaji and unreadable answers SHALL come with no reading.

#### Scenario: Readings
- **WHEN** the answers `tokyo`, `タカイ` and `高い` are checked against the accepted answer `たかい`
- **THEN** their readings are `ときょ`, `たかい` and `高い`

#### Scenario: No reading
- **WHEN** the answer `食beru` is checked
- **THEN** it is wrong and has no reading

### Requirement: Which reading is reported when romaji has several
When a romaji answer has several readings, a correct answer SHALL come with the reading that matched, and a wrong answer with its textbook reading (`nn` before a vowel read as `ん` plus an `n`-row syllable). A Japanese answer has exactly one reading, which SHALL be reported whether it is correct or wrong.

#### Scenario: Wrong answer with two readings
- **WHEN** the accepted answer is `こんばんは` and the answer is `konnichiwa`
- **THEN** the answer is wrong, read as `こんにちわ`

#### Scenario: Correct through the keyboard reading
- **WHEN** the accepted answer is `こんや` and the answer is `konnya`
- **THEN** the answer is correct, read as `こんや`

### Requirement: Katakana folding
Before comparing, every katakana that has a hiragana counterpart SHALL be turned into that hiragana, and a kana followed by a combining voiced or semi-voiced sound mark SHALL be read as the single voiced or semi-voiced kana. Kanji, `ー` and other characters stay as they are.

#### Scenario: Katakana words
- **WHEN** `タカイ`, `コーヒー`, `ヴ` and `ッ` are folded
- **THEN** the results are `たかい`, `こーひー`, `ゔ` and `っ`

#### Scenario: Combining sound marks
- **WHEN** the answer is `か` followed by U+3099 and `い`, or `ハ` followed by U+309A, and the accepted answers are `がい` and `ぱ`
- **THEN** both answers are correct
