//! GUI module for Emulite
//! 
//! This module provides a modern graphical user interface for the emulator
//! using the egui framework.

pub mod app;
pub mod widgets;
pub mod themes;
pub mod dialogs;
pub mod panels;

pub use app::EmuliteApp;
pub use widgets::*;
pub use themes::*;
pub use dialogs::*;
pub use panels::*;

use crate::core::{Emulator, EmulatorState, Breakpoint};
use crate::{EmuliteResult, EmuliteError};
use crate::platforms::PlatformFactory;
use crate::config::*;
use eframe::egui;
use std::sync::{Arc, Mutex};

/// Main GUI application state
pub struct GuiState {
    pub emulator: Option<Arc<Mutex<Emulator>>>,
    pub config: Config,
    pub show_debug_panel: bool,
    pub show_settings: bool,
    pub show_about: bool,
    pub show_rom_browser: bool,
    pub current_rom_path: Option<String>,
    pub is_running: bool,
    pub is_paused: bool,
    pub frame_count: u64,
    pub fps: f32,
    pub last_frame_time: std::time::Instant,
}

impl Default for GuiState {
    fn default() -> Self {
        Self {
            emulator: None,
            config: Config::default(),
            show_debug_panel: false,
            show_settings: false,
            show_about: false,
            show_rom_browser: false,
            current_rom_path: None,
            is_running: false,
            is_paused: false,
            frame_count: 0,
            fps: 0.0,
            last_frame_time: std::time::Instant::now(),
        }
    }
}

impl GuiState {
    pub fn new() -> Self {
        Self::default()
    }

    /// Load a ROM file
    pub fn load_rom(&mut self, rom_path: &str) -> EmuliteResult<()> {
        log::info!("Starting to load ROM: {}", rom_path);
        
        let rom_data = std::fs::read(rom_path)?;
        log::info!("ROM data read successfully, size: {} bytes", rom_data.len());
        
        // Detect platform from ROM
        let platform_name = self.detect_platform(&rom_data)?;
        log::info!("Detected platform: {}", platform_name);
        
        // Create platform
        let platform = PlatformFactory::create(&platform_name)?;
        log::info!("Platform created successfully");
        
        // Create emulator
        let mut emulator = Emulator::new(self.config.clone())?;
        log::info!("Emulator created successfully");
        
        emulator.load_rom(&rom_path, &platform_name)?;
        log::info!("ROM loaded into emulator successfully");
        
        // Start the emulator
        emulator.start()?;
        log::info!("Emulator started");
        
        self.emulator = Some(Arc::new(Mutex::new(emulator)));
        self.current_rom_path = Some(rom_path.to_string());
        self.is_running = true;
        self.is_paused = false;
        
        log::info!("ROM loading completed successfully");
        Ok(())
    }

    /// Detect platform from ROM data
    fn detect_platform(&self, rom_data: &[u8]) -> EmuliteResult<String> {
        if rom_data.len() < 16 {
            return Err(EmuliteError::InvalidRom("ROM too small".to_string()));
        }

        // Check for NES header
        if rom_data.len() >= 16 && &rom_data[0..4] == b"NES\x1a" {
            return Ok("nes".to_string());
        }

        // Check for SNES header
        if rom_data.len() >= 0x8000 {
            let header = &rom_data[0x7FC0..0x8000];
            if header[0x15] & 0xF0 == 0x20 {
                return Ok("snes".to_string());
            }
        }

        // Check for PlayStation
        if rom_data.len() >= 0x800 && &rom_data[0x800..0x808] == b"PS-X EXE" {
            return Ok("ps1".to_string());
        }

        // Check for Atari 2600 (no header, just size)
        if rom_data.len() <= 32768 {
            return Ok("atari2600".to_string());
        }

        // Default to Atari 2600 for unknown formats (more likely than NES)
        Ok("atari2600".to_string())
    }

    /// Start emulation
    pub fn start(&mut self) -> EmuliteResult<()> {
        if let Some(emulator) = &self.emulator {
            let mut emu = emulator.lock().unwrap();
            emu.start()?;
            self.is_running = true;
            self.is_paused = false;
        }
        Ok(())
    }

    /// Pause emulation
    pub fn pause(&mut self) -> EmuliteResult<()> {
        if let Some(emulator) = &self.emulator {
            let mut emu = emulator.lock().unwrap();
            emu.pause()?;
            self.is_paused = true;
        }
        Ok(())
    }

