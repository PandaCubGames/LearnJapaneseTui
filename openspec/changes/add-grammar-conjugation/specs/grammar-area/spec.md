# Spec Delta

## Purpose

The Grammar area is the app's conjugation practice: it lists verbs and adjectives, and asks for their plain and polite forms through the shared practice screens.

## ADDED Requirements

### Requirement: Grammar in the list of areas
The app SHALL register an area named `Grammar`. It SHALL have questions, so choosing it SHALL open the JLPT level picker, not the "no questions" screen.

#### Scenario: Listed
- **WHEN** the user presses Enter on the Welcome screen of the app
- **THEN** the area list shows a line `Grammar`

#### Scenario: Chosen
- **WHEN** the user highlights `Grammar` and presses Enter
- **THEN** the level picker is shown with the heading `Grammar`

#### Scenario: Practised
- **WHEN** the user then presses Enter on the level picker with N5 ticked
- **THEN** a question whose prompt starts with `What is the ` is shown with an answer field

### Requirement: One entry per word
Each word of the Grammar word list SHALL be one entry, carrying the word's JLPT level and nine questions: plain non-past, negative, past, past negative and te-form, and polite non-past, negative, past and past negative.

#### Scenario: Nine questions
- **WHEN** the Grammar area is asked for its entries at all five levels
- **THEN** there is one entry per word, and every entry has exactly nine questions with nine different prompts

#### Scenario: Japanese answers
- **WHEN** any Grammar question is checked
- **THEN** its kind is a Japanese answer

### Requirement: Entries at the chosen levels
The Grammar area SHALL supply exactly the words that carry one of the chosen levels.

#### Scenario: Only N4
- **WHEN** the Grammar area is asked for its entries at N4 only
- **THEN** every entry carries N4, and there is at least one

#### Scenario: Levels without words
- **WHEN** the Grammar area is asked for its entries at N2 and N1
- **THEN** it supplies no entry

### Requirement: The prompt
A prompt SHALL read `What is the <form> of <word>?`. `<form>` is one of `plain non-past`, `plain negative`, `plain past`, `plain past negative`, `te-form`, `polite non-past`, `polite negative`, `polite past`, `polite past negative`. `<word>` is the written form, then in brackets the reading and the meaning (`食べる (たべる, to eat)`); a word in kana shows only its meaning (`する (to do)`). Text that is not Japanese SHALL be plain ASCII.

#### Scenario: A word with kanji
- **WHEN** the polite past negative of 食べる is asked
- **THEN** the prompt is `What is the polite past negative of 食べる (たべる, to eat)?`

#### Scenario: A word in kana
- **WHEN** the plain negative of する is asked
- **THEN** the prompt is `What is the plain negative of する (to do)?`

#### Scenario: Te-form
- **WHEN** the te-form of 行く is asked
- **THEN** the prompt is `What is the te-form of 行く (いく, to go)?`

#### Scenario: Fits one line
- **WHEN** any Grammar prompt is measured
- **THEN** it is at most 80 terminal columns wide

### Requirement: Shown text
A Grammar question SHALL list as its shown text the word as written and, when it differs, its reading. So an answer that copies a written form with kanji back is wrong, and typing the reading is not.

#### Scenario: Plain non-past of a word with kanji
- **WHEN** the plain non-past of 食べる is answered with `食べる`
- **THEN** the answer is wrong
- **AND** `たべる` and `taberu` are correct

#### Scenario: Plain non-past of a word in kana
- **WHEN** the plain non-past of する is answered with `する` or `suru`
- **THEN** the answer is correct

### Requirement: Accepted answers
A Grammar question SHALL accept every answer the conjugation rules give, each in its written form and in its reading, in the rules' order (written form, then reading, for each answer), with no duplicates and no romaji. So each answer can be typed with kanji, in kana, or in romaji.

#### Scenario: Polite past of 食べる
- **WHEN** the polite past of 食べる is asked
- **THEN** the accepted answers are `食べました`, `たべました`, in this order
- **AND** `tabemashita`, `tabemasita`, `タベマシタ` and `食べました` are correct

#### Scenario: Polite negative of 高い
- **WHEN** the polite negative of 高い is asked
- **THEN** the accepted answers are `高くないです`, `たかくないです`, `高くありません`, `たかくありません`, in this order

#### Scenario: A word in kana
- **WHEN** the polite non-past of する is asked
- **THEN** the only accepted answer is `します`

#### Scenario: では typed in romaji
- **WHEN** the polite negative of 静か is answered with `shizukadehaarimasen`, `shizukadewaarimasen`, `shizukajaarimasen` or `shizukajanaidesu`
- **THEN** each is correct
- **AND** no accepted answer of any Grammar question contains でわ

#### Scenario: The plain negative of 静か typed six ways
- **WHEN** the plain negative of 静か is answered
- **THEN** `shizukadehanai` is correct, read as `しずかではない`
- **AND** `shizukadewanai` is correct, read as `しずかではない`
- **AND** `しずかではない` is correct, read as `しずかではない`
- **AND** `静かではない` is correct, read as `静かではない`
- **AND** `しずかでわない` is wrong, read as `しずかでわない`
- **AND** `shizukajanai` is correct, read as `しずかじゃない`

#### Scenario: dewa in a polite past negative
- **WHEN** the polite past negative of 静か is answered with `shizukadewaarimasendeshita` and with `shizukadewanakattadesu`
- **THEN** both are correct, read as `しずかではありませんでした` and `しずかではなかったです`

#### Scenario: いい accepts よい and 良
- **WHEN** the plain non-past of いい is asked
- **THEN** the prompt is `What is the plain non-past of いい (good)?` and the shown text is `いい` only
- **AND** the accepted answers are `いい`, `よい`, `良い`, in this order
- **AND** `ii`, `yoi`, `いい`, `よい`, `イイ` and `良い` are correct

#### Scenario: いい in the polite non-past and the past
- **WHEN** the polite non-past and the plain past of いい are asked
- **THEN** the accepted answers are `いいです`, `よいです`, `良いです`, and `よかった`, `良かった`
- **AND** `yoidesu`, `良いです`, `yokatta` and `良かった` are correct, while `iikatta` is wrong

#### Scenario: 来る read with the right sound
- **WHEN** the plain negative of 来る is answered with `konai` and with `kinai`
- **THEN** `konai` is correct and `kinai` is wrong

### Requirement: The starting word list
The word list SHALL contain at least one ichidan verb, at least one godan verb for each of the endings う く ぐ す つ ぬ ぶ む る, the irregular verbs する, a noun plus する, 来る, 行く and ある, at least one i-adjective, いい, and at least one na-adjective. It SHALL use more than one JLPT level.

#### Scenario: Coverage
- **WHEN** the word list is checked
- **THEN** every group and every godan ending above has at least one word, and at least two different levels are used

### Requirement: Word list entries are valid
Every word SHALL have a reading in hiragana only, a written form that contains a kanji whenever it differs from the reading, a non-empty English meaning in plain ASCII, and a group whose rules give an answer for all nine asked forms. No two words SHALL have the same written form. Every ichidan verb's reading SHALL end in an i-row or e-row kana followed by る.

#### Scenario: The whole list
- **WHEN** every word of the list is checked
- **THEN** each one meets every rule above
