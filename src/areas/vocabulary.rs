// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 PandaCubGames

//! The Vocabulary area. Its words and questions come with issue #11.

use super::Area;
use crate::text;

#[derive(Debug)]
pub struct Vocabulary;

impl Area for Vocabulary {
    fn name(&self) -> &'static str {
        text::AREA_VOCABULARY
    }

    fn has_questions(&self) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_named_vocabulary() {
        assert_eq!(Vocabulary.name(), "Vocabulary");
    }
}
