# Tmux
<!-- prefix: Ctrl-s -->

## Prefix & Config
- `Ctrl-s r` - Reload tmux config
- `Ctrl-s |` - Split pane horizontal
- `Ctrl-s -` - Split pane vertical
- `Ctrl-s c` - New window (keep path)
- `Ctrl-s n` - Next window
- `Ctrl-s p` - Previous window
- `Ctrl-s <` - Swap window left
- `Ctrl-s >` - Swap window right

## Pane Navigation
- `Ctrl-s h` - Select left pane
- `Ctrl-s j` - Select down pane
- `Ctrl-s k` - Select up pane
- `Ctrl-s l` - Select right pane

## Pane Resize
- `Ctrl-s H` - Resize left
- `Ctrl-s J` - Resize down
- `Ctrl-s K` - Resize up
- `Ctrl-s L` - Resize right

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

## Windows (built-in)
- `Ctrl-s w` - Choose window (interactive)
- `Ctrl-s f` - Find window by name
- `Ctrl-s ,` - Rename window
- `Ctrl-s &` - Kill window
- `Ctrl-s .` - Move/renumber window
- `Ctrl-s 0-9` - Jump to window by index

## Panes (built-in)
- `Ctrl-s z` - Zoom pane (toggle fullscreen)
- `Ctrl-s q` - Show pane numbers
- `Ctrl-s x` - Kill pane
- `Ctrl-s o` - Cycle to next pane
- `Ctrl-s ;` - Last active pane
- `Ctrl-s {` - Swap pane left
- `Ctrl-s }` - Swap pane right
- `Ctrl-s Space` - Cycle pane layouts
- `Ctrl-s !` - Break pane into new window

## Misc (built-in)
- `Ctrl-s ?` - Show all keybindings
- `Ctrl-s :` - Command prompt
- `Ctrl-s ]` - Paste from buffer
- `Ctrl-s t` - Show clock
- `Ctrl-s i` - Window info

# Neovim
<!-- leader: , -->

## Files & Buffers
- `Ctrl-t` - Toggle NvimTree
- `-` - Open Mini Files
- `Ctrl-p` - Find files (Telescope)
- `,<space>` - Find open buffers
- `,?` - Recently opened files
- `,/` - Fuzzy search in buffer
- `,jr` - Set tree root to cwd

## Find (Telescope)
- `,ff` - Find files
- `,fg` - Grep (project)
- `,fr` - Grep (git root)
- `,fw` - Find word under cursor
- `,fo` - Find in open files
- `,fb` - Find buffers
- `,fd` - Find diagnostics
- `,fh` - Find help tags
- `,fi` - Find files in git
- `,fa` - Resume last search
- `,fs` - Select telescope picker
- `,fj` - Hop word jump

## Harpoon
- `,ha` - Add file to harpoon
- `,hh` - Toggle harpoon menu
- `,h1` - Harpoon file 1
- `,h2` - Harpoon file 2
- `,h3` - Harpoon file 3
- `,h4` - Harpoon file 4

## Go To (LSP)
- `gd` - Go to definition
- `gD` - Go to type definition
- `gr` - Go to references
- `gm` - Go to implementation
- `gi` - Incoming calls
- `gu` - Outgoing calls
- `ge` - Go to error
- `gs` - Go to symbols (workspace)

## LSP Actions
- `K` - Signature help
- `,K` - Hover docs
- `,rn` - Rename symbol
- `,ca` - Code action
- `,lf` - Format buffer
- `,ds` - Document symbols
- `,ws` - Workspace symbols

## Splits & Windows
- `,sv` - Split vertical
- `,sh` - Split horizontal
- `,se` - Equalize splits
- `,sx` - Close split
- `,sj` - Shorter (height)
- `,sk` - Taller (height)
- `,sl` - Wider (width)
- `,ws` - Swap windows
- `,vh` - Vertical - horizontal
- `,hv` - Horizontal - vertical

## Pane Navigation
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
- `Ctrl-b` - Page back (centered)
- `n` - Next search (centered)
- `N` - Prev search (centered)
- `s` - Leap forward
- `S` - Leap backward
- `J` - Move selection down (visual)
- `K` - Move selection up (visual)

## Terminal
- `Ctrl-\` - Open terminal split
- `jk` - Exit terminal mode
- `Esc` - Exit terminal mode

## Debug (DAP)
- `F5` - Continue
- `F10` - Step over
- `F11` - Step into
- `,db` - Toggle breakpoint
- `,dx` - Terminate debugger

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
- `,oc` - Clear search highlight
- `,os` - Toggle scratch buffer
- `,m` - Copy messages
- `,ko` - Switch source/header (Clangd)
- `jk` - Escape (insert mode)

# Neovim Completion

## nvim-cmp (Insert Mode)
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

# Neovim GUI

## Font Size (GUI only)
- `Ctrl-=` - Increase font size
- `Ctrl--` - Decrease font size
