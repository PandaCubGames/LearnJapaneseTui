// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 PandaCubGames

//! Draws the current screen. It only reads the state it is given.
//!
//! Screen text must not use characters of ambiguous width, such as the
//! multiplication sign or the em dash: terminals set up for CJK draw them
//! two columns wide, which breaks centring.

use ratatui::Frame;
use ratatui::layout::{Alignment, Constraint, Flex, Layout};
use ratatui::style::Stylize;
use ratatui::text::Line;
use ratatui::widgets::Paragraph;

use crate::session::{Screen, Session};

pub const GREETING: &str = "日本語を勉強しましょう！";

pub fn draw(frame: &mut Frame, session: &Session) {
    match session.screen() {
        Screen::Welcome => draw_welcome(frame, session),
    }
}

fn draw_welcome(frame: &mut Frame, session: &Session) {
    let (width, height) = session.size();
    let tenths = session.elapsed().as_millis() / 100;
    let last_key = session
        .last_char()
        .map_or_else(|| "-".to_string(), String::from);

    let lines = vec![
        Line::from(format!("tui-learnjapanese v{}", env!("CARGO_PKG_VERSION"))).bold(),
        Line::default(),
        Line::from(GREETING),
        Line::default(),
        Line::from(format!(
            "Size: {width}x{height}   Elapsed: {}.{} s",
            tenths / 10,
            tenths % 10
        )),
        Line::from(format!("Last key: {last_key}")),
        Line::from("Press q or Ctrl+C to quit"),
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
    fn any_size_draws_without_panicking() {
        for (width, height) in [(1, 1), (5, 2), (200, 60)] {
            render(&Session::new(width, height));
        }
    }
}
