//! Core emulator functionality and main emulator struct

use crate::{
    platforms::{Platform, PlatformFactory},
    config::Config,
    audio::AudioSystem,
    video::VideoSystem,
    input::InputSystem,
    debug::Debugger,
    EmuliteResult, EmuliteError,
};
use std::path::Path;
use serde::{Deserialize, Serialize};

/// Emulator state for debugging and GUI
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmulatorState {
    pub running: bool,
    pub platform_name: String,
    pub cpu_registers: Vec<u32>,
    pub memory_dump: Vec<u8>,
    pub current_instruction: String,
}

/// Breakpoint for debugging
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Breakpoint {
    pub address: u32,
    pub enabled: bool,
    pub description: String,
}

/// Main emulator struct that coordinates all subsystems
pub struct Emulator {
    platform: Box<dyn Platform>,
    audio: AudioSystem,
    video: VideoSystem,
    input: InputSystem,
    debugger: Debugger,
    config: Config,
    running: bool,
}

impl Emulator {
    /// Create a new emulator instance
    pub fn new(config: Config) -> EmuliteResult<Self> {
        let audio = AudioSystem::new(&config.audio)?;
        let video = VideoSystem::new(&config.video)?;
        let input = InputSystem::new(&config.input)?;
        let debugger = Debugger::new(&config.debug)?;
        
        Ok(Self {
            platform: PlatformFactory::create_placeholder(),
            audio,
            video,
            input,
            debugger,
            config,
            running: false,
        })
    }
    
    /// Load a ROM file and detect/select platform
    pub fn load_rom(&mut self, rom_path: &str, platform_hint: &str) -> EmuliteResult<()> {
        let path = Path::new(rom_path);
        
        if !path.exists() {
            return Err(EmuliteError::InvalidRom(format!("File not found: {}", rom_path)));
        }
        
        // Detect platform if auto-detection is requested
        let platform_name = if platform_hint == "auto" {
            self.detect_platform(rom_path)?
        } else {
            platform_hint.to_string()
        };
        
        // Create platform-specific emulator
        self.platform = PlatformFactory::create(&platform_name)?;
        
        // Load ROM into platform
        self.platform.load_rom(rom_path)?;
        
        log::info!("Loaded ROM: {} for platform: {}", rom_path, platform_name);
        Ok(())
    }
    
    /// Detect platform from ROM file
    fn detect_platform(&self, rom_path: &str) -> EmuliteResult<String> {
        let path = Path::new(rom_path);
        let extension = path.extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("")
            .to_lowercase();
        
        // Read file header for better detection
        let rom_data = std::fs::read(rom_path)?;
        
        // Check for iNES header (NES)
        if rom_data.len() >= 16 && &rom_data[0..4] == b"NES\x1a" {
            return Ok("nes".to_string());
        }
        
        // Check for SNES header
        if rom_data.len() >= 0x7FC0 {
            let header_offset = if rom_data.len() >= 0x8000 { 0x7FC0 } else { 0x7FB0 };
            if header_offset < rom_data.len() {
                let title = &rom_data[header_offset..header_offset + 21];
                if title.iter().any(|&b| b != 0) {
                    return Ok("snes".to_string());
                }
            }
        }
        
        // Extension-based detection for other formats
        match extension.as_str() {
            "a26" => Ok("atari2600".to_string()),
            "nes" => Ok("nes".to_string()),
            "smc" | "sfc" => Ok("snes".to_string()),
            "iso" | "img" => Ok("ps1".to_string()),
            "bin" => {
                // For .bin files, check file size to help determine platform
                match rom_data.len() {
                    0..=4096 => Ok("atari2600".to_string()), // Small files are likely Atari 2600
                    4097..=1048576 => Ok("atari2600".to_string()), // Medium files could be Atari 2600
                    _ => Ok("ps1".to_string()), // Large files are likely PS1
                }
            },
            _ => Err(EmuliteError::UnsupportedPlatform(
                format!("Cannot detect platform for file: {}", rom_path)
            )),
        }
    }
    
    /// Run the emulator main loop
    pub fn run(&mut self) -> EmuliteResult<()> {
        self.running = true;
        log::info!("Starting emulator main loop");
        
        while self.running {
            // Handle input
            self.input.update()?;
            
            // Update platform (CPU, memory, etc.)
            self.platform.step()?;
            
            // Update audio
            self.audio.update()?;
            
            // Update video
            self.video.update()?;
            
            // Debug step if enabled
            if self.config.debug.enabled {
                self.debugger.step(&*self.platform)?;
            }
            
            // Check for exit conditions
            if self.input.should_exit() {
                self.running = false;
            }
        }
        
        log::info!("Emulator stopped");
        Ok(())
    }
    
    /// Stop the emulator
    pub fn stop(&mut self) {
        self.running = false;
    }
    
    /// Get current platform information
    pub fn platform_info(&self) -> &dyn Platform {
        &*self.platform
    }
    
    /// Get mutable reference to platform for debugging
    pub fn platform_mut(&mut self) -> &mut dyn Platform {
        &mut *self.platform
    }
    
    /// Get current emulator state
    pub fn get_state(&self) -> EmulatorState {
        EmulatorState {
            running: self.running,
            platform_name: self.platform.info().name.clone(),
            cpu_registers: vec![], // TODO: Get from platform
            memory_dump: vec![], // TODO: Get from platform
            current_instruction: "NOP".to_string(), // TODO: Get from platform
        }
    }
    
    /// Start the emulator
    pub fn start(&mut self) -> EmuliteResult<()> {
        self.running = true;
        log::info!("Emulator started");
        Ok(())
    }
    
    /// Pause the emulator
    pub fn pause(&mut self) -> EmuliteResult<()> {
        self.running = false;
        log::info!("Emulator paused");
        Ok(())
    }
    
    /// Resume the emulator
    pub fn resume(&mut self) -> EmuliteResult<()> {
        self.running = true;
        log::info!("Emulator resumed");
        Ok(())
    }
    
    /// Reset the emulator
    pub fn reset(&mut self) -> EmuliteResult<()> {
        self.platform.reset()?;
        log::info!("Emulator reset");
        Ok(())
    }
    
    /// Execute one emulation step
    pub fn step(&mut self) -> EmuliteResult<()> {
        if !self.running {
            return Ok(());
        }
        
        // Handle input
        self.input.update()?;
        
        // Update platform-specific input
        self.platform.update_input(&self.input)?;
        
        // Update platform (CPU, memory, etc.)
        self.platform.step()?;
        
        // Update audio
        self.audio.update()?;
        
        // Get audio samples from platform
        let audio_samples = self.platform.get_audio_samples()?;
        if !audio_samples.is_empty() {
            // TODO: Send audio samples to audio system
            // For now, we'll just ignore them
        }
        
        // Update video
        self.video.update()?;
        
        // Debug step if enabled
        if self.config.debug.enabled {
            self.debugger.step(&*self.platform)?;
        }
        
        Ok(())
    }
    
    /// Get current frame data for rendering
    pub fn get_frame_data(&self) -> EmuliteResult<Vec<u8>> {
        // Get actual frame data from platform
        self.platform.get_frame_data()
    }
    
    /// Check if emulator is running
    pub fn is_running(&self) -> bool {
        self.running
    }
    
    /// Check if emulator is paused
    pub fn is_paused(&self) -> bool {
        !self.running
    }
}
