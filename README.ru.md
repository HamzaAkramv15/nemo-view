# yami-view

Быстрый просмотрщик и редактор картинок для Wayland. Часть рабочего стола [yami](https://github.com/Trendorin/yami), которая работает и сама по себе: бери в любой Wayland-райс, шелл yami не нужен.

## Установка

```sh
./install.sh
```

Нужен Rust 1.89 или новее, root не нужен. Ставится в `~/.local`. `./install.sh --deps` — что нужно от системы, `./install.sh --uninstall` — удалить.

## Как пользоваться

`yami-view [ФАЙЛ|ПАПКА…]`. Чтобы «сделать обоями» работало вне yami, добавь в `style.json` `"wallpaperCmd": "swww img {}"`.

## Внешний вид

Без настроек программа выглядит нейтрально: системный шрифт, скруглённые углы, мягкие тени, светлая
или тёмная тема как в системе. Поменять можно всё:

| Файл | Что |
|---|---|
| `~/.config/yami/style.json` | вид всех модулей yami |
| `~/.config/yami/style.view.json` | чем этот модуль отличается (только эти ключи) |

| Ключ | Значения |
|---|---|
| `preset` | `plain` (нейтральный) или `yami` (пиксельный) |
| `accent` | `#rrggbb` |
| `light` | `true`, `false`; нет ключа — как в системе |
| `corners` | `notch`, `round`, `square` |
| `radius` | радиус углов панелей в пикселях |
| `shadow` | `hard`, `soft`, `none` |
| `reveal` | `fade`, `none`, `dither`, `scan`, `noise`, `wipe`, `blocks` |
| `font` | семейство шрифта |
| `animations`, `animSpeed` | вкл/выкл, скорость |
| `colors` | любой из `bg panel surface surfaceHi codeBg border dim accent deep glow bright selection text textDim textFaint onAccent textHi ok warn` как `#rrggbb` |
| `lang` | `ru`, `en`, `pt-BR`; пусто — как в системе |

Файлы перечитываются при изменении, программа перекрашивается без перезапуска.

## Лицензия

MIT
