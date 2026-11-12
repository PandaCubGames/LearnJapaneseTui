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
