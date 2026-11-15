// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 PandaCubGames

//! JLPT levels and sets of levels.

/// A JLPT level.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Level {
    N5,
    N4,
    N3,
    N2,
    N1,
}

impl Level {
    /// Every level, in the order the level picker lists them.
    pub const ALL: [Level; 5] = [Level::N5, Level::N4, Level::N3, Level::N2, Level::N1];

    fn bit(self) -> u8 {
        match self {
            Level::N5 => 1,
            Level::N4 => 2,
            Level::N3 => 4,
            Level::N2 => 8,
            Level::N1 => 16,
        }
    }
}

/// A set of JLPT levels.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Levels(u8);

impl Levels {
    pub fn only(level: Level) -> Levels {
        Levels(level.bit())
    }

    pub fn contains(self, level: Level) -> bool {
        self.0 & level.bit() != 0
    }

    pub fn toggled(self, level: Level) -> Levels {
        Levels(self.0 ^ level.bit())
    }

    pub fn is_empty(self) -> bool {
        self.0 == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_lists_n5_to_n1() {
        assert_eq!(
            Level::ALL,
            [Level::N5, Level::N4, Level::N3, Level::N2, Level::N1]
        );
    }

    #[test]
    fn only_contains_just_that_level() {
        for l in Level::ALL {
            for m in Level::ALL {
                assert_eq!(Levels::only(l).contains(m), l == m);
            }
        }
    }

    #[test]
    fn toggled_ticks_and_unticks() {
        let two = Levels::only(Level::N5).toggled(Level::N4);
        assert!(two.contains(Level::N5));
        assert!(two.contains(Level::N4));
        assert_eq!(two.toggled(Level::N4), Levels::only(Level::N5));
    }

    #[test]
    fn empty_set() {
        assert!(Levels::only(Level::N5).toggled(Level::N5).is_empty());
        assert!(!Levels::only(Level::N5).is_empty());
    }
}
