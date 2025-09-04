# Emulite Development Guide

Este documento fornece um guia completo para desenvolvimento no Emulite.

## Visão Geral

O Emulite é um projeto complexo que requer conhecimento em várias áreas: emulação de hardware, Rust, gráficos, áudio e desenvolvimento de software. Este guia fornece as informações necessárias para contribuir com o projeto.

## Pré-requisitos

### 1. Ferramentas Necessárias

- **Rust**: Versão 1.70 ou superior
- **Cargo**: Gerenciador de pacotes do Rust
- **Git**: Controle de versão
- **Make**: Para comandos de build (opcional)
- **Clang**: Para compilação de shaders (opcional)

### 2. Conhecimento Necessário

- **Rust**: Linguagem de programação principal
- **Emulação**: Conceitos de emulação de hardware
- **Gráficos**: OpenGL/Vulkan/WGPU
- **Áudio**: Processamento de áudio digital
- **Arquitetura de Computadores**: CPUs, memória, I/O

## Configuração do Ambiente

### 1. Instalação do Rust

```bash
# Instalar Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Configurar PATH
source ~/.cargo/env

# Verificar instalação
rustc --version
cargo --version
```

### 2. Configuração do Projeto

```bash
# Clonar o repositório
git clone https://github.com/emulite/emulite.git
cd emulite

# Instalar dependências
cargo build

# Executar testes
cargo test

# Executar benchmarks
cargo bench
```

### 3. Configuração do Editor

#### VS Code

```json
// .vscode/settings.json
{
    "rust-analyzer.cargo.features": "all",
    "rust-analyzer.checkOnSave.command": "clippy",
    "rust-analyzer.checkOnSave.extraArgs": ["--", "-W", "clippy::all"],
    "rust-analyzer.procMacro.enable": true,
    "rust-analyzer.cargo.loadOutDirsFromCheck": true
}
```

#### Vim/Neovim

```vim
" .vimrc
Plug 'rust-lang/rust.vim'
Plug 'neoclide/coc.nvim'
Plug 'neoclide/coc-rust-analyzer'
```

## Estrutura do Projeto

### 1. Diretórios Principais

```
emulite/
├── src/                    # Código fonte
│   ├── core/              # Sistema central
│   ├── platforms/         # Implementações de plataformas
│   ├── cpu/               # Implementações de CPU
│   ├── memory/            # Sistema de memória
│   ├── audio/             # Sistema de áudio
│   ├── video/             # Sistema de vídeo
│   ├── input/             # Sistema de entrada
│   ├── debug/             # Sistema de debug
│   ├── config/            # Sistema de configuração
│   └── utils/             # Utilitários
├── tests/                 # Testes de integração
├── benches/               # Benchmarks
├── examples/              # Exemplos de uso
├── docs/                  # Documentação
├── .github/               # GitHub Actions
└── assets/                # Recursos (shaders, etc.)
```

### 2. Arquivos de Configuração

- **Cargo.toml**: Configuração do projeto e dependências
- **Cargo.lock**: Versões exatas das dependências
- **rustfmt.toml**: Configuração do formatador
- **clippy.toml**: Configuração do linter
- **Makefile**: Comandos de build e desenvolvimento

## Desenvolvimento

### 1. Fluxo de Desenvolvimento

```bash
# 1. Criar branch para feature
git checkout -b feature/nova-funcionalidade

# 2. Fazer alterações
# ... editar código ...

# 3. Executar testes
cargo test

# 4. Executar linter
cargo clippy

# 5. Formatar código
cargo fmt

# 6. Commit
git add .
git commit -m "feat: adicionar nova funcionalidade"

# 7. Push
git push origin feature/nova-funcionalidade

# 8. Criar Pull Request
```

### 2. Convenções de Código

#### Nomenclatura

```rust
// Structs e enums: PascalCase
pub struct Emulator;
pub enum PlatformType;

// Funções e variáveis: snake_case
pub fn load_rom();
let mut emulator_state;

// Constantes: SCREAMING_SNAKE_CASE
pub const MAX_MEMORY_SIZE: usize = 0x10000;

// Traits: PascalCase com sufixo "Trait" (opcional)
pub trait CpuTrait;
pub trait MemoryBus;
```

#### Documentação

