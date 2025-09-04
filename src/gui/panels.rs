//! Panel widgets for Emulite
//! 
//! This module provides various panel widgets for the main interface.

use crate::core::*;
use crate::gui::*;
use eframe::egui;
use std::sync::{Arc, Mutex};

/// Main control panel
pub struct ControlPanel {
    is_visible: bool,
    show_advanced: bool,
}

impl ControlPanel {
    pub fn new() -> Self {
        Self {
            is_visible: true,
            show_advanced: false,
        }
    }

    pub fn show(&mut self, ctx: &egui::Context, state: &mut GuiState) {
        if !self.is_visible {
            return;
        }

        egui::SidePanel::left("control_panel")
            .resizable(true)
            .default_width(200.0)
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    ui.heading("Controls");
                    ui.add_space(10.0);

                    // Main controls
                    self.render_main_controls(ui, state);
                    
                    ui.add_space(10.0);
                    ui.separator();
                    ui.add_space(10.0);

                    // Advanced controls
                    ui.checkbox(&mut self.show_advanced, "Advanced");
                    if self.show_advanced {
                        self.render_advanced_controls(ui, state);
                    }

                    ui.add_space(10.0);
                    ui.separator();
                    ui.add_space(10.0);

                    // Status info
                    self.render_status_info(ui, state);
                });
            });
    }

    fn render_main_controls(&mut self, ui: &mut egui::Ui, state: &mut GuiState) {
        ui.group(|ui| {
            ui.set_min_width(ui.available_width());
            ui.vertical(|ui| {
                ui.heading("Main Controls");
                ui.add_space(5.0);

                // Start/Pause/Resume button
                let button_text = if !state.is_running {
                    "Start"
                } else if state.is_paused {
                    "Resume"
                } else {
                    "Pause"
                };

                let can_start = !state.is_running || state.is_paused;
                if ui.add_enabled(can_start, egui::Button::new(button_text)).clicked() {
                    if !state.is_running {
                        let _ = state.start();
                    } else if state.is_paused {
                        let _ = state.resume();
                    } else {
                        let _ = state.pause();
                    }
                }

                // Reset button
                if ui.add_enabled(state.is_running, egui::Button::new("Reset")).clicked() {
                    let _ = state.reset();
                }

                // Stop button
                if ui.add_enabled(state.is_running, egui::Button::new("Stop")).clicked() {
                    let _ = state.stop();
                }
            });
        });
    }

    fn render_advanced_controls(&mut self, ui: &mut egui::Ui, state: &mut GuiState) {
        ui.group(|ui| {
            ui.set_min_width(ui.available_width());
            ui.vertical(|ui| {
                ui.heading("Advanced");
                ui.add_space(5.0);

                // Save state
                if ui.add_enabled(state.is_running, egui::Button::new("Save State")).clicked() {
                    if let Some(emulator) = &state.emulator {
                        let mut emu = emulator.lock().unwrap();
                        // if let Err(e) = emu.save_state("save_state.bin") { // Method not available in this version
                        //     log::error!("Failed to save state: {}", e);
                        // }
                    }
                }

                // Load state
                if ui.add_enabled(state.is_running, egui::Button::new("Load State")).clicked() {
                    if let Some(emulator) = &state.emulator {
                        let mut emu = emulator.lock().unwrap();
                        // if let Err(e) = emu.load_state("save_state.bin") { // Method not available in this version
                        //     log::error!("Failed to load state: {}", e);
                        // }
                    }
                }

                ui.add_space(5.0);

                // Screenshot
                if ui.add_enabled(state.is_running, egui::Button::new("Screenshot")).clicked() {
                    if let Some(emulator) = &state.emulator {
                        let mut emu = emulator.lock().unwrap();
                        // if let Err(e) = emu.take_screenshot("screenshot.png") { // Method not available in this version
                        //     log::error!("Failed to take screenshot: {}", e);
                        // }
                    }
                }

                // Record input
                if ui.add_enabled(state.is_running, egui::Button::new("Record Input")).clicked() {
                    if let Some(emulator) = &state.emulator {
                        let mut emu = emulator.lock().unwrap();
                        // if let Err(e) = emu.start_input_recording("input_recording.bin") { // Method not available in this version
                        //     log::error!("Failed to start input recording: {}", e);
                        // }
                    }
                }
            });
        });
    }

    fn render_status_info(&mut self, ui: &mut egui::Ui, state: &mut GuiState) {
        ui.group(|ui| {
            ui.set_min_width(ui.available_width());
            ui.vertical(|ui| {
                ui.heading("Status");
                ui.add_space(5.0);

                // ROM info
                if let Some(rom_path) = &state.current_rom_path {
                    ui.label(format!("ROM: {}", rom_path));
                } else {
                    ui.label("No ROM loaded");
                }

                // Emulation status
                let status = if state.is_running {
                    if state.is_paused { "Paused" } else { "Running" }
                } else {
                    "Stopped"
                };
                ui.label(format!("Status: {}", status));

                // FPS
                ui.label(format!("FPS: {:.1}", state.fps));

                // Frame count
                ui.label(format!("Frames: {}", state.frame_count));
            });
        });
    }

    pub fn set_visible(&mut self, visible: bool) {
        self.is_visible = visible;
    }

    pub fn is_visible(&self) -> bool {
        self.is_visible
    }
}

