// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 PandaCubGames

//! The practice areas the app knows about, listed in one place.

/// One thing the user can practise. #21 grows this into the full area
/// contract (questions for a set of JLPT levels).
pub trait Area: std::fmt::Debug {
    /// The name shown in the list of areas.
    fn name(&self) -> &'static str;
    /// Whether the area has any question at all. An area without questions
    /// can still be chosen; it says so.
    fn has_questions(&self) -> bool;
}

/// Every area, in the order the list shows them. There are none yet. Adding
/// an area adds its own file under `src/areas/` and one entry here.
const ALL: &[&dyn Area] = &[];

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
}
