// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 PandaCubGames

//! The text field an answer is typed into. It keeps the text exactly as typed
//! (romaji or text committed by an IME) and converts nothing.

use ratatui::buffer::Buffer;
use ratatui::crossterm::event::{Event, KeyCode, KeyEventKind, KeyModifiers};
use ratatui::layout::{Position, Rect};
use ratatui::style::Style;
use ratatui::text::Span;

/// One change to the answer text.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Edit {
    /// Append a character at the end.
    Insert(char),
    /// Remove the last character the user sees.
    DeleteBack,
}

/// The edit a terminal event makes to an answer field, if any.
pub fn edit_for(event: &Event) -> Option<Edit> {
    let Event::Key(key) = event else {
        return None;
    };
    if key.kind == KeyEventKind::Release {
        return None;
    }
    // Some terminals send 0x08 for Backspace, which arrives as Ctrl+H.
    if key.code == KeyCode::Char('h') && key.modifiers == KeyModifiers::CONTROL {
        return Some(Edit::DeleteBack);
    }
    let plain = (key.modifiers - KeyModifiers::SHIFT).is_empty();
    match key.code {
        KeyCode::Backspace if plain => Some(Edit::DeleteBack),
        KeyCode::Char(c) if plain && !c.is_control() => Some(Edit::Insert(c)),
        _ => None,
    }
}

/// The answer being typed. It only appends and deletes at the end.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct AnswerField {
    text: String,
}

impl AnswerField {
    /// The text exactly as typed.
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Applies one edit.
    pub fn apply(&mut self, edit: Edit) {
        match edit {
            Edit::Insert(c) => {
                if !c.is_control() {
                    self.text.push(c);
                }
            }
            Edit::DeleteBack => {
                // A user-perceived character can be several `char`s (a kana
                // with a voiced mark, a kanji with a variation selector).
                let last = Span::raw(self.text.as_str())
                    .styled_graphemes(Style::default())
                    .last()
                    .map(|g| g.symbol.len());
                if let Some(len) = last {
                    let keep = self.text.len() - len;
                    self.text.truncate(keep);
                }
            }
        }
    }
    /// Draws the end of the text on the first row of `area`, by display width,
    /// and returns where the cursor goes, or `None` when `area` has no columns
    /// or no rows. One column is kept free for the cursor.
    pub fn render(&self, area: Rect, buf: &mut Buffer) -> Option<Position> {
        if area.width == 0 || area.height == 0 {
            return None;
        }
        let room = usize::from(area.width - 1);
        let mut width = 0;
        let mut bytes = 0;
        let span = Span::raw(self.text.as_str());
        let graphemes: Vec<_> = span.styled_graphemes(Style::default()).collect();
        for grapheme in graphemes.iter().rev() {
            let cluster = Span::raw(grapheme.symbol).width();
            if width + cluster > room {
                break;
            }
            width += cluster;
            bytes += grapheme.symbol.len();
        }
        let visible = &self.text[self.text.len() - bytes..];
        buf.set_stringn(area.x, area.y, visible, room, Style::default());
        let columns = u16::try_from(width).unwrap_or(u16::MAX);
        Some(Position::new(area.x.saturating_add(columns), area.y))
    }
}

#[cfg(test)]
mod tests {
    use ratatui::Terminal;
    use ratatui::backend::TestBackend;
    use ratatui::crossterm::event::KeyEvent;
    use ratatui::style::Color;

    use super::*;

    fn press(code: KeyCode, modifiers: KeyModifiers) -> Event {
        Event::Key(KeyEvent::new(code, modifiers))
    }

    fn with_kind(code: KeyCode, kind: KeyEventKind) -> Event {
        Event::Key(KeyEvent::new_with_kind(code, KeyModifiers::NONE, kind))
    }

    fn field(text: &str) -> AnswerField {
        let mut field = AnswerField::default();
        for c in text.chars() {
            field.apply(Edit::Insert(c));
        }
        field
    }

    #[test]
    fn new_field_is_empty() {
        assert_eq!(AnswerField::default().text(), "");
    }

    #[test]
    fn typed_text_is_kept_as_typed() {
        assert_eq!(field("takai").text(), "takai");
    }

    #[test]
    fn committed_ime_text_is_appended() {
        assert_eq!(field("高いタカイ").text(), "高いタカイ");
    }

    #[test]
    fn control_characters_are_ignored() {
        let mut field = field("a");
        for c in ['\t', '\n', '\u{7f}'] {
            field.apply(Edit::Insert(c));
        }
        assert_eq!(field.text(), "a");
    }

