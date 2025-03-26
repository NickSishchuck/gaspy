# Gaspy

A simple command-line utility for reading and displaying file contents or standard input.

## Overview

Gaspy is a lightweight Rust application that reads and displays the contents of files or standard input. It's similar to the Unix `cat` command, allowing you to quickly view file contents or process piped input.


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

### Processing Standard Input

To read from standard input (piped or direct):

```
echo "Hello, World!" | ./gaspy
```

Or interactively:

```
./gaspy
Type something here...
```

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
