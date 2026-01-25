# Tree

A simple command-line utility written in Rust to visualize directory structures.

## Features

* **Recursive Visualization**: Displays a hierarchical tree of files and directories.
* **Nerd Fonts Support**: Displays icons based on file types and extensions (e.g., Rust, Python, Markdown) for a more intuitive and aesthetic output.
* **Clean Output**: Uses standard tree characters (`├──`, `└──`, `│`) for clear formatting.
* **File Size Display**: Supports the `-s` or `--size` flag to show formatted file sizes alongside names.
* **Depth Control**: Supports the `-d` or `--depth` flag to limit the depth of the directory traversal.
* **Filtering**: Supports the `-e` or `--exclude` flag to ignore specific files or directories. By default, hidden files (starting with `.`) are ignored unless the `-a` flag is used.

## Requirements

* **Nerd Fonts**: To display file icons correctly, ensure your terminal is configured to use a [Nerd Font](https://www.nerdfonts.com/) (e.g., *FiraCode Nerd Font* or *JetBrainsMono Nerd Font*).

## Usage

You can run the utility using `cargo run --` followed by the path and desired options:

```bash
cargo run -- [PATH] [OPTIONS]

```

### Arguments & Options

| Option | Description | Example |
| --- | --- | --- |
| `PATH` | Specifies the target path (defaults to `.`) | `cargo run -- ./src` |
| `-a, --all` | Displays hidden files and directories | `cargo run -- -a` |
| `-s, --size` | Displays formatted file sizes | `cargo run -- -s` |
| `-d, --depth <NUM>` | Limits the depth of the tree traversal | `cargo run -- -d 2` |
| `-e, --exclude <NAME>` | Excludes specific directory or file names | `cargo run -- -e target .git` |

## Implementation Details

* **Language**: Rust (2024 edition).
* **Argument Parsing**: Uses the `clap` crate for robust command-line flag and argument handling.
* **Standard Library**: Utilizes `std::fs` and `std::path` for efficient file system traversal.

## Future Improvements

* Support saving the output directly to a text file.
* Implement color coding to distinguish between different file types and permissions.
* Expand the icon mapping for more specific file formats.