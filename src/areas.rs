// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 PandaCubGames

//! The practice areas the app knows about, listed in one place.

mod grammar;
#[cfg(test)]
pub mod sample;

use crate::practice::{self, Entry, Levels};
use grammar::Grammar;

/// One thing the user can practise.
pub trait Area: std::fmt::Debug {
    /// The name shown in the list of areas.
    fn name(&self) -> &'static str;
    /// The area's entries that have at least one of `levels`, each with all its
    /// questions, generated and hand-written alike. The practice module drops
    /// any entry without a chosen level or without questions. No entries by default.
    fn questions(&self, _levels: Levels) -> Vec<Entry> {
        Vec::new()
    }
    /// Whether the area has any question at all. Built on `questions`; areas do not
    /// override it. An area without questions can still be chosen; it says so.
    fn has_questions(&self) -> bool {
        !practice::selection(self.questions(Levels::ALL), Levels::ALL).is_empty()
    }
}

/// Every area, in the order the list shows them. Adding an area adds its own
/// file under `src/areas/` and one entry here.
const ALL: &[&dyn Area] = &[&Grammar];

pub fn all() -> &'static [&'static dyn Area] {
    ALL
}

#[cfg(test)]
pub mod fake {
    use super::Area;

    /// An area for tests, with a fixed name and a fixed answer to `has_questions`.
    #[derive(Debug)]
    pub struct FakeArea {
        pub name: &'static str,
        pub has_questions: bool,
    }

    impl Area for FakeArea {
        fn name(&self) -> &'static str {
            self.name
        }

        fn has_questions(&self) -> bool {
            self.has_questions
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn area_names_are_unique_and_not_empty() {
        let names: Vec<_> = all().iter().map(|area| area.name()).collect();
        for (i, name) in names.iter().enumerate() {
            assert!(!name.is_empty());
            assert!(!names[i + 1..].contains(name), "duplicate {name:?}");
        }
    }

    use crate::practice::Level;
    use crate::practice::question::{Kind, Question};

    #[derive(Debug)]
    struct Bare;

    impl Area for Bare {
        fn name(&self) -> &'static str {
            "Bare"
        }
    }

    #[derive(Debug)]
    struct Hollow;

    impl Area for Hollow {
        fn name(&self) -> &'static str {
            "Hollow"
        }

        fn questions(&self, _levels: Levels) -> Vec<Entry> {
            vec![Entry {
                levels: Levels::only(Level::N5),
                questions: Vec::new(),
            }]
        }
    }

    #[derive(Debug)]
    struct Levelless;

    impl Area for Levelless {
        fn name(&self) -> &'static str {
            "Levelless"
        }

        fn questions(&self, _levels: Levels) -> Vec<Entry> {
            vec![Entry {
                levels: Levels::only(Level::N5).toggled(Level::N5),
                questions: vec![Question {
                    prompt: "p".to_string(),
                    accepted: vec!["あ".to_string()],
                    shown: Vec::new(),
                    kind: Kind::Japanese,
                }],
            }]
        }
    }

    #[test]
    fn an_area_without_entries_has_no_questions() {
        assert!(!Bare.has_questions());
    }

    #[test]
    fn entries_without_questions_do_not_count() {
        assert!(!Hollow.has_questions());
    }

    #[test]
    fn entries_without_a_level_do_not_count() {
        assert!(!Levelless.has_questions());
    }

    #[test]
    fn every_listed_area_agrees_with_its_questions() {
        for area in all() {
            let selected = practice::selection(area.questions(Levels::ALL), Levels::ALL);
            assert_eq!(
                area.has_questions(),
                !selected.is_empty(),
                "{}",
                area.name()
            );
        }
    }
}
