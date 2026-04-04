# Tools

Ferramentas auxiliares do projeto `harbour-rust`.

## Disponíveis

### Comparador harbour-core

O comparador inicial da Fase 12 está disponível via:

```text
cargo run -p harbour-rust-tests --bin compare-harbour -- --fixture examples/hello.prg --harbour-runner /caminho/para/runner-harbour
```

O `runner-harbour` deve aceitar o caminho do `.prg` como primeiro argumento e retornar:

- código de saída,
- `stdout`,
- `stderr`.

O comparador executa o mesmo fixture com `harbour-rust` e com esse runner externo, compara `exit code` e `stdout`, e pode comparar `stderr` com `--match-stderr`.

### Benchmark runner

O benchmark inicial da Fase 12 está disponível via:

```text
cargo run -p harbour-rust-tests --bin benchmark-suite -- --fixture examples/hello.prg --iterations 3
```

O runner mede:

- `check`
- `transpile`
- `run`

e emite um relatório markdown simples por fixture.

### Fuzzing

O skeleton inicial de fuzzing fica em `fuzz/` com targets para:

- `lexer`
- `parser`
- `pp`

Smoke check do harness:

```text
cargo check --manifest-path fuzz/Cargo.toml
```

### Release helper

O preflight de release da Fase 12 fica documentado em `docs/release.md` e automatizado em:

- `.github/workflows/quality.yml`
- `.github/workflows/release.yml`

## Planejadas

### Comparador harbour-core

Script que executa o mesmo `.prg` com `harbour-rust` e `harbour-core`, comparando:

- código de saída
- stdout
- stderr

### Gerador de fixtures

Automatiza criação de golden files a partir da saída atual do compilador para novas fixtures `.prg`.

### Fuzzer

Geração aleatória de tokens/sources para stress test de:

- lexer
- parser
- PP

### Benchmark runner

Executa um conjunto de `.prg` com medição de tempo para:

- compilação (parse, sema, codegen)
- execução do binário gerado
- comparação com harbour-core

### Release helper

Valida checklist de release:

- testes verdes
- COMPATIBILITY.md atualizada
- ROADMAP.md refletindo o estado real
- changelog gerado

## Como adicionar uma ferramenta

1. Crie um script ou binário em `tools/`
2. Documente uso e propósito neste README
3. Se aplicável, adicione ao CI
