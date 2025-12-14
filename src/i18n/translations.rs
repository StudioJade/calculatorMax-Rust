//! Translations for different languages

use std::collections::HashMap;

/// Supported languages
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Language {
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
            Language::SimplifiedChinese => "zh-CN",
            Language::TraditionalChineseTW => "zh-TW",
            Language::TraditionalChineseHK => "zh-HK",
            Language::English => "en",
            Language::Russian => "ru",
            Language::Cat => "cat",
        }
    }
    
    /// Get all supported languages
    pub fn all() -> Vec<Language> {
        vec![
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
    strings: HashMap<&'static str, HashMap<Language, String>>,
}

impl Translations {
    /// Create a new translation instance
    pub fn new() -> Self {
        let mut translations = Translations {
            strings: HashMap::new(),
        };
        
        // Initialize translations
        translations.init_translations();
        translations
    }
    
    /// Initialize all translations
    fn init_translations(&mut self) {
        // Application title
        self.add_translation("app_title", Language::SimplifiedChinese, "计算器 Max");
        self.add_translation("app_title", Language::TraditionalChineseTW, "計算機 Max");
        self.add_translation("app_title", Language::TraditionalChineseHK, "計算機 Max");
        self.add_translation("app_title", Language::English, "Calculator Max");
        self.add_translation("app_title", Language::Russian, "Калькулятор Max");
        self.add_translation("app_title", Language::Cat, "喵喵计算器 Max");
        
        // Expression label
        self.add_translation("expression", Language::SimplifiedChinese, "表达式:");
        self.add_translation("expression", Language::TraditionalChineseTW, "運算式:");
        self.add_translation("expression", Language::TraditionalChineseHK, "表達式:");
        self.add_translation("expression", Language::English, "Expression:");
        self.add_translation("expression", Language::Russian, "Выражение:");
        self.add_translation("expression", Language::Cat, "喵喵:");
        
        // Calculate button
        self.add_translation("calculate", Language::SimplifiedChinese, "计算");
        self.add_translation("calculate", Language::TraditionalChineseTW, "計算");
        self.add_translation("calculate", Language::TraditionalChineseHK, "計算");
        self.add_translation("calculate", Language::English, "Calculate");
        self.add_translation("calculate", Language::Russian, "Вычислить");
        self.add_translation("calculate", Language::Cat, "喵喵");
        
        // Result label
        self.add_translation("result", Language::SimplifiedChinese, "结果:");
        self.add_translation("result", Language::TraditionalChineseTW, "結果:");
        self.add_translation("result", Language::TraditionalChineseHK, "結果:");
        self.add_translation("result", Language::English, "Result:");
        self.add_translation("result", Language::Russian, "Результат:");
        self.add_translation("result", Language::Cat, "喵果:");
        
        // Memory label
        self.add_translation("memory", Language::SimplifiedChinese, "记忆 (m):");
        self.add_translation("memory", Language::TraditionalChineseTW, "記憶 (m):");
        self.add_translation("memory", Language::TraditionalChineseHK, "記憶 (m):");
        self.add_translation("memory", Language::English, "Memory (m):");
        self.add_translation("memory", Language::Russian, "Память (m):");
        self.add_translation("memory", Language::Cat, "喵記 (m):");
        
        // History button
        self.add_translation("history", Language::SimplifiedChinese, "历史记录");
        self.add_translation("history", Language::TraditionalChineseTW, "歷史記錄");
        self.add_translation("history", Language::TraditionalChineseHK, "歷史記錄");
        self.add_translation("history", Language::English, "History");
        self.add_translation("history", Language::Russian, "История");
        self.add_translation("history", Language::Cat, "喵史");
        
        // Clear History button
        self.add_translation("clear_history", Language::SimplifiedChinese, "清空历史记录");
        self.add_translation("clear_history", Language::TraditionalChineseTW, "清空歷史記錄");
        self.add_translation("clear_history", Language::TraditionalChineseHK, "清空歷史記錄");
        self.add_translation("clear_history", Language::English, "Clear History");
        self.add_translation("clear_history", Language::Russian, "Очистить историю");
        self.add_translation("clear_history", Language::Cat, "清空喵史");
        
        // Settings button
        self.add_translation("settings", Language::SimplifiedChinese, "设置");
        self.add_translation("settings", Language::TraditionalChineseTW, "設定");
        self.add_translation("settings", Language::TraditionalChineseHK, "設定");
        self.add_translation("settings", Language::English, "Settings");
        self.add_translation("settings", Language::Russian, "Настройки");
        self.add_translation("settings", Language::Cat, "喵设");
        
        // Exit button
        self.add_translation("exit", Language::SimplifiedChinese, "退出");
        self.add_translation("exit", Language::TraditionalChineseTW, "離開");
        self.add_translation("exit", Language::TraditionalChineseHK, "離開");
        self.add_translation("exit", Language::English, "Exit");
        self.add_translation("exit", Language::Russian, "Выход");
        self.add_translation("exit", Language::Cat, "喵出");
        
        // Safe Mode checkbox
        self.add_translation("safe_mode", Language::SimplifiedChinese, "安全模式");
        self.add_translation("safe_mode", Language::TraditionalChineseTW, "安全模式");
        self.add_translation("safe_mode", Language::TraditionalChineseHK, "安全模式");
        self.add_translation("safe_mode", Language::English, "Safe Mode");
        self.add_translation("safe_mode", Language::Russian, "Безопасный режим");
        self.add_translation("safe_mode", Language::Cat, "安全喵式");
        
        // Save History button
        self.add_translation("save_history", Language::SimplifiedChinese, "保存历史记录");
        self.add_translation("save_history", Language::TraditionalChineseTW, "儲存歷史記錄");
        self.add_translation("save_history", Language::TraditionalChineseHK, "儲存歷史記錄");
        self.add_translation("save_history", Language::English, "Save History");
        self.add_translation("save_history", Language::Russian, "Сохранить историю");
        self.add_translation("save_history", Language::Cat, "保存喵史");
        
        // Filename label
        self.add_translation("filename", Language::SimplifiedChinese, "文件名:");
        self.add_translation("filename", Language::TraditionalChineseTW, "檔案名稱:");
        self.add_translation("filename", Language::TraditionalChineseHK, "檔案名稱:");
        self.add_translation("filename", Language::English, "Filename:");
        self.add_translation("filename", Language::Russian, "Имя файла:");
        self.add_translation("filename", Language::Cat, "喵件名:");
        
        // Settings heading
        self.add_translation("settings_heading", Language::SimplifiedChinese, "设置");
        self.add_translation("settings_heading", Language::TraditionalChineseTW, "設定");
        self.add_translation("settings_heading", Language::TraditionalChineseHK, "設定");
        self.add_translation("settings_heading", Language::English, "Settings");
        self.add_translation("settings_heading", Language::Russian, "Настройки");
        self.add_translation("settings_heading", Language::Cat, "喵设");
        
        // History heading
        self.add_translation("history_heading", Language::SimplifiedChinese, "历史记录");
        self.add_translation("history_heading", Language::TraditionalChineseTW, "歷史記錄");
        self.add_translation("history_heading", Language::TraditionalChineseHK, "歷史記錄");
        self.add_translation("history_heading", Language::English, "History");
        self.add_translation("history_heading", Language::Russian, "История");
        self.add_translation("history_heading", Language::Cat, "喵史");
    }
    
    /// Add a translation for a key in a specific language
    fn add_translation(&mut self, key: &'static str, lang: Language, translation: &'static str) {
        self.strings
            .entry(key)
            .or_insert_with(HashMap::new)
            .insert(lang, translation.to_string());
    }
    
    /// Get translation for a key in the specified language
    pub fn get<'a>(&'a self, key: &'a str, lang: Language) -> &'a str {
        self.strings
            .get(key)
            .and_then(|map| map.get(&lang))
            .map(|s| s.as_str())
            .unwrap_or_else(move || {
                // This is a workaround for the lifetime issue
                // In a real implementation, we might want to handle this differently
                key
            })
    }
}

impl Default for Translations {
    fn default() -> Self {
        Self::new()
    }
}