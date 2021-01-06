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
### Automatically (recommended)
If the plugin is published to [autoclip-plugins](https://github.com/siketyan/autoclip-plugins) repository, you can install it automatically:

```console
$ ./autoclip-app install [name]
```

### Manually
1. Open the local data directory.
    - Windows: `C:\Users\[Your Name]\AppData\Local`
    - macOS: `/Users/[Your Name]/Library/Application Support`
    - Linux: `/home/[your_name]/.local/share`
1. Now go into `autoclip` directory, then `plugins` .
    - If the directories not exists, create them.
1. Put the `.dll`, `.dylib` or `.so` files of plugins into the `plugins` directory.

## 🔧 Developing Plugins
1. Setup your Rust environment.
1. Create a new lib crate.
   ```console
   $ cargo new --lib plugin-name-of-your-plugin
   ```
1. Configure Cargo.toml, changing the crate type to `cdylib`.
   ```toml
   [lib]
   crate-type = ["cdylib"]
   ```
1. Add `autoclip-core` as a dependency.
   ```toml
   [dependencies]
   autoclip-core = "0.1.0"
   ```
1. Implement `AutoclipPlugin` trait as you like.
1. Export the plugin with a macro:
   ```rust
   autoclip_core::export_plugin!("name-of-your-plugin", AutoclipPluginImpl);
   ```
1. Build & distribute `.dll`, `.dylib` and `.so` files!

## ☑ ToDo
- OS Support
    - [x] Windows Support
    - [x] macOS Support
    - [x] Linux Support
- Customisation
    - [x] Polling Interval
- [ ] Installer
- [x] Plugin Installer
