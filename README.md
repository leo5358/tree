# Tree

A simple command-line utility written in Rust to visualize directory structures.

## Features

* **Recursive Visualization**: Displays a hierarchical tree of files and directories.
* **Clean Output**: Uses standard tree characters (`├──`, `└──`, `│`) for clear formatting.
* **File Size Display**: Supports the `-s` flag to show formatted file sizes alongside names.
* **Depth Control**: Supports the `-d` flag to limit the depth of the directory traversal.
* **Custom Path**: Supports the `-p` flag to specify a target directory instead of just the current one.

## Usage

You can run the utility using `cargo run --` followed by the desired options:

```bash
cargo run -- [OPTIONS]

```

### Arguments

| Option | Description | Example |
| --- | --- | --- |
| `-p, --path <PATH>` | Specifies the target path (defaults to `.`) | `cargo run -- -p ./src` |
| `-s, --size` | Displays the size of files and directories | `cargo run -- -s` |
| `-d, --depth <NUMBER>` | Limits the depth of the tree traversal | `cargo run -- -d 2` |

## Implementation Details

* **Language**: Rust (2024 edition).
* **Argument Parsing**: Uses the `clap` crate for robust command-line flag and argument handling.
* **Standard Library**: Utilizes `std::fs` and `std::path` for efficient file system traversal.

## Future Improvements

* Add support for excluding specific files or directories (e.g., `.git` or `target`).
* Support saving the output directly to a text file.
* Implement color coding to distinguish between files and directories.
