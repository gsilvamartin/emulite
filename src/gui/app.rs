//! Main GUI application
//! 
//! This module contains the main application structure and event handling.

use crate::core::*;
use crate::config::*;
use crate::gui::*;
use eframe::egui;
use std::sync::{Arc, Mutex};

/// Main Emulite application
pub struct EmuliteApp {
    state: GuiState,
    theme: GuiTheme,
    rom_browser: RomBrowser,
    settings_panel: SettingsPanel,
    debug_panel: DebugPanel,
    about_dialog: AboutDialog,
}

impl EmuliteApp {
    pub fn new() -> Self {
        Self {
            state: GuiState::new(),
            theme: GuiTheme::default(),
            rom_browser: RomBrowser::new(),
            settings_panel: SettingsPanel::new(),
            debug_panel: DebugPanel::new(),
            about_dialog: AboutDialog::new(),
        }
    }

    /// Handle keyboard shortcuts
    fn handle_shortcuts(&mut self, ctx: &egui::Context) {
        let input = ctx.input(|i| i.clone());
        
        // Ctrl+O: Open ROM
        if input.modifiers.ctrl && input.key_pressed(egui::Key::O) {
            self.state.show_rom_browser = true;
        }
        
        // Ctrl+S: Settings
        if input.modifiers.ctrl && input.key_pressed(egui::Key::S) {
            self.state.show_settings = true;
        }
        
        // F5: Start/Pause
        if input.key_pressed(egui::Key::F5) {
            if self.state.is_running {
                if self.state.is_paused {
                    let _ = self.state.resume();
                } else {
                    let _ = self.state.pause();
                }
            } else {
                let _ = self.state.start();
            }
        }
        
        // F6: Reset
        if input.key_pressed(egui::Key::F6) {
            let _ = self.state.reset();
        }
        
        // F7: Stop
        if input.key_pressed(egui::Key::F7) {
            let _ = self.state.stop();
        }
        
        // F12: Debug panel
        if input.key_pressed(egui::Key::F12) {
            self.state.show_debug_panel = !self.state.show_debug_panel;
        }
        
        // Escape: Close dialogs
        if input.key_pressed(egui::Key::Escape) {
            self.state.show_rom_browser = false;
            self.state.show_settings = false;
            self.state.show_about = false;
        }
    }