    /// Resume emulation
    pub fn resume(&mut self) -> EmuliteResult<()> {
        if let Some(emulator) = &self.emulator {
            let mut emu = emulator.lock().unwrap();
            emu.resume()?;
            self.is_paused = false;
        }
        Ok(())
    }

    /// Reset emulation
    pub fn reset(&mut self) -> EmuliteResult<()> {
        if let Some(emulator) = &self.emulator {
            let mut emu = emulator.lock().unwrap();
            emu.reset()?;
            self.frame_count = 0;
        }
        Ok(())
    }

    /// Stop emulation
    pub fn stop(&mut self) -> EmuliteResult<()> {
        if let Some(emulator) = &self.emulator {
            let mut emu = emulator.lock().unwrap();
            // emu.stop()?; // Method not available in this version
            self.is_running = false;
            self.is_paused = false;
            self.frame_count = 0;
        }
        Ok(())
    }

    /// Update FPS counter
    pub fn update_fps(&mut self) {
        self.frame_count += 1;
        let now = std::time::Instant::now();
        let elapsed = now.duration_since(self.last_frame_time).as_secs_f32();
        
        if elapsed >= 1.0 {
            self.fps = self.frame_count as f32 / elapsed;
            self.frame_count = 0;
            self.last_frame_time = now;
        }
    }

    /// Get emulator state for debugging
    pub fn get_emulator_state(&self) -> Option<EmulatorState> {
        if let Some(emulator) = &self.emulator {
            let emu = emulator.lock().unwrap();
            Some(emu.get_state())
        } else {
            None
        }
    }
}

/// GUI theme configuration
#[derive(Debug, Clone)]
pub struct GuiTheme {
    pub name: String,
    pub colors: ThemeColors,
    pub fonts: ThemeFonts,
    pub spacing: ThemeSpacing,
}

#[derive(Debug, Clone)]
pub struct ThemeColors {
    pub background: egui::Color32,
    pub foreground: egui::Color32,
    pub accent: egui::Color32,
    pub error: egui::Color32,
    pub warning: egui::Color32,
    pub success: egui::Color32,
}

#[derive(Debug, Clone)]
pub struct ThemeFonts {
    pub heading: egui::FontId,
    pub body: egui::FontId,
    pub monospace: egui::FontId,
}

#[derive(Debug, Clone)]
pub struct ThemeSpacing {
    pub small: f32,
    pub medium: f32,
    pub large: f32,
}

impl Default for GuiTheme {
    fn default() -> Self {
        Self {
            name: "Default".to_string(),
            colors: ThemeColors {
                background: egui::Color32::from_rgb(30, 30, 30),
                foreground: egui::Color32::from_rgb(200, 200, 200),
                accent: egui::Color32::from_rgb(100, 150, 255),
                error: egui::Color32::from_rgb(255, 100, 100),
                warning: egui::Color32::from_rgb(255, 200, 100),
                success: egui::Color32::from_rgb(100, 255, 100),
            },
            fonts: ThemeFonts {
                heading: egui::FontId::proportional(18.0),
                body: egui::FontId::proportional(14.0),
                monospace: egui::FontId::monospace(12.0),
            },
            spacing: ThemeSpacing {
                small: 4.0,
                medium: 8.0,
                large: 16.0,
            },
        }
    }
}

/// Apply theme to egui context
pub fn apply_theme(ctx: &egui::Context, theme: &GuiTheme) {
    let mut style = (*ctx.style()).clone();
    
    // Apply colors
    style.visuals.window_fill = theme.colors.background;
    style.visuals.panel_fill = theme.colors.background;
    style.visuals.window_stroke = egui::Stroke::new(1.0, theme.colors.foreground);
    style.visuals.override_text_color = Some(theme.colors.foreground);
    // style.visuals.accent_color = theme.colors.accent; // Not available in this egui version
    style.visuals.error_fg_color = theme.colors.error;
    style.visuals.warn_fg_color = theme.colors.warning;
    
    // Apply spacing
    style.spacing.item_spacing = egui::vec2(theme.spacing.medium, theme.spacing.medium);
    style.spacing.window_margin = egui::Margin::same(theme.spacing.medium);
    style.spacing.button_padding = egui::vec2(theme.spacing.medium, theme.spacing.small);
    
    ctx.set_style(style);
}
