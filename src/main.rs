// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 PandaCubGames

use std::process::ExitCode;

mod app;
// No screen uses this module yet: the practice question screen (issue #21)
// does, and removes this attribute. Tests use every item already.
#[cfg_attr(
    not(test),
    expect(dead_code, reason = "used by the practice question screen, issue #21")
)]
mod grading;
// No screen uses this module yet: the practice question screen (issue #21)
// does, and removes this attribute. Tests use every item already.
#[cfg_attr(
    not(test),
    expect(dead_code, reason = "used by the practice question screen, issue #21")
)]
mod kana;
mod keys;
// No screen uses this module yet: the practice question screen (issue #21)
// does, and removes this attribute. Tests use every item already.
#[cfg_attr(
    not(test),
    expect(dead_code, reason = "used by the practice question screen, issue #21")
)]
mod romaji;
mod session;
mod text;
mod ui;

fn main() -> ExitCode {
    // `app::run` has already restored the terminal when it returns, so the
    // message lands on the normal screen.
    match app::run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{}", text::fatal_error(error));
            ExitCode::FAILURE
        }
    }
}
