//! Mod loader for custom functions defined in TOML format

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Represents a single mod loaded from a .cmfun file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mod {
    #[serde(default)]
    pub desc: ModDesc,
    #[serde(default)]
    pub var: ModVar,
    #[serde(default)]
    pub calc: ModCalc,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModDesc {
    pub name: Option<String>,
}

impl Default for ModDesc {
    fn default() -> Self {
        ModDesc { name: None }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModVar {
    pub needvars: Vec<String>,
}

impl Default for ModVar {
    fn default() -> Self {
        ModVar { needvars: Vec::new() }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModCalc {
    pub howto: Option<String>,
}

impl Default for ModCalc {
    fn default() -> Self {
        ModCalc { howto: None }
    }
}

/// Mod manager that loads and stores mods
#[derive(Debug, Clone)]
pub struct ModManager {
    mods: HashMap<String, Mod>,
    loaded: bool,
}

impl ModManager {
    /// Create a new mod manager
    pub fn new() -> Self {
        ModManager { 
            mods: HashMap::new(),
            loaded: false,
        }
    }

    /// Load all mods from the mods directory
    pub fn load_mods(&mut self) -> Result<(), anyhow::Error> {
        // 使用懒加载机制来减少依赖
        if !self.loaded {
            self.load_mods_from_dir()?;
            self.loaded = true;
        }
        Ok(())
    }

    /// Reload all mods from the mods directory
    pub fn reload_mods(&mut self) -> Result<(), anyhow::Error> {
        // Clear existing mods and reload
        self.mods.clear();
        self.loaded = false;
        self.load_mods_from_dir()?;
        self.loaded = true;
        Ok(())
    }

    fn load_mods_from_dir(&mut self) -> Result<(), anyhow::Error> {
        let mods_dir = Path::new("mods");

        // If mods directory doesn't exist, just return without error
        if !mods_dir.exists() {
            return Ok(());
        }

        if !mods_dir.is_dir() {
            return Err(anyhow::anyhow!("mods is not a directory"));
        }

        // Read all .cmfun files in the mods directory
        for entry in fs::read_dir(mods_dir)? {
            let entry = entry?;
            let path = entry.path();

            // Only process .cmfun files
            if path.extension().and_then(|s| s.to_str()) == Some("cmfun") {
                if let Ok(content) = fs::read_to_string(&path) {
                    // Parse TOML content
                    match toml::from_str::<Mod>(&content) {
                        Ok(mod_def) => {
                            // Use mod name as key, or filename if no name specified
                            let mod_name = mod_def.desc.name.clone().unwrap_or_else(|| {
                                path.file_stem()
                                    .and_then(|s| s.to_str())
                                    .unwrap_or("unnamed")
                                    .to_string()
                            });

                            self.mods.insert(mod_name, mod_def);
                        }
                        Err(e) => {
                            eprintln!("Failed to parse mod file {:?}: {}", path, e);
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// Get a mod by name
    pub fn get_mod(&self, name: &str) -> Option<&Mod> {
        self.mods.get(name)
    }

    /// Get the calculation expression for a mod
    pub fn get_expression(&self, name: &str) -> Option<String> {
        self.mods.get(name).and_then(|m| m.calc.howto.clone())
    }

    /// Get the required variables for a mod
    pub fn get_required_vars(&self, name: &str) -> Option<Vec<String>> {
        self.mods.get(name).map(|m| m.var.needvars.clone())
    }

    /// Get all loaded mod names
    pub fn list_mods(&self) -> Vec<String> {
        self.mods.keys().cloned().collect()
    }
}

impl Default for ModManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mod_parsing() {
        let toml_content = r#"
[desc]
name = "add"

[var]
needvars = ["a", "b"]

[calc]
howto = "a + b"
"#;

        let mod_def: Mod = toml::from_str(toml_content).expect("Failed to parse");
        assert_eq!(mod_def.desc.name, Some("add".to_string()));
        assert_eq!(mod_def.var.needvars, vec!["a", "b"]);
        assert_eq!(mod_def.calc.howto, Some("a + b".to_string()));
    }
}
