// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 PandaCubGames

//! The generic practice module: what every area has in common. It never sees an area's own words, kanji or verbs, only questions (issue #21).

mod level;

pub use level::{Level, Levels};
