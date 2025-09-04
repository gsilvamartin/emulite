//! ARM CPU emulation (used in modern consoles and mobile devices)

use crate::cpu::{Cpu, CpuFlags, CpuInfo, Endianness};
use crate::{EmuliteResult, EmuliteError};
use std::collections::HashMap;

/// ARM CPU implementation
pub struct Arm {
    // General purpose registers (R0-R15)
    pub registers: [u32; 16],
    
    // Program counter
    pub pc: u32,
    
    // Current Program Status Register
    pub cpsr: u32,
    
    // Saved Program Status Registers
    pub spsr: [u32; 5],
    
    // Memory interface
    memory: Vec<u8>,
    
    // CPU state
    cycles: u64,
    halted: bool,
    
    // Instruction decoding
    instruction: u32,
    opcode: u8,
    condition: u8,
    rn: u8,
    rd: u8,
    rm: u8,
    rs: u8,
    immediate: u32,
    shift_amount: u8,
    shift_type: u8,
}

impl Arm {
    pub fn new() -> EmuliteResult<Self> {
        Ok(Self {
            registers: [0; 16],
            pc: 0,
            cpsr: 0x000000D3, // Supervisor mode, interrupts disabled
            spsr: [0; 5],
            memory: vec![0; 0x10000000], // 256MB address space
            cycles: 0,
            halted: false,
            instruction: 0,
            opcode: 0,
            condition: 0,
            rn: 0,
            rd: 0,
            rm: 0,
            rs: 0,
            immediate: 0,
            shift_amount: 0,
            shift_type: 0,
        })
    }
    
    fn read_memory_32(&self, address: u32) -> u32 {
        let b0 = self.memory[address as usize];
        let b1 = self.memory[(address + 1) as usize];
        let b2 = self.memory[(address + 2) as usize];
        let b3 = self.memory[(address + 3) as usize];
        u32::from_le_bytes([b0, b1, b2, b3]) // Little-endian
    }
    
    fn write_memory_32(&mut self, address: u32, value: u32) {
        let bytes = value.to_le_bytes();
        self.memory[address as usize] = bytes[0];
        self.memory[(address + 1) as usize] = bytes[1];
        self.memory[(address + 2) as usize] = bytes[2];
        self.memory[(address + 3) as usize] = bytes[3];
    }
    
    fn read_memory_16(&self, address: u32) -> u16 {
        let low = self.memory[address as usize];
        let high = self.memory[(address + 1) as usize];
        u16::from_le_bytes([low, high]) // Little-endian
    }
    
    fn write_memory_16(&mut self, address: u32, value: u16) {
        let bytes = value.to_le_bytes();
        self.memory[address as usize] = bytes[0];
        self.memory[(address + 1) as usize] = bytes[1];
    }
    
    fn check_condition(&self, condition: u8) -> bool {
        match condition {
            0x0 => (self.cpsr & 0x40000000) != 0, // EQ (Equal)
            0x1 => (self.cpsr & 0x40000000) == 0, // NE (Not Equal)
            0x2 => (self.cpsr & 0x20000000) != 0, // CS/HS (Carry Set/Unsigned Higher or Same)
            0x3 => (self.cpsr & 0x20000000) == 0, // CC/LO (Carry Clear/Unsigned Lower)
            0x4 => (self.cpsr & 0x80000000) != 0, // MI (Minus/Negative)
            0x5 => (self.cpsr & 0x80000000) == 0, // PL (Plus/Positive or Zero)
            0x6 => (self.cpsr & 0x10000000) != 0, // VS (Overflow)
            0x7 => (self.cpsr & 0x10000000) == 0, // VC (No Overflow)
            0x8 => (self.cpsr & 0x20000000) != 0 && (self.cpsr & 0x40000000) == 0, // HI (Unsigned Higher)
            0x9 => (self.cpsr & 0x20000000) == 0 || (self.cpsr & 0x40000000) != 0, // LS (Unsigned Lower or Same)
            0xA => (self.cpsr & 0x80000000) == (self.cpsr & 0x10000000), // GE (Signed Greater than or Equal)
            0xB => (self.cpsr & 0x80000000) != (self.cpsr & 0x10000000), // LT (Signed Less Than)
            0xC => (self.cpsr & 0x40000000) == 0 && (self.cpsr & 0x80000000) == (self.cpsr & 0x10000000), // GT (Signed Greater Than)
            0xD => (self.cpsr & 0x40000000) != 0 || (self.cpsr & 0x80000000) != (self.cpsr & 0x10000000), // LE (Signed Less than or Equal)
            0xE => true, // AL (Always)
            0xF => false, // NV (Never)
            _ => false,
        }
    }
    
