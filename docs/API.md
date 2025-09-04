# Emulite API Documentation

Este documento descreve a API pública do Emulite.

## Visão Geral

A API do Emulite é projetada para ser simples e intuitiva, permitindo que desenvolvedores criem emuladores personalizados ou integrem o Emulite em suas aplicações.

## Estrutura da API

### 1. Core API

#### Emulator

```rust
pub struct Emulator {
    platform: Box<dyn Platform>,
    config: Config,
    debugger: Option<Debugger>,
}

impl Emulator {
    pub fn new(platform: Box<dyn Platform>, config: Config) -> Self;
    pub fn load_rom(&mut self, rom_data: &[u8]) -> EmuliteResult<()>;
    pub fn run(&mut self) -> EmuliteResult<()>;
    pub fn pause(&mut self);
    pub fn resume(&mut self);
    pub fn reset(&mut self) -> EmuliteResult<()>;
    pub fn get_state(&self) -> EmulatorState;
    pub fn set_state(&mut self, state: &EmulatorState) -> EmuliteResult<()>;
}
```

#### Platform

```rust
pub trait Platform {
    fn name(&self) -> &str;
    fn version(&self) -> &str;
    fn cpu(&self) -> &dyn Cpu;
    fn memory(&self) -> &dyn MemoryBus;
    fn ppu(&self) -> &dyn Ppu;
    fn apu(&self) -> &dyn Apu;
    fn input(&self) -> &dyn InputDevice;
    fn step(&mut self) -> EmuliteResult<()>;
    fn reset(&mut self) -> EmuliteResult<()>;
}
```

### 2. CPU API

#### Cpu

```rust
pub trait Cpu {
    fn step(&mut self) -> EmuliteResult<()>;
    fn reset(&mut self) -> EmuliteResult<()>;
    fn get_register(&self, reg: Register) -> u32;
    fn set_register(&mut self, reg: Register, value: u32);
    fn get_pc(&self) -> u32;
    fn set_pc(&mut self, pc: u32);
    fn get_flags(&self) -> CpuFlags;
    fn set_flags(&mut self, flags: CpuFlags);
}
```

#### Register

```rust
pub enum Register {
    A, B, C, D, E, F, H, L,  // 8-bit registers
    AF, BC, DE, HL,          // 16-bit register pairs
    SP, PC,                  // Stack pointer and program counter
    IX, IY,                  // Index registers
}
```

#### CpuFlags

```rust
pub struct CpuFlags {
    pub zero: bool,
    pub negative: bool,
    pub carry: bool,
    pub overflow: bool,
    pub interrupt_disable: bool,
    pub decimal: bool,
}
```

### 3. Memory API

#### MemoryBus

```rust
pub trait MemoryBus {
    fn read_byte(&self, addr: u32) -> EmuliteResult<u8>;
    fn write_byte(&mut self, addr: u32, value: u8) -> EmuliteResult<()>;
    fn read_word(&self, addr: u32) -> EmuliteResult<u16>;
    fn write_word(&mut self, addr: u32, value: u16) -> EmuliteResult<()>;
    fn read_dword(&self, addr: u32) -> EmuliteResult<u32>;
    fn write_dword(&mut self, addr: u32, value: u32) -> EmuliteResult<()>;
}
```

#### MemoryAccess

```rust
pub trait MemoryAccess {
    fn read_byte(&self, addr: u32) -> EmuliteResult<u8>;
    fn write_byte(&mut self, addr: u32, value: u8) -> EmuliteResult<()>;
    fn get_size(&self) -> usize;
    fn get_start_addr(&self) -> u32;
    fn get_end_addr(&self) -> u32;
}
```

### 4. Audio API

#### Apu

```rust
pub trait Apu {
    fn step(&mut self) -> EmuliteResult<()>;
    fn reset(&mut self) -> EmuliteResult<()>;
    fn get_sample_rate(&self) -> u32;
    fn get_channels(&self) -> u8;
    fn get_buffer_size(&self) -> usize;
    fn get_audio_data(&self) -> &[f32];
}
```

#### AudioOutput

```rust
pub trait AudioOutput {
    fn play(&mut self, data: &[f32]) -> EmuliteResult<()>;
    fn stop(&mut self) -> EmuliteResult<()>;
    fn set_volume(&mut self, volume: f32) -> EmuliteResult<()>;
    fn get_volume(&self) -> f32;
}
```

### 5. Video API

#### Ppu

```rust
pub trait Ppu {
    fn step(&mut self) -> EmuliteResult<()>;
    fn reset(&mut self) -> EmuliteResult<()>;
    fn get_width(&self) -> u32;
    fn get_height(&self) -> u32;
    fn get_pixel_data(&self) -> &[u8];
    fn get_palette(&self) -> &[u32];
}
```

#### VideoOutput

```rust
pub trait VideoOutput {
    fn present(&mut self, data: &[u8]) -> EmuliteResult<()>;
    fn set_resolution(&mut self, width: u32, height: u32) -> EmuliteResult<()>;
    fn get_resolution(&self) -> (u32, u32);
    fn set_fullscreen(&mut self, fullscreen: bool) -> EmuliteResult<()>;
    fn is_fullscreen(&self) -> bool;
}
```

### 6. Input API

#### InputDevice

```rust
pub trait InputDevice {
    fn get_button_state(&self, button: Button) -> bool;
    fn get_axis_value(&self, axis: Axis) -> f32;
    fn get_connected_controllers(&self) -> Vec<ControllerId>;
    fn get_controller(&self, id: ControllerId) -> Option<&dyn Controller>;
}
```

