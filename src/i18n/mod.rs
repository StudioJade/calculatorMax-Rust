//! Internationalization (i18n) module for supporting multiple languages

pub mod translations;

// Note: We're not directly re-exporting Translations here to avoid unused import warning
// Users should import it directly from crate::i18n::translations::Translations if needed
