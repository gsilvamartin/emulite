//! Memory management and mapping system

use crate::{EmuliteResult, EmuliteError};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// Memory access permissions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MemoryAccess {
    ReadOnly,
    WriteOnly,
    ReadWrite,
    NoAccess,
}

/// Memory region information
#[derive(Debug, Clone)]
pub struct MemoryRegion {
    pub start: u32,
    pub end: u32,
    pub access: MemoryAccess,
    pub name: String,
    pub device: Option<String>,
}

/// Memory device trait for memory-mapped I/O
pub trait MemoryDevice: Send + Sync {
    /// Read from device memory
    fn read(&self, address: u32) -> EmuliteResult<u8>;
    
    /// Write to device memory
    fn write(&mut self, address: u32, value: u8) -> EmuliteResult<()>;
    
    /// Get device name
    fn name(&self) -> &str;
    
    /// Get memory range
    fn range(&self) -> (u32, u32);
    
    /// Reset device
    fn reset(&mut self) -> EmuliteResult<()>;
}

/// RAM memory device
pub struct RamDevice {
    data: Vec<u8>,
    start_address: u32,
    name: String,
}

impl RamDevice {
    pub fn new(size: usize, start_address: u32, name: String) -> Self {
        Self {
            data: vec![0; size],
            start_address,
            name,
        }
    }
}

impl MemoryDevice for RamDevice {
    fn read(&self, address: u32) -> EmuliteResult<u8> {
        let offset = address - self.start_address;
        if offset as usize >= self.data.len() {
            return Err(EmuliteError::MemoryAccessViolation(address));
        }
        Ok(self.data[offset as usize])
    }
    
    fn write(&mut self, address: u32, value: u8) -> EmuliteResult<()> {
        let offset = address - self.start_address;
        if offset as usize >= self.data.len() {
            return Err(EmuliteError::MemoryAccessViolation(address));
        }
        self.data[offset as usize] = value;
        Ok(())
    }
    
    fn name(&self) -> &str {
        &self.name
    }
    
    fn range(&self) -> (u32, u32) {
        (self.start_address, self.start_address + self.data.len() as u32 - 1)
    }
    
    fn reset(&mut self) -> EmuliteResult<()> {
        self.data.fill(0);
        Ok(())
    }
}

/// ROM memory device
pub struct RomDevice {
    data: Vec<u8>,
    start_address: u32,
    name: String,
}

impl RomDevice {
    pub fn new(data: Vec<u8>, start_address: u32, name: String) -> Self {
        Self {
            data,
            start_address,
            name,
        }
    }
    
    pub fn from_file(path: &str, start_address: u32, name: String) -> EmuliteResult<Self> {
        let data = std::fs::read(path)?;
        Ok(Self::new(data, start_address, name))
    }
}

impl MemoryDevice for RomDevice {
    fn read(&self, address: u32) -> EmuliteResult<u8> {
        let offset = address - self.start_address;
        if offset as usize >= self.data.len() {
            return Err(EmuliteError::MemoryAccessViolation(address));
        }
        Ok(self.data[offset as usize])
    }
    
    fn write(&mut self, _address: u32, _value: u8) -> EmuliteResult<()> {
        Err(EmuliteError::MemoryAccessViolation(_address))
    }
    
    fn name(&self) -> &str {
        &self.name
    }
    
    fn range(&self) -> (u32, u32) {
        (self.start_address, self.start_address + self.data.len() as u32 - 1)
    }
    
    fn reset(&mut self) -> EmuliteResult<()> {
        // ROM doesn't change
        Ok(())
    }
}

/// Memory mapper for handling memory access and device mapping
pub struct MemoryMapper {
    devices: HashMap<String, Arc<RwLock<dyn MemoryDevice>>>,
    regions: Vec<MemoryRegion>,
    address_space_size: u32,
}

impl MemoryMapper {
    pub fn new(address_space_size: u32) -> Self {
        Self {
            devices: HashMap::new(),
            regions: Vec::new(),
            address_space_size,
        }
    }
    
    /// Add a memory device to the mapper
    pub fn add_device(&mut self, name: String, device: Arc<RwLock<dyn MemoryDevice>>) -> EmuliteResult<()> {
        let (start, end) = device.read().unwrap().range();
        
        // Check for overlapping regions
        for region in &self.regions {
            if !(end < region.start || start > region.end) {
                return Err(EmuliteError::MemoryAccessViolation(start));
            }
        }
        
        // Add device
        self.devices.insert(name.clone(), device);
        
        // Add memory region
        self.regions.push(MemoryRegion {
            start,
            end,
            access: MemoryAccess::ReadWrite,
            name: name.clone(),
            device: Some(name),
        });
        
        // Sort regions by start address
        self.regions.sort_by_key(|r| r.start);
        
        Ok(())
    }
    
    /// Remove a memory device
    pub fn remove_device(&mut self, name: &str) -> EmuliteResult<()> {
        self.devices.remove(name);
        self.regions.retain(|r| r.name != name);
        Ok(())
    }
    
