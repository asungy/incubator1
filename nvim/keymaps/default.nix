let
  options = desc: {
    noremap = true;
    silent = true;
    desc = desc;
  };
in
[
  {
    action = "k<S-j>";
    key = "<S-k>";
    mode = "n";
    options = options "Join current line with the one below it";
  }
  {
    action = "<gv";
    key = "<";
    mode = "v";
    options = options "Stay in indent-mode when left-indenting in visual mode";
  }
  {
    action = ">gv";
    key = ">";
    mode = "v";
    options = options "Stay in indent-mode when right-indenting in visual mode";
  }
  {
    action = ":m .+1<CR>==";
    key = "<A-j>";
    mode = "x";
    options = options "Move text down in visual block mode";
  }
  {
    action = ":m .-2<CR>==";
    key = "<A-k>";
    mode = "x";
    options = options "Move text up in visual block mode";
  }
  {
    action = "<C-w>h";
    key = "<C-h>";
    mode = "n";
    options = options "Move to left-adjacent window";
  }
  {
    action = "<C-w>j";
    key = "<C-j>";
    mode = "n";
    options = options "Move to bottom-adjacent window";
  }
  {
    action = "<C-w>k";
    key = "<C-k>";
    mode = "n";
    options = options "Move to top-adjacent window";
  }
  {
    action = "<C-w>l";
    key = "<C-l>";
    mode = "n";
    options = options "Move to right-adjacent window";
  }
  {
    action = "gt";
    key = "<S-l>";
    mode = "n";
    options = options "Move to right tab";
  }
  {
    action = "gT";
    key = "<S-h>";
    mode = "n";
    options = options "Move to left tab";
  }
  {
    action = "<Nop>";
    key = "<Space>";
    mode = "";
    options = options "Unmap space key (which is used as the leader key)";
  }
  {
    action = "<CMD>vsplit<CR>";
    key = "<leader>vs";
    mode = "n";
    options = options "Vertical window split";
  }
  {
    action = "<CMD>tabnew<CR>";
    key = "<leader>nt";
    mode = "n";
    options = options "New tab";
  }
  {
    action = "<CMD>terminal<CR>";
    key = "<leader>tr";
    mode = "n";
    options = options "Open terminal";
  }
  {
    action = "<CMD>NvimTreeToggle<CR>";
    key = "<C-n>";
    mode = "n";
    options = options "Open file explorer";
  }
]
