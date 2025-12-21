//! GUI implementation using egui

use anyhow::Result;
use eframe::egui;

use crate::calculator::mods::SimplifiedMod;
use crate::calculator::{Evaluator, HistoryManager};
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
        let chinese_fonts = ["Microsoft YaHei", "SimHei"];

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

    /// Warning messages if any
    warnings: Vec<String>,

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

    /// Suggestions for the current input
    suggestions: Vec<String>,

    /// Selected suggestion index
    selected_suggestion: usize,

    /// Whether to show mod list
    show_mod_list: bool,
}

/// State for the mod creator UI
#[derive(Debug, Clone)]
struct ModCreator {
    /// Mod ID (a.b.c format)
    mod_id: String,

    /// Name of the mod
    name: String,

    /// Description of the mod
    description: String,

    /// Type of the mod ("fun" for function, "num" for constant)
    mod_type: String,

    /// Required variables
    required_vars: String,

    /// Calculation expression
    expression: String,

    /// Constant value (for num type)
    constant_value: String,

    /// Filename for saving
    filename: String,

    /// Success message
    success_message: String,

    /// Error message
    error_message: String,
}

impl Default for CalculatorApp {
    fn default() -> Self {
        // Detect system language, default to English if detection fails
        let detected_language = Language::detect_system_language();

        // Create evaluator and get warnings
        let evaluator = Evaluator::default();
        let warnings: Vec<String> = evaluator.get_warnings().to_vec();

        Self {
            expression: String::new(),
            result: String::new(),
            error: String::new(),
            warnings,
            history: HistoryManager::default(),
            settings: Settings::default(),
            evaluator,
            show_history: false,
            memory: 0.0,
            show_settings: false,
            history_filename: String::new(),
            language: detected_language,
            translations: Translations::default(),
            show_mod_creator: false,
            mod_creator: ModCreator::default(),
            suggestions: Vec::new(),
            selected_suggestion: 0,
            show_mod_list: false,
        }
    }
}

impl Default for ModCreator {
    fn default() -> Self {
        Self {
            mod_id: String::new(),
            name: String::new(),
            description: String::new(),
            mod_type: "fun".to_string(), // Default to function type
            required_vars: String::new(),
            expression: String::new(),
            constant_value: String::new(),
            filename: String::new(),
            success_message: String::new(),
            error_message: String::new(),
        }
    }
}

impl CalculatorApp {
    /// Generates suggestions based on current input
    fn generate_suggestions(&mut self) {
        // Clear previous suggestions
        self.suggestions.clear();
        self.selected_suggestion = 0;

        // If expression is empty, show common functions
        if self.expression.is_empty() {
            self.suggestions.extend([
                "sin()".to_string(),
                "cos()".to_string(),
                "tan()".to_string(),
                "sqrt()".to_string(),
                "log()".to_string(),
                "exp()".to_string(),
                "pi".to_string(),
                "e".to_string(),
            ]);
            return;
        }

        // Get the last token (word or partial word)
        let tokens: Vec<&str> = self.expression.split(|c: char| !c.is_alphabetic()).collect();
        if let Some(last_token) = tokens.last() {
            if !last_token.is_empty() {
                // Suggest built-in functions
                let builtin_functions = [
                    "sin",
                    "cos",
                    "csin",
                    "tan",
                    "asin",
                    "acos",
                    "atan",
                    "sinh",
                    "cosh",
                    "tanh",
                    "exp",
                    "sqrt",
                    "log",
                    "log10",
                    "log2",
                    "ceil",
                    "floor",
                    "trunc",
                    "fabs",
                    "factorial",
                    "gamma",
                    "erf",
                    "erfc",
                    "degrees",
                    "radians",
                    "s_circle",
                    "s_tri",
                    "s_rect",
                ];

                // Filter functions that start with the last token
                for func in builtin_functions.iter() {
                    if func.starts_with(last_token) {
                        // Add opening parenthesis for functions
                        self.suggestions.push(format!("{}()", func));
                    }
                }

                // Suggest constants
                let constants = ["pi", "e"];
                for constant in constants.iter() {
                    if constant.starts_with(last_token) {
                        self.suggestions.push(constant.to_string());
                    }
                }

                // Suggest custom mods
                let mod_list = self.evaluator.list_mods();
                for mod_name in mod_list {
                    if mod_name.starts_with(last_token) {
                        // Get required variables for this mod
                        if let Some(vars) = self.evaluator.get_required_vars(&mod_name) {
                            let args = vars.join(", ");
                            self.suggestions.push(format!("{}({})", mod_name, args));
                        } else {
                            self.suggestions.push(format!("{}()", mod_name));
                        }
                    }
                }
            }
        }

        // Limit suggestions to 10 items
        self.suggestions.truncate(10);
    }

