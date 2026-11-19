// Язык просмотрщика: надписи по-русски, английский по
// таблице при отрисовке. Язык берётся из style.json.
// Третий язык — бразильский португальский: таблица PT_BR (i18n_pt_br.rs) от английского, у tf! —
// третий шаблон. Какой язык — решает nemo_px::i18n (общий выбор для всех программ).

use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Mutex, OnceLock};

pub use nemo_px::i18n::{code, pt_br, set, translated};

/// `shot`: собирать надписи, которые остались без перевода
static AUDIT: AtomicBool = AtomicBool::new(false);
static MISSED: Mutex<Vec<String>> = Mutex::new(Vec::new());

/// перевод целой надписи (английский по PAIRS, португальский по PT_BR от английского); нет — как есть
pub fn tr(s: &str) -> &str {
    if !translated() {
        return s;
    }
    let english = table().get(s).copied().unwrap_or(s);
    let out = select(s, english);
    // без перевода: русская надпись не из таблицы; у португальского — ещё и нет строки в его таблице
    // (остальные языки переводятся по частям: без строки — английская)
    let known = table().contains_key(s);
    // названия языков (Русский, Українська) не переводятся нарочно
    let name = nemo_px::i18n::LANGS.iter().any(|l| l.1 == s);
    // украинский сам кириллица: там выдаёт русский только буква, которой в нём нет (ы э ъ ё)
    let ru = if code() == "uk" { s.chars().any(|c| matches!(c, 'ы' | 'э' | 'ъ' | 'ё' | 'Ы' | 'Э' | 'Ё')) } else { s.chars().any(|c| matches!(c, 'а'..='я' | 'А'..='Я' | 'ё' | 'Ё')) };
    let missing = (!known && !name && ru) || (pt_br() && known && !lang_table().is_some_and(|t| t.contains_key(english)));
    if AUDIT.load(Ordering::Relaxed) && missing {
        if let Ok(mut m) = MISSED.lock() {
            if !m.iter().any(|x| x == out) {
                m.push(out.to_string());
            }
        }
    }
    out
}

/// пара прямо в коде (клавиши, doctor): русская и английская, остальные языки — по английской
pub fn select<'a>(ru: &'a str, en: &'a str) -> &'a str {
    if !translated() {
        return ru;
    }
    lang_table().and_then(|t| t.get(en).copied()).unwrap_or(en)
}

pub fn audit(on: bool) {
    AUDIT.store(on, Ordering::Relaxed);
}

/// надписи без перевода с прошлого вызова
pub fn take_missed() -> Vec<String> {
    MISSED.lock().map(|mut m| std::mem::take(&mut *m)).unwrap_or_default()
}

/// строка с подстановками: русский, английский и португальский шаблоны рядом (литералы: Rust
/// проверяет аргументы при сборке); остальные языки — по английскому шаблону и таблице языка (localize)
#[macro_export]
macro_rules! tf {
    ($ru:literal, $en:literal, $pt:literal $(, $a:expr)* $(,)?) => {
        if $crate::i18n::pt_br() {
            format!($pt $(, $a)*)
        } else if $crate::i18n::translated() {
            $crate::i18n::localize($en, format!($en $(, $a)*))
        } else {
            format!($ru $(, $a)*)
        }
    };
}

fn table() -> &'static HashMap<&'static str, &'static str> {
    static T: OnceLock<HashMap<&'static str, &'static str>> = OnceLock::new();
    T.get_or_init(|| PAIRS.iter().copied().collect())
}

/// таблица текущего языка от английского (португальский и остальные); у русского и английского её нет
fn lang_table() -> Option<&'static HashMap<&'static str, &'static str>> {
    static T: [OnceLock<HashMap<&'static str, &'static str>>; 6] = [const { OnceLock::new() }; 6];
    let (i, rows): (usize, &'static [(&'static str, &'static str)]) = match code() {
        "pt-BR" => (0, PT_BR),
        "es" => (1, ES),
        "de" => (2, DE),
        "fr" => (3, FR),
        "uk" => (4, UK),
        "pl" => (5, PL),
        _ => return None,
    };
    Some(T[i].get_or_init(|| rows.iter().copied().collect()))
}

/// строка tf! на языке без своего шаблона в коде: out собрана по английскому шаблону en, значения
/// переезжают в шаблон языка (nemo_px::i18n::refill); нет строки в таблице — остаётся английская
pub fn localize(en: &str, out: String) -> String {
    match lang_table().and_then(|t| t.get(en)) {
        Some(to) => nemo_px::i18n::refill(en, &out, to).unwrap_or(out),
        None => out,
    }
}

include!("i18n_pt_br.rs");
include!("i18n_es.rs");
include!("i18n_de.rs");
include!("i18n_fr.rs");
include!("i18n_uk.rs");
include!("i18n_pl.rs");

const PAIRS: &[(&str, &str)] = &[
    ("сохранено", "saved"),
    ("пусто", "empty"),
    ("прошлая правка не сохранена: вернись к ней — ctrl+s или ctrl+z", "the last edit is not saved: go back for ctrl+s or ctrl+z"),
    ("нечего отменять", "nothing to undo"),
    ("правка отменена", "edit undone"),
    ("выдели область мышью", "select an area with the mouse"),
    ("изменений нет", "no changes"),
    ("анимацию не перезаписываю: ctrl+shift+s — сохранить как png", "not overwriting an animation: ctrl+shift+s saves a png"),
    ("в корзине", "moved to trash"),
    ("корзина недоступна", "trash is not available"),
    ("обои поменяются через секунду", "the wallpaper changes in a second"),
    ("скопировано", "copied"),
    ("слайд-шоу: каждые 4 с", "slideshow: every 4 s"),
    ("слайд-шоу выключено", "slideshow off"),
    ("нет картинок", "no images"),
    ("открываю", "opening"),
    ("назад · ←", "back · ←"),
    ("вперёд · →", "forward · →"),
    ("мельче · −", "smaller · −"),
    ("вписать · 0", "fit · 0"),
    ("крупнее · +", "bigger · +"),
    ("повернуть влево · shift+r", "rotate left · shift+r"),
    ("повернуть вправо · r", "rotate right · r"),
    ("отразить · h", "flip · h"),
    ("кадрировать · c", "crop · c"),
    ("в буфер · ctrl+c", "copy · ctrl+c"),
    ("сохранить · ctrl+s", "save · ctrl+s"),
    ("сохранить как… · ctrl+shift+s", "save as… · ctrl+shift+s"),
    ("сделать обоями · w", "set as wallpaper · w"),
    ("показать в папке · e", "show in folder · e"),
    ("в корзину · del", "move to trash · del"),
    ("сведения · i", "details · i"),
    ("слайд-шоу · пробел", "slideshow · space"),
    ("во весь экран · f", "fullscreen · f"),
    ("обрезать", "crop"),
    ("отмена", "cancel"),
    ("тяни по картинке · enter — обрезать · esc — отмена", "drag over the image · enter to crop · esc to cancel"),
    ("сведения", "details"),
    ("размер", "size"),
    ("файл", "file"),
    ("формат", "format"),
    ("изменён", "modified"),
    ("папка", "folder"),
    ("прозрачность", "transparency"),
    ("есть", "yes"),
    ("нет", "no"),
];
