//! GUI implementation using egui

use anyhow::Result;
use eframe::egui;

use crate::calculator::{Evaluator, HistoryManager};
use crate::calculator::mods::{Mod, ModDesc, ModVar, ModCalc};
use crate::config::Settings;
use crate::i18n::translations::{Language, Translations};

/// Runs the GUI application
pub fn run_gui() -> Result<()> {
    let mut options = eframe::NativeOptions::default();

    // Enable OpenGL renderer which generally has better font support
    options.renderer = eframe::Renderer::Glow;

    eframe::run_native(
        "Calculator Max",
        options,
        Box::new(|cc| {
            // Set up Chinese font support
            setup_chinese_fonts(&cc.egui_ctx);

            // Set a Chinese-friendly theme
            cc.egui_ctx.set_visuals(egui::Visuals::light());

            Ok(Box::new(CalculatorApp::default()) as Box<dyn eframe::App>)
        }),
    )
    .map_err(|e| anyhow::anyhow!("Failed to start GUI: {}", e))
}

/// Sets up Chinese font support for the application
fn setup_chinese_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();

    // Try to load embedded font if it exists
    #[cfg(feature = "embedded-fonts")]
    {
        // Add the embedded Harmony OS Sans SC font
        fonts.font_data.insert(
            "HarmonyOS_Sans_SC".to_owned(),
            egui::FontData::from_static(include_bytes!("../../assets/fonts/font.ttf")).into(),
        );

        // Use it for both proportional and monospace text
        fonts
            .families
            .entry(egui::FontFamily::Proportional)
            .or_default()
            .insert(0, "HarmonyOS_Sans_SC".to_owned());

        fonts
            .families
            .entry(egui::FontFamily::Monospace)
            .or_default()
            .insert(0, "HarmonyOS_Sans_SC".to_owned());
    }

    // Fallback: try to use system fonts
    #[cfg(not(feature = "embedded-fonts"))]
    {
        let mut font_family = fonts
            .families
            .get(&egui::FontFamily::Proportional)
            .cloned()
            .unwrap_or_default();

        // Add common Chinese system fonts at the beginning
        let chinese_fonts = [
            "Microsoft YaHei",
            "SimHei",
        ];

        for font in chinese_fonts.iter() {
            font_family.insert(0, font.to_string());
        }

        fonts
            .families
            .insert(egui::FontFamily::Proportional, font_family.clone());
        fonts.families.insert(egui::FontFamily::Monospace, font_family);
    }

    ctx.set_fonts(fonts);
}

/// Main application structure
struct CalculatorApp {
    /// Current input expression
    expression: String,

    /// Last calculation result
    result: String,

    /// Error message if any
    error: String,

    /// History manager
    history: HistoryManager,

    /// Application settings
    settings: Settings,

    /// Evaluator
    evaluator: Evaluator,

    /// Whether to show history
    show_history: bool,

    /// Memory value (like 'm' in the Python version)
    memory: f64,

    /// Whether to show settings
    show_settings: bool,

    /// Filename for saving history
    history_filename: String,

    /// Current language
    language: Language,

    /// Translations
    translations: Translations,

    /// Whether to show mod creator
    show_mod_creator: bool,

    /// Mod creator state
    mod_creator: ModCreator,
}

/// State for the mod creator UI
#[derive(Debug, Clone)]
struct ModCreator {
    /// Name of the mod
    name: String,

    /// Description of the mod
    description: String,

    /// Required variables
    required_vars: String,

    /// Calculation expression
    expression: String,

    /// Filename for saving
    filename: String,

    /// Success message
    success_message: String,

    /// Error message
    error_message: String,
}

impl Default for CalculatorApp {
    fn default() -> Self {
        Self {
            expression: String::new(),
            result: String::new(),
            error: String::new(),
            history: HistoryManager::default(),
            settings: Settings::default(),
            evaluator: Evaluator::default(),
            show_history: false,
            memory: 0.0,
            show_settings: false,
            history_filename: String::new(),
            language: Language::English, // Default to English
            translations: Translations::default(),
            show_mod_creator: false,
            mod_creator: ModCreator::default(),
        }
    }
}

