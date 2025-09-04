//! Debug system usage example

use emulite::{Emulator, Config, EmuliteResult};
use emulite::debug::{Debugger, DebugConfig, DebugCommands};
use std::env;

fn main() -> EmuliteResult<()> {
    // Initialize logging
    env_logger::init();
    
    println!("Emulite Debug Example");
    
    // Parse command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <rom_file>", args[0]);
        return Ok(());
    }
    
    let rom_path = &args[1];
    
    // Create debug configuration
    let mut debug_config = DebugConfig::default();
    debug_config.enabled = true;
    debug_config.log_level = emulite::debug::LogLevel::Debug;
    debug_config.trace_execution = true;
    debug_config.trace_instructions = true;
    debug_config.max_trace_entries = 1000;
    
    // Load main configuration
    let mut config = Config::load()?;
    config.debug = debug_config;
    
    // Create emulator
    let mut emulator = Emulator::new(config)?;
    
    // Load ROM
    emulator.load_rom(rom_path, "auto")?;
    println!("ROM loaded: {}", rom_path);
    
    // Create debugger
    let mut debugger = Debugger::new(&emulator.config().debug)?;
    
    // Add some breakpoints
    debugger.add_breakpoint(0x8000);
    debugger.add_breakpoint(0x8100);
    
    // Add watchpoints
    debugger.add_watchpoint(0x2000, 1, true, true); // Watch memory at 0x2000
    
    // Enable step mode
    debugger.set_step_mode(true);
    
    println!("Debugger initialized with breakpoints and watchpoints");
    
    // Run emulator with debugging
    let mut step_count = 0;
    let max_steps = 10000;
    
    while step_count < max_steps {
        // Check for breakpoints
        if debugger.breakpoint_hit() {
            println!("Breakpoint hit at step {}", step_count);
            debugger.clear_breakpoint_hit();
            
            // Show current state
            let platform = emulator.platform_info();
            if let Some(cpu) = platform.get_cpu() {
                println!("PC: 0x{:08X}", cpu.pc());
                println!("Registers: {:?}", cpu.get_registers());
                println!("Flags: {:?}", cpu.get_flags());
            }
            
            // Show memory around PC
            if let Some(cpu) = platform.get_cpu() {
                let pc = cpu.pc();
                println!("Memory around PC (0x{:08X}):", pc);
                for i in 0..16 {
                    let addr = pc.wrapping_add(i);
                    match cpu.read_memory(addr) {
                        Ok(value) => print!("{:02X} ", value),
                        Err(_) => print!("?? "),
                    }
                }
                println!();
            }
            
            // Wait for user input (in a real implementation)
            println!("Press Enter to continue...");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
        }
        
        // Step emulator
        emulator.step()?;
        
        // Step debugger
        debugger.step(emulator.platform_info())?;
        
        step_count += 1;
        
        // Show progress
        if step_count % 1000 == 0 {
            println!("Step {}: PC = 0x{:08X}", step_count, 
                emulator.platform_info().get_cpu().map(|c| c.pc()).unwrap_or(0));
        }
    }
    
    println!("Debug session completed after {} steps", step_count);
    
    // Show final statistics
    println!("Final statistics:");
    println!("  Instructions executed: {}", debugger.instruction_count());
    println!("  Cycles: {}", debugger.cycle_count());
    println!("  Trace entries: {}", debugger.get_trace().len());
    
    // Export trace
    debugger.export_trace("debug_trace.txt")?;
    println!("Trace exported to debug_trace.txt");
    
    Ok(())
}