    /// Get the display name for a language in its own language
    fn get_language_display_name(&self, language: Language, _display_language: Language) -> String {
        match language {
            Language::Auto => {
                let auto_lang = Language::detect_system_language();
                let auto_lang_name = match auto_lang {
                    Language::SimplifiedChinese => "简体中文".to_string(),
                    Language::TraditionalChineseTW => "繁體中文（台灣）".to_string(),
                    Language::TraditionalChineseHK => "繁體中文（香港）".to_string(),
                    Language::English => "English".to_string(),
                    Language::Russian => "Русский".to_string(),
                    Language::Cat => "喵语".to_string(),
                    _ => "Unknown".to_string(),
                };
                format!("自动 ({})", auto_lang_name)
            }
            Language::SimplifiedChinese => "简体中文".to_string(),
            Language::TraditionalChineseTW => "繁體中文（台灣）".to_string(),
            Language::TraditionalChineseHK => "繁體中文（香港）".to_string(),
            Language::English => "English".to_string(),
            Language::Russian => "Русский".to_string(),
            Language::Cat => "喵语".to_string(),
        }
    }

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
        if self.mod_creator.mod_id.is_empty() {
            self.mod_creator.error_message = "Mod ID is required".to_string();
            return;
        }

        // Validate mod ID format (should be a.b.c)
        if !self.mod_creator.mod_id.contains('.') {
            self.mod_creator.error_message = "Mod ID must be in a.b.c format".to_string();
            return;
        }

        if self.mod_creator.name.is_empty() {
            self.mod_creator.error_message = "Mod name is required".to_string();
            return;
        }

        if self.mod_creator.filename.is_empty() {
            self.mod_creator.error_message = "Filename is required".to_string();
            return;
        }

        // Validate based on mod type
        if self.mod_creator.mod_type == "fun" {
            if self.mod_creator.expression.is_empty() {
                self.mod_creator.error_message = "Expression is required for function mods".to_string();
                return;
            }
        } else if self.mod_creator.mod_type == "num" {
            if self.mod_creator.constant_value.is_empty() {
                self.mod_creator.error_message = "Constant value is required for constant mods".to_string();
                return;
            }
        } else {
            self.mod_creator.error_message = "Invalid mod type".to_string();
            return;
        }

        // Parse required variables for function mods
        let required_vars: Vec<String> =
            if self.mod_creator.mod_type == "fun" && !self.mod_creator.required_vars.is_empty() {
                self.mod_creator
                    .required_vars
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect()
            } else {
                Vec::new()
            };

        // Create simplified mod structure
        let mut mods_map = std::collections::HashMap::new();

        let mod_id = self.mod_creator.mod_id.clone();

        let simplified_mod = if self.mod_creator.mod_type == "fun" {
            SimplifiedMod {
                name: Some(self.mod_creator.name.clone()),
                needs: if required_vars.is_empty() {
                    None
                } else {
                    Some(required_vars)
                },
                method: if self.mod_creator.expression.is_empty() {
                    None
                } else {
                    Some(self.mod_creator.expression.clone())
                },
                res: None, // For function mods, res is None
                mod_type: Some("fun".to_string()),
            }
        } else {
            // Parse constant value
            let constant_value: f64 = match self.mod_creator.constant_value.parse() {
                Ok(val) => val,
                Err(_) => {
                    self.mod_creator.error_message = "Invalid constant value".to_string();
                    return;
                }
            };

            SimplifiedMod {
                name: Some(self.mod_creator.name.clone()),
                needs: None,  // Constants don't need variables
                method: None, // Constants don't have methods
                res: Some(constant_value),
                mod_type: Some("num".to_string()),
            }
        };

        mods_map.insert(mod_id, simplified_mod);

