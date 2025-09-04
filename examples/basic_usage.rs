//! Basic usage example for Emulite

use emulite::{Emulator, Config, EmuliteResult};
use std::env;

fn main() -> EmuliteResult<()> {
    // Initialize logging
    env_logger::init();
    
    println!("Emulite - Multi-platform emulator example");
    
    // Parse command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <rom_file> [platform]", args[0]);
        eprintln!("Supported platforms: atari2600, nes, snes, ps1, ps2, ps3");
        return Ok(());
    }
    
    let rom_path = &args[1];
    let platform = args.get(2).map(|s| s.as_str()).unwrap_or("auto");
    
    // Load configuration
    let config = Config::load()?;
    println!("Configuration loaded successfully");
    
    // Create emulator
    let mut emulator = Emulator::new(config)?;
    println!("Emulator created successfully");
    
    // Load ROM
    emulator.load_rom(rom_path, platform)?;
    println!("ROM loaded: {}", rom_path);
    
    // Get platform information
    let platform_info = emulator.platform_info().info();
    println!("Platform: {}", platform_info.name);
    println!("Version: {}", platform_info.version);
    println!("CPU: {}", platform_info.cpu_type);
    println!("Memory: {} bytes", platform_info.memory_size);
    println!("Resolution: {}x{}", platform_info.video_resolution.0, platform_info.video_resolution.1);
    println!("Audio channels: {}", platform_info.audio_channels);
    
    // Run emulator for a few steps
    println!("Running emulator for 1000 steps...");
    for i in 0..1000 {
        emulator.step()?;
        if i % 100 == 0 {
            println!("Step {}", i);
        }
    }
    
    println!("Emulation completed successfully");
    Ok(())
}
