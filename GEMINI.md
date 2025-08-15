# Project Overview

This project, `llama-mgr`, is a command-line utility written in Rust for managing `llama.cpp` installations. It
simplifies the process of downloading, building, and using `llama.cpp` tools like `llama-quantize` and `llama-server`.
The tool is designed to be a more flexible alternative to `ollama`, allowing users to work with the latest `llama.cpp`
builds.

The project uses the `clap` crate for command-line argument parsing and `git2` for managing `llama.cpp`'s Git
repository.

## Building and Running

To build the project, use the standard Rust build command:

```bash
cargo build --release
```

To run the application, use `cargo run` with the desired command. For example, to see the help message for the `install`
command:

```bash
cargo run -- install --help
```

### Testing

There are no tests in the project yet.

## Development Conventions

The project follows standard Rust conventions. The code is organized into modules by functionality, with a clear
separation of concerns. The `commands` module contains the logic for each of the CLI commands, and the `external_tools`
module appears to handle interactions with tools like `cmake` and `ninja`.

The code is well-documented with comments explaining the purpose of each module and command-line argument.

## Architecture

The project is designed around a modular architecture. The core logic is separated into two main modules: `commands` and
`external_tools`.

### External Tool Management

The `external_tools` module provides a generic interface for interacting with external command-line tools. This is
achieved through the `ExternalTool` trait, which defines a common set of methods for finding and interacting with these
tools.

The following external tools are currently managed:

- **cmake:** Used for building `llama.cpp`.
- **ninja:** Used as a build system for `llama.cpp`.
- **uv:** A Python package manager, used to create a virtual environment and install dependencies for `llama.cpp`'s
  Python scripts.

This design makes it easy to add support for new external tools in the future.