/// Debug panel
pub struct DebugPanel {
    is_visible: bool,
    selected_tab: String,
    memory_viewer: MemoryViewer,
    register_viewer: RegisterViewer,
    disassembler: Disassembler,
    breakpoint_manager: BreakpointManager,
}

impl DebugPanel {
    pub fn new() -> Self {
        Self {
            is_visible: false,
            selected_tab: "Registers".to_string(),
            memory_viewer: MemoryViewer::new(),
            register_viewer: RegisterViewer::new(),
            disassembler: Disassembler::new(),
            breakpoint_manager: BreakpointManager::new(),
        }
    }

    pub fn show(&mut self, ctx: &egui::Context, state: &mut GuiState) {
        if !self.is_visible {
            return;
        }

        egui::SidePanel::right("debug_panel")
            .resizable(true)
            .default_width(300.0)
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    ui.heading("Debug Panel");
                    ui.add_space(10.0);

                    // Tab bar
                    ui.horizontal(|ui| {
                        ui.selectable_value(&mut self.selected_tab, "Registers".to_string(), "Registers");
                        ui.selectable_value(&mut self.selected_tab, "Memory".to_string(), "Memory");
                        ui.selectable_value(&mut self.selected_tab, "Disasm".to_string(), "Disasm");
                        ui.selectable_value(&mut self.selected_tab, "Break".to_string(), "Break");
                    });

                    ui.add_space(10.0);
                    ui.separator();
                    ui.add_space(10.0);

                    // Tab content
                    match self.selected_tab.as_str() {
                        "Registers" => self.register_viewer.show(ui, state),
                        "Memory" => self.memory_viewer.show(ui, state),
                        "Disasm" => self.disassembler.show(ui, state),
                        "Break" => self.breakpoint_manager.show(ui, state),
                        _ => {}
                    }
                });
            });
    }

    pub fn set_visible(&mut self, visible: bool) {
        self.is_visible = visible;
    }

    pub fn is_visible(&self) -> bool {
        self.is_visible
    }
}

/// Memory viewer panel
pub struct MemoryViewer {
    start_address: u32,
    bytes_per_line: usize,
    memory_data: Vec<u8>,
    selected_address: Option<u32>,
}