    fn update_flags(&mut self, result: u32, carry: bool, overflow: bool) {
        // Zero flag
        if result == 0 {
            self.cpsr |= 0x40000000;
        } else {
            self.cpsr &= !0x40000000;
        }
        
        // Negative flag
        if (result & 0x80000000) != 0 {
            self.cpsr |= 0x80000000;
        } else {
            self.cpsr &= !0x80000000;
        }
        
        // Carry flag
        if carry {
            self.cpsr |= 0x20000000;
        } else {
            self.cpsr &= !0x20000000;
        }
        
        // Overflow flag
        if overflow {
            self.cpsr |= 0x10000000;
        } else {
            self.cpsr &= !0x10000000;
        }
    }
    
    fn execute_instruction(&mut self) -> EmuliteResult<()> {
        if !self.check_condition(self.condition) {
            return Ok(());
        }
        
        match self.opcode {
            // Data Processing instructions
            0x0 => { // AND
                let result = self.registers[self.rn as usize] & self.registers[self.rm as usize];
                self.registers[self.rd as usize] = result;
                self.update_flags(result, false, false);
            },
            0x1 => { // EOR
                let result = self.registers[self.rn as usize] ^ self.registers[self.rm as usize];
                self.registers[self.rd as usize] = result;
                self.update_flags(result, false, false);
            },
            0x2 => { // SUB
                let result = self.registers[self.rn as usize].wrapping_sub(self.registers[self.rm as usize]);
                let carry = self.registers[self.rn as usize] >= self.registers[self.rm as usize];
                let overflow = ((self.registers[self.rn as usize] ^ result) & (self.registers[self.rm as usize] ^ result) & 0x80000000) != 0;
                self.registers[self.rd as usize] = result;
                self.update_flags(result, carry, overflow);
            },
            0x3 => { // RSB
                let result = self.registers[self.rm as usize].wrapping_sub(self.registers[self.rn as usize]);
                let carry = self.registers[self.rm as usize] >= self.registers[self.rn as usize];
                let overflow = ((self.registers[self.rm as usize] ^ result) & (self.registers[self.rn as usize] ^ result) & 0x80000000) != 0;
                self.registers[self.rd as usize] = result;
                self.update_flags(result, carry, overflow);
            },
            0x4 => { // ADD
                let result = self.registers[self.rn as usize].wrapping_add(self.registers[self.rm as usize]);
                let carry = result < self.registers[self.rn as usize];
                let overflow = ((self.registers[self.rn as usize] ^ result) & (self.registers[self.rm as usize] ^ result) & 0x80000000) != 0;
                self.registers[self.rd as usize] = result;
                self.update_flags(result, carry, overflow);
            },
            0x5 => { // ADC
                let carry_in = if (self.cpsr & 0x20000000) != 0 { 1 } else { 0 };
                let result = self.registers[self.rn as usize].wrapping_add(self.registers[self.rm as usize]).wrapping_add(carry_in);
                let carry = result < self.registers[self.rn as usize] || (carry_in != 0 && result == self.registers[self.rn as usize]);
                let overflow = ((self.registers[self.rn as usize] ^ result) & (self.registers[self.rm as usize] ^ result) & 0x80000000) != 0;
                self.registers[self.rd as usize] = result;
                self.update_flags(result, carry, overflow);
            },
            0x6 => { // SBC
                let carry_in = if (self.cpsr & 0x20000000) != 0 { 1 } else { 0 };
                let result = self.registers[self.rn as usize].wrapping_sub(self.registers[self.rm as usize]).wrapping_sub(1 - carry_in);
                let carry = self.registers[self.rn as usize] >= self.registers[self.rm as usize] && (carry_in != 0 || self.registers[self.rn as usize] > self.registers[self.rm as usize]);
                let overflow = ((self.registers[self.rn as usize] ^ result) & (self.registers[self.rm as usize] ^ result) & 0x80000000) != 0;
                self.registers[self.rd as usize] = result;
                self.update_flags(result, carry, overflow);
            },
            0x7 => { // RSC
                let carry_in = if (self.cpsr & 0x20000000) != 0 { 1 } else { 0 };
                let result = self.registers[self.rm as usize].wrapping_sub(self.registers[self.rn as usize]).wrapping_sub(1 - carry_in);
                let carry = self.registers[self.rm as usize] >= self.registers[self.rn as usize] && (carry_in != 0 || self.registers[self.rm as usize] > self.registers[self.rn as usize]);
                let overflow = ((self.registers[self.rm as usize] ^ result) & (self.registers[self.rn as usize] ^ result) & 0x80000000) != 0;
                self.registers[self.rd as usize] = result;
                self.update_flags(result, carry, overflow);
            },
            0x8 => { // TST
                let result = self.registers[self.rn as usize] & self.registers[self.rm as usize];
                self.update_flags(result, false, false);
            },
            0x9 => { // TEQ
                let result = self.registers[self.rn as usize] ^ self.registers[self.rm as usize];
                self.update_flags(result, false, false);
            },
            0xA => { // CMP
                let result = self.registers[self.rn as usize].wrapping_sub(self.registers[self.rm as usize]);
                let carry = self.registers[self.rn as usize] >= self.registers[self.rm as usize];
                let overflow = ((self.registers[self.rn as usize] ^ result) & (self.registers[self.rm as usize] ^ result) & 0x80000000) != 0;
                self.update_flags(result, carry, overflow);
            },
            0xB => { // CMN
                let result = self.registers[self.rn as usize].wrapping_add(self.registers[self.rm as usize]);
                let carry = result < self.registers[self.rn as usize];
                let overflow = ((self.registers[self.rn as usize] ^ result) & (self.registers[self.rm as usize] ^ result) & 0x80000000) != 0;
                self.update_flags(result, carry, overflow);
            },
            0xC => { // ORR
                let result = self.registers[self.rn as usize] | self.registers[self.rm as usize];
                self.registers[self.rd as usize] = result;
                self.update_flags(result, false, false);
            },
            0xD => { // MOV
                let result = self.registers[self.rm as usize];
                self.registers[self.rd as usize] = result;
                self.update_flags(result, false, false);
            },
            0xE => { // BIC
                let result = self.registers[self.rn as usize] & !self.registers[self.rm as usize];
                self.registers[self.rd as usize] = result;
                self.update_flags(result, false, false);
            },
            0xF => { // MVN
                let result = !self.registers[self.rm as usize];
                self.registers[self.rd as usize] = result;
                self.update_flags(result, false, false);
            },
            
            // Load/Store instructions
            0x40 => { // LDR
                let address = self.registers[self.rn as usize] + self.registers[self.rm as usize];
                let value = self.read_memory_32(address);
                self.registers[self.rd as usize] = value;
            },
            0x41 => { // LDRB
                let address = self.registers[self.rn as usize] + self.registers[self.rm as usize];
                let value = self.memory[address as usize] as u32;
                self.registers[self.rd as usize] = value;
            },
            0x42 => { // LDRH
                let address = self.registers[self.rn as usize] + self.registers[self.rm as usize];
                let value = self.read_memory_16(address) as u32;
                self.registers[self.rd as usize] = value;
            },
            0x44 => { // STR
                let address = self.registers[self.rn as usize] + self.registers[self.rm as usize];
                let value = self.registers[self.rd as usize];
                self.write_memory_32(address, value);
            },
            0x45 => { // STRB
                let address = self.registers[self.rn as usize] + self.registers[self.rm as usize];
                let value = self.registers[self.rd as usize] as u8;
                self.memory[address as usize] = value;
            },
            0x46 => { // STRH
                let address = self.registers[self.rn as usize] + self.registers[self.rm as usize];
                let value = self.registers[self.rd as usize] as u16;
                self.write_memory_16(address, value);
            },
            
            // Branch instructions
            0x50 => { // B
                let offset = self.immediate as i32;
                if (offset & 0x800000) != 0 {
                    // Sign extend
                    let sign_extended = offset | 0xFF000000u32 as i32;
                    self.pc = self.pc.wrapping_add(sign_extended as u32);
                } else {
                    self.pc = self.pc.wrapping_add(offset as u32);
                }
            },
            0x51 => { // BL
                let offset = self.immediate as i32;
                self.registers[14] = self.pc; // Save return address in LR
                if (offset & 0x800000) != 0 {
                    // Sign extend
                    let sign_extended = offset | 0xFF000000u32 as i32;
                    self.pc = self.pc.wrapping_add(sign_extended as u32);
                } else {
                    self.pc = self.pc.wrapping_add(offset as u32);
                }
            },
            
            _ => {
                return Err(EmuliteError::CpuError(
                    format!("Unimplemented opcode: 0x{:02X}", self.opcode)
                ));
            }
        }
        
        Ok(())
    }
}

