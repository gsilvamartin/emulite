//! MOS 6502 CPU emulation (used in Atari 2600, NES, etc.)

use crate::cpu::{Cpu, CpuFlags, CpuInfo, Endianness};
use crate::{EmuliteResult, EmuliteError};
use crate::memory::MemoryMapper;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// MOS 6502 CPU implementation
pub struct Mos6502 {
    // Registers
    pub a: u8,      // Accumulator
    pub x: u8,      // X register
    pub y: u8,      // Y register
    pub sp: u8,     // Stack pointer
    pub pc: u16,    // Program counter
    pub flags: CpuFlags,
    
    // Memory interface
    memory: Vec<u8>,
    
    // CPU state
    cycles: u64,
    halted: bool,
    
    // Instruction decoding
    opcode: u8,
    operand: u16,
    addressing_mode: AddressingMode,
}

#[derive(Debug, Clone, Copy)]
enum AddressingMode {
    Implied,
    Immediate,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Indirect,
    IndirectX,
    IndirectY,
    Relative,
}

impl Mos6502 {
    pub fn new() -> EmuliteResult<Self> {
        Ok(Self {
            a: 0,
            x: 0,
            y: 0,
            sp: 0xFF,
            pc: 0xFFFC, // Reset vector
            flags: CpuFlags::default(),
            memory: vec![0; 0x10000], // 64KB memory
            cycles: 0,
            halted: false,
            opcode: 0,
            operand: 0,
            addressing_mode: AddressingMode::Implied,
        })
    }
    
    
    fn read_memory_16(&self, address: u16) -> u16 {
        let low = self.memory[address as usize];
        let high = self.memory[(address + 1) as usize];
        u16::from_le_bytes([low, high])
    }
    
    fn write_memory_16(&mut self, address: u16, value: u16) {
        let bytes = value.to_le_bytes();
        self.memory[address as usize] = bytes[0];
        self.memory[(address + 1) as usize] = bytes[1];
    }
    
    fn push_stack(&mut self, value: u8) {
        self.memory[0x100 + self.sp as usize] = value;
        self.sp = self.sp.wrapping_sub(1);
    }
    
    fn pop_stack(&mut self) -> u8 {
        self.sp = self.sp.wrapping_add(1);
        self.memory[0x100 + self.sp as usize]
    }
    
    fn push_stack_16(&mut self, value: u16) {
        let bytes = value.to_le_bytes();
        self.push_stack(bytes[1]);
        self.push_stack(bytes[0]);
    }
    
    fn pop_stack_16(&mut self) -> u16 {
        let low = self.pop_stack();
        let high = self.pop_stack();
        u16::from_le_bytes([low, high])
    }
    
    fn get_effective_address(&mut self) -> u16 {
        match self.addressing_mode {
            AddressingMode::Immediate => self.pc,
            AddressingMode::ZeroPage => self.operand as u16,
            AddressingMode::ZeroPageX => (self.operand as u8).wrapping_add(self.x) as u16,
            AddressingMode::ZeroPageY => (self.operand as u8).wrapping_add(self.y) as u16,
            AddressingMode::Absolute => self.operand,
            AddressingMode::AbsoluteX => self.operand.wrapping_add(self.x as u16),
            AddressingMode::AbsoluteY => self.operand.wrapping_add(self.y as u16),
            AddressingMode::Indirect => self.read_memory_16(self.operand),
            AddressingMode::IndirectX => {
                let addr = (self.operand as u8).wrapping_add(self.x) as u16;
                self.read_memory_16(addr)
            },
            AddressingMode::IndirectY => {
                let addr = self.read_memory_16(self.operand as u16);
                addr.wrapping_add(self.y as u16)
            },
            AddressingMode::Relative => self.pc.wrapping_add(self.operand as u16),
            AddressingMode::Implied => 0,
        }
    }
    
