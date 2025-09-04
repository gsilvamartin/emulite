//! PlayStation 1 emulator implementation

use crate::{
    platforms::{Platform, PlatformInfo},
    cpu::{Cpu, CpuFactory},
    memory::{MemoryMapper, RomDevice, RamDevice},
    EmuliteResult, EmuliteError,
};
use std::path::Path;
use std::sync::{Arc, RwLock};

/// PlayStation 1 emulator
pub struct Ps1 {
    cpu: Box<dyn Cpu>,
    memory: MemoryMapper,
    rom: Option<Vec<u8>>,
    gpu: Ps1Gpu,
    spu: Ps1Spu,
    cdrom: Ps1Cdrom,
    cart: Ps1Cartridge,
    cycles: u64,
}

/// PlayStation 1 GPU (Graphics Processing Unit)
struct Ps1Gpu {
    // Registers
    gp0: u32,
    gp1: u32,
    status: u32,
    
    // Internal state
    vram: Box<[u8; 1024 * 1024]>, // 1MB VRAM
    scanline: u16,
    cycle: u16,
    frame: u64,
    display_enabled: bool,
    frame_buffer: Vec<u8>,
    
    // Drawing state
    drawing_area: (u16, u16, u16, u16),
    display_area: (u16, u16, u16, u16),
    display_mode: u32,
}

/// PlayStation 1 SPU (Sound Processing Unit)
struct Ps1Spu {
    // Voice registers
    voices: [SpuVoice; 24],
    
    // Global registers
    main_volume_left: u16,
    main_volume_right: u16,
    reverb_volume_left: u16,
    reverb_volume_right: u16,
    voice_key_on: u32,
    voice_key_off: u32,
    voice_fm: u32,
    voice_noise: u32,
    voice_reverb: u32,
    voice_end: u32,
    
    // Reverb
    reverb_enabled: bool,
    reverb_buffer: Box<[i16; 32768]>,
    
    // Sample RAM
    sample_ram: Box<[u8; 512 * 1024]>, // 512KB sample RAM
}

/// SPU Voice
#[derive(Copy, Clone)]
struct SpuVoice {
    volume_left: u16,
    volume_right: u16,
    sample_rate: u16,
    start_address: u16,
    attack_rate: u8,
    decay_rate: u8,
    sustain_rate: u8,
    release_rate: u8,
    attack_level: u8,
    decay_level: u8,
    sustain_level: u8,
    release_level: u8,
    current_address: u32,
    current_volume: u16,
    adsr_phase: AdsrPhase,
    adsr_volume: u16,
    key_on: bool,
    key_off: bool,
    reverb_enabled: bool,
    noise_enabled: bool,
    pitch_modulation: bool,
}

/// ADSR Phase
#[derive(Debug, Clone, Copy)]
pub enum AdsrPhase {
    Attack,
    Decay,
    Sustain,
    Release,
    Off,
}

/// PlayStation 1 CD-ROM
struct Ps1Cdrom {
    // Registers
    index: u8,
    status: u8,
    command: u8,
    request: u8,
    interrupt_enable: u8,
    interrupt_flag: u8,
    
    // Drive state
    drive_state: CdromDriveState,
    sector_buffer: [u8; 2352],
    current_sector: u32,
    total_sectors: u32,
    
    // Disc information
    disc_type: CdromDiscType,
    track_count: u8,
    current_track: u8,
}

/// CD-ROM Drive State
#[derive(Debug, Clone, Copy)]
enum CdromDriveState {
    Idle,
    Reading,
    Seeking,
    Spinning,
    Error,
}

/// CD-ROM Disc Type
#[derive(Debug, Clone, Copy)]
enum CdromDiscType {
    Audio,
    Data,
    Mixed,
    None,
}

/// PlayStation 1 Cartridge
struct Ps1Cartridge {
    data: Vec<u8>,
    header: Ps1Header,
    region: Ps1Region,
}

/// PlayStation 1 Header
struct Ps1Header {
    system_id: String,
    cd_type: u8,
    region: u8,
    version: u8,
    title: String,
    publisher: String,
    disc_id: String,
}

/// PlayStation 1 Region
#[derive(Debug, Clone, Copy)]
enum Ps1Region {
    NTSC,
    PAL,
    NTSCJ,
}

impl Ps1 {
    pub fn new() -> EmuliteResult<Self> {
        let cpu = CpuFactory::create("mips")?; // PS1 uses MIPS R3000A
        let mut memory = MemoryMapper::new(0x20000000); // 512MB address space
        
        // Add RAM (2MB at 0x00000000-0x001FFFFF)
        let ram = Arc::new(RwLock::new(RamDevice::new(2 * 1024 * 1024, 0x00000000, "RAM".to_string())));
        memory.add_device("RAM".to_string(), ram)?;
        
        // Add BIOS (512KB at 0x1FC00000-0x1FC7FFFF)
        let bios = Arc::new(RwLock::new(RamDevice::new(512 * 1024, 0x1FC00000, "BIOS".to_string())));
        memory.add_device("BIOS".to_string(), bios)?;
        
        // Add GPU registers (0x1F801810-0x1F80181F)
        let gpu = Ps1Gpu::new();
        
        // Add SPU registers (0x1F801C00-0x1F801DFF)
        let spu = Ps1Spu::new();
        
        // Add CD-ROM registers (0x1F801800-0x1F801803)
        let cdrom = Ps1Cdrom::new();
        
        Ok(Self {
            cpu,
            memory,
            rom: None,
            gpu,
            spu,
            cdrom,
            cart: Ps1Cartridge::new(),
            cycles: 0,
        })
    }
    
    fn step_cpu(&mut self) -> EmuliteResult<()> {
        // PS1 runs at ~33.8688 MHz
        self.cpu.step()?;
        self.cycles += 1;
        
        // Update GPU (1x CPU speed)
        self.gpu.step()?;
        
        // Update SPU (1x CPU speed)
        self.spu.step()?;
        
        // Update CD-ROM (1x CPU speed)
        self.cdrom.step()?;
        
        Ok(())
    }
}

impl Platform for Ps1 {
    fn load_rom(&mut self, rom_path: &str) -> EmuliteResult<()> {
        let path = Path::new(rom_path);
        let rom_data = std::fs::read(path)?;
        
        if rom_data.len() < 2048 {
            return Err(EmuliteError::InvalidRom("ROM too small".to_string()));
        }
        
        // Parse PlayStation header
        let header = Ps1Header::parse(&rom_data)?;
        let region = Ps1Region::from_byte(header.region)?;
        
        self.cart = Ps1Cartridge {
            data: rom_data.clone(),
            header,
            region,
        };
        
        // Load BIOS if available
        self.load_bios()?;
        
        // Reset CPU
        self.cpu.reset()?;
        self.cycles = 0;
        
        self.rom = Some(rom_data);
        Ok(())
    }
    
    fn step(&mut self) -> EmuliteResult<()> {
        self.step_cpu()?;
        Ok(())
    }
    
    fn reset(&mut self) -> EmuliteResult<()> {
        self.cpu.reset()?;
        self.gpu.reset();
        self.spu.reset();
        self.cdrom.reset();
        self.cycles = 0;
        Ok(())
    }
    
    fn name(&self) -> &str {
        "PlayStation"
    }
    
    fn version(&self) -> &str {
        "1.0.0"
    }
    
    fn info(&self) -> PlatformInfo {
        PlatformInfo {
            name: "PlayStation".to_string(),
            version: "1.0.0".to_string(),
            cpu_type: "MIPS R3000A".to_string(),
            memory_size: 2 * 1024 * 1024, // 2MB RAM
            video_resolution: (320, 240),
            audio_channels: 24,
            supported_formats: vec!["iso".to_string(), "bin".to_string(), "img".to_string()],
        }
    }
    
    fn save_state(&self, path: &str) -> EmuliteResult<()> {
        // TODO: Implement save state
        Err(EmuliteError::UnsupportedPlatform("Save state not implemented".to_string()))
    }
    
    fn load_state(&mut self, path: &str) -> EmuliteResult<()> {
        // TODO: Implement load state
        Err(EmuliteError::UnsupportedPlatform("Load state not implemented".to_string()))
    }
    
    fn get_cpu(&self) -> Option<&dyn crate::cpu::Cpu> {
        Some(self.cpu.as_ref())
    }
    
    fn get_frame_data(&self) -> EmuliteResult<Vec<u8>> {
        // PS1 resolution is 320x240 pixels
        let width = 320;
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

impl Ps1 {
    fn load_bios(&mut self) -> EmuliteResult<()> {
        // Try to load BIOS from common locations
        let bios_paths = [
            "bios/SCPH1001.bin",
            "bios/SCPH5501.bin",
            "bios/SCPH7001.bin",
        ];
        
        for path in &bios_paths {
            if Path::new(path).exists() {
                let bios_data = std::fs::read(path)?;
                if let Some(bios_device) = self.memory.get_device("BIOS") {
                    // Load BIOS into memory
                    for (i, &byte) in bios_data.iter().enumerate() {
                        bios_device.write().unwrap().write(0x1FC00000 + i as u32, byte)?;
                    }
                }
                return Ok(());
            }
        }
        
        // No BIOS found, continue without it
        log::warn!("No BIOS found, some games may not work properly");
        Ok(())
    }
}

impl Ps1Gpu {
    fn new() -> Self {
        Self {
            gp0: 0,
            gp1: 0,
            status: 0,
            vram: Box::new([0; 1024 * 1024]),
            scanline: 0,
            cycle: 0,
            frame: 0,
            display_enabled: true,
            frame_buffer: vec![0; 320 * 240],
            drawing_area: (0, 0, 0, 0),
            display_area: (0, 0, 0, 0),
            display_mode: 0,
        }
    }
    
    fn step(&mut self) -> EmuliteResult<()> {
        self.cycle += 1;
        
        // PS1 GPU runs at 53.2 MHz
        // Each scanline has 3413 cycles
        if self.cycle >= 3413 {
            self.cycle = 0;
            self.scanline += 1;
            
            // End of frame
            if self.scanline >= 263 {
                self.scanline = 0;
                self.frame += 1;
            }
        }
        
        Ok(())
    }
    
    fn reset(&mut self) {
        *self = Self::new();
    }
}

impl Ps1Spu {
    fn new() -> Self {
        Self {
            voices: [SpuVoice::new(); 24],
            main_volume_left: 0,
            main_volume_right: 0,
            reverb_volume_left: 0,
            reverb_volume_right: 0,
            voice_key_on: 0,
            voice_key_off: 0,
            voice_fm: 0,
            voice_noise: 0,
            voice_reverb: 0,
            voice_end: 0,
            reverb_enabled: false,
            reverb_buffer: Box::new([0; 32768]),
            sample_ram: Box::new([0; 512 * 1024]),
        }
    }
    
    fn step(&mut self) -> EmuliteResult<()> {
        // Update all voices
        for voice in &mut self.voices {
            voice.step();
        }
        
        Ok(())
    }
    
    fn reset(&mut self) {
        *self = Self::new();
    }
}

impl SpuVoice {
    fn new() -> Self {
        Self {
            volume_left: 0,
            volume_right: 0,
            sample_rate: 0,
            start_address: 0,
            attack_rate: 0,
            decay_rate: 0,
            sustain_rate: 0,
            release_rate: 0,
            attack_level: 0,
            decay_level: 0,
            sustain_level: 0,
            release_level: 0,
            current_address: 0,
            current_volume: 0,
            adsr_phase: AdsrPhase::Off,
            adsr_volume: 0,
            key_on: false,
            key_off: false,
            reverb_enabled: false,
            noise_enabled: false,
            pitch_modulation: false,
        }
    }
    
    fn step(&mut self) {
        if self.key_on {
            self.adsr_phase = AdsrPhase::Attack;
            self.key_on = false;
        }
        
        if self.key_off {
            self.adsr_phase = AdsrPhase::Release;
            self.key_off = false;
        }
        
        // Update ADSR envelope
        match self.adsr_phase {
            AdsrPhase::Attack => {
                // Attack phase
                if self.adsr_volume >= self.attack_level as u16 {
                    self.adsr_phase = AdsrPhase::Decay;
                }
            },
            AdsrPhase::Decay => {
                // Decay phase
                if self.adsr_volume <= self.decay_level as u16 {
                    self.adsr_phase = AdsrPhase::Sustain;
                }
            },
            AdsrPhase::Sustain => {
                // Sustain phase
            },
            AdsrPhase::Release => {
                // Release phase
                if self.adsr_volume <= 0 {
                    self.adsr_phase = AdsrPhase::Off;
                }
            },
            AdsrPhase::Off => {
                // Voice is off
            }
        }
    }
}

impl Ps1Cdrom {
    fn new() -> Self {
        Self {
            index: 0,
            status: 0,
            command: 0,
            request: 0,
            interrupt_enable: 0,
            interrupt_flag: 0,
            drive_state: CdromDriveState::Idle,
            sector_buffer: [0; 2352],
            current_sector: 0,
            total_sectors: 0,
            disc_type: CdromDiscType::None,
            track_count: 0,
            current_track: 0,
        }
    }
    
    fn step(&mut self) -> EmuliteResult<()> {
        // Update CD-ROM drive state
        match self.drive_state {
            CdromDriveState::Reading => {
                // Simulate reading
                self.current_sector += 1;
                if self.current_sector >= self.total_sectors {
                    self.drive_state = CdromDriveState::Idle;
                }
            },
            _ => {}
        }
        
        Ok(())
    }
    
    fn reset(&mut self) {
        *self = Self::new();
    }
}

impl Ps1Header {
    fn parse(data: &[u8]) -> EmuliteResult<Self> {
        if data.len() < 2048 {
            return Err(EmuliteError::InvalidRom("ROM too small for header".to_string()));
        }
        
        let system_id = String::from_utf8_lossy(&data[0x00..0x0A]).trim_end_matches('\0').to_string();
        let cd_type = data[0x0F];
        let region = data[0x0E];
        let version = data[0x0D];
        let title = String::from_utf8_lossy(&data[0x10..0x20]).trim_end_matches('\0').to_string();
        let publisher = String::from_utf8_lossy(&data[0x20..0x30]).trim_end_matches('\0').to_string();
        let disc_id = String::from_utf8_lossy(&data[0x30..0x40]).trim_end_matches('\0').to_string();
        
        Ok(Self {
            system_id,
            cd_type,
            region,
            version,
            title,
            publisher,
            disc_id,
        })
    }
}

impl Ps1Region {
    fn from_byte(region: u8) -> EmuliteResult<Self> {
        match region {
            0x41 => Ok(Ps1Region::NTSC),
            0x44 => Ok(Ps1Region::PAL),
            0x49 => Ok(Ps1Region::NTSCJ),
            _ => Err(EmuliteError::InvalidRom(format!("Unknown region: 0x{:02X}", region))),
        }
    }
}

impl Ps1Cartridge {
    fn new() -> Self {
        Self {
            data: Vec::new(),
            header: Ps1Header {
                system_id: String::new(),
                cd_type: 0,
                region: 0,
                version: 0,
                title: String::new(),
                publisher: String::new(),
                disc_id: String::new(),
            },
            region: Ps1Region::NTSC,
        }
    }
}
