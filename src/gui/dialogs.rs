//! Dialog boxes for Emulite
//! 
//! This module provides various dialog boxes for user interaction.

use crate::core::*;
use crate::gui::*;
use eframe::egui;

/// File dialog for opening ROMs
pub struct FileDialog {
    current_path: String,
    selected_file: Option<String>,
    file_list: Vec<FileEntry>,
    filter_extensions: Vec<String>,
    show_hidden: bool,
}

#[derive(Debug, Clone)]
struct FileEntry {
    name: String,
    path: String,
    is_directory: bool,
    size: u64,
}

impl FileDialog {
    pub fn new() -> Self {
        Self {
            current_path: std::env::current_dir().unwrap_or_default().to_string_lossy().to_string(),
            selected_file: None,
            file_list: Vec::new(),
            filter_extensions: vec![
                "nes".to_string(),
                "smc".to_string(),
                "sfc".to_string(),
                "bin".to_string(),
                "rom".to_string(),
                "iso".to_string(),
                "cue".to_string(),
            ],
            show_hidden: false,
        }
    }

    pub fn show(&mut self, ctx: &egui::Context, title: &str, callback: impl FnOnce(&str)) {
        let mut open = true;
        
        egui::Window::new(title)
            .open(&mut open)
            .default_size([600.0, 400.0])
            .show(ctx, |ui| {
                self.render_content(ui, callback);
            });
    }

    fn render_content(&mut self, ui: &mut egui::Ui, callback: impl FnOnce(&str)) {
        ui.vertical(|ui| {
            // Path bar
            ui.horizontal(|ui| {
                ui.label("Path:");
                ui.text_edit_singleline(&mut self.current_path);
                if ui.button("Go").clicked() {
                    self.refresh_file_list();
                }
            });

            // Options
            ui.horizontal(|ui| {
                ui.checkbox(&mut self.show_hidden, "Show hidden files");
                if ui.button("Parent Directory").clicked() {
                    self.go_to_parent();
                }
            });

            ui.separator();

            // File list
            egui::ScrollArea::vertical()
                .max_height(300.0)
                .show(ui, |ui| {
                    self.refresh_file_list();
                    
                    for file in &self.file_list {
                        let is_selected = self.selected_file.as_ref() == Some(&file.path);
                        let icon = if file.is_directory { "📁" } else { "📄" };
                        
                        if ui.selectable_label(is_selected, format!("{} {}", icon, file.name)).clicked() {
                            if file.is_directory {
                                self.current_path = file.path.clone();
                                self.selected_file = None;
                            } else {
                                self.selected_file = Some(file.path.clone());
                            }
                        }
                    }
                });

            ui.separator();

            // Buttons
            ui.horizontal(|ui| {
                if ui.add_enabled(self.selected_file.is_some(), egui::Button::new("Open")).clicked() {
                    if let Some(file) = &self.selected_file {
                        callback(file);
                    }
                }

                if ui.button("Cancel").clicked() {
                    // Dialog will be closed
                }
            });
        });
    }

    fn refresh_file_list(&mut self) {
        self.file_list.clear();
        
        if let Ok(entries) = std::fs::read_dir(&self.current_path) {
            for entry in entries.flatten() {
                if let Some(file_name) = entry.file_name().to_str() {
                    if !self.show_hidden && file_name.starts_with('.') {
                        continue;
                    }

                    let path = entry.path().to_string_lossy().to_string();
                    let is_directory = entry.path().is_dir();
                    let size = if is_directory {
                        0
                    } else {
                        entry.metadata().map(|m| m.len()).unwrap_or(0)
                    };

                    if is_directory || self.is_supported_file(&file_name) {
                        self.file_list.push(FileEntry {
                            name: file_name.to_string(),
                            path,
                            is_directory,
                            size,
                        });
                    }
                }
            }
        }
        
        // Sort: directories first, then files
        self.file_list.sort_by(|a, b| {
            match (a.is_directory, b.is_directory) {
                (true, false) => std::cmp::Ordering::Less,
                (false, true) => std::cmp::Ordering::Greater,
                _ => a.name.cmp(&b.name),
            }
        });
    }

