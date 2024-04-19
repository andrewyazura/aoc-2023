{
  description = "aoc-2023 rust env";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system overlays; };
        overlays = [ (import rust-overlay) ];
      in with pkgs; {
        devShells.default = mkShell {
          buildInputs = [
            openssl
            pkg-config
            (rust-bin.stable.latest.default.override {
              extensions = [ "rust-src" "rust-analyzer" ];
            })
          ];

          shellHook = ''
            export PATH="$HOME/.cargo/bin:$PATH"
          '';
        };
      });
}
