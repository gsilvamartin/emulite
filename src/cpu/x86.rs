//! x86 CPU emulation (used in PC emulation)

use crate::cpu::{Cpu, CpuFlags, CpuInfo, Endianness};
use crate::{EmuliteResult, EmuliteError};
use std::collections::HashMap;

/// x86 CPU implementation
pub struct X86 {
    // General purpose registers
    pub eax: u32,
    pub ebx: u32,
    pub ecx: u32,
    pub edx: u32,
    pub esi: u32,
    pub edi: u32,
    pub ebp: u32,
    pub esp: u32,
    
    // Segment registers
    pub cs: u16,
    pub ds: u16,
    pub es: u16,
    pub fs: u16,
    pub gs: u16,
    pub ss: u16,
    
    // Control registers
    pub eip: u32,
    pub eflags: u32,
    
    // Memory interface
    memory: Vec<u8>,
    
    // CPU state
    cycles: u64,
    halted: bool,
    
    // Instruction decoding
    opcode: u8,
    modrm: u8,
    sib: u8,
    displacement: u32,
    immediate: u32,
}

impl X86 {
    pub fn new() -> EmuliteResult<Self> {
        Ok(Self {
            eax: 0,
            ebx: 0,
            ecx: 0,
            edx: 0,
            esi: 0,
            edi: 0,
            ebp: 0,
            esp: 0,
            cs: 0,
            ds: 0,
            es: 0,
            fs: 0,
            gs: 0,
            ss: 0,
            eip: 0,
            eflags: 0x00000002, // Interrupt flag set
            memory: vec![0; 0x10000000], // 256MB address space
            cycles: 0,
            halted: false,
            opcode: 0,
            modrm: 0,
            sib: 0,
            displacement: 0,
            immediate: 0,
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
    
    fn get_effective_address(&mut self) -> u32 {
        let mod_val = (self.modrm >> 6) & 0x03;
        let reg_val = (self.modrm >> 3) & 0x07;
        let rm_val = self.modrm & 0x07;
        
        match mod_val {
            0 => {
                match rm_val {
                    0 => self.eax + self.displacement,
                    1 => self.ecx + self.displacement,
                    2 => self.edx + self.displacement,
                    3 => self.ebx + self.displacement,
                    4 => {
                        // SIB addressing
                        let scale = 1 << ((self.sib >> 6) & 0x03);
                        let index = (self.sib >> 3) & 0x07;
                        let base = self.sib & 0x07;
                        
                        let base_addr = match base {
                            0 => self.eax,
                            1 => self.ecx,
                            2 => self.edx,
                            3 => self.ebx,
                            4 => self.esp,
                            5 => self.ebp,
                            6 => self.esi,
                            7 => self.edi,
                            _ => 0,
                        };
                        
                        let index_addr = match index {
                            0 => self.eax,
                            1 => self.ecx,
                            2 => self.edx,
                            3 => self.ebx,
                            4 => 0, // No index
                            5 => self.ebp,
                            6 => self.esi,
                            7 => self.edi,
                            _ => 0,
                        };
                        
                        base_addr + (index_addr * scale) + self.displacement
                    },
                    5 => self.displacement, // Direct addressing
                    6 => self.esi + self.displacement,
                    7 => self.edi + self.displacement,
                    _ => 0,
                }
            },
            1 => {
                // 8-bit displacement
                let disp8 = self.displacement as i8 as u32;
                match rm_val {
                    0 => self.eax + disp8,
                    1 => self.ecx + disp8,
                    2 => self.edx + disp8,
                    3 => self.ebx + disp8,
                    4 => {
                        // SIB addressing with 8-bit displacement
                        let scale = 1 << ((self.sib >> 6) & 0x03);
                        let index = (self.sib >> 3) & 0x07;
                        let base = self.sib & 0x07;
                        
                        let base_addr = match base {
                            0 => self.eax,
                            1 => self.ecx,
                            2 => self.edx,
                            3 => self.ebx,
                            4 => self.esp,
                            5 => self.ebp,
                            6 => self.esi,
                            7 => self.edi,
                            _ => 0,
                        };
                        
                        let index_addr = match index {
                            0 => self.eax,
                            1 => self.ecx,
                            2 => self.edx,
                            3 => self.ebx,
                            4 => 0,
                            5 => self.ebp,
                            6 => self.esi,
                            7 => self.edi,
                            _ => 0,
                        };
                        
                        base_addr + (index_addr * scale) + disp8
                    },
                    5 => self.ebp + disp8,
                    6 => self.esi + disp8,
                    7 => self.edi + disp8,
                    _ => 0,
                }
            },
            2 => {
                // 32-bit displacement
                match rm_val {
                    0 => self.eax + self.displacement,
                    1 => self.ecx + self.displacement,
                    2 => self.edx + self.displacement,
                    3 => self.ebx + self.displacement,
                    4 => {
                        // SIB addressing with 32-bit displacement
                        let scale = 1 << ((self.sib >> 6) & 0x03);
                        let index = (self.sib >> 3) & 0x07;
                        let base = self.sib & 0x07;
                        
                        let base_addr = match base {
                            0 => self.eax,
                            1 => self.ecx,
                            2 => self.edx,
                            3 => self.ebx,
                            4 => self.esp,
                            5 => self.ebp,
                            6 => self.esi,
                            7 => self.edi,
                            _ => 0,
                        };
                        
                        let index_addr = match index {
                            0 => self.eax,
                            1 => self.ecx,
                            2 => self.edx,
                            3 => self.ebx,
                            4 => 0,
                            5 => self.ebp,
                            6 => self.esi,
                            7 => self.edi,
                            _ => 0,
                        };
                        
                        base_addr + (index_addr * scale) + self.displacement
                    },
                    5 => self.ebp + self.displacement,
                    6 => self.esi + self.displacement,
                    7 => self.edi + self.displacement,
                    _ => 0,
                }
            },
            3 => {
                // Register addressing
                match rm_val {
                    0 => self.eax,
                    1 => self.ecx,
                    2 => self.edx,
                    3 => self.ebx,
                    4 => self.esp,
                    5 => self.ebp,
                    6 => self.esi,
                    7 => self.edi,
                    _ => 0,
                }
            },
            _ => 0,
        }
    }
    
    fn get_register(&self, reg: u8) -> u32 {
        match reg {
            0 => self.eax,
            1 => self.ecx,
            2 => self.edx,
            3 => self.ebx,
            4 => self.esp,
            5 => self.ebp,
            6 => self.esi,
            7 => self.edi,
            _ => 0,
        }
    }
    
    fn set_register(&mut self, reg: u8, value: u32) {
        match reg {
            0 => self.eax = value,
            1 => self.ecx = value,
            2 => self.edx = value,
            3 => self.ebx = value,
            4 => self.esp = value,
            5 => self.ebp = value,
            6 => self.esi = value,
            7 => self.edi = value,
            _ => {}
        }
    }
    
    fn update_flags(&mut self, result: u32, carry: bool, overflow: bool) {
        // Zero flag
        if result == 0 {
            self.eflags |= 0x00000040;
        } else {
            self.eflags &= !0x00000040;
        }
        
        // Sign flag
        if (result & 0x80000000) != 0 {
            self.eflags |= 0x00000080;
        } else {
            self.eflags &= !0x00000080;
        }
        
        // Carry flag
        if carry {
            self.eflags |= 0x00000001;
        } else {
            self.eflags &= !0x00000001;
        }
        
        // Overflow flag
        if overflow {
            self.eflags |= 0x00000800;
        } else {
            self.eflags &= !0x00000800;
        }
    }
    
    fn execute_instruction(&mut self) -> EmuliteResult<()> {
        match self.opcode {
            // MOV instruction
            0x88 => { // MOV r/m8, r8
                let reg = (self.modrm >> 3) & 0x07;
                let rm = self.modrm & 0x07;
                let mod_val = (self.modrm >> 6) & 0x03;
                
                if mod_val == 3 {
                    // Register to register
                    let value = (self.get_register(reg) & 0xFF) as u8;
                    let mut dest = self.get_register(rm);
                    dest = (dest & 0xFFFFFF00) | (value as u32);
                    self.set_register(rm, dest);
                } else {
                    // Register to memory
                    let addr = self.get_effective_address();
                    let value = (self.get_register(reg) & 0xFF) as u8;
                    self.memory[addr as usize] = value;
                }
            },
            0x89 => { // MOV r/m32, r32
                let reg = (self.modrm >> 3) & 0x07;
                let rm = self.modrm & 0x07;
                let mod_val = (self.modrm >> 6) & 0x03;
                
                if mod_val == 3 {
                    // Register to register
                    let value = self.get_register(reg);
                    self.set_register(rm, value);
                } else {
                    // Register to memory
                    let addr = self.get_effective_address();
                    let value = self.get_register(reg);
                    self.write_memory_32(addr, value);
                }
            },
            0x8A => { // MOV r8, r/m8
                let reg = (self.modrm >> 3) & 0x07;
                let rm = self.modrm & 0x07;
                let mod_val = (self.modrm >> 6) & 0x03;
                
                if mod_val == 3 {
                    // Register to register
                    let value = (self.get_register(rm) & 0xFF) as u8;
                    let mut dest = self.get_register(reg);
                    dest = (dest & 0xFFFFFF00) | (value as u32);
                    self.set_register(reg, dest);
                } else {
                    // Memory to register
                    let addr = self.get_effective_address();
                    let value = self.memory[addr as usize];
                    let mut dest = self.get_register(reg);
                    dest = (dest & 0xFFFFFF00) | (value as u32);
                    self.set_register(reg, dest);
                }
            },
            0x8B => { // MOV r32, r/m32
                let reg = (self.modrm >> 3) & 0x07;
                let rm = self.modrm & 0x07;
                let mod_val = (self.modrm >> 6) & 0x03;
                
                if mod_val == 3 {
                    // Register to register
                    let value = self.get_register(rm);
                    self.set_register(reg, value);
                } else {
                    // Memory to register
                    let addr = self.get_effective_address();
                    let value = self.read_memory_32(addr);
                    self.set_register(reg, value);
                }
            },
            
            // ADD instruction
            0x00 => { // ADD r/m8, r8
                let reg = (self.modrm >> 3) & 0x07;
                let rm = self.modrm & 0x07;
                let mod_val = (self.modrm >> 6) & 0x03;
                
                let reg_value = (self.get_register(reg) & 0xFF) as u8;
                let rm_value = if mod_val == 3 {
                    (self.get_register(rm) & 0xFF) as u8
                } else {
                    let addr = self.get_effective_address();
                    self.memory[addr as usize]
                };
                
                let result = reg_value.wrapping_add(rm_value);
                let carry = (result as u16) < (reg_value as u16);
                let overflow = ((reg_value ^ result) & (rm_value ^ result) & 0x80) != 0;
                
                if mod_val == 3 {
                    let mut dest = self.get_register(rm);
                    dest = (dest & 0xFFFFFF00) | (result as u32);
                    self.set_register(rm, dest);
                } else {
                    let addr = self.get_effective_address();
                    self.memory[addr as usize] = result;
                }
                
                self.update_flags(result as u32, carry, overflow);
            },
            0x01 => { // ADD r/m32, r32
                let reg = (self.modrm >> 3) & 0x07;
                let rm = self.modrm & 0x07;
                let mod_val = (self.modrm >> 6) & 0x03;
                
                let reg_value = self.get_register(reg);
                let rm_value = if mod_val == 3 {
                    self.get_register(rm)
                } else {
                    let addr = self.get_effective_address();
                    self.read_memory_32(addr)
                };
                
                let result = reg_value.wrapping_add(rm_value);
                let carry = result < reg_value;
                let overflow = ((reg_value ^ result) & (rm_value ^ result) & 0x80000000) != 0;
                
                if mod_val == 3 {
                    self.set_register(rm, result);
                } else {
                    let addr = self.get_effective_address();
                    self.write_memory_32(addr, result);
                }
                
                self.update_flags(result, carry, overflow);
            },
            
            // SUB instruction
            0x28 => { // SUB r/m8, r8
                let reg = (self.modrm >> 3) & 0x07;
                let rm = self.modrm & 0x07;
                let mod_val = (self.modrm >> 6) & 0x03;
                
                let reg_value = (self.get_register(reg) & 0xFF) as u8;
                let rm_value = if mod_val == 3 {
                    (self.get_register(rm) & 0xFF) as u8
                } else {
                    let addr = self.get_effective_address();
                    self.memory[addr as usize]
                };
                
                let result = rm_value.wrapping_sub(reg_value);
                let carry = (rm_value as u16) < (reg_value as u16);
                let overflow = ((rm_value ^ reg_value) & (rm_value ^ result) & 0x80) != 0;
                
                if mod_val == 3 {
                    let mut dest = self.get_register(rm);
                    dest = (dest & 0xFFFFFF00) | (result as u32);
                    self.set_register(rm, dest);
                } else {
                    let addr = self.get_effective_address();
                    self.memory[addr as usize] = result;
                }
                
                self.update_flags(result as u32, carry, overflow);
            },
            0x29 => { // SUB r/m32, r32
                let reg = (self.modrm >> 3) & 0x07;
                let rm = self.modrm & 0x07;
                let mod_val = (self.modrm >> 6) & 0x03;
                
                let reg_value = self.get_register(reg);
                let rm_value = if mod_val == 3 {
                    self.get_register(rm)
                } else {
                    let addr = self.get_effective_address();
                    self.read_memory_32(addr)
                };
                
                let result = rm_value.wrapping_sub(reg_value);
                let carry = rm_value < reg_value;
                let overflow = ((rm_value ^ reg_value) & (rm_value ^ result) & 0x80000000) != 0;
                
                if mod_val == 3 {
                    self.set_register(rm, result);
                } else {
                    let addr = self.get_effective_address();
                    self.write_memory_32(addr, result);
                }
                
                self.update_flags(result, carry, overflow);
            },
            
            // JMP instruction
            0xE9 => { // JMP rel32
                let offset = self.immediate as i32;
                self.eip = self.eip.wrapping_add(offset as u32);
            },
            0xEB => { // JMP rel8
                let offset = self.immediate as i8 as i32;
                self.eip = self.eip.wrapping_add(offset as u32);
            },
            
            // CALL instruction
            0xE8 => { // CALL rel32
                let offset = self.immediate as i32;
                self.esp -= 4;
                self.write_memory_32(self.esp, self.eip);
                self.eip = self.eip.wrapping_add(offset as u32);
            },
            
            // RET instruction
            0xC3 => { // RET
                self.eip = self.read_memory_32(self.esp);
                self.esp += 4;
            },
            
            // NOP instruction
            0x90 => { // NOP
                // Do nothing
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

impl Cpu for X86 {
    fn step(&mut self) -> EmuliteResult<()> {
        if self.halted {
            return Ok(());
        }
        
        // Fetch instruction
        self.opcode = self.memory[self.eip as usize];
        self.eip += 1;
        
        // Decode ModR/M if needed
        if self.needs_modrm(self.opcode) {
            self.modrm = self.memory[self.eip as usize];
            self.eip += 1;
            
            // Decode SIB if needed
            if self.needs_sib(self.modrm) {
                self.sib = self.memory[self.eip as usize];
                self.eip += 1;
            }
            
            // Decode displacement
            let mod_val = (self.modrm >> 6) & 0x03;
            if mod_val == 1 {
                // 8-bit displacement
                self.displacement = self.memory[self.eip as usize] as u32;
                self.eip += 1;
            } else if mod_val == 2 {
                // 32-bit displacement
                self.displacement = self.read_memory_32(self.eip);
                self.eip += 4;
            }
        }
        
        // Decode immediate if needed
        if self.needs_immediate(self.opcode) {
            if self.is_32_bit_immediate(self.opcode) {
                self.immediate = self.read_memory_32(self.eip);
                self.eip += 4;
            } else {
                self.immediate = self.read_memory_16(self.eip) as u32;
                self.eip += 2;
            }
        }
        
        // Execute instruction
        self.execute_instruction()?;
        
        self.cycles += 1;
        Ok(())
    }
    
    fn reset(&mut self) -> EmuliteResult<()> {
        self.eax = 0;
        self.ebx = 0;
        self.ecx = 0;
        self.edx = 0;
        self.esi = 0;
        self.edi = 0;
        self.ebp = 0;
        self.esp = 0;
        self.cs = 0;
        self.ds = 0;
        self.es = 0;
        self.fs = 0;
        self.gs = 0;
        self.ss = 0;
        self.eip = self.read_memory_32(0xFFFFFFF0); // Reset vector
        self.eflags = 0x00000002; // Interrupt flag set
        self.memory.fill(0);
        self.cycles = 0;
        self.halted = false;
        Ok(())
    }
    
    fn name(&self) -> &str {
        "x86"
    }
    
    fn pc(&self) -> u32 {
        self.eip
    }
    
    fn set_pc(&mut self, pc: u32) -> EmuliteResult<()> {
        self.eip = pc;
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
            "eax" => Ok(self.eax),
            "ebx" => Ok(self.ebx),
            "ecx" => Ok(self.ecx),
            "edx" => Ok(self.edx),
            "esi" => Ok(self.esi),
            "edi" => Ok(self.edi),
            "ebp" => Ok(self.ebp),
            "esp" => Ok(self.esp),
            "cs" => Ok(self.cs as u32),
            "ds" => Ok(self.ds as u32),
            "es" => Ok(self.es as u32),
            "fs" => Ok(self.fs as u32),
            "gs" => Ok(self.gs as u32),
            "ss" => Ok(self.ss as u32),
            "eip" => Ok(self.eip),
            "eflags" => Ok(self.eflags),
            _ => Err(EmuliteError::CpuError(format!("Unknown register: {}", reg))),
        }
    }
    
    fn set_register(&mut self, reg: &str, value: u32) -> EmuliteResult<()> {
        match reg.to_lowercase().as_str() {
            "eax" => self.eax = value,
            "ebx" => self.ebx = value,
            "ecx" => self.ecx = value,
            "edx" => self.edx = value,
            "esi" => self.esi = value,
            "edi" => self.edi = value,
            "ebp" => self.ebp = value,
            "esp" => self.esp = value,
            "cs" => self.cs = value as u16,
            "ds" => self.ds = value as u16,
            "es" => self.es = value as u16,
            "fs" => self.fs = value as u16,
            "gs" => self.gs = value as u16,
            "ss" => self.ss = value as u16,
            "eip" => self.eip = value,
            "eflags" => self.eflags = value,
            _ => return Err(EmuliteError::CpuError(format!("Unknown register: {}", reg))),
        }
        Ok(())
    }
    
    fn get_registers(&self) -> HashMap<String, u32> {
        let mut registers = HashMap::new();
        registers.insert("eax".to_string(), self.eax);
        registers.insert("ebx".to_string(), self.ebx);
        registers.insert("ecx".to_string(), self.ecx);
        registers.insert("edx".to_string(), self.edx);
        registers.insert("esi".to_string(), self.esi);
        registers.insert("edi".to_string(), self.edi);
        registers.insert("ebp".to_string(), self.ebp);
        registers.insert("esp".to_string(), self.esp);
        registers.insert("cs".to_string(), self.cs as u32);
        registers.insert("ds".to_string(), self.ds as u32);
        registers.insert("es".to_string(), self.es as u32);
        registers.insert("fs".to_string(), self.fs as u32);
        registers.insert("gs".to_string(), self.gs as u32);
        registers.insert("ss".to_string(), self.ss as u32);
        registers.insert("eip".to_string(), self.eip);
        registers.insert("eflags".to_string(), self.eflags);
        registers
    }
    
    fn is_halted(&self) -> bool {
        self.halted
    }
    
    fn get_flags(&self) -> CpuFlags {
        CpuFlags {
            carry: (self.eflags & 0x00000001) != 0,
            zero: (self.eflags & 0x00000040) != 0,
            overflow: (self.eflags & 0x00000800) != 0,
            negative: (self.eflags & 0x00000080) != 0,
            extend: false, // Not used in x86
            interrupt_disable: (self.eflags & 0x00000200) != 0,
            decimal: false, // Not used in x86
            break_command: false, // Not used in x86
            sign: (self.eflags & 0x00000080) != 0,
            parity: (self.eflags & 0x00000004) != 0,
            auxiliary_carry: (self.eflags & 0x00000010) != 0,
        }
    }
    
    fn set_flags(&mut self, flags: CpuFlags) -> EmuliteResult<()> {
        if flags.carry {
            self.eflags |= 0x00000001;
        } else {
            self.eflags &= !0x00000001;
        }
        
        if flags.zero {
            self.eflags |= 0x00000040;
        } else {
            self.eflags &= !0x00000040;
        }
        
        if flags.overflow {
            self.eflags |= 0x00000800;
        } else {
            self.eflags &= !0x00000800;
        }
        
        if flags.negative {
            self.eflags |= 0x00000080;
        } else {
            self.eflags &= !0x00000080;
        }
        
        if flags.interrupt_disable {
            self.eflags |= 0x00000200;
        } else {
            self.eflags &= !0x00000200;
        }
        
        if flags.parity {
            self.eflags |= 0x00000004;
        } else {
            self.eflags &= !0x00000004;
        }
        
        if flags.auxiliary_carry {
            self.eflags |= 0x00000010;
        } else {
            self.eflags &= !0x00000010;
        }
        
        Ok(())
    }
    
    fn info(&self) -> CpuInfo {
        CpuInfo {
            name: "x86".to_string(),
            architecture: "x86".to_string(),
            bits: 32,
            endianness: Endianness::Little,
            register_count: 16,
            instruction_count: 256,
            clock_speed_hz: 100_000_000, // 100 MHz
        }
    }
    
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl X86 {
    fn needs_modrm(&self, opcode: u8) -> bool {
        // Simplified check - in reality this is more complex
        matches!(opcode, 0x00..=0x03 | 0x08..=0x0B | 0x20..=0x23 | 0x28..=0x2B | 0x80..=0x83 | 0x88..=0x8B)
    }
    
    fn needs_sib(&self, modrm: u8) -> bool {
        let rm = modrm & 0x07;
        rm == 4
    }
    
    fn needs_immediate(&self, opcode: u8) -> bool {
        matches!(opcode, 0xE8 | 0xE9 | 0xEB)
    }
    
    fn is_32_bit_immediate(&self, opcode: u8) -> bool {
        matches!(opcode, 0xE8 | 0xE9)
    }
}
