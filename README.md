# 🧹 sweep

A minimal, fast, and **thorough** system cleaner for Linux that finds and removes:

- 📦 **Orphaned packages** (packages with no dependencies)
- 🗑️ **Residual configurations** (leftover config files after package removal)
- 🏠 **Home directory artifacts** (leftover config/cache/data directories from uninstalled apps)

Built with **Rust** for speed, safety, and zero runtime dependencies.  
Works on **Void Linux** (`xbps`) and **Debian/Ubuntu/Linux Mint** (`dpkg`/`apt`).

> **"I don't want my home directory polluted with `~/Downloads`, `~/.appname`, and orphaned packages."**  
> — Every tidy Linux user, probably

---

## ✨ Features

- **Interactive TUI** (Terminal User Interface) — select items with spacebar, confirm with Enter
- **Dry-run mode** (enabled by default) — see what will be removed before doing it
- **Deep cleanup** — removes packages **and** their traces in `~/.config`, `~/.local/share`, `~/.cache`
- **Cross-distro** — auto-detects your package manager
- **Safe by default** — requires explicit confirmation for real deletion
- **Single binary** — no Python, no Node.js, no bloat

---

## 🚀 Installation

### From source (requires Rust)

```bash
git clone https://github.com/zornfeuer/sweep.git
cd sweep
cargo build --release
sudo cp target/release/sweep /usr/local/bin/
```

Make sure `/usr/local/bin` is in your `$PATH`.

---

## 🛠️ Usage

```bash
# Dry-run (safe preview) — DEFAULT
sweep

# Real cleanup (requires confirmation)
sweep --delete

# Show only orphaned packages (Void)
sweep --orphans

# Show only residual configs (Debian/Mint)
sweep --residual
```

### In the TUI:
- **↑/↓** — navigate
- **Space** — select/deselect
- **Enter** — confirm deletion (in non-dry-run mode)
- **q / Esc** — quit

---

## 🔍 What it cleans

### On Void Linux
- Orphaned packages via `xbps-query -O`
- Home artifacts matching orphaned package names

### On Debian/Ubuntu/Linux Mint
- Residual config packages (`dpkg -l` status `rc`)
- Home artifacts matching residual package names

> 💡 **Home artifacts** are directories in:
> - `~/.config/`
> - `~/.local/share/`
> - `~/.cache/`  
> that match the name of a removed/residual package.

---

## 🔒 Safety

- **Dry-run is enabled by default** — nothing is deleted without your explicit consent.
- **Real deletion requires typing `yes`** — no accidental wipes.
- **Only removes what you select** — full control in the TUI.
- **Uses `sudo` only for package removal** (on Debian-based systems) — your password is handled by the system.

---

## 🧪 Tested on

- **Void Linux** (musl/glibc)
- **Linux Mint 21.x** (Ubuntu 22.04 base)

Should work on any Debian/Ubuntu derivative and Void.

---

## 📦 Roadmap

- [ ] Support for Arch Linux (`pacman -Qdt`)
- [ ] Export cleanup report to `~/.local/state/sweep/`
- [ ] Color themes and UI improvements

---

## 🤝 Contributing

PRs welcome! Especially:
- New package manager support
- Better home artifact detection (via `.desktop` files, etc.)
- UX improvements

---

## 📜 License

MIT

---

> **Keep your system lean. Keep your home clean.**  
> — `sweep` 🦀
