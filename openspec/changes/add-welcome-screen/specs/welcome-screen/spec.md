# Spec Delta

## Purpose

The Welcome screen is the first screen the app shows: it greets the user, says how to start and how to quit, and leads into the rest of the app.

## ADDED Requirements

### Requirement: Welcome screen lines
The Welcome screen SHALL show these lines, top to bottom: the app name and version, the greeting `日本語を勉強しましょう！`, the hint `Press Enter to start`, the terminal size with the elapsed time, the last character typed, and the quit hint `Press q or Ctrl+C to quit`.

#### Scenario: Fresh start
- **WHEN** the Welcome screen is shown in a terminal of 80 columns by 24 rows
- **THEN** it shows the app name and version, the greeting, `Press Enter to start`, `Size: 80x24`, `Elapsed: 0.0 s`, `Last key: -` and `Press q or Ctrl+C to quit`

#### Scenario: Order of the lines
- **WHEN** the Welcome screen is shown in a terminal of 80 columns by 24 rows
- **THEN** the title is above the greeting, the greeting is above the start hint, the start hint is above the size line, the size line is above the last-key line and the last-key line is above the quit hint

### Requirement: Enter starts the app
On the Welcome screen, pressing Enter without modifiers SHALL show the area selection screen. The app MUST keep running.

#### Scenario: Enter on the Welcome screen
- **WHEN** the Welcome screen is shown and the user presses Enter
- **THEN** the area selection screen is shown
- **AND** the app keeps running

#### Scenario: Enter released
- **WHEN** the terminal reports only the release of the Enter key on the Welcome screen
- **THEN** the Welcome screen stays

#### Scenario: Enter with a modifier
- **WHEN** the Welcome screen is shown and the user presses Ctrl+Enter
- **THEN** the Welcome screen stays

### Requirement: Quitting from the Welcome screen
On the Welcome screen, `q` without modifiers and `Ctrl+C` SHALL quit the app. Esc and the arrow keys MUST do nothing.

#### Scenario: q quits
- **WHEN** the Welcome screen is shown and the user presses `q`
- **THEN** the app quits

#### Scenario: Ctrl+C quits
- **WHEN** the Welcome screen is shown and the user presses `Ctrl+C`
- **THEN** the app quits

#### Scenario: Esc does nothing
- **WHEN** the Welcome screen is shown and the user presses Esc or Up
- **THEN** the Welcome screen stays and the app keeps running

### Requirement: Welcome screen centred by display width
Every line of the Welcome screen SHALL be centred horizontally by terminal display width, where kana and kanji take two columns, and the block of lines SHALL be centred vertically. It MUST stay centred after a resize.

#### Scenario: Start hint centred
- **WHEN** the Welcome screen is shown in a terminal 80 columns wide
- **THEN** `Press Enter to start` (20 columns) starts at column 30, counting from 0

#### Scenario: Greeting centred after a resize
- **WHEN** the terminal is resized from 80x24 to 100x30 while the Welcome screen is shown
- **THEN** the greeting (24 columns) starts at column 38 and `Press Enter to start` starts at column 40

### Requirement: Welcome screen at any size
The Welcome screen MUST draw without panicking at any terminal size, including sizes too small for its content, down to one column by one row.

#### Scenario: Tiny and large terminals
- **WHEN** the Welcome screen is drawn at 1x1, 5x2, 20x3 and 200x60
- **THEN** drawing finishes without a panic

### Requirement: Welcome screen without colour
The Welcome screen MUST NOT rely on colour: no text has a foreground or background colour set, so it reads the same in a terminal with few or no colours.

#### Scenario: No colours used
- **WHEN** the Welcome screen is drawn in a terminal of 80 columns by 24 rows
- **THEN** every cell has the terminal's default foreground and background colour
