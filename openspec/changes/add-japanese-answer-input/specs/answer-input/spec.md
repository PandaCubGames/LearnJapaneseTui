# Spec Delta

## Purpose

The answer field is where the user types an answer: in romaji, or in Japanese through their own IME. It keeps the text exactly as typed, converts nothing while typing, and is drawn by terminal display width so kana and kanji line up.

## ADDED Requirements

### Requirement: Typed text is kept as typed
The answer field SHALL append every typed character to its text unchanged, whether it is a Latin letter, kana, kanji or any other UTF-8 character, including each character of text committed by an IME or pasted. It MUST NOT convert romaji or anything else while typing. Control characters MUST be ignored.

#### Scenario: Romaji stays romaji
- **WHEN** `t`, `a`, `k`, `a`, `i` are typed into an empty field
- **THEN** the field's text is `takai`

#### Scenario: Committed IME text
- **WHEN** the characters of `高い` and then of `タカイ` arrive one after the other in an empty field
- **THEN** the field's text is `高いタカイ`

#### Scenario: Control characters ignored
- **WHEN** a tab or a newline character arrives as typed text
- **THEN** the field's text does not change

### Requirement: Backspace removes one character
Backspace SHALL remove the last user-perceived character (grapheme cluster) of the text, never part of one and never a single byte. On an empty field it MUST do nothing.

#### Scenario: Kanji removed whole
- **WHEN** the field holds `食べる` and Backspace is pressed twice
- **THEN** the field holds `食`

#### Scenario: Kana with a combining mark
- **WHEN** the field holds `た`, `か` and the combining voiced sound mark U+3099, and Backspace is pressed once
- **THEN** the field holds `た`

#### Scenario: Empty field
- **WHEN** the field is empty and Backspace is pressed
- **THEN** the field stays empty

### Requirement: Keys that edit the field
A character key with no modifier or only Shift SHALL type that character, including when the key is held and repeats. Backspace with no modifier or only Shift, and `Ctrl+H`, SHALL delete. Key releases and every other key or event, such as Enter, Esc, arrows, Tab, Delete, `Ctrl` or `Alt` with a letter, paste, focus and resize events, MUST NOT edit the field.

#### Scenario: Typing keys
- **WHEN** the keys `a`, Shift+`A` and `あ` are pressed, and `a` is held so it repeats
- **THEN** each types its character

#### Scenario: Deleting keys
- **WHEN** Backspace, Shift+Backspace or `Ctrl+H` is pressed, or Backspace is held so it repeats
- **THEN** each deletes

#### Scenario: Keys that do not edit
- **WHEN** Enter, Esc, Up, Left, Tab, Delete, `Ctrl+A`, `Alt+A`, the release of `a`, a paste event, a focus event or a resize event arrives
- **THEN** none of them edits the field

### Requirement: Field drawn by display width
The field SHALL be drawn on the first row of its area, from the left, with no colour, each kana and kanji taking two columns. The cursor SHALL be placed in the column right after the last character drawn.

#### Scenario: Kana take two columns
- **WHEN** a field holding `たかい` is drawn in an area 10 columns wide
- **THEN** `た`, `か` and `い` start at columns 0, 2 and 4, and the cursor is at column 6

#### Scenario: Empty field
- **WHEN** an empty field is drawn
- **THEN** the cursor is at the area's first column

#### Scenario: No colour
- **WHEN** a field holding `takaiたかい` is drawn
- **THEN** every cell has the terminal's default foreground and background colour

### Requirement: Long answers keep their end visible
When the text does not fit in the area's width minus one column (kept for the cursor), the field SHALL show the longest end of the text that fits, made of whole characters only. A wide character that would only half fit MUST NOT be drawn.

#### Scenario: Latin and kana scrolled
- **WHEN** a field holding `abcdefたか` is drawn in an area 6 columns wide
- **THEN** columns 0 to 4 show `f`, `た` and `か`, and the cursor is at column 5

#### Scenario: Wide character not cut in half
- **WHEN** a field holding `あいう` is drawn in an area 4 columns wide
- **THEN** only `う` is drawn, at column 0, and the cursor is at column 2

### Requirement: Field at any size
Drawing the field MUST NOT panic at any area size. An area with no columns or no rows SHALL draw nothing and give no cursor position.

#### Scenario: Tiny areas
- **WHEN** a field holding `食べる` is drawn in areas of 0x0, 0x1, 1x0, 1x1, 2x1 and 3x1 columns by rows
- **THEN** drawing does not panic; 0x0, 0x1 and 1x0 give no cursor position; 1x1 and 2x1 draw no text and put the cursor at column 0; 3x1 draws `る` at column 0 and puts the cursor at column 2