impl Cpu for Arm {
    fn step(&mut self) -> EmuliteResult<()> {
        if self.halted {
            return Ok(());
        }
        
        // Fetch instruction
        self.instruction = self.read_memory_32(self.pc);
        self.pc += 4;
        
        // Decode instruction
        self.condition = ((self.instruction >> 28) & 0xF) as u8;
        self.opcode = ((self.instruction >> 21) & 0xF) as u8;
        self.rn = ((self.instruction >> 16) & 0xF) as u8;
        self.rd = ((self.instruction >> 12) & 0xF) as u8;
        self.rm = (self.instruction & 0xF) as u8;
        self.rs = ((self.instruction >> 8) & 0xF) as u8;
        self.immediate = self.instruction & 0xFFF;
        self.shift_amount = ((self.instruction >> 7) & 0x1F) as u8;
        self.shift_type = ((self.instruction >> 5) & 0x3) as u8;
        
        // Execute instruction
        self.execute_instruction()?;
        
        self.cycles += 1;
        Ok(())
    }
    
    fn reset(&mut self) -> EmuliteResult<()> {
        self.registers.fill(0);
        self.pc = self.read_memory_32(0); // Reset vector
        self.cpsr = 0x000000D3; // Supervisor mode, interrupts disabled
        self.spsr.fill(0);
        self.memory.fill(0);
        self.cycles = 0;
        self.halted = false;
        Ok(())
    }
    
    fn name(&self) -> &str {
        "ARM"
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
            "r0" => Ok(self.registers[0]),
            "r1" => Ok(self.registers[1]),
            "r2" => Ok(self.registers[2]),
            "r3" => Ok(self.registers[3]),
            "r4" => Ok(self.registers[4]),
            "r5" => Ok(self.registers[5]),
            "r6" => Ok(self.registers[6]),
            "r7" => Ok(self.registers[7]),
            "r8" => Ok(self.registers[8]),
            "r9" => Ok(self.registers[9]),
            "r10" => Ok(self.registers[10]),
            "r11" => Ok(self.registers[11]),
            "r12" => Ok(self.registers[12]),
            "r13" | "sp" => Ok(self.registers[13]),
            "r14" | "lr" => Ok(self.registers[14]),
            "r15" | "pc" => Ok(self.registers[15]),
            "cpsr" => Ok(self.cpsr),
            _ => Err(EmuliteError::CpuError(format!("Unknown register: {}", reg))),
        }
    }
    
    fn set_register(&mut self, reg: &str, value: u32) -> EmuliteResult<()> {
        match reg.to_lowercase().as_str() {
            "r0" => self.registers[0] = value,
            "r1" => self.registers[1] = value,
            "r2" => self.registers[2] = value,
            "r3" => self.registers[3] = value,
            "r4" => self.registers[4] = value,
            "r5" => self.registers[5] = value,
            "r6" => self.registers[6] = value,
            "r7" => self.registers[7] = value,
            "r8" => self.registers[8] = value,
            "r9" => self.registers[9] = value,
            "r10" => self.registers[10] = value,
            "r11" => self.registers[11] = value,
            "r12" => self.registers[12] = value,
            "r13" | "sp" => self.registers[13] = value,
            "r14" | "lr" => self.registers[14] = value,
            "r15" | "pc" => self.registers[15] = value,
            "cpsr" => self.cpsr = value,
            _ => return Err(EmuliteError::CpuError(format!("Unknown register: {}", reg))),
        }
        Ok(())
    }
    
    fn get_registers(&self) -> HashMap<String, u32> {
        let mut registers = HashMap::new();
        for (i, &value) in self.registers.iter().enumerate() {
            registers.insert(format!("r{}", i), value);
        }
        registers.insert("pc".to_string(), self.pc);
        registers.insert("cpsr".to_string(), self.cpsr);
        registers
    }
    
    fn is_halted(&self) -> bool {
        self.halted
    }
    
    fn get_flags(&self) -> CpuFlags {
        CpuFlags {
            carry: (self.cpsr & 0x20000000) != 0,
            zero: (self.cpsr & 0x40000000) != 0,
            overflow: (self.cpsr & 0x10000000) != 0,
            negative: (self.cpsr & 0x80000000) != 0,
            extend: false, // Not used in ARM
            interrupt_disable: (self.cpsr & 0x00000080) != 0,
            decimal: false, // Not used in ARM
            break_command: false, // Not used in ARM
            sign: (self.cpsr & 0x80000000) != 0,
            parity: false, // Not used in ARM
            auxiliary_carry: false, // Not used in ARM
        }
    }
    
    fn set_flags(&mut self, flags: CpuFlags) -> EmuliteResult<()> {
        if flags.carry {
            self.cpsr |= 0x20000000;
        } else {
            self.cpsr &= !0x20000000;
        }
        
        if flags.zero {
            self.cpsr |= 0x40000000;
        } else {
            self.cpsr &= !0x40000000;
        }
        
        if flags.overflow {
            self.cpsr |= 0x10000000;
        } else {
            self.cpsr &= !0x10000000;
        }
        
        if flags.negative {
            self.cpsr |= 0x80000000;
        } else {
            self.cpsr &= !0x80000000;
        }
        
        if flags.interrupt_disable {
            self.cpsr |= 0x00000080;
        } else {
            self.cpsr &= !0x00000080;
        }
        
        Ok(())
    }
    
    fn info(&self) -> CpuInfo {
        CpuInfo {
            name: "ARM".to_string(),
            architecture: "ARM".to_string(),
            bits: 32,
            endianness: Endianness::Little,
            register_count: 16,
            instruction_count: 64,
            clock_speed_hz: 200_000_000, // 200 MHz
        }
    }
    
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
