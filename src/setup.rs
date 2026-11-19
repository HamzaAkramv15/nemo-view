// Что ставит `nemo-view install`: файлы, вшитые в бинарь, и вопросы к человеку (nemo_px::install).

use nemo_px::install::{Asset, Choice, Spec};

pub const SPEC: Spec = Spec {
    name: "nemo-view",
    version: env!("CARGO_PKG_VERSION"),
    about: ["просмотр и быстрая правка картинок", "image viewer and quick editor", "visualizador e editor rápido de imagens"], // i18n-ok
    deps: "wayland, libxkbcommon",
    assets: &[Asset { path: "applications/nemo-view.desktop", body: include_str!("../config/nemo-view.desktop") }],
    choices: &[Choice { ask: ["Открывать картинки в nemo-view по умолчанию?", "Open pictures in nemo-view by default?", "Abrir imagens no nemo-view por padrão?"], assets: &[], default_app: true, note: None }], // i18n-ok
};
