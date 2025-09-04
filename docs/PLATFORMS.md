# Emulite Supported Platforms

Este documento descreve as plataformas suportadas pelo Emulite e suas características específicas.

## Visão Geral

O Emulite suporta uma ampla gama de plataformas de videogames, desde os primeiros sistemas da Atari até consoles modernos como o PlayStation 3. Cada plataforma é implementada como um módulo separado com suas próprias características e otimizações.

## Plataformas Suportadas

### 1. Atari 2600 (1977)

**Características:**
- CPU: MOS 6507 (6502 modificado)
- Memória: 128 bytes RAM, 4KB ROM
- Resolução: 160x192 pixels
- Cores: 128 cores disponíveis
- Áudio: 2 canais mono

**Implementação:**
- CPU: `src/cpu/mos6502.rs`
- Plataforma: `src/platforms/atari2600.rs`
- Características especiais: TIA (Television Interface Adapter)

**Exemplo de uso:**
```rust
let platform = PlatformFactory::create("atari2600")?;
let mut emulator = Emulator::new(platform, config);
emulator.load_rom(&rom_data)?;
emulator.run()?;
```

### 2. Nintendo Entertainment System (NES) (1983)

**Características:**
- CPU: Ricoh 2A03 (6502 modificado)
- Memória: 2KB RAM, 32KB ROM
- Resolução: 256x240 pixels
- Cores: 64 cores disponíveis
- Áudio: 5 canais (2 pulse, 1 triangle, 1 noise, 1 DMC)

**Implementação:**
- CPU: `src/cpu/mos6502.rs`
- Plataforma: `src/platforms/nes.rs`
- Características especiais: PPU (Picture Processing Unit), APU (Audio Processing Unit)

**Exemplo de uso:**
```rust
let platform = PlatformFactory::create("nes")?;
let mut emulator = Emulator::new(platform, config);
emulator.load_rom(&rom_data)?;
emulator.run()?;
```

### 3. Super Nintendo Entertainment System (SNES) (1990)

**Características:**
- CPU: Ricoh 5A22 (65816 modificado)
- Memória: 128KB RAM, 4MB ROM
- Resolução: 256x224 pixels (modo 7: 512x448)
- Cores: 32,768 cores disponíveis
- Áudio: 8 canais (4 ADPCM, 4 PCM)

**Implementação:**
- CPU: `src/cpu/mos6502.rs` (65816)
- Plataforma: `src/platforms/snes.rs`
- Características especiais: PPU avançado, Modo 7, Super FX

**Exemplo de uso:**
```rust
let platform = PlatformFactory::create("snes")?;
let mut emulator = Emulator::new(platform, config);
emulator.load_rom(&rom_data)?;
emulator.run()?;
```

### 4. PlayStation (PS1) (1994)

**Características:**
- CPU: MIPS R3000A (32-bit RISC)
- Memória: 2MB RAM, 512KB VRAM
- Resolução: 320x240 pixels (até 640x480)
- Cores: 16,777,216 cores disponíveis
- Áudio: 24 canais ADPCM

**Implementação:**
- CPU: `src/cpu/mips.rs`
- Plataforma: `src/platforms/ps1.rs`
- Características especiais: GPU, SPU, CD-ROM

**Exemplo de uso:**
```rust
let platform = PlatformFactory::create("ps1")?;
let mut emulator = Emulator::new(platform, config);
emulator.load_rom(&rom_data)?;
emulator.run()?;
```

### 5. PlayStation 2 (PS2) (2000)

**Características:**
- CPU: MIPS R5900 (64-bit RISC)
- Memória: 32MB RAM, 4MB VRAM
- Resolução: 640x480 pixels (até 1920x1080)
- Cores: 16,777,216 cores disponíveis
- Áudio: 48 canais ADPCM

**Implementação:**
- CPU: `src/cpu/mips.rs`
- Plataforma: `src/platforms/ps2.rs`
- Características especiais: EE (Emotion Engine), GS (Graphics Synthesizer), IOP

**Exemplo de uso:**
```rust
let platform = PlatformFactory::create("ps2")?;
let mut emulator = Emulator::new(platform, config);
emulator.load_rom(&rom_data)?;
emulator.run()?;
```

### 6. PlayStation 3 (PS3) (2006)

**Características:**
- CPU: PowerPC Cell (64-bit)
- Memória: 256MB RAM, 256MB VRAM
- Resolução: 720p/1080p
- Cores: 16,777,216 cores disponíveis
- Áudio: 7.1 surround

**Implementação:**
- CPU: `src/cpu/arm.rs` (PowerPC)
- Plataforma: `src/platforms/ps3.rs`
- Características especiais: Cell Broadband Engine, RSX GPU

**Exemplo de uso:**
```rust
let platform = PlatformFactory::create("ps3")?;
let mut emulator = Emulator::new(platform, config);
emulator.load_rom(&rom_data)?;
emulator.run()?;
```

## Características por Plataforma

### CPU

| Plataforma | CPU | Arquitetura | Frequência | Registros |
|------------|-----|-------------|------------|-----------|
| Atari 2600 | MOS 6507 | 8-bit | 1.19 MHz | A, X, Y, SP, PC |
| NES | Ricoh 2A03 | 8-bit | 1.79 MHz | A, X, Y, SP, PC |
| SNES | Ricoh 5A22 | 16-bit | 3.58 MHz | A, X, Y, SP, PC, DBR |
| PS1 | MIPS R3000A | 32-bit | 33.87 MHz | 32 registros |
| PS2 | MIPS R5900 | 64-bit | 294.91 MHz | 32 registros |
| PS3 | PowerPC Cell | 64-bit | 3.2 GHz | 32 registros |

### Memória

| Plataforma | RAM | ROM | VRAM | Características |
|------------|-----|-----|------|-----------------|
| Atari 2600 | 128 bytes | 4KB | - | TIA |
| NES | 2KB | 32KB | 2KB | PPU |
| SNES | 128KB | 4MB | 64KB | PPU avançado |
| PS1 | 2MB | 512KB | 1MB | GPU |
| PS2 | 32MB | 4MB | 4MB | GS |
| PS3 | 256MB | 256MB | 256MB | RSX |

### Áudio

| Plataforma | Canais | Formato | Frequência | Características |
|------------|--------|---------|------------|-----------------|
| Atari 2600 | 2 | Mono | 31.4 kHz | TIA |
| NES | 5 | Mono | 44.1 kHz | APU |
| SNES | 8 | Stereo | 32 kHz | SPC700 |
| PS1 | 24 | Stereo | 44.1 kHz | SPU |
| PS2 | 48 | Stereo | 48 kHz | SPU2 |
| PS3 | 7.1 | Surround | 48 kHz | RSX |

### Vídeo

| Plataforma | Resolução | Cores | Características |
|------------|-----------|-------|-----------------|
| Atari 2600 | 160x192 | 128 | TIA |
| NES | 256x240 | 64 | PPU |
| SNES | 256x224 | 32,768 | PPU avançado, Modo 7 |
| PS1 | 320x240 | 16M | GPU |
| PS2 | 640x480 | 16M | GS |
| PS3 | 720p/1080p | 16M | RSX |

## Implementação de Novas Plataformas

Para adicionar uma nova plataforma ao Emulite:

### 1. Criar o Arquivo da Plataforma

```rust
// src/platforms/nova_plataforma.rs
use crate::core::*;
use crate::cpu::*;
use crate::memory::*;
use crate::audio::*;
use crate::video::*;
use crate::input::*;

pub struct NovaPlataforma {
    cpu: Box<dyn Cpu>,
    memory: Box<dyn MemoryBus>,
    ppu: Box<dyn Ppu>,
    apu: Box<dyn Apu>,
    input: Box<dyn InputDevice>,
}

impl Platform for NovaPlataforma {
    fn name(&self) -> &str {
        "nova_plataforma"
    }
    
    fn version(&self) -> &str {
        "1.0.0"
    }
    
    fn cpu(&self) -> &dyn Cpu {
        self.cpu.as_ref()
    }
    
    fn memory(&self) -> &dyn MemoryBus {
        self.memory.as_ref()
    }
    
    fn ppu(&self) -> &dyn Ppu {
        self.ppu.as_ref()
    }
    
    fn apu(&self) -> &dyn Apu {
        self.apu.as_ref()
    }
    
    fn input(&self) -> &dyn InputDevice {
        self.input.as_ref()
    }
    
    fn step(&mut self) -> EmuliteResult<()> {
        // Implementar lógica específica da plataforma
        Ok(())
    }
    
    fn reset(&mut self) -> EmuliteResult<()> {
        // Implementar reset específico da plataforma
        Ok(())
    }
}
```

### 2. Adicionar ao Factory

```rust
// src/platforms/mod.rs
pub mod nova_plataforma;

use nova_plataforma::NovaPlataforma;

impl PlatformFactory {
    pub fn create(platform_name: &str) -> EmuliteResult<Box<dyn Platform>> {
        match platform_name {
            "nova_plataforma" => Ok(Box::new(NovaPlataforma::new()?)),
            // ... outras plataformas
            _ => Err(EmuliteError::PlatformNotSupported(platform_name.to_string())),
        }
    }
}
```

### 3. Implementar Componentes Específicos

```rust
// src/cpu/nova_cpu.rs
use crate::core::*;

pub struct NovaCpu {
    // Implementar CPU específica
}

impl Cpu for NovaCpu {
    fn step(&mut self) -> EmuliteResult<()> {
        // Implementar execução de instrução
        Ok(())
    }
    
    fn reset(&mut self) -> EmuliteResult<()> {
        // Implementar reset da CPU
        Ok(())
    }
    
    // ... implementar outros métodos
}
```

## Conclusão

O Emulite suporta uma ampla gama de plataformas de videogames, cada uma com suas próprias características e otimizações. A arquitetura modular permite fácil adição de novas plataformas e componentes específicos.
