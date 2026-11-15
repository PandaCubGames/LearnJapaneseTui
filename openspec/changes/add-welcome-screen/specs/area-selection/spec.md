# Spec Delta

## Purpose

The area selection screen is where the user chooses what to practise. Until the real list of areas is built, it is a placeholder that says so and can only be left by quitting.

## ADDED Requirements

### Requirement: Area selection placeholder content
Until the list of areas exists, the area selection screen SHALL show the line `Area selection is coming soon.` and the quit hint `Press Ctrl+C to quit`. It MUST NOT show the Welcome screen's greeting, start hint or debug lines.

#### Scenario: Placeholder shown
- **WHEN** the user presses Enter on the Welcome screen in a terminal of 80 columns by 24 rows
- **THEN** the screen shows `Area selection is coming soon.` and `Press Ctrl+C to quit`
- **AND** it does not show `日本語を勉強しましょう！`, `Press Enter to start` or `Last key:`

### Requirement: Leaving the area selection placeholder
On the area selection placeholder, `Ctrl+C` SHALL quit the app. Every other key, including `q`, Enter, Esc, the arrow keys and typed characters, MUST do nothing: the screen stays and there is no way back to the Welcome screen.

#### Scenario: Ctrl+C quits
- **WHEN** the area selection placeholder is shown and the user presses `Ctrl+C`
- **THEN** the app quits

#### Scenario: Other keys do nothing
- **WHEN** the area selection placeholder is shown and the user presses `q`, `a`, `あ`, Enter, Esc or Up
- **THEN** the area selection placeholder stays and the app keeps running

### Requirement: Area selection placeholder layout
The area selection placeholder SHALL be centred by terminal display width, follow terminal resizes, draw without panicking at any size down to one column by one row, and MUST NOT use colour.

#### Scenario: Resized
- **WHEN** the terminal is resized to 100x30 while the area selection placeholder is shown
- **THEN** the placeholder is drawn for 100x30, with `Press Ctrl+C to quit` (20 columns) starting at column 40

#### Scenario: Any size
- **WHEN** the area selection placeholder is drawn at 1x1, 5x2, 20x3 and 200x60
- **THEN** drawing finishes without a panic

#### Scenario: No colours used
- **WHEN** the area selection placeholder is drawn in a terminal of 80 columns by 24 rows
- **THEN** every cell has the terminal's default foreground and background colour
