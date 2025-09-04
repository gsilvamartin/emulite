//! Integration tests for Emulite

use emulite::{Emulator, Config, EmuliteResult};
use std::path::Path;

#[test]
fn test_emulator_creation() {
    let config = Config::default();
    let emulator = Emulator::new(config);
    assert!(emulator.is_ok());
}

#[test]
fn test_config_loading() {
    let config = Config::load();
    assert!(config.is_ok());
}

#[test]
fn test_config_validation() {
    let mut config = Config::default();
    let result = config.validate();
    assert!(result.is_ok());
}

#[test]
fn test_platform_detection() {
    // Test NES ROM detection
    let nes_extensions = ["nes"];
    for ext in &nes_extensions {
        let path = format!("test.{}", ext);
        // This would test platform detection logic
        assert!(Path::new(&path).extension().is_some());
    }
}

#[test]
fn test_memory_mapping() {
    use emulite::memory::{MemoryMapper, RamDevice};
    use std::sync::{Arc, RwLock};
    
    let mut mapper = MemoryMapper::new(0x10000);
    let ram = Arc::new(RwLock::new(RamDevice::new(1024, 0x0000, "RAM".to_string())));
    
    let result = mapper.add_device("RAM".to_string(), ram);
    assert!(result.is_ok());
    
    // Test memory access
    let read_result = mapper.read(0x0000);
    assert!(read_result.is_ok());
    
    let write_result = mapper.write(0x0000, 0x42);
    assert!(write_result.is_ok());
    
    let read_after_write = mapper.read(0x0000);
    assert_eq!(read_after_write.unwrap(), 0x42);
}

#[test]
fn test_cpu_creation() {
    use emulite::cpu::{CpuFactory, Cpu};
    
    let cpu_types = ["6502", "68000", "mips", "x86", "arm"];
    
    for cpu_type in &cpu_types {
        let cpu = CpuFactory::create(cpu_type);
        assert!(cpu.is_ok(), "Failed to create CPU type: {}", cpu_type);
        
        let cpu = cpu.unwrap();
        assert_eq!(cpu.name(), cpu_type);
    }
}

#[test]
fn test_cpu_execution() {
    use emulite::cpu::{CpuFactory, Cpu};
    
    let mut cpu = CpuFactory::create("6502").unwrap();
    
    // Test basic execution
    let result = cpu.step();
    assert!(result.is_ok());
    
    // Test reset
    let result = cpu.reset();
    assert!(result.is_ok());
}

#[test]
fn test_audio_system() {
    use emulite::audio::{AudioSystem, AudioConfig};
    
    let config = AudioConfig::default();
    let audio = AudioSystem::new(&config);
    assert!(audio.is_ok());
}

#[test]
fn test_video_system() {
    use emulite::video::{VideoSystem, VideoConfig};
    
    let config = VideoConfig::default();
    let video = VideoSystem::new(&config);
    assert!(video.is_ok());
}

#[test]
fn test_input_system() {
    use emulite::input::{InputSystem, InputConfig};
    
    let config = InputConfig::default();
    let input = InputSystem::new(&config);
    assert!(input.is_ok());
}

#[test]
fn test_debug_system() {
    use emulite::debug::{Debugger, DebugConfig};
    
    let config = DebugConfig::default();
    let debugger = Debugger::new(&config);
    assert!(debugger.is_ok());
}

#[test]
fn test_platform_factory() {
    use emulite::platforms::PlatformFactory;
    
    let platforms = ["atari2600", "nes", "snes", "ps1", "ps2", "ps3"];
    
    for platform in &platforms {
        let platform_instance = PlatformFactory::create(platform);
        assert!(platform_instance.is_ok(), "Failed to create platform: {}", platform);
    }
}

#[test]
fn test_utils() {
    use emulite::utils::{FileUtils, MathUtils, StringUtils, TimeUtils};
    
    // Test math utilities
    assert_eq!(MathUtils::clamp(5, 0, 10), 5);
    assert_eq!(MathUtils::clamp(-1, 0, 10), 0);
    assert_eq!(MathUtils::clamp(15, 0, 10), 10);
    
    // Test string utilities
    assert_eq!(StringUtils::to_lowercase("HELLO"), "hello");
    assert_eq!(StringUtils::to_uppercase("hello"), "HELLO");
    assert_eq!(StringUtils::trim("  hello  "), "hello");
    
    // Test time utilities
    let timestamp = TimeUtils::current_timestamp();
    assert!(timestamp > 0);
}

#[test]
fn test_error_handling() {
    use emulite::{EmuliteError, EmuliteResult};
    
    // Test error creation
    let error = EmuliteError::UnsupportedPlatform("test".to_string());
    assert!(matches!(error, EmuliteError::UnsupportedPlatform(_)));
    
    // Test error conversion
    let io_error = std::io::Error::new(std::io::ErrorKind::NotFound, "test");
    let emulite_error: EmuliteError = io_error.into();
    assert!(matches!(emulite_error, EmuliteError::IoError(_)));
}

