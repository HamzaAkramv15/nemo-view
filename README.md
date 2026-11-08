# yami-view

Fast image viewer and quick editor for Wayland. A part of the [yami](https://github.com/Trendorin/yami) desktop that works on its own: take it into any Wayland setup, no yami shell needed.

## Install

```sh
chmod +x yami-view
./yami-view install
```

One file, no Rust and no root. `install` asks for the settings, lists what it will change and installs into `~/.local` only after your yes (`--yes` skips the questions, `--all` also turns on every option). `./yami-view deps` lists what it wants from the system, `./yami-view uninstall` removes it. To build it yourself: Rust 1.89 or newer, `cargo build --release --locked`, then `target/release/yami-view install`.

## Use

`yami-view [FILE|FOLDER…]`. To make «set as wallpaper» work outside yami, add `"wallpaperCmd": "swww img {}"` to `style.json`.

## Look

Without any configuration the program uses a neutral look: the system font, rounded corners, soft
shadows, light or dark as your system is set. Everything can be changed:

| File | What |
|---|---|
| `~/.config/yami/style.json` | the look of every yami module |
| `~/.config/yami/style.view.json` | what this module does differently (only those keys) |

| Key | Values |
|---|---|
| `preset` | `plain` (neutral) or `yami` (the pixel look) |
| `accent` | `#rrggbb` |
| `light` | `true`, `false`; absent: as the system |
| `corners` | `notch`, `round`, `square` |
| `radius` | panel corner radius in pixels |
| `shadow` | `hard`, `soft`, `none` |
| `reveal` | `fade`, `none`, `dither`, `scan`, `noise`, `wipe`, `blocks` |
| `font` | a font family |
| `animations`, `animSpeed` | on or off, speed |
| `colors` | any of `bg panel surface surfaceHi codeBg border dim accent deep glow bright selection text textDim textFaint onAccent textHi ok warn` as `#rrggbb` |
| `lang` | `ru`, `en`, `pt-BR`; empty: as the system |

The files are reread when they change, the program restyles without a restart.

## License

MIT
