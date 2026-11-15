// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 PandaCubGames

//! The Kanji area. Its words and questions come with issue #12.

use super::Area;
use crate::text;

#[derive(Debug)]
pub struct Kanji;

impl Area for Kanji {
    fn name(&self) -> &'static str {
        text::AREA_KANJI
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_named_kanji() {
        assert_eq!(Kanji.name(), "Kanji");
    }
}