    /// Render the main menu bar
    fn render_menu_bar(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                // File menu
                ui.menu_button("File", |ui| {
                    if ui.button("Open ROM...").clicked() {
                        self.state.show_rom_browser = true;
                        ui.close_menu();
                    }
                    
                    ui.separator();
                    
                    if ui.button("Recent ROMs").clicked() {
                        // Show recent ROMs dialog
                        ui.close_menu();
                    }
                    
                    ui.separator();
                    
                    if ui.button("Exit").clicked() {
                        // Close the application
                        std::process::exit(0);
                    }
                });

                // Emulation menu
                ui.menu_button("Emulation", |ui| {
                    let can_start = !self.state.is_running || self.state.is_paused;
                    let can_pause = self.state.is_running && !self.state.is_paused;
                    let can_resume = self.state.is_running && self.state.is_paused;
                    let can_reset = self.state.is_running;
                    let can_stop = self.state.is_running;

                    if ui.add_enabled(can_start, egui::Button::new("Start")).clicked() {
                        let _ = self.state.start();
                        ui.close_menu();
                    }
                    
                    if ui.add_enabled(can_pause, egui::Button::new("Pause")).clicked() {
                        let _ = self.state.pause();
                        ui.close_menu();
                    }
                    
                    if ui.add_enabled(can_resume, egui::Button::new("Resume")).clicked() {
                        let _ = self.state.resume();
                        ui.close_menu();
                    }
                    
                    ui.separator();
                    
                    if ui.add_enabled(can_reset, egui::Button::new("Reset")).clicked() {
                        let _ = self.state.reset();
                        ui.close_menu();
                    }
                    
                    if ui.add_enabled(can_stop, egui::Button::new("Stop")).clicked() {
                        let _ = self.state.stop();
                        ui.close_menu();
                    }
                });

                // View menu
                ui.menu_button("View", |ui| {
                    if ui.button("Debug Panel").clicked() {
                        self.state.show_debug_panel = !self.state.show_debug_panel;
                        ui.close_menu();
                    }
                    
                    if ui.button("Settings").clicked() {
                        self.state.show_settings = true;
                        ui.close_menu();
                    }
                });

                // Help menu
                ui.menu_button("Help", |ui| {
                    if ui.button("About").clicked() {
                        self.state.show_about = true;
                        ui.close_menu();
                    }
                });

                // Status info
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if let Some(rom_path) = &self.state.current_rom_path {
                        ui.label(format!("ROM: {}", rom_path));
                    }
                    
                    if self.state.is_running {
                        let status = if self.state.is_paused { "Paused" } else { "Running" };
                        ui.label(format!("Status: {}", status));
                    }
                    
                    ui.label(format!("FPS: {:.1}", self.state.fps));
                });
            });
        });
    }

    /// Render the main content area
    fn render_main_content(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(emulator) = self.state.emulator.clone() {
                // Render emulator screen
                self.render_emulator_screen(ui, &emulator);
            } else {
                // Welcome screen
                self.render_welcome_screen(ui);
            }
        });
    }

    /// Render the emulator screen
    fn render_emulator_screen(&mut self, ui: &mut egui::Ui, emulator: &Arc<Mutex<Emulator>>) {
        let available_size = ui.available_size();
        let aspect_ratio = 4.0 / 3.0; // Default aspect ratio
        
        let screen_size = if available_size.x / available_size.y > aspect_ratio {
            egui::vec2(available_size.y * aspect_ratio, available_size.y)
        } else {
            egui::vec2(available_size.x, available_size.x / aspect_ratio)
        };

        let screen_rect = egui::Rect::from_center_size(
            ui.available_rect_before_wrap().center(),
            screen_size
        );

        // Get frame data from emulator
        if let Ok(mut emu) = emulator.lock() {
            // Execute emulator steps to run the game
            if emu.is_running() {
                // Run multiple steps per frame for better performance
                for _ in 0..100 {
                    if let Err(e) = emu.step() {
                        log::error!("Emulator step error: {:?}", e);
                        break;
                    }
                }
            }
            
            if let Ok(frame_data) = emu.get_frame_data() {
                // Create texture from frame data
                let texture = self.create_texture_from_frame(ui.ctx(), &frame_data);
                
                // Render the texture
                ui.allocate_ui_at_rect(screen_rect, |ui| {
                    ui.image((texture.id(), screen_size));
                });
            }
        }

        // Update FPS
        self.state.update_fps();
    }

    /// Render the welcome screen
    fn render_welcome_screen(&mut self, ui: &mut egui::Ui) {
        ui.allocate_ui_at_rect(ui.available_rect_before_wrap(), |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(ui.available_height() * 0.3);
                
                // Title
                ui.heading("Emulite");
                ui.add_space(20.0);
                
                // Subtitle
                ui.label("Multi-platform Video Game Emulator");
                ui.add_space(40.0);
                
                // Open ROM button
                if ui.add_sized([200.0, 40.0], egui::Button::new("Open ROM")).clicked() {
                    log::info!("Open ROM button clicked");
                    self.state.show_rom_browser = true;
                }
                
                ui.add_space(20.0);
                
                // Quick start info
                ui.group(|ui| {
                    ui.set_min_width(ui.available_width());
                    ui.vertical(|ui| {
                        ui.heading("Quick Start");
                        ui.add_space(10.0);
                        
                        ui.horizontal(|ui| {
                            ui.label("• Press");
                            ui.code("Ctrl+O");
                            ui.label("to open a ROM file");
                        });
                        
                        ui.horizontal(|ui| {
                            ui.label("• Press");
                            ui.code("F5");
                            ui.label("to start/pause emulation");
                        });
                        
                        ui.horizontal(|ui| {
                            ui.label("• Press");
                            ui.code("F12");
                            ui.label("to open debug panel");
                        });
                        
                        ui.horizontal(|ui| {
                            ui.label("• Press");
                            ui.code("Ctrl+S");
                            ui.label("to open settings");
                        });
                    });
                });
            });
        });
    }

    /// Create texture from frame data
    fn create_texture_from_frame(&self, ctx: &egui::Context, frame_data: &[u8]) -> egui::TextureHandle {
        // Frame data is already in RGBA format (4 bytes per pixel)
        // Calculate dimensions from data size
        let pixel_count = frame_data.len() / 4;
        let width = 256; // Standard Atari 2600 width
        let height = pixel_count / width;
        
        // Ensure we have enough data
        if frame_data.len() < width * height * 4 {
            // Create a fallback texture with correct size
            let mut fallback_data = vec![0u8; width * height * 4];
            for (i, pixel) in fallback_data.chunks_exact_mut(4).enumerate() {
                let x = i % width;
                let y = i / width;
                pixel[0] = ((x * 255) / width) as u8; // Red
                pixel[1] = ((y * 255) / height) as u8; // Green
                pixel[2] = 128; // Blue
                pixel[3] = 255; // Alpha
            }
            let texture = egui::ColorImage::from_rgba_unmultiplied([width, height], &fallback_data);
            return ctx.load_texture("emulator_frame", texture, Default::default());
        }

        // Create texture with correct dimensions
        let texture = egui::ColorImage::from_rgba_unmultiplied([width, height], frame_data);
        ctx.load_texture("emulator_frame", texture, Default::default())
    }

    /// Render dialogs and panels
    fn render_dialogs(&mut self, ctx: &egui::Context) {
        // ROM Browser
        if self.state.show_rom_browser {
            self.rom_browser.show(ctx, &mut self.state);
        }

        // Settings Panel
        if self.state.show_settings {
            self.settings_panel.show(ctx, &mut self.state);
        }

        // Debug Panel
        if self.state.show_debug_panel {
            self.debug_panel.show(ctx, &mut self.state);
        }

        // About Dialog
        if self.state.show_about {
            self.about_dialog.show(ctx);
        }
    }
}

impl eframe::App for EmuliteApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Handle shortcuts
        self.handle_shortcuts(ctx);

        // Apply theme
        apply_theme(ctx, &self.theme);

        // Render UI
        self.render_menu_bar(ctx);
        self.render_main_content(ctx);
        self.render_dialogs(ctx);

        // Request repaint for smooth animation
        if self.state.is_running && !self.state.is_paused {
            ctx.request_repaint();
        }
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        // Save configuration
        // TODO: Implement config save
        // if let Err(e) = self.state.config.save_to_file("emulite.toml") {
        //     log::error!("Failed to save configuration: {}", e);
        // }
    }
}
