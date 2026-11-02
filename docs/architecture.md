<!--
SPDX-License-Identifier: GPL-3.0-or-later
Copyright (C) 2026 PandaCubGames
-->

# Architecture (draft)

> **Status: planned.** None of this exists in code yet. These diagrams describe the
> intended design and will change as it gets built.

The diagrams use [Mermaid](https://mermaid.js.org/), which GitHub renders directly.
To edit one visually, paste its code into Excalidraw (*More tools → Mermaid to Excalidraw*).

## Core idea

You pick a study mode (Grammar, Vocabulary or Kanji) and keep answering questions
until you quit. **One shared session engine runs the loop.** Each module only answers
two things: *what's the next item?* and *is this answer correct?* Input handling,
romaji conversion, scoring and the quit key live in one place, and adding a module
doesn't touch the others.

## 1. Screen flow

The app is one TUI event loop (draw → read key → update). "Studying" is a screen
state, not a blocking loop, so resizes and quit keys always work.

```mermaid
stateDiagram-v2
    [*] --> Menu: no argument
    [*] --> Session: CLI argument (g / v / k, optional --level)
    Menu --> Session: choose mode
    Session --> Menu: Esc (back to menu, any time)
    Session --> Summary: Ctrl+C
    Summary --> [*]: quit
    Menu --> [*]: quit

    note right of Session
        Repeats questions until the user leaves.
        Esc works even mid-answer.
        A window resize redraws any screen.
    end note
```

## 2. The study loop inside a session

```mermaid
flowchart TD
    A[Scheduler picks next item] --> B[Module builds the prompt]
    B --> C[Render question]
    C --> D[/User types the answer<br/>IME kana/kanji or romaji/]
    D --> E[input: romaji → kana<br/>+ normalize]
    E --> F{Module check}
    F -->|Correct| G[Feedback]
    F -->|Close| G
    F -->|Wrong + expected answer| G
    G --> H[Record result in Scheduler<br/>update score and streak]
    H --> A
    C -. Esc .-> M([Back to menu])
    D -. Esc .-> M
    C -. Ctrl+C .-> S([Summary, then quit])
    D -. Ctrl+C .-> S
```

**Leaving a session:** `Esc` returns to the menu at any time, even while typing
an answer (the unfinished answer is dropped). `Ctrl+C` shows the session summary
and quits the app. Letter keys like `q` don't quit, because they can be part of a
typed answer.

## 3. Components

```mermaid
flowchart LR
    main[main.rs<br/>parse CLI args] --> app

    subgraph core[Core]
        app[app<br/>event loop + screen state]
        session[session engine<br/>score, streak, loop]
        scheduler[scheduler<br/>trait: next item, record result]
        input[input<br/>romaji → kana, normalize]
        ui[ui<br/>ratatui widgets, display width]
    end

    subgraph modules[Modules]
        registry[registry<br/>the only list of modules]
        vocab[vocabulary]
        kanji[kanji]
        grammar[grammar<br/>conjugation, particles,<br/>multiple choice]
    end

    subgraph content[Embedded content]
        cv[(content/vocab.toml)]
        ck[(content/kanji.toml)]
        cg[(content/grammar.toml)]
    end

    progress[(progress file<br/>OS data dir)]

    app --> session
    app --> ui
    session --> scheduler
    session --> input
    session --> registry
    registry --> vocab & kanji & grammar
    vocab --> cv
    kanji --> ck
    grammar --> cg
    scheduler -. later .-> progress

    classDef future stroke-dasharray: 5 5
    class progress future
```

- **Scheduler:** the first version is a shuffle that shows missed items more often,
  in memory only. Spaced repetition and saved progress come later (dashed).
- **Content** is embedded in the binary with `include_str!`, so the release stays
  a single file. Sources and licenses go in `ATTRIBUTION.md`.

## 4. Module contract

```mermaid
classDiagram
    class StudyModule {
        <<trait>>
        +id() str
        +name() str
        +items(level) Vec~Item~
        +prompt(item) Prompt
        +check(item, answer) Verdict
    }
    class Item {
        +id: stable key, e.g. kanji:日
        +level: e.g. N5
    }
    class Prompt {
        +text
        +hint: optional furigana
        +choices: optional
    }
    class Verdict {
        <<enum>>
        Correct
        Close
        Wrong(expected)
    }
    StudyModule <|.. Vocabulary
    StudyModule <|.. Kanji
    StudyModule <|.. Grammar
    StudyModule ..> Item
    StudyModule ..> Prompt
    StudyModule ..> Verdict
```

**Adding a module** = one new file in `modules/`, one line in the registry, and one
content file. Nothing else changes.

## Build order

1. Session engine, Menu → Session → Summary, romaji input, and **Vocabulary**.
2. **Kanji** (reuses the typed-answer flow: reading / meaning).
3. **Grammar** (exercise types: conjugation, particle fill-in, multiple choice).
4. Saved progress and spaced repetition.
