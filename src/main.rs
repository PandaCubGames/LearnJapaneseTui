// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 PandaCubGames

// Not called from `main` yet: the loop that uses these modules comes later.
#[allow(dead_code)]
mod app;
#[allow(dead_code)]
mod keys;
#[allow(dead_code)]
mod session;
#[allow(dead_code)]
mod ui;

fn main() {
    println!(
        "tui-learnjapanese v{} — 日本語を勉強しましょう！",
        env!("CARGO_PKG_VERSION")
    );
}