```rust
/// Executa um ciclo de CPU
/// 
/// # Arguments
/// 
/// * `cycles` - Número de ciclos a executar
/// 
/// # Returns
/// 
/// * `EmuliteResult<()>` - Resultado da execução
/// 
/// # Errors
/// 
/// * `CpuError` - Erro na execução da CPU
/// 
/// # Examples
/// 
/// ```rust
/// let mut cpu = Cpu::new();
/// cpu.step(1)?;
/// ```
pub fn step(&mut self, cycles: u32) -> EmuliteResult<()> {
    // Implementação
}
```

#### Tratamento de Erros

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum EmuliteError {
    #[error("ROM not found: {0}")]
    RomNotFound(String),
    
    #[error("Invalid ROM format: {0}")]
    InvalidRomFormat(String),
    
    #[error("Platform not supported: {0}")]
    PlatformNotSupported(String),
}

pub type EmuliteResult<T> = Result<T, EmuliteError>;
```

### 3. Testes

#### Testes Unitários

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_cpu_step() {
        let mut cpu = Cpu::new();
        assert!(cpu.step(1).is_ok());
    }
    
    #[test]
    fn test_memory_read_write() {
        let mut memory = Memory::new(1024);
        memory.write_byte(0x100, 0x42).unwrap();
        assert_eq!(memory.read_byte(0x100).unwrap(), 0x42);
    }
}
```

#### Testes de Integração

```rust
// tests/integration_tests.rs
use emulite::*;

#[test]
fn test_emulator_lifecycle() {
    let platform = PlatformFactory::create("nes").unwrap();
    let config = Config::default();
    let mut emulator = Emulator::new(platform, config);
    
    // Testar carregamento de ROM
    let rom_data = vec![0x00; 1024];
    assert!(emulator.load_rom(&rom_data).is_ok());
    
    // Testar execução
    assert!(emulator.run().is_ok());
}
```

#### Benchmarks

```rust
// benches/cpu_bench.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use emulite::*;

fn bench_cpu_step(c: &mut Criterion) {
    let mut cpu = Cpu::new();
    
    c.bench_function("cpu_step", |b| {
        b.iter(|| {
            cpu.step(black_box(1))
        })
    });
}

criterion_group!(benches, bench_cpu_step);
criterion_main!(benches);
```

### 4. Debugging

#### Logging

```rust
use log::{debug, info, warn, error};

fn debug_example() {
    debug!("Debug message");
    info!("Info message");
    warn!("Warning message");
    error!("Error message");
}
```

#### Breakpoints

```rust
use emulite::*;

fn debug_with_breakpoints() -> EmuliteResult<()> {
    let mut emulator = Emulator::new(platform, config);
    
    // Adicionar breakpoint
    let bp = Breakpoint {
        address: 0x8000,
        condition: None,
        enabled: true,
    };
    emulator.debugger.as_mut().unwrap().add_breakpoint(bp)?;
    
    // Executar até breakpoint
    emulator.debugger.as_mut().unwrap().continue_execution()?;
    
    // Inspecionar estado
    let registers = emulator.debugger.as_ref().unwrap().get_registers();
    println!("Registers: {:?}", registers);
    
    Ok(())
}
```

## Contribuição

### 1. Como Contribuir

1. **Fork** o repositório
2. **Clone** seu fork
3. **Crie** uma branch para sua feature
4. **Faça** suas alterações
5. **Teste** suas alterações
6. **Commit** suas alterações
7. **Push** para seu fork
8. **Crie** um Pull Request

### 2. Diretrizes de Contribuição

- **Código limpo**: Siga as convenções de código
- **Testes**: Adicione testes para novas funcionalidades
- **Documentação**: Documente APIs públicas
- **Performance**: Considere o impacto na performance
- **Compatibilidade**: Mantenha compatibilidade com versões anteriores

### 3. Tipos de Contribuição

- **Bug fixes**: Correção de bugs
- **Features**: Novas funcionalidades
- **Documentação**: Melhoria da documentação
- **Testes**: Adição de testes
- **Performance**: Otimizações de performance
- **Refactoring**: Refatoração de código

## Release

### 1. Versionamento

O Emulite segue o [Semantic Versioning](https://semver.org/):

- **MAJOR**: Mudanças incompatíveis na API
- **MINOR**: Novas funcionalidades compatíveis
- **PATCH**: Correções de bugs compatíveis

### 2. Processo de Release

1. **Atualizar** versão no Cargo.toml
2. **Atualizar** CHANGELOG.md
3. **Criar** tag de release
4. **Publicar** no crates.io
5. **Criar** release no GitHub

### 3. CI/CD

O projeto usa GitHub Actions para:

- **Build**: Compilação em múltiplas plataformas
- **Testes**: Execução de testes
- **Linting**: Verificação de código
- **Formatting**: Formatação de código
- **Release**: Publicação automática

## Conclusão

Este guia fornece as informações necessárias para contribuir com o desenvolvimento do Emulite. Para mais informações, consulte a documentação específica de cada módulo ou entre em contato com a equipe de desenvolvimento.
