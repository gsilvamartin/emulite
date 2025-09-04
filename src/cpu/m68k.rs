//! Motorola 68000 CPU emulation (used in Sega Genesis, etc.)

use crate::cpu::{Cpu, CpuFlags, CpuInfo, Endianness};
use crate::{EmuliteResult, EmuliteError};
use std::collections::HashMap;

/// Motorola 68000 CPU implementation
pub struct M68k {
    // Data registers (D0-D7)
    pub d: [u32; 8],
    
    // Address registers (A0-A7)
    pub a: [u32; 8],
    
    // Program counter
    pub pc: u32,
    
    // Status register
    pub sr: u16,
    
    // Memory interface
    memory: Vec<u8>,
    
    // CPU state
    cycles: u64,
    halted: bool,
    
    // Instruction decoding
    opcode: u16,
    operand: u32,
    addressing_mode: AddressingMode,
}

#[derive(Debug, Clone, Copy)]
enum AddressingMode {
    DataRegister,
    AddressRegister,
    AddressRegisterIndirect,
    AddressRegisterPostIncrement,
    AddressRegisterPreDecrement,
    AddressRegisterDisplacement,
    AddressRegisterIndexed,
    AbsoluteShort,
    AbsoluteLong,
    ProgramCounterDisplacement,
    ProgramCounterIndexed,
    Immediate,
    Quick,
}

impl M68k {
    pub fn new() -> EmuliteResult<Self> {
        Ok(Self {
            d: [0; 8],
            a: [0; 8],
            pc: 0,
            sr: 0x2000, // Supervisor mode, interrupts enabled
            memory: vec![0; 0x1000000], // 16MB address space
            cycles: 0,
            halted: false,
            opcode: 0,
            operand: 0,
            addressing_mode: AddressingMode::DataRegister,
        })
    }
    
    fn read_memory_16(&self, address: u32) -> u16 {
        let low = self.memory[address as usize];
        let high = self.memory[(address + 1) as usize];
        u16::from_be_bytes([high, low]) // Big-endian
    }
    
    fn write_memory_16(&mut self, address: u32, value: u16) {
        let bytes = value.to_be_bytes();
        self.memory[address as usize] = bytes[1];
        self.memory[(address + 1) as usize] = bytes[0];
    }
    
    fn read_memory_32(&self, address: u32) -> u32 {
        let b0 = self.memory[address as usize];
        let b1 = self.memory[(address + 1) as usize];
        let b2 = self.memory[(address + 2) as usize];
        let b3 = self.memory[(address + 3) as usize];
        u32::from_be_bytes([b3, b2, b1, b0]) // Big-endian
    }
    
    fn write_memory_32(&mut self, address: u32, value: u32) {
        let bytes = value.to_be_bytes();
        self.memory[address as usize] = bytes[3];
        self.memory[(address + 1) as usize] = bytes[2];
        self.memory[(address + 2) as usize] = bytes[1];
        self.memory[(address + 3) as usize] = bytes[0];
    }
    
    fn get_effective_address(&mut self) -> u32 {
        match self.addressing_mode {
            AddressingMode::DataRegister => self.d[0], // Simplified
            AddressingMode::AddressRegister => self.a[0], // Simplified
            AddressingMode::AddressRegisterIndirect => self.a[0], // Simplified
            AddressingMode::AddressRegisterPostIncrement => {
                let addr = self.a[0];
                self.a[0] += 1;
                addr
            },
            AddressingMode::AddressRegisterPreDecrement => {
                self.a[0] -= 1;
                self.a[0]
            },
            AddressingMode::AddressRegisterDisplacement => {
                self.a[0] + self.operand
            },
            AddressingMode::AddressRegisterIndexed => {
                self.a[0] + self.operand
            },
            AddressingMode::AbsoluteShort => self.operand as u16 as u32,
            AddressingMode::AbsoluteLong => self.operand,
            AddressingMode::ProgramCounterDisplacement => {
                self.pc + self.operand
            },
            AddressingMode::ProgramCounterIndexed => {
                self.pc + self.operand
            },
            AddressingMode::Immediate => self.operand,
            AddressingMode::Quick => self.operand,
        }
    }
    
    fn update_flags_nz(&mut self, value: u32, size: u8) {
        let mask = match size {
            8 => 0x80,
            16 => 0x8000,
            32 => 0x80000000,
            _ => 0x80000000,
        };
        
        self.set_flag(1, value == 0); // Zero flag
        self.set_flag(3, (value & mask) != 0); // Negative flag
    }
    