    fn is_supported_file(&self, filename: &str) -> bool {
        if let Some(extension) = filename.split('.').last() {
            self.filter_extensions.contains(&extension.to_lowercase())
        } else {
            false
        }
    }

    fn go_to_parent(&mut self) {
        if let Some(parent) = std::path::Path::new(&self.current_path).parent() {
            self.current_path = parent.to_string_lossy().to_string();
            self.selected_file = None;
        }
    }
}

/// Settings dialog
pub struct SettingsDialog {
    selected_tab: String,
    temp_config: Config,
    has_changes: bool,
}

impl SettingsDialog {
    pub fn new(config: &Config) -> Self {
        Self {
            selected_tab: "General".to_string(),
            temp_config: config.clone(),
            has_changes: false,
        }
    }

    pub fn show(&mut self, ctx: &egui::Context, config: &mut Config) -> bool {
        let mut open = true;
        let mut result = false;
        
        egui::Window::new("Settings")
            .open(&mut open)
            .default_size([500.0, 400.0])
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    // Tab bar
                    ui.vertical(|ui| {
                        ui.selectable_value(&mut self.selected_tab, "General".to_string(), "General");
                        ui.selectable_value(&mut self.selected_tab, "Audio".to_string(), "Audio");
                        ui.selectable_value(&mut self.selected_tab, "Video".to_string(), "Video");
                        ui.selectable_value(&mut self.selected_tab, "Input".to_string(), "Input");
                        ui.selectable_value(&mut self.selected_tab, "Debug".to_string(), "Debug");
                    });

                    ui.separator();

                    // Settings content
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        match self.selected_tab.as_str() {
                            "General" => self.render_general_settings(ui),
                            "Audio" => self.render_audio_settings(ui),
                            "Video" => self.render_video_settings(ui),
                            "Input" => self.render_input_settings(ui),
                            "Debug" => self.render_debug_settings(ui),
                            _ => {}
                        }
                    });
                });

                ui.separator();

                // Buttons
                ui.horizontal(|ui| {
                    if ui.button("OK").clicked() {
                        *config = self.temp_config.clone();
                        result = true;
                    }
                    
                    if ui.button("Cancel").clicked() {
                        result = false;
                    }
                    
                    if ui.button("Apply").clicked() {
                        *config = self.temp_config.clone();
                        self.has_changes = false;
                    }
                    
                    if ui.button("Reset").clicked() {
                        self.temp_config = Config::default();
                        self.has_changes = true;
                    }
                });
            });

        result
    }

    fn render_general_settings(&mut self, ui: &mut egui::Ui) {
        ui.group(|ui| {
            ui.set_min_width(ui.available_width());
            ui.vertical(|ui| {
                ui.heading("General Settings");
                ui.add_space(10.0);

                // ui.checkbox(&mut self.temp_config.general.auto_save_state, "Auto-save state"); // Field not available
                
                ui.horizontal(|ui| {
                    ui.label("Save state directory:");
                    // ui.text_edit_singleline(&mut self.temp_config.general.save_state_dir); // Field not available
                });

                ui.horizontal(|ui| {
                    ui.label("Recent ROMs limit:");
                    // ui.add(egui::Slider::new(&mut self.temp_config.general.recent_roms_limit, 1..=20)); // Field not available
                });
            });
        });
    }

    fn render_audio_settings(&mut self, ui: &mut egui::Ui) {
        ui.group(|ui| {
            ui.set_min_width(ui.available_width());
            ui.vertical(|ui| {
                ui.heading("Audio Settings");
                ui.add_space(10.0);

                ui.checkbox(&mut self.temp_config.audio.enabled, "Enable audio");
                
                ui.horizontal(|ui| {
                    ui.label("Sample rate:");
                    ui.add(egui::Slider::new(&mut self.temp_config.audio.sample_rate, 8000..=48000));
                    ui.label("Hz");
                });

                ui.horizontal(|ui| {
                    ui.label("Channels:");
                    ui.add(egui::Slider::new(&mut self.temp_config.audio.channels, 1..=2));
                });

                ui.horizontal(|ui| {
                    ui.label("Volume:");
                    ui.add(egui::Slider::new(&mut self.temp_config.audio.volume, 0.0..=1.0));
                });
            });
        });
    }

    fn render_video_settings(&mut self, ui: &mut egui::Ui) {
        ui.group(|ui| {
            ui.set_min_width(ui.available_width());
            ui.vertical(|ui| {
                ui.heading("Video Settings");
                ui.add_space(10.0);

                ui.checkbox(&mut self.temp_config.video.fullscreen, "Fullscreen");
                ui.checkbox(&mut self.temp_config.video.vsync, "VSync");
                
                ui.horizontal(|ui| {
                    ui.label("Filter:");
                    egui::ComboBox::from_id_source("video_filter")
                        .selected_text(&self.temp_config.video.filter)
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut self.temp_config.video.filter, "None".to_string(), "None");
                            ui.selectable_value(&mut self.temp_config.video.filter, "Linear".to_string(), "Linear");
                            ui.selectable_value(&mut self.temp_config.video.filter, "Nearest".to_string(), "Nearest");
                        });
                });
            });
        });
    }

    fn render_input_settings(&mut self, ui: &mut egui::Ui) {
        ui.group(|ui| {
            ui.set_min_width(ui.available_width());
            ui.vertical(|ui| {
                ui.heading("Input Settings");
                ui.add_space(10.0);

                // ui.checkbox(&mut self.temp_config.input.input_recording, "Enable input recording"); // Field not available
                
                ui.collapsing("Keyboard Mapping", |ui| {
                    ui.label("Keyboard mapping configuration will be implemented here.");
                });

                ui.collapsing("Gamepad Mapping", |ui| {
                    ui.label("Gamepad mapping configuration will be implemented here.");
                });
            });
        });
    }

    fn render_debug_settings(&mut self, ui: &mut egui::Ui) {
        ui.group(|ui| {
            ui.set_min_width(ui.available_width());
            ui.vertical(|ui| {
                ui.heading("Debug Settings");
                ui.add_space(10.0);

                ui.checkbox(&mut self.temp_config.debug.enabled, "Enable debug mode");
                
                ui.horizontal(|ui| {
                    ui.label("Logging level:");
                    egui::ComboBox::from_id_source("logging_level")
                        .selected_text(&self.temp_config.debug.log_level)
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut self.temp_config.debug.log_level, "Error".to_string(), "Error");
                            ui.selectable_value(&mut self.temp_config.debug.log_level, "Warn".to_string(), "Warn");
                            ui.selectable_value(&mut self.temp_config.debug.log_level, "Info".to_string(), "Info");
                            ui.selectable_value(&mut self.temp_config.debug.log_level, "Debug".to_string(), "Debug");
                            ui.selectable_value(&mut self.temp_config.debug.log_level, "Trace".to_string(), "Trace");
                        });
                });
            });
        });
    }
}

