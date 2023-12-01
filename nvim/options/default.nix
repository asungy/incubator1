let
  listchars = "precedes:<,extends:>,tab:>-,trail:_";
  status_fmt = ""
    + "%n "  # buffer number
    + "%m "  # modified flag
    + "%r "  # readonly flag
    + "%y "  # file type
    + "%F "  # absolute full file path
    + "%= "  # separation point
    + "L:%l" # current line number
    + "/%L " # number of lines in buffer
    + "C:%v" # column number
    ;
in
{
  autoindent = true;               # copy indent from current line when creating new line
  backup = false;                  # create backup file
  breakindent = true;              # indent wrapped lines
  clipboard = "unnamedplus";       # use clipboard register '+'
  cmdheight = 0;                   # number of lines used for the command line
  completeopt = [                  # show completion menu
    "menuone"
    "noselect"
  ];
  confirm = true;                  # raise dialog for confirmation
  cursorline = false;              # highlight text line of cursor
  expandtab = true;                # use appropriate number of spaces to insert a <Tab>
  fileencoding = "utf-8";          # encoding written to a file
  hlsearch = true;                 # highlight all matches on previous search pattern
  hidden = true;                   # do not unload abandoned buffers
  ignorecase = false;              # ignore case in search patterns
  icon = true;                     # sets text to window title
  iconstring = "";                 # used for icon text of window
  keymap = "";                     # set keyboard mapping for insert and search modes.
  laststatus = 2;                  # value option to select when last window will have status line
                                   # (2: always)
  list = true;                     # set list mode. useful for seeing trailing whitespaces
  listchars = listchars;           # string used for list mode
  makeprg = "make";                # program used for ":make" command
  matchpairs = [                   # characters that form pairs for '%' command
    "(:)"
    "[:]"
    "{:}"
  ];
  menuitems = 25;                  # max number of items to use in a menu
  mouse = "a";                     # enable mouse support
  number = true;                   # display line numbers
  numberwidth = 4;                 # line number column width
  pumblend = 10;                   # pseudo-transparency for popup-menu
  pumheight = 10;                  # max number of items to show in popup-menu
  relativenumber = true;           # show line number relative to cursor line
  scrolloff = 10;                  # min number of lines to keep above and below cursor
  shell = "/bin/bash"; # name of shell to use for :! commands
  shiftwidth = 2;                  # number of spaces to use for each step of (auto)indent
  showbreak = "+++ ";              # string to put at start of lines that have been wrapped
  showtabline = 0;                 # when to show tabline (0: never)
  smarttab = true;                 # enable smart tab
  smartindent = true;              # enable smart indent
  splitbelow = true;               # splitting window will put new window below the current one
  splitright = true;               # splitting window will put new window to right of current one
  statusline = status_fmt;         # set status line format
  swapfile = false;                # use swapfile for buffer
  tabstop = 2;                     # number of spaces that <Tab> counts for
  termguicolors = true;            # enable 24-bit RGB color
  wildmenu = true;                 # enable "enhanced mode" of command-line completion
  winbar = "";                     # set winbar format
  wrap = true;                     # set text wrap
  wrapmargin = 0;                  # number of characters from window border before wrapping starts
}
