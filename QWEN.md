# llama-mgr Project Documentation

## Overview

`llama-mgr` is a utility for managing llama.cpp tools, models, and server instances. It provides functionality similar to `ollama` but with more flexibility by not bundling the inference engine, allowing it to work with any recent `llama.cpp` build.

The project is implemented in Rust as a command-line application that can also run as a daemon exposing an HTTP API compatible with the `ollama` API.

## Key Features

- Managing llama.cpp installation (pulling, building, installing)
- Providing aliases for llama.cpp utilities (llama-server, llama-quantize, etc.)
- Indexing available models and quants
- Running and managing llama-server instances
- HTTP API compatible with ollama API

## Project Structure

```
llama-mgr/
├── Cargo.toml          # Rust project configuration
├── README.md           # Project documentation
├── src/
│   ├── main.rs         # Entry point
│   ├── commands/     # Command implementations
│   │   ├── install.rs  # Install command implementation
│   │   ├── uninstall.rs
│   │   ├── quantize.rs
│   │   ├── convert.rs
│   │   ├── server.rs
│   │   └── daemon.rs
│   └── external_tools/  # External tool management
│       ├── cmake.rs
│       ├── ninja.rs
│       ├── uv.rs
│       └── version.rs
└── target/           # Build artifacts
```

## Commands

### Install

Downloads, builds, and installs llama.cpp with the following options:
- `--pull-only`: Only download the repo without building
- `--update-only`: Update existing installation only
- `--ignore-python`: Skip Python environment setup
- `--repo-url`: Custom repository URL
- `--branch`: Custom branch to checkout
- `--cmake-args`: Additional CMake arguments
- `--arch`: Architecture to build for (cpu, rocm)
- `--parallel`: Number of threads for building

### Uninstall

Removes the specified llama.cpp installation instance.

### Quantize

Runs `llama-quantize` from llama.cpp to quantize GGUF files.

### Convert

Converts HuggingFace models to GGUF format using convert_hf_to_gguf.py script.

### Server

Starts and manages `llama-server` instances.

### Daemon

Starts the llama-mgr in daemon mode with HTTP API.

## Dependencies

- Rust 2024 edition
- CMake and Ninja for building
- C++ toolchain (GCC/Clang/MSVC)
- For GPU support: ROCm (AMD) or CUDA (NVIDIA)
- `uv` for Python package management

## Build Instructions

To build the project:

```bash
# Build the project
cargo build

# Build in release mode
cargo build --release
```

To run:

```bash
# Run with default command
./target/debug/llama-mgr

# Run with specific command
./target/debug/llama-mgr install
```

## Usage Examples

```bash
# Install llama.cpp
llama-mgr install --instance my-instance

# Convert a HuggingFace model to GGUF
llama-mgr convert --input /path/to/model --output /path/to/output

# Run server
llama-mgr server --instance my-instance

# Start daemon
llama-mgr daemon
```

## Architecture

The application follows a modular architecture with:
1. Main entry point in `src/main.rs` that parses commands
2. Command implementations in `src/commands/` directory
3. External tool management in `src/external_tools/` for handling cmake, ninja, uv, etc.
4. Common arguments shared across commands defined in `src/commands/mod.rs`

The project uses clap for command-line argument parsing and follows Rust best practices.