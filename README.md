# llama.cpp manager

This program is an utility for managing llama.cpp tools and models.

## Features

- Managing llama.cpp installation (pulling, building, installing)
- Providing aliases for llama.cpp utilities (llama-server, llama-quantize, etc.)
- Indexing available models and quants
- Running and managing llama-server instances (similarly to ollama)

## How does it work

`llama-mgr` is a simple way of managing `llama.cpp` installation, tools that it offers, and models on current system - similarly to `ollama`.
The difference between `ollama` and `llama-mgr` is the fact that `ollama` bundles the inference engine, while `llama-mgr` does not.
This allows `llama-mgr` to work with any (recent) `llama.cpp` build, giving you access to bleeding-edge functionality and best possible performance.
It also provides some functionality that `ollama` does not, for example tokenization.

Any recent `llama.cpp` version should work fine, the same applies to forks.
Note that if the fork provides additional functionality to the tools, that functionality won't be supported.
Build process, however, can be customized.

`llama-mgr` can work either as a command-line application, or a daemon that exposes it's functionality via HTTP API.
HTTP API is based on [`ollama` API](https://ollama.readthedocs.io/en/api/#parameters), so `llama-mgr` is technically ollama-compatible.
Both command-line interface and API provide the same functionality.

## Command-line interface

TODO

## HTTP API

HTTP API is based on [`ollama` API](https://ollama.readthedocs.io/en/api/#parameters)

TODO custom endpoints/differences
