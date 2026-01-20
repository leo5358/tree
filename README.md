# Tree

A simple command-line utility written in Rust to visualize directory structures.

## Features

* **Recursive Visualization**: Displays a hierarchical tree of files and directories.
* **Clean Output**: Uses standard tree characters (`├──`, `└──`, `│`) for clear formatting.

## Usage

Currently, the tool is configured to scan the current directory by default.

To run the application:

```bash
cargo run
```

## Implementation Details

* **Language**: Rust (2024 edition).
* **Standard Library**: Utilizes `std::fs` and `std::path` for file system traversal.

## Future Improvements

* Add support for specifying a target directory via command-line arguments.