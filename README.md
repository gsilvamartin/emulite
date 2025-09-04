# Emulite - Multi-Platform Video Game Emulator

Emulite é um emulador de videogames multi-plataforma escrito em Rust, suportando sistemas desde o Atari 2600 até o PlayStation 3.

## Características

- **Multi-plataforma**: Suporte para múltiplas arquiteturas de CPU (6502, 68000, MIPS, x86, ARM)
- **Sistemas suportados**: Atari 2600, NES, SNES, PlayStation 1/2/3
- **Arquitetura modular**: Sistema extensível para adicionar novas plataformas
- **Alta performance**: Implementado em Rust para máxima eficiência
- **Sistema de debug**: Ferramentas avançadas de debugging e análise
- **Interface moderna**: Sistema de áudio e vídeo moderno com suporte a shaders

## Sistemas Suportados

### Atari 2600
- CPU: MOS 6507 (6502)
- Memória: 128 bytes RAM
- Resolução: 160x192
- Áudio: 2 canais

### Nintendo Entertainment System (NES)
- CPU: Ricoh 2A03 (6502)
- Memória: 2KB RAM
- Resolução: 256x240
- Áudio: 5 canais

### Super Nintendo Entertainment System (SNES)
- CPU: Ricoh 5A22 (65816)
- Memória: 128KB RAM
- Resolução: 256x224
- Áudio: 8 canais

### PlayStation
- CPU: MIPS R3000A
- Memória: 2MB RAM
- Resolução: 320x240
- Áudio: 24 canais

### PlayStation 2
- CPU: MIPS R5900 (Emotion Engine)
- Memória: 32MB RAM
- Resolução: 640x480
- Áudio: 48 canais

### PlayStation 3
- CPU: PowerPC Cell (PPE + 6 SPEs)
- Memória: 256MB RAM
- Resolução: 1920x1080
- Áudio: 64 canais

## Instalação

### Pré-requisitos

- Rust 1.70 ou superior
- Cargo
- OpenGL 3.3 ou superior
- ALSA (Linux) ou DirectSound (Windows)

### Compilação

```bash
git clone https://github.com/emulite/emulite.git
cd emulite
cargo build --release
```

### Execução

```bash
cargo run --release -- <rom_file> [platform]
```

Exemplos:
```bash
# Auto-detectar plataforma
cargo run --release -- game.nes

# Especificar plataforma
cargo run --release -- game.nes nes
cargo run --release -- game.iso ps1
```

## Configuração

O Emulite usa arquivos de configuração TOML para personalizar o comportamento:

### Localização dos arquivos de configuração

- **Linux**: `~/.config/emulite/config.toml`
- **Windows**: `%APPDATA%/emulite/config.toml`
- **macOS**: `~/Library/Application Support/emulite/config.toml`

### Exemplo de configuração

```toml
[audio]
enabled = true
sample_rate = 44100
buffer_size = 1024
volume = 0.8
channels = 2

[video]
width = 256
height = 240
fullscreen = false
vsync = true
scale = 3.0
filter = "nearest"
aspect_ratio = "4:3"

[input]
deadzone = 0.1
sensitivity = 1.0
auto_fire = false
turbo_speed = 10

[debug]
enabled = false
log_level = "info"
trace_execution = false
trace_memory = false
trace_instructions = false

[emulation]
speed = 1.0
frame_skip = 0
auto_save = false
save_interval = 300
rewind_enabled = false
rewind_frames = 300
region = "NTSC"

[ui]
theme = "dark"
show_fps = true
show_debug = false
show_controls = false
language = "en"
font_size = 12
```

## Controles

### Teclado (padrão)

- **Setas**: D-pad
- **Z**: Botão A
- **X**: Botão B
- **A**: Botão X (SNES/PS)
- **S**: Botão Y (SNES/PS)
- **Q**: Botão L (SNES/PS)
- **W**: Botão R (SNES/PS)
- **Enter**: Start
- **Espaço**: Select
- **Escape**: Sair

### Gamepad

