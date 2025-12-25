use rust_i18n::Backend;
use std::sync::OnceLock;

rust_i18n::i18n!("../../locales");

static SYSTEM_LOCALE: OnceLock<&'static str> = OnceLock::new();

fn detect_system_locale() -> &'static str {
    if let Some(locale) = sys_locale::get_locale() {
        if locale.starts_with("zh") {
            "zh"
        } else if locale.starts_with("ja") {
            "ja"
        } else {
            "eng"
        }
    } else {
        "eng"
    }
}

pub struct I18nBackend;

impl Backend for I18nBackend {
    fn available_locales(&self) -> Vec<&str> {
        _RUST_I18N_BACKEND.available_locales()
    }

    fn translate(&self, locale: &str, key: &str) -> Option<&str> {
        let system_locale = SYSTEM_LOCALE.get_or_init(|| detect_system_locale());
        let val = _RUST_I18N_BACKEND.translate(system_locale, key);
        if val.is_none() {
            _RUST_I18N_BACKEND.translate(locale, key)
        } else {
            val
        }
    }
}

#[macro_export]
macro_rules! init {
    () => {
        linkura_i18n::rust_i18n::i18n!(backend = linkura_i18n::I18nBackend);
    };
}

pub use rust_i18n::set_locale;
pub use rust_i18n::t;
pub use rust_i18n;
