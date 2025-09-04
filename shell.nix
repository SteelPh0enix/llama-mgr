{
  pkgs ? import <nixpkgs> { },
}:
with pkgs;
let
  dev-packages = with pkgs; [
    # generic
    bashInteractive

    # rust
    cargo
    rustc

    # llama.cpp core
    cmake
    git
    ninja
    uv
    gcc
    clang

    # llama.cpp optionals
    openmpi
    blas
    vulkan-headers
    vulkan-tools
    vulkan-extension-layer
    vulkan-validation-layers
    vulkan-utility-libraries
  ];
in
mkShell {
  buildInputs = dev-packages;
}
