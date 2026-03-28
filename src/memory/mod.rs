//! Memory management module
//!
//! Handles saving and loading application memory (memory value and history)
//! to system-specific application data directories.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

/// Application memory structure
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Memory {
    /// Memory value (like 'm' in the calculator)
    pub memory_value: f64,

    /// Calculation history
    pub history: HashMap<String, String>,
}

impl Memory {
    /// Creates a new empty memory
    pub fn new() -> Self {
        Self::default()
    }

    /// Gets the memory file path in system-specific application data directory
    pub fn get_memory_file_path() -> Result<PathBuf> {
        // Get the system-specific data directory
        let data_dir = dirs::data_local_dir().ok_or_else(|| anyhow::anyhow!("Failed to get system data directory"))?;

        // Create the calculatorMaxRs directory path
        let app_dir = data_dir.join("calculatorMaxRs");

        // Create the directory if it doesn't exist
        fs::create_dir_all(&app_dir)?;

        // Return the memory.toml file path
        Ok(app_dir.join("memory.toml"))
    }

    /// Loads memory from the system-specific memory file
    pub fn load() -> Result<Self> {
        let file_path = Self::get_memory_file_path()?;

        if !file_path.exists() {
            return Ok(Self::new());
        }

        let content = fs::read_to_string(&file_path)?;
        let memory: Memory = toml::from_str(&content)?;
        Ok(memory)
    }

    /// Saves memory to the system-specific memory file
    pub fn save(&self) -> Result<()> {
        let file_path = Self::get_memory_file_path()?;

        // Serialize to TOML
        let content = toml::to_string_pretty(self)?;

        // Write to file
        fs::write(&file_path, content)?;
        Ok(())
    }

    /// Adds a calculation to history
    pub fn add_to_history(&mut self, expression: String, result: String) {
        self.history.insert(expression, result);
    }

    /// Clears all history
    pub fn clear_history(&mut self) {
        self.history.clear();
    }

    /// Sets the memory value
    pub fn set_memory_value(&mut self, value: f64) {
        self.memory_value = value;
    }

    /// Gets the memory value
    pub fn get_memory_value(&self) -> f64 {
        self.memory_value
    }

    /// Gets all history entries
    pub fn get_history(&self) -> &HashMap<String, String> {
        &self.history
    }
}
