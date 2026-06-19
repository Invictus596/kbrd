```
$$       $$\                       $$\ 
$$ |      $$ |                      $$ |
$$ |  $$\ $$$$$$$\   $$$$$$\   $$$$$$$ |
$$ | $$  |$$  __$$\ $$  __$$\ $$  __$$ |
$$$$$$  / $$ |  $$ |$$ |  \__|$$ /  $$ |
$$  _$$<  $$ |  $$ |$$ |      $$ |  $$ |
$$ | \$$\ $$$$$$$  |$$ |      \$$$$$$$ |
\__|  \__|\_______/ \__|       \_______|
```

# kbrd

A terminal-based typing tutor inspired by [Keybr](https://www.keybr.com/), built with Rust and Ratatui.

Features an adaptive algorithm that targets your weakest keystrokes, all wrapped in a modern, low-latency TUI.

## Features

- **Adaptive training** — Markov-chain-based text generation that weights letters by your historical error rate (coming soon)
- **Keyboard heatmap** — visual feedback showing which keys need the most work (coming soon)
- **Low-latency** — built for linear mechanical switches; sub-16ms frame budget
- **Aesthetic TUI** — deep cool-gray palette with soft blue accents, rounded borders, true color support

### Phase 2 (current)

- [x] Project scaffold with Ratatui + Crossterm
- [x] Three-zone layout: Header / Typing Arena / Footer
- [x] Graceful exit via `Ctrl+C` or `Esc`
- [x] Static placeholder text: "The quick brown fox jumps over the lazy dog."
- [x] Character-level event capture & per-key timing
- [x] Per-key accuracy tracking and heatmap
- [x] Results screen with WPM, accuracy, per-key breakdown
- [x] Sentence pool with random selection on restart

## Getting started

```bash
cargo run --release
```

Requires a terminal with true color (24-bit) support. Most modern terminal emulators support this out of the box.

## Roadmap

| Phase | Focus |
|-------|-------|
| 1 | Project skeleton & TUI layout |
| 2 | Character-level event capture & per-key timing ✓ |
| 3 | Adaptive algorithm (Markov chains, error weighting) |
| 4 | Keyboard heatmap rendering ✓ (basic) |
| 5 | Session stats, progress graphs, word/sentence modes |

## Tech stack

| Layer | Choice |
|-------|--------|
| Language | Rust |
| TUI framework | [Ratatui](https://ratatui.rs) + [Crossterm](https://github.com/crossterm-rs/crossterm) |
| Error handling | `color-eyre` |

## License

MIT
