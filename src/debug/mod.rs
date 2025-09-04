//! Debug system for emulator development and analysis

use crate::{EmuliteResult, EmuliteError, platforms::Platform, cpu::Cpu, config::DebugConfig};
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

/// Debug configuration
#[derive(Debug, Clone)]
pub struct DebuggerConfig {
    pub enabled: bool,
    pub log_level: LogLevel,
    pub log_file: Option<String>,
    pub breakpoints: Vec<u32>,
    pub watchpoints: Vec<Watchpoint>,
    pub trace_execution: bool,
    pub trace_memory: bool,
    pub trace_instructions: bool,
    pub max_trace_entries: usize,
}

impl Default for DebuggerConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            log_level: LogLevel::Info,
            log_file: None,
            breakpoints: Vec::new(),
            watchpoints: Vec::new(),
            trace_execution: false,
            trace_memory: false,
            trace_instructions: false,
            max_trace_entries: 10000,
        }
    }
}

/// Log levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

/// Watchpoint for memory monitoring
#[derive(Debug, Clone)]
pub struct Watchpoint {
    pub address: u32,
    pub size: usize,
    pub read: bool,
    pub write: bool,
    pub enabled: bool,
    pub hit_count: u64,
}

/// Debug event types
#[derive(Debug, Clone)]
pub enum DebugEvent {
    BreakpointHit { address: u32, instruction: String },
    WatchpointHit { address: u32, value: u8, is_write: bool },
    InstructionExecuted { address: u32, instruction: String, registers: HashMap<String, u32> },
    MemoryAccess { address: u32, value: u8, is_write: bool },
    CpuState { registers: HashMap<String, u32>, flags: String },
    PlatformState { info: String },
}

/// Debugger for emulator debugging
pub struct Debugger {
    config: DebuggerConfig,
    log_file: Option<File>,
    trace_buffer: Vec<DebugEvent>,
    breakpoint_hit: bool,
    step_mode: bool,
    instruction_count: u64,
    cycle_count: u64,
}

impl Debugger {
    pub fn new(config: &DebugConfig) -> EmuliteResult<Self> {
        let debugger_config = DebuggerConfig {
            enabled: config.enabled,
            log_level: match config.log_level.as_str() {
                "Error" => LogLevel::Error,
                "Warn" => LogLevel::Warn,
                "Info" => LogLevel::Info,
                "Debug" => LogLevel::Debug,
                "Trace" => LogLevel::Trace,
                _ => LogLevel::Info,
            },
            log_file: config.log_file.clone(),
            breakpoints: config.breakpoints.clone(),
            watchpoints: vec![],
            trace_execution: config.trace_execution,
            trace_memory: config.trace_memory,
            trace_instructions: config.trace_instructions,
            max_trace_entries: config.max_trace_entries,
        };
        
        Self::new_with_debugger_config(&debugger_config)
    }
    
    pub fn new_with_debugger_config(config: &DebuggerConfig) -> EmuliteResult<Self> {
        let mut debugger = Self {
            config: config.clone(),
            log_file: None,
            trace_buffer: Vec::new(),
            breakpoint_hit: false,
            step_mode: false,
            instruction_count: 0,
            cycle_count: 0,
        };
        
        // Open log file if specified
        if let Some(log_path) = &config.log_file {
            debugger.log_file = Some(File::create(log_path)?);
        }
        
        Ok(debugger)
    }
    
    /// Step debugger (called each emulation step)
    pub fn step(&mut self, platform: &dyn Platform) -> EmuliteResult<()> {
        if !self.config.enabled {
            return Ok(());
        }
        
        self.cycle_count += 1;
        
        // Check for breakpoints
        if let Some(cpu) = platform.get_cpu() {
            let pc = cpu.pc();
            if self.config.breakpoints.contains(&pc) {
                self.breakpoint_hit = true;
                self.log(LogLevel::Info, &format!("Breakpoint hit at 0x{:08X}", pc))?;
            }
        }
        
        // Trace execution if enabled
        if self.config.trace_execution {
            self.trace_execution(platform)?;
        }
        
        Ok(())
    }
    
