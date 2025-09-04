//! CPU performance benchmarks

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use emulite::cpu::{CpuFactory, Cpu};

fn bench_cpu_step(c: &mut Criterion) {
    let mut group = c.benchmark_group("cpu_step");
    
    // Benchmark 6502 CPU
    group.bench_function("6502_step", |b| {
        let mut cpu = CpuFactory::create("6502").unwrap();
        b.iter(|| {
            black_box(cpu.step().unwrap());
        });
    });
    
    // Benchmark 68000 CPU
    group.bench_function("68000_step", |b| {
        let mut cpu = CpuFactory::create("68000").unwrap();
        b.iter(|| {
            black_box(cpu.step().unwrap());
        });
    });
    
    // Benchmark MIPS CPU
    group.bench_function("mips_step", |b| {
        let mut cpu = CpuFactory::create("mips").unwrap();
        b.iter(|| {
            black_box(cpu.step().unwrap());
        });
    });
    
    // Benchmark x86 CPU
    group.bench_function("x86_step", |b| {
        let mut cpu = CpuFactory::create("x86").unwrap();
        b.iter(|| {
            black_box(cpu.step().unwrap());
        });
    });
    
    // Benchmark ARM CPU
    group.bench_function("arm_step", |b| {
        let mut cpu = CpuFactory::create("arm").unwrap();
        b.iter(|| {
            black_box(cpu.step().unwrap());
        });
    });
    
    group.finish();
}

fn bench_cpu_memory_access(c: &mut Criterion) {
    let mut group = c.benchmark_group("cpu_memory");
    
    // Benchmark memory read
    group.bench_function("memory_read", |b| {
        let mut cpu = CpuFactory::create("6502").unwrap();
        b.iter(|| {
            black_box(cpu.read_memory(black_box(0x1000)).unwrap());
        });
    });
    
    // Benchmark memory write
    group.bench_function("memory_write", |b| {
        let mut cpu = CpuFactory::create("6502").unwrap();
        b.iter(|| {
            black_box(cpu.write_memory(black_box(0x1000), black_box(0x42)).unwrap());
        });
    });
    
    group.finish();
}

fn bench_cpu_register_access(c: &mut Criterion) {
    let mut group = c.benchmark_group("cpu_registers");
    
    // Benchmark register read
    group.bench_function("register_read", |b| {
        let cpu = CpuFactory::create("6502").unwrap();
        b.iter(|| {
            black_box(cpu.get_register(black_box("a")).unwrap());
        });
    });
    
    // Benchmark register write
    group.bench_function("register_write", |b| {
        let mut cpu = CpuFactory::create("6502").unwrap();
        b.iter(|| {
            black_box(cpu.set_register(black_box("a"), black_box(0x42)).unwrap());
        });
    });
    
    // Benchmark get all registers
    group.bench_function("get_all_registers", |b| {
        let cpu = CpuFactory::create("6502").unwrap();
        b.iter(|| {
            black_box(cpu.get_registers());
        });
    });
    
    group.finish();
}

fn bench_cpu_flags(c: &mut Criterion) {
    let mut group = c.benchmark_group("cpu_flags");
    
    // Benchmark get flags
    group.bench_function("get_flags", |b| {
        let cpu = CpuFactory::create("6502").unwrap();
        b.iter(|| {
            black_box(cpu.get_flags());
        });
    });
    
    // Benchmark set flags
    group.bench_function("set_flags", |b| {
        let mut cpu = CpuFactory::create("6502").unwrap();
        let flags = cpu.get_flags();
        b.iter(|| {
            black_box(cpu.set_flags(black_box(flags)).unwrap());
        });
    });
    
    group.finish();
}

fn bench_cpu_reset(c: &mut Criterion) {
    let mut group = c.benchmark_group("cpu_reset");
    
    // Benchmark CPU reset
    group.bench_function("reset", |b| {
        let mut cpu = CpuFactory::create("6502").unwrap();
        b.iter(|| {
            black_box(cpu.reset().unwrap());
        });
    });
    
    group.finish();
}

fn bench_cpu_info(c: &mut Criterion) {
    let mut group = c.benchmark_group("cpu_info");
    
    // Benchmark CPU info
    group.bench_function("info", |b| {
        let cpu = CpuFactory::create("6502").unwrap();
        b.iter(|| {
            black_box(cpu.info());
        });
    });
    
    group.finish();
}

fn bench_cpu_creation(c: &mut Criterion) {
    let mut group = c.benchmark_group("cpu_creation");
    
    // Benchmark CPU creation
    group.bench_function("create_6502", |b| {
        b.iter(|| {
            black_box(CpuFactory::create("6502").unwrap());
        });
    });
    
    group.bench_function("create_68000", |b| {
        b.iter(|| {
            black_box(CpuFactory::create("68000").unwrap());
        });
    });
    
    group.bench_function("create_mips", |b| {
        b.iter(|| {
            black_box(CpuFactory::create("mips").unwrap());
        });
    });
    
    group.bench_function("create_x86", |b| {
        b.iter(|| {
            black_box(CpuFactory::create("x86").unwrap());
        });
    });
    
    group.bench_function("create_arm", |b| {
        b.iter(|| {
            black_box(CpuFactory::create("arm").unwrap());
        });
    });
    
    group.finish();
}

fn bench_cpu_instruction_cycles(c: &mut Criterion) {
    let mut group = c.benchmark_group("cpu_cycles");
    
    // Benchmark 1000 instruction cycles
    group.bench_function("1000_cycles_6502", |b| {
        let mut cpu = CpuFactory::create("6502").unwrap();
        b.iter(|| {
            for _ in 0..1000 {
                cpu.step().unwrap();
            }
        });
    });
    
    group.bench_function("1000_cycles_68000", |b| {
        let mut cpu = CpuFactory::create("68000").unwrap();
        b.iter(|| {
            for _ in 0..1000 {
                cpu.step().unwrap();
            }
        });
    });
    
    group.bench_function("1000_cycles_mips", |b| {
        let mut cpu = CpuFactory::create("mips").unwrap();
        b.iter(|| {
            for _ in 0..1000 {
                cpu.step().unwrap();
            }
        });
    });
    
    group.bench_function("1000_cycles_x86", |b| {
        let mut cpu = CpuFactory::create("x86").unwrap();
        b.iter(|| {
            for _ in 0..1000 {
                cpu.step().unwrap();
            }
        });
    });
    
    group.bench_function("1000_cycles_arm", |b| {
        let mut cpu = CpuFactory::create("arm").unwrap();
        b.iter(|| {
            for _ in 0..1000 {
                cpu.step().unwrap();
            }
        });
    });
    
    group.finish();
}

criterion_group!(
    benches,
    bench_cpu_step,
    bench_cpu_memory_access,
    bench_cpu_register_access,
    bench_cpu_flags,
    bench_cpu_reset,
    bench_cpu_info,
    bench_cpu_creation,
    bench_cpu_instruction_cycles
);

criterion_main!(benches);
