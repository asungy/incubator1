{
  description = "I don't know what this is yet.";

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
        frontend-name = "client";
        backend-name = "server";
        shell-hook = shell-name: ''
          PS1="\n\[\033[01;32m\]${name}(${shell-name}) >\[\033[00m\] "
        '';
      in
      {
        devShells.${frontend-name} = with pkgs; mkShell {
          buildInputs = [
            (rust-bin.selectLatestNightlyWith (toolchain: toolchain.default.override {
              targets = [ "wasm32-unknown-unknown" ];
            }))

            deno
            wasm-pack
            rust-analyzer
          ];

          shellHook = shell-hook frontend-name;
        };

        devShells.${backend-name} = with pkgs; mkShell {
          buildInputs = [
            cargo-expand
            rust-analyzer
            rust-bin.nightly.latest.default
          ];

          shellHook = shell-hook backend-name;
        };
      }
    );
}
