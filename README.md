# KeyViewer

A minimal, single-shot keyboard shortcut viewer. Launch it, glance at your keybindings, press **Escape** to close.
Vibe the keymap with Claude; just tell it where your configs are, have it build.  Instant personal key chart.

![KeyViewer screenshot](screenshot.png)

## What it does

KeyViewer reads a simple markdown file (`keys.md`) listing your keyboard shortcuts and displays them in a compact, always-on-top overlay. It's designed for those moments when you can't quite remember a binding — launch it, find what you need, dismiss it.

- **One-shot**: opens, shows your keybindings, closes when you press Escape. No background process, no tray icon.
- **Always on top**: appears over your current workspace so you don't lose context.
- **Single instance on Windows**: launching a second copy closes the first.
- **Fast**: native Rust + egui, starts in milliseconds.

## The keybindings file

KeyViewer ships with a built-in `keys.md` compiled into the binary.

On startup it:

1. Looks for `keys.md` next to the executable.
2. Uses that local file if it exists.
3. Writes out the built-in default if no local file is present.
4. Falls back to `keys.md` in the current working directory only if the executable path can't be resolved.

That means a custom `keys.md` beside the installed binary overrides the built-in keymap. The format is simple markdown:

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

The binary is at `target/release/kv.exe` on Windows and `target/release/kv` on macOS/Linux.

## Usage

```
cargo install --path .
kv
```

Press **Escape** to close.

If you don't want to install it, you can also run the built binary directly:

```powershell
.\target\release\kv.exe
```

```bash
./target/release/kv
```

## Quick launch shortcuts

### Windows: PowerToys Keyboard Manager

If you use Microsoft PowerToys, you can bind a global shortcut directly to KeyViewer:

1. Open **PowerToys** and enable **Keyboard Manager**.
2. Open **Remap a shortcut**.
3. Pick the shortcut you want to use.
4. Set the action to **Start App**.
5. Point it at your `kv.exe` path, for example `%USERPROFILE%\.cargo\bin\kv.exe` if you installed with Cargo.
6. Set **Start in** to the folder containing `kv.exe`.

This gives you a launcher chord without needing a separate `.lnk` file.

### macOS: Shortcuts.app

The built-in macOS way to do this is with the **Shortcuts** app:

1. Create a shortcut that launches KeyViewer.
2. If you have an app bundle, use **Open App**.
3. If you installed the Cargo binary, use **Run Shell Script** and call the full path, for example `$HOME/.cargo/bin/kv`.
4. Open the shortcut details and click **Add Keyboard Shortcut**.
5. Press the key combo you want to use.

After that, the shortcut can launch KeyViewer globally from the keyboard.

## Generating your keybindings file with Claude

Rather than writing `keys.md` by hand, you can point Claude at your config files and have it generate the keybinding list for you. For example:

> "Read my `~/.tmux.conf` and `~/.config/nvim/init.lua` and generate a `keys.md` file for KeyViewer with all my keybindings organized by tool and category."

Claude will parse your configs, extract the bindings, and produce a ready-to-use markdown file in the right format.

## License

MIT
