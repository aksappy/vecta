# Vecta - A powerful desktop file indexing and searching tool

## Features

** Text file indexing and searching **
** Global and local directory indexing **
** File type detection and categorization **
** File metadata extraction and storage **
** Full text search **


## Installation
### Binary download
1. Download the latest release from [GitHub](https://github.com/vecta/vecta/releases).
2. Extract the archive and move the `vecta` binary to a directory in your PATH.

# Vecta CLI Usage Guide

Vecta is a command-line tool for indexing and searching source code.

## Installation

To use Vecta, compile the Rust project and ensure the binary is available in your system path.

```sh
cargo build --release
mv target/release/vecta /usr/local/bin/
```

## Commands

### Initialize Vecta

```sh
vecta init <directory>
```

Initializes the Vecta environment in the specified directory by creating a `.vecta` directory with `config`, `data`, and `logs` subdirectories.

Example:

```sh
vecta init ~/projects/myrepo
```

### Index a Directory

```sh
vecta index <directory> [--global | -g]
```

Indexes the specified directory for searching. Use `--global` or `-g` to generate a global index.

Example:

```sh
vecta index ~/projects/myrepo
vecta index ~/projects/myrepo -g
```

### Search the Index

```sh
vecta search <query>
```

Searches for the given query in the indexed directories.

Example:

```sh
vecta search "TODO"
```

### List Indexed Directories

```sh
vecta list
```

Displays all directories that have been indexed.

### Remove an Indexed Directory

```sh
vecta remove [directory]
```

Removes a specific directory from the index. If no directory is specified, all indexed directories are removed.

Example:

```sh
vecta remove ~/projects/myrepo
vecta remove  # Removes all indexed directories
```

### Destroy Vecta Data

```sh
vecta destroy <directory>
```

Completely removes the `.vecta` directory and all its data from the specified directory.

Example:

```sh
vecta destroy ~/projects/myrepo
```

### Display Version

```sh
vecta version
```

Prints the version of Vecta installed.

Example:

```sh
vecta version
```

---

### Notes
- **Destructive Operations**: The `destroy` command permanently deletes data and cannot be undone.
- **Search Performance**: The speed of search depends on the number of indexed files.


❤️ Built using Rust ❤️
