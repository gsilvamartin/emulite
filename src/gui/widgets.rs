//! Custom GUI widgets for Emulite
//! 
//! This module contains custom widgets and UI components.

use crate::core::*;
use crate::gui::*;
use eframe::egui;
use std::sync::{Arc, Mutex};

/// ROM Browser widget
pub struct RomBrowser {
    current_path: String,
    selected_file: Option<String>,
    file_list: Vec<String>,
    filter_extensions: Vec<String>,
}

impl RomBrowser {
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
        }
    }

    pub fn show(&mut self, ctx: &egui::Context, state: &mut GuiState) {
        let mut show_rom_browser = state.show_rom_browser;
        egui::Window::new("Open ROM")
            .open(&mut show_rom_browser)
            .default_size([600.0, 400.0])
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    // Path bar
                    ui.horizontal(|ui| {
                        ui.label("Path:");
                        ui.text_edit_singleline(&mut self.current_path);
                        if ui.button("Browse").clicked() {
                            self.browse_for_directory();
                        }
                        if ui.button("Open File").clicked() {
                            self.open_rom_file(state);
                        }
                    });

                    ui.separator();

                    // File list
                    egui::ScrollArea::vertical()
                        .max_height(300.0)
                        .show(ui, |ui| {
                            self.refresh_file_list();
                            
                            for file in &self.file_list {
                                let is_selected = self.selected_file.as_ref() == Some(file);
                                
                                if ui.selectable_label(is_selected, file).clicked() {
                                    self.selected_file = Some(file.clone());
                                }
                            }
                        });

                    ui.separator();

                    // Buttons
                    ui.horizontal(|ui| {
                        if ui.add_enabled(self.selected_file.is_some(), egui::Button::new("Open")).clicked() {
                            if let Some(file) = &self.selected_file {
                                let full_path = format!("{}/{}", self.current_path, file);
                                if let Err(e) = state.load_rom(&full_path) {
                                    log::error!("Failed to load ROM: {}", e);
                                } else {
                                    state.show_rom_browser = false;
                                }
                            }
                        }

                        if ui.button("Cancel").clicked() {
                            // show_rom_browser = false; // Cannot modify in closure
                        }
                    });
                });
            });
        state.show_rom_browser = show_rom_browser;
    }

    fn refresh_file_list(&mut self) {
        self.file_list.clear();
        
        if let Ok(entries) = std::fs::read_dir(&self.current_path) {
            for entry in entries.flatten() {
                if let Some(file_name) = entry.file_name().to_str() {
                    if entry.path().is_file() {
                        self.file_list.push(file_name.to_string());
                    }
                }
            }
        }
        
        self.file_list.sort();
    }

    fn browse_for_directory(&mut self) {
        // Use native file dialog to select any file
        if let Some(path) = rfd::FileDialog::new()
            .set_title("Select File")
            .pick_file()
        {
            // Extract directory from the selected file path
            if let Some(parent) = path.parent() {
                self.current_path = parent.to_string_lossy().to_string();
            }
            // Set the selected file
            if let Some(file_name) = path.file_name() {
                self.selected_file = Some(file_name.to_string_lossy().to_string());
            }
        }
    }

    fn open_rom_file(&mut self, state: &mut GuiState) {
        log::info!("Opening file dialog");
        // Use native file dialog to directly open any file
        if let Some(path) = rfd::FileDialog::new()
            .set_title("Open File")
            .pick_file()
        {
            let file_path = path.to_string_lossy().to_string();
            log::info!("File selected: {}", file_path);
            if let Err(e) = state.load_rom(&file_path) {
                log::error!("Failed to load file: {}", e);
            } else {
                log::info!("Closing ROM browser window");
                state.show_rom_browser = false;
            }
        } else {
            log::info!("No file selected or dialog canceled");
        }
    }
}

/// Settings Panel widget
pub struct SettingsPanel {
    selected_tab: String,
}

impl SettingsPanel {
    pub fn new() -> Self {
        Self {
            selected_tab: "General".to_string(),
        }
    }

