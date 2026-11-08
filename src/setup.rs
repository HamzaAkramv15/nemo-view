// Что ставит `yami-view install`: файлы, вшитые в бинарь, и вопросы к человеку (yami_px::install).

use yami_px::install::{Asset, Choice, Spec};

pub const SPEC: Spec = Spec {
    name: "yami-view",
    version: env!("CARGO_PKG_VERSION"),
    about: ["просмотр и быстрая правка картинок", "image viewer and quick editor", "visualizador e editor rápido de imagens"], // i18n-ok
    deps: "wayland, libxkbcommon",
    assets: &[Asset { path: "applications/yami-view.desktop", body: include_str!("../config/yami-view.desktop") }],
    choices: &[Choice { ask: ["Открывать картинки в yami-view по умолчанию?", "Open pictures in yami-view by default?", "Abrir imagens no yami-view por padrão?"], assets: &[], default_app: true, note: None }], // i18n-ok
};
