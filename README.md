# JournalView

A fast TUI for journalctl — pick a unit, scroll the logs.

<!-- TODO: replace with demo.gif or asciinema cast -->

[![crates.io](https://img.shields.io/crates/v/journalview.svg)](https://crates.io/crates/journalview)
[![CI](https://github.com/codervijo/journalview/actions/workflows/rust.yml/badge.svg)](https://github.com/codervijo/journalview/actions/workflows/rust.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Downloads](https://img.shields.io/crates/d/journalview.svg)](https://crates.io/crates/journalview)

![jview1](https://github.com/user-attachments/assets/78a97592-1c5e-4f34-ba42-dc8bf5d8b573)

## Why JournalView?

`journalctl` is powerful but pages of text are awkward to navigate, and remembering `-u <unit>` for every service gets old fast. JournalView is a lightweight terminal UI written in Rust on top of [Ratatui](https://github.com/ratatui/ratatui): it lists your systemd service units in a side panel, lets you pick one with the keyboard, and shows that unit's journal in a scrollable pane next to it. If no unit is selected it falls back to `journalctl --since=yesterday`. No daemon, no config — just a binary that shells out to `journalctl` and `systemctl`.

## Features

- **Three-pane layout** — left: systemd service units; top right: search box; bottom right: log view; bottom: help line.
- **Unit-scoped log view** — pick a `.service` unit with `Enter` and the log pane refreshes with `journalctl -u <unit>`.
- **Pure keyboard navigation** — `Tab` cycles panes, arrow keys scroll vertically and horizontally inside each pane.
- **No config to write** — works out of the box on any host where `journalctl` and `systemctl` are on `$PATH`.
- **Search box** — `TODO/WIP`: typing in the search pane is captured but does not yet filter the log view. Tracked in [ROADMAP.md](ROADMAP.md).

## Comparison

> Cells marked `TODO` need confirmation against current upstream behavior — see issue tracker.

| Capability                          | JournalView          | `journalctl`            | [`lnav`](https://lnav.org)       | [`lazyjournal`](https://github.com/Lifailon/lazyjournal) |
|-------------------------------------|----------------------|-------------------------|----------------------------------|----------------------------------------------------------|
| Interactive TUI                     | Yes                  | No (pager only)         | Yes                              | Yes                                                      |
| Reads systemd journal               | Yes (shells out)     | Yes (native)            | TODO                             | TODO                                                     |
| Built-in unit picker                | Yes                  | No                      | TODO                             | TODO                                                     |
| Live tail / follow                  | No                   | Yes (`-f`)              | TODO                             | TODO                                                     |
| Regex search inside the TUI         | No (WIP)             | Yes (`-g`)              | TODO                             | TODO                                                     |
| Reads arbitrary log files           | No                   | No                      | TODO                             | TODO                                                     |
| Single static binary                | Yes (Rust)           | n/a (ships with systemd)| TODO                             | TODO                                                     |

## Installation

### Prebuilt binary (Linux)

```bash
curl -sL https://github.com/codervijo/journalview/raw/main/install.sh | bash
```

This installs to `~/.local/bin/journalview`. Make sure `~/.local/bin` is on your `$PATH`.

### From crates.io

```bash
cargo install journalview
```

### From source

Ensure Rust is installed (see [rustup.rs](https://rustup.rs/)), then:

```bash
cargo install --git https://github.com/codervijo/journalview.git journalview
```

### Other package managers

- **Homebrew** — TODO
- **Arch (AUR)** — TODO
- **Standalone release binaries** — see [the releases page](https://github.com/codervijo/journalview/releases) (added by the release workflow on tag push).

### Requirements

JournalView shells out to `journalctl` and `systemctl`, so it expects a Linux host with systemd. It does not need root, but it can only show journal entries the current user is allowed to read.

## Usage

Launch:

```bash
journalview
```

The UI opens with the **Logs** pane focused, showing `journalctl --since=yesterday`. `Tab` cycles between **Logs → Selector → Search → Logs**. In the Selector pane, move the cursor with `Up`/`Down` and press `Enter` to filter the log pane by that service unit.

### Hotkeys

| Key            | Pane            | Action                                              |
|----------------|-----------------|-----------------------------------------------------|
| `Tab`          | any             | Cycle to the next pane                              |
| `q` / `Q`      | any             | Quit                                                |
| `Up` / `Down`  | Logs, Selector  | Scroll vertically                                   |
| `Left` / `Right` | Logs, Selector | Scroll horizontally                                |
| `Enter`        | Selector        | Filter the log pane by the highlighted service unit |
| `Up` / `Down`  | Selector        | Also clears the active unit filter                  |
| any character  | Search          | Append to search buffer (filtering not yet wired up — see ROADMAP) |
| `Backspace`    | Search          | Delete last character from the buffer               |
| `Enter` / `Esc`| Search          | Leave the Search pane                               |

## Contributing

Contributions are very welcome. See [CONTRIBUTING.md](CONTRIBUTING.md) for dev setup, the test loop, and the PR process, and have a look at [issues tagged `good first issue`](https://github.com/codervijo/journalview/labels/good%20first%20issue) for a starting point.

## License

This project is licensed under the [MIT License](LICENSE).

## Acknowledgements

Thanks to the following projects for inspiration:
- [Ratatui](https://github.com/ratatui/ratatui)
- [Lazyjournal](https://github.com/Lifailon/lazyjournal)

Inspired by the functionality of `journalctl` and enhanced by the capabilities of Rust for high-performance, terminal-based tools.
