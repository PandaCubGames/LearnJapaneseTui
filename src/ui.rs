// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 PandaCubGames

//! Draws the current screen. It only reads the state it is given, and takes
//! all wording from [`crate::text`].

use ratatui::Frame;
use ratatui::layout::{Alignment, Constraint, Flex, Layout, Rect};
use ratatui::style::Stylize;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Paragraph, Wrap};

use crate::practice::Level;
use crate::practice::question::Kind;
use crate::session::{Screen, Session};
use crate::text::{self, GREETING};

pub fn draw(frame: &mut Frame, session: &Session) {
    match session.screen() {
        Screen::Welcome => draw_welcome(frame, session),
        Screen::AreaSelection => draw_area_selection(frame, session),
        Screen::NoQuestions => draw_no_questions(frame, session),
        Screen::LevelPicker => draw_level_picker(frame, session),
        Screen::NoQuestionsAtLevels => draw_no_questions_at_levels(frame, session),
        Screen::Question => draw_question(frame, session),
    }
}

fn draw_welcome(frame: &mut Frame, session: &Session) {
    let (width, height) = session.size();

    let lines = vec![
        Line::from(text::TITLE).bold(),
        Line::default(),
        Line::from(GREETING),
        Line::default(),
        Line::from(text::START_HINT),
        Line::default(),
        Line::from(text::status(width, height, session.elapsed())),
        Line::from(text::last_key(session.last_char())),
        Line::from(text::QUIT_HINT),
    ];

    draw_centred(frame, lines);
}

fn draw_area_selection(frame: &mut Frame, session: &Session) {
    let names = session.areas().iter().map(|area| area.name());
    let widest = names
        .clone()
        .map(|name| Span::raw(name).width())
        .max()
        .unwrap_or(0);

    let mut lines = vec![Line::from(text::AREA_SELECTION_TITLE), Line::default()];
    for (index, name) in names.enumerate() {
        let marker = if index == session.highlighted() {
            text::AREA_MARKER
        } else {
            text::AREA_NO_MARKER
        };
        let padding = " ".repeat(widest.saturating_sub(Span::raw(name).width()));
        lines.push(Line::from(vec![
            Span::raw(marker),
            Span::raw(name),
            Span::raw(padding),
        ]));
    }
    lines.push(Line::default());
    lines.push(Line::from(text::AREA_SELECTION_HINT));

    draw_centred(frame, lines);
}

fn draw_no_questions(frame: &mut Frame, session: &Session) {
    let name = session.highlighted_area().map_or("", |area| area.name());
    let lines = vec![
        Line::from(name),
        Line::default(),
        Line::from(text::NO_QUESTIONS),
        Line::default(),
        Line::from(text::NO_QUESTIONS_HINT),
    ];

    draw_centred(frame, lines);
}

fn draw_level_picker(frame: &mut Frame, session: &Session) {
    let name = session.highlighted_area().map_or("", |area| area.name());
    let mut lines = vec![
        Line::from(name),
        Line::default(),
        Line::from(text::LEVEL_PICKER_TITLE),
        Line::default(),
    ];
    for level in Level::ALL {
        let marker = if level == session.highlighted_level() {
            text::AREA_MARKER
        } else {
            text::AREA_NO_MARKER
        };
        let tick = if session.levels().contains(level) {
            text::LEVEL_TICKED
        } else {
            text::LEVEL_UNTICKED
        };
        lines.push(Line::from(vec![
            Span::raw(marker),
            Span::raw(tick),
            Span::raw(text::level_name(level)),
        ]));
    }
    lines.push(Line::default());
    lines.push(Line::from(text::LEVEL_PICKER_HINT));
    lines.push(if session.levels().is_empty() {
        Line::from(text::NO_LEVEL_TICKED)
    } else {
        Line::default()
    });

    draw_centred(frame, lines);
}

