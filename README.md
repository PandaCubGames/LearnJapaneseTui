# tui-learnjapanese

**Learn Japanese in your terminal.** 日本語を勉強しましょう！

`tui-learnjapanese` is a terminal app (TUI) for learning Japanese, written in Rust. It's organized into independent learning modules, ships as a single executable, and works anywhere you have a terminal, including over SSH.

> **Status: early development.** The project is just getting started. There's no usable learning content yet, and the features below are the plan, not what exists today.

## Planned features

- **Learning modules** you pick from a menu:
  - **Verbs:** conjugation practice.
  - **Vocabulary:** word drills.
  - More modules to come.
- **Type answers your way:**
  - in Japanese, through your system's input method (IME),
  - or in romaji, which the app converts to kana as you type (`taberu` → `たべる`).
- **Proper Japanese display:** kana and kanji line up correctly, because layout uses terminal column width (CJK characters are two columns wide).
- **One file, nothing to install:** each release is a single self-contained executable with all learning content built in.
  - Linux (x86_64, aarch64) as a static binary that runs on any distribution
  - macOS (Intel, Apple Silicon)
  - Windows (x86_64)
- **Works over SSH:** keyboard-driven, no mouse, no images, no GUI dependencies.

## Requirements

- A terminal using **UTF-8**.
- A font that includes **Japanese characters**.
  - The app can't ship fonts, so this depends on your terminal setup.
  - Most modern systems already have one. If you see boxes (□) or question marks instead of kana, install a CJK font such as Noto Sans CJK JP.
- Optional: a **Japanese input method** (e.g. fcitx5/ibus with Mozc on Linux, or the built-in Japanese IME on macOS/Windows) if you want to type kana and kanji directly. Romaji works without one.

## Building from source

You need [Rust](https://www.rust-lang.org/tools/install) **1.85 or newer**.

```sh
git clone <repository-url>
cd tui-learnjapanese
cargo run                # run a debug build
cargo build --release    # optimized binary in target/release/
```

Prebuilt releases will be available once the first version is published.

## Contributing

Contributions are welcome! Please read [CONTRIBUTING.md](CONTRIBUTING.md) first. It covers the branch workflow, the commit message format, the automatic checks, and the licensing rules.

## License

Copyright (C) 2026 PandaCubGames

This program is free software: you can redistribute it and/or modify it under the terms of the **GNU General Public License** as published by the Free Software Foundation, either **version 3** of the License, or (at your option) any later version. See [LICENSE](LICENSE) for the full text.

Third-party learning content (dictionaries, example sentences, etc.) is listed with its own license and attribution in [ATTRIBUTION.md](ATTRIBUTION.md).