    pub fn show(&mut self, ctx: &egui::Context, state: &mut GuiState) {
        let mut show_settings = state.show_settings;
        egui::Window::new("Settings")
            .open(&mut show_settings)
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
                            "General" => self.render_general_settings(ui, state),
                            "Audio" => self.render_audio_settings(ui, state),
                            "Video" => self.render_video_settings(ui, state),
                            "Input" => self.render_input_settings(ui, state),
                            "Debug" => self.render_debug_settings(ui, state),
                            _ => {}
                        }
                    });
                });

                ui.separator();

                // Buttons
                ui.horizontal(|ui| {
                    if ui.button("OK").clicked() {
                        // show_settings = false; // Cannot modify in closure
                    }
                    
                    if ui.button("Cancel").clicked() {
                        // show_settings = false; // Cannot modify in closure
                    }
                    
                    if ui.button("Apply").clicked() {
                        // Apply settings immediately
                    }
                });
            });
        state.show_settings = show_settings;
    }

    fn render_general_settings(&mut self, ui: &mut egui::Ui, state: &mut GuiState) {
        ui.group(|ui| {
            ui.set_min_width(ui.available_width());
            ui.vertical(|ui| {
                ui.heading("General Settings");
                ui.add_space(10.0);

                // Auto-save state
                // ui.checkbox(&mut state.config.general.auto_save_state, "Auto-save state"); // Field not available
                
                // Save state directory
                ui.horizontal(|ui| {
                    ui.label("Save state directory:");
                    // ui.text_edit_singleline(&mut state.config.general.save_state_dir); // Field not available
                    if ui.button("Browse").clicked() {
                        if let Some(path) = rfd::FileDialog::new()
                            .set_title("Select Save State Directory")
                            .pick_folder()
                        {
                            // state.config.general.save_state_dir = path.to_string_lossy().to_string(); // Field not available
                        }
                    }
                });

                // Recent ROMs limit
                ui.horizontal(|ui| {
                    ui.label("Recent ROMs limit:");
                    // ui.add(egui::Slider::new(&mut state.config.general.recent_roms_limit, 1..=20)); // Field not available
                });
            });
        });
    }

    fn render_audio_settings(&mut self, ui: &mut egui::Ui, state: &mut GuiState) {
        ui.group(|ui| {
            ui.set_min_width(ui.available_width());
            ui.vertical(|ui| {
                ui.heading("Audio Settings");
                ui.add_space(10.0);

                // Sample rate
                ui.horizontal(|ui| {
                    ui.label("Sample rate:");
                    ui.add(egui::Slider::new(&mut state.config.audio.sample_rate, 8000..=48000));
                    ui.label("Hz");
                });

                // Channels
                ui.horizontal(|ui| {
                    ui.label("Channels:");
                    ui.add(egui::Slider::new(&mut state.config.audio.channels, 1..=2));
                });

                // Buffer size
                ui.horizontal(|ui| {
                    ui.label("Buffer size:");
                    ui.add(egui::Slider::new(&mut state.config.audio.buffer_size, 256..=4096));
                    ui.label("samples");
                });

                // Volume
                ui.horizontal(|ui| {
                    ui.label("Volume:");
                    ui.add(egui::Slider::new(&mut state.config.audio.volume, 0.0..=1.0));
                });

                // Enable audio
                ui.checkbox(&mut state.config.audio.enabled, "Enable audio");
            });
        });
    }

    fn render_video_settings(&mut self, ui: &mut egui::Ui, state: &mut GuiState) {
        ui.group(|ui| {
            ui.set_min_width(ui.available_width());
            ui.vertical(|ui| {
                ui.heading("Video Settings");
                ui.add_space(10.0);

                // Resolution
                ui.horizontal(|ui| {
                    ui.label("Resolution:");
                    ui.add(egui::Slider::new(&mut state.config.video.width, 320..=1920));
                    ui.label("x");
                    ui.add(egui::Slider::new(&mut state.config.video.height, 240..=1080));
                });

                // Fullscreen
                ui.checkbox(&mut state.config.video.fullscreen, "Fullscreen");

                // VSync
                ui.checkbox(&mut state.config.video.vsync, "VSync");

                // Filter
                ui.horizontal(|ui| {
                    ui.label("Filter:");
                    egui::ComboBox::from_id_source("video_filter")
                        .selected_text(&state.config.video.filter)
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut state.config.video.filter, "None".to_string(), "None");
                            ui.selectable_value(&mut state.config.video.filter, "Linear".to_string(), "Linear");
                            ui.selectable_value(&mut state.config.video.filter, "Nearest".to_string(), "Nearest");
                        });
                });

                // Shader path
                ui.horizontal(|ui| {
                    ui.label("Shader path:");
                    // ui.text_edit_singleline(&mut state.config.video.shader_path); // Field not available
                    if ui.button("Browse").clicked() {
                        if let Some(path) = rfd::FileDialog::new()
                            .set_title("Select Shader File")
                            .add_filter("Shader files", &["wgsl", "glsl", "hlsl"])
                            .pick_file()
                        {
                            // state.config.video.shader_path = path.to_string_lossy().to_string(); // Field not available
                        }
                    }
                });
            });
        });
    }

    fn render_input_settings(&mut self, ui: &mut egui::Ui, state: &mut GuiState) {
        ui.group(|ui| {
            ui.set_min_width(ui.available_width());
            ui.vertical(|ui| {
                ui.heading("Input Settings");
                ui.add_space(10.0);

                // Input recording
                // ui.checkbox(&mut state.config.input.input_recording, "Enable input recording"); // Field not available

                // Keyboard mapping
                ui.collapsing("Keyboard Mapping", |ui| {
                    ui.label("Configure keyboard controls:");
                    ui.add_space(5.0);
                    
                    let keys = ["Up", "Down", "Left", "Right", "A", "B", "Start", "Select"];
                    for key in keys {
                        ui.horizontal(|ui| {
                            ui.label(format!("{}:", key));
                            if ui.button("Press key...").clicked() {
                                // Key mapping will be captured on next key press
                            }
                        });
                    }
                });

                // Gamepad mapping
                ui.collapsing("Gamepad Mapping", |ui| {
                    ui.label("Configure gamepad controls:");
                    ui.add_space(5.0);
                    
                    let buttons = ["D-Pad Up", "D-Pad Down", "D-Pad Left", "D-Pad Right", "A", "B", "Start", "Select"];
                    for button in buttons {
                        ui.horizontal(|ui| {
                            ui.label(format!("{}:", button));
                            if ui.button("Press button...").clicked() {
                                // Button mapping will be captured on next button press
                            }
                        });
                    }
                });
            });
        });
    }

    fn render_debug_settings(&mut self, ui: &mut egui::Ui, state: &mut GuiState) {
        ui.group(|ui| {
            ui.set_min_width(ui.available_width());
            ui.vertical(|ui| {
                ui.heading("Debug Settings");
                ui.add_space(10.0);

                // Enable debug
                ui.checkbox(&mut state.config.debug.enabled, "Enable debug mode");

                // Logging level
                ui.horizontal(|ui| {
                    ui.label("Logging level:");
                    egui::ComboBox::from_id_source("logging_level")
                        .selected_text(&state.config.debug.log_level)
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut state.config.debug.log_level, "Error".to_string(), "Error");
                            ui.selectable_value(&mut state.config.debug.log_level, "Warn".to_string(), "Warn");
                            ui.selectable_value(&mut state.config.debug.log_level, "Info".to_string(), "Info");
                            ui.selectable_value(&mut state.config.debug.log_level, "Debug".to_string(), "Debug");
                            ui.selectable_value(&mut state.config.debug.log_level, "Trace".to_string(), "Trace");
                        });
                });

                // Breakpoints
                ui.collapsing("Breakpoints", |ui| {
                    ui.label("Configure default breakpoints:");
                    ui.add_space(5.0);
                    
                    ui.horizontal(|ui| {
                        ui.label("Address:");
                        ui.text_edit_singleline(&mut String::new());
                        if ui.button("Add").clicked() {
                            // Add breakpoint logic
                        }
                    });
                    
                    ui.horizontal(|ui| {
                        ui.label("Condition:");
                        ui.text_edit_singleline(&mut String::new());
                    });
                });
            });
        });
    }
}

