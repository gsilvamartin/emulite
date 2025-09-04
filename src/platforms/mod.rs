//! Platform-specific emulator implementations

use crate::{EmuliteResult, EmuliteError};
use std::path::Path;

pub mod atari2600;
pub mod nes;
pub mod snes;
pub mod ps1;
pub mod ps2;
pub mod ps3;

/// Trait that all platform emulators must implement
pub trait Platform: Send + Sync {
    /// Load a ROM file into the platform
    fn load_rom(&mut self, rom_path: &str) -> EmuliteResult<()>;
    
    /// Execute one emulation step
    fn step(&mut self) -> EmuliteResult<()>;
    
    /// Reset the platform to initial state
    fn reset(&mut self) -> EmuliteResult<()>;
    
    /// Get platform name
    fn name(&self) -> &str;
    
    /// Get platform version
    fn version(&self) -> &str;
    
    /// Get platform information
    fn info(&self) -> PlatformInfo;
    
    /// Save state
    fn save_state(&self, path: &str) -> EmuliteResult<()>;
    
    /// Load state
    fn load_state(&mut self, path: &str) -> EmuliteResult<()>;
    
    /// Get CPU for debugging (optional)
    fn get_cpu(&self) -> Option<&dyn crate::cpu::Cpu>;
    
    /// Get current frame data for rendering
    fn get_frame_data(&self) -> EmuliteResult<Vec<u8>>;
    
    /// Update input state (optional - platforms can implement this)
    fn update_input(&mut self, _input_system: &crate::input::InputSystem) -> EmuliteResult<()> {
        // Default implementation does nothing
        Ok(())
    }
    
    /// Get audio samples for the current frame (optional - platforms can implement this)
    fn get_audio_samples(&self) -> EmuliteResult<Vec<f32>> {
        // Default implementation returns empty audio
        Ok(Vec::new())
    }
}

/// Platform information structure
#[derive(Debug, Clone)]
pub struct PlatformInfo {
    pub name: String,
    pub version: String,
    pub cpu_type: String,
    pub memory_size: usize,
    pub video_resolution: (u32, u32),
    pub audio_channels: u8,
    pub supported_formats: Vec<String>,
}

/// Placeholder platform for initialization
pub struct PlaceholderPlatform;

impl Platform for PlaceholderPlatform {
    fn load_rom(&mut self, _rom_path: &str) -> EmuliteResult<()> {
        Err(EmuliteError::UnsupportedPlatform("No platform loaded".to_string()))
    }
    
    fn step(&mut self) -> EmuliteResult<()> {
        Ok(())
    }
    
    fn reset(&mut self) -> EmuliteResult<()> {
        Ok(())
    }
    
    fn name(&self) -> &str {
        "placeholder"
    }
    
    fn version(&self) -> &str {
        "0.0.0"
    }
    
    fn info(&self) -> PlatformInfo {
        PlatformInfo {
            name: "placeholder".to_string(),
            version: "0.0.0".to_string(),
            cpu_type: "none".to_string(),
            memory_size: 0,
            video_resolution: (0, 0),
            audio_channels: 0,
            supported_formats: vec![],
        }
    }
    
    fn save_state(&self, _path: &str) -> EmuliteResult<()> {
        Err(EmuliteError::UnsupportedPlatform("No platform loaded".to_string()))
    }
    
    fn load_state(&mut self, _path: &str) -> EmuliteResult<()> {
        Err(EmuliteError::UnsupportedPlatform("No platform loaded".to_string()))
    }
    
    fn get_cpu(&self) -> Option<&dyn crate::cpu::Cpu> {
        None
    }
    
    fn get_frame_data(&self) -> EmuliteResult<Vec<u8>> {
        // Placeholder resolution
        let width = 256;
        let height = 240;
        let mut frame_data = vec![0u8; width * height * 4]; // RGBA format
        
        // Fill with a simple pattern for testing
        for (i, pixel) in frame_data.chunks_exact_mut(4).enumerate() {
            let x = i % width;
            let y = i / width;
            pixel[0] = ((x * 255) / width) as u8; // Red
            pixel[1] = ((y * 255) / height) as u8; // Green
            pixel[2] = 128; // Blue
            pixel[3] = 255; // Alpha
        }
        
        Ok(frame_data)
    }
}

/// Factory for creating platform instances
pub struct PlatformFactory;

impl PlatformFactory {
    /// Create a platform instance by name
    pub fn create(platform_name: &str) -> EmuliteResult<Box<dyn Platform>> {
        match platform_name.to_lowercase().as_str() {
            "atari2600" | "atari" => Ok(Box::new(atari2600::Atari2600::new()?)),
            "nes" | "nintendo" => Ok(Box::new(nes::Nes::new()?)),
            "snes" | "super nintendo" => Ok(Box::new(snes::Snes::new()?)),
            "ps1" | "playstation" => Ok(Box::new(ps1::Ps1::new()?)),
            "ps2" | "playstation2" => Ok(Box::new(ps2::Ps2::new()?)),
            "ps3" | "playstation3" => Ok(Box::new(ps3::Ps3::new()?)),
            _ => Err(EmuliteError::UnsupportedPlatform(
                format!("Platform '{}' is not supported", platform_name)
            )),
        }
    }
    
    /// Create a placeholder platform
    pub fn create_placeholder() -> Box<dyn Platform> {
        Box::new(PlaceholderPlatform)
    }
    
    /// Get list of supported platforms
    pub fn supported_platforms() -> Vec<&'static str> {
        vec![
            "atari2600",
            "nes", 
            "snes",
            "ps1",
            "ps2",
            "ps3",
        ]
    }
}
