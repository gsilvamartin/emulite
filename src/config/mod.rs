//! Configuration system for emulator settings

use crate::{EmuliteResult, EmuliteError};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::collections::HashMap;
use std::fs;

/// Main configuration structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub audio: AudioConfig,
    pub video: VideoConfig,
    pub input: InputConfig,
    pub debug: DebugConfig,
    pub emulation: EmulationConfig,
    pub ui: UiConfig,
}

/// Audio configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioConfig {
    pub enabled: bool,
    pub sample_rate: u32,
    pub buffer_size: usize,
    pub volume: f32,
    pub channels: u8,
    pub device: Option<String>,
}

/// Video configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoConfig {
    pub width: u32,
    pub height: u32,
    pub fullscreen: bool,
    pub vsync: bool,
    pub scale: f32,
    pub filter: String,
    pub shader: Option<String>,
    pub aspect_ratio: String,
}

/// Input configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputConfig {
    pub keyboard_mapping: HashMap<String, String>,
    pub gamepad_mapping: HashMap<String, String>,
    pub deadzone: f32,
    pub sensitivity: f32,
    pub auto_fire: bool,
    pub turbo_speed: u32,
}

/// Debug configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebugConfig {
    pub enabled: bool,
    pub log_level: String,
    pub log_file: Option<String>,
    pub breakpoints: Vec<u32>,
    pub trace_execution: bool,
    pub trace_memory: bool,
    pub trace_instructions: bool,
    pub max_trace_entries: usize,
}

/// Emulation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmulationConfig {
    pub speed: f32,
    pub frame_skip: u32,
    pub auto_save: bool,
    pub save_interval: u32,
    pub rewind_enabled: bool,
    pub rewind_frames: u32,
    pub cheats: Vec<Cheat>,
    pub region: String,
    pub bios_path: Option<String>,
}

/// UI configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UiConfig {
    pub theme: String,
    pub show_fps: bool,
    pub show_debug: bool,
    pub show_controls: bool,
    pub language: String,
    pub font_size: u32,
    pub window_width: u32,
    pub window_height: u32,
    pub window_x: i32,
    pub window_y: i32,
}

/// Cheat code structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cheat {
    pub name: String,
    pub description: String,
    pub address: u32,
    pub value: u8,
    pub enabled: bool,
    pub platform: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            audio: AudioConfig::default(),
            video: VideoConfig::default(),
            input: InputConfig::default(),
            debug: DebugConfig::default(),
            emulation: EmulationConfig::default(),
            ui: UiConfig::default(),
        }
    }
}

impl Default for AudioConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            sample_rate: 44100,
            buffer_size: 1024,
            volume: 0.8,
            channels: 2,
            device: None,
        }
    }
}

impl Default for VideoConfig {
    fn default() -> Self {
        Self {
            width: 256,
            height: 240,
            fullscreen: false,
            vsync: true,
            scale: 3.0,
            filter: "nearest".to_string(),
            shader: None,
            aspect_ratio: "4:3".to_string(),
        }
    }
}

impl Default for InputConfig {
    fn default() -> Self {
        let mut keyboard_mapping = HashMap::new();
        keyboard_mapping.insert("up".to_string(), "Up".to_string());
        keyboard_mapping.insert("down".to_string(), "Down".to_string());
        keyboard_mapping.insert("left".to_string(), "Left".to_string());
        keyboard_mapping.insert("right".to_string(), "Right".to_string());
        keyboard_mapping.insert("a".to_string(), "Z".to_string());
        keyboard_mapping.insert("b".to_string(), "X".to_string());
        keyboard_mapping.insert("start".to_string(), "Return".to_string());
        keyboard_mapping.insert("select".to_string(), "Space".to_string());
        
        let mut gamepad_mapping = HashMap::new();
        gamepad_mapping.insert("up".to_string(), "DPadUp".to_string());
        gamepad_mapping.insert("down".to_string(), "DPadDown".to_string());
        gamepad_mapping.insert("left".to_string(), "DPadLeft".to_string());
        gamepad_mapping.insert("right".to_string(), "DPadRight".to_string());
        gamepad_mapping.insert("a".to_string(), "South".to_string());
        gamepad_mapping.insert("b".to_string(), "East".to_string());
        gamepad_mapping.insert("start".to_string(), "Start".to_string());
        gamepad_mapping.insert("select".to_string(), "Select".to_string());
        
        Self {
            keyboard_mapping,
            gamepad_mapping,
            deadzone: 0.1,
            sensitivity: 1.0,
            auto_fire: false,
            turbo_speed: 10,
        }
    }
}

impl Default for DebugConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            log_level: "info".to_string(),
            log_file: None,
            breakpoints: Vec::new(),
            trace_execution: false,
            trace_memory: false,
            trace_instructions: false,
            max_trace_entries: 10000,
        }
    }
}

impl Default for EmulationConfig {
    fn default() -> Self {
        Self {
            speed: 1.0,
            frame_skip: 0,
            auto_save: false,
            save_interval: 300, // 5 minutes
            rewind_enabled: false,
            rewind_frames: 300, // 5 seconds at 60fps
            cheats: Vec::new(),
            region: "NTSC".to_string(),
            bios_path: None,
        }
    }
}

