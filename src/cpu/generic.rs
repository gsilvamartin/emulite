//! Generic CPU implementation for testing and fallback

use crate::cpu::{Cpu, CpuFlags, CpuInfo, Endianness};
use crate::{EmuliteResult, EmuliteError};
use std::collections::HashMap;

/// Generic CPU implementation
pub struct GenericCpu {
    // Registers
    pub registers: HashMap<String, u32>,
    
    // Program counter
    pub pc: u32,
    
    // Flags
    pub flags: CpuFlags,
    
    // Memory interface
    memory: Vec<u8>,
    
    // CPU state
    cycles: u64,
    halted: bool,
    
    // Instruction decoding
    opcode: u8,
    operand: u32,
}

impl GenericCpu {
    pub fn new(memory_size: usize) -> Self {
        let mut registers = HashMap::new();
        registers.insert("r0".to_string(), 0);
        registers.insert("r1".to_string(), 0);
        registers.insert("r2".to_string(), 0);
        registers.insert("r3".to_string(), 0);
        registers.insert("r4".to_string(), 0);
        registers.insert("r5".to_string(), 0);
        registers.insert("r6".to_string(), 0);
        registers.insert("r7".to_string(), 0);
        registers.insert("r8".to_string(), 0);
        registers.insert("r9".to_string(), 0);
        registers.insert("r10".to_string(), 0);
        registers.insert("r11".to_string(), 0);
        registers.insert("r12".to_string(), 0);
        registers.insert("r13".to_string(), 0);
        registers.insert("r14".to_string(), 0);
        registers.insert("r15".to_string(), 0);
        
        Self {
            registers,
            pc: 0,
            flags: CpuFlags::default(),
            memory: vec![0; memory_size],
            cycles: 0,
            halted: false,
            opcode: 0,
            operand: 0,
        }
    }
    
