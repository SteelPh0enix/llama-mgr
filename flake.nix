{
  description = "llama-mgr development flake";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.05";
    systems.url = "github:nix-systems/default";
    flake-utils = {
      url = "github:numtide/flake-utils";
      inputs.systems.follows = "systems";
    };
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      nixpkgs,
      flake-utils,
      rust-overlay,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        dev-packages = with pkgs; [
          # generic
          bashInteractive

          # rust
          (rust-bin.stable.latest.default.override {
            extensions = [
              "cargo"
              "rust-analysis"
              "rust-src"
              "rust-std"
              "rustc"
              "rustfmt"
            ];
          })

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
      {
        devShells.default = pkgs.mkShell {
          buildInputs = dev-packages;
        };
      }
    );
}
