// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 PandaCubGames

//! The event loop and the terminal lifecycle.

use std::time::Duration;

/// How often the app updates itself when no input arrives.
pub const TICK_INTERVAL: Duration = Duration::from_millis(100);