fn draw_no_questions_at_levels(frame: &mut Frame, session: &Session) {
    let name = session.highlighted_area().map_or("", |area| area.name());
    let lines = vec![
        Line::from(name),
        Line::default(),
        Line::from(text::NO_QUESTIONS_AT_LEVELS),
        Line::default(),
        Line::from(text::NO_QUESTIONS_AT_LEVELS_HINT),
    ];

    draw_centred(frame, lines);
}

/// Rows the prompt of a question may use.
const PROMPT_ROWS: u16 = 3;

/// Columns of the answer field.
const FIELD_WIDTH: u16 = 30;

/// Columns of the label in front of the answer field.
const ANSWER_LABEL_WIDTH: u16 = 8;

fn draw_question(frame: &mut Frame, session: &Session) {
    let Some(drill) = session.drill() else {
        return;
    };
    let question = drill.question();
    let name = session.highlighted_area().map_or("", |area| area.name());
    let how_to_answer = match question.kind {
        Kind::Japanese => text::ANSWER_IN_JAPANESE,
        Kind::Meaning => text::ANSWER_IN_ENGLISH,
    };
    let [
        name_row,
        _,
        prompt_row,
        _,
        field_row,
        how_to_answer_row,
        _,
        _,
        _,
        _,
        hint_row,
    ] = Layout::vertical([
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Length(PROMPT_ROWS),
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Length(1),
        Constraint::Length(1),
    ])
    .flex(Flex::Center)
    .areas(frame.area());

    centred(frame, name_row, Line::from(name));
    frame.render_widget(
        Paragraph::new(question.prompt.as_str())
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true }),
        prompt_row,
    );
    let [label_area, field_area] = Layout::horizontal([
        Constraint::Length(ANSWER_LABEL_WIDTH),
        Constraint::Length(FIELD_WIDTH),
    ])
    .flex(Flex::Center)
    .areas(field_row);
    frame.render_widget(Paragraph::new(text::ANSWER_LABEL), label_area);
    if let Some(position) = drill.field().render(field_area, frame.buffer_mut()) {
        frame.set_cursor_position(position);
    }
    centred(frame, how_to_answer_row, Line::from(how_to_answer));
    centred(frame, hint_row, Line::from(text::QUESTION_HINT));
}

/// Draws one line centred in `area` by display width.
fn centred(frame: &mut Frame, area: Rect, line: Line) {
    frame.render_widget(Paragraph::new(line).alignment(Alignment::Center), area);
}

/// Draws `lines` as a block centred both ways, each line by display width.
fn draw_centred(frame: &mut Frame, lines: Vec<Line>) {
    let rows = u16::try_from(lines.len()).unwrap_or(u16::MAX);
    let [area] = Layout::vertical([Constraint::Length(rows)])
        .flex(Flex::Center)
        .areas(frame.area());
    frame.render_widget(Paragraph::new(lines).alignment(Alignment::Center), area);
}

#[cfg(test)]
mod tests {
    use ratatui::Terminal;
    use ratatui::backend::TestBackend;
    use ratatui::buffer::Buffer;
    use ratatui::layout::Position;
    use ratatui::style::Color;

    use super::*;
    use crate::areas::Area;
    use crate::areas::fake::FakeArea;
    use crate::session::Action;

    /// Draws `session` into a terminal of its own size and returns the rows,
    /// one symbol per cell.
    fn render(session: &Session) -> Vec<Vec<String>> {
        let (width, height) = session.size();
        let mut terminal = Terminal::new(TestBackend::new(width, height)).unwrap();
        terminal.draw(|frame| draw(frame, session)).unwrap();
        let buffer = terminal.backend().buffer();
        (0..height)
            .map(|y| {
                (0..width)
                    .map(|x| buffer[(x, y)].symbol().to_string())
                    .collect()
            })
            .collect()
    }

    fn buffer(session: &Session) -> Buffer {
        let (width, height) = session.size();
        let mut terminal = Terminal::new(TestBackend::new(width, height)).unwrap();
        terminal.draw(|frame| draw(frame, session)).unwrap();
        terminal.backend().buffer().clone()
    }