        // Serialize to TOML manually to match the desired format
        let mut toml_content = String::new();
        for (mod_id, mod_def) in mods_map {
            toml_content.push_str(&format!("[{}]\n", mod_id));
            if let Some(name) = mod_def.name {
                toml_content.push_str(&format!("name = \"{}\"\n", name));
            }
            if let Some(mod_type) = mod_def.mod_type {
                toml_content.push_str(&format!("type = \"{}\"\n", mod_type));
            }
            if let Some(needs) = mod_def.needs {
                toml_content.push_str("needs = [");
                for (i, need) in needs.iter().enumerate() {
                    if i > 0 {
                        toml_content.push_str(", ");
                    }
                    toml_content.push_str(&format!("\"{}\"", need));
                }
                toml_content.push_str("]\n");
            }
            if let Some(method) = mod_def.method {
                toml_content.push_str(&format!("method = \"{}\"\n", method));
            }
            if let Some(res) = mod_def.res {
                toml_content.push_str(&format!("res = {}\n", res));
            }
            toml_content.push_str("\n");
        }

        // Use the manually created TOML content
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
}

impl eframe::App for CalculatorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Check for global Tab key press for suggestion completion
        if !self.suggestions.is_empty() && ctx.input(|i| i.key_pressed(egui::Key::Tab)) {
            // Apply selected suggestion
            if self.selected_suggestion < self.suggestions.len() {
                let suggestion = self.suggestions[self.selected_suggestion].clone();
                if let Some(last_space) = self.expression.rfind(|c: char| !c.is_alphabetic()) {
                    self.expression = self.expression[..=last_space].to_string() + &suggestion;
                } else {
                    self.expression = suggestion;
                }

                // Clear suggestions
                self.suggestions.clear();
                self.selected_suggestion = 0;

                // Request repaint to update UI
                ctx.request_repaint();
            }
        }

        // Check for global Enter key press for calculation
        if ctx.input(|i| i.key_pressed(egui::Key::Enter)) {
            self.calculate();
            ctx.request_repaint();
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            let display_language = if self.language == Language::Auto {
                Language::detect_system_language()
            } else {
                self.language
            };

            // Create a scrollable area for the main content
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.heading(self.translations.get("app_title", display_language));

                // Show error if any
                if !self.error.is_empty() {
                    ui.colored_label(egui::Color32::RED, &self.error);
                }

                // Show warnings if any
                for warning in &self.warnings {
                    ui.colored_label(egui::Color32::LIGHT_RED, warning);
                }

                // Input field with suggestions
                ui.horizontal(|ui| {
                    ui.label(self.translations.get("expression", display_language));

                    // Create a text edit widget
                    let response = ui.text_edit_singleline(&mut self.expression);

                    // Generate suggestions when the text changes
                    if response.changed() {
                        self.generate_suggestions();
                    }

                    // Handle keyboard events when text edit has focus
                    if response.has_focus() {
                        // Check for Enter key press to calculate
                        if ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                            self.calculate();
                        }

                        // Handle Tab key completion when there are suggestions
                        if !self.suggestions.is_empty() {
                            // Check for Tab key press in the global input
                            if ui.input(|i| i.key_pressed(egui::Key::Tab)) {
                                // Apply selected suggestion
                                if self.selected_suggestion < self.suggestions.len() {
                                    let suggestion = self.suggestions[self.selected_suggestion].clone();
                                    if let Some(last_space) = self.expression.rfind(|c: char| !c.is_alphabetic()) {
                                        self.expression = self.expression[..=last_space].to_string() + &suggestion;
                                    } else {
                                        self.expression = suggestion;
                                    }

                                    // Clear suggestions
                                    self.suggestions.clear();
                                    self.selected_suggestion = 0;

                                    // Request focus removal to prevent further processing
                                    response.surrender_focus();
                                }
                            }

                            // Handle arrow keys for navigation
                            if ui.input(|i| i.key_pressed(egui::Key::ArrowDown)) {
                                self.selected_suggestion = (self.selected_suggestion + 1) % self.suggestions.len();
                            }
                            if ui.input(|i| i.key_pressed(egui::Key::ArrowUp)) {
                                self.selected_suggestion = if self.selected_suggestion > 0 {
                                    self.selected_suggestion - 1
                                } else {
                                    self.suggestions.len() - 1
                                };
                            }
                            if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                                // Clear suggestions when pressing Escape
                                self.suggestions.clear();
                                self.selected_suggestion = 0;
                            }
                        }
                    }

                    if ui
                        .button(self.translations.get("calculate", display_language))
                        .clicked()
                    {
                        self.calculate();
                    }
                });

                // Display suggestions if any
                if !self.suggestions.is_empty() {
                    ui.vertical(|ui| {
                        let mut clicked_index: Option<usize> = None;
                        for (i, suggestion) in self.suggestions.iter().enumerate() {
                            let response = ui.button(suggestion);
                            if i == self.selected_suggestion {
                                // Highlight selected suggestion
                                ui.painter().rect_filled(
                                    response.rect,
                                    egui::Rounding::same(2.0),
                                    egui::Color32::from_rgba_premultiplied(0, 120, 255, 30),
                                );
                            }
                            if response.clicked() {
                                clicked_index = Some(i);
                            }
                        }

                        // Apply clicked suggestion outside the loop to avoid borrowing issues
                        if let Some(index) = clicked_index {
                            if index < self.suggestions.len() {
                                let suggestion = &self.suggestions[index];
                                if let Some(last_space) = self.expression.rfind(|c: char| !c.is_alphabetic()) {
                                    self.expression = self.expression[..=last_space].to_string() + suggestion;
                                } else {
                                    self.expression = suggestion.clone();
                                }

                                // Clear suggestions
                                self.suggestions.clear();
                                self.selected_suggestion = 0;
                            }
                        }
                    });
                }

                // Result display
                ui.horizontal(|ui| {
                    ui.label(self.translations.get("result", display_language));
                    ui.label(&self.result);
                });

                // Memory display
                ui.horizontal(|ui| {
                    // Get the translated memory label and remove the (m) part if present, then add colon
                    let memory_label = self.translations.get("memory", display_language);
                    let display_label = if memory_label.ends_with(" (m):") {
                        format!("{}:", &memory_label[..memory_label.len() - 5])
                    } else if memory_label.ends_with(" (m)") {
                        format!("{}:", &memory_label[..memory_label.len() - 4])
                    } else {
                        memory_label
                    };
                    ui.label(display_label);
                    ui.label(self.memory.to_string());
                });

                // Language selector
                ui.horizontal(|ui| {
                    ui.label(self.translations.get("language_label", display_language));
                    egui::ComboBox::from_id_source("language_selector")
                        .selected_text(self.get_language_display_name(self.language, display_language))
                        .show_ui(ui, |ui| {
                            for lang in Language::all() {
                                let display_text = self.get_language_display_name(lang, display_language);
                                ui.selectable_value(&mut self.language, lang, display_text);
                            }
                        });
                });

                // Controls
                ui.separator();

                ui.horizontal_wrapped(|ui| {
                    if ui.button(self.translations.get("history", display_language)).clicked() {
                        self.show_history = !self.show_history;
                    }

                    if ui
                        .button(self.translations.get("clear_history", display_language))
                        .clicked()
                    {
                        self.clear_history();
                    }

                    if ui.button(self.translations.get("settings", display_language)).clicked() {
                        self.show_settings = !self.show_settings;
                    }

                    if ui
                        .button(self.translations.get("show_mods", display_language))
                        .clicked()
                    {
                        self.show_mod_list = !self.show_mod_list;
                    }

                    if ui
                        .button(self.translations.get("create_mod", display_language))
                        .clicked()
                    {
                        self.show_mod_creator = !self.show_mod_creator;
                    }

                    if ui.button(self.translations.get("exit", display_language)).clicked() {
                        std::process::exit(0);
                    }
                });

                // Show settings if requested
                if self.show_settings {
                    ui.separator();
                    ui.heading(self.translations.get("settings_heading", display_language));
                    ui.horizontal(|ui| {
                        if ui
                            .checkbox(
                                &mut self.settings.safe_mode,
                                self.translations.get("safe_mode", display_language),
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
                    ui.heading(self.translations.get("history_heading", display_language));
                    ui.label(self.history.to_string());

                    // Add option to save history to file
                    ui.horizontal(|ui| {
                        ui.label(self.translations.get("filename", display_language));
                        ui.text_edit_singleline(&mut self.history_filename);
                        if ui
                            .button(self.translations.get("save_history", display_language))
                            .clicked()
                        {
                            self.save_history();
                        }
                    });
                }

                // Show mod creator if requested
                if self.show_mod_creator {
                    ui.separator();
                    ui.heading(self.translations.get("create_mod_heading", display_language));

                    // Show success message if any
                    if !self.mod_creator.success_message.is_empty() {
                        ui.colored_label(egui::Color32::GREEN, &self.mod_creator.success_message);
                    }

                    // Show error message if any
                    if !self.mod_creator.error_message.is_empty() {
                        ui.colored_label(egui::Color32::RED, &self.mod_creator.error_message);
                    }

                    ui.horizontal(|ui| {
                        ui.label(self.translations.get("mod_id", display_language));
                        ui.text_edit_singleline(&mut self.mod_creator.mod_id);
                    });

                    ui.horizontal(|ui| {
                        ui.label(self.translations.get("mod_name", display_language));
                        ui.text_edit_singleline(&mut self.mod_creator.name);
                    });

                    ui.horizontal(|ui| {
                        ui.label(self.translations.get("mod_description", display_language));
                        ui.text_edit_singleline(&mut self.mod_creator.description);
                    });

                    ui.horizontal(|ui| {
                        ui.label(self.translations.get("mod_type", display_language));
                        let selected_text = if self.mod_creator.mod_type == "fun" {
                            self.translations.get("mod_type_function", display_language)
                        } else {
                            self.translations.get("mod_type_constant", display_language)
                        };

                        egui::ComboBox::from_id_source("mod_type")
                            .selected_text(selected_text)
                            .show_ui(ui, |ui: &mut egui::Ui| {
                                ui.selectable_value(
                                    &mut self.mod_creator.mod_type,
                                    "fun".to_string(),
                                    self.translations.get("mod_type_function", display_language),
                                );
                                ui.selectable_value(
                                    &mut self.mod_creator.mod_type,
                                    "num".to_string(),
                                    self.translations.get("mod_type_constant", display_language),
                                );
                            });
                    });

                    // Show different fields based on mod type
                    if self.mod_creator.mod_type == "fun" {
                        ui.horizontal(|ui| {
                            ui.label(self.translations.get("mod_required_vars", display_language));
                            ui.text_edit_singleline(&mut self.mod_creator.required_vars);
                        });

                        ui.horizontal(|ui| {
                            ui.label(self.translations.get("mod_expression", display_language));
                            ui.text_edit_singleline(&mut self.mod_creator.expression);
                        });
                    } else {
                        ui.horizontal(|ui| {
                            ui.label(self.translations.get("mod_constant_value", display_language));
                            ui.text_edit_singleline(&mut self.mod_creator.constant_value);
                        });
                    }

                    ui.horizontal(|ui| {
                        ui.label(self.translations.get("mod_filename", display_language));
                        ui.text_edit_singleline(&mut self.mod_creator.filename);
                    });

                    if ui.button(self.translations.get("save_mod", display_language)).clicked() {
                        self.save_mod();
                    }

                    if ui.button(self.translations.get("cancel", display_language)).clicked() {
                        self.show_mod_creator = false;
                        self.mod_creator = ModCreator::default();
                    }
                }

                // Show mod list if requested
                if self.show_mod_list {
                    ui.separator();
                    ui.heading(self.translations.get("loaded_mods", display_language));

                    // Get list of mods
                    let mod_list = self.evaluator.list_mods();
                    if mod_list.is_empty() {
                        ui.label(self.translations.get("no_mods_loaded", display_language));
                    } else {
                        // Create a scrollable area for the mod list
                        egui::ScrollArea::vertical().max_height(200.0).show(ui, |ui| {
                            for mod_name in mod_list {
                                // Get mod details
                                if let Some(mod_def) = self.evaluator.get_mod(&mod_name) {
                                    let display_name = mod_def.desc.name.clone().unwrap_or_else(|| mod_name.clone());
                                    ui.horizontal(|ui| {
                                        ui.label(format!(
                                            "{}: {}",
                                            self.translations.get("mod_id_display", display_language),
                                            mod_name
                                        ));
                                        ui.label(format!(
                                            "{}: {}",
                                            self.translations.get("mod_name_display", display_language),
                                            display_name
                                        ));
                                    });
                                    ui.separator();
                                }
                            }
                        });
                    }
                }

                // Add some spacing
                ui.allocate_space(egui::Vec2::new(1.0, 10.0));
            }); // End of ScrollArea
        });
    }
}
