# Spec Delta

## ADDED Requirements

### Requirement: List of areas
The area selection screen SHALL show the heading `What do you want to practise?` and below it one line per area the app has registered, in the order they are registered. The screen MUST NOT depend on which areas exist: it lists whatever is registered, and registering another area SHALL add one line to the list without any change to the screen. This capability registers no area itself; each area is registered by its own change.

#### Scenario: Registered areas are listed in order
- **WHEN** the app has registered the areas `First`, `Second` and `Third`, in that order, and the user presses Enter on the Welcome screen in a terminal of 80 columns by 24 rows
- **THEN** the screen shows `What do you want to practise?` and the lines `First`, `Second` and `Third`, in this order from top to bottom
- **AND** it does not show `Area selection is coming soon.`, `Press Enter to start` or `Last key:`

#### Scenario: Another area
- **WHEN** the app has registered four areas and the area selection screen is shown
- **THEN** the screen shows four area lines, in the registered order

#### Scenario: A list with areas does not say it is empty
- **WHEN** the app has registered at least one area and the area selection screen is shown
- **THEN** the screen does not show `Nothing to practise yet`

### Requirement: Empty list of areas
When the app has registered no area, the area selection screen SHALL still be shown after the Welcome screen. It SHALL show the heading `What do you want to practise?`, the line `Nothing to practise yet` in place of the list, and the same key hint as a list with areas, in that order from top to bottom. No line is highlighted and no `> ` marker is drawn. Up, Down and Enter MUST do nothing on an empty list; Esc and `Ctrl+C` SHALL quit, as on a list with areas.

#### Scenario: Empty list shown
- **WHEN** the app has registered no area and the user presses Enter on the Welcome screen in a terminal of 80 columns by 24 rows
- **THEN** the screen shows `What do you want to practise?`, `Nothing to practise yet` and `Up/Down to move, Enter to choose, Esc to leave`, in this order from top to bottom
- **AND** it does not show `> `

#### Scenario: Empty list centred
- **WHEN** the empty list is shown in a terminal 80 columns wide
- **THEN** `Nothing to practise yet` (23 columns) starts at column 29 (counting from 0) and the hint starts at column 17
- **AND** after a resize to 100 columns by 30 rows, `Nothing to practise yet` starts at column 39 and the hint at column 27

#### Scenario: Up, Down and Enter on an empty list
- **WHEN** the empty list is shown and the user presses Up, Down and Enter
- **THEN** the area selection screen stays, no area is highlighted, the "no questions" screen is not shown and the app keeps running

#### Scenario: Esc on an empty list
- **WHEN** the empty list is shown and the user presses Esc
- **THEN** the app quits

#### Scenario: Ctrl+C on an empty list
- **WHEN** the empty list is shown and the user presses `Ctrl+C`
- **THEN** the app quits

### Requirement: Highlighted area
When the list has areas, exactly one line of the list SHALL be highlighted. When the screen is reached from the Welcome screen, the first area SHALL be highlighted. The highlighted line MUST start with the marker `> `, and every other area line MUST start with two spaces, so the highlight is visible without colour.

#### Scenario: First area highlighted on arrival
- **WHEN** the app has registered `First`, `Second` and `Third` and the user presses Enter on the Welcome screen
- **THEN** the `First` line starts with `> `
- **AND** the `Second` and `Third` lines start with two spaces

### Requirement: Moving the highlight
On the area selection screen, Down without modifiers SHALL move the highlight to the next area and Up without modifiers to the previous one. The highlight MUST stop at the first and the last area: it does not wrap around. A held arrow key SHALL keep moving the highlight.

#### Scenario: Down and Up
- **WHEN** the areas are `First`, `Second` and `Third`, `First` is highlighted and the user presses Down
- **THEN** `Second` is highlighted
- **AND** when the user then presses Up, `First` is highlighted

#### Scenario: Stops at the last area
- **WHEN** the user presses Down three times from the first of three areas
- **THEN** the third area is highlighted

#### Scenario: Stops at the first area
- **WHEN** the first area is highlighted and the user presses Up
- **THEN** the first area is still highlighted

#### Scenario: Arrow with a modifier
- **WHEN** the user presses Shift+Down or Ctrl+Down on the area selection screen
- **THEN** the highlight does not move

