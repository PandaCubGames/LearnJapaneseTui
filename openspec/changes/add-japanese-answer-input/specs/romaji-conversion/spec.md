# Spec Delta

## Purpose

Romaji conversion turns an answer typed in Latin letters into hiragana, letter for letter, so it can be compared with Japanese answers. Most text has one reading; `nn` before a vowel has two, and the conversion returns every reading. It never guesses: text that is not romaji is reported as unreadable.

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
`n` SHALL give `ん` when it is followed by a consonant other than `n` and `y`, by `-`, or by nothing. `n'` SHALL always give `ん`, with a single reading. `nn` SHALL give `ん`, consuming both letters, with a single reading, except when it is directly followed by a vowel or by `ya`, `yu` or `yo` (see *Two readings of nn*). `n` followed by a vowel or by `y` SHALL start a syllable of the `n` row (`na`, `nya`, ...). A doubled `n` MUST NOT give `っ`.

#### Scenario: n before a consonant and at the end
- **WHEN** `shinbun`, `hon` and `kantan` are converted
- **THEN** the results are `しんぶん`, `ほん` and `かんたん`, each as the only reading

#### Scenario: n before a vowel starts a syllable
- **WHEN** `kinen` is converted
- **THEN** the only reading is `きねん`

#### Scenario: n' before a vowel or y
- **WHEN** `kin'en` and `kon'ya` are converted
- **THEN** the only readings are `きんえん` and `こんや`, while `konya` gives only `こにゃ`

#### Scenario: nn at the end or before a consonant
- **WHEN** `nn`, `honn` and `onnna` are converted
- **THEN** the only readings are `ん`, `ほん` and `おんな`

#### Scenario: Three n before a vowel
- **WHEN** `konnnichiha` is converted
- **THEN** the only reading is `こんにちは`, because the first two `n` are read together as `ん` and the third starts `に`

### Requirement: Two readings of nn
Where `nn` is directly followed by a vowel or by `ya`, `yu` or `yo`, the conversion SHALL give two readings of that spot: the textbook reading, `ん` plus the `n`-row syllable that starts with the second `n`, and the keyboard reading, `ん` plus the syllable of the vowel or `ya`/`yu`/`yo`. The conversion SHALL return every reading of the text.

#### Scenario: konnichiha
- **WHEN** `konnichiha` is converted
- **THEN** the readings are `こんにちは` and then `こんいちは`

#### Scenario: onna
- **WHEN** `onna` is converted
- **THEN** the readings are `おんな` and then `おんあ`

#### Scenario: kinnen
- **WHEN** `kinnen` is converted
- **THEN** the readings are `きんねん` and then `きんえん`

#### Scenario: nn before y
- **WHEN** `konnya` is converted
- **THEN** the readings are `こんにゃ` and then `こんや`

### Requirement: Readings are ordered and bounded
The readings SHALL be all different and in a fixed order: by the reading chosen at each `nn` spot, the first spot deciding first, the textbook reading before the keyboard reading. So the first reading is the textbook reading of every spot. At most four spots SHALL give two readings, so a text has at most 16 readings; from the fifth spot on, only the textbook reading SHALL be used.

#### Scenario: Two spots
- **WHEN** `kinnennen` is converted
- **THEN** the readings are `きんねんねん`, `きんねんえん`, `きんえんねん` and `きんえんえん`, in that order

#### Scenario: More spots than the bound
- **WHEN** `nna` repeated five times is converted
- **THEN** there are 16 readings, the first is `んなんなんなんなんな` and the last is `んあんあんあんあんな`

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

#### Scenario: Capitals with two readings
- **WHEN** `KONNICHIHA` is converted
- **THEN** the readings are those of `konnichiha`: `こんにちは` and then `こんいちは`

### Requirement: Unreadable romaji is reported
When the text has no valid reading, because some part of it cannot be read with the table and the rules above, the conversion SHALL report that the text cannot be converted, and MUST NOT return a partial or guessed result. Both readings of an `nn` spot consume the same letters, so a spot never makes a text readable or unreadable.

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

#### Scenario: Unreadable text with an nn spot
- **WHEN** `konnichiq` and `konnyk` are converted
- **THEN** each is reported as not convertible

#### Scenario: m is not a syllabic n
- **WHEN** `shimbun` is converted
- **THEN** it is reported as not convertible

#### Scenario: Empty text
- **WHEN** the empty text is converted
- **THEN** the only reading is the empty text
