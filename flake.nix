{
  description = "llama-mgr development flake";
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.05";
  inputs.systems.url = "github:nix-systems/default";
  inputs.flake-utils = {
    url = "github:numtide/flake-utils";
    inputs.systems.follows = "systems";
  };

  outputs =
    { nixpkgs, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
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
      {
        devShells.default = pkgs.mkShell {
          buildInputs = dev-packages;
        };
      }
    );
}