#[test]
fn test_config_presets() {
    use emulite::config::ConfigPresets;
    
    let nes_config = ConfigPresets::nes();
    assert_eq!(nes_config.video.width, 256);
    assert_eq!(nes_config.video.height, 240);
    
    let snes_config = ConfigPresets::snes();
    assert_eq!(snes_config.video.width, 256);
    assert_eq!(snes_config.video.height, 224);
    
    let ps1_config = ConfigPresets::playstation();
    assert_eq!(ps1_config.video.width, 320);
    assert_eq!(ps1_config.video.height, 240);
    
    let ps2_config = ConfigPresets::playstation2();
    assert_eq!(ps2_config.video.width, 640);
    assert_eq!(ps2_config.video.height, 480);
    
    let ps3_config = ConfigPresets::playstation3();
    assert_eq!(ps3_config.video.width, 1280);
    assert_eq!(ps3_config.video.height, 720);
}

#[test]
fn test_input_presets() {
    use emulite::input::InputPresets;
    
    let nes_mapping = InputPresets::nes_mapping();
    assert!(nes_mapping.keyboard_mapping.contains_key("a"));
    assert!(nes_mapping.keyboard_mapping.contains_key("b"));
    
    let snes_mapping = InputPresets::snes_mapping();
    assert!(snes_mapping.keyboard_mapping.contains_key("x"));
    assert!(snes_mapping.keyboard_mapping.contains_key("y"));
    
    let ps_mapping = InputPresets::playstation_mapping();
    assert!(ps_mapping.keyboard_mapping.contains_key("l1"));
    assert!(ps_mapping.keyboard_mapping.contains_key("r1"));
}

#[test]
fn test_memory_bank_controller() {
    use emulite::memory::MemoryBankController;
    
    let mut mbc = MemoryBankController::new(2048, 4);
    
    // Test bank switching
    let result = mbc.switch_bank(1);
    assert!(result.is_ok());
    
    // Test memory access
    let result = mbc.write(0x0000, 0x42);
    assert!(result.is_ok());
    
    let result = mbc.read(0x0000);
    assert_eq!(result.unwrap(), 0x42);
}

#[test]
fn test_cheat_system() {
    use emulite::config::{Config, Cheat};
    
    let mut config = Config::default();
    
    // Add cheat
    let cheat = Cheat {
        name: "test_cheat".to_string(),
        description: "Test cheat".to_string(),
        address: 0x8000,
        value: 0xFF,
        enabled: true,
        platform: "nes".to_string(),
    };
    
    config.add_cheat(cheat);
    
    // Test cheat retrieval
    let cheats = config.get_cheats_for_platform("nes");
    assert_eq!(cheats.len(), 1);
    assert_eq!(cheats[0].name, "test_cheat");
    
    // Test cheat toggle
    let result = config.toggle_cheat("test_cheat");
    assert!(result.is_ok());
    
    // Test cheat removal
    config.remove_cheat("test_cheat");
    let cheats = config.get_cheats_for_platform("nes");
    assert_eq!(cheats.len(), 0);
}

#[test]
fn test_performance_utils() {
    use emulite::utils::PerformanceUtils;
    
    // Test execution time measurement
    let (result, duration) = PerformanceUtils::measure_time(|| {
        std::thread::sleep(std::time::Duration::from_millis(10));
        42
    });
    
    assert_eq!(result, 42);
    assert!(duration.as_millis() >= 10);
    
    // Test benchmarking
    let duration = PerformanceUtils::benchmark(|| {
        // Simple operation
        let _ = 1 + 1;
    }, 1000);
    
    assert!(duration.as_nanos() > 0);
}

#[test]
fn test_validation_utils() {
    use emulite::utils::ValidationUtils;
    
    // Test ROM validation (this would fail with non-existent file)
    let result = ValidationUtils::validate_rom("nonexistent.rom");
    assert!(result.is_err());
    
    // Test memory address validation
    let result = ValidationUtils::validate_address(0x1000, 0x2000);
    assert!(result.is_ok());
    
    let result = ValidationUtils::validate_address(0x3000, 0x2000);
    assert!(result.is_err());
}

#[test]
fn test_hash_utils() {
    use emulite::utils::HashUtils;
    
    let data = b"Hello, World!";
    
    // Test CRC32
    let crc32 = HashUtils::crc32(data);
    assert!(crc32 > 0);
    
    // Test simple hash
    let simple_hash = HashUtils::simple_hash(data);
    assert!(simple_hash > 0);
    
    // Test MD5 (placeholder)
    let md5 = HashUtils::md5(data);
    assert_eq!(md5.len(), 32);
}

#[test]
fn test_platform_utils() {
    use emulite::utils::PlatformUtils;
    
    // Test OS detection
    let os = PlatformUtils::get_os();
    assert!(!os.is_empty());
    
    // Test architecture detection
    let arch = PlatformUtils::get_arch();
    assert!(!arch.is_empty());
    
    // Test mobile detection
    let is_mobile = PlatformUtils::is_mobile();
    assert!(!is_mobile); // Should be false on desktop
    
    // Test path separator
    let separator = PlatformUtils::get_path_separator();
    assert!(separator == '/' || separator == '\\');
}
