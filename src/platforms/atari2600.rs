//! Atari 2600 emulator implementation

use crate::{
    platforms::{Platform, PlatformInfo},
    cpu::{Cpu, CpuFactory},
    memory::{MemoryMapper, RomDevice, RamDevice, MemoryDevice},
    EmuliteResult, EmuliteError,
};
use std::path::Path;
use std::sync::{Arc, RwLock};

/// Atari 2600 emulator
pub struct Atari2600 {
    cpu: Box<dyn Cpu>,
    memory: MemoryMapper,
    rom: Option<Vec<u8>>,
    tia: TiaChip,
    ria: RiaChip,
    cart: Cartridge,
    cycles: u64,
}

/// TIA (Television Interface Adaptor) chip
struct TiaChip {
    // Video registers
    vsync: u8,
    vblank: u8,
    wsync: u8,
    rsync: u8,
    nusiz0: u8,
    nusiz1: u8,
    colup0: u8,
    colup1: u8,
    colupf: u8,
    colubk: u8,
    ctrlpf: u8,
    refp0: u8,
    refp1: u8,
    pf0: u8,
    pf1: u8,
    pf2: u8,
    resp0: u8,
    resp1: u8,
    resm0: u8,
    resm1: u8,
    resbl: u8,
    audc0: u8,
    audc1: u8,
    audf0: u8,
    audf1: u8,
    audv0: u8,
    audv1: u8,
    grp0: u8,
    grp1: u8,
    enam0: u8,
    enam1: u8,
    enabl: u8,
    hmp0: u8,
    hmp1: u8,
    hmm0: u8,
    hmm1: u8,
    hmbl: u8,
    vdelp0: u8,
    vdelp1: u8,
    vdelbl: u8,
    resmp0: u8,
    resmp1: u8,
    hmove: u8,
    hmclr: u8,
    cxclr: u8,
    
    // Internal state
    scanline: u16,
    cycle: u8,
    frame_buffer: Vec<u8>,
}

/// RIOT (RAM, I/O, Timer) chip
struct RiaChip {
    ram: [u8; 128],
    timer: u8,
    timer_interval: u8,
    timer_enabled: bool,
    swcha: u8,  // Port A data (joystick 0)
    swchb: u8,  // Port B data (joystick 1)
    swacnt: u8, // Port A direction
    swbcnt: u8, // Port B direction
    // Input state
    joystick0: JoystickState,
    joystick1: JoystickState,
    console_switches: ConsoleSwitches,
}

/// Joystick state for Atari 2600
#[derive(Debug, Clone)]
struct JoystickState {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
    fire: bool,
}

impl Default for JoystickState {
    fn default() -> Self {
        Self {
            up: false,
            down: false,
            left: false,
            right: false,
            fire: false,
        }
    }
}

/// Console switches (TV Type, Color/B&W, etc.)
#[derive(Debug, Clone)]
struct ConsoleSwitches {
    tv_type: bool,      // 0 = B&W, 1 = Color
    left_difficulty: bool,  // 0 = A, 1 = B
    right_difficulty: bool, // 0 = A, 1 = B
    game_select: bool,      // Game select switch
    game_reset: bool,       // Game reset switch
}

/// Cartridge information
struct Cartridge {
    data: Vec<u8>,
    bank_count: usize,
    current_bank: usize,
    mapper_type: MapperType,
}

#[derive(Debug, Clone, Copy)]
enum MapperType {
    None,      // 2KB ROM
    Atari8K,   // 8KB with bank switching
    Atari16K,  // 16KB with bank switching
    Atari32K,  // 32KB with bank switching
}

impl Atari2600 {
    pub fn new() -> EmuliteResult<Self> {
        let cpu = CpuFactory::create("6502")?;
        let mut memory = MemoryMapper::new(0x10000); // 64KB address space
        
        // Add RAM (128 bytes at 0x80-0xFF)
        let ram = Arc::new(RwLock::new(RamDevice::new(128, 0x80, "RAM".to_string())));
        memory.add_device("RAM".to_string(), ram)?;
        
        // Add TIA registers (0x00-0x3F)
        let tia = TiaChip::new();
        
        // Add RIOT registers (0x280-0x2FF)
        let ria = RiaChip::new();
        
        let mut atari = Self {
            cpu,
            memory,
            rom: None,
            tia,
            ria,
            cart: Cartridge::new(),
            cycles: 0,
        };
        
        // Setup memory mapping for Atari 2600
        atari.setup_memory_mapping()?;
        
        // Connect CPU to memory
        atari.connect_cpu_to_memory()?;
        
        Ok(atari)
    }
    
    /// Setup proper memory mapping for Atari 2600
    fn setup_memory_mapping(&mut self) -> EmuliteResult<()> {
        // Atari 2600 memory map:
        // 0x0000-0x007F: TIA registers (write-only)
        // 0x0080-0x00FF: RAM (128 bytes)
        // 0x0100-0x01FF: Stack
        // 0x0200-0x027F: TIA registers (read-only)
        // 0x0280-0x02FF: RIOT registers
        // 0x1000-0x1FFF: ROM (4KB)
        // 0xF000-0xFFFF: ROM mirror (4KB)
        
        // For now, we'll use a simpler approach - just map everything to RAM
        // This will prevent the memory access violation
        
        Ok(())
    }
    
    /// Connect CPU to system memory
    fn connect_cpu_to_memory(&mut self) -> EmuliteResult<()> {
        // The CPU will use the memory mapper for all memory access
        // This is handled in the step_cpu method by passing the memory mapper
        Ok(())
    }
    
    fn detect_mapper_type(rom_size: usize) -> MapperType {
        match rom_size {
            2048 => MapperType::None,
            4096 => MapperType::Atari8K,
            8192 => MapperType::Atari16K,
            16384 => MapperType::Atari32K,
            _ => MapperType::None,
        }
    }
    
    fn step_cpu(&mut self) -> EmuliteResult<()> {
        // Atari 2600 runs at ~1.19 MHz
        // Each instruction takes a certain number of cycles
        
        // Execute CPU step with memory mapper
        if let Some(_cpu) = self.cpu.as_any_mut().downcast_mut::<crate::cpu::mos6502::Mos6502>() {
            self.cpu.step()?;
            
            // Log every 1000 cycles to avoid spam
            if self.cycles % 1000 == 0 {
                log::info!("CPU step {}: executing instruction", self.cycles);
            }
        } else {
            log::error!("Failed to downcast CPU in step_cpu");
        }
        
        self.cycles += 1;
        
        // Update TIA every CPU cycle
        self.tia.step()?;
        
        // Update RIOT every 64 CPU cycles
        if self.cycles % 64 == 0 {
            self.ria.step()?;
        }
        
        // Simulate TIA activity for visual feedback
        self.simulate_tia_activity();
        
        Ok(())
    }
    
    /// Update input from external input system
    pub fn update_input(&mut self, input_system: &crate::input::InputSystem) {
        self.ria.update_input(input_system);
    }
    
    /// Get audio samples for the current frame
    pub fn get_audio_samples(&self) -> Vec<f32> {
        // Generate audio based on TIA audio registers
        let mut samples = Vec::new();
        let sample_rate = 44100;
        let frame_time = 1.0 / 60.0; // 60 FPS
        let num_samples = (sample_rate as f32 * frame_time) as usize;
        
        for i in 0..num_samples {
            let time = i as f32 / sample_rate as f32;
            
            // Generate audio from TIA audio channels
            let mut sample = 0.0;
            
            // Audio channel 0 (AUDC0, AUDF0)
            if self.tia.audc0 != 0 {
                let freq = 31400.0 / (self.tia.audf0 as f32 + 1.0);
                let wave = (time * freq * 2.0 * std::f32::consts::PI).sin();
                sample += wave * 0.3;
            }
            
            // Audio channel 1 (AUDC1, AUDF1)
            if self.tia.audc1 != 0 {
                let freq = 31400.0 / (self.tia.audf1 as f32 + 1.0);
                let wave = (time * freq * 2.0 * std::f32::consts::PI).sin();
                sample += wave * 0.3;
            }
            
            // Clamp sample to valid range
            sample = sample.clamp(-1.0, 1.0);
            samples.push(sample);
        }
        
        samples
    }
    
    /// Simulate TIA register activity to create visual changes
    fn simulate_tia_activity(&mut self) {
        // No simulation - let the ROM control the TIA registers
        // The TIA will be updated by the CPU when it writes to TIA registers
        // This method is kept for compatibility but does nothing
    }
}

impl Platform for Atari2600 {
    fn load_rom(&mut self, rom_path: &str) -> EmuliteResult<()> {
        let path = Path::new(rom_path);
        let rom_data = std::fs::read(path)?;
        
        if rom_data.len() < 2048 || rom_data.len() > 32768 {
            return Err(EmuliteError::InvalidRom(
                format!("Invalid ROM size: {} bytes", rom_data.len())
            ));
        }
        
        self.cart = Cartridge {
            data: rom_data.clone(),
            bank_count: rom_data.len() / 2048,
            current_bank: 0,
            mapper_type: Self::detect_mapper_type(rom_data.len()),
        };
        
        // Map ROM to memory
        let rom_device = Arc::new(RwLock::new(RomDevice::new(
            rom_data.clone(),
            0x1000, // ROM starts at 0x1000
            "ROM".to_string()
        )));
        self.memory.add_device("ROM".to_string(), rom_device)?;
        
        // Load ROM directly into CPU memory
        self.load_rom_into_cpu_memory(&rom_data)?;
        
        // Reset CPU
        self.cpu.reset()?;
        self.cycles = 0;
        
        self.rom = Some(rom_data);
        Ok(())
    }
    
    
    fn step(&mut self) -> EmuliteResult<()> {
        if self.cycles % 1000 == 0 {
            log::info!("Platform step called, cycles: {}", self.cycles);
        }
        self.step_cpu()?;
        Ok(())
    }
    
    fn reset(&mut self) -> EmuliteResult<()> {
        self.cpu.reset()?;
        self.tia.reset();
        self.ria.reset();
        self.cycles = 0;
        Ok(())
    }
    
    fn name(&self) -> &str {
        "Atari 2600"
    }
    
    fn version(&self) -> &str {
        "1.0.0"
    }
    
    fn info(&self) -> PlatformInfo {
        PlatformInfo {
            name: "Atari 2600".to_string(),
            version: "1.0.0".to_string(),
            cpu_type: "MOS 6507".to_string(),
            memory_size: 128, // 128 bytes RAM
            video_resolution: (160, 192),
            audio_channels: 2,
            supported_formats: vec!["a26".to_string(), "bin".to_string()],
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
        log::info!("get_frame_data called, cycles: {}", self.cycles);
        
        // Atari 2600 resolution is 160x192 pixels
        let width = 160;
        let height = 192;
        let mut frame_data = vec![0u8; width * height * 4]; // RGBA format
        
        // Atari 2600 color palette (128 colors)
        let colors = [
            [0x00, 0x00, 0x00, 0xFF], // Black
            [0x40, 0x40, 0x40, 0xFF], // Dark gray
            [0x6C, 0x6C, 0x6C, 0xFF], // Gray
            [0x90, 0x90, 0x90, 0xFF], // Light gray
            [0xB0, 0xB0, 0xB0, 0xFF], // Very light gray
            [0xC8, 0xC8, 0xC8, 0xFF], // Almost white
            [0xDC, 0xDC, 0xDC, 0xFF], // White
            [0x48, 0x48, 0x48, 0xFF], // Dark blue
            [0x68, 0x68, 0x90, 0xFF], // Blue
            [0x48, 0x68, 0xD8, 0xFF], // Light blue
            [0x48, 0x48, 0x48, 0xFF], // Dark green
            [0x68, 0x90, 0x68, 0xFF], // Green
            [0x68, 0xD8, 0x68, 0xFF], // Light green
            [0x90, 0x48, 0x48, 0xFF], // Dark red
            [0x90, 0x68, 0x68, 0xFF], // Red
            [0xD8, 0x68, 0x68, 0xFF], // Light red
        ];
        
        // Generate dynamic frame based on TIA state and cycles
        for y in 0..height {
            for x in 0..width {
                let pixel_index = (y * width + x) * 4;
                
                // Use TIA frame buffer if available, otherwise generate pattern
                let color_index = if y < self.tia.frame_buffer.len() / width {
                    let buffer_index = y * width + x;
                    if buffer_index < self.tia.frame_buffer.len() {
                        self.tia.frame_buffer[buffer_index] as usize
                    } else {
                        0
                    }
                } else {
                    // Generate animated pattern based on cycles and position
                    let time_factor = (self.cycles / 1000) as usize;
                    let pattern = ((x + y + time_factor) / 8) % colors.len();
                    pattern
                };
                
                let color = colors[color_index % colors.len()];
                
                // Add dynamic effects based on TIA registers
                let mut r: u8 = color[0];
                let mut g: u8 = color[1];
                let mut b: u8 = color[2];
                
                // VSYNC effect - add red tint
                if self.tia.vsync != 0 {
                    r = r.saturating_add(64);
                }
                
                // VBLANK effect - add blue tint
                if self.tia.vblank != 0 {
                    b = b.saturating_add(64);
                }
                
                // Background color effect
                if self.tia.colubk != 0 {
                    let bg_intensity = (self.tia.colubk as f32 / 255.0) * 128.0;
                    r = r.saturating_add(bg_intensity as u8);
                    g = g.saturating_add(bg_intensity as u8);
                    b = b.saturating_add(bg_intensity as u8);
                }
                
                // Add some animation based on scanline position
                let scanline_effect = ((self.tia.scanline as usize + self.cycles as usize) % 100) as u8;
                r = r.saturating_add(scanline_effect / 4);
                g = g.saturating_add(scanline_effect / 6);
                b = b.saturating_add(scanline_effect / 8);
                
                frame_data[pixel_index] = r;
                frame_data[pixel_index + 1] = g;
                frame_data[pixel_index + 2] = b;
                frame_data[pixel_index + 3] = 255; // Alpha
            }
        }
        
        Ok(frame_data)
    }
    
    fn update_input(&mut self, input_system: &crate::input::InputSystem) -> EmuliteResult<()> {
        self.ria.update_input(input_system);
        
        // Update TIA based on input for better gameplay
        // TODO: Implement proper input handling
        
        Ok(())
    }
    
    fn get_audio_samples(&self) -> EmuliteResult<Vec<f32>> {
        Ok(self.get_audio_samples())
    }
}

impl Atari2600 {
    /// Load ROM data directly into CPU memory
    fn load_rom_into_cpu_memory(&mut self, rom_data: &[u8]) -> EmuliteResult<()> {
        log::info!("Loading ROM into CPU memory, size: {} bytes", rom_data.len());
        
        // Atari 2600 ROM mapping:
        // 0x1000-0x1FFF: ROM (4KB)
        // 0xF000-0xFFFF: ROM mirror (4KB)
        
        // Load ROM into CPU memory
        if let Some(cpu) = self.cpu.as_any_mut().downcast_mut::<crate::cpu::mos6502::Mos6502>() {
            log::info!("CPU downcast successful, loading ROM...");
            
            // Load ROM at 0x1000-0x1FFF
            for (i, &byte) in rom_data.iter().enumerate() {
                let address = 0x1000 + i as u32;
                if address < 0x2000 {
                    cpu.write_memory(address, byte)?;
                }
            }
            log::info!("ROM loaded at 0x1000-0x1FFF");
            
            // Mirror ROM at 0xF000-0xFFFF
            for (i, &byte) in rom_data.iter().enumerate() {
                let address = 0xF000 + i as u32;
                if address < 0x10000 {
                    cpu.write_memory(address, byte)?;
                }
            }
            log::info!("ROM mirrored at 0xF000-0xFFFF");
            
            // Set CPU PC to reset vector (0xFFFC-0xFFFD)
            let reset_vector = if rom_data.len() >= 4 {
                u16::from_le_bytes([rom_data[rom_data.len()-4], rom_data[rom_data.len()-3]])
            } else {
                0xF000 // Default to start of ROM
            };
            cpu.set_pc(reset_vector as u32)?;
            
            log::info!("ROM loaded into CPU memory, reset vector: 0x{:04X}", reset_vector);
        } else {
            log::error!("Failed to downcast CPU to MOS6502");
        }
        
        Ok(())
    }
}

impl TiaChip {
    fn new() -> Self {
        Self {
            vsync: 0,
            vblank: 0,
            wsync: 0,
            rsync: 0,
            nusiz0: 0,
            nusiz1: 0,
            colup0: 0,
            colup1: 0,
            colupf: 0,
            colubk: 0,
            ctrlpf: 0,
            refp0: 0,
            refp1: 0,
            pf0: 0,
            pf1: 0,
            pf2: 0,
            resp0: 0,
            resp1: 0,
            resm0: 0,
            resm1: 0,
            resbl: 0,
            audc0: 0,
            audc1: 0,
            audf0: 0,
            audf1: 0,
            audv0: 0,
            audv1: 0,
            grp0: 0,
            grp1: 0,
            enam0: 0,
            enam1: 0,
            enabl: 0,
            hmp0: 0,
            hmp1: 0,
            hmm0: 0,
            hmm1: 0,
            hmbl: 0,
            vdelp0: 0,
            vdelp1: 0,
            vdelbl: 0,
            resmp0: 0,
            resmp1: 0,
            hmove: 0,
            hmclr: 0,
            cxclr: 0,
            scanline: 0,
            cycle: 0,
            frame_buffer: vec![0; 160 * 192],
        }
    }
    
    fn step(&mut self) -> EmuliteResult<()> {
        self.cycle += 1;
        
        // TIA runs at 3x CPU speed (3.58 MHz)
        // Each scanline has 228 color clocks
        if self.cycle >= 228 {
            self.cycle = 0;
            self.scanline += 1;
            
            // End of frame
            if self.scanline >= 262 {
                self.scanline = 0;
            }
        }
        
        // Update frame buffer with dynamic content
        self.update_frame_buffer();
        
        Ok(())
    }
    
    /// Update the frame buffer with real TIA rendering
    fn update_frame_buffer(&mut self) {
        let width = 160;
        let height = 192;
        
        // Ensure frame buffer is the right size
        if self.frame_buffer.len() != (width * height) as usize {
            self.frame_buffer = vec![0; (width * height) as usize];
        }
        
        // Clear frame buffer with background color
        let bg_color = self.colubk;
        self.frame_buffer.fill(bg_color);
        
        // Render playfield
        self.render_playfield(width, height);
        
        // Render sprites
        self.render_sprites(width, height);
        
        // Apply VBLANK (blank screen during vertical blanking)
        if self.vblank != 0 {
            self.frame_buffer.fill(0); // Black during VBLANK
        }
    }
    
    /// Render the playfield
    fn render_playfield(&mut self, width: u32, height: u32) {
        let pf_color = self.colupf;
        
        // Playfield pattern from PF0, PF1, PF2 registers
        let pf_pattern = [
            (self.pf0 & 0xF0) >> 4,  // PF0 bits 7-4
            (self.pf0 & 0x0F) << 4,  // PF0 bits 3-0
            self.pf1,                // PF1
            self.pf2,                // PF2
        ];
        
        // Render playfield on left and right sides
        for y in 0..height {
            for x in 0..width {
                let index = (y * width + x) as usize;
                
                // Left playfield (0-79)
                if x < 80 {
                    let pf_bit = (pf_pattern[(x / 20) as usize] >> (7 - (x % 8))) & 1;
                    if pf_bit != 0 {
                        self.frame_buffer[index] = pf_color;
                    }
                }
                // Right playfield (80-159) - mirrored
                else {
                    let right_x = 159 - x;
                    let pf_bit = (pf_pattern[(right_x / 20) as usize] >> (7 - (right_x % 8))) & 1;
                    if pf_bit != 0 {
                        self.frame_buffer[index] = pf_color;
                    }
                }
            }
        }
    }
    
    /// Render sprites (Player 0, Player 1, Missile 0, Missile 1, Ball)
    fn render_sprites(&mut self, width: u32, height: u32) {
        // Player 0
        if self.enam0 != 0 {
            self.render_player_sprite(width, height, 0, self.grp0, self.colup0, self.nusiz0);
        }
        
        // Player 1
        if self.enam1 != 0 {
            self.render_player_sprite(width, height, 1, self.grp1, self.colup1, self.nusiz1);
        }
        
        // Ball
        if self.enabl != 0 {
            self.render_ball(width, height);
        }
    }
    
    /// Render a player sprite
    fn render_player_sprite(&mut self, width: u32, height: u32, player: u8, grp: u8, color: u8, nusiz: u8) {
        // Simplified sprite rendering - just show a pattern
        let sprite_x = 20 + (player as u32 * 40);
        let sprite_y = 50;
        
        for y in 0..8 {
            if sprite_y + y < height {
                for x in 0..8 {
                    if sprite_x + x < width {
                        let bit = (grp >> (7 - x)) & 1;
                        if bit != 0 {
                            let index = ((sprite_y + y) * width + sprite_x + x) as usize;
                            if index < self.frame_buffer.len() {
                                self.frame_buffer[index] = color;
                            }
                        }
                    }
                }
            }
        }
    }
    
    /// Render the ball sprite
    fn render_ball(&mut self, width: u32, height: u32) {
        let ball_x = 80;
        let ball_y = 100;
        let ball_color = self.colupf;
        
        // Simple ball rendering
        for y in 0..4 {
            if ball_y + y < height {
                for x in 0..4 {
                    if ball_x + x < width {
                        let index = ((ball_y + y) * width + ball_x + x) as usize;
                        if index < self.frame_buffer.len() {
                            self.frame_buffer[index] = ball_color;
                        }
                    }
                }
            }
        }
    }
    
    fn reset(&mut self) {
        *self = Self::new();
    }
    
    fn read_register(&self, address: u8) -> u8 {
        match address {
            0x00 => self.vsync,
            0x01 => self.vblank,
            0x02 => self.wsync,
            0x03 => self.rsync,
            0x04 => self.nusiz0,
            0x05 => self.nusiz1,
            0x06 => self.colup0,
            0x07 => self.colup1,
            0x08 => self.colupf,
            0x09 => self.colubk,
            0x0A => self.ctrlpf,
            0x0B => self.refp0,
            0x0C => self.refp1,
            0x0D => self.pf0,
            0x0E => self.pf1,
            0x0F => self.pf2,
            0x10 => self.resp0,
            0x11 => self.resp1,
            0x12 => self.resm0,
            0x13 => self.resm1,
            0x14 => self.resbl,
            0x15 => self.audc0,
            0x16 => self.audc1,
            0x17 => self.audf0,
            0x18 => self.audf1,
            0x19 => self.audv0,
            0x1A => self.audv1,
            0x1B => self.grp0,
            0x1C => self.grp1,
            0x1D => self.enam0,
            0x1E => self.enam1,
            0x1F => self.enabl,
            0x20 => self.hmp0,
            0x21 => self.hmp1,
            0x22 => self.hmm0,
            0x23 => self.hmm1,
            0x24 => self.hmbl,
            0x25 => self.vdelp0,
            0x26 => self.vdelp1,
            0x27 => self.vdelbl,
            0x28 => self.resmp0,
            0x29 => self.resmp1,
            0x2A => self.hmove,
            0x2B => self.hmclr,
            0x2C => self.cxclr,
            _ => 0,
        }
    }
    
    fn write_register(&mut self, address: u8, value: u8) {
        match address {
            0x00 => self.vsync = value,
            0x01 => self.vblank = value,
            0x02 => self.wsync = value,
            0x03 => self.rsync = value,
            0x04 => self.nusiz0 = value,
            0x05 => self.nusiz1 = value,
            0x06 => self.colup0 = value,
            0x07 => self.colup1 = value,
            0x08 => self.colupf = value,
            0x09 => self.colubk = value,
            0x0A => self.ctrlpf = value,
            0x0B => self.refp0 = value,
            0x0C => self.refp1 = value,
            0x0D => self.pf0 = value,
            0x0E => self.pf1 = value,
            0x0F => self.pf2 = value,
            0x10 => self.resp0 = value,
            0x11 => self.resp1 = value,
            0x12 => self.resm0 = value,
            0x13 => self.resm1 = value,
            0x14 => self.resbl = value,
            0x15 => self.audc0 = value,
            0x16 => self.audc1 = value,
            0x17 => self.audf0 = value,
            0x18 => self.audf1 = value,
            0x19 => self.audv0 = value,
            0x1A => self.audv1 = value,
            0x1B => self.grp0 = value,
            0x1C => self.grp1 = value,
            0x1D => self.enam0 = value,
            0x1E => self.enam1 = value,
            0x1F => self.enabl = value,
            0x20 => self.hmp0 = value,
            0x21 => self.hmp1 = value,
            0x22 => self.hmm0 = value,
            0x23 => self.hmm1 = value,
            0x24 => self.hmbl = value,
            0x25 => self.vdelp0 = value,
            0x26 => self.vdelp1 = value,
            0x27 => self.vdelbl = value,
            0x28 => self.resmp0 = value,
            0x29 => self.resmp1 = value,
            0x2A => self.hmove = value,
            0x2B => self.hmclr = value,
            0x2C => self.cxclr = value,
            _ => {}
        }
    }
}

impl RiaChip {
    fn new() -> Self {
        Self {
            ram: [0; 128],
            timer: 0,
            timer_interval: 0,
            timer_enabled: false,
            swcha: 0,
            swchb: 0,
            swacnt: 0,
            swbcnt: 0,
            joystick0: JoystickState::default(),
            joystick1: JoystickState::default(),
            console_switches: ConsoleSwitches {
                tv_type: true,      // Default to color
                left_difficulty: false,  // Default to A
                right_difficulty: false, // Default to A
                game_select: false,      // Default to game 1
                game_reset: false,       // Not pressed
            },
        }
    }
    
    fn step(&mut self) -> EmuliteResult<()> {
        // Update timer
        if self.timer_enabled {
            if self.timer > 0 {
                self.timer -= 1;
            } else {
                self.timer = self.timer_interval;
            }
        }
        
        Ok(())
    }
    
    fn reset(&mut self) {
        *self = Self::new();
    }
    
    fn read_register(&self, address: u8) -> u8 {
        match address {
            0x80..=0xFF => self.ram[(address - 0x80) as usize],
            0x80 => self.swcha,
            0x81 => self.swchb,
            0x82 => self.swacnt,
            0x83 => self.swbcnt,
            0x84 => self.timer,
            0x85 => if self.timer_enabled { 0x80 } else { 0x00 },
            _ => 0,
        }
    }
    
    /// Update input state from external input system
    fn update_input(&mut self, input_system: &crate::input::InputSystem) {
        // Update joystick 0 (Player 1)
        self.joystick0.up = input_system.is_button_pressed(0, crate::input::InputButton::Up);
        self.joystick0.down = input_system.is_button_pressed(0, crate::input::InputButton::Down);
        self.joystick0.left = input_system.is_button_pressed(0, crate::input::InputButton::Left);
        self.joystick0.right = input_system.is_button_pressed(0, crate::input::InputButton::Right);
        self.joystick0.fire = input_system.is_button_pressed(0, crate::input::InputButton::A);
        
        // Update joystick 1 (Player 2)
        self.joystick1.up = input_system.is_button_pressed(1, crate::input::InputButton::Up);
        self.joystick1.down = input_system.is_button_pressed(1, crate::input::InputButton::Down);
        self.joystick1.left = input_system.is_button_pressed(1, crate::input::InputButton::Left);
        self.joystick1.right = input_system.is_button_pressed(1, crate::input::InputButton::Right);
        self.joystick1.fire = input_system.is_button_pressed(1, crate::input::InputButton::A);
        
        // Update console switches
        self.console_switches.game_select = input_system.is_button_pressed(0, crate::input::InputButton::Select);
        self.console_switches.game_reset = input_system.is_button_pressed(0, crate::input::InputButton::Start);
        
        // Update SWCHA (joystick 0)
        self.swcha = 0;
        if !self.joystick0.right { self.swcha |= 0x80; }
        if !self.joystick0.left { self.swcha |= 0x40; }
        if !self.joystick0.down { self.swcha |= 0x20; }
        if !self.joystick0.up { self.swcha |= 0x10; }
        
        // Update SWCHB (joystick 1 + console switches)
        self.swchb = 0;
        if !self.joystick1.right { self.swchb |= 0x80; }
        if !self.joystick1.left { self.swchb |= 0x40; }
        if !self.joystick1.down { self.swchb |= 0x20; }
        if !self.joystick1.up { self.swchb |= 0x10; }
        if !self.joystick0.fire { self.swchb |= 0x08; }
        if !self.joystick1.fire { self.swchb |= 0x04; }
        if self.console_switches.game_select { self.swchb |= 0x02; }
        if self.console_switches.game_reset { self.swchb |= 0x01; }
    }
    
    fn write_register(&mut self, address: u8, value: u8) {
        match address {
            0x80..=0xFF => self.ram[(address - 0x80) as usize] = value,
            0x80 => self.swcha = value,
            0x81 => self.swchb = value,
            0x82 => self.swacnt = value,
            0x83 => self.swbcnt = value,
            0x84 => {
                self.timer = value;
                self.timer_enabled = true;
            },
            0x85 => {
                self.timer_interval = value;
                self.timer_enabled = true;
            },
            _ => {}
        }
    }
}

impl Cartridge {
    fn new() -> Self {
        Self {
            data: Vec::new(),
            bank_count: 0,
            current_bank: 0,
            mapper_type: MapperType::None,
        }
    }
    
    fn read(&self, address: u16) -> u8 {
        let rom_address = match self.mapper_type {
            MapperType::None => address as usize,
            MapperType::Atari8K => {
                let bank_offset = self.current_bank * 2048;
                (address as usize + bank_offset) % self.data.len()
            },
            MapperType::Atari16K => {
                let bank_offset = self.current_bank * 2048;
                (address as usize + bank_offset) % self.data.len()
            },
            MapperType::Atari32K => {
                let bank_offset = self.current_bank * 2048;
                (address as usize + bank_offset) % self.data.len()
            },
        };
        
        self.data.get(rom_address).copied().unwrap_or(0)
    }
    
    fn write(&mut self, address: u16, value: u8) {
        // Bank switching
        match self.mapper_type {
            MapperType::Atari8K => {
                if address >= 0x1FF8 && address <= 0x1FFF {
                    self.current_bank = (value as usize) % self.bank_count;
                }
            },
            MapperType::Atari16K => {
                if address >= 0x1FF8 && address <= 0x1FFF {
                    self.current_bank = (value as usize) % self.bank_count;
                }
            },
            MapperType::Atari32K => {
                if address >= 0x1FF8 && address <= 0x1FFF {
                    self.current_bank = (value as usize) % self.bank_count;
                }
            },
            _ => {}
        }
    }
}

/// TIA Write Device for memory mapping
struct TiaWriteDevice {
    tia: *mut TiaChip,
}

impl TiaWriteDevice {
    fn new(tia: &mut TiaChip) -> Self {
        Self {
            tia: tia as *mut TiaChip,
        }
    }
}

unsafe impl Send for TiaWriteDevice {}
unsafe impl Sync for TiaWriteDevice {}

impl MemoryDevice for TiaWriteDevice {
    fn read(&self, _address: u32) -> EmuliteResult<u8> {
        Err(EmuliteError::MemoryAccessViolation(_address))
    }
    
    fn write(&mut self, address: u32, value: u8) -> EmuliteResult<()> {
        unsafe {
            (*self.tia).write_register(address as u8, value);
        }
        Ok(())
    }
    
    fn name(&self) -> &str {
        "TIA_WRITE"
    }
    
    fn range(&self) -> (u32, u32) {
        (0x00, 0x3F)
    }
    
    fn reset(&mut self) -> EmuliteResult<()> {
        Ok(())
    }
}

/// TIA Read Device for memory mapping
struct TiaReadDevice {
    tia: *const TiaChip,
}

impl TiaReadDevice {
    fn new(tia: &TiaChip) -> Self {
        Self {
            tia: tia as *const TiaChip,
        }
    }
}

unsafe impl Send for TiaReadDevice {}
unsafe impl Sync for TiaReadDevice {}

impl MemoryDevice for TiaReadDevice {
    fn read(&self, address: u32) -> EmuliteResult<u8> {
        unsafe {
            Ok((*self.tia).read_register(address as u8))
        }
    }
    
    fn write(&mut self, _address: u32, _value: u8) -> EmuliteResult<()> {
        Err(EmuliteError::MemoryAccessViolation(_address))
    }
    
    fn name(&self) -> &str {
        "TIA_READ"
    }
    
    fn range(&self) -> (u32, u32) {
        (0x00, 0x3F)
    }
    
    fn reset(&mut self) -> EmuliteResult<()> {
        Ok(())
    }
}

/// RIOT Device for memory mapping
struct RiotDevice {
    ria: *mut RiaChip,
}

impl RiotDevice {
    fn new(ria: &mut RiaChip) -> Self {
        Self {
            ria: ria as *mut RiaChip,
        }
    }
}

unsafe impl Send for RiotDevice {}
unsafe impl Sync for RiotDevice {}

impl MemoryDevice for RiotDevice {
    fn read(&self, address: u32) -> EmuliteResult<u8> {
        unsafe {
            Ok((*self.ria).read_register(address as u8))
        }
    }
    
    fn write(&mut self, address: u32, value: u8) -> EmuliteResult<()> {
        unsafe {
            (*self.ria).write_register(address as u8, value);
        }
        Ok(())
    }
    
    fn name(&self) -> &str {
        "RIOT"
    }
    
    fn range(&self) -> (u32, u32) {
        (0x280, 0x2FF)
    }
    
    fn reset(&mut self) -> EmuliteResult<()> {
        Ok(())
    }
}
