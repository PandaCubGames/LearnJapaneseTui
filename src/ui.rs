// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 PandaCubGames

//! Draws the current screen. It only reads the state it is given, and takes
//! all wording from [`crate::text`].

use ratatui::Frame;
use ratatui::layout::{Alignment, Constraint, Flex, Layout};
use ratatui::style::Stylize;
use ratatui::text::Line;
use ratatui::widgets::Paragraph;

use crate::session::{Screen, Session};
use crate::text::{self, GREETING};

pub fn draw(frame: &mut Frame, session: &Session) {
    match session.screen() {
        Screen::Welcome => draw_welcome(frame, session),
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
    use ratatui::style::Color;

    use super::*;
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

    #[test]
    fn any_size_draws_without_panicking() {
        for (width, height) in [(1, 1), (5, 2), (200, 60)] {
            render(&Session::new(width, height));
        }
    }
}
