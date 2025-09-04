//! CPU testing example

use emulite::cpu::{CpuFactory, Cpu};
use emulite::EmuliteResult;

fn test_cpu_basic_operations(cpu_type: &str) -> EmuliteResult<()> {
    println!("Testing {} CPU basic operations", cpu_type);
    
    let mut cpu = CpuFactory::create(cpu_type)?;
    
    // Test reset
    cpu.reset()?;
    println!("  Reset: OK");
    
    // Test memory access
    cpu.write_memory(0x1000, 0x42)?;
    let value = cpu.read_memory(0x1000)?;
    assert_eq!(value, 0x42);
    println!("  Memory access: OK");
    
    // Test register access
    cpu.set_register("a", 0x1234)?;
    let reg_value = cpu.get_register("a")?;
    assert_eq!(reg_value, 0x1234);
    println!("  Register access: OK");
    
    // Test instruction execution
    for _ in 0..100 {
        cpu.step()?;
    }
    println!("  Instruction execution: OK");
    
    // Test CPU info
    let info = cpu.info();
    println!("  CPU Info: {} ({} bits, {} Hz)", 
        info.name, info.bits, info.clock_speed_hz);
    
    Ok(())
}

fn test_cpu_performance(cpu_type: &str, iterations: usize) -> EmuliteResult<()> {
    println!("Testing {} CPU performance ({} iterations)", cpu_type, iterations);
    
    let mut cpu = CpuFactory::create(cpu_type)?;
    
    let start = std::time::Instant::now();
    
    for _ in 0..iterations {
        cpu.step()?;
    }
    
    let duration = start.elapsed();
    let instructions_per_second = iterations as f64 / duration.as_secs_f64();
    
    println!("  Performance: {:.2} instructions/second", instructions_per_second);
    println!("  Duration: {:?}", duration);
    
    Ok(())
}

fn test_cpu_memory_bandwidth(cpu_type: &str, iterations: usize) -> EmuliteResult<()> {
    println!("Testing {} CPU memory bandwidth ({} iterations)", cpu_type, iterations);
    
    let mut cpu = CpuFactory::create(cpu_type)?;
    
    // Test memory write bandwidth
    let start = std::time::Instant::now();
    for i in 0..iterations {
        cpu.write_memory(i as u32, (i & 0xFF) as u8)?;
    }
    let write_duration = start.elapsed();
    let write_bandwidth = (iterations as f64 * 1.0) / write_duration.as_secs_f64();
    
    // Test memory read bandwidth
    let start = std::time::Instant::now();
    for i in 0..iterations {
        let _ = cpu.read_memory(i as u32)?;
    }
    let read_duration = start.elapsed();
    let read_bandwidth = (iterations as f64 * 1.0) / read_duration.as_secs_f64();
    
    println!("  Write bandwidth: {:.2} bytes/second", write_bandwidth);
    println!("  Read bandwidth: {:.2} bytes/second", read_bandwidth);
    
    Ok(())
}

fn test_cpu_register_bandwidth(cpu_type: &str, iterations: usize) -> EmuliteResult<()> {
    println!("Testing {} CPU register bandwidth ({} iterations)", cpu_type, iterations);
    
    let mut cpu = CpuFactory::create(cpu_type)?;
    
    // Test register write bandwidth
    let start = std::time::Instant::now();
    for i in 0..iterations {
        cpu.set_register("a", i as u32)?;
    }
    let write_duration = start.elapsed();
    let write_bandwidth = (iterations as f64 * 4.0) / write_duration.as_secs_f64();
    
    // Test register read bandwidth
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        let _ = cpu.get_register("a")?;
    }
    let read_duration = start.elapsed();
    let read_bandwidth = (iterations as f64 * 4.0) / read_duration.as_secs_f64();
    
    println!("  Register write bandwidth: {:.2} bytes/second", write_bandwidth);
    println!("  Register read bandwidth: {:.2} bytes/second", read_bandwidth);
    
    Ok(())
}

fn main() -> EmuliteResult<()> {
    println!("Emulite CPU Testing Example");
    println!("==========================");
    
    let cpu_types = ["6502", "68000", "mips", "x86", "arm"];
    
    for cpu_type in &cpu_types {
        println!("\n--- Testing {} CPU ---", cpu_type);
        
        // Basic operations test
        test_cpu_basic_operations(cpu_type)?;
        
        // Performance test
        test_cpu_performance(cpu_type, 10000)?;
        
        // Memory bandwidth test
        test_cpu_memory_bandwidth(cpu_type, 10000)?;
        
        // Register bandwidth test
        test_cpu_register_bandwidth(cpu_type, 10000)?;
    }
    
    println!("\nAll CPU tests completed successfully!");
    
    Ok(())
}