    fn execute_instruction(&mut self) -> EmuliteResult<()> {
        match self.opcode {
            0x00 => { // NOP - No Operation
                // Do nothing
            },
            0x01 => { // HLT - Halt
                self.halted = true;
            },
            0x02 => { // MOV - Move
                let source = self.registers.get("r1").copied().unwrap_or(0);
                self.registers.insert("r0".to_string(), source);
            },
            0x03 => { // ADD - Add
                let a = self.registers.get("r1").copied().unwrap_or(0);
                let b = self.registers.get("r2").copied().unwrap_or(0);
                let result = a.wrapping_add(b);
                self.registers.insert("r0".to_string(), result);
                self.update_flags(result);
            },
            0x04 => { // SUB - Subtract
                let a = self.registers.get("r1").copied().unwrap_or(0);
                let b = self.registers.get("r2").copied().unwrap_or(0);
                let result = a.wrapping_sub(b);
                self.registers.insert("r0".to_string(), result);
                self.update_flags(result);
            },
            0x05 => { // AND - Logical AND
                let a = self.registers.get("r1").copied().unwrap_or(0);
                let b = self.registers.get("r2").copied().unwrap_or(0);
                let result = a & b;
                self.registers.insert("r0".to_string(), result);
                self.update_flags(result);
            },
            0x06 => { // OR - Logical OR
                let a = self.registers.get("r1").copied().unwrap_or(0);
                let b = self.registers.get("r2").copied().unwrap_or(0);
                let result = a | b;
                self.registers.insert("r0".to_string(), result);
                self.update_flags(result);
            },
            0x07 => { // XOR - Logical XOR
                let a = self.registers.get("r1").copied().unwrap_or(0);
                let b = self.registers.get("r2").copied().unwrap_or(0);
                let result = a ^ b;
                self.registers.insert("r0".to_string(), result);
                self.update_flags(result);
            },
            0x08 => { // NOT - Logical NOT
                let a = self.registers.get("r1").copied().unwrap_or(0);
                let result = !a;
                self.registers.insert("r0".to_string(), result);
                self.update_flags(result);
            },
            0x09 => { // SHL - Shift Left
                let a = self.registers.get("r1").copied().unwrap_or(0);
                let b = self.registers.get("r2").copied().unwrap_or(0);
                let result = a << (b & 0x1F);
                self.registers.insert("r0".to_string(), result);
                self.update_flags(result);
            },
            0x0A => { // SHR - Shift Right
                let a = self.registers.get("r1").copied().unwrap_or(0);
                let b = self.registers.get("r2").copied().unwrap_or(0);
                let result = a >> (b & 0x1F);
                self.registers.insert("r0".to_string(), result);
                self.update_flags(result);
            },
            0x0B => { // JMP - Jump
                let address = self.registers.get("r1").copied().unwrap_or(0);
                self.pc = address;
            },
            0x0C => { // JZ - Jump if Zero
                if self.flags.zero {
                    let address = self.registers.get("r1").copied().unwrap_or(0);
                    self.pc = address;
                }
            },
            0x0D => { // JNZ - Jump if Not Zero
                if !self.flags.zero {
                    let address = self.registers.get("r1").copied().unwrap_or(0);
                    self.pc = address;
                }
            },
            0x0E => { // JC - Jump if Carry
                if self.flags.carry {
                    let address = self.registers.get("r1").copied().unwrap_or(0);
                    self.pc = address;
                }
            },
            0x0F => { // JNC - Jump if Not Carry
                if !self.flags.carry {
                    let address = self.registers.get("r1").copied().unwrap_or(0);
                    self.pc = address;
                }
            },
            0x10 => { // CALL - Call Subroutine
                let address = self.registers.get("r1").copied().unwrap_or(0);
                self.registers.insert("r15".to_string(), self.pc); // Save return address
                self.pc = address;
            },
            0x11 => { // RET - Return
                let return_address = self.registers.get("r15").copied().unwrap_or(0);
                self.pc = return_address;
            },
            0x12 => { // PUSH - Push to Stack
                let value = self.registers.get("r1").copied().unwrap_or(0);
                let sp = self.registers.get("r13").copied().unwrap_or(0);
                self.write_memory_32(sp, value)?;
                self.registers.insert("r13".to_string(), sp.wrapping_sub(4));
            },
            0x13 => { // POP - Pop from Stack
                let sp = self.registers.get("r13").copied().unwrap_or(0);
                let value = self.read_memory_32(sp.wrapping_add(4))?;
                self.registers.insert("r0".to_string(), value);
                self.registers.insert("r13".to_string(), sp.wrapping_add(4));
            },
            0x14 => { // LOAD - Load from Memory
                let address = self.registers.get("r1").copied().unwrap_or(0);
                let value = self.read_memory_32(address)?;
                self.registers.insert("r0".to_string(), value);
            },
            0x15 => { // STORE - Store to Memory
                let address = self.registers.get("r1").copied().unwrap_or(0);
                let value = self.registers.get("r2").copied().unwrap_or(0);
                self.write_memory_32(address, value)?;
            },
            0x16 => { // LOADB - Load Byte from Memory
                let address = self.registers.get("r1").copied().unwrap_or(0);
                let value = self.read_memory(address)?;
                self.registers.insert("r0".to_string(), value as u32);
            },
            0x17 => { // STOREB - Store Byte to Memory
                let address = self.registers.get("r1").copied().unwrap_or(0);
                let value = self.registers.get("r2").copied().unwrap_or(0);
                self.write_memory(address, value as u8)?;
            },
            0x18 => { // CMP - Compare
                let a = self.registers.get("r1").copied().unwrap_or(0);
                let b = self.registers.get("r2").copied().unwrap_or(0);
                let result = a.wrapping_sub(b);
                self.update_flags(result);
            },
            0x19 => { // TEST - Test
                let a = self.registers.get("r1").copied().unwrap_or(0);
                let b = self.registers.get("r2").copied().unwrap_or(0);
                let result = a & b;
                self.update_flags(result);
            },
            0x1A => { // INC - Increment
                let a = self.registers.get("r1").copied().unwrap_or(0);
                let result = a.wrapping_add(1);
                self.registers.insert("r0".to_string(), result);
                self.update_flags(result);
            },
            0x1B => { // DEC - Decrement
                let a = self.registers.get("r1").copied().unwrap_or(0);
                let result = a.wrapping_sub(1);
                self.registers.insert("r0".to_string(), result);
                self.update_flags(result);
            },
            0x1C => { // NEG - Negate
                let a = self.registers.get("r1").copied().unwrap_or(0);
                let result = a.wrapping_neg();
                self.registers.insert("r0".to_string(), result);
                self.update_flags(result);
            },
            0x1D => { // MUL - Multiply
                let a = self.registers.get("r1").copied().unwrap_or(0);
                let b = self.registers.get("r2").copied().unwrap_or(0);
                let result = a.wrapping_mul(b);
                self.registers.insert("r0".to_string(), result);
                self.update_flags(result);
            },
            0x1E => { // DIV - Divide
                let a = self.registers.get("r1").copied().unwrap_or(0);
                let b = self.registers.get("r2").copied().unwrap_or(0);
                if b == 0 {
                    return Err(EmuliteError::CpuError("Division by zero".to_string()));
                }
                let result = a / b;
                self.registers.insert("r0".to_string(), result);
                self.update_flags(result);
            },
            0x1F => { // MOD - Modulo
                let a = self.registers.get("r1").copied().unwrap_or(0);
                let b = self.registers.get("r2").copied().unwrap_or(0);
                if b == 0 {
                    return Err(EmuliteError::CpuError("Division by zero".to_string()));
                }
                let result = a % b;
                self.registers.insert("r0".to_string(), result);
                self.update_flags(result);
            },
            _ => {
                return Err(EmuliteError::CpuError(
                    format!("Unimplemented opcode: 0x{:02X}", self.opcode)
                ));
            }
        }
        
        Ok(())
    }
    
