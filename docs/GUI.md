# Emulite GUI Documentation

Este documento descreve a interface gráfica do Emulite, construída com o framework egui.

## Visão Geral

A interface gráfica do Emulite é projetada para ser intuitiva e funcional, fornecendo todas as ferramentas necessárias para emulação, debug e configuração. A GUI é construída usando o framework egui, que oferece uma interface nativa e responsiva.

## Características Principais

### 1. Interface Moderna
- **Design Limpo**: Interface minimalista e profissional
- **Temas**: Múltiplos temas disponíveis (Default, Dark, Light, Retro, High Contrast)
- **Responsiva**: Adapta-se a diferentes tamanhos de tela
- **Nativa**: Performance nativa em todas as plataformas

### 2. Funcionalidades Principais
- **Carregamento de ROMs**: Suporte para múltiplos formatos
- **Controles de Emulação**: Start, Pause, Resume, Reset, Stop
- **Debug Avançado**: Visualizador de memória, registros, disassembler
- **Configurações**: Painel completo de configurações
- **Atalhos de Teclado**: Atalhos para todas as funções principais

## Estrutura da Interface

### 1. Barra de Menu
```
File | Emulation | View | Help
```

#### Menu File
- **Open ROM...** (Ctrl+O): Abre o navegador de ROMs
- **Recent ROMs**: Lista de ROMs recentes
- **Exit**: Fecha o aplicativo

#### Menu Emulation
- **Start** (F5): Inicia a emulação
- **Pause** (F5): Pausa a emulação
- **Resume** (F5): Retoma a emulação
- **Reset** (F6): Reinicia a emulação
- **Stop** (F7): Para a emulação

#### Menu View
- **Debug Panel** (F12): Alterna o painel de debug
- **Settings** (Ctrl+S): Abre as configurações

#### Menu Help
- **About**: Informações sobre o emulador

### 2. Área Principal
- **Tela de Boas-vindas**: Quando nenhuma ROM está carregada
- **Tela do Emulador**: Exibe a saída de vídeo do emulador
- **Aspecto 4:3**: Mantém a proporção correta da tela

### 3. Painel de Controles (Opcional)
- **Controles Principais**: Start, Pause, Reset, Stop
- **Controles Avançados**: Save State, Load State, Screenshot
- **Informações de Status**: ROM carregada, status, FPS, frames

### 4. Painel de Debug (Opcional)
- **Aba Registers**: Visualização de registros da CPU
- **Aba Memory**: Visualizador de memória
- **Aba Disassembler**: Desmontagem de código
- **Aba Breakpoints**: Gerenciamento de breakpoints

### 5. Barra de Status
- **ROM Carregada**: Caminho do arquivo ROM
- **Status**: Running, Paused, Stopped
- **FPS**: Taxa de quadros atual
- **Frames**: Contador de quadros
- **Mensagens**: Mensagens de status recentes

## Componentes da Interface

### 1. Navegador de ROMs
```rust
pub struct RomBrowser {
    current_path: String,
    selected_file: Option<String>,
    file_list: Vec<String>,
    filter_extensions: Vec<String>,
}
```

**Características:**
- Navegação por diretórios
- Filtro por extensões suportadas
- Visualização de arquivos
- Suporte para múltiplos formatos

**Formatos Suportados:**
- `.nes` - Nintendo Entertainment System
- `.smc`, `.sfc` - Super Nintendo
- `.bin`, `.rom` - ROMs genéricas
- `.iso`, `.cue` - CDs/DVDs

### 2. Painel de Configurações
```rust
pub struct SettingsPanel {
    selected_tab: String,
}
```

**Abas Disponíveis:**
- **General**: Configurações gerais
- **Audio**: Configurações de áudio
- **Video**: Configurações de vídeo
- **Input**: Configurações de entrada
- **Debug**: Configurações de debug

#### Configurações Gerais
- Auto-save state
- Diretório de save states
- Limite de ROMs recentes

#### Configurações de Áudio
- Taxa de amostragem (8kHz - 48kHz)
- Canais (Mono/Stereo)
- Tamanho do buffer
- Volume
- Habilitar/desabilitar áudio

#### Configurações de Vídeo
- Resolução
- Tela cheia
- VSync
- Filtro (None, Linear, Nearest)
- Caminho do shader

#### Configurações de Entrada
- Gravação de entrada
- Mapeamento de teclado
- Mapeamento de gamepad

#### Configurações de Debug
- Modo debug
- Nível de logging
- Breakpoints

### 3. Painel de Debug
```rust
pub struct DebugPanel {
    selected_tab: String,
    memory_viewer: MemoryViewer,
    register_viewer: RegisterViewer,
    disassembler: Disassembler,
    breakpoint_manager: BreakpointManager,
}
```

#### Visualizador de Memória
- Endereço inicial configurável
- Bytes por linha (8-32)
- Visualização hexadecimal e ASCII
- Navegação por scroll

#### Visualizador de Registros
- Registros da CPU em tempo real
- Valores em hexadecimal
- Atualização automática

#### Disassembler
- Endereço inicial configurável
- Número de instruções (10-200)
- Desmontagem em tempo real
- Navegação por scroll

