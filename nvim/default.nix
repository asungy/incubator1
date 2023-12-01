{ pkgs, ... }:

{
  config =
  {
    globals.mapleader = " ";

    colorschemes.tokyonight.enable = true;

    options = import ./options;

    keymaps = import ./keymaps;

    plugins = {}
      // import ./plugins/nvim-tree.nix
      // import ./plugins/lualine.nix
      ;
  };
}
