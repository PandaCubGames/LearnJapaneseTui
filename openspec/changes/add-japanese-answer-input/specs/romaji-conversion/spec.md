# Spec Delta

## Purpose

Romaji conversion turns an answer typed in Latin letters into hiragana, letter for letter, so it can be compared with Japanese answers. It never guesses: text that is not romaji is reported as unreadable.

## ADDED Requirements

### Requirement: Basic syllables
The conversion SHALL turn every syllable of the romaji table in `design.md` (D4) into its hiragana, including voiced, semi-voiced and small-kana combinations. It MUST have no state and no terminal access: the same text always gives the same result.

#### Scenario: Plain syllables
- **WHEN** `takai`, `taberu`, `sakana`, `gakkou` and `hiragana` are converted
- **THEN** the results are `たかい`, `たべる`, `さかな`, `がっこう` and `ひらがな`

#### Scenario: Combinations with small kana
- **WHEN** `kyou`, `ryokou`, `byouin`, `nyuugaku` and `myaku` are converted
- **THEN** the results are `きょう`, `りょこう`, `びょういん`, `にゅうがく` and `みゃく`

### Requirement: Typed variants give the same kana
The common typed variants listed in `design.md` (D4) SHALL give the same hiragana as the Hepburn spelling.

#### Scenario: Hepburn and its variants
- **WHEN** each pair `shi`/`si`, `chi`/`ti`, `tsu`/`tu`, `fu`/`hu`, `ji`/`zi`, `sha`/`sya`, `cha`/`tya` is converted
- **THEN** both spellings of each pair give the same hiragana: `し`, `ち`, `つ`, `ふ`, `じ`, `しゃ`, `ちゃ`

#### Scenario: Three spellings of ja
- **WHEN** `ja`, `jya` and `zya` are converted
- **THEN** all three give `じゃ`

#### Scenario: Kana that are easy to confuse
- **WHEN** `wo`, `o`, `di`, `du`, `dzu`, `ji` and `ha` are converted
- **THEN** the results are `を`, `お`, `ぢ`, `づ`, `づ`, `じ` and `は`

### Requirement: Doubled consonant gives small tsu
A consonant letter other than `n` followed by the same letter SHALL give `っ` followed by the syllable that starts with the second letter. `t` followed by `ch` SHALL also give `っ`.

#### Scenario: Doubled consonants
- **WHEN** `kitte`, `zasshi`, `ippai` and `motto` are converted
- **THEN** the results are `きって`, `ざっし`, `いっぱい` and `もっと`

#### Scenario: Small tsu before ch
- **WHEN** `matcha` and `maccha` are converted
- **THEN** both give `まっちゃ`

### Requirement: Syllabic n
`n` SHALL give `ん` when it is followed by a consonant other than `n` and `y`, by `-`, or by nothing. `nn` and `n'` SHALL always give `ん`. `n` followed by a vowel or by `y` SHALL start a syllable of the `n` row (`na`, `nya`, ...).

#### Scenario: n before a consonant and at the end
- **WHEN** `shinbun`, `hon` and `kantan` are converted
- **THEN** the results are `しんぶん`, `ほん` and `かんたん`

#### Scenario: n before a vowel starts a syllable
- **WHEN** `kinen` is converted
- **THEN** the result is `きねん`

#### Scenario: nn and n' before a vowel
- **WHEN** `kin'en` and `kinnen` are converted
- **THEN** both give `きんえん`

#### Scenario: n' and nn before y
- **WHEN** `kon'ya` and `konnya` are converted
- **THEN** both give `こんや`, while `konya` gives `こにゃ`

#### Scenario: nn always closes a syllabic n
- **WHEN** `konnichiha` and `konnnichiha` are converted
- **THEN** the results are `こんいちは` and `こんにちは`

### Requirement: Long vowel mark
A hyphen-minus `-` SHALL give the long vowel mark `ー`.

#### Scenario: Hyphen as long vowel mark
- **WHEN** `ko-hi-` and `ra-men` are converted
- **THEN** the results are `こーひー` and `らーめん`

### Requirement: Capital letters
Capital letters SHALL be converted as their lower-case letters.

#### Scenario: Upper and mixed case
- **WHEN** `TAKAI`, `Takai` and `KIN'EN` are converted
- **THEN** the results are `たかい`, `たかい` and `きんえん`

### Requirement: Unreadable romaji is reported
When any part of the text cannot be read with the table and the rules above, the conversion SHALL report that the text cannot be converted, and MUST NOT return a partial or guessed result.

#### Scenario: Letters that spell no kana
- **WHEN** `tabxru`, `ca`, `qa` and `wu` are converted
- **THEN** each is reported as not convertible

#### Scenario: Trailing lone consonant
- **WHEN** `tabek`, `kk` and `ky` are converted
- **THEN** each is reported as not convertible

#### Scenario: Apostrophe in the wrong place
- **WHEN** `ta'kai`, `'takai` and `kan''i` are converted
- **THEN** each is reported as not convertible

#### Scenario: Characters that are not romaji
- **WHEN** `taka1`, `taka i`, `taka.` and `tōkyō` are converted
- **THEN** each is reported as not convertible

#### Scenario: m is not a syllabic n
- **WHEN** `shimbun` is converted
- **THEN** it is reported as not convertible

#### Scenario: Empty text
- **WHEN** the empty text is converted
- **THEN** the result is the empty text