impl MemoryViewer {
    pub fn new() -> Self {
        Self {
            start_address: 0x0000,
            bytes_per_line: 16,
            memory_data: Vec::new(),
            selected_address: None,
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui, state: &mut GuiState) {
        ui.group(|ui| {
            ui.set_min_width(ui.available_width());
            ui.vertical(|ui| {
                ui.heading("Memory Viewer");
                ui.add_space(5.0);

                // Controls
                ui.horizontal(|ui| {
                    ui.label("Address:");
                    ui.add(egui::TextEdit::singleline(&mut format!("0x{:04X}", self.start_address)));
                    
                    ui.label("Per line:");
                    ui.add(egui::Slider::new(&mut self.bytes_per_line, 8..=32));
                });

                ui.add_space(5.0);

                // Memory content
                egui::ScrollArea::vertical()
                    .max_height(300.0)
                    .show(ui, |ui| {
                        if let Some(emulator_state) = state.get_emulator_state() {
                            self.render_memory_content(ui, &emulator_state);
                        } else {
                            ui.label("No emulator running.");
                        }
                    });
            });
        });
    }

    fn render_memory_content(&mut self, ui: &mut egui::Ui, _state: &EmulatorState) {
        // Display memory content in hex format
        ui.label("Address | 00 01 02 03 04 05 06 07 08 09 0A 0B 0C 0D 0E 0F | ASCII");
        ui.label("--------|------------------------------------------------|----------------");
        
        for i in 0..16 {
            let addr = self.start_address + (i * self.bytes_per_line as u32);
            let mut hex_str = String::new();
            let mut ascii_str = String::new();
            
            for j in 0..self.bytes_per_line {
                let byte_addr = addr + j as u32;
                let byte = if byte_addr < 0x10000 { 0x00 } else { 0xFF }; // Placeholder
                
                hex_str.push_str(&format!("{:02X} ", byte));
                ascii_str.push(if byte >= 32 && byte <= 126 { char::from_u32(byte as u32).unwrap_or('.') } else { '.' });
            }
            
            ui.label(format!("{:04X}   | {} | {}", addr, hex_str, ascii_str));
        }
    }
}

/// Register viewer panel
pub struct RegisterViewer {
    registers: std::collections::HashMap<String, u32>,
}

impl RegisterViewer {
    pub fn new() -> Self {
        Self {
            registers: std::collections::HashMap::new(),
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui, state: &mut GuiState) {
        ui.group(|ui| {
            ui.set_min_width(ui.available_width());
            ui.vertical(|ui| {
                ui.heading("Registers");
                ui.add_space(5.0);

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
}

/// Disassembler panel
pub struct Disassembler {
    start_address: u32,
    instruction_count: usize,
    instructions: Vec<DisassemblyLine>,
}

#[derive(Debug, Clone)]
struct DisassemblyLine {
    address: u32,
    bytes: Vec<u8>,
    mnemonic: String,
    operands: String,
}

impl Disassembler {
    pub fn new() -> Self {
        Self {
            start_address: 0x8000,
            instruction_count: 50,
            instructions: Vec::new(),
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui, state: &mut GuiState) {
        ui.group(|ui| {
            ui.set_min_width(ui.available_width());
            ui.vertical(|ui| {
                ui.heading("Disassembler");
                ui.add_space(5.0);

                // Controls
                ui.horizontal(|ui| {
                    ui.label("Address:");
                    ui.add(egui::TextEdit::singleline(&mut format!("0x{:04X}", self.start_address)));
                    
                    ui.label("Count:");
                    ui.add(egui::Slider::new(&mut self.instruction_count, 10..=200));
                });

                ui.add_space(5.0);

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

/// Breakpoint manager panel
pub struct BreakpointManager {
    breakpoints: Vec<Breakpoint>,
    new_breakpoint_address: String,
    new_breakpoint_condition: String,
}

impl BreakpointManager {
    pub fn new() -> Self {
        Self {
            breakpoints: Vec::new(),
            new_breakpoint_address: String::new(),
            new_breakpoint_condition: String::new(),
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui, state: &mut GuiState) {
        ui.group(|ui| {
            ui.set_min_width(ui.available_width());
            ui.vertical(|ui| {
                ui.heading("Breakpoints");
                ui.add_space(5.0);

                // Add new breakpoint
                ui.horizontal(|ui| {
                    ui.label("Address:");
                    ui.text_edit_singleline(&mut self.new_breakpoint_address);
                });

                ui.horizontal(|ui| {
                    ui.label("Condition:");
                    ui.text_edit_singleline(&mut self.new_breakpoint_condition);
                });

                if ui.button("Add").clicked() {
                    if let Ok(address) = u32::from_str_radix(&self.new_breakpoint_address, 16) {
                        let breakpoint = Breakpoint {
                            address,
                            description: if self.new_breakpoint_condition.is_empty() {
                                "Breakpoint".to_string()
                            } else {
                                self.new_breakpoint_condition.clone()
                            },
                            enabled: true,
                        };
                        self.breakpoints.push(breakpoint);
                        self.new_breakpoint_address.clear();
                        self.new_breakpoint_condition.clear();
                    }
                }

                ui.add_space(5.0);
                ui.separator();
                ui.add_space(5.0);

                // Breakpoint list
                egui::ScrollArea::vertical()
                    .max_height(200.0)
                    .show(ui, |ui| {
                        let mut to_remove = Vec::new();
                        for (i, breakpoint) in self.breakpoints.iter_mut().enumerate() {
                            ui.horizontal(|ui| {
                                ui.checkbox(&mut breakpoint.enabled, "");
                                ui.label(format!("0x{:04X}", breakpoint.address));
                                ui.label(format!("({})", breakpoint.description));
                                if ui.button("Remove").clicked() {
                                    to_remove.push(i);
                                }
                            });
                        }
                        // Remove breakpoints in reverse order to maintain indices
                        for &i in to_remove.iter().rev() {
                            self.breakpoints.remove(i);
                        }
                    });
            });
        });
    }
}

/// Status bar panel
pub struct StatusBar {
    is_visible: bool,
    messages: Vec<StatusMessage>,
}

#[derive(Debug, Clone)]
struct StatusMessage {
    text: String,
    level: StatusLevel,
    timestamp: std::time::Instant,
}

#[derive(Debug, Clone)]
enum StatusLevel {
    Info,
    Warning,
    Error,
}

impl StatusBar {
    pub fn new() -> Self {
        Self {
            is_visible: true,
            messages: Vec::new(),
        }
    }

    pub fn show(&mut self, ctx: &egui::Context, state: &mut GuiState) {
        if !self.is_visible {
            return;
        }

        egui::TopBottomPanel::bottom("status_bar")
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    // Status text
                    if let Some(rom_path) = &state.current_rom_path {
                        ui.label(format!("ROM: {}", rom_path));
                    } else {
                        ui.label("No ROM loaded");
                    }

                    ui.separator();

                    // Emulation status
                    let status = if state.is_running {
                        if state.is_paused { "Paused" } else { "Running" }
                    } else {
                        "Stopped"
                    };
                    ui.label(format!("Status: {}", status));

                    ui.separator();

                    // FPS
                    ui.label(format!("FPS: {:.1}", state.fps));

                    ui.separator();

                    // Frame count
                    ui.label(format!("Frames: {}", state.frame_count));

                    // Spacer
                    ui.allocate_ui_at_rect(ui.available_rect_before_wrap(), |ui| {
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            // Recent messages
                            if let Some(message) = self.messages.last() {
                                let color = match message.level {
                                    StatusLevel::Info => egui::Color32::WHITE,
                                    StatusLevel::Warning => egui::Color32::YELLOW,
                                    StatusLevel::Error => egui::Color32::RED,
                                };
                                ui.colored_label(color, &message.text);
                            }
                        });
                    });
                });
            });
    }

    pub fn add_message(&mut self, text: String, level: StatusLevel) {
        self.messages.push(StatusMessage {
            text,
            level,
            timestamp: std::time::Instant::now(),
        });

        // Keep only the last 10 messages
        if self.messages.len() > 10 {
            self.messages.remove(0);
        }
    }

    pub fn set_visible(&mut self, visible: bool) {
        self.is_visible = visible;
    }

    pub fn is_visible(&self) -> bool {
        self.is_visible
    }
}