    fn update_flags_nz(&mut self, value: u8) {
        self.flags.zero = value == 0;
        self.flags.negative = (value & 0x80) != 0;
    }
    
    
    fn flags_to_byte(&self) -> u8 {
        let mut flags_byte = 0;
        if self.flags.carry { flags_byte |= 0x01; }
        if self.flags.zero { flags_byte |= 0x02; }
        if self.flags.interrupt_disable { flags_byte |= 0x04; }
        if self.flags.decimal { flags_byte |= 0x08; }
        if self.flags.break_command { flags_byte |= 0x10; }
        flags_byte |= 0x20; // Always set bit 5
        if self.flags.overflow { flags_byte |= 0x40; }
        if self.flags.negative { flags_byte |= 0x80; }
        flags_byte
    }
    
    fn byte_to_flags(&mut self, flags_byte: u8) {
        self.flags.carry = (flags_byte & 0x01) != 0;
        self.flags.zero = (flags_byte & 0x02) != 0;
        self.flags.interrupt_disable = (flags_byte & 0x04) != 0;
        self.flags.decimal = (flags_byte & 0x08) != 0;
        self.flags.break_command = (flags_byte & 0x10) != 0;
        self.flags.overflow = (flags_byte & 0x40) != 0;
        self.flags.negative = (flags_byte & 0x80) != 0;
    }
    
    
    fn update_zero_negative_flags(&mut self, value: u8) {
        self.flags.zero = value == 0;
        self.flags.negative = (value & 0x80) != 0;
    }
    
    fn execute_instruction(&mut self) -> EmuliteResult<()> {
        match self.opcode {
            // LDA (Load Accumulator)
            0xA9 => { // LDA #immediate
                let value = self.read_memory(self.pc as u32)?;
                self.pc = self.pc.wrapping_add(1);
                self.a = value;
                self.update_zero_negative_flags(self.a);
            },
            0xA5 => { // LDA zero page
                let addr = self.read_memory(self.pc as u32)? as u16;
                self.pc = self.pc.wrapping_add(1);
                self.a = self.read_memory(addr as u32)?;
                self.update_zero_negative_flags(self.a);
            },
            0xAD => { // LDA absolute
                let addr = self.read_memory_16(self.pc);
                self.pc = self.pc.wrapping_add(2);
                self.a = self.read_memory(addr as u32)?;
                self.update_zero_negative_flags(self.a);
            },
            
            // STA (Store Accumulator)
            0x85 => { // STA zero page
                let addr = self.read_memory(self.pc as u32)? as u16;
                self.pc = self.pc.wrapping_add(1);
                self.write_memory(addr as u32, self.a)?;
            },
            0x8D => { // STA absolute
                let addr = self.read_memory_16(self.pc);
                self.pc = self.pc.wrapping_add(2);
                self.write_memory(addr as u32, self.a)?;
            },
            
            // JMP (Jump)
            0x4C => { // JMP absolute
                let addr = self.read_memory_16(self.pc);
                self.pc = self.pc.wrapping_add(2);
                self.pc = addr;
            },
            
            // NOP (No Operation)
            0xEA => {
                // Do nothing
            },
            
            // BRK (Break)
            0x00 => {
                self.halted = true;
            },
            
            // Default case - log unknown opcode
            _ => {
                log::warn!("Unknown opcode: 0x{:02X} at PC: 0x{:04X}", self.opcode, self.pc - 1);
            }
        }
        
        Ok(())
    }
}

impl Cpu for Mos6502 {
    fn step(&mut self) -> EmuliteResult<()> {
        if self.halted {
            return Ok(());
        }
        
        // Fetch instruction
        self.opcode = self.read_memory(self.pc as u32)?;
        self.pc = self.pc.wrapping_add(1);
        
        // Execute instruction based on opcode
        self.execute_instruction()?;
        
        Ok(())
    }
    
    
    
    fn reset(&mut self) -> EmuliteResult<()> {
        self.a = 0;
        self.x = 0;
        self.y = 0;
        self.sp = 0xFF;
        self.pc = self.read_memory_16(0xFFFC); // Reset vector
        self.flags = CpuFlags::default();
        self.flags.interrupt_disable = true;
        self.cycles = 0;
        self.halted = false;
        Ok(())
    }
    
    fn name(&self) -> &str {
        "MOS 6502"
    }
    
    fn pc(&self) -> u32 {
        self.pc as u32
    }
    