    /// Trace execution
    fn trace_execution(&mut self, platform: &dyn Platform) -> EmuliteResult<()> {
        if let Some(cpu) = platform.get_cpu() {
            let pc = cpu.pc();
            let registers = cpu.get_registers();
            let flags = format!("{:?}", cpu.get_flags());
            
            let event = DebugEvent::InstructionExecuted {
                address: pc,
                instruction: format!("0x{:02X}", cpu.read_memory(pc).unwrap_or(0)),
                registers: registers.clone(),
            };
            
            self.add_trace_event(event);
            
            // Log instruction if enabled
            if self.config.trace_instructions {
                self.log(LogLevel::Trace, &format!(
                    "PC: 0x{:08X}, A: 0x{:02X}, X: 0x{:02X}, Y: 0x{:02X}, SP: 0x{:02X}, Flags: {}",
                    pc,
                    registers.get("a").copied().unwrap_or(0) as u8,
                    registers.get("x").copied().unwrap_or(0) as u8,
                    registers.get("y").copied().unwrap_or(0) as u8,
                    registers.get("sp").copied().unwrap_or(0) as u8,
                    flags
                ))?;
            }
        }
        
        Ok(())
    }
    
    /// Add trace event
    fn add_trace_event(&mut self, event: DebugEvent) {
        self.trace_buffer.push(event);
        
        // Keep buffer size manageable
        if self.trace_buffer.len() > self.config.max_trace_entries {
            self.trace_buffer.remove(0);
        }
    }
    
    /// Log message
    pub fn log(&mut self, level: LogLevel, message: &str) -> EmuliteResult<()> {
        if level < self.config.log_level {
            return Ok(());
        }
        
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let log_message = format!("[{}] [{}] {}: {}\n", 
            timestamp, 
            self.cycle_count,
            level.to_string(), 
            message
        );
        
        // Write to file if available
        if let Some(file) = &mut self.log_file {
            file.write_all(log_message.as_bytes())?;
            file.flush()?;
        }
        
        // Also log to console
        match level {
            LogLevel::Trace => log::trace!("{}", message),
            LogLevel::Debug => log::debug!("{}", message),
            LogLevel::Info => log::info!("{}", message),
            LogLevel::Warn => log::warn!("{}", message),
            LogLevel::Error => log::error!("{}", message),
        }
        
        Ok(())
    }
    
    /// Add breakpoint
    pub fn add_breakpoint(&mut self, address: u32) {
        if !self.config.breakpoints.contains(&address) {
            self.config.breakpoints.push(address);
            self.log(LogLevel::Info, &format!("Breakpoint added at 0x{:08X}", address)).ok();
        }
    }
    
    /// Remove breakpoint
    pub fn remove_breakpoint(&mut self, address: u32) {
        self.config.breakpoints.retain(|&addr| addr != address);
        self.log(LogLevel::Info, &format!("Breakpoint removed at 0x{:08X}", address)).ok();
    }
    
    /// Add watchpoint
    pub fn add_watchpoint(&mut self, address: u32, size: usize, read: bool, write: bool) {
        let watchpoint = Watchpoint {
            address,
            size,
            read,
            write,
            enabled: true,
            hit_count: 0,
        };
        
        self.config.watchpoints.push(watchpoint);
        self.log(LogLevel::Info, &format!(
            "Watchpoint added at 0x{:08X} (size: {}, read: {}, write: {})", 
            address, size, read, write
        )).ok();
    }
    
    /// Remove watchpoint
    pub fn remove_watchpoint(&mut self, address: u32) {
        self.config.watchpoints.retain(|wp| wp.address != address);
        self.log(LogLevel::Info, &format!("Watchpoint removed at 0x{:08X}", address)).ok();
    }
    
    /// Check if breakpoint was hit
    pub fn breakpoint_hit(&self) -> bool {
        self.breakpoint_hit
    }
    