    fn set_flag(&mut self, bit: u8, value: bool) {
        if value {
            self.sr |= 1 << bit;
        } else {
            self.sr &= !(1 << bit);
        }
    }
    
    fn get_flag(&self, bit: u8) -> bool {
        (self.sr & (1 << bit)) != 0
    }
    
    fn execute_instruction(&mut self) -> EmuliteResult<()> {
        match self.opcode {
            // MOVE instruction
            0x2000..=0x2FFF => { // MOVE.B
                let source = self.get_effective_address();
                let dest = self.get_effective_address();
                let value = self.memory[source as usize];
                self.memory[dest as usize] = value;
                self.update_flags_nz(value as u32, 8);
            },
            0x3000..=0x3FFF => { // MOVE.W
                let source = self.get_effective_address();
                let dest = self.get_effective_address();
                let value = self.read_memory_16(source);
                self.write_memory_16(dest, value);
                self.update_flags_nz(value as u32, 16);
            },
            0x2000..=0x2FFF => { // MOVE.L
                let source = self.get_effective_address();
                let dest = self.get_effective_address();
                let value = self.read_memory_32(source);
                self.write_memory_32(dest, value);
                self.update_flags_nz(value, 32);
            },
            
            // ADD instruction
            0xD000..=0xDFFF => { // ADD.B
                let source = self.get_effective_address();
                let dest = self.get_effective_address();
                let src_value = self.memory[source as usize] as u8;
                let dest_value = self.memory[dest as usize] as u8;
                let result = src_value.wrapping_add(dest_value);
                self.memory[dest as usize] = result;
                self.update_flags_nz(result as u32, 8);
            },
            
            // SUB instruction
            0x9000..=0x9FFF => { // SUB.B
                let source = self.get_effective_address();
                let dest = self.get_effective_address();
                let src_value = self.memory[source as usize] as u8;
                let dest_value = self.memory[dest as usize] as u8;
                let result = dest_value.wrapping_sub(src_value);
                self.memory[dest as usize] = result;
                self.update_flags_nz(result as u32, 8);
            },
            
            // JMP instruction
            0x4EC0..=0x4EFF => { // JMP
                let address = self.get_effective_address();
                self.pc = address;
            },
            
            // JSR instruction
            0x4E80..=0x4EBF => { // JSR
                let address = self.get_effective_address();
                self.a[7] -= 4; // Push return address
                self.write_memory_32(self.a[7], self.pc);
                self.pc = address;
            },
            
            // RTS instruction
            0x4E75 => { // RTS
                self.pc = self.read_memory_32(self.a[7]);
                self.a[7] += 4; // Pop return address
            },
            
            // NOP instruction
            0x4E71 => { // NOP
                // Do nothing
            },
            
            // RTE instruction
            0x4E73 => { // RTE
                self.sr = self.read_memory_16(self.a[7]);
                self.a[7] += 2;
                self.pc = self.read_memory_32(self.a[7]);
                self.a[7] += 4;
            },
            
            _ => {
                return Err(EmuliteError::CpuError(
                    format!("Unimplemented opcode: 0x{:04X}", self.opcode)
                ));
            }
        }
        
        Ok(())
    }
}

impl Cpu for M68k {
    fn step(&mut self) -> EmuliteResult<()> {
        if self.halted {
            return Ok(());
        }
        
        // Fetch instruction
        self.opcode = self.read_memory_16(self.pc);
        self.pc += 2;
        
        // Decode addressing mode and fetch operands
        match self.opcode {
            0x2000..=0x2FFF | 0x3000..=0x3FFF | 0x2000..=0x2FFF => { // MOVE instructions
                // Simplified addressing mode decoding
                self.addressing_mode = AddressingMode::DataRegister;
                self.operand = 0;
            },
            0xD000..=0xDFFF | 0x9000..=0x9FFF => { // ADD/SUB instructions
                self.addressing_mode = AddressingMode::DataRegister;
                self.operand = 0;
            },
            0x4EC0..=0x4EFF | 0x4E80..=0x4EBF => { // JMP/JSR instructions
                self.addressing_mode = AddressingMode::AbsoluteLong;
                self.operand = self.read_memory_32(self.pc);
                self.pc += 4;
            },
            _ => {
                self.addressing_mode = AddressingMode::DataRegister;
                self.operand = 0;
            }
        }
        
        // Execute instruction
        self.execute_instruction()?;
        
        self.cycles += 1;
        Ok(())
    }
    