impl Default for UiConfig {
    fn default() -> Self {
        Self {
            theme: "dark".to_string(),
            show_fps: true,
            show_debug: false,
            show_controls: false,
            language: "en".to_string(),
            font_size: 12,
            window_width: 800,
            window_height: 600,
            window_x: 100,
            window_y: 100,
        }
    }
}

impl Config {
    /// Load configuration from file
    pub fn load() -> EmuliteResult<Self> {
        let config_path = Self::get_config_path()?;
        
        if config_path.exists() {
            let content = fs::read_to_string(&config_path)?;
            let config: Config = toml::from_str(&content)
                .map_err(|e| EmuliteError::ConfigError(format!("Failed to parse config: {}", e)))?;
            Ok(config)
        } else {
            // Create default config
            let config = Config::default();
            config.save()?;
            Ok(config)
        }
    }
    
    /// Save configuration to file
    pub fn save(&self) -> EmuliteResult<()> {
        let config_path = Self::get_config_path()?;
        
        // Create config directory if it doesn't exist
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        let content = toml::to_string_pretty(self)
            .map_err(|e| EmuliteError::ConfigError(format!("Failed to serialize config: {}", e)))?;
        
        fs::write(&config_path, content)?;
        Ok(())
    }
    
    /// Get configuration file path
    fn get_config_path() -> EmuliteResult<std::path::PathBuf> {
        let mut config_dir = dirs::config_dir()
            .ok_or_else(|| EmuliteError::ConfigError("Could not find config directory".to_string()))?;
        
        config_dir.push("emulite");
        config_dir.push("config.toml");
        
        Ok(config_dir)
    }
    
    /// Get ROM directory path
    pub fn get_rom_dir() -> EmuliteResult<std::path::PathBuf> {
        let mut rom_dir = dirs::home_dir()
            .ok_or_else(|| EmuliteError::ConfigError("Could not find home directory".to_string()))?;
        
        rom_dir.push("emulite");
        rom_dir.push("roms");
        
        Ok(rom_dir)
    }
    
    /// Get save directory path
    pub fn get_save_dir() -> EmuliteResult<std::path::PathBuf> {
        let mut save_dir = dirs::data_dir()
            .ok_or_else(|| EmuliteError::ConfigError("Could not find data directory".to_string()))?;
        
        save_dir.push("emulite");
        save_dir.push("saves");
        
        Ok(save_dir)
    }
    
    /// Get screenshot directory path
    pub fn get_screenshot_dir() -> EmuliteResult<std::path::PathBuf> {
        let mut screenshot_dir = dirs::picture_dir()
            .ok_or_else(|| EmuliteError::ConfigError("Could not find pictures directory".to_string()))?;
        
        screenshot_dir.push("emulite");
        screenshot_dir.push("screenshots");
        
        Ok(screenshot_dir)
    }
    
    /// Validate configuration
    pub fn validate(&self) -> EmuliteResult<()> {
        // Validate audio config
        if self.audio.sample_rate < 8000 || self.audio.sample_rate > 192000 {
            return Err(EmuliteError::ConfigError("Invalid sample rate".to_string()));
        }
        
        if self.audio.buffer_size < 64 || self.audio.buffer_size > 8192 {
            return Err(EmuliteError::ConfigError("Invalid buffer size".to_string()));
        }
        
        if self.audio.volume < 0.0 || self.audio.volume > 1.0 {
            return Err(EmuliteError::ConfigError("Invalid volume".to_string()));
        }
        
        // Validate video config
        if self.video.width < 64 || self.video.width > 4096 {
            return Err(EmuliteError::ConfigError("Invalid video width".to_string()));
        }
        
        if self.video.height < 64 || self.video.height > 4096 {
            return Err(EmuliteError::ConfigError("Invalid video height".to_string()));
        }
        
        if self.video.scale < 0.5 || self.video.scale > 10.0 {
            return Err(EmuliteError::ConfigError("Invalid scale".to_string()));
        }
        
        // Validate input config
        if self.input.deadzone < 0.0 || self.input.deadzone > 1.0 {
            return Err(EmuliteError::ConfigError("Invalid deadzone".to_string()));
        }
        
        if self.input.sensitivity < 0.1 || self.input.sensitivity > 5.0 {
            return Err(EmuliteError::ConfigError("Invalid sensitivity".to_string()));
        }
        
        // Validate emulation config
        if self.emulation.speed < 0.1 || self.emulation.speed > 10.0 {
            return Err(EmuliteError::ConfigError("Invalid emulation speed".to_string()));
        }
        
        if self.emulation.frame_skip > 10 {
            return Err(EmuliteError::ConfigError("Invalid frame skip".to_string()));
        }
        
        Ok(())
    }
    
    /// Reset to default values
    pub fn reset_to_defaults(&mut self) {
        *self = Config::default();
    }
    
