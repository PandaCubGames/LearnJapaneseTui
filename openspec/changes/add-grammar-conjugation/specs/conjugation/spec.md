# Spec Delta

## Purpose

Conjugation turns a word (its reading in kana, its written form and its group) into one form in one formality, by rule, so that correct answers are generated and never stored per word.

## ADDED Requirements

### Requirement: Forms and formalities
The rules SHALL produce five forms in plain speech (non-past, negative, past, past negative, te-form) and four in polite speech (non-past, negative, past, past negative). The polite te-form does not exist: asking for it SHALL give no answer, for every group.

#### Scenario: Nine forms of a verb
- **WHEN** the nine forms of the ichidan verb 食べる (たべる) are asked
- **THEN** each gives at least one answer

#### Scenario: No polite te-form
- **WHEN** the polite te-form of 食べる, 書く, する, 来る, 高い or 静か is asked
- **THEN** there is no answer

### Requirement: Written form and reading
Every answer SHALL have a written form and a reading. The rules SHALL work on the reading and on the written form separately: the kanji of the written form are kept, and only the kana ending changes. A word written only in kana has the same written form and reading.

#### Scenario: Kanji kept
- **WHEN** the plain negative of 食べる (たべる) is asked
- **THEN** the answer is written 食べない and read たべない

#### Scenario: A word in kana
- **WHEN** the polite non-past of する is asked
- **THEN** the answer is written します and read します

### Requirement: Ichidan verbs
An ichidan verb's form SHALL be its dictionary form without the final る, followed by: plain る, ない, た, なかった, て; polite ます, ません, ました, ませんでした.

#### Scenario: 食べる
- **WHEN** the nine forms of 食べる (たべる) are asked, in the order plain non-past, negative, past, past negative, te-form, polite non-past, negative, past, past negative
- **THEN** the answers are written 食べる, 食べない, 食べた, 食べなかった, 食べて, 食べます, 食べません, 食べました, 食べませんでした
- **AND** read たべる, たべない, たべた, たべなかった, たべて, たべます, たべません, たべました, たべませんでした

#### Scenario: A one-kanji stem
- **WHEN** the plain past negative and the polite past of 見る (みる) are asked
- **THEN** the answers are 見なかった (みなかった) and 見ました (みました)

### Requirement: Godan negative and polite stems
A godan verb's negative forms SHALL replace its final kana with the a-row kana (う becomes わ), followed by ない or なかった. Its polite forms SHALL replace the final kana with the i-row kana, followed by ます, ません, ました or ませんでした.

#### Scenario: Negative stems
- **WHEN** the plain negative of 買う, 書く, 泳ぐ, 話す, 待つ, 死ぬ, 遊ぶ, 飲む and 帰る is asked
- **THEN** the answers are 買わない, 書かない, 泳がない, 話さない, 待たない, 死なない, 遊ばない, 飲まない and 帰らない
- **AND** read かわない, かかない, およがない, はなさない, またない, しなない, あそばない, のまない and かえらない

#### Scenario: Polite stems
- **WHEN** the polite non-past of the same nine verbs is asked
- **THEN** the answers are 買います, 書きます, 泳ぎます, 話します, 待ちます, 死にます, 遊びます, 飲みます and 帰ります

#### Scenario: Plain past negative and polite past negative
- **WHEN** the plain past negative and the polite past negative of 死ぬ (しぬ) are asked
- **THEN** the answers are 死ななかった (しななかった) and 死にませんでした (しにませんでした)

### Requirement: Godan te-form and past
A godan verb's te-form and plain past SHALL follow its final kana: う, つ and る give って and った; く gives いて and いた; ぐ gives いで and いだ; す gives して and した; ぬ, ぶ and む give んで and んだ.

#### Scenario: Te-forms
- **WHEN** the te-form of 買う, 書く, 泳ぐ, 話す, 待つ, 死ぬ, 遊ぶ, 飲む and 帰る is asked
- **THEN** the answers are 買って, 書いて, 泳いで, 話して, 待って, 死んで, 遊んで, 飲んで and 帰って
- **AND** read かって, かいて, およいで, はなして, まって, しんで, あそんで, のんで and かえって

#### Scenario: Plain past
- **WHEN** the plain past of the same nine verbs is asked
- **THEN** the answers are 買った, 書いた, 泳いだ, 話した, 待った, 死んだ, 遊んだ, 飲んだ and 帰った

### Requirement: する and noun plus する
A verb of the する group SHALL replace its final する with: plain する, しない, した, しなかった, して; polite します, しません, しました, しませんでした. Anything before する (勉強 in 勉強する) SHALL stay.

#### Scenario: する
- **WHEN** the nine forms of する are asked in the order of the ichidan scenario
- **THEN** the answers are する, しない, した, しなかった, して, します, しません, しました, しませんでした

#### Scenario: 勉強する
- **WHEN** the plain past negative and the polite past of 勉強する (べんきょうする) are asked
- **THEN** the answers are 勉強しなかった (べんきょうしなかった) and 勉強しました (べんきょうしました)

### Requirement: 来る
The verb 来る SHALL keep the kanji 来 in every written form and change its reading: plain くる, こない, きた, こなかった, きて; polite きます, きません, きました, きませんでした.