/// About dialog
pub struct AboutDialog {
    version: String,
    build_date: String,
}

impl AboutDialog {
    pub fn new() -> Self {
        Self {
            version: env!("CARGO_PKG_VERSION").to_string(),
            build_date: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs()
                .to_string(),
        }
    }

    pub fn show(&mut self, ctx: &egui::Context) {
        let mut open = true;
        
        egui::Window::new("About Emulite")
            .open(&mut open)
            .default_size([400.0, 300.0])
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(20.0);
                    
                    // Logo/Title
                    ui.heading("Emulite");
                    ui.label("Multi-platform Video Game Emulator");
                    ui.add_space(20.0);
                    
                    // Version info
                    ui.label(format!("Version: {}", self.version));
                    ui.label(format!("Build Date: {}", self.build_date));
                    ui.label("Built with Rust");
                    ui.add_space(20.0);
                    
                    // Description
                    ui.label("A modern, high-performance emulator supporting");
                    ui.label("multiple gaming platforms from Atari to PlayStation 3.");
                    ui.add_space(20.0);
                    
                    // Links
                    ui.horizontal(|ui| {
                        if ui.link("GitHub").clicked() {
                            if let Err(e) = open::that("https://github.com/emulite/emulite") {
                                log::error!("Failed to open GitHub: {}", e);
                            }
                        }
                        if ui.link("Documentation").clicked() {
                            if let Err(e) = open::that("https://docs.emulite.dev") {
                                log::error!("Failed to open documentation: {}", e);
                            }
                        }
                        if ui.link("Report Bug").clicked() {
                            if let Err(e) = open::that("https://github.com/emulite/emulite/issues") {
                                log::error!("Failed to open bug report: {}", e);
                            }
                        }
                    });
                    
                    ui.add_space(20.0);
                    
                    // Credits
                    ui.collapsing("Credits", |ui| {
                        ui.label("Developed by the Emulite Team");
                        ui.label("Built with Rust and egui");
                        ui.label("Special thanks to the emulation community");
                    });
                });
            });
    }
}

