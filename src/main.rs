// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 PandaCubGames

use std::process::ExitCode;

mod answer_field;
mod app;
mod areas;
mod conjugation;
mod grading;
mod kana;
mod keys;
mod practice;
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