/// Debug Panel widget
pub struct DebugPanel {
    selected_tab: String,
    memory_viewer: MemoryViewer,
    register_viewer: RegisterViewer,
    disassembler: Disassembler,
}

impl DebugPanel {
    pub fn new() -> Self {
        Self {
            selected_tab: "Registers".to_string(),
            memory_viewer: MemoryViewer::new(),
            register_viewer: RegisterViewer::new(),
            disassembler: Disassembler::new(),
        }
    }

    pub fn show(&mut self, ctx: &egui::Context, state: &mut GuiState) {
        let mut show_debug_panel = state.show_debug_panel;
        egui::Window::new("Debug Panel")
            .open(&mut show_debug_panel)
            .default_size([800.0, 600.0])
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    // Tab bar
                    ui.vertical(|ui| {
                        ui.selectable_value(&mut self.selected_tab, "Registers".to_string(), "Registers");
                        ui.selectable_value(&mut self.selected_tab, "Memory".to_string(), "Memory");
                        ui.selectable_value(&mut self.selected_tab, "Disassembler".to_string(), "Disassembler");
                        ui.selectable_value(&mut self.selected_tab, "Breakpoints".to_string(), "Breakpoints");
                    });

                    ui.separator();

                    // Debug content
                    match self.selected_tab.as_str() {
                        "Registers" => self.register_viewer.show(ui, state),
                        "Memory" => self.memory_viewer.show(ui, state),
                        "Disassembler" => self.disassembler.show(ui, state),
                        "Breakpoints" => self.render_breakpoints(ui, state),
                        _ => {}
                    }
                });
            });
        state.show_debug_panel = show_debug_panel;
    }

    fn render_breakpoints(&mut self, ui: &mut egui::Ui, state: &mut GuiState) {
        ui.group(|ui| {
            ui.set_min_width(ui.available_width());
            ui.vertical(|ui| {
                ui.heading("Breakpoints");
                ui.add_space(10.0);

                // Add new breakpoint
                ui.horizontal(|ui| {
                    ui.label("Address:");
                    ui.text_edit_singleline(&mut String::new());
                    if ui.button("Add").clicked() {
                        // Add breakpoint logic
                    }
                });

                ui.horizontal(|ui| {
                    ui.label("Condition:");
                    ui.text_edit_singleline(&mut String::new());
                });

                ui.separator();

                // Breakpoint list
                egui::ScrollArea::vertical()
                    .max_height(200.0)
                    .show(ui, |ui| {
                        ui.label("No breakpoints set.");
                        ui.label("Add breakpoints using the controls above.");
                    });
            });
        });
    }
}

