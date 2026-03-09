# KeyViewer

A minimal, single-shot keyboard shortcut viewer for Windows. Launch it, glance at your keybindings, press **Escape** to close.
Vibe the keymap with Claude; just tell it where your configs are, have it build.  Instant personal key chart.

![KeyViewer screenshot](screenshot.png)

## What it does

KeyViewer reads a simple markdown file (`keys.md`) listing your keyboard shortcuts and displays them in a compact, always-on-top overlay. It's designed for those moments when you can't quite remember a binding — launch it, find what you need, dismiss it.

- **One-shot**: opens, shows your keybindings, closes when you press Escape. No background process, no tray icon.
- **Always on top**: appears over your current workspace so you don't lose context.
- **Single instance**: launching a second copy closes the first.
- **Fast**: native Rust + egui, starts in milliseconds.

## The keybindings file

KeyViewer reads `keys.md` from the same directory as the executable (falls back to the current working directory). The format is simple markdown:

```markdown
# Section Name

## Category Name
- `Key` - Description
- `Ctrl-S r` - Reload config
- `Leader gd` - Go to definition
```

- **`# Heading`** creates a top-level section (displayed in a distinct color).
- **`## Heading`** creates a category within the current section.
- **`` - `Key` - Description ``** defines a keybinding. The key goes in backticks, followed by a separator and description.
- **`<!-- comments -->`** are ignored, so you can annotate your file.

Categories are laid out in a 4-column grid within each section.

### Example

```markdown
# Tmux

## Prefix & Config
- `Ctrl-s r` - Reload tmux config
- `Ctrl-s |` - Split pane horizontal
- `Ctrl-s -` - Split pane vertical

## Pane Navigation
- `Ctrl-s h` - Select left pane
- `Ctrl-s j` - Select down pane
- `Ctrl-s k` - Select up pane
- `Ctrl-s l` - Select right pane

# Neovim

## Telescope
- `Leader ff` - Find files
- `Leader fg` - Live grep
- `Leader fb` - Buffers
```

## Building

```
cargo build --release
```

The binary is at `target/release/keyviewer.exe`. Copy it alongside your `keys.md` wherever you like.

## Usage

```
keyviewer.exe
```

Press **Escape** to close.

### Windows hotkey tip

To launch KeyViewer with a global hotkey:

1. Create a shortcut to `keyviewer.exe`
2. Right-click the shortcut, select **Properties**
3. Click the **Shortcut key** field and press your preferred key combo (e.g. `Ctrl+Shift+K`)
4. Place the shortcut in your Start Menu or desktop

Since KeyViewer auto-closes any existing instance on launch, pressing the hotkey again will refresh and re-show the overlay.

## Generating your keybindings file with Claude

Rather than writing `keys.md` by hand, you can point Claude at your config files and have it generate the keybinding list for you. For example:

> "Read my `~/.tmux.conf` and `~/.config/nvim/init.lua` and generate a `keys.md` file for KeyViewer with all my keybindings organized by tool and category."

Claude will parse your configs, extract the bindings, and produce a ready-to-use markdown file in the right format.

## License

MIT