Suporte completo para gamepads USB com mapeamento automático.

## Debugging

O Emulite inclui um sistema de debug avançado:

### Comandos de debug

- `break <address>`: Definir breakpoint
- `delete <address>`: Remover breakpoint
- `watch <address> [r|w|rw]`: Definir watchpoint
- `step`: Modo passo a passo
- `continue`: Continuar execução
- `registers`: Mostrar registradores
- `memory <address> [count]`: Mostrar memória
- `trace`: Mostrar trace de execução
- `info`: Informações da plataforma

### Exemplo de uso

```bash
# Iniciar com debug habilitado
cargo run --release -- --debug game.nes

# No console de debug:
> break 0x8000
> step
> registers
> memory 0x8000 16
> continue
```

## Desenvolvimento

### Estrutura do projeto

```
src/
├── lib.rs              # Biblioteca principal
├── main.rs             # Executável principal
├── core/               # Sistema central
├── platforms/          # Emuladores específicos
│   ├── atari2600.rs
│   ├── nes.rs
│   ├── snes.rs
│   ├── ps1.rs
│   ├── ps2.rs
│   └── ps3.rs
├── cpu/                # Emuladores de CPU
│   ├── mos6502.rs
│   ├── m68k.rs
│   ├── mips.rs
│   ├── x86.rs
│   └── arm.rs
├── memory/             # Sistema de memória
├── audio/              # Sistema de áudio
├── video/              # Sistema de vídeo
├── input/              # Sistema de input
├── debug/              # Sistema de debug
├── config/             # Sistema de configuração
└── utils/              # Utilitários
```

### Adicionando uma nova plataforma

1. Crie um novo arquivo em `src/platforms/`
2. Implemente o trait `Platform`
3. Adicione a plataforma ao `PlatformFactory`
4. Implemente os componentes específicos (CPU, memória, etc.)

### Exemplo de nova plataforma

```rust
use crate::platforms::{Platform, PlatformInfo};
use crate::{EmuliteResult, EmuliteError};

pub struct NovaPlataforma {
    // Componentes específicos
}

impl Platform for NovaPlataforma {
    fn load_rom(&mut self, rom_path: &str) -> EmuliteResult<()> {
        // Implementar carregamento de ROM
        Ok(())
    }
    
    fn step(&mut self) -> EmuliteResult<()> {
        // Implementar passo de emulação
        Ok(())
    }
    
    // ... outros métodos
}
```

## Testes

```bash
# Executar todos os testes
cargo test

# Executar testes específicos
cargo test --test cpu_tests
cargo test --test memory_tests

# Executar benchmarks
cargo bench
```

## Contribuição

1. Fork o projeto
2. Crie uma branch para sua feature (`git checkout -b feature/nova-feature`)
3. Commit suas mudanças (`git commit -am 'Adiciona nova feature'`)
4. Push para a branch (`git push origin feature/nova-feature`)
5. Abra um Pull Request

## Licença

Este projeto está licenciado sob a Licença MIT - veja o arquivo [LICENSE](LICENSE) para detalhes.

## Roadmap

- [ ] Suporte para mais plataformas (Game Boy, Sega Genesis, etc.)
- [ ] Interface gráfica moderna
- [ ] Suporte a netplay
- [ ] Sistema de achievements
- [ ] Suporte a shaders personalizados
- [ ] Emulação de periféricos (mouse, teclado, etc.)
- [ ] Suporte a save states
- [ ] Sistema de cheats
- [ ] Suporte a múltiplos monitores
- [ ] Emulação de rede

## Agradecimentos

- Comunidade Rust
- Desenvolvedores de emuladores existentes
- Documentação técnica dos sistemas originais
- Contribuidores do projeto

## Suporte

Para suporte, reportar bugs ou solicitar features:

- **Issues**: [GitHub Issues](https://github.com/emulite/emulite/issues)
- **Discord**: [Servidor Discord](https://discord.gg/emulite)
- **Email**: support@emulite.dev

---

**Emulite** - Emulando o passado, construindo o futuro.