    #[test]
    fn backspace_removes_a_whole_kanji() {
        let mut field = field("食べる");
        field.apply(Edit::DeleteBack);
        field.apply(Edit::DeleteBack);
        assert_eq!(field.text(), "食");
    }

    #[test]
    fn backspace_removes_a_kana_with_its_mark() {
        let mut field = field("たか\u{3099}");
        field.apply(Edit::DeleteBack);
        assert_eq!(field.text(), "た");
    }

    #[test]
    fn backspace_removes_a_kanji_with_its_variation_selector() {
        let mut field = field("a葛\u{E0100}");
        field.apply(Edit::DeleteBack);
        assert_eq!(field.text(), "a");
    }

    #[test]
    fn backspace_on_empty_does_nothing() {
        let mut field = AnswerField::default();
        field.apply(Edit::DeleteBack);
        assert_eq!(field.text(), "");
    }

    #[test]
    fn characters_type() {
        assert_eq!(
            edit_for(&press(KeyCode::Char('a'), KeyModifiers::NONE)),
            Some(Edit::Insert('a'))
        );
        assert_eq!(
            edit_for(&press(KeyCode::Char('A'), KeyModifiers::SHIFT)),
            Some(Edit::Insert('A'))
        );
        assert_eq!(
            edit_for(&press(KeyCode::Char('あ'), KeyModifiers::NONE)),
            Some(Edit::Insert('あ'))
        );
        assert_eq!(
            edit_for(&with_kind(KeyCode::Char('a'), KeyEventKind::Repeat)),
            Some(Edit::Insert('a'))
        );
    }

    #[test]
    fn backspace_deletes() {
        for event in [
            press(KeyCode::Backspace, KeyModifiers::NONE),
            press(KeyCode::Backspace, KeyModifiers::SHIFT),
            press(KeyCode::Char('h'), KeyModifiers::CONTROL),
            with_kind(KeyCode::Backspace, KeyEventKind::Repeat),
        ] {
            assert_eq!(edit_for(&event), Some(Edit::DeleteBack), "{event:?}");
        }
    }

    #[test]
    fn other_keys_do_not_edit() {
        let mut events = vec![
            press(KeyCode::Char('a'), KeyModifiers::CONTROL),
            press(KeyCode::Char('a'), KeyModifiers::ALT),
            press(KeyCode::Char('c'), KeyModifiers::CONTROL),
            press(KeyCode::Backspace, KeyModifiers::CONTROL),
            with_kind(KeyCode::Char('a'), KeyEventKind::Release),
            with_kind(KeyCode::Backspace, KeyEventKind::Release),
            Event::Paste("たかい".to_string()),
            Event::FocusGained,
            Event::Resize(80, 24),
        ];
        for code in [
            KeyCode::Enter,
            KeyCode::Esc,
            KeyCode::Up,
            KeyCode::Left,
            KeyCode::Tab,
            KeyCode::Delete,
            KeyCode::Home,
            KeyCode::End,
        ] {
            events.push(press(code, KeyModifiers::NONE));
        }
        for event in events {
            assert_eq!(edit_for(&event), None, "{event:?}");
        }
    }

    #[test]
    fn typing_through_the_key_mapping() {
        let mut field = AnswerField::default();
        for event in [
            press(KeyCode::Char('t'), KeyModifiers::NONE),
            press(KeyCode::Char('a'), KeyModifiers::NONE),
            press(KeyCode::Char('k'), KeyModifiers::NONE),
            press(KeyCode::Char('a'), KeyModifiers::NONE),
            press(KeyCode::Char('x'), KeyModifiers::NONE),
            press(KeyCode::Backspace, KeyModifiers::NONE),
            press(KeyCode::Char('i'), KeyModifiers::NONE),
        ] {
            if let Some(edit) = edit_for(&event) {
                field.apply(edit);
            }
        }
        assert_eq!(field.text(), "takai");
    }

    fn draw(field: &AnswerField, area: Rect) -> (Buffer, Option<Position>) {
        let mut buffer = Buffer::empty(Rect::new(0, 0, 20, 5));
        let cursor = field.render(area, &mut buffer);
        (buffer, cursor)
    }

    fn symbols(buffer: &Buffer, y: u16, columns: std::ops::Range<u16>) -> Vec<String> {
        columns
            .map(|x| buffer[(x, y)].symbol().to_string())
            .collect()
    }

