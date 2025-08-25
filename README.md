
# giftint

> 🎨 A small experimental CLI tool that attempts to recolor GIFs into a chosen color palette — built to match ricing setups and aesthetic themes.

---

## ⚡ Overview

`giftint` is a personal experiment in GIF recoloring. The idea is to take an existing animated GIF and **remap its colors** to a custom palette (for example, your favorite Catppuccin, Gruvbox, or custom dotfiles theme).  

It’s not perfect — expect some **bleeding**, **loss of detail**, and transparency issues — but it can still be useful for generating themed GIFs for your desktop setup, status bars, or memes.

---

## ✨ Features

- CLI-based workflow.
- Attempts to map GIF colors to your palette.
- Transparency support (experimental).
- Built in **Rust** for speed.

---

## 🚧 Status / Limitations

- Color bleeding is common.
- Transparency is **experimental** and may not look right.
- Not production-ready — this is mainly a ricing side-project.

---

## 📦 Installation

Clone and build from source:

```bash
git clone https://github.com/waougri/giftint
cd giftint
cargo build --release
````

The compiled binary will be in:

```
target/release/giftint
```

You can copy it somewhere in your `$PATH` if you want to call it globally.

---

## 🔧 Usage

Basic command format:

```bash
giftint <input.gif> <palette_file> <output.gif>
```

* `<input.gif>` → the GIF you want to recolor
* `<palette_file>` → a file describing your palette (format TBD / WIP)
* `<output.gif>` → the path to save the recolored result

Example:

```bash
giftint meme.gif catppuccin.txt meme_catppuccin.gif
```

---

## 🎨 Palette Format

Currently, palettes are provided as a plain text file with hex codes:

```
#1e1e2e
#f38ba8
#a6e3a1
#fab387
#89b4fa
```

Colors will be matched **in order** against the GIF’s detected palette.

---

## 🛠 Roadmap

* Improve color-matching to reduce bleeding.
* Better transparency handling.
* Smarter palette mapping algorithms.
* Add preset palettes (Catppuccin, Gruvbox, Solarized, etc).
* Configurable dithering options.

---

## 🤝 Contributing

This is just a personal experiment, but contributions are welcome if you want to play around with palette mapping, transparency fixes, or performance improvements.

---

## 📜 License

[MIT](./LICENSE) — do whatever you want with it.

---

## 🐧 Why?

Because ricing isn’t complete until your GIFs match your color scheme.