#### Gerenciador de Breakpoints
- Adicionar/remover breakpoints
- Condições opcionais
- Habilitar/desabilitar breakpoints

### 4. Temas
```rust
pub enum ThemeType {
    Default,
    Dark,
    Light,
    Retro,
    HighContrast,
}
```

#### Tema Default
- Cores: Cinza escuro com azul
- Fonte: Proporcional
- Espaçamento: Médio

#### Tema Dark
- Cores: Preto com azul claro
- Fonte: Proporcional
- Espaçamento: Médio

#### Tema Light
- Cores: Branco com azul escuro
- Fonte: Proporcional
- Espaçamento: Médio

#### Tema Retro
- Cores: Preto com verde
- Fonte: Monospace
- Espaçamento: Pequeno

#### Tema High Contrast
- Cores: Preto e branco
- Fonte: Proporcional grande
- Espaçamento: Grande

## Atalhos de Teclado

### Atalhos Gerais
- **Ctrl+O**: Abrir ROM
- **Ctrl+S**: Configurações
- **Escape**: Fechar diálogos

### Atalhos de Emulação
- **F5**: Start/Pause/Resume
- **F6**: Reset
- **F7**: Stop

### Atalhos de Debug
- **F12**: Alternar painel de debug

## Eventos e Callbacks

### 1. Carregamento de ROM
```rust
pub fn load_rom(&mut self, rom_path: &str) -> EmuliteResult<()> {
    let rom_data = std::fs::read(rom_path)?;
    let platform_name = self.detect_platform(&rom_data)?;
    let platform = PlatformFactory::create(&platform_name)?;
    let mut emulator = Emulator::new(platform, self.config.clone());
    emulator.load_rom(&rom_data)?;
    self.emulator = Some(Arc::new(Mutex::new(Box::new(emulator))));
    self.current_rom_path = Some(rom_path.to_string());
    self.is_running = true;
    self.is_paused = false;
    Ok(())
}
```

### 2. Controles de Emulação
```rust
pub fn start(&mut self) -> EmuliteResult<()> {
    if let Some(emulator) = &self.emulator {
        let mut emu = emulator.lock().unwrap();
        emu.start()?;
        self.is_running = true;
        self.is_paused = false;
    }
    Ok(())
}
```

### 3. Atualização de FPS
```rust
pub fn update_fps(&mut self) {
    self.frame_count += 1;
    let now = std::time::Instant::now();
    let elapsed = now.duration_since(self.last_frame_time).as_secs_f32();
    
    if elapsed >= 1.0 {
        self.fps = self.frame_count as f32 / elapsed;
        self.frame_count = 0;
        self.last_frame_time = now;
    }
}
```

## Personalização

### 1. Temas Customizados
```rust
pub fn create_custom_theme(
    name: String,
    background: egui::Color32,
    foreground: egui::Color32,
    accent: egui::Color32,
) -> GuiTheme {
    GuiTheme {
        name,
        colors: ThemeColors {
            background,
            foreground,
            accent,
            error: egui::Color32::from_rgb(255, 100, 100),
            warning: egui::Color32::from_rgb(255, 200, 100),
            success: egui::Color32::from_rgb(100, 255, 100),
        },
        fonts: ThemeFonts {
            heading: egui::FontId::proportional(18.0),
            body: egui::FontId::proportional(14.0),
            monospace: egui::FontId::monospace(12.0),
        },
        spacing: ThemeSpacing {
            small: 4.0,
            medium: 8.0,
            large: 16.0,
        },
    }
}
```

### 2. Layout Personalizado
- Painéis redimensionáveis
- Layout salvo entre sessões
- Posições de janelas memorizadas

### 3. Configurações Persistentes
- Configurações salvas em `emulite.toml`
- Temas salvos
- Layout salvo
- ROMs recentes

## Performance

### 1. Otimizações
- Renderização eficiente com egui
- Atualização apenas quando necessário
- Texturas otimizadas para emulação

### 2. Responsividade
- Interface não bloqueante
- Atualizações em tempo real
- Feedback visual imediato

### 3. Memória
- Uso eficiente de memória
- Limpeza automática de recursos
- Gerenciamento de texturas

## Extensibilidade

### 1. Novos Widgets
```rust
pub trait CustomWidget {
    fn show(&mut self, ui: &mut egui::Ui, state: &mut GuiState);
    fn update(&mut self, state: &mut GuiState);
}
```

### 2. Novos Temas
```rust
pub trait ThemeProvider {
    fn get_theme(&self) -> GuiTheme;
    fn apply_theme(&self, ctx: &egui::Context);
}
```

### 3. Novos Diálogos
```rust
pub trait Dialog {
    fn show(&mut self, ctx: &egui::Context) -> bool;
    fn get_result(&self) -> Option<DialogResult>;
}
```

## Conclusão

A interface gráfica do Emulite oferece uma experiência completa e profissional para emulação de videogames. Com sua arquitetura modular e extensível, ela pode ser facilmente adaptada e expandida para atender às necessidades específicas dos usuários.

A combinação de funcionalidades avançadas, interface intuitiva e performance otimizada torna o Emulite uma ferramenta poderosa para entusiastas de emulação e desenvolvedores.
