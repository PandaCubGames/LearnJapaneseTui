// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 PandaCubGames

use std::process::ExitCode;

mod app;
mod keys;
mod session;
mod ui;

fn main() -> ExitCode {
    // `app::run` has already restored the terminal when it returns, so the
    // message lands on the normal screen.
    match app::run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("tui-learnjapanese: {error}");
            ExitCode::FAILURE
        }
    }
}