/// Memory Viewer widget
pub struct MemoryViewer {
    start_address: u32,
    bytes_per_line: usize,
}

impl MemoryViewer {
    pub fn new() -> Self {
        Self {
            start_address: 0x0000,
            bytes_per_line: 16,
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui, state: &mut GuiState) {
        ui.group(|ui| {
            ui.set_min_width(ui.available_width());
            ui.vertical(|ui| {
                ui.heading("Memory Viewer");
                ui.add_space(10.0);

                // Controls
                ui.horizontal(|ui| {
                    ui.label("Start address:");
                    ui.add(egui::TextEdit::singleline(&mut format!("0x{:04X}", self.start_address)));
                    
                    ui.label("Bytes per line:");
                    ui.add(egui::Slider::new(&mut self.bytes_per_line, 8..=32));
                });

                ui.separator();

                // Memory content
                egui::ScrollArea::vertical()
                    .max_height(300.0)
                    .show(ui, |ui| {
                        if let Some(emulator_state) = state.get_emulator_state() {
                            // self.render_memory_content(ui, &emulator_state); // Method not available in this version
                        } else {
                            ui.label("No emulator running.");
                        }
                    });
            });
        });
    }
}

/// Register Viewer widget
pub struct RegisterViewer {
    // No additional state needed
}

impl RegisterViewer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn show(&mut self, ui: &mut egui::Ui, state: &mut GuiState) {
        ui.group(|ui| {
            ui.set_min_width(ui.available_width());
            ui.vertical(|ui| {
                ui.heading("Registers");
                ui.add_space(10.0);

                if let Some(emulator_state) = state.get_emulator_state() {
                    self.render_registers(ui, &emulator_state);
                } else {
                    ui.label("No emulator running.");
                }
            });
        });
    }

