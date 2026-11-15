# Spec Delta

## ADDED Requirements

### Requirement: List of areas
The area selection screen SHALL show the heading `What do you want to practise?` and below it one line per area the app knows about, in the app's order. The app knows Vocabulary, Kanji and Grammar, in that order. Adding an area to the app SHALL add one line to the list.

#### Scenario: The three areas
- **WHEN** the user presses Enter on the Welcome screen in a terminal of 80 columns by 24 rows
- **THEN** the screen shows `What do you want to practise?` and the lines `Vocabulary`, `Kanji` and `Grammar`, in this order from top to bottom
- **AND** it does not show `Area selection is coming soon.`, `Press Enter to start` or `Last key:`

#### Scenario: Another area
- **WHEN** the app knows four areas and the area selection screen is shown
- **THEN** the screen shows four area lines, in the app's order

### Requirement: Highlighted area
Exactly one line of the list SHALL be highlighted. When the screen is reached from the Welcome screen, the first area SHALL be highlighted. The highlighted line MUST start with the marker `> `, and every other area line MUST start with two spaces, so the highlight is visible without colour.

#### Scenario: First area highlighted on arrival
- **WHEN** the user presses Enter on the Welcome screen
- **THEN** the `Vocabulary` line starts with `> `
- **AND** the `Kanji` and `Grammar` lines start with two spaces

### Requirement: Moving the highlight
On the area selection screen, Down without modifiers SHALL move the highlight to the next area and Up without modifiers to the previous one. The highlight MUST stop at the first and the last area: it does not wrap around. A held arrow key SHALL keep moving the highlight.

#### Scenario: Down and Up
- **WHEN** the first area is highlighted and the user presses Down
- **THEN** `Kanji` is highlighted
- **AND** when the user then presses Up, `Vocabulary` is highlighted

#### Scenario: Stops at the last area
- **WHEN** the user presses Down three times from the first of three areas
- **THEN** `Grammar` is highlighted

#### Scenario: Stops at the first area
- **WHEN** the first area is highlighted and the user presses Up
- **THEN** `Vocabulary` is still highlighted

#### Scenario: Arrow with a modifier
- **WHEN** the user presses Shift+Down or Ctrl+Down on the area selection screen
- **THEN** the highlight does not move

### Requirement: Choosing an area without questions
On the area selection screen, a press of Enter without modifiers on an area that has no questions SHALL show the "no questions" screen: the area's name, the line `No questions in this area yet` and the hint `Press Enter to go back`. Enter with a modifier, and a held Enter reported as a repeat, MUST do nothing.

#### Scenario: Enter on an empty area
- **WHEN** `Kanji` is highlighted, it has no questions, and the user presses Enter
- **THEN** the screen shows `Kanji`, `No questions in this area yet` and `Press Enter to go back`
- **AND** it does not show `What do you want to practise?`

#### Scenario: Enter with a modifier
- **WHEN** the user presses Ctrl+Enter or Alt+Enter on the area selection screen
- **THEN** the area selection screen stays

#### Scenario: Held Enter
- **WHEN** the terminal reports Enter as a repeat on the area selection screen
- **THEN** the area selection screen stays

### Requirement: Choosing an area with questions
On the area selection screen, Enter on an area that has questions MUST NOT show the "no questions" screen and MUST NOT quit the app. What it shows is decided by the practice screens, which are not part of this capability.

#### Scenario: Enter on an area with questions
- **WHEN** the highlighted area has questions and the user presses Enter
- **THEN** the "no questions" screen is not shown
- **AND** the app keeps running

### Requirement: Going back from the no questions screen
On the "no questions" screen, Enter without modifiers (not a repeat) and Esc without modifiers SHALL go back to the area selection screen, with the same area still highlighted. Every other key except `Ctrl+C` MUST do nothing.

#### Scenario: Enter goes back
- **WHEN** the user chose `Kanji`, it had no questions, and the user presses Enter on the "no questions" screen
- **THEN** the area selection screen is shown with `Kanji` highlighted

#### Scenario: Esc goes back
- **WHEN** the "no questions" screen is shown and the user presses Esc
- **THEN** the area selection screen is shown