    fn reset(&mut self) -> EmuliteResult<()> {
        self.d.fill(0);
        self.a.fill(0);
        self.pc = self.read_memory_32(0); // Reset vector
        self.sr = 0x2000; // Supervisor mode, interrupts enabled
        self.memory.fill(0);
        self.cycles = 0;
        self.halted = false;
        Ok(())
    }
    
    fn name(&self) -> &str {
        "Motorola 68000"
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
        match reg.to_lowercase().as_str() {
            "d0" => Ok(self.d[0]),
            "d1" => Ok(self.d[1]),
            "d2" => Ok(self.d[2]),
            "d3" => Ok(self.d[3]),
            "d4" => Ok(self.d[4]),
            "d5" => Ok(self.d[5]),
            "d6" => Ok(self.d[6]),
            "d7" => Ok(self.d[7]),
            "a0" => Ok(self.a[0]),
            "a1" => Ok(self.a[1]),
            "a2" => Ok(self.a[2]),
            "a3" => Ok(self.a[3]),
            "a4" => Ok(self.a[4]),
            "a5" => Ok(self.a[5]),
            "a6" => Ok(self.a[6]),
            "a7" => Ok(self.a[7]),
            "pc" => Ok(self.pc),
            "sr" => Ok(self.sr as u32),
            _ => Err(EmuliteError::CpuError(format!("Unknown register: {}", reg))),
        }
    }
    
    fn set_register(&mut self, reg: &str, value: u32) -> EmuliteResult<()> {
        match reg.to_lowercase().as_str() {
            "d0" => self.d[0] = value,
            "d1" => self.d[1] = value,
            "d2" => self.d[2] = value,
            "d3" => self.d[3] = value,
            "d4" => self.d[4] = value,
            "d5" => self.d[5] = value,
            "d6" => self.d[6] = value,
            "d7" => self.d[7] = value,
            "a0" => self.a[0] = value,
            "a1" => self.a[1] = value,
            "a2" => self.a[2] = value,
            "a3" => self.a[3] = value,
            "a4" => self.a[4] = value,
            "a5" => self.a[5] = value,
            "a6" => self.a[6] = value,
            "a7" => self.a[7] = value,
            "pc" => self.pc = value,
            "sr" => self.sr = value as u16,
            _ => return Err(EmuliteError::CpuError(format!("Unknown register: {}", reg))),
        }
        Ok(())
    }
    
    fn get_registers(&self) -> HashMap<String, u32> {
        let mut registers = HashMap::new();
        for (i, &value) in self.d.iter().enumerate() {
            registers.insert(format!("d{}", i), value);
        }
        for (i, &value) in self.a.iter().enumerate() {
            registers.insert(format!("a{}", i), value);
        }
        registers.insert("pc".to_string(), self.pc);
        registers.insert("sr".to_string(), self.sr as u32);
        registers
    }
    
    fn is_halted(&self) -> bool {
        self.halted
    }
    
    fn get_flags(&self) -> CpuFlags {
        CpuFlags {
            carry: self.get_flag(0),
            zero: self.get_flag(1),
            overflow: self.get_flag(2),
            negative: self.get_flag(3),
            extend: self.get_flag(4),
            interrupt_disable: self.get_flag(8),
            decimal: false, // Not used in 68000
            break_command: false, // Not used in 68000
            sign: self.get_flag(3),
            parity: false, // Not used in 68000
            auxiliary_carry: false, // Not used in 68000
        }
    }
    
    fn set_flags(&mut self, flags: CpuFlags) -> EmuliteResult<()> {
        self.set_flag(0, flags.carry);
        self.set_flag(1, flags.zero);
        self.set_flag(2, flags.overflow);
        self.set_flag(3, flags.negative);
        self.set_flag(4, flags.extend);
        self.set_flag(8, flags.interrupt_disable);
        Ok(())
    }
    
    fn info(&self) -> CpuInfo {
        CpuInfo {
            name: "Motorola 68000".to_string(),
            architecture: "68000".to_string(),
            bits: 32,
            endianness: Endianness::Big,
            register_count: 18,
            instruction_count: 56,
            clock_speed_hz: 7_670_000, // 7.67 MHz
        }
    }
    
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
