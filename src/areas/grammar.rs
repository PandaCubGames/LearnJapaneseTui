// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 PandaCubGames

//! The Grammar area. Its words and questions come with issue #13.

use super::Area;
use crate::text;

#[derive(Debug)]
pub struct Grammar;

impl Area for Grammar {
    fn name(&self) -> &'static str {
        text::AREA_GRAMMAR
    }

    fn has_questions(&self) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_named_grammar() {
        assert_eq!(Grammar.name(), "Grammar");
    }
}