    fn set_pc(&mut self, pc: u32) -> EmuliteResult<()> {
        self.pc = pc as u16;
        Ok(())
    }
    
    fn read_memory(&self, address: u32) -> EmuliteResult<u8> {
        if address >= 0x10000 {
            // Return 0 for unmapped memory instead of error
            return Ok(0);
        }
        
        // Atari 2600 memory mapping
        match address {
            // RAM (128 bytes, mirrored)
            0x0000..=0x007F => {
                // TIA registers (write-only) - return 0 for reads
                Ok(0)
            },
            0x0080..=0x00FF => {
                // RAM
                Ok(self.memory[address as usize])
            },
            0x0100..=0x01FF => {
                // Stack (RAM mirror)
                Ok(self.memory[(address - 0x0100) as usize])
            },
            0x0200..=0x027F => {
                // TIA registers (read-only)
                Ok(0) // TODO: Implement TIA register reads
            },
            0x0280..=0x02FF => {
                // RIOT registers
                Ok(0) // TODO: Implement RIOT register reads
            },
            0x1000..=0x1FFF => {
                // ROM
                Ok(self.memory[address as usize])
            },
            0xF000..=0xFFFF => {
                // ROM mirror
                Ok(self.memory[address as usize])
            },
            _ => {
                // Unmapped memory
                Ok(0)
            }
        }
    }
    
    fn write_memory(&mut self, address: u32, value: u8) -> EmuliteResult<()> {
        if address >= 0x10000 {
            // Ignore writes to unmapped memory instead of error
            return Ok(());
        }
        
        // Atari 2600 memory mapping
        match address {
            // RAM (128 bytes, mirrored)
            0x0000..=0x007F => {
                // TIA registers (write-only)
                // TODO: Implement TIA register writes
                Ok(())
            },
            0x0080..=0x00FF => {
                // RAM
                self.memory[address as usize] = value;
                Ok(())
            },
            0x0100..=0x01FF => {
                // Stack (RAM mirror)
                self.memory[(address - 0x0100) as usize] = value;
                Ok(())
            },
            0x0200..=0x027F => {
                // TIA registers (read-only) - ignore writes
                Ok(())
            },
            0x0280..=0x02FF => {
                // RIOT registers
                // TODO: Implement RIOT register writes
                Ok(())
            },
            0x1000..=0x1FFF => {
                // ROM - ignore writes
                Ok(())
            },
            0xF000..=0xFFFF => {
                // ROM mirror - ignore writes
                Ok(())
            },
            _ => {
                // Unmapped memory - ignore writes
                Ok(())
            }
        }
    }
    
    fn get_register(&self, reg: &str) -> EmuliteResult<u32> {
        match reg.to_lowercase().as_str() {
            "a" => Ok(self.a as u32),
            "x" => Ok(self.x as u32),
            "y" => Ok(self.y as u32),
            "sp" => Ok(self.sp as u32),
            "pc" => Ok(self.pc as u32),
            _ => Err(EmuliteError::CpuError(format!("Unknown register: {}", reg))),
        }
    }
    
    fn set_register(&mut self, reg: &str, value: u32) -> EmuliteResult<()> {
        match reg.to_lowercase().as_str() {
            "a" => self.a = value as u8,
            "x" => self.x = value as u8,
            "y" => self.y = value as u8,
            "sp" => self.sp = value as u8,
            "pc" => self.pc = value as u16,
            _ => return Err(EmuliteError::CpuError(format!("Unknown register: {}", reg))),
        }
        Ok(())
    }
    
    fn get_registers(&self) -> HashMap<String, u32> {
        let mut registers = HashMap::new();
        registers.insert("a".to_string(), self.a as u32);
        registers.insert("x".to_string(), self.x as u32);
        registers.insert("y".to_string(), self.y as u32);
        registers.insert("sp".to_string(), self.sp as u32);
        registers.insert("pc".to_string(), self.pc as u32);
        registers
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
            name: "MOS 6502".to_string(),
            architecture: "6502".to_string(),
            bits: 8,
            endianness: Endianness::Little,
            register_count: 5,
            instruction_count: 56,
            clock_speed_hz: 1_789_773, // NTSC frequency
        }
    }
    
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