    /// Clear breakpoint hit flag
    pub fn clear_breakpoint_hit(&mut self) {
        self.breakpoint_hit = false;
    }
    
    /// Enable/disable step mode
    pub fn set_step_mode(&mut self, enabled: bool) {
        self.step_mode = enabled;
        self.log(LogLevel::Info, &format!("Step mode {}", if enabled { "enabled" } else { "disabled" })).ok();
    }
    
    /// Check if in step mode
    pub fn step_mode(&self) -> bool {
        self.step_mode
    }
    
    /// Get trace buffer
    pub fn get_trace(&self) -> &[DebugEvent] {
        &self.trace_buffer
    }
    
    /// Clear trace buffer
    pub fn clear_trace(&mut self) {
        self.trace_buffer.clear();
        self.log(LogLevel::Info, "Trace buffer cleared").ok();
    }
    
    /// Export trace to file
    pub fn export_trace(&self, path: &str) -> EmuliteResult<()> {
        let mut file = File::create(path)?;
        
        for event in &self.trace_buffer {
            match event {
                DebugEvent::InstructionExecuted { address, instruction, registers } => {
                    writeln!(file, "PC: 0x{:08X}, Instruction: {}, Registers: {:?}", 
                        address, instruction, registers)?;
                },
                DebugEvent::MemoryAccess { address, value, is_write } => {
                    writeln!(file, "Memory {} at 0x{:08X}: 0x{:02X}", 
                        if *is_write { "write" } else { "read" }, address, value)?;
                },
                _ => {
                    writeln!(file, "{:?}", event)?;
                }
            }
        }
        
        Ok(())
    }
    
    /// Get instruction count
    pub fn instruction_count(&self) -> u64 {
        self.instruction_count
    }
    
    /// Get cycle count
    pub fn cycle_count(&self) -> u64 {
        self.cycle_count
    }
    
    /// Reset counters
    pub fn reset_counters(&mut self) {
        self.instruction_count = 0;
        self.cycle_count = 0;
        self.log(LogLevel::Info, "Counters reset").ok();
    }
    
    /// Get current configuration
    pub fn config(&self) -> &DebuggerConfig {
        &self.config
    }
    
    /// Update configuration
    pub fn update_config(&mut self, config: DebuggerConfig) -> EmuliteResult<()> {
        self.config = config;
        
        // Reopen log file if path changed
        if let Some(log_path) = &self.config.log_file {
            self.log_file = Some(File::create(log_path)?);
        }
        
        Ok(())
    }
}

impl LogLevel {
    fn to_string(&self) -> &'static str {
        match self {
            LogLevel::Trace => "TRACE",
            LogLevel::Debug => "DEBUG",
            LogLevel::Info => "INFO",
            LogLevel::Warn => "WARN",
            LogLevel::Error => "ERROR",
        }
    }
}

/// Debug commands for interactive debugging
pub struct DebugCommands;