### Requirement: Choosing an area without questions
On the area selection screen, a press of Enter without modifiers on an area that has no questions SHALL show the "no questions" screen: the area's name, the line `No questions in this area yet` and the hint `Press Enter to go back`. Enter with a modifier, and a held Enter reported as a repeat, MUST do nothing.

#### Scenario: Enter on an empty area
- **WHEN** an area named `Only` is highlighted, it has no questions, and the user presses Enter
- **THEN** the screen shows `Only`, `No questions in this area yet` and `Press Enter to go back`
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
- **WHEN** the user chose the second of three areas, it had no questions, and the user presses Enter on the "no questions" screen
- **THEN** the area selection screen is shown with the second area highlighted

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
The area selection screen SHALL show the hint `Up/Down to move, Enter to choose, Esc to leave` below the list, or below the `Nothing to practise yet` line when the list is empty.

#### Scenario: Hint shown below the list
- **WHEN** the app has registered `First`, `Second` and `Third` and the area selection screen is shown in a terminal of 80 columns by 24 rows
- **THEN** the screen shows `Up/Down to move, Enter to choose, Esc to leave` on a row below the `Third` line

#### Scenario: Hint shown with whatever the app registers
- **WHEN** the area selection screen of the app as built is shown in a terminal of 80 columns by 24 rows
- **THEN** the screen shows `What do you want to practise?` and `Up/Down to move, Enter to choose, Esc to leave`

### Requirement: Area selection layout by display width
The heading, the hint, the `Nothing to practise yet` line and the "no questions" lines SHALL each be centred horizontally by terminal display width. The area lines SHALL form one block centred by the display width of its widest line, with every marker in the same column. Both screens SHALL be centred vertically and MUST stay centred after a resize.

#### Scenario: List centred
- **WHEN** the areas are `First`, `Second` and `Third` and the area selection screen is shown in a terminal 80 columns wide
- **THEN** the marker `>` of the `First` line is at column 36 (counting from 0), and `First`, `Second` and `Third` each start at column 38

#### Scenario: Names of different display widths
- **WHEN** the app has registered areas named `Vocabulary` (10 columns) and `漢字` (2 characters, 4 columns) and the area selection screen is shown 80 columns wide
- **THEN** both names start at column 36

#### Scenario: Hint centred
- **WHEN** the area selection screen is shown in a terminal 80 columns wide
- **THEN** the hint (46 columns) starts at column 17

#### Scenario: Resized
- **WHEN** the areas are `First`, `Second` and `Third` and the terminal is resized to 100 columns by 30 rows while the area selection screen is shown
- **THEN** the marker of the `First` line is at column 46, `First` and `Second` start at column 48, and the hint starts at column 27

#### Scenario: No questions message centred
- **WHEN** the "no questions" screen is shown in a terminal 80 columns wide
- **THEN** `No questions in this area yet` (29 columns) starts at column 26

### Requirement: Area selection at any size
The area selection screen, with areas and with an empty list, and the "no questions" screen MUST draw without panicking at any terminal size, down to one column by one row.

#### Scenario: Tiny and large terminals
- **WHEN** the area selection screen with areas, the empty list and the "no questions" screen are each drawn at 1x1, 5x2, 20x3 and 200x60
- **THEN** drawing finishes without a panic

### Requirement: Area selection without colour
The area selection screen, with areas and with an empty list, and the "no questions" screen MUST NOT use colour: no cell has a foreground or background colour set, including the highlighted line.

#### Scenario: No colours used
- **WHEN** the area selection screen with areas, the empty list or the "no questions" screen is drawn in a terminal of 80 columns by 24 rows
- **THEN** every cell has the terminal's default foreground and background colour

## REMOVED Requirements

### Requirement: Area selection placeholder content
**Reason**: The placeholder line `Area selection is coming soon.` and its quit hint are replaced by the real list of areas (issue #20).
**Migration**: See *List of areas*, *Empty list of areas*, *Highlighted area* and *Area selection key hint*.

### Requirement: Leaving the area selection placeholder
**Reason**: Area selection now has its own keys: arrows move the highlight, Enter chooses and Esc leaves (issue #20). `Ctrl+C` still quits.
**Migration**: See *Moving the highlight*, *Choosing an area without questions*, *Empty list of areas* and *Leaving area selection*.

### Requirement: Area selection placeholder layout
**Reason**: The layout rules now cover the list, the empty list and the "no questions" screen instead of the placeholder.
**Migration**: See *Area selection layout by display width*, *Area selection at any size* and *Area selection without colour*.
