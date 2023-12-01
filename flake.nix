{
  description = "I don't know what this is yet.";

  inputs = {
    nixpkgs.url      = "github:nixos/nixpkgs/nixpkgs-unstable";
    flake-utils.url  = "github:numtide/flake-utils";
		nixvim.url       = "github:nix-community/nixvim";
  };

  outputs = { self, nixpkgs, flake-utils, nixvim, } @ inputs:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };

				nvim = nixvim.legacyPackages.${system}.makeNixvimWithModule {
					inherit pkgs;
					module = import ./nvim;
				};

        name = "kodo";
        shell-hook = shell-name: ''
          PS1="\n\[\033[01;32m\]${name}(${shell-name}) >\[\033[00m\] "
        '';
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = [
						nvim
          ];

          shellHook = shell-hook "default";
        };
      }
    );
}
