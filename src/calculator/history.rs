//! History management module

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufWriter, Write};

/// Manages calculation history
#[derive(Debug, Serialize, Deserialize)]
pub struct HistoryManager {
    /// Stores expression-result pairs
    history: HashMap<String, String>,
}

impl HistoryManager {
    /// Creates a new history manager
    pub fn new() -> Self {
        Self {
            history: HashMap::new(),
        }
    }

    /// Adds a calculation to history
    pub fn add(&mut self, expression: String, result: String) {
        self.history.insert(expression, result);
    }

    /// Gets all history entries
    pub fn get_all(&self) -> &HashMap<String, String> {
        &self.history
    }

    /// Clears all history
    pub fn clear(&mut self) {
        self.history.clear();
    }

    /// Gets history as a formatted string
    pub fn to_string(&self) -> String {
        let mut result = String::new();
        for (expr, res) in &self.history {
            result.push_str(&format!("{} = {}\n", expr, res));
        }
        result
    }

    /// Saves history to a file
    pub fn save_to_file(&self, filename: &str) -> Result<()> {
        let file = File::create(filename)?;
        let mut writer = BufWriter::new(file);

        for (expr, res) in &self.history {
            writeln!(writer, "{} = {}", expr, res)?;
        }

        Ok(())
    }
}

impl Default for HistoryManager {
    fn default() -> Self {
        Self::new()
    }
}
