# Spec Delta

## Purpose

The level picker is where the user chooses which JLPT levels to practise, after choosing an area and before the first question. One picker serves every area, and a second screen says when the chosen levels have no questions.

## ADDED Requirements

### Requirement: Level picker shown after choosing an area with questions
On area selection, Enter on an area that has questions SHALL show the level picker. The picker SHALL show the area's name, the heading `Which JLPT levels do you want to practise?`, one line per level in the order N5, N4, N3, N2, N1, and the hint `Up/Down to move, Space to tick, Enter to start, Esc to go back`.

#### Scenario: Picker after choosing the Sample area
- **WHEN** the Sample area is highlighted on area selection and the user presses Enter
- **THEN** the screen shows `Sample`, `Which JLPT levels do you want to practise?`, the lines for `N5`, `N4`, `N3`, `N2` and `N1` from top to bottom, and the hint

#### Scenario: An empty area still says so
- **WHEN** an area without questions is chosen
- **THEN** the "No questions in this area yet" screen is shown, not the level picker

### Requirement: Tick boxes and highlight
Each level line SHALL show a tick box: `[x] ` when the level is ticked and `[ ] ` when it is not. The highlighted line SHALL start with `> ` and every other level line with two spaces. The first line (N5) SHALL be highlighted each time the picker opens.

#### Scenario: Picker opened
- **WHEN** the picker opens for the first time since the app started
- **THEN** the lines read `> [x] N5`, `  [ ] N4`, `  [ ] N3`, `  [ ] N2` and `  [ ] N1`

### Requirement: Starting selection and memory
When the app starts, only N5 SHALL be ticked. The ticks SHALL be kept while the app runs: leaving the picker, practising, or choosing another area MUST NOT reset them.

#### Scenario: Ticks kept after going back
- **WHEN** the user ticks N4 on the picker, presses Esc, and chooses an area with questions again
- **THEN** N5 and N4 are ticked and N5 is highlighted

### Requirement: Moving the highlight on the picker
Down without modifiers SHALL move the highlight to the next level and Up without modifiers to the previous one. The highlight MUST stop at N5 and at N1 and does not wrap around. A held arrow SHALL keep moving.

#### Scenario: Down and Up
- **WHEN** N5 is highlighted and the user presses Down twice and Up once
- **THEN** N4 is highlighted

#### Scenario: Stops at both ends
- **WHEN** the user presses Up on N5, or presses Down five times from N5
- **THEN** N5, respectively N1, is highlighted

### Requirement: Ticking a level
Space without modifiers SHALL tick the highlighted level when it is not ticked and untick it when it is. A held Space reported as a repeat MUST do nothing.

#### Scenario: Tick and untick
- **WHEN** N4 is highlighted and the user presses Space, then Space again
- **THEN** N4 is ticked after the first press and not ticked after the second

#### Scenario: Unticking the last level
- **WHEN** only N5 is ticked and the user presses Space on N5
- **THEN** no level is ticked

### Requirement: Nothing ticked
While no level is ticked, the picker SHALL show `Tick at least one level` below the hint, and Enter MUST do nothing. The line SHALL disappear as soon as a level is ticked.

#### Scenario: Enter with nothing ticked
- **WHEN** no level is ticked and the user presses Enter
- **THEN** the picker stays and shows `Tick at least one level`

### Requirement: Starting practice
Enter without modifiers, not a repeat, SHALL start practice with the ticked levels when the area has at least one entry with any of them. When it has none, the screen `No questions at these levels yet` SHALL be shown, with the area's name and the hint `Press Enter to choose other levels`.

#### Scenario: Levels with questions
- **WHEN** N5 is ticked on the Sample area's picker and the user presses Enter
- **THEN** a question screen is shown

#### Scenario: Levels without questions
- **WHEN** only N2 and N1 are ticked on the Sample area's picker and the user presses Enter
- **THEN** the screen shows `Sample`, `No questions at these levels yet` and `Press Enter to choose other levels`

### Requirement: Leaving the picker screens
On the level picker, Esc without modifiers SHALL go back to area selection with the same area highlighted. On the "no questions at these levels" screen, Enter SHALL go back to the picker with the same ticks, and Esc SHALL go back to area selection. On both screens, every other key except `Ctrl+C` MUST do nothing, and `Ctrl+C` SHALL quit.

#### Scenario: Esc on the picker
- **WHEN** the picker is shown and the user presses Esc
- **THEN** area selection is shown with the same area highlighted

#### Scenario: Enter on no questions at these levels
- **WHEN** the "no questions at these levels" screen is shown and the user presses Enter
- **THEN** the picker is shown with the same levels ticked

#### Scenario: Esc on no questions at these levels
- **WHEN** the "no questions at these levels" screen is shown and the user presses Esc
- **THEN** area selection is shown

#### Scenario: Other keys
- **WHEN** the picker is shown and the user presses `q`, `a`, `x`, Left, Right or Tab
- **THEN** nothing changes and the app keeps running

#### Scenario: Ctrl+C
- **WHEN** either screen is shown and the user presses `Ctrl+C`
- **THEN** the app quits

### Requirement: Level picker layout
Every line of both screens SHALL be centred horizontally by terminal display width, the level lines SHALL form one block with every marker in the same column, both screens SHALL be centred vertically, and they MUST stay centred after a resize.

#### Scenario: Level lines centred
- **WHEN** the picker is shown in a terminal 80 columns wide
- **THEN** the marker `>` of the N5 line is at column 36 (counting from 0) and every tick box `[` is at column 38

#### Scenario: Resized
- **WHEN** the terminal is resized to 100 columns while the picker is shown
- **THEN** every tick box `[` is at column 48

#### Scenario: Message centred
- **WHEN** the "no questions at these levels" screen is shown 80 columns wide
- **THEN** `No questions at these levels yet` (32 columns) starts at column 24

### Requirement: Level picker at any size and without colour
Both screens MUST draw without panicking at any terminal size down to one column by one row, and MUST NOT use colour.

#### Scenario: Sizes
- **WHEN** each screen is drawn at 1x1, 5x2, 20x3 and 200x60
- **THEN** drawing finishes without a panic

#### Scenario: No colour
- **WHEN** either screen is drawn at 80x24
- **THEN** every cell has the terminal's default foreground and background colour