impl DebugCommands {
    /// Execute debug command
    pub fn execute_command(debugger: &mut Debugger, command: &str, platform: &dyn Platform) -> EmuliteResult<String> {
        let parts: Vec<&str> = command.split_whitespace().collect();
        if parts.is_empty() {
            return Ok("No command specified".to_string());
        }
        
        match parts[0] {
            "help" => Ok(Self::help()),
            "break" | "b" => {
                if parts.len() < 2 {
                    return Ok("Usage: break <address>".to_string());
                }
                let address = u32::from_str_radix(parts[1].trim_start_matches("0x"), 16)
                    .map_err(|_| EmuliteError::ConfigError("Invalid address".to_string()))?;
                debugger.add_breakpoint(address);
                Ok(format!("Breakpoint set at 0x{:08X}", address))
            },
            "delete" | "d" => {
                if parts.len() < 2 {
                    return Ok("Usage: delete <address>".to_string());
                }
                let address = u32::from_str_radix(parts[1].trim_start_matches("0x"), 16)
                    .map_err(|_| EmuliteError::ConfigError("Invalid address".to_string()))?;
                debugger.remove_breakpoint(address);
                Ok(format!("Breakpoint removed at 0x{:08X}", address))
            },
            "watch" | "w" => {
                if parts.len() < 2 {
                    return Ok("Usage: watch <address> [r|w|rw]".to_string());
                }
                let address = u32::from_str_radix(parts[1].trim_start_matches("0x"), 16)
                    .map_err(|_| EmuliteError::ConfigError("Invalid address".to_string()))?;
                let mode = parts.get(2).unwrap_or(&"rw");
                let (read, write) = match *mode {
                    "r" => (true, false),
                    "w" => (false, true),
                    "rw" => (true, true),
                    _ => (true, true),
                };
                debugger.add_watchpoint(address, 1, read, write);
                Ok(format!("Watchpoint set at 0x{:08X}", address))
            },
            "step" | "s" => {
                debugger.set_step_mode(true);
                Ok("Step mode enabled".to_string())
            },
            "continue" | "c" => {
                debugger.set_step_mode(false);
                debugger.clear_breakpoint_hit();
                Ok("Continuing execution".to_string())
            },
            "registers" | "r" => {
                if let Some(cpu) = platform.get_cpu() {
                    let registers = cpu.get_registers();
                    let mut result = String::new();
                    for (name, value) in registers {
                        result.push_str(&format!("{}: 0x{:08X}\n", name, value));
                    }
                    Ok(result)
                } else {
                    Ok("No CPU available".to_string())
                }
            },
            "memory" | "m" => {
                if parts.len() < 2 {
                    return Ok("Usage: memory <address> [count]".to_string());
                }
                let address = u32::from_str_radix(parts[1].trim_start_matches("0x"), 16)
                    .map_err(|_| EmuliteError::ConfigError("Invalid address".to_string()))?;
                let count = parts.get(2).and_then(|s| s.parse().ok()).unwrap_or(16);
                
                if let Some(cpu) = platform.get_cpu() {
                    let mut result = String::new();
                    for i in 0..count {
                        let addr = address + i;
                        match cpu.read_memory(addr) {
                            Ok(value) => result.push_str(&format!("0x{:08X}: 0x{:02X}\n", addr, value)),
                            Err(_) => result.push_str(&format!("0x{:08X}: <error>\n", addr)),
                        }
                    }
                    Ok(result)
                } else {
                    Ok("No CPU available".to_string())
                }
            },
            "trace" | "t" => {
                let trace = debugger.get_trace();
                let mut result = String::new();
                for event in trace.iter().rev().take(20) {
                    match event {
                        DebugEvent::InstructionExecuted { address, instruction, .. } => {
                            result.push_str(&format!("PC: 0x{:08X}, Instruction: {}\n", address, instruction));
                        },
                        _ => {}
                    }
                }
                Ok(result)
            },
            "info" | "i" => {
                let info = platform.info();
                Ok(format!(
                    "Platform: {}\nVersion: {}\nCPU: {}\nMemory: {} bytes\nResolution: {}x{}\nAudio: {} channels",
                    info.name, info.version, info.cpu_type, info.memory_size,
                    info.video_resolution.0, info.video_resolution.1, info.audio_channels
                ))
            },
            _ => Ok(format!("Unknown command: {}. Type 'help' for available commands.", parts[0]))
        }
    }
    
    fn help() -> String {
        r#"Available debug commands:
  help, h          - Show this help
  break <addr>, b  - Set breakpoint at address
  delete <addr>, d - Remove breakpoint at address
  watch <addr> [r|w|rw], w - Set watchpoint at address
  step, s          - Enable step mode
  continue, c      - Continue execution
  registers, r     - Show CPU registers
  memory <addr> [count], m - Show memory contents
  trace, t         - Show execution trace
  info, i          - Show platform information
"#.to_string()
    }
}

/// Extension trait for platforms to support debugging
pub trait DebuggablePlatform: Platform {
    fn get_cpu(&self) -> Option<&dyn Cpu>;
    fn get_cpu_mut(&mut self) -> Option<&mut dyn Cpu>;
}
