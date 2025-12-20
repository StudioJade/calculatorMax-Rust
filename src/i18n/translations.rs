//! Translations for different languages

use std::collections::HashMap;
use sys_locale::get_locale;

/// Supported languages
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Language {
    Auto, // Automatically detect system language
    SimplifiedChinese,
    TraditionalChineseTW,
    TraditionalChineseHK,
    English,
    Russian,
    Cat,
}

impl Language {
    /// Get the language code
    pub fn code(&self) -> &'static str {
        match self {
            Language::Auto => "auto",
            Language::SimplifiedChinese => "zh-CN",
            Language::TraditionalChineseTW => "zh-TW",
            Language::TraditionalChineseHK => "zh-HK",
            Language::English => "en",
            Language::Russian => "ru",
            Language::Cat => "cat",
        }
    }

    /// Detect system language
    pub fn detect_system_language() -> Language {
        let locale = get_locale().unwrap_or_else(|| "en".to_string());
        
        match locale.as_str() {
            "zh-CN" | "zh-SG" | "zh" => Language::SimplifiedChinese,
            "zh-TW" | "zh-HK" | "zh-MO" => Language::TraditionalChineseTW,
            "ru" | "ru-RU" => Language::Russian,
            // Add more language mappings as needed
            _ => {
                // Default to English for unsupported languages
                Language::English
            }
        }
    }

    /// Get the display name of the language (fixed, not localized)
    pub fn display_name(&self) -> &'static str {
        match self {
            Language::Auto => "Auto",
            Language::SimplifiedChinese => "简体中文",
            Language::TraditionalChineseTW => "繁体中文",
            Language::TraditionalChineseHK => "繁体中文",
            Language::English => "English",
            Language::Russian => "Русский",
            Language::Cat => "喵语",
        }
    }

    /// Get all supported languages
    pub fn all() -> Vec<Language> {
        vec![
            Language::Auto,
            Language::SimplifiedChinese,
            Language::TraditionalChineseTW,
            Language::TraditionalChineseHK,
            Language::English,
            Language::Russian,
            Language::Cat,
        ]
    }
}

/// Translation strings for each language
#[derive(Debug)]
pub struct Translations {
    strings: HashMap<String, HashMap<Language, String>>,
}

impl Translations {
    /// Create a new translation instance by loading from JSON files
    pub fn new() -> Self {
        let mut translations = Translations {
            strings: HashMap::new(),
        };

        // Load translations from JSON files
        translations.load_from_json();
        translations
    }

    /// Load translations from embedded JSON files
    fn load_from_json(&mut self) {
        let translations_data = [
            (Language::SimplifiedChinese, "zh-CN", include_str!("locales/zh-CN.json")),
            (
                Language::TraditionalChineseTW,
                "zh-TW",
                include_str!("locales/zh-TW.json"),
            ),
            (
                Language::TraditionalChineseHK,
                "zh-HK",
                include_str!("locales/zh-HK.json"),
            ),
            (Language::English, "en", include_str!("locales/en.json")),
            (Language::Russian, "ru", include_str!("locales/ru.json")),
            (Language::Cat, "cat", include_str!("locales/cat.json")),
        ];

        for (lang, _code, json_str) in &translations_data {
            if let Ok(value) = serde_json::from_str::<serde_json::Value>(json_str) {
                if let Some(obj) = value.as_object() {
                    for (key, val) in obj.iter() {
                        if let Some(text) = val.as_str() {
                            self.strings
                                .entry(key.clone())
                                .or_insert_with(HashMap::new)
                                .insert(*lang, text.to_string());
                        }
                    }
                }
            }
        }
    }

    /// Get translation for a key in the specified language
    pub fn get(&self, key: &str, lang: Language) -> String {
        self.strings
            .get(key)
            .and_then(|map| map.get(&lang))
            .cloned()
            .unwrap_or_else(|| key.to_string())
    }
}

impl Default for Translations {
    fn default() -> Self {
        Self::new()
    }
}
