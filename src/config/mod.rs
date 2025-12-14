//! Configuration module

/// Application settings
#[derive(Debug)]
pub struct Settings {
    /// Whether to use safe evaluation mode
    pub safe_mode: bool,
    
    /// Whether to automatically save history
    pub auto_save_history: bool,
    
    /// Number of decimal places to display
    pub decimal_places: usize,
}

impl Settings {
    /// Creates new settings with default values
    pub fn new() -> Self {
        Self {
            safe_mode: true,
            auto_save_history: true,
            decimal_places: 10,
        }
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self::new()
    }
}