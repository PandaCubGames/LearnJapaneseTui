// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 PandaCubGames

//! The text field an answer is typed into. It keeps the text exactly as typed
//! (romaji or text committed by an IME) and converts nothing.

use ratatui::crossterm::event::{Event, KeyCode, KeyEventKind, KeyModifiers};
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
}

#[cfg(test)]
mod tests {
    use ratatui::crossterm::event::KeyEvent;

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
}