    fn row_index(rows: &[Vec<String>], needle: &str) -> usize {
        rows.iter()
            .position(|row| row.concat().contains(needle))
            .unwrap_or_else(|| panic!("{needle:?} is not on screen"))
    }

    fn text(rows: &[Vec<String>]) -> String {
        rows.iter()
            .map(|row| row.concat())
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn greeting_row(rows: &[Vec<String>]) -> &Vec<String> {
        rows.iter()
            .find(|row| row.iter().any(|symbol| symbol == "日"))
            .expect("the greeting is on screen")
    }

    #[test]
    fn greeting_is_24_columns_but_12_characters() {
        assert_eq!(Line::from(GREETING).width(), 24);
        assert_eq!(GREETING.chars().count(), 12);
    }

    #[test]
    fn greeting_is_centred_by_display_width() {
        let rows = render(&Session::new(80, 24));
        let row = greeting_row(&rows);
        assert_eq!(row[28], "日");
        assert_eq!(row[50], "！");
        assert!(row[..28].iter().all(|symbol| symbol == " "));
        assert!(row[52..].iter().all(|symbol| symbol == " "));
    }

    #[test]
    fn greeting_starts_at_the_same_column_in_an_odd_width() {
        let rows = render(&Session::new(81, 24));
        assert_eq!(greeting_row(&rows)[28], "日");
    }

    #[test]
    fn a_fresh_session_shows_every_line() {
        let screen = text(&render(&Session::new(80, 24)));
        let title = format!("tui-learnjapanese v{}", env!("CARGO_PKG_VERSION"));
        for expected in [
            title.as_str(),
            "Size: 80x24",
            "Elapsed: 0.0 s",
            "Last key: -",
            "Press q or Ctrl+C to quit",
        ] {
            assert!(screen.contains(expected), "missing {expected:?}");
        }
    }

    #[test]
    fn typed_character_and_elapsed_time_are_shown() {
        let mut session = Session::new(80, 24);
        session.apply(Action::Typed('あ'));
        for _ in 0..123 {
            session.apply(Action::Tick);
        }
        let screen = text(&render(&session));
        assert!(screen.contains("Last key: あ"));
        assert!(screen.contains("Elapsed: 12.3 s"));
    }

    #[test]
    fn fresh_welcome_shows_the_start_hint() {
        let screen = text(&render(&Session::new(80, 24)));
        assert!(screen.contains("Press Enter to start"));
    }

    #[test]
    fn welcome_lines_are_in_order() {
        let rows = render(&Session::new(80, 24));
        let order = [
            row_index(&rows, text::TITLE),
            row_index(&rows, "日"),
            row_index(&rows, text::START_HINT),
            row_index(&rows, "Size: "),
            row_index(&rows, "Last key: "),
            row_index(&rows, text::QUIT_HINT),
        ];
        assert!(order.windows(2).all(|pair| pair[0] < pair[1]), "{order:?}");
    }

    #[test]
    fn start_hint_is_centred_by_display_width() {
        let rows = render(&Session::new(80, 24));
        let row = &rows[row_index(&rows, text::START_HINT)];
        assert_eq!(row[30], "P");
        assert_eq!(row[49], "t");
        assert!(row[..30].iter().all(|symbol| symbol == " "));
        assert!(row[50..].iter().all(|symbol| symbol == " "));
    }

    #[test]
    fn welcome_stays_centred_after_a_resize() {
        let mut session = Session::new(80, 24);
        session.apply(Action::Resized {
            width: 100,
            height: 30,
        });
        let rows = render(&session);
        assert_eq!(greeting_row(&rows)[38], "日");
        assert_eq!(rows[row_index(&rows, text::START_HINT)][40], "P");
    }

    #[test]
    fn welcome_draws_at_more_sizes_without_panicking() {
        for (width, height) in [(1, 1), (5, 2), (20, 3), (200, 60)] {
            render(&Session::new(width, height));
        }
    }

    #[test]
    fn welcome_uses_no_colour() {
        let buffer = buffer(&Session::new(80, 24));
        assert!(
            buffer
                .content()
                .iter()
                .all(|cell| cell.fg == Color::Reset && cell.bg == Color::Reset)
        );
    }

    fn started(width: u16, height: u16) -> Session {
        let mut session = Session::new(width, height);
        session.apply(Action::Start);
        session
    }

    #[test]
    fn area_selection_placeholder_is_shown() {
        let screen = text(&render(&started(80, 24)));
        assert!(screen.contains("What do you want to practise?"));
        assert!(screen.contains("Up/Down to move, Enter to choose, Esc to leave"));
        for unexpected in ["日", "Press Enter to start", "Last key:"] {
            assert!(!screen.contains(unexpected), "unexpected {unexpected:?}");
        }
    }

    #[test]
    fn area_selection_follows_a_resize() {
        let mut session = started(80, 24);
        session.apply(Action::Resized {
            width: 100,
            height: 30,
        });
        let rows = render(&session);
        let row = &rows[row_index(&rows, text::AREA_SELECTION_HINT)];
        assert_eq!(row[27], "U");
        assert!(row[..27].iter().all(|symbol| symbol == " "));
    }

    #[test]
    fn area_selection_draws_at_any_size_without_panicking() {
        for (width, height) in [(1, 1), (5, 2), (20, 3), (200, 60)] {
            render(&started(width, height));
        }
    }

    #[test]
    fn area_selection_uses_no_colour() {
        let buffer = buffer(&started(80, 24));
        assert!(
            buffer
                .content()
                .iter()
                .all(|cell| cell.fg == Color::Reset && cell.bg == Color::Reset)
        );
    }

    #[test]
    fn any_size_draws_without_panicking() {
        for (width, height) in [(1, 1), (5, 2), (200, 60)] {
            render(&Session::new(width, height));
        }
    }

    fn started_with(areas: &'static [&'static dyn Area], width: u16, height: u16) -> Session {
        let mut session = Session::with_areas(width, height, areas);
        session.apply(Action::Start);
        session
    }