impl Default for ModCreator {
    fn default() -> Self {
        Self {
            name: String::new(),
            description: String::new(),
            required_vars: String::new(),
            expression: String::new(),
            filename: String::new(),
            success_message: String::new(),
            error_message: String::new(),
        }
    }
}

impl CalculatorApp {
    /// Processes the current expression
    fn calculate(&mut self) {
        // Clear previous error
        self.error.clear();

        match self.evaluator.evaluate(&self.expression) {
            Ok(value) => {
                self.result = value.to_string();
                // Add to history
                self.history.add(self.expression.clone(), self.result.clone());
                // Store in memory
                self.memory = value;
            }
            Err(e) => {
                self.error = e.to_string();
                self.result = "Error".to_string();
            }
        }
    }

    /// Clears the history
    fn clear_history(&mut self) {
        self.history.clear();
    }

    /// Saves history to file
    fn save_history(&mut self) {
        if self.history_filename.is_empty() {
            self.error = "Please enter a filename".to_string();
            return;
        }

        match self.history.save_to_file(&self.history_filename) {
            Ok(_) => {
                self.error = format!("History saved to {}", self.history_filename);
            }
            Err(e) => {
                self.error = format!("Failed to save history: {}", e);
            }
        }
    }

    /// Saves a mod to file
    fn save_mod(&mut self) {
        // Clear previous messages
        self.mod_creator.success_message.clear();
        self.mod_creator.error_message.clear();

        // Validate inputs
        if self.mod_creator.name.is_empty() {
            self.mod_creator.error_message = "Mod name is required".to_string();
            return;
        }

        if self.mod_creator.filename.is_empty() {
            self.mod_creator.error_message = "Filename is required".to_string();
            return;
        }

        // Parse required variables
        let required_vars: Vec<String> = if self.mod_creator.required_vars.is_empty() {
            Vec::new()
        } else {
            self.mod_creator
                .required_vars
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect()
        };

        // Create mod structure
        let mod_def = Mod {
            desc: ModDesc {
                name: Some(self.mod_creator.name.clone()),
            },
            var: ModVar {
                needvars: required_vars,
            },
            calc: ModCalc {
                howto: if self.mod_creator.expression.is_empty() {
                    None
                } else {
                    Some(self.mod_creator.expression.clone())
                },
            },
        };

        // Serialize to TOML
        match toml::to_string_pretty(&mod_def) {
            Ok(toml_content) => {
                // Save to file
                let filename = if self.mod_creator.filename.ends_with(".cmfun") {
                    self.mod_creator.filename.clone()
                } else {
                    format!("{}.cmfun", self.mod_creator.filename)
                };

                match std::fs::write(format!("mods/{}", filename), toml_content) {
                    Ok(_) => {
                        self.mod_creator.success_message = format!("Mod saved to {}", filename);
                        // Reload mods in the evaluator
                        if let Err(e) = self.evaluator.reload_mods() {
                            self.mod_creator.error_message = format!("Mod saved but failed to reload: {}", e);
                        }
                        // Reset form
                        self.mod_creator = ModCreator::default();
                    }
                    Err(e) => {
                        self.mod_creator.error_message = format!("Failed to save mod: {}", e);
                    }
                }
            }
            Err(e) => {
                self.mod_creator.error_message = format!("Failed to serialize mod: {}", e);
            }
        }
    }
}

