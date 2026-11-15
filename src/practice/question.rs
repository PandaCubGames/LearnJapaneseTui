// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 PandaCubGames

//! A question and the entry it belongs to, as areas supply them.

use super::Levels;

/// How an answer is typed and checked.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    /// A Japanese answer: romaji, kana or kanji, checked by `grading`.
    Japanese,
    /// A meaning in plain English, checked by `practice::meaning`.
    // No real area asks for a meaning yet (Vocabulary and Kanji, issues #11 and
    // #12): the first one that does removes this attribute. Tests construct it already.
    #[cfg_attr(
        not(test),
        expect(
            dead_code,
            reason = "constructed by the Vocabulary and Kanji questions, issues #11 and #12"
        )
    )]
    Meaning,
}

/// One question, as the practice module sees it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Question {
    /// What the user is asked, ready to show.
    pub prompt: String,
    /// Every accepted answer. Japanese answers are written in kana or kanji, never romaji.
    pub accepted: Vec<String>,
    /// The Japanese texts the prompt already shows. An answer typed back as one of them
    /// is wrong when that text contains a kanji, so kanji cannot be copied and pasted;
    /// kana-only texts never make an answer wrong. Empty when the prompt shows no
    /// Japanese. Ignored for `Meaning`.
    pub shown: Vec<String>,
    pub kind: Kind,
}

/// One entry of an area (a word, a kanji, a verb), with its questions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    pub levels: Levels,
    pub questions: Vec<Question>,
}
