# Spec Delta

## Purpose

The app shell owns the terminal for the lifetime of the app: it takes over the screen at start-up, runs the loop that every screen lives inside, and gives the terminal back in a usable state however the app ends.

## ADDED Requirements

### Requirement: Full-screen start-up
The app SHALL take over the terminal at start-up by entering raw mode and the alternate screen, and SHALL show the Welcome screen as its first screen.

#### Scenario: Starting in an interactive terminal
- **WHEN** the user starts the app in an interactive terminal
- **THEN** the app fills the whole terminal with the Welcome screen
- **AND** the text that was in the terminal before start-up is no longer visible

#### Scenario: Starting without an interactive terminal
- **WHEN** the app is started and the terminal cannot be set up (for example, there is no terminal at all, as in a CI job)
- **THEN** the app prints an error message to standard error
- **AND** exits with a non-zero status

### Requirement: Terminal restored on every exit
The app SHALL leave raw mode and the alternate screen, and show the cursor again, whenever it ends: on a normal quit, on an error and on a panic.

#### Scenario: Normal quit
- **WHEN** the user quits the app
- **THEN** the terminal shows the content it had before start-up
- **AND** typed characters are echoed and line editing works again
- **AND** the app exits with status zero

#### Scenario: Error while running
- **WHEN** reading an event or drawing fails while the app is running
- **THEN** the terminal is restored first
- **AND** the error message is then printed to standard error, readable on the normal screen
- **AND** the app exits with a non-zero status

#### Scenario: Panic while running
- **WHEN** the app panics while the TUI is active, in a debug or a release build
- **THEN** the terminal is restored first
- **AND** the panic message is then printed, readable on the normal screen

### Requirement: Quitting
The app SHALL quit when the user presses `Ctrl+C` on any screen, and when the user presses `q` on the Welcome screen.

#### Scenario: Ctrl+C
- **WHEN** the user presses `Ctrl+C` on any screen
- **THEN** the app quits

#### Scenario: q on the Welcome screen
- **WHEN** the Welcome screen is shown and the user presses `q` without modifiers
- **THEN** the app quits

#### Scenario: Other keys do not quit
- **WHEN** the Welcome screen is shown and the user presses any key other than `q` and `Ctrl+C`
- **THEN** the app keeps running

### Requirement: Key presses
The app SHALL act once per key press. It MUST ignore key-release events, which some terminals report in addition to presses.

#### Scenario: Press and release reported
- **WHEN** a terminal reports both the press and the release of one key
- **THEN** the app acts on the key exactly once

### Requirement: Japanese input arrives as text
The app SHALL accept any Unicode character the terminal delivers, including kana and kanji committed by the user's IME, as ordinary typed text.

#### Scenario: Kana from an IME
- **WHEN** the user commits `あ` through an IME on the Welcome screen
- **THEN** the Welcome screen shows `あ` as the last character typed

#### Scenario: ASCII character
- **WHEN** the user types `a` on the Welcome screen
- **THEN** the Welcome screen shows `a` as the last character typed

### Requirement: Resize handling
The app SHALL redraw the current screen to fit the new size whenever the terminal is resized, and MUST NOT crash at any terminal size.

#### Scenario: Terminal resized
- **WHEN** the user resizes the terminal while the Welcome screen is shown
- **THEN** the Welcome screen is redrawn for the new size
- **AND** the size shown on the Welcome screen is the new size

#### Scenario: Terminal too small for the content
- **WHEN** the terminal is resized to be smaller than the Welcome screen's content, down to one column by one row
- **THEN** the app keeps running and draws what fits

### Requirement: Fixed tick
The app SHALL update itself at a fixed interval of 100 ms even when no input arrives, so a screen can change on its own.

#### Scenario: No input
- **WHEN** the Welcome screen is shown and the user presses nothing for two seconds
- **THEN** the elapsed time shown on the Welcome screen has advanced by about two seconds

#### Scenario: Continuous input
- **WHEN** keys arrive faster than the tick interval for two seconds
- **THEN** the elapsed time shown on the Welcome screen still advances by about two seconds

### Requirement: Welcome screen content
The Welcome screen SHALL show the app name and version, a Japanese greeting, the terminal size, the elapsed time, the last character typed and how to quit. Each line SHALL be centred horizontally.

#### Scenario: Content shown
- **WHEN** the Welcome screen is shown in a terminal of 80 columns by 24 rows
- **THEN** it shows the app name and version, the greeting `日本語を勉強しましょう！`, the size `80x24`, the elapsed time, the last character typed and the quit keys

### Requirement: Layout by display width
The app SHALL lay out text by terminal display width, where kana and kanji take two columns each, not by byte count or character count.

#### Scenario: Centring Japanese text
- **WHEN** the greeting `日本語を勉強しましょう！` (12 characters, 24 columns) is centred in a terminal 80 columns wide
- **THEN** it starts at column 28 (counting from 0) and ends at column 51, with 28 empty columns on each side

### Requirement: Standard terminal features only
The app shell MUST NOT require mouse support, image protocols, a clipboard or a local GUI, and its screens MUST stay readable in a terminal without colour.

#### Scenario: Monochrome terminal
- **WHEN** the Welcome screen is shown in a terminal that supports no colour
- **THEN** all of its text is readable

#### Scenario: No mouse
- **WHEN** the user has no mouse, for example over SSH
- **THEN** every action of the app shell is available from the keyboard