    /// Read from memory
    pub fn read(&self, address: u32) -> EmuliteResult<u8> {
        if address >= self.address_space_size {
            return Err(EmuliteError::MemoryAccessViolation(address));
        }
        
        // Find the region containing this address
        for region in &self.regions {
            if address >= region.start && address <= region.end {
                if region.access == MemoryAccess::WriteOnly || region.access == MemoryAccess::NoAccess {
                    return Err(EmuliteError::MemoryAccessViolation(address));
                }
                
                if let Some(device_name) = &region.device {
                    if let Some(device) = self.devices.get(device_name) {
                        return device.read().unwrap().read(address);
                    }
                }
            }
        }
        
        // No device mapped to this address
        Err(EmuliteError::MemoryAccessViolation(address))
    }
    
    /// Write to memory
    pub fn write(&mut self, address: u32, value: u8) -> EmuliteResult<()> {
        if address >= self.address_space_size {
            return Err(EmuliteError::MemoryAccessViolation(address));
        }
        
        // Find the region containing this address
        for region in &self.regions {
            if address >= region.start && address <= region.end {
                if region.access == MemoryAccess::ReadOnly || region.access == MemoryAccess::NoAccess {
                    return Err(EmuliteError::MemoryAccessViolation(address));
                }
                
                if let Some(device_name) = &region.device {
                    if let Some(device) = self.devices.get(device_name) {
                        return device.write().unwrap().write(address, value);
                    }
                }
            }
        }
        
        // No device mapped to this address
        Err(EmuliteError::MemoryAccessViolation(address))
    }
    
    /// Read 16-bit value (little-endian)
    pub fn read16(&self, address: u32) -> EmuliteResult<u16> {
        let low = self.read(address)?;
        let high = self.read(address + 1)?;
        Ok(u16::from_le_bytes([low, high]))
    }
    
    /// Write 16-bit value (little-endian)
    pub fn write16(&mut self, address: u32, value: u16) -> EmuliteResult<()> {
        let bytes = value.to_le_bytes();
        self.write(address, bytes[0])?;
        self.write(address + 1, bytes[1])?;
        Ok(())
    }
    
    /// Read 32-bit value (little-endian)
    pub fn read32(&self, address: u32) -> EmuliteResult<u32> {
        let low = self.read16(address)?;
        let high = self.read16(address + 2)?;
        Ok(u32::from_le_bytes([low as u8, (low >> 8) as u8, high as u8, (high >> 8) as u8]))
    }
    
    /// Write 32-bit value (little-endian)
    pub fn write32(&mut self, address: u32, value: u32) -> EmuliteResult<()> {
        let bytes = value.to_le_bytes();
        self.write(address, bytes[0])?;
        self.write(address + 1, bytes[1])?;
        self.write(address + 2, bytes[2])?;
        self.write(address + 3, bytes[3])?;
        Ok(())
    }
    
    /// Get memory regions
    pub fn regions(&self) -> &[MemoryRegion] {
        &self.regions
    }
    
    /// Get device by name
    pub fn get_device(&self, name: &str) -> Option<&Arc<RwLock<dyn MemoryDevice>>> {
        self.devices.get(name)
    }
    
    /// Reset all devices
    pub fn reset(&mut self) -> EmuliteResult<()> {
        for device in self.devices.values() {
            device.write().unwrap().reset()?;
        }
        Ok(())
    }
    
    /// Dump memory contents to file
    pub fn dump_memory(&self, path: &str) -> EmuliteResult<()> {
        let mut data = Vec::new();
        
        for region in &self.regions {
            for addr in region.start..=region.end {
                match self.read(addr) {
                    Ok(value) => data.push(value),
                    Err(_) => data.push(0xFF), // Fill with 0xFF for unmapped areas
                }
            }
        }
        
        std::fs::write(path, data)?;
        Ok(())
    }
}

/// Memory bank controller for systems with bank switching
pub struct MemoryBankController {
    banks: Vec<Vec<u8>>,
    current_bank: usize,
    bank_size: usize,
    total_banks: usize,
}

impl MemoryBankController {
    pub fn new(bank_size: usize, total_banks: usize) -> Self {
        Self {
            banks: vec![vec![0; bank_size]; total_banks],
            current_bank: 0,
            bank_size,
            total_banks,
        }
    }
    
    pub fn switch_bank(&mut self, bank: usize) -> EmuliteResult<()> {
        if bank >= self.total_banks {
            return Err(EmuliteError::MemoryAccessViolation(bank as u32));
        }
        self.current_bank = bank;
        Ok(())
    }
    
    pub fn read(&self, address: u32) -> EmuliteResult<u8> {
        let offset = address as usize;
        if offset >= self.bank_size {
            return Err(EmuliteError::MemoryAccessViolation(address));
        }
        Ok(self.banks[self.current_bank][offset])
    }
    
    pub fn write(&mut self, address: u32, value: u8) -> EmuliteResult<()> {
        let offset = address as usize;
        if offset >= self.bank_size {
            return Err(EmuliteError::MemoryAccessViolation(address));
        }
        self.banks[self.current_bank][offset] = value;
        Ok(())
    }
    
    pub fn load_bank(&mut self, bank: usize, data: Vec<u8>) -> EmuliteResult<()> {
        if bank >= self.total_banks {
            return Err(EmuliteError::MemoryAccessViolation(bank as u32));
        }
        if data.len() != self.bank_size {
            return Err(EmuliteError::MemoryAccessViolation(data.len() as u32));
        }
        self.banks[bank] = data;
        Ok(())
    }
}
