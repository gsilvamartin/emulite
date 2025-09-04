//! MIPS CPU emulation (used in PlayStation, Nintendo 64, etc.)

use crate::cpu::{Cpu, CpuFlags, CpuInfo, Endianness};
use crate::{EmuliteResult, EmuliteError};
use std::collections::HashMap;

/// MIPS CPU implementation
pub struct Mips {
    // General purpose registers (R0-R31)
    pub registers: [u32; 32],
    
    // Program counter
    pub pc: u32,
    
    // Coprocessor 0 registers
    pub cp0_registers: [u32; 32],
    
    // Memory interface
    memory: Vec<u8>,
    
    // CPU state
    cycles: u64,
    halted: bool,
    
    // Instruction decoding
    opcode: u32,
    rs: u8,
    rt: u8,
    rd: u8,
    shamt: u8,
    funct: u8,
    immediate: u16,
    address: u32,
}

impl Mips {
    pub fn new() -> EmuliteResult<Self> {
        Ok(Self {
            registers: [0; 32],
            pc: 0,
            cp0_registers: [0; 32],
            memory: vec![0; 0x10000000], // 256MB address space
            cycles: 0,
            halted: false,
            opcode: 0,
            rs: 0,
            rt: 0,
            rd: 0,
            shamt: 0,
            funct: 0,
            immediate: 0,
            address: 0,
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
    
    fn execute_instruction(&mut self) -> EmuliteResult<()> {
        match self.opcode {
            // R-type instructions
            0x00 => {
                match self.funct {
                    0x20 => { // ADD
                        let result = self.registers[self.rs as usize].wrapping_add(self.registers[self.rt as usize]);
                        self.registers[self.rd as usize] = result;
                    },
                    0x22 => { // SUB
                        let result = self.registers[self.rs as usize].wrapping_sub(self.registers[self.rt as usize]);
                        self.registers[self.rd as usize] = result;
                    },
                    0x24 => { // AND
                        self.registers[self.rd as usize] = self.registers[self.rs as usize] & self.registers[self.rt as usize];
                    },
                    0x25 => { // OR
                        self.registers[self.rd as usize] = self.registers[self.rs as usize] | self.registers[self.rt as usize];
                    },
                    0x26 => { // XOR
                        self.registers[self.rd as usize] = self.registers[self.rs as usize] ^ self.registers[self.rt as usize];
                    },
                    0x27 => { // NOR
                        self.registers[self.rd as usize] = !(self.registers[self.rs as usize] | self.registers[self.rt as usize]);
                    },
                    0x2A => { // SLT
                        let result = if (self.registers[self.rs as usize] as i32) < (self.registers[self.rt as usize] as i32) {
                            1
                        } else {
                            0
                        };
                        self.registers[self.rd as usize] = result;
                    },
                    0x00 => { // SLL
                        self.registers[self.rd as usize] = self.registers[self.rt as usize] << self.shamt;
                    },
                    0x02 => { // SRL
                        self.registers[self.rd as usize] = self.registers[self.rt as usize] >> self.shamt;
                    },
                    0x03 => { // SRA
                        self.registers[self.rd as usize] = (self.registers[self.rt as usize] as i32 >> self.shamt) as u32;
                    },
                    0x08 => { // JR
                        self.pc = self.registers[self.rs as usize];
                    },
                    0x09 => { // JALR
                        self.registers[self.rd as usize] = self.pc + 4;
                        self.pc = self.registers[self.rs as usize];
                    },
                    _ => {
                        return Err(EmuliteError::CpuError(
                            format!("Unimplemented R-type function: 0x{:02X}", self.funct)
                        ));
                    }
                }
            },
            
            // J-type instructions
            0x02 => { // J
                self.pc = (self.pc & 0xF0000000) | (self.address << 2);
            },
            0x03 => { // JAL
                self.registers[31] = self.pc + 4; // Save return address in $ra
                self.pc = (self.pc & 0xF0000000) | (self.address << 2);
            },
            
            // I-type instructions
            0x08 => { // ADDI
                let result = self.registers[self.rs as usize].wrapping_add(self.immediate as i16 as u32);
                self.registers[self.rt as usize] = result;
            },
            0x09 => { // ADDIU
                let result = self.registers[self.rs as usize].wrapping_add(self.immediate as u32);
                self.registers[self.rt as usize] = result;
            },
            0x0A => { // SLTI
                let result = if (self.registers[self.rs as usize] as i32) < (self.immediate as i16 as i32) {
                    1
                } else {
                    0
                };
                self.registers[self.rt as usize] = result;
            },
            0x0C => { // ANDI
                self.registers[self.rt as usize] = self.registers[self.rs as usize] & (self.immediate as u32);
            },
            0x0D => { // ORI
                self.registers[self.rt as usize] = self.registers[self.rs as usize] | (self.immediate as u32);
            },
            0x0E => { // XORI
                self.registers[self.rt as usize] = self.registers[self.rs as usize] ^ (self.immediate as u32);
            },
            0x0F => { // LUI
                self.registers[self.rt as usize] = (self.immediate as u32) << 16;
            },
            0x20 => { // LB
                let address = self.registers[self.rs as usize] + (self.immediate as i16 as u32);
                let value = self.memory[address as usize] as i8 as u32;
                self.registers[self.rt as usize] = value;
            },
            0x21 => { // LH
                let address = self.registers[self.rs as usize] + (self.immediate as i16 as u32);
                let value = self.read_memory_16(address) as i16 as u32;
                self.registers[self.rt as usize] = value;
            },
            0x23 => { // LW
                let address = self.registers[self.rs as usize] + (self.immediate as i16 as u32);
                let value = self.read_memory_32(address);
                self.registers[self.rt as usize] = value;
            },
            0x24 => { // LBU
                let address = self.registers[self.rs as usize] + (self.immediate as i16 as u32);
                let value = self.memory[address as usize] as u32;
                self.registers[self.rt as usize] = value;
            },
            0x25 => { // LHU
                let address = self.registers[self.rs as usize] + (self.immediate as i16 as u32);
                let value = self.read_memory_16(address) as u32;
                self.registers[self.rt as usize] = value;
            },
            0x28 => { // SB
                let address = self.registers[self.rs as usize] + (self.immediate as i16 as u32);
                self.memory[address as usize] = self.registers[self.rt as usize] as u8;
            },
            0x29 => { // SH
                let address = self.registers[self.rs as usize] + (self.immediate as i16 as u32);
                self.write_memory_16(address, self.registers[self.rt as usize] as u16);
            },
            0x2B => { // SW
                let address = self.registers[self.rs as usize] + (self.immediate as i16 as u32);
                self.write_memory_32(address, self.registers[self.rt as usize]);
            },
            0x04 => { // BEQ
                if self.registers[self.rs as usize] == self.registers[self.rt as usize] {
                    self.pc += (self.immediate as i16 as u32) << 2;
                }
            },
            0x05 => { // BNE
                if self.registers[self.rs as usize] != self.registers[self.rt as usize] {
                    self.pc += (self.immediate as i16 as u32) << 2;
                }
            },
            0x06 => { // BLEZ
                if (self.registers[self.rs as usize] as i32) <= 0 {
                    self.pc += (self.immediate as i16 as u32) << 2;
                }
            },
            0x07 => { // BGTZ
                if (self.registers[self.rs as usize] as i32) > 0 {
                    self.pc += (self.immediate as i16 as u32) << 2;
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

impl Cpu for Mips {
    fn step(&mut self) -> EmuliteResult<()> {
        if self.halted {
            return Ok(());
        }
        
        // Fetch instruction
        let instruction = self.read_memory_32(self.pc);
        self.pc += 4;
        
        // Decode instruction
        self.opcode = (instruction >> 26) & 0x3F;
        self.rs = ((instruction >> 21) & 0x1F) as u8;
        self.rt = ((instruction >> 16) & 0x1F) as u8;
        self.rd = ((instruction >> 11) & 0x1F) as u8;
        self.shamt = ((instruction >> 6) & 0x1F) as u8;
        self.funct = (instruction & 0x3F) as u8;
        self.immediate = (instruction & 0xFFFF) as u16;
        self.address = instruction & 0x3FFFFFF;
        
        // Execute instruction
        self.execute_instruction()?;
        
        self.cycles += 1;
        Ok(())
    }
    
    fn reset(&mut self) -> EmuliteResult<()> {
        self.registers.fill(0);
        self.pc = self.read_memory_32(0xBFC00000); // Reset vector
        self.cp0_registers.fill(0);
        self.memory.fill(0);
        self.cycles = 0;
        self.halted = false;
        Ok(())
    }
    
    fn name(&self) -> &str {
        "MIPS R3000A"
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
            "r0" | "zero" => Ok(self.registers[0]),
            "r1" | "at" => Ok(self.registers[1]),
            "r2" | "v0" => Ok(self.registers[2]),
            "r3" | "v1" => Ok(self.registers[3]),
            "r4" | "a0" => Ok(self.registers[4]),
            "r5" | "a1" => Ok(self.registers[5]),
            "r6" | "a2" => Ok(self.registers[6]),
            "r7" | "a3" => Ok(self.registers[7]),
            "r8" | "t0" => Ok(self.registers[8]),
            "r9" | "t1" => Ok(self.registers[9]),
            "r10" | "t2" => Ok(self.registers[10]),
            "r11" | "t3" => Ok(self.registers[11]),
            "r12" | "t4" => Ok(self.registers[12]),
            "r13" | "t5" => Ok(self.registers[13]),
            "r14" | "t6" => Ok(self.registers[14]),
            "r15" | "t7" => Ok(self.registers[15]),
            "r16" | "s0" => Ok(self.registers[16]),
            "r17" | "s1" => Ok(self.registers[17]),
            "r18" | "s2" => Ok(self.registers[18]),
            "r19" | "s3" => Ok(self.registers[19]),
            "r20" | "s4" => Ok(self.registers[20]),
            "r21" | "s5" => Ok(self.registers[21]),
            "r22" | "s6" => Ok(self.registers[22]),
            "r23" | "s7" => Ok(self.registers[23]),
            "r24" | "t8" => Ok(self.registers[24]),
            "r25" | "t9" => Ok(self.registers[25]),
            "r26" | "k0" => Ok(self.registers[26]),
            "r27" | "k1" => Ok(self.registers[27]),
            "r28" | "gp" => Ok(self.registers[28]),
            "r29" | "sp" => Ok(self.registers[29]),
            "r30" | "fp" => Ok(self.registers[30]),
            "r31" | "ra" => Ok(self.registers[31]),
            "pc" => Ok(self.pc),
            _ => Err(EmuliteError::CpuError(format!("Unknown register: {}", reg))),
        }
    }
    
    fn set_register(&mut self, reg: &str, value: u32) -> EmuliteResult<()> {
        match reg.to_lowercase().as_str() {
            "r0" | "zero" => self.registers[0] = value,
            "r1" | "at" => self.registers[1] = value,
            "r2" | "v0" => self.registers[2] = value,
            "r3" | "v1" => self.registers[3] = value,
            "r4" | "a0" => self.registers[4] = value,
            "r5" | "a1" => self.registers[5] = value,
            "r6" | "a2" => self.registers[6] = value,
            "r7" | "a3" => self.registers[7] = value,
            "r8" | "t0" => self.registers[8] = value,
            "r9" | "t1" => self.registers[9] = value,
            "r10" | "t2" => self.registers[10] = value,
            "r11" | "t3" => self.registers[11] = value,
            "r12" | "t4" => self.registers[12] = value,
            "r13" | "t5" => self.registers[13] = value,
            "r14" | "t6" => self.registers[14] = value,
            "r15" | "t7" => self.registers[15] = value,
            "r16" | "s0" => self.registers[16] = value,
            "r17" | "s1" => self.registers[17] = value,
            "r18" | "s2" => self.registers[18] = value,
            "r19" | "s3" => self.registers[19] = value,
            "r20" | "s4" => self.registers[20] = value,
            "r21" | "s5" => self.registers[21] = value,
            "r22" | "s6" => self.registers[22] = value,
            "r23" | "s7" => self.registers[23] = value,
            "r24" | "t8" => self.registers[24] = value,
            "r25" | "t9" => self.registers[25] = value,
            "r26" | "k0" => self.registers[26] = value,
            "r27" | "k1" => self.registers[27] = value,
            "r28" | "gp" => self.registers[28] = value,
            "r29" | "sp" => self.registers[29] = value,
            "r30" | "fp" => self.registers[30] = value,
            "r31" | "ra" => self.registers[31] = value,
            "pc" => self.pc = value,
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
        registers
    }
    
    fn is_halted(&self) -> bool {
        self.halted
    }
    
    fn get_flags(&self) -> CpuFlags {
        CpuFlags {
            carry: false, // Not used in MIPS
            zero: false, // Not used in MIPS
            overflow: false, // Not used in MIPS
            negative: false, // Not used in MIPS
            extend: false, // Not used in MIPS
            interrupt_disable: false, // Not used in MIPS
            decimal: false, // Not used in MIPS
            break_command: false, // Not used in MIPS
            sign: false, // Not used in MIPS
            parity: false, // Not used in MIPS
            auxiliary_carry: false, // Not used in MIPS
        }
    }
    
    fn set_flags(&mut self, _flags: CpuFlags) -> EmuliteResult<()> {
        // MIPS doesn't use flags like other architectures
        Ok(())
    }
    
    fn info(&self) -> CpuInfo {
        CpuInfo {
            name: "MIPS R3000A".to_string(),
            architecture: "MIPS".to_string(),
            bits: 32,
            endianness: Endianness::Little,
            register_count: 32,
            instruction_count: 64,
            clock_speed_hz: 33_868_800, // 33.8688 MHz
        }
    }
    
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
