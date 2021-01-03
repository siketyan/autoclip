# 📎 autoclip
![Rust](https://github.com/siketyan/autoclip/workflows/Rust/badge.svg)

Do something on your clipboard, automatically.

## ✨ Features
- Automatic
- Customisable with Plugins

## 📦 Installation
```
$ cargo build --release
```

## 🔌 Installing Plugins
1. Open the local data directory.
    - Windows: `C:\Users\[Your Name]\AppData\Local`
    - macOS: `/Users/[Your Name]/Library/Application Support`
    - Linux: `/home/[your_name]/.local/share`
1. Now go into `autoclip` directory, then `plugins` .
    - If the directories not exists, create them.
1. Put the `.dll` or `.so` files of plugins into the `plugins` directory.

## ☑ ToDo
- OS Support
    - [x] Windows Support
    - [x] macOS Support
    - [x] Linux Support
- Customisation
    - [x] Polling Interval
- [ ] Installer
- [ ] Plugin Installer