    fn chosen_with(areas: &'static [&'static dyn Area], width: u16, height: u16) -> Session {
        let mut session = started_with(areas, width, height);
        session.apply(Action::Choose);
        session
    }

    const EMPTY_KANJI: &[&dyn Area] = &[&FakeArea {
        name: "Kanji",
        has_questions: false,
    }];

    const FOUR: &[&dyn Area] = &[
        &FakeArea {
            name: "Alpha",
            has_questions: false,
        },
        &FakeArea {
            name: "Beta",
            has_questions: false,
        },
        &FakeArea {
            name: "Gamma",
            has_questions: false,
        },
        &FakeArea {
            name: "Delta",
            has_questions: false,
        },
    ];

    const WIDE_PAIR: &[&dyn Area] = &[
        &FakeArea {
            name: "Vocabulary",
            has_questions: false,
        },
        &FakeArea {
            name: "漢字",
            has_questions: false,
        },
    ];

    #[test]
    fn area_selection_lists_the_areas_in_order() {
        let rows = render(&started(80, 24));
        let order = [
            row_index(&rows, text::AREA_SELECTION_TITLE),
            row_index(&rows, "Vocabulary"),
            row_index(&rows, "Kanji"),
            row_index(&rows, "Grammar"),
        ];
        assert!(order.windows(2).all(|pair| pair[0] < pair[1]), "{order:?}");
        assert!(!text(&rows).contains("Area selection is coming soon."));
    }

    #[test]
    fn first_area_is_highlighted_with_a_marker() {
        let rows = render(&started(80, 24));
        let marked: Vec<_> = rows
            .iter()
            .map(|row| row.concat())
            .filter(|row| row.contains("> "))
            .collect();
        assert_eq!(marked.len(), 1, "{marked:?}");
        assert!(marked[0].contains("> Vocabulary"));
    }

    #[test]
    fn area_list_is_centred_by_display_width() {
        let rows = render(&started(80, 24));
        let vocabulary = &rows[row_index(&rows, "Vocabulary")];
        assert_eq!(vocabulary[34], ">");
        assert_eq!(vocabulary[36], "V");
        assert!(vocabulary[..34].iter().all(|symbol| symbol == " "));
        for (name, first) in [("Kanji", "K"), ("Grammar", "G")] {
            let row = &rows[row_index(&rows, name)];
            assert_eq!(row[34], " ");
            assert_eq!(row[35], " ");
            assert_eq!(row[36], first);
        }
    }

    #[test]
    fn area_names_line_up_by_display_width() {
        let rows = render(&started_with(WIDE_PAIR, 80, 24));
        assert_eq!(rows[row_index(&rows, "Vocabulary")][36], "V");
        assert_eq!(rows[row_index(&rows, "漢")][36], "漢");
    }

    #[test]
    fn another_area_adds_a_line() {
        let rows = render(&started_with(FOUR, 80, 24));
        let order = [
            row_index(&rows, "Alpha"),
            row_index(&rows, "Beta"),
            row_index(&rows, "Gamma"),
            row_index(&rows, "Delta"),
        ];
        assert!(order.windows(2).all(|pair| pair[0] < pair[1]), "{order:?}");
    }

    #[test]
    fn area_selection_hint_is_centred_below_the_list() {
        let rows = render(&started(80, 24));
        let hint = row_index(&rows, text::AREA_SELECTION_HINT);
        assert!(hint > row_index(&rows, "Grammar"));
        assert_eq!(rows[hint][17], "U");
        assert!(rows[hint][..17].iter().all(|symbol| symbol == " "));
    }

    #[test]
    fn area_list_stays_centred_after_a_resize() {
        let mut session = started(80, 24);
        session.apply(Action::Resized {
            width: 100,
            height: 30,
        });
        let rows = render(&session);
        let vocabulary = &rows[row_index(&rows, "Vocabulary")];
        assert_eq!(vocabulary[44], ">");
        assert_eq!(vocabulary[46], "V");
        assert_eq!(rows[row_index(&rows, "Kanji")][46], "K");
    }

    #[test]
    fn area_selection_with_no_areas_still_draws() {
        let screen = text(&render(&started_with(&[], 80, 24)));
        assert!(screen.contains(text::AREA_SELECTION_TITLE));
        assert!(screen.contains(text::AREA_SELECTION_HINT));
        render(&started_with(&[], 1, 1));
    }

    #[test]
    fn no_questions_screen_shows_the_area_and_the_message() {
        let rows = render(&chosen_with(EMPTY_KANJI, 80, 24));
        let order = [
            row_index(&rows, "Kanji"),
            row_index(&rows, text::NO_QUESTIONS),
            row_index(&rows, text::NO_QUESTIONS_HINT),
        ];
        assert!(order.windows(2).all(|pair| pair[0] < pair[1]), "{order:?}");
        let screen = text(&rows);
        assert!(!screen.contains(text::AREA_SELECTION_TITLE));
        assert!(!screen.contains(text::AREA_SELECTION_HINT));
    }

    #[test]
    fn no_questions_message_is_centred() {
        let rows = render(&chosen_with(EMPTY_KANJI, 80, 24));
        let row = &rows[row_index(&rows, text::NO_QUESTIONS)];
        assert_eq!(row[26], "N");
        assert!(row[..26].iter().all(|symbol| symbol == " "));
    }

    #[test]
    fn no_questions_draws_at_any_size_without_panicking() {
        for (width, height) in [(1, 1), (5, 2), (20, 3), (200, 60)] {
            render(&chosen_with(EMPTY_KANJI, width, height));
        }
    }

    #[test]
    fn no_questions_uses_no_colour() {
        let buffer = buffer(&chosen_with(EMPTY_KANJI, 80, 24));
        assert!(
            buffer
                .content()
                .iter()
                .all(|cell| cell.fg == Color::Reset && cell.bg == Color::Reset)
        );
    }

    #[test]
    fn moving_the_highlight_moves_the_marker() {
        let mut session = started(80, 24);
        session.apply(Action::Next);
        let rows = render(&session);
        assert_eq!(rows[row_index(&rows, "Kanji")][34], ">");
        assert_eq!(rows[row_index(&rows, "Vocabulary")][34], " ");
    }

    const PICKABLE: &[&dyn Area] = &[&FakeArea {
        name: "Sample",
        has_questions: true,
    }];

    fn on_picker(areas: &'static [&'static dyn Area], width: u16, height: u16) -> Session {
        chosen_with(areas, width, height)
    }

    fn level_rows(rows: &[Vec<String>]) -> Vec<&Vec<String>> {
        ["N5", "N4", "N3", "N2", "N1"]
            .iter()
            .map(|name| &rows[row_index(rows, name)])
            .collect()
    }

    #[test]
    fn level_picker_shows_the_levels_in_order() {
        let rows = render(&on_picker(PICKABLE, 80, 24));
        let order = [
            row_index(&rows, "Sample"),
            row_index(&rows, text::LEVEL_PICKER_TITLE),
            row_index(&rows, "N5"),
            row_index(&rows, "N4"),
            row_index(&rows, "N3"),
            row_index(&rows, "N2"),
            row_index(&rows, "N1"),
            row_index(&rows, text::LEVEL_PICKER_HINT),
        ];
        assert!(order.windows(2).all(|pair| pair[0] < pair[1]), "{order:?}");
        assert!(!text(&rows).contains(text::AREA_SELECTION_TITLE));
    }

    #[test]
    fn level_picker_starts_with_n5_ticked_and_highlighted() {
        let rows = render(&on_picker(PICKABLE, 80, 24));
        assert!(rows[row_index(&rows, "N5")].concat().contains("> [x] N5"));
        for name in ["N4", "N3", "N2", "N1"] {
            let row = rows[row_index(&rows, name)].concat();
            assert!(row.contains(&format!("[ ] {name}")), "{row:?}");
            assert!(!row.contains('>'), "{row:?}");
        }
    }

    #[test]
    fn level_lines_are_centred_by_display_width() {
        let rows = render(&on_picker(PICKABLE, 80, 24));
        let levels = level_rows(&rows);
        assert_eq!(levels[0][36], ">");
        assert_eq!(levels[0][38], "[");
        for row in &levels[1..] {
            assert_eq!(row[36], " ");
            assert_eq!(row[38], "[");
        }
    }

    #[test]
    fn level_picker_stays_centred_after_a_resize() {
        let mut session = on_picker(PICKABLE, 80, 24);
        session.apply(Action::Resized {
            width: 100,
            height: 30,
        });
        let rows = render(&session);
        for row in level_rows(&rows) {
            assert_eq!(row[48], "[");
        }
    }

    #[test]
    fn moving_on_the_picker_moves_the_marker() {
        let mut session = on_picker(PICKABLE, 80, 24);
        session.apply(Action::Next);
        let rows = render(&session);
        assert_eq!(rows[row_index(&rows, "N4")][36], ">");
        assert_eq!(rows[row_index(&rows, "N5")][36], " ");
    }

    #[test]
    fn level_picker_draws_at_any_size_without_panicking() {
        for (width, height) in [(1, 1), (5, 2), (20, 3), (200, 60)] {
            render(&on_picker(PICKABLE, width, height));
        }
    }

    #[test]
    fn level_picker_uses_no_colour() {
        let buffer = buffer(&on_picker(PICKABLE, 80, 24));
        assert!(
            buffer
                .content()
                .iter()
                .all(|cell| cell.fg == Color::Reset && cell.bg == Color::Reset)
        );
    }

    #[test]
    fn ticked_levels_show_an_x() {
        let mut session = on_picker(PICKABLE, 80, 24);
        session.apply(Action::Next);
        session.apply(Action::Toggle);
        let rows = render(&session);
        assert!(rows[row_index(&rows, "N4")].concat().contains("[x] N4"));
    }

    #[test]
    fn nothing_ticked_says_so() {
        let mut session = on_picker(PICKABLE, 80, 24);
        session.apply(Action::Toggle);
        let rows = render(&session);
        assert!(
            row_index(&rows, text::NO_LEVEL_TICKED) > row_index(&rows, text::LEVEL_PICKER_HINT)
        );
    }

    #[test]
    fn the_message_is_hidden_while_a_level_is_ticked() {
        let screen = text(&render(&on_picker(PICKABLE, 80, 24)));
        assert!(!screen.contains(text::NO_LEVEL_TICKED));
    }

    const SAMPLE: &[&dyn Area] = &[&crate::areas::sample::Sample];

    /// The Sample area with only N1 ticked, after Enter.
    fn at_no_questions_at_levels(width: u16, height: u16) -> Session {
        let mut session = on_picker(SAMPLE, width, height);
        session.apply(Action::Toggle);
        for _ in 0..4 {
            session.apply(Action::Next);
        }
        session.apply(Action::Toggle);
        session.apply(Action::Confirm);
        session
    }

    #[test]
    fn no_questions_at_levels_shows_the_message() {
        let rows = render(&at_no_questions_at_levels(80, 24));
        let order = [
            row_index(&rows, "Sample"),
            row_index(&rows, text::NO_QUESTIONS_AT_LEVELS),
            row_index(&rows, text::NO_QUESTIONS_AT_LEVELS_HINT),
        ];
        assert!(order.windows(2).all(|pair| pair[0] < pair[1]), "{order:?}");
        assert!(!text(&rows).contains(text::LEVEL_PICKER_TITLE));
    }

    #[test]
    fn no_questions_at_levels_message_is_centred() {
        let rows = render(&at_no_questions_at_levels(80, 24));
        let row = &rows[row_index(&rows, text::NO_QUESTIONS_AT_LEVELS)];
        assert_eq!(row[24], "N");
        assert!(row[..24].iter().all(|symbol| symbol == " "));
    }

    #[test]
    fn no_questions_at_levels_draws_at_any_size_without_panicking() {
        for (width, height) in [(1, 1), (5, 2), (20, 3), (200, 60)] {
            render(&at_no_questions_at_levels(width, height));
        }
    }

    #[test]
    fn no_questions_at_levels_uses_no_colour() {
        let buffer = buffer(&at_no_questions_at_levels(80, 24));
        assert!(
            buffer
                .content()
                .iter()
                .all(|cell| cell.fg == Color::Reset && cell.bg == Color::Reset)
        );
    }

    const READING_ONLY: &[&dyn Area] = &[&crate::areas::sample::READING];
    const MEANING_ONLY: &[&dyn Area] = &[&crate::areas::sample::MEANING];

    #[derive(Debug)]
    struct LongPrompt;

    impl Area for LongPrompt {
        fn name(&self) -> &'static str {
            "Long"
        }

        fn questions(&self, _levels: crate::practice::Levels) -> Vec<crate::practice::Entry> {
            use crate::practice::question::Question;
            vec![crate::practice::Entry {
                levels: crate::practice::Levels::only(Level::N5),
                questions: vec![Question {
                    prompt: "word ".repeat(20).trim().to_string(),
                    accepted: vec!["あ".to_string()],
                    shown: Vec::new(),
                    kind: Kind::Japanese,
                }],
            }]
        }
    }

    const LONG: &[&dyn Area] = &[&LongPrompt];

    fn practising(areas: &'static [&'static dyn Area], width: u16, height: u16) -> Session {
        let mut session = on_picker(areas, width, height);
        session.apply(Action::Confirm);
        session
    }

    #[test]
    fn question_screen_shows_the_prompt() {
        let rows = render(&practising(READING_ONLY, 80, 24));
        let order = [
            row_index(&rows, "Sample"),
            row_index(&rows, "How do you read"),
            row_index(&rows, text::ANSWER_IN_JAPANESE),
            row_index(&rows, text::QUESTION_HINT),
        ];
        assert!(order.windows(2).all(|pair| pair[0] < pair[1]), "{order:?}");
        assert!(!text(&rows).contains(text::LEVEL_PICKER_TITLE));
    }

    #[test]
    fn meaning_question_says_answer_in_english() {
        let screen = text(&render(&practising(MEANING_ONLY, 80, 24)));
        assert!(screen.contains(text::ANSWER_IN_ENGLISH));
        assert!(!screen.contains(text::ANSWER_IN_JAPANESE));
    }

    #[test]
    fn question_hint_is_centred() {
        let rows = render(&practising(READING_ONLY, 80, 24));
        let row = &rows[row_index(&rows, text::QUESTION_HINT)];
        assert_eq!(row[26], "E");
        assert!(row[..26].iter().all(|symbol| symbol == " "));
    }

    #[test]
    fn long_prompt_wraps() {
        let rows = render(&practising(LONG, 80, 24));
        let with_word = rows
            .iter()
            .filter(|row| row.concat().contains("word"))
            .count();
        assert!(with_word >= 2, "{with_word}");
    }

    #[test]
    fn question_screen_draws_at_any_size_without_panicking() {
        for (width, height) in [(1, 1), (5, 2), (20, 3), (200, 60)] {
            render(&practising(READING_ONLY, width, height));
        }
    }

    #[test]
    fn question_screen_uses_no_colour() {
        let buffer = buffer(&practising(READING_ONLY, 80, 24));
        assert!(
            buffer
                .content()
                .iter()
                .all(|cell| cell.fg == Color::Reset && cell.bg == Color::Reset)
        );
    }

    fn cursor(session: &Session) -> Position {
        let (width, height) = session.size();
        let mut terminal = Terminal::new(TestBackend::new(width, height)).unwrap();
        terminal.draw(|frame| draw(frame, session)).unwrap();
        terminal.get_cursor_position().unwrap()
    }

    fn type_text(session: &mut Session, text: &str) {
        for c in text.chars() {
            session.apply(Action::Edit(crate::answer_field::Edit::Insert(c)));
        }
    }

    #[test]
    fn answer_label_width_matches_the_text() {
        assert_eq!(
            Span::raw(text::ANSWER_LABEL).width(),
            usize::from(ANSWER_LABEL_WIDTH)
        );
    }

    #[test]
    fn answer_label_and_field_are_centred() {
        let session = practising(READING_ONLY, 80, 24);
        let rows = render(&session);
        let y = row_index(&rows, text::ANSWER_LABEL);
        assert_eq!(rows[y][21], "A");
        assert_eq!(
            cursor(&session),
            Position::new(29, u16::try_from(y).unwrap())
        );
    }

    #[test]
    fn cursor_follows_the_typed_text() {
        let mut session = practising(READING_ONLY, 80, 24);
        type_text(&mut session, "たか");
        let rows = render(&session);
        let y = row_index(&rows, text::ANSWER_LABEL);
        assert_eq!(rows[y][29], "た");
        assert_eq!(rows[y][31], "か");
        assert_eq!(cursor(&session).x, 33);
    }

    #[test]
    fn field_stays_centred_after_a_resize() {
        let mut session = practising(READING_ONLY, 80, 24);
        session.apply(Action::Resized {
            width: 100,
            height: 30,
        });
        let rows = render(&session);
        assert_eq!(rows[row_index(&rows, text::ANSWER_LABEL)][31], "A");
    }

    #[test]
    fn question_screen_with_text_uses_no_colour() {
        let mut session = practising(READING_ONLY, 80, 24);
        type_text(&mut session, "たかい");
        let buffer = buffer(&session);
        assert!(
            buffer
                .content()
                .iter()
                .all(|cell| cell.fg == Color::Reset && cell.bg == Color::Reset)
        );
    }
}
