use i18n::t;

i18n::init!();

#[test]
fn test_locale_switching() {
    let locales = ["en", "zh", "ja"];
    
    for locale in locales {
        rust_i18n::set_locale(locale);
        
        // 确保翻译存在且不为空
        let translation = t!("linkura.cli.about");
        assert!(!translation.is_empty());
        println!("Locale {}: {}", locale, translation);
    }
}

#[test]
fn test_specific_translations() {
    rust_i18n::set_locale("zh");
    let locale = rust_i18n::locale();
    assert_eq!(&*locale, "zh");
    assert_eq!(t!("linkura.cli.about"), "林库拉的交互式 API 客户端");
    assert_eq!(t!("common.config.initialize.failed"), "初始化配置失败");
    
    rust_i18n::set_locale("en");
    let locale = rust_i18n::locale();
    assert_eq!(&*locale, "en");
    assert_eq!(t!("linkura.cli.about"), "Interactive cli api client for Linkura");
    assert_eq!(t!("common.config.initialize.failed"), "Failed to initialize config");
    
    rust_i18n::set_locale("ja");
    let locale = rust_i18n::locale();
    assert_eq!(&*locale, "ja");
    assert_eq!(t!("linkura.cli.about"), "リンクラのインタラクティブな API クライアント");
    assert_eq!(t!("common.config.initialize.failed"), "設定の初期化に失敗しました");
}

#[test]
fn test_fallback_behavior() {
    // 测试不存在的语言，应该回退到默认语言
    rust_i18n::set_locale("fr");
    let translation = t!("linkura.cli.about");
    
    // 应该回退到英文
    assert_eq!(translation, "Interactive cli api client for Linkura");
}