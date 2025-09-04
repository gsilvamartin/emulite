//! CPU emulation system with support for multiple architectures

use crate::{EmuliteResult, EmuliteError};
use std::collections::HashMap;

pub mod generic;
pub mod mos6502;
pub mod m68k;
pub mod mips;
pub mod x86;
pub mod arm;

/// CPU trait that all CPU implementations must implement
pub trait Cpu: Send + Sync {
    /// Execute one instruction
    fn step(&mut self) -> EmuliteResult<()>;
    
    /// Reset CPU to initial state
    fn reset(&mut self) -> EmuliteResult<()>;
    
    /// Get CPU name/type
    fn name(&self) -> &str;
    
    /// Get current program counter
    fn pc(&self) -> u32;
    
    /// Set program counter
    fn set_pc(&mut self, pc: u32) -> EmuliteResult<()>;
    
    /// Read from memory
    fn read_memory(&self, address: u32) -> EmuliteResult<u8>;
    
    /// Write to memory
    fn write_memory(&mut self, address: u32, value: u8) -> EmuliteResult<()>;
    
    /// Get register value
    fn get_register(&self, reg: &str) -> EmuliteResult<u32>;
    
    /// Set register value
    fn set_register(&mut self, reg: &str, value: u32) -> EmuliteResult<()>;
    
    /// Get all registers
    fn get_registers(&self) -> HashMap<String, u32>;
    
    /// Check if CPU is halted
    fn is_halted(&self) -> bool;
    
    /// Get CPU flags/status
    fn get_flags(&self) -> CpuFlags;
    
    /// Set CPU flags/status
    fn set_flags(&mut self, flags: CpuFlags) -> EmuliteResult<()>;
    
    /// Get CPU info
    fn info(&self) -> CpuInfo;
    
    /// Get CPU as Any for downcasting
    fn as_any(&self) -> &dyn std::any::Any;
    
    /// Get CPU as Any for downcasting (mutable)
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
}

/// CPU flags/status register
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CpuFlags {
    pub carry: bool,
    pub zero: bool,
    pub interrupt_disable: bool,
    pub decimal: bool,
    pub break_command: bool,
    pub overflow: bool,
    pub negative: bool,
    pub sign: bool,
    pub parity: bool,
    pub auxiliary_carry: bool,
    pub extend: bool,
}

impl Default for CpuFlags {
    fn default() -> Self {
        Self {
            carry: false,
            zero: false,
            interrupt_disable: false,
            decimal: false,
            break_command: false,
            overflow: false,
            negative: false,
            sign: false,
            parity: false,
            auxiliary_carry: false,
            extend: false,
        }
    }
}

/// CPU information structure
#[derive(Debug, Clone)]
pub struct CpuInfo {
    pub name: String,
    pub architecture: String,
    pub bits: u8,
    pub endianness: Endianness,
    pub register_count: usize,
    pub instruction_count: usize,
    pub clock_speed_hz: u64,
}

/// Endianness enumeration
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Endianness {
    Little,
    Big,
}

/// CPU factory for creating CPU instances
pub struct CpuFactory;

impl CpuFactory {
    /// Create a CPU instance by type
    pub fn create(cpu_type: &str) -> EmuliteResult<Box<dyn Cpu>> {
        match cpu_type.to_lowercase().as_str() {
            "6502" | "mos6502" => Ok(Box::new(mos6502::Mos6502::new()?)),
            "68000" | "m68k" => Ok(Box::new(m68k::M68k::new()?)),
            "mips" => Ok(Box::new(mips::Mips::new()?)),
            "x86" | "i386" => Ok(Box::new(x86::X86::new()?)),
            "arm" => Ok(Box::new(arm::Arm::new()?)),
            _ => Err(EmuliteError::CpuError(
                format!("Unsupported CPU type: {}", cpu_type)
            )),
        }
    }
    
    /// Get list of supported CPU types
    pub fn supported_cpus() -> Vec<&'static str> {
        vec![
            "6502",
            "68000", 
            "mips",
            "x86",
            "arm",
        ]
    }
}

/// Generic CPU implementation for testing and fallback
pub struct GenericCpu {
    pc: u32,
    registers: HashMap<String, u32>,
    flags: CpuFlags,
    memory: Vec<u8>,
    halted: bool,
}

impl GenericCpu {
    pub fn new(memory_size: usize) -> Self {
        Self {
            pc: 0,
            registers: HashMap::new(),
            flags: CpuFlags::default(),
            memory: vec![0; memory_size],
            halted: false,
        }
    }
}

impl Cpu for GenericCpu {
    fn step(&mut self) -> EmuliteResult<()> {
        if self.halted {
            return Ok(());
        }
        
        // Simple NOP implementation
        self.pc = self.pc.wrapping_add(1);
        Ok(())
    }
    
    fn reset(&mut self) -> EmuliteResult<()> {
        self.pc = 0;
        self.registers.clear();
        self.flags = CpuFlags::default();
        self.memory.fill(0);
        self.halted = false;
        Ok(())
    }
    
    fn name(&self) -> &str {
        "Generic"
    }
    
    fn pc(&self) -> u32 {
        self.pc
    }
    
    fn set_pc(&mut self, pc: u32) -> EmuliteResult<()> {
        self.pc = pc;
        Ok(())
    }
    
    fn read_memory(&self, address: u32) -> EmuliteResult<u8> {
        if address as usize >= self.memory.len() {
            return Err(EmuliteError::MemoryAccessViolation(address));
        }
        Ok(self.memory[address as usize])
    }
    
    fn write_memory(&mut self, address: u32, value: u8) -> EmuliteResult<()> {
        if address as usize >= self.memory.len() {
            return Err(EmuliteError::MemoryAccessViolation(address));
        }
        self.memory[address as usize] = value;
        Ok(())
    }
    
    fn get_register(&self, reg: &str) -> EmuliteResult<u32> {
        self.registers.get(reg).copied().ok_or_else(|| {
            EmuliteError::CpuError(format!("Register '{}' not found", reg))
        })
    }
    
    fn set_register(&mut self, reg: &str, value: u32) -> EmuliteResult<()> {
        self.registers.insert(reg.to_string(), value);
        Ok(())
    }
    
    fn get_registers(&self) -> HashMap<String, u32> {
        self.registers.clone()
    }
    
    fn is_halted(&self) -> bool {
        self.halted
    }
    
    fn get_flags(&self) -> CpuFlags {
        self.flags
    }
    
    fn set_flags(&mut self, flags: CpuFlags) -> EmuliteResult<()> {
        self.flags = flags;
        Ok(())
    }
    
    fn info(&self) -> CpuInfo {
        CpuInfo {
            name: "Generic CPU".to_string(),
            architecture: "Generic".to_string(),
            bits: 32,
            endianness: Endianness::Little,
            register_count: self.registers.len(),
            instruction_count: 1, // Only NOP
            clock_speed_hz: 1_000_000,
        }
    }
    
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