impl eframe::App for CalculatorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(self.translations.get("app_title", self.language));

            // Show error if any
            if !self.error.is_empty() {
                ui.colored_label(egui::Color32::RED, &self.error);
            }

            // Input field
            ui.horizontal(|ui| {
                ui.label(self.translations.get("expression", self.language));
                ui.text_edit_singleline(&mut self.expression);
                if ui.button(self.translations.get("calculate", self.language)).clicked() {
                    self.calculate();
                }
            });

            // Result display
            ui.horizontal(|ui| {
                ui.label(self.translations.get("result", self.language));
                ui.label(&self.result);
            });

            // Memory display
            ui.horizontal(|ui| {
                ui.label(self.translations.get("memory", self.language));
                ui.label(self.memory.to_string());
            });

            // Language selector
            ui.horizontal(|ui| {
                ui.label("Language:");
                egui::ComboBox::from_label("Language")
                    .selected_text(self.language.code())
                    .show_ui(ui, |ui| {
                        for lang in Language::all() {
                            ui.selectable_value(&mut self.language, lang, lang.code());
                        }
                    });
            });

            // Controls
            ui.separator();

            ui.horizontal_wrapped(|ui| {
                if ui.button(self.translations.get("history", self.language)).clicked() {
                    self.show_history = !self.show_history;
                }

                if ui
                    .button(self.translations.get("clear_history", self.language))
                    .clicked()
                {
                    self.clear_history();
                }

                if ui.button(self.translations.get("settings", self.language)).clicked() {
                    self.show_settings = !self.show_settings;
                }

                if ui.button(self.translations.get("exit", self.language)).clicked() {
                    std::process::exit(0);
                }

                if ui.button(self.translations.get("create_mod", self.language)).clicked() {
                    self.show_mod_creator = !self.show_mod_creator;
                }
            });

            // Show settings if requested
            if self.show_settings {
                ui.separator();
                ui.heading(self.translations.get("settings_heading", self.language));
                ui.horizontal(|ui| {
                    if ui
                        .checkbox(
                            &mut self.settings.safe_mode,
                            self.translations.get("safe_mode", self.language),
                        )
                        .changed()
                    {
                        self.evaluator.set_safe_mode(self.settings.safe_mode);
                    }
                    ui.label("(Uncheck for extended functionality)");
                });
            }

            // Show history if requested
            if self.show_history {
                ui.separator();
                ui.heading(self.translations.get("history_heading", self.language));
                ui.label(self.history.to_string());

                // Add option to save history to file
                ui.horizontal(|ui| {
                    ui.label(self.translations.get("filename", self.language));
                    ui.text_edit_singleline(&mut self.history_filename);
                    if ui
                        .button(self.translations.get("save_history", self.language))
                        .clicked()
                    {
                        self.save_history();
                    }
                });
            }

            // Show mod creator if requested
            if self.show_mod_creator {
                ui.separator();
                ui.heading(self.translations.get("create_mod_heading", self.language));
                
                // Show success message if any
                if !self.mod_creator.success_message.is_empty() {
                    ui.colored_label(egui::Color32::GREEN, &self.mod_creator.success_message);
                }
                
                // Show error message if any
                if !self.mod_creator.error_message.is_empty() {
                    ui.colored_label(egui::Color32::RED, &self.mod_creator.error_message);
                }
                
                ui.horizontal(|ui| {
                    ui.label(self.translations.get("mod_name", self.language));
                    ui.text_edit_singleline(&mut self.mod_creator.name);
                });
                
                ui.horizontal(|ui| {
                    ui.label(self.translations.get("mod_description", self.language));
                    ui.text_edit_singleline(&mut self.mod_creator.description);
                });
                
                ui.horizontal(|ui| {
                    ui.label(self.translations.get("mod_required_vars", self.language));
                    ui.text_edit_singleline(&mut self.mod_creator.required_vars);
                });
                
                ui.horizontal(|ui| {
                    ui.label(self.translations.get("mod_expression", self.language));
                    ui.text_edit_singleline(&mut self.mod_creator.expression);
                });
                
                ui.horizontal(|ui| {
                    ui.label(self.translations.get("mod_filename", self.language));
                    ui.text_edit_singleline(&mut self.mod_creator.filename);
                });
                
                if ui.button(self.translations.get("save_mod", self.language)).clicked() {
                    self.save_mod();
                }
                
                if ui.button(self.translations.get("cancel", self.language)).clicked() {
                    self.show_mod_creator = false;
                    self.mod_creator = ModCreator::default();
                }
            }

            // Add some spacing
            ui.allocate_space(egui::Vec2::new(1.0, 10.0));
        });
    }
}
