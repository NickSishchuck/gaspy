# Gaspy

A command-line utility for reading file contents and counting lines in files and directories.

## Overview

Gaspy is a lightweight Rust application that provides file reading and line counting capabilities. It can:

- Display file contents (similar to Unix `cat` command)
- Count non-empty lines in files and directories
- Process directories recursively
- Exclude specific files or directories from processing

## Installation

### Prerequisites

- Rust and Cargo (latest stable version recommended)

### Building from Source

1. Clone the repository:
   ```
   git clone https://github.com/yourusername/gaspy.git
   cd gaspy
   ```

2. Build the project:
   ```
   ./src/build.sh
   ```

This will create an executable named `gaspy` in the root directory of the project.

## Usage

### Reading Files

To read and display the contents of one or more files:

```
./gaspy path/to/file1.txt path/to/file2.txt
```

### Counting Lines

To count non-empty lines in one or more files:

```
./gaspy -c path/to/file1.txt path/to/file2.txt
```

When multiple files are provided, a total count will also be displayed.

### Processing Directories

To count lines recursively in directories:

```
./gaspy -c -r path/to/directory
```

### Excluding Files/Directories

To exclude specific files or directories by name:

```
./gaspy -c -r -e node_modules -e .git path/to/directory
```

## Command Line Options

- `-c, --count`: Count non-empty lines instead of displaying file contents
- `-r, --recursive`: Process directories recursively
- `-e, --exclude <NAME>`: Exclude files/directories with the specified name (can be used multiple times)

## Error Handling

If a file cannot be opened or read, Gaspy will display an error message and continue processing any remaining files.

## Development

### Project Structure

- `src/main.rs` - Main application code
- `src/build.sh` - Build script for compiling the project

### Building

To build the project manually:

```
cargo build --release
```

The executable will be located at `target/release/gaspy`.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
