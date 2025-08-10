# llama.cpp manager

This program is an utility for managing llama.cpp tools and models.

## Features

- Managing llama.cpp installation (pulling, building, installing)
- Providing aliases for llama.cpp utilities (llama-server, llama-quantize, etc.)
- Indexing available models and quants
- Running and managing llama-server instances (similarly to ollama)

## How does it work

`llama-mgr` is a simple way of managing `llama.cpp` installation, tools that it offers,
and models on current system - similarly to `ollama`.

The difference between `ollama` and `llama-mgr` is the fact that `ollama` bundles the inference engine,
while `llama-mgr` does not.
This allows `llama-mgr` to work with any (recent) `llama.cpp` build, giving you access to bleeding-edge
functionality and best possible performance.
It also provides some functionality that `ollama` does not, for example tokenization.

Any recent `llama.cpp` version should work fine, the same applies to forks.
Note that if the fork provides additional functionality to the tools, that functionality won't be supported.
Build process, however, can be customized.

`llama-mgr` can work either as a command-line application, or a daemon that exposes it's functionality via HTTP API.
HTTP API is based on [`ollama` API](https://ollama.readthedocs.io/en/api/#parameters), so `llama-mgr` is technically ollama-compatible.
Both command-line interface and API provide the same functionality.

## Prerequisites

- `uv` for managing Python
- `cmake` and `ninja` for building `llama.cpp`
- C++ toolchain for the platform of your choice
  - For CPU inference, any modern version of GCC/Clang/MSVC should be fine
  - For AMD GPU inference, ROCm is required
  - For NVIDIA GPU inference, CUDA is required
  - For other platforms, consult `llama.cpp` documentation.

## Command-line interface

Usage:

`llama-mgr <command> <command-args> [-h] [--data-dir/-d <config-file>] [--instance/-i <name>]`

`-h` forces `llama-mgr` to print help and immediately exit.

`--data-dir/-d` forces `llama-mgr` to use non-default data directory and configuration file, or create a new one.
Default data directory is `$HOME/.llama-mgr`, referred to as `$LLAMA_MGR_DIR`.
Configuration file is in `$LLAMA_MGR_DIR/config.toml`.

`--instance/-i` makes `llama-mgr` use specified `llama.cpp` instance. `global` is the default one.

Available commands:

- `install` - download and install `llama.cpp`
- `uninstall` - uninstall `llama.cpp`
- `set-instance` - set active instance of `llama.cpp`
- `quantize` - run `llama-quantize`
- `convert` - convert a raw huggingface model to GGUF
- `server` - run and manage `llama-server` instances
- `daemon` - start the `llama-mgr` in daemon mode

Each command may accept additional arguments.

### Commands

#### `install`

This command can be used to download, build and install `llama.cpp`.
It does that in few steps.

Before doing anything, `llama-mgr` verifies that the host environment has all the prerequisites installed.
If not, the command immediately fails with appropriate message.

All the data for `llama.cpp` instance is stored in configurable installation directory, by default
`$LLAMA_MGR_DIR/instances/$INSTALL_NAME`, where `$INSTALL_NAME` is the name of `llama.cpp` instance.
This directory will be referred to as `$LLAMA_INSTANCE_DIR`.

First, it pulls the source code of `llama.cpp` from remote Git server (along with all the submodules) into the `/repo`
subdirectory of install dir (`$LLAMA_INSTANCE_DIR/repo`).
The URL of repository and branch to be checked out is configurable.
By default it checks out `master` branch of [official `llama.cpp` repository](https://github.com/ggml-org/llama.cpp).
If the repository has already been pulled, it's updated before proceeding.

If `--pull-only` option is specified, the process ends here.

Next, `cmake` is called with arguments gathered from defaults, configuration file and CLI, to generate the build files
in `$LLAMA_INSTANCE_DIR/build`.
Default arguments specify build type to `Release` and install prefix to `$LLAMA_INSTANCE_DIR/install`.
User can manually specify additional arguments via CLI or configuration file.
Building for specific architecture via `--arch` adds the arguments for that architecture from configuration file.

Afterwards, `llama.cpp` is built and installed with `cmake`.

When the binaries are installed, unless `--ignore-python` argument is specified, `llama-mgr` will use `uv` to install
latest recommended version of Python and create virtual environment in `$LLAMA_INSTANCE_DIR/venv`.
Then, it will install all the dependencies for `llama.cpp` and check if conversion scripts can be successfully called.

This command accepts following additional arguments:

- `--pull-only` - Do not trigger installation, only download the repo.
- `--update-only` - Prevents installing llama.cpp if it's not installed already,
                    only updates existing installation.
- `--ignore-python` - Skips the Python environment configuration.
                      Note that it will make conversion scripts unavailable.
- `--repo-url [url]` - Use custom repository, instead of [official one](https://github.com/ggml-org/llama.cpp).
- `--branch [name]` - Use custom branch, instead of `master`.
- `--cmake-args [arguments]` - Add custom arguments to `cmake` for building `llama.cpp`.
                               It's strongly recommended to put the argument list in double-quotes.
- `--arch [architecture]` - Specify the architecture to build for. Available choices: `cpu`, `rocm`. (TODO: NVIDIA/Intel).
                            Using this arguments makes `llama-mgr` add arch-specific arguments to CMake call.
- `--parallel [n]` - Specify the amount of threads to use for building, uses 1 thread per CPU core by default.

#### `uninstall`

Removes current instance of `llama.cpp` by deleting it's directory from `llama-mgr`'s data dir.

#### `quantize`

Calls `llama-quantize` from `llama.cpp` instance to quantize a GGUF file.
If it's called without arguments, it lists available quantization types.

This command accepts following additional arguments:

- `--input [path]` - Path to the input GGUF file
- `--output [path]` - Path to the output GGUF file
- `--quant [type]` - Quantization type.

#### `convert`

Calls `convert_hf_to_gguf.py` script to convert a HuggingFace model to GGUF file.

This command accepts following additional arguments:

- `--input [path]` - Path to the directory with HuggingFace model
- `--output [path]` - Path to the directory with output GGUF file

#### `server`

Starts `llama-server` instance.

This command forwards it's arguments (except consumed ones, like `--instance`) to `llama-server`.

#### `daemon`

TODO

## HTTP API

HTTP API is based on [`ollama` API](https://ollama.readthedocs.io/en/api/#parameters)

TODO custom endpoints/differences
