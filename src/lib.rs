//! Emulite - Multi-platform video game emulator
//! 
//! A comprehensive emulator supporting multiple gaming platforms from Atari to PS3.

pub mod core;
pub mod platforms;
pub mod audio;
pub mod video;
pub mod input;
pub mod memory;
pub mod cpu;
pub mod debug;
pub mod config;
pub mod utils;
pub mod gui;

pub use core::Emulator;
pub use platforms::Platform;
pub use config::Config;

/// Re-export commonly used types
pub use anyhow::Result;
pub use thiserror::Error;

/// Main emulator error type
#[derive(Error, Debug)]
pub enum EmuliteError {
    #[error("Platform not supported: {0}")]
    UnsupportedPlatform(String),
    
    #[error("Invalid ROM file: {0}")]
    InvalidRom(String),
    
    #[error("Memory access violation at address: 0x{0:08X}")]
    MemoryAccessViolation(u32),
    
    #[error("CPU error: {0}")]
    CpuError(String),
    
    #[error("Audio error: {0}")]
    AudioError(String),
    
    #[error("Video error: {0}")]
    VideoError(String),
    
    #[error("Input error: {0}")]
    InputError(String),
    
    #[error("Configuration error: {0}")]
    ConfigError(String),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Audio error: {0}")]
    AudioBuildError(#[from] cpal::BuildStreamError),
    
    #[error("Image error: {0}")]
    ImageError(#[from] image::ImageError),
    
    #[error("WGPU surface error: {0}")]
    WgpuSurfaceError(#[from] wgpu::CreateSurfaceError),
}

/// Result type for emulator operations
pub type EmuliteResult<T> = Result<T, EmuliteError>;
