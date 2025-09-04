# Contributing to Emulite

Obrigado por considerar contribuir com o Emulite! Este documento fornece diretrizes e informações para contribuidores.

## Código de Conduta

Este projeto adere ao [Código de Conduta do Rust](https://www.rust-lang.org/policies/code-of-conduct). Ao participar, você concorda em manter este código.

## Como Contribuir

### 1. Reportando Bugs

Antes de reportar um bug, verifique se:

- O bug não foi reportado anteriormente
- Você está usando a versão mais recente
- O bug é reproduzível

Ao reportar um bug, inclua:

- **Descrição clara** do problema
- **Passos para reproduzir** o bug
- **Comportamento esperado** vs **comportamento atual**
- **Ambiente**: Sistema operacional, versão do Rust, etc.
- **Logs** relevantes (se houver)

### 2. Sugerindo Melhorias

Para sugerir melhorias:

- **Descreva** a melhoria claramente
- **Explique** por que seria útil
- **Forneça** exemplos de uso
- **Considere** alternativas

### 3. Contribuindo com Código

#### Processo de Contribuição

1. **Fork** o repositório
2. **Clone** seu fork
3. **Crie** uma branch para sua feature
4. **Faça** suas alterações
5. **Teste** suas alterações
6. **Commit** suas alterações
7. **Push** para seu fork
8. **Crie** um Pull Request

#### Convenções de Código

- **Rust**: Siga as convenções padrão do Rust
- **Nomenclatura**: Use snake_case para funções e variáveis, PascalCase para tipos
- **Documentação**: Documente APIs públicas
- **Testes**: Adicione testes para novas funcionalidades
- **Performance**: Considere o impacto na performance

#### Exemplo de Commit

```bash
git commit -m "feat: adicionar suporte para PlayStation 2

- Implementar CPU MIPS R5900
- Adicionar sistema de memória de 32MB
- Implementar GPU Graphics Synthesizer
- Adicionar testes para PS2

Closes #123"
```

#### Tipos de Commit

- **feat**: Nova funcionalidade
- **fix**: Correção de bug
- **docs**: Documentação
- **style**: Formatação, sem mudança de código
- **refactor**: Refatoração de código
- **test**: Adição de testes
- **chore**: Tarefas de manutenção

### 4. Pull Requests

#### Antes de Enviar

- [ ] Código compila sem warnings
- [ ] Todos os testes passam
- [ ] Código está formatado (`cargo fmt`)
- [ ] Linter passa (`cargo clippy`)
- [ ] Documentação está atualizada
- [ ] CHANGELOG.md está atualizado

#### Template de Pull Request

```markdown
## Descrição

Breve descrição das mudanças.

## Tipo de Mudança

- [ ] Bug fix
- [ ] Nova funcionalidade
- [ ] Breaking change
- [ ] Documentação

## Testes

- [ ] Testes unitários passam
- [ ] Testes de integração passam
- [ ] Benchmarks passam

## Checklist

- [ ] Código compila sem warnings
- [ ] Código está formatado
- [ ] Linter passa
- [ ] Documentação está atualizada
- [ ] CHANGELOG.md está atualizado

## Screenshots (se aplicável)

Adicione screenshots para mudanças visuais.

## Informações Adicionais

Qualquer informação adicional relevante.
```

## Desenvolvimento

### Configuração do Ambiente

```bash
# Instalar Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clonar repositório
git clone https://github.com/emulite/emulite.git
cd emulite

# Instalar dependências
cargo build

# Executar testes
cargo test

# Executar benchmarks
cargo bench
```

### Comandos Úteis

```bash
# Build
cargo build

# Testes
cargo test

# Benchmarks
cargo bench

# Linting
cargo clippy

# Formatação
cargo fmt

# Documentação
cargo doc --open

# Limpeza
cargo clean
```

### Estrutura do Projeto

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
└── .github/               # GitHub Actions
```

## Testes

### Executando Testes

```bash
# Todos os testes
cargo test

# Testes específicos
cargo test test_name

# Testes com output
cargo test -- --nocapture

# Testes de integração
cargo test --test integration_tests
```

### Adicionando Testes

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_example() {
        assert_eq!(2 + 2, 4);
    }
}
```

## Documentação

### Documentando Código

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

### Gerando Documentação

```bash
# Gerar documentação
cargo doc

# Abrir documentação
cargo doc --open

# Documentação com dependências
cargo doc --all-features
```

## Performance

### Benchmarks

```bash
# Executar benchmarks
cargo bench

# Benchmark específico
cargo bench bench_name
```

### Profiling

```bash
# Instalar ferramentas de profiling
cargo install flamegraph

# Gerar flamegraph
cargo flamegraph
```

## Release

### Versionamento

O projeto segue [Semantic Versioning](https://semver.org/):

- **MAJOR**: Mudanças incompatíveis na API
- **MINOR**: Novas funcionalidades compatíveis
- **PATCH**: Correções de bugs compatíveis

### Processo de Release

1. Atualizar versão no `Cargo.toml`
2. Atualizar `CHANGELOG.md`
3. Criar tag de release
4. Publicar no crates.io
5. Criar release no GitHub

## Suporte

### Canais de Comunicação

- **GitHub Issues**: Para bugs e melhorias
- **GitHub Discussions**: Para discussões gerais
- **Discord**: Para chat em tempo real
- **Email**: Para questões privadas

### Recursos Úteis

- [Documentação do Rust](https://doc.rust-lang.org/)
- [Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Cargo Book](https://doc.rust-lang.org/cargo/)

## Agradecimentos

Obrigado a todos os contribuidores que tornam este projeto possível!

## Licença

Este projeto está licenciado sob a [MIT License](LICENSE).