/// Error dialog
pub struct ErrorDialog {
    title: String,
    message: String,
    details: Option<String>,
}

impl ErrorDialog {
    pub fn new(title: String, message: String) -> Self {
        Self {
            title,
            message,
            details: None,
        }
    }

    pub fn with_details(mut self, details: String) -> Self {
        self.details = Some(details);
        self
    }

    pub fn show(&mut self, ctx: &egui::Context) {
        let mut open = true;
        
        let mut show_dialog = open;
        egui::Window::new(&self.title)
            .open(&mut show_dialog)
            .default_size([400.0, 200.0])
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    // Error icon and message
                    ui.horizontal(|ui| {
                        ui.label("❌");
                        ui.label(&self.message);
                    });
                    
                    // Details (if available)
                    if let Some(details) = &self.details {
                        ui.collapsing("Details", |ui| {
                            ui.label(details);
                        });
                    }
                    
                    ui.add_space(20.0);
                    
                    // OK button
                    ui.horizontal(|ui| {
                        ui.allocate_ui_at_rect(ui.available_rect_before_wrap(), |ui| {
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                if ui.button("OK").clicked() {
                                    // show_dialog = false; // Cannot modify in closure
                                }
                            });
                        });
                    });
                });
            });
    }
}

/// Confirmation dialog
pub struct ConfirmDialog {
    title: String,
    message: String,
    confirm_text: String,
    cancel_text: String,
}

impl ConfirmDialog {
    pub fn new(title: String, message: String) -> Self {
        Self {
            title,
            message,
            confirm_text: "OK".to_string(),
            cancel_text: "Cancel".to_string(),
        }
    }

    pub fn with_buttons(mut self, confirm: String, cancel: String) -> Self {
        self.confirm_text = confirm;
        self.cancel_text = cancel;
        self
    }

    pub fn show(&mut self, ctx: &egui::Context) -> bool {
        let mut open = true;
        let mut result = false;
        
        let mut show_confirm = open;
        egui::Window::new(&self.title)
            .open(&mut show_confirm)
            .default_size([300.0, 150.0])
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    ui.label(&self.message);
                    ui.add_space(20.0);
                    
                    ui.horizontal(|ui| {
                        if ui.button(&self.confirm_text).clicked() {
                            result = true;
                            // show_confirm = false; // Cannot modify in closure
                        }
                        
                        if ui.button(&self.cancel_text).clicked() {
                            result = false;
                            // show_confirm = false; // Cannot modify in closure
                        }
                    });
                });
            });

        result
    }
}

/// Progress dialog
pub struct ProgressDialog {
    title: String,
    message: String,
    progress: f32,
    show_percentage: bool,
}

impl ProgressDialog {
    pub fn new(title: String, message: String) -> Self {
        Self {
            title,
            message,
            progress: 0.0,
            show_percentage: true,
        }
    }

    pub fn set_progress(&mut self, progress: f32) {
        self.progress = progress.clamp(0.0, 1.0);
    }

    pub fn show(&mut self, ctx: &egui::Context) {
        egui::Window::new(&self.title)
            .collapsible(false)
            .resizable(false)
            .default_size([300.0, 100.0])
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    ui.label(&self.message);
                    ui.add_space(10.0);
                    
                    ui.add(egui::ProgressBar::new(self.progress));
                    
                    if self.show_percentage {
                        ui.label(format!("{:.1}%", self.progress * 100.0));
                    }
                });
            });
    }
}