#### Scenario: 来る
- **WHEN** the nine forms of 来る (くる) are asked in the order of the ichidan scenario
- **THEN** the answers are written 来る, 来ない, 来た, 来なかった, 来て, 来ます, 来ません, 来ました, 来ませんでした
- **AND** read くる, こない, きた, こなかった, きて, きます, きません, きました, きませんでした

### Requirement: 行く
The verb 行く SHALL conjugate as a godan verb ending in く, except that its te-form is 行って (いって) and its plain past 行った (いった).

#### Scenario: 行く
- **WHEN** the nine forms of 行く (いく) are asked in the order of the ichidan scenario
- **THEN** the answers are 行く, 行かない, 行った, 行かなかった, 行って, 行きます, 行きません, 行きました, 行きませんでした
- **AND** read いく, いかない, いった, いかなかった, いって, いきます, いきません, いきました, いきませんでした

### Requirement: ある
The verb ある SHALL give: plain ある, ない, あった, なかった, あって; polite あります, ありません, ありました, ありませんでした.

#### Scenario: ある
- **WHEN** the nine forms of ある are asked in the order of the ichidan scenario
- **THEN** the answers are ある, ない, あった, なかった, あって, あります, ありません, ありました, ありませんでした

### Requirement: I-adjectives
An i-adjective's forms SHALL be its dictionary form without the final い, followed by: plain い, くない, かった, くなかった, くて; polite いです, then くないです or くありません, then かったです, then くなかったです or くありませんでした. Where two endings are given, both SHALL be answers, in that order.

#### Scenario: 高い, plain
- **WHEN** the plain forms of 高い (たかい) are asked
- **THEN** the answers are 高い, 高くない, 高かった, 高くなかった and 高くて, read たかい, たかくない, たかかった, たかくなかった and たかくて

#### Scenario: 高い, polite
- **WHEN** the polite non-past, negative, past and past negative of 高い are asked
- **THEN** the answers are 高いです; 高くないです and 高くありません; 高かったです; 高くなかったです and 高くありませんでした
- **AND** the polite past is not 高いでした

### Requirement: いい
The i-adjective いい SHALL give, after whatever comes before its final いい: plain いい or よい, then よくない, よかった, よくなかった, よくて; polite いいです or よいです, then よくないです or よくありません, then よかったです, then よくなかったです or よくありませんでした. Every form that starts with よ SHALL also be an answer with よ written 良 (良い, 良くない, 良いです), read the same, right after its kana spelling. The いい variant SHALL come first; いい itself has no kanji spelling.

#### Scenario: いい, non-past
- **WHEN** the plain non-past and the polite non-past of いい are asked
- **THEN** the plain answers are いい (read いい), よい (read よい) and 良い (read よい), in this order
- **AND** the polite answers are いいです, よいです and 良いです (read よいです), in this order

#### Scenario: いい, other forms
- **WHEN** the plain negative, plain past, te-form and polite past negative of いい are asked
- **THEN** the answers are よくない and 良くない; よかった and 良かった; よくて and 良くて; よくなかったです, 良くなかったです, よくありませんでした and 良くありませんでした
- **AND** each 良 spelling is read like the kana spelling before it

#### Scenario: いい at the end of a longer word
- **WHEN** the plain negative of かっこいい is asked as a word of the いい group
- **THEN** the answers are かっこよくない and かっこ良くない, both read かっこよくない

### Requirement: Na-adjectives
A na-adjective is stored without だ or な. Its forms SHALL be the word followed by: plain だ, then じゃない or ではない, then だった, then じゃなかった or ではなかった, then で; polite です, then じゃないです, じゃありません, ではないです or ではありません, then でした, then じゃなかったです, じゃありませんでした, ではなかったです or ではありませんでした. Every listed variant SHALL be an answer, in that order. No answer SHALL be spelled with でわ: the romaji `dewa` is handled by answer checking, not by extra answers.

#### Scenario: 静か, plain
- **WHEN** the plain forms of 静か (しずか) are asked
- **THEN** the answers are 静かだ; 静かじゃない and 静かではない; 静かだった; 静かじゃなかった and 静かではなかった; 静かで
- **AND** the plain non-past is not the bare word 静か

#### Scenario: 静か, polite
- **WHEN** the polite negative and polite past negative of 静か are asked
- **THEN** the answers are 静かじゃないです, 静かじゃありません, 静かではないです and 静かではありません
- **AND** 静かじゃなかったです, 静かじゃありませんでした, 静かではなかったです and 静かではありませんでした

#### Scenario: A na-adjective ending in い
- **WHEN** the plain negative of the na-adjective きれい is asked
- **THEN** the answers are きれいじゃない and きれいではない, not きれくない

#### Scenario: Never でわ
- **WHEN** the nine forms of 静か (しずか) are asked
- **THEN** no answer, written or read, contains でわ

### Requirement: Words that do not fit their group
A word whose reading or written form does not end the way its group needs (an ichidan verb not ending in る, a godan verb not ending in one of う く ぐ す つ ぬ ぶ む る, an i-adjective not ending in い, a word of the いい group not ending in いい) SHALL give no answer for any form, instead of a wrong one.

#### Scenario: A misfiled word
- **WHEN** the plain negative of 静か is asked as if it were an ichidan verb
- **THEN** there is no answer

#### Scenario: A misfiled いい word
- **WHEN** the plain negative of 高い (たかい) is asked as if it were in the いい group
- **THEN** there is no answer