    #[test]
    fn kana_take_two_columns() {
        let (buffer, cursor) = draw(&field("たかい"), Rect::new(0, 0, 10, 1));
        assert_eq!(buffer[(0, 0)].symbol(), "た");
        assert_eq!(buffer[(2, 0)].symbol(), "か");
        assert_eq!(buffer[(4, 0)].symbol(), "い");
        assert_eq!(cursor, Some(Position::new(6, 0)));
    }

    #[test]
    fn latin_letters_take_one_column() {
        let (buffer, cursor) = draw(&field("takai"), Rect::new(0, 0, 10, 1));
        assert_eq!(symbols(&buffer, 0, 0..5), ["t", "a", "k", "a", "i"]);
        assert_eq!(cursor, Some(Position::new(5, 0)));
    }

    #[test]
    fn empty_field_puts_the_cursor_at_the_start() {
        let (buffer, cursor) = draw(&AnswerField::default(), Rect::new(3, 2, 10, 1));
        assert_eq!(cursor, Some(Position::new(3, 2)));
        assert_eq!(symbols(&buffer, 2, 3..13), [" "; 10]);
    }

    #[test]
    fn field_is_drawn_inside_its_area() {
        let (buffer, cursor) = draw(&field("たかい"), Rect::new(3, 2, 10, 1));
        assert_eq!(buffer[(3, 2)].symbol(), "た");
        assert_eq!(buffer[(7, 2)].symbol(), "い");
        assert_eq!(cursor, Some(Position::new(9, 2)));
        for y in [0, 1, 3, 4] {
            assert_eq!(symbols(&buffer, y, 0..20), [" "; 20], "row {y}");
        }
    }

    #[test]
    fn field_uses_no_colour() {
        let (buffer, _) = draw(&field("takaiたかい"), Rect::new(0, 0, 20, 5));
        for y in 0..5 {
            for x in 0..20 {
                assert_eq!(buffer[(x, y)].fg, Color::Reset);
                assert_eq!(buffer[(x, y)].bg, Color::Reset);
            }
        }
    }

    #[test]
    fn long_text_keeps_its_end_visible() {
        let (buffer, cursor) = draw(&field("abcdefたか"), Rect::new(0, 0, 6, 1));
        assert_eq!(buffer[(0, 0)].symbol(), "f");
        assert_eq!(buffer[(1, 0)].symbol(), "た");
        assert_eq!(buffer[(3, 0)].symbol(), "か");
        assert_eq!(cursor, Some(Position::new(5, 0)));
    }

    #[test]
    fn wide_character_is_not_cut_in_half() {
        let (buffer, cursor) = draw(&field("あいう"), Rect::new(0, 0, 4, 1));
        assert_eq!(buffer[(0, 0)].symbol(), "う");
        assert_eq!(buffer[(2, 0)].symbol(), " ");
        assert_eq!(buffer[(3, 0)].symbol(), " ");
        assert_eq!(cursor, Some(Position::new(2, 0)));
    }

    #[test]
    fn field_at_any_size() {
        let field = field("食べる");
        for area in [
            Rect::new(0, 0, 0, 0),
            Rect::new(0, 0, 0, 1),
            Rect::new(0, 0, 1, 0),
        ] {
            assert_eq!(draw(&field, area).1, None, "{area:?}");
        }
        let (buffer, cursor) = draw(&field, Rect::new(0, 0, 1, 1));
        assert_eq!(cursor, Some(Position::new(0, 0)));
        assert_eq!(buffer[(0, 0)].symbol(), " ");
        let (buffer, cursor) = draw(&field, Rect::new(0, 0, 2, 1));
        assert_eq!(cursor, Some(Position::new(0, 0)));
        assert_eq!(symbols(&buffer, 0, 0..2), [" ", " "]);
        let (buffer, cursor) = draw(&field, Rect::new(0, 0, 3, 1));
        assert_eq!(buffer[(0, 0)].symbol(), "る");
        assert_eq!(cursor, Some(Position::new(2, 0)));
    }

    #[test]
    fn only_the_first_row_is_drawn() {
        let (buffer, _) = draw(&field("takai"), Rect::new(0, 0, 10, 3));
        for y in [1, 2] {
            assert_eq!(symbols(&buffer, y, 0..20), [" "; 20], "row {y}");
        }
    }

    #[test]
    fn cursor_reaches_the_terminal() {
        let field = field("たかい");
        let mut terminal = Terminal::new(TestBackend::new(20, 3)).unwrap();
        terminal
            .draw(|frame| {
                if let Some(p) = field.render(frame.area(), frame.buffer_mut()) {
                    frame.set_cursor_position(p);
                }
            })
            .unwrap();
        assert_eq!(terminal.get_cursor_position().unwrap(), Position::new(6, 0));
    }
}