#### Scenario: Other keys on the no questions screen
- **WHEN** the "no questions" screen is shown and the user presses `q`, `a`, Up or Down
- **THEN** the "no questions" screen stays and the app keeps running

#### Scenario: Ctrl+C on the no questions screen
- **WHEN** the "no questions" screen is shown and the user presses `Ctrl+C`
- **THEN** the app quits

### Requirement: Leaving area selection
On the area selection screen, Esc without modifiers and `Ctrl+C` SHALL quit the app. Every other key that is not part of this screen, including `q`, letters, `j`, `k`, digits, Left and Right, MUST do nothing.

#### Scenario: Esc quits
- **WHEN** the area selection screen is shown and the user presses Esc
- **THEN** the app quits

#### Scenario: Ctrl+C quits
- **WHEN** the area selection screen is shown and the user presses `Ctrl+C`
- **THEN** the app quits

#### Scenario: Other keys do nothing
- **WHEN** the area selection screen is shown and the user presses `q`, `a`, `あ`, `j`, `k`, `1`, Left or Right
- **THEN** the area selection screen stays, the highlight does not move and the app keeps running

### Requirement: Area selection key hint
The area selection screen SHALL show the hint `Up/Down to move, Enter to choose, Esc to leave` below the list.

#### Scenario: Hint shown below the list
- **WHEN** the area selection screen is shown in a terminal of 80 columns by 24 rows
- **THEN** the screen shows `Up/Down to move, Enter to choose, Esc to leave` on a row below the `Grammar` line

### Requirement: Area selection layout by display width
The heading, the hint and the "no questions" lines SHALL each be centred horizontally by terminal display width. The area lines SHALL form one block centred by the display width of its widest line, with every marker in the same column. Both screens SHALL be centred vertically and MUST stay centred after a resize.

#### Scenario: List centred
- **WHEN** the area selection screen is shown in a terminal 80 columns wide with the three areas
- **THEN** the marker `>` of the `Vocabulary` line is at column 34 (counting from 0), and `Vocabulary`, `Kanji` and `Grammar` each start at column 36

#### Scenario: Names of different display widths
- **WHEN** the app knows the areas `Vocabulary` (10 columns) and `漢字` (2 characters, 4 columns) and the area selection screen is shown 80 columns wide
- **THEN** both names start at column 36

#### Scenario: Hint centred
- **WHEN** the area selection screen is shown in a terminal 80 columns wide
- **THEN** the hint (46 columns) starts at column 17

#### Scenario: Resized
- **WHEN** the terminal is resized to 100 columns by 30 rows while the area selection screen is shown
- **THEN** `Vocabulary` starts at column 46 and the hint starts at column 27

#### Scenario: No questions message centred
- **WHEN** the "no questions" screen is shown in a terminal 80 columns wide
- **THEN** `No questions in this area yet` (29 columns) starts at column 26

### Requirement: Area selection at any size
The area selection screen and the "no questions" screen MUST draw without panicking at any terminal size, down to one column by one row.

#### Scenario: Tiny and large terminals
- **WHEN** each of the two screens is drawn at 1x1, 5x2, 20x3 and 200x60
- **THEN** drawing finishes without a panic

### Requirement: Area selection without colour
The area selection screen and the "no questions" screen MUST NOT use colour: no cell has a foreground or background colour set, including the highlighted line.

#### Scenario: No colours used
- **WHEN** either screen is drawn in a terminal of 80 columns by 24 rows
- **THEN** every cell has the terminal's default foreground and background colour

## REMOVED Requirements

### Requirement: Area selection placeholder content
**Reason**: The placeholder line `Area selection is coming soon.` and its quit hint are replaced by the real list of areas (issue #20).
**Migration**: See *List of areas*, *Highlighted area* and *Area selection key hint*.

### Requirement: Leaving the area selection placeholder
**Reason**: Area selection now has its own keys: arrows move the highlight, Enter chooses and Esc leaves (issue #20). `Ctrl+C` still quits.
**Migration**: See *Moving the highlight*, *Choosing an area without questions* and *Leaving area selection*.

### Requirement: Area selection placeholder layout
**Reason**: The layout rules now cover the list and the "no questions" screen instead of the placeholder.
**Migration**: See *Area selection layout by display width*, *Area selection at any size* and *Area selection without colour*.