    fn update_flags(&mut self, result: u32) {
        self.flags.zero = result == 0;
        self.flags.negative = (result & 0x80000000) != 0;
        self.flags.carry = result < 0xFFFFFFFF; // Simplified carry detection
        self.flags.overflow = false; // Simplified overflow detection
    }
    
    fn read_memory_32(&self, address: u32) -> EmuliteResult<u32> {
        if address as usize + 4 > self.memory.len() {
            return Err(EmuliteError::MemoryAccessViolation(address));
        }
        let b0 = self.memory[address as usize];
        let b1 = self.memory[(address + 1) as usize];
        let b2 = self.memory[(address + 2) as usize];
        let b3 = self.memory[(address + 3) as usize];
        Ok(u32::from_le_bytes([b0, b1, b2, b3]))
    }
    
    fn write_memory_32(&mut self, address: u32, value: u32) -> EmuliteResult<()> {
        if address as usize + 4 > self.memory.len() {
            return Err(EmuliteError::MemoryAccessViolation(address));
        }
        let bytes = value.to_le_bytes();
        self.memory[address as usize] = bytes[0];
        self.memory[(address + 1) as usize] = bytes[1];
        self.memory[(address + 2) as usize] = bytes[2];
        self.memory[(address + 3) as usize] = bytes[3];
        Ok(())
    }
}

impl Cpu for GenericCpu {
    fn step(&mut self) -> EmuliteResult<()> {
        if self.halted {
            return Ok(());
        }
        
        // Fetch instruction
        self.opcode = self.memory[self.pc as usize];
        self.pc = self.pc.wrapping_add(1);
        
        // Decode operand if needed
        if self.needs_operand(self.opcode) {
            self.operand = self.read_memory_32(self.pc)?;
            self.pc = self.pc.wrapping_add(4);
        }
        
        // Execute instruction
        self.execute_instruction()?;
        
        self.cycles += 1;
        Ok(())
    }
    
    fn reset(&mut self) -> EmuliteResult<()> {
        self.registers.clear();
        self.registers.insert("r0".to_string(), 0);
        self.registers.insert("r1".to_string(), 0);
        self.registers.insert("r2".to_string(), 0);
        self.registers.insert("r3".to_string(), 0);
        self.registers.insert("r4".to_string(), 0);
        self.registers.insert("r5".to_string(), 0);
        self.registers.insert("r6".to_string(), 0);
        self.registers.insert("r7".to_string(), 0);
        self.registers.insert("r8".to_string(), 0);
        self.registers.insert("r9".to_string(), 0);
        self.registers.insert("r10".to_string(), 0);
        self.registers.insert("r11".to_string(), 0);
        self.registers.insert("r12".to_string(), 0);
        self.registers.insert("r13".to_string(), 0);
        self.registers.insert("r14".to_string(), 0);
        self.registers.insert("r15".to_string(), 0);
        self.pc = 0;
        self.flags = CpuFlags::default();
        self.memory.fill(0);
        self.cycles = 0;
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
            instruction_count: 32, // 0x00-0x1F
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

impl GenericCpu {
    fn needs_operand(&self, opcode: u8) -> bool {
        // Some instructions need operands
        matches!(opcode, 0x0B | 0x0C | 0x0D | 0x0E | 0x0F | 0x10)
    }
}
