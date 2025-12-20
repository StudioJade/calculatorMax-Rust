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
    #[serde(default)]
    pub res: ModRes,
}

impl Default for Mod {
    fn default() -> Self {
        Mod {
            desc: ModDesc::default(),
            var: ModVar::default(),
            calc: ModCalc::default(),
            res: ModRes::default(),
        }
    }
}

/// New flat mod structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlatMod {
    #[serde(flatten)]
    pub sections: std::collections::HashMap<String, ModSection>,
}

/// Section of a mod
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModSection {
    #[serde(default)]
    pub desc: Option<ModDesc>,
    #[serde(default)]
    pub vars: Option<ModVars>,
    #[serde(default)]
    pub calc: Option<ModCalc>,
    #[serde(default)]
    pub res: Option<ModRes>,
}

/// Definition of a single mod in the nested structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModDefinition {
    #[serde(default)]
    pub desc: ModDesc,
    #[serde(default)]
    pub vars: Option<ModVars>,
    #[serde(default)]
    pub calc: Option<ModCalc>,
    #[serde(default)]
    pub res: Option<ModRes>,
}

/// Simplified mod structure for the new format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimplifiedMod {
    pub name: Option<String>,
    #[serde(default)]
    pub needs: Option<Vec<String>>,
    pub method: Option<String>,
    pub res: Option<f64>,
    #[serde(rename = "type")]
    pub mod_type: Option<String>,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModRes {
    pub res: Option<f64>,
}

impl Default for ModRes {
    fn default() -> Self {
        ModRes { res: None }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModVars {
    pub needs: Option<std::collections::HashMap<String, String>>,
}

impl Default for ModVars {
    fn default() -> Self {
        ModVars { needs: None }
    }
}

/// Mod manager that loads and stores mods
#[derive(Debug, Clone)]
pub struct ModManager {
    mods: HashMap<String, Mod>,
    loaded: bool,
    warnings: Vec<String>,
}

impl ModManager {
    /// Create a new mod manager
    pub fn new() -> Self {
        ModManager {
            mods: HashMap::new(),
            loaded: false,
            warnings: Vec::new(),
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
        // Clear previous warnings
        self.warnings.clear();
        
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
                    // Check if this is the new format ([x.x.x] table headers)
                    if content.contains("[") && content.contains("=") && !content.contains("[desc]") {
                        // This looks like the new simplified format with table structure
                        
                        // Split content into sections by table headers
                        let mut sections: Vec<String> = Vec::new();
                        let mut current_section = String::new();
                        
                        for line in content.lines() {
                            if line.starts_with('[') && !current_section.is_empty() {
                                // New section started, save the previous one
                                sections.push(current_section.clone());
                                current_section.clear();
                            }
                            current_section.push_str(line);
                            current_section.push('\n');
                        }
                        
                        // Don't forget the last section
                        if !current_section.is_empty() {
                            sections.push(current_section);
                        }
                        
                        // Parse each section individually
                        for section in sections {
                            if section.trim().is_empty() {
                                continue;
                            }
                            
                            // Extract mod ID from the first line which should be [mod.id]
                            if let Some(first_line) = section.lines().next() {
                                if first_line.starts_with('[') && first_line.ends_with(']') {
                                    let mod_id = first_line[1..first_line.len()-1].to_string();
                                    
                                    // Remove the first line (table header) and parse the rest as SimplifiedMod
                                    let content_without_header = section.lines().skip(1).collect::<Vec<_>>().join("\n");
                                    
                                    // Try to parse the content as a single SimplifiedMod
                                    if let Ok(simplified_mod) = toml::from_str::<SimplifiedMod>(&content_without_header) {
                                        // Use mod_id as the mod name to preserve the full identifier
                                        let mod_name = mod_id.clone();
                                        
                                        // Convert to legacy Mod structure for compatibility
                                        let legacy_mod = Mod {
                                            desc: ModDesc {
                                                name: simplified_mod.name.clone(),
                                            },
                                            var: ModVar {
                                                needvars: simplified_mod.needs.unwrap_or_else(Vec::new),
                                            },
                                            calc: ModCalc {
                                                howto: simplified_mod.method,
                                            },
                                            res: ModRes {
                                                res: simplified_mod.res,
                                            },
                                        };
                                        
                                        self.mods.insert(mod_name, legacy_mod);
                                    } else {
                                        eprintln!("Warning: Failed to parse mod section in file {:?}: Invalid format", path);
                                    }
                                }
                            }
                        }
                        
                        // Successfully parsed as simplified format, continue to next file
                        continue;
                    } else if content.contains("[desc]") {
                        // This is the old format, show warning and skip
                        self.warnings.push(format!("Warning: Skipping old format mod file {:?}. Please convert to new format [x.x.x].", path));
                        continue;
                    } else {
                        // Unknown format, show warning and skip
                        self.warnings.push(format!("Warning: Skipping unknown format mod file {:?}.", path));
                        continue;
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

impl ModManager {
    /// Get warnings collected during mod loading
    pub fn get_warnings(&self) -> &[String] {
        &self.warnings
    }
    
    /// Clear warnings
    pub fn clear_warnings(&mut self) {
        self.warnings.clear();
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