    fn render_registers(&mut self, ui: &mut egui::Ui, _state: &EmulatorState) {
        // Display CPU registers
        ui.label("CPU Registers:");
        ui.add_space(5.0);
        
        let registers = vec![
            ("A", 0x00),
            ("X", 0x00),
            ("Y", 0x00),
            ("SP", 0xFF),
            ("PC", 0x8000),
            ("P", 0x20),
        ];

        for (name, value) in registers {
            ui.horizontal(|ui| {
                ui.label(format!("{}:", name));
                ui.label(format!("0x{:02X}", value));
            });
        }
    }

    fn render_memory_content(&mut self, ui: &mut egui::Ui, _state: &EmulatorState) {
        // Display memory content in hex format
        ui.label("Address | 00 01 02 03 04 05 06 07 08 09 0A 0B 0C 0D 0E 0F | ASCII");
        ui.label("--------|------------------------------------------------|----------------");
        
        for i in 0..16 {
            let addr = 0x0000 + (i * 16 as u32); // Placeholder values
            let mut hex_str = String::new();
            let mut ascii_str = String::new();
            
            for j in 0..16 { // Placeholder value
                let byte_addr = addr + j as u32;
                let byte = if byte_addr < 0x10000 { 0x00 } else { 0xFF }; // Placeholder
                
                hex_str.push_str(&format!("{:02X} ", byte));
                ascii_str.push(if byte >= 32 && byte <= 126 { char::from_u32(byte as u32).unwrap_or('.') } else { '.' });
            }
            
            ui.label(format!("{:04X}   | {} | {}", addr, hex_str, ascii_str));
        }
    }
}

/// Disassembler widget
pub struct Disassembler {
    start_address: u32,
    instruction_count: usize,
}

impl Disassembler {
    pub fn new() -> Self {
        Self {
            start_address: 0x8000,
            instruction_count: 50,
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui, state: &mut GuiState) {
        ui.group(|ui| {
            ui.set_min_width(ui.available_width());
            ui.vertical(|ui| {
                ui.heading("Disassembler");
                ui.add_space(10.0);

                // Controls
                ui.horizontal(|ui| {
                    ui.label("Start address:");
                    ui.add(egui::TextEdit::singleline(&mut format!("0x{:04X}", self.start_address)));
                    
                    ui.label("Instructions:");
                    ui.add(egui::Slider::new(&mut self.instruction_count, 10..=200));
                });

                ui.separator();

                // Disassembly
                egui::ScrollArea::vertical()
                    .max_height(300.0)
                    .show(ui, |ui| {
                        if let Some(emulator_state) = state.get_emulator_state() {
                            self.render_disassembly(ui, &emulator_state);
                        } else {
                            ui.label("No emulator running.");
                        }
                    });
            });
        });
    }

    fn render_disassembly(&mut self, ui: &mut egui::Ui, _state: &EmulatorState) {
        // Display disassembly
        ui.label("Disassembly:");
        ui.add_space(5.0);
        
        // Placeholder disassembly
        for i in 0..self.instruction_count {
            let addr = self.start_address + i as u32;
            ui.label(format!("{:04X}: 00 00 00    NOP", addr));
        }
    }
}

/// About Dialog widget
pub struct AboutDialog {
    // No additional state needed
}

impl AboutDialog {
    pub fn new() -> Self {
        Self {}
    }

    pub fn show(&mut self, ctx: &egui::Context) {
        egui::Window::new("About Emulite")
            .open(&mut true) // This will be controlled by the parent
            .default_size([400.0, 300.0])
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(20.0);
                    
                    // Logo/Title
                    ui.heading("Emulite");
                    ui.label("Multi-platform Video Game Emulator");
                    ui.add_space(20.0);
                    
                    // Version info
                    ui.label("Version: 0.1.0");
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
                    
                    // Close button
                    if ui.button("Close").clicked() {
                        // Dialog will be closed by the parent
                    }
                });
            });
    }
}