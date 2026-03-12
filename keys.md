# Tmux
<!-- prefix: Ctrl-s -->

## Prefix & Config
- `Ctrl-s r` - Reload tmux config
- `Ctrl-s ?` - Show all keybindings
- `Ctrl-s :` - Command prompt
- `Ctrl-s ]` - Paste from buffer
- `Ctrl-s t` - Show clock

## Panes
- `Ctrl-s h` - Select left pane
- `Ctrl-s j` - Select down pane
- `Ctrl-s k` - Select up pane
- `Ctrl-s l` - Select right pane
- `Ctrl-s H` - Resize left
- `Ctrl-s J` - Resize down
- `Ctrl-s K` - Resize up
- `Ctrl-s L` - Resize right
- `Ctrl-s z` - Zoom pane (fullscreen)
- `Ctrl-s q` - Show pane numbers
- `Ctrl-s x` - Kill pane
- `Ctrl-s o` - Cycle to next pane
- `Ctrl-s ;` - Last active pane
- `Ctrl-s {` - Swap pane left
- `Ctrl-s }` - Swap pane right
- `Ctrl-s Space` - Cycle pane layouts
- `Ctrl-s !` - Break pane into window

## Copy Mode (vi)
- `Ctrl-s Enter` - Enter copy mode
- `v` - Begin selection
- `y` - Copy and exit
- `H` - Start of line
- `L` - End of line
- `/` - Search forward
- `?` - Search backward
- `n` - Next match
- `N` - Previous match
- `g` - Top of buffer
- `G` - Bottom of buffer
- `Esc` - Cancel

## Sessions
- `Ctrl-s d` - Detach from session
- `Ctrl-s s` - Choose session (interactive)
- `Ctrl-s $` - Rename session
- `Ctrl-s (` - Previous session
- `Ctrl-s )` - Next session

## Windows
- `Ctrl-s |` - Split pane horizontal
- `Ctrl-s -` - Split pane vertical
- `Ctrl-s c` - New window (keep path)
- `Ctrl-s n` - Next window
- `Ctrl-s p` - Previous window
- `Ctrl-s <` - Swap window left
- `Ctrl-s >` - Swap window right
- `Ctrl-s w` - Choose window (interactive)
- `Ctrl-s f` - Find window by name
- `Ctrl-s ,` - Rename window
- `Ctrl-s &` - Kill window
- `Ctrl-s .` - Move/renumber window
- `Ctrl-s 0-9` - Jump to window by index
- `Ctrl-s i` - Window info

# Neovim
<!-- leader: , -->

## Files & Buffers
- `-` - Open Mini Files (at current file)
- `Ctrl-t` - Toggle Mini Files
- `Ctrl-p` - Find files (Telescope)
- `,<space>` - Find open buffers
- `,?` - Recently opened files
- `,/` - Fuzzy search in buffer

## Find (Telescope)
- `,ff` - Find files
- `,fg` - Live grep (project)
- `,fr` - Live grep (git root)
- `,fw` - Find word under cursor
- `,fo` - Grep in open files
- `,fb` - Find buffers
- `,fd` - Find diagnostics
- `,fh` - Find help tags
- `,fi` - Find files in git
- `,fa` - Resume last search
- `,fs` - Select telescope picker
- `,ft` - Find todo comments
- `,fu` - Find undo history
- `,fA` - Find symbol via Aerial

## Harpoon
- `,ha` - Add file to harpoon
- `,hh` - Toggle harpoon menu
- `,h1` - Harpoon file 1
- `,h2` - Harpoon file 2
- `,h3` - Harpoon file 3
- `,h4` - Harpoon file 4
- `,h5` - Harpoon file 5
- `,h6` - Harpoon file 6
- `,h7` - Harpoon file 7
- `,h8` - Harpoon file 8
- `,h9` - Harpoon file 9

## Go To (LSP)
- `gd` - Go to definition
- `gD` - Go to type definition
- `gr` - Go to references
- `gm` - Go to implementation
- `gs` - Document symbols
- `gS` - Workspace symbols
- `ge` - Go to error (float)
- `K` - Hover docs
- `gK` - Signature help

## LSP Actions
- `,la` - Code action
- `,lr` - Rename symbol
- `,li` - Incoming calls
- `,lu` - Outgoing calls
- `,lf` - Format buffer
- `,ll` - Lint buffer
- `,ld` - Diagnostics to quickfix
- `,lh` - Rust: hover actions
- `,lo` - Switch source/header (Clangd)

## Gitsigns
- `]h` - Next git hunk
- `[h` - Previous git hunk
- `,gs` - Stage hunk
- `,gu` - Undo stage hunk
- `,gp` - Preview hunk
- `,gd` - Diff this
- `,gb` - Toggle line blame

## Aerial (Symbol Outline)
- `,a` - Toggle aerial panel
- `{` - Previous symbol
- `}` - Next symbol

## Folds (nvim-ufo)
- `zR` - Open all folds
- `zM` - Close all folds
- `zK` - Peek fold under cursor

## Trouble (Diagnostics)
- `,xx` - Buffer diagnostics
- `,xX` - Workspace diagnostics
- `,xq` - Quickfix list (Trouble)

## Search & Replace (grug-far)
- `,sr` - Search & replace (project)
- `,sw` - Search word under cursor

## CopilotChat
- `,cc` - Toggle CopilotChat
- `,ce` - Explain selection
- `,cr` - Review selection
- `,cf` - Fix selection

## Obsidian
- `,of` - Find note
- `,og` - Grep notes
- `,on` - New note
- `,od` - Daily notes
- `,ob` - Backlinks
- `,ot` - Tags
- `,ol` - Follow link

## Todo Comments
- `]t` - Next todo
- `[t` - Previous todo

## Splits & Windows
- `,sv` - Split vertical
- `,sh` - Split horizontal
- `,se` - Equalize splits
- `,sx` - Close split
- `,sJ` - Shorter (height)
- `,sK` - Taller (height)
- `,sL` - Wider (width)
- `,sH` - Narrower (width)
- `,vh` - Vertical to horizontal
- `,hv` - Horizontal to vertical

## Window Swap (smart-splits)
- `,wh` - Swap buffer left
- `,wj` - Swap buffer down
- `,wk` - Swap buffer up
- `,wl` - Swap buffer right

## Pane Navigation (tmux-aware)
- `Ctrl-h` - Move to left pane
- `Ctrl-j` - Move to down pane
- `Ctrl-k` - Move to up pane
- `Ctrl-l` - Move to right pane
- `Ctrl-Shift-H` - Previous tab
- `Ctrl-Shift-L` - Next tab

## Movement
- `Ctrl-d` - Half page down (centered)
- `Ctrl-u` - Half page up (centered)
- `Ctrl-f` - Page forward (centered)
- `n` - Next search (centered)
- `N` - Prev search (centered)
- `s` - Leap forward
- `S` - Leap backward
- `J` - Append line to current (keep cursor)
- `J` - Move selection down (visual)
- `K` - Move selection up (visual)

## Terminal
- `Ctrl-\` - Open terminal split
- `jk` - Exit terminal mode
- `Esc` - Exit terminal mode
- `Ctrl-o` - Exit terminal (keep jump list)

## Session
- `,ps` - Save session
- `,pr` - Restore session
- `,px` - Delete session

## Testing
- `,tn` - Test nearest
- `,tf` - Test file

## Quickfix
- `]q` - Next quickfix
- `[q` - Previous quickfix
- `F8` - Open quickfix / next
- `Shift-F8` - Previous quickfix item

## Edit Config
- `,ev` - Edit vimrc
- `,ek` - Edit keymaps
- `,eo` - Edit options
- `,ep` - Edit plugins
- `,ec` - Edit config folder

## Misc
- `,z` - Toggle Zen mode
- `,cs` - Clear search highlight
- `,os` - Toggle scratch buffer
- `,m` - Copy messages
- `jk` - Escape (insert mode)
- `,p` - Paste over selection (visual, keep clipboard)

## Completion (nvim-cmp insert mode)
- `Ctrl-n` - Next suggestion
- `Ctrl-p` - Previous suggestion
- `Ctrl-Space` - Trigger complete
- `Ctrl-o` - Complete
- `Tab` - Confirm (replace)
- `Ctrl-CR` - Confirm (replace)
- `Ctrl-e` - Abort
- `Ctrl-b` - Scroll docs up
- `Ctrl-f` - Scroll docs down
- `Ctrl-j` - Next / expand snippet
- `Ctrl-k` - Prev / jump snippet back

## Surround (nvim-surround)
- `ys{motion}{char}` - Add surround
- `cs{old}{new}` - Change surround
- `ds{char}` - Delete surround
- `S{char}` - Surround selection (visual)

## GUI (Font Size)
- `Ctrl-=` - Increase font size
- `Ctrl--` - Decrease font size

# Tooling (from nvim repo)

## ripgrep (rg)
- `rg "<pattern>"` - Search recursively with smart defaults
- `rg -n "<pattern>"` - Search with line numbers
- `rg --files` - List files tracked by ripgrep
- `rg -g "*.lua" "<p>"` - Search only Lua files
- `rg -uu "<pattern>"` - Include hidden and ignored files

## fd
- `fd <name>` - Fast filename search
- `fd -e lua` - Find Lua files
- `fd -t f <name>` - Find files by name
- `fd --hidden` - Include hidden files (exclude `.git` as needed)

## fzf
- `fzf` - Interactive fuzzy picker
- `fd -t f | fzf` - Fuzzy-find files from fd output
- `rg -n "<p>" | fzf` - Fuzzy-pick grep matches
- `Ctrl-t` - PSFzf file/dir completion chord
- `Ctrl-r` - PSFzf reverse history search chord

## git
- `git status -sb` - Short status with branch info
- `git diff` - Show unstaged changes
- `git log --oneline` - Compact recent history
- `git grep "<pattern>"` - Search tracked files quickly

## Mason + LSP
- `:Mason` - Open Mason package manager UI
- `:MasonInstall <lsps>` - Install configured LSP servers
- `:LspInfo` - Show active LSP clients
- `:checkhealth` - Run Neovim health checks

## eza
- `eza` - Modern ls output
- `eza -lah` - Long listing with hidden files
- `eza -T -L 2` - Two-level tree view

## bat
- `bat <file>` - Syntax-highlighted file viewer
- `bat -n <file>` - Show line numbers
- `bat --style=plain` - Plain cat-like output

## zoxide
- `z <query>` - Jump to most-used matching directory
- `zi <query>` - Interactive jump using fzf
- `z <a> <b>` - Jump using multiple match terms

## pyenv
- `pyenv versions` - List installed Python versions
- `pyenv install <ver>` - Install Python version
- `pyenv global <ver>` - Set global Python version

## Node / npm provider
- `npm i -g neovim` - Install Neovim Node provider
- `npm ls -g --depth=0` - List globally installed npm packages
- `:checkhealth provider` - Verify Node provider inside Neovim

## psmux (Windows)
- `psmux` - Start tmux-compatible multiplexer
- `tmux` - Alias provided by psmux for tmux muscle memory
- `Prefix + I` - Install/update psmux plugins (ppm)

## btop
- `btop` - Interactive system monitor (CPU/RAM/disk/network)
