{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url      = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url  = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        name = "kodo";
      in
      {
        # NOTE: THIS CURRENTLY DOES NOT WORK AFTER PROJECT HAS BEEN REFACTORED.
        defaultPackage = with pkgs; stdenv.mkDerivation {
          inherit name;
          src = self;
          buildInputs = [
            pkgs.rust-bin.stable.latest.default
          ];
          buildPhase = ''
            cargo build --release
          '';
          installPhase = ''
            mkdir -p $out/bin
            cp ./target/release/${name} $out/bin
          '';
        };

        devShells.default = with pkgs; mkShell {
          buildInputs = [
            cargo-expand
            rust-analyzer
            rust-bin.stable.latest.default
          ];

          shellHook = ''
            PS1="\n\[\033[01;32m\]${name}-nix >\[\033[00m\] "
          '';
        };
      }
    );
}