#### Controller

```rust
pub trait Controller {
    fn get_button_state(&self, button: Button) -> bool;
    fn get_axis_value(&self, axis: Axis) -> f32;
    fn get_dpad_state(&self) -> DPadState;
    fn is_connected(&self) -> bool;
}
```

### 7. Debug API

#### Debugger

```rust
pub trait Debugger {
    fn add_breakpoint(&mut self, bp: Breakpoint) -> EmuliteResult<()>;
    fn remove_breakpoint(&mut self, addr: u32) -> EmuliteResult<()>;
    fn step(&mut self) -> EmuliteResult<()>;
    fn continue_execution(&mut self) -> EmuliteResult<()>;
    fn get_registers(&self) -> HashMap<String, u32>;
    fn get_memory(&self, addr: u32, size: usize) -> EmuliteResult<Vec<u8>>;
}
```

#### Breakpoint

```rust
pub struct Breakpoint {
    pub address: u32,
    pub condition: Option<String>,
    pub enabled: bool,
}
```

### 8. Configuration API

#### Config

```rust
pub struct Config {
    pub audio: AudioConfig,
    pub video: VideoConfig,
    pub input: InputConfig,
    pub debug: DebugConfig,
    pub performance: PerformanceConfig,
}

pub struct AudioConfig {
    pub sample_rate: u32,
    pub channels: u8,
    pub buffer_size: usize,
    pub volume: f32,
}

pub struct VideoConfig {
    pub width: u32,
    pub height: u32,
    pub fullscreen: bool,
    pub vsync: bool,
    pub shader_path: Option<String>,
}

pub struct InputConfig {
    pub keyboard_mapping: HashMap<Key, Button>,
    pub gamepad_mapping: HashMap<Button, Button>,
    pub input_recording: bool,
}

pub struct DebugConfig {
    pub enabled: bool,
    pub breakpoints: Vec<Breakpoint>,
    pub logging_level: LogLevel,
}

pub struct PerformanceConfig {
    pub target_fps: u32,
    pub cpu_cycles_per_frame: u64,
    pub audio_latency: f32,
}
```

## Exemplos de Uso

### 1. Criando um Emulador Básico

```rust
use emulite::*;

fn main() -> EmuliteResult<()> {
    // Criar configuração
    let config = Config::default();
    
    // Criar plataforma
    let platform = PlatformFactory::create("nes")?;
    
    // Criar emulador
    let mut emulator = Emulator::new(platform, config);
    
    // Carregar ROM
    let rom_data = std::fs::read("game.nes")?;
    emulator.load_rom(&rom_data)?;
    
    // Executar emulador
    emulator.run()?;
    
    Ok(())
}
```

### 2. Usando o Sistema de Debug

```rust
use emulite::*;

fn debug_example() -> EmuliteResult<()> {
    let mut emulator = Emulator::new(platform, config);
    
    // Adicionar breakpoint
    let bp = Breakpoint {
        address: 0x8000,
        condition: None,
        enabled: true,
    };
    emulator.debugger.as_mut().unwrap().add_breakpoint(bp)?;
    
    // Executar até o breakpoint
    emulator.debugger.as_mut().unwrap().continue_execution()?;
    
    // Inspecionar registros
    let registers = emulator.debugger.as_ref().unwrap().get_registers();
    println!("Registers: {:?}", registers);
    
    // Inspecionar memória
    let memory = emulator.debugger.as_ref().unwrap().get_memory(0x8000, 16)?;
    println!("Memory: {:?}", memory);
    
    Ok(())
}
```

### 3. Configurando Áudio e Vídeo

```rust
use emulite::*;

fn configure_audio_video() -> EmuliteResult<()> {
    let mut config = Config::default();
    
    // Configurar áudio
    config.audio.sample_rate = 44100;
    config.audio.channels = 2;
    config.audio.volume = 0.8;
    
    // Configurar vídeo
    config.video.width = 1920;
    config.video.height = 1080;
    config.video.fullscreen = true;
    config.video.vsync = true;
    
    let platform = PlatformFactory::create("snes")?;
    let mut emulator = Emulator::new(platform, config);
    
    // ... resto do código
    
    Ok(())
}
```

## Tratamento de Erros

O Emulite usa o tipo `EmuliteResult<T>` para tratamento de erros:

```rust
pub type EmuliteResult<T> = Result<T, EmuliteError>;

#[derive(Debug, thiserror::Error)]
pub enum EmuliteError {
    #[error("ROM not found: {0}")]
    RomNotFound(String),
    
    #[error("Invalid ROM format: {0}")]
    InvalidRomFormat(String),
    
    #[error("Platform not supported: {0}")]
    PlatformNotSupported(String),
    
    #[error("CPU error: {0}")]
    CpuError(String),
    
    #[error("Memory error: {0}")]
    MemoryError(String),
    
    #[error("Audio error: {0}")]
    AudioError(String),
    
    #[error("Video error: {0}")]
    VideoError(String),
    
    #[error("Input error: {0}")]
    InputError(String),
    
    #[error("Debug error: {0}")]
    DebugError(String),
    
    #[error("Configuration error: {0}")]
    ConfigError(String),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}
```

## Conclusão

A API do Emulite é projetada para ser simples, intuitiva e extensível. Ela fornece uma base sólida para criar emuladores personalizados ou integrar o Emulite em aplicações existentes.
