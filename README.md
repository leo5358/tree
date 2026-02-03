# Tree

A simple command-line utility written in Rust to visualize directory structures.

## Features

* **Recursive Visualization**: Displays a hierarchical tree of files and directories.
* **Nerd Fonts Support**: Displays icons based on file types and extensions (e.g., Rust, Python, Markdown) for a more intuitive and aesthetic output.
* **Colorized Output**: Highlighting to distinguish between different types of content:
* **Bold Blue**: Directories for better structural recognition.
* **Yellow**: Error messages and permission warnings.
* **Gray**: Tree connectors (`├──`) and metadata to keep the focus on filenames.


* **Clean Output**: Uses standard tree characters (`├──`, `└──`, `│`) for clear formatting.
* **File Size Display**: Supports the `-s` or `--size` flag to show formatted file sizes alongside names.
* **Depth Control**: Supports the `-d` or `--depth` flag to limit the depth of the directory traversal.
* **Filtering**: Supports the `-e` or `--exclude` flag to ignore specific files or directories. By default, hidden files (starting with `.`) are ignored unless the `-a` flag is used.

## Requirements

* **Nerd Fonts**: To display file icons correctly, ensure your terminal is configured to use a [Nerd Font](https://www.nerdfonts.com/) (e.g., *FiraCode Nerd Font* or *JetBrainsMono Nerd Font*).
* **ANSI Color Support**: A terminal emulator that supports ANSI escape sequences for color display.

## Installation

### From Source

You can install the binary directly from the source code using `cargo`:

```bash
git clone https://github.com/leo5358/tree.git
cd tree
cargo install --path .

```

This will compile the project and place the `tree` binary in your `~/.cargo/bin` directory. Make sure this directory is in your system's `PATH`.

## Usage

You can run the utility using the installed binary or `cargo run --` followed by the path and desired options:

```bash
tree [PATH] [OPTIONS]
# or
cargo run -- [PATH] [OPTIONS]

```

### Arguments & Options

| Option | Description | Example |
| --- | --- | --- |
| `PATH` | Specifies the target path (defaults to `.`) | `tree ./src` |
| `-a, --all` | Displays hidden files and directories | `tree -a` |
| `-s, --size` | Displays formatted file sizes | `tree -s` |
| `-d, --depth <NUM>` | Limits the depth of the tree traversal | `tree -d 2` |
| `-e, --exclude <NAME>` | Excludes specific directory or file names | `tree -e target .git` |

## Implementation Details

* **Language**: Rust (2024 edition).
* **Argument Parsing**: Uses the `clap` crate (v4.5) for robust command-line flag and argument handling.
* **File Traversal**: Utilizes the `ignore` crate for efficient filesystem walking with `.gitignore` support.

## Future Improvements

* Expand the icon mapping for more specific file formats.
* Implement parallel directory walking for improved performance on large filesystems.

