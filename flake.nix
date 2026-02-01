{
  description = "Flake for romu";

  inputs = {
    nixpks.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    nixpkgs,
    rust-overlay,
    flake-utils,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        overlays = [(import rust-overlay)];
        pkgs = import nixpkgs {
          inherit system overlays;
          config.allowUnfree = true;
        };
        rust = pkgs.rust-bin.selectLatestNightlyWith (
          toolchain:
            toolchain.default.override {
              extensions = [
                "rust-src"
                "rust-analyzer"
                "miri"
              ];
              targets = ["x86_64-unknown-linux-gnu"];
            }
        );

        buildInputs = [
          rust
        ];
        tooling = with pkgs; [
          cargo-nextest
          cargo-mutants
        ];
      in
        with pkgs; {
          devShells.default = mkShell {
            buildInputs = buildInputs ++ tooling;
          };
        }
    );
}