    /// Update audio configuration
    pub fn update_audio(&mut self, audio: AudioConfig) -> EmuliteResult<()> {
        self.audio = audio;
        self.validate()?;
        Ok(())
    }
    
    /// Update video configuration
    pub fn update_video(&mut self, video: VideoConfig) -> EmuliteResult<()> {
        self.video = video;
        self.validate()?;
        Ok(())
    }
    
    /// Update input configuration
    pub fn update_input(&mut self, input: InputConfig) -> EmuliteResult<()> {
        self.input = input;
        self.validate()?;
        Ok(())
    }
    
    /// Update debug configuration
    pub fn update_debug(&mut self, debug: DebugConfig) -> EmuliteResult<()> {
        self.debug = debug;
        self.validate()?;
        Ok(())
    }
    
    /// Update emulation configuration
    pub fn update_emulation(&mut self, emulation: EmulationConfig) -> EmuliteResult<()> {
        self.emulation = emulation;
        self.validate()?;
        Ok(())
    }
    
    /// Update UI configuration
    pub fn update_ui(&mut self, ui: UiConfig) -> EmuliteResult<()> {
        self.ui = ui;
        self.validate()?;
        Ok(())
    }
    
    /// Add cheat code
    pub fn add_cheat(&mut self, cheat: Cheat) {
        self.emulation.cheats.push(cheat);
    }
    
    /// Remove cheat code
    pub fn remove_cheat(&mut self, name: &str) {
        self.emulation.cheats.retain(|c| c.name != name);
    }
    
    /// Get cheat codes for platform
    pub fn get_cheats_for_platform(&self, platform: &str) -> Vec<&Cheat> {
        self.emulation.cheats.iter()
            .filter(|c| c.platform == platform)
            .collect()
    }
    
    /// Enable/disable cheat
    pub fn toggle_cheat(&mut self, name: &str) -> EmuliteResult<()> {
        if let Some(cheat) = self.emulation.cheats.iter_mut().find(|c| c.name == name) {
            cheat.enabled = !cheat.enabled;
            Ok(())
        } else {
            Err(EmuliteError::ConfigError(format!("Cheat '{}' not found", name)))
        }
    }
}

/// Configuration presets for different platforms
pub struct ConfigPresets;

impl ConfigPresets {
    /// Get NES configuration preset
    pub fn nes() -> Config {
        let mut config = Config::default();
        config.video.width = 256;
        config.video.height = 240;
        config.video.aspect_ratio = "4:3".to_string();
        config.emulation.region = "NTSC".to_string();
        config
    }
    
    /// Get SNES configuration preset
    pub fn snes() -> Config {
        let mut config = Config::default();
        config.video.width = 256;
        config.video.height = 224;
        config.video.aspect_ratio = "4:3".to_string();
        config.emulation.region = "NTSC".to_string();
        config
    }
    
    /// Get PlayStation configuration preset
    pub fn playstation() -> Config {
        let mut config = Config::default();
        config.video.width = 320;
        config.video.height = 240;
        config.video.aspect_ratio = "4:3".to_string();
        config.emulation.region = "NTSC".to_string();
        config
    }
    
    /// Get PlayStation 2 configuration preset
    pub fn playstation2() -> Config {
        let mut config = Config::default();
        config.video.width = 640;
        config.video.height = 480;
        config.video.aspect_ratio = "4:3".to_string();
        config.emulation.region = "NTSC".to_string();
        config
    }
    
    /// Get PlayStation 3 configuration preset
    pub fn playstation3() -> Config {
        let mut config = Config::default();
        config.video.width = 1280;
        config.video.height = 720;
        config.video.aspect_ratio = "16:9".to_string();
        config.emulation.region = "NTSC".to_string();
        config
    }
}

/// Configuration manager for handling multiple configurations
pub struct ConfigManager {
    current: Config,
    presets: HashMap<String, Config>,
}

impl ConfigManager {
    pub fn new() -> EmuliteResult<Self> {
        let current = Config::load()?;
        let mut presets = HashMap::new();
        
        presets.insert("nes".to_string(), ConfigPresets::nes());
        presets.insert("snes".to_string(), ConfigPresets::snes());
        presets.insert("playstation".to_string(), ConfigPresets::playstation());
        presets.insert("playstation2".to_string(), ConfigPresets::playstation2());
        presets.insert("playstation3".to_string(), ConfigPresets::playstation3());
        
        Ok(Self { current, presets })
    }
    
    pub fn get_current(&self) -> &Config {
        &self.current
    }
    
    pub fn get_current_mut(&mut self) -> &mut Config {
        &mut self.current
    }
    
    pub fn load_preset(&mut self, name: &str) -> EmuliteResult<()> {
        if let Some(preset) = self.presets.get(name) {
            self.current = preset.clone();
            Ok(())
        } else {
            Err(EmuliteError::ConfigError(format!("Preset '{}' not found", name)))
        }
    }
    
    pub fn save_current(&self) -> EmuliteResult<()> {
        self.current.save()
    }
    
    pub fn get_preset_names(&self) -> Vec<&String> {
        self.presets.keys().collect()
    }
}
