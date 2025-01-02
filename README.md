# JournalView

A lightweight, terminal-based application written in Rust for efficiently viewing, filtering, and navigating system logs from `journalctl`.

## Features

- **Log Viewing**: Access logs from `journalctl` logs.
- **Filtering**: Powerful filtering capabilities
- **Keyboard Navigation**: Intuitive hotkeys for seamless log exploration.
- **Customizable View**: Adjust the display for better readability.

## Installation

### Prerequisites

- **Rust**: Ensure Rust is installed. You can install Rust using [rustup](https://rustup.rs/).

### Clone the Repository

```bash
git clone https://github.com/codervijo/journalview.git
cd journalview
```

### Build the Application

```bash
cargo build --release
```

### Run the Application

```bash
./target/release/journalview
```

## Usage

### Basic Commands

- Launch the application:
  ```bash
  journalview
  ```

- Navigate logs using arrow keys or predefined hotkeys.

### Filtering Logs

- Press `/` to enter a search query (regex supported).
- Use `Tab` to switch between different log sources.

### Hotkeys

| Key        | Action                       |
|------------|------------------------------|
| `Arrow Up` | Scroll up in the log list    |
| `Arrow Down` | Scroll down in the log list |
| `Enter`    | Select a log entry           |
| `/`        | Start a search               |
| `q`        | Quit the application         |

## Contributing

Contributions are welcome! Follow these steps to contribute:

1. Fork the repository.
2. Create a feature branch:
   ```bash
   git checkout -b feature-name
   ```
3. Commit your changes:
   ```bash
   git commit -m "Description of changes"
   ```
4. Push to your fork:
   ```bash
   git push origin feature-name
   ```
5. Open a Pull Request.

Refer to the `CONTRIBUTING.md` for more details.

## License

This project is licensed under the [MIT License](LICENSE).

## Acknowledgments

Inspired by the functionality of `journalctl` and enhanced by the capabilities of Rust for high-performance, terminal-based tools.
