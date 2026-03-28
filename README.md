# harbour-rust

Compilador moderno em Rust compatível com CA-Clipper/Harbour, construído em fases pequenas, verificáveis e cumulativas.

Este repositório começa pela governança e pelo plano de execução. O código-fonte de `harbour-core/` é tratado como:

- oráculo de comportamento,
- mapa de arquitetura,
- fonte de corpus de testes e fixtures,
- referência para compatibilidade,
- não como convite para transliteração direta de C para Rust.

## Princípios

- Compatibilidade primeiro.
- Cada mudança precisa ser pequena, compilável e reversível.
- Cada fase entrega artefatos concretos, testes e documentação.
- O `harbour-core/` serve como baseline observável sempre que possível.
- Extensões Harbour entram atrás de flag de dialeto.

## Leitura obrigatória

1. `AGENTS.md`
2. `ROADMAP.md`
3. `COMPATIBILITY.md`
4. `docs/architecture.md`
5. `docs/test-strategy.md`
6. `docs/grammar.md`, `docs/runtime.md` e demais docs relevantes para a task

## Como usar o upstream `harbour-core/`

Áreas de maior valor para `harbour-rust`:

- `harbour-core/src/compiler/harbour.y`: gramática e precedências históricas.
- `harbour-core/src/compiler/genc.c`: backend C e formato do código gerado.
- `harbour-core/src/pp/ppcore.c` e `harbour-core/doc/pp.txt`: pré-processador compatível com Clipper.
- `harbour-core/include/hbpcode.h`, `harbour-core/doc/pcode.txt`, `harbour-core/doc/vm.txt`: modelo de pcode e VM.
- `harbour-core/src/vm`: semântica de execução, memvars, codeblocks, arrays e stack.
- `harbour-core/src/rtl`: builtins e comportamento de runtime.
- `harbour-core/src/rdd`: DBF/RDD.
- `harbour-core/tests`, `harbour-core/tests/hbpp`, `harbour-core/utils/hbtest`: corpus de compatibilidade e regressão.

## Política de derivação

- Reimplementar em Rust com desenho próprio.
- Usar docs, testes, comportamento observado e pequenas leituras pontuais do upstream para orientar decisões.
- Evitar copiar blocos substanciais de código C para Rust.
- Qualquer divergência conhecida deve ser registrada em `COMPATIBILITY.md`.

## Estado atual

O repositório está preparado para iniciar a Fase 0. O plano completo está em `ROADMAP.md` e nos arquivos de `docs/`.

## Desenvolvimento

Comandos principais do workspace:

```text
cargo fmt --all
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test
```

Estrutura inicial criada na Fase 0:

- `crates/`: crates do compilador, runtime, compat e testes
- `examples/`: programas `.prg` mínimos para smoke tests
- `tests/`: fixtures e testes de integração do projeto
- `tools/`: ferramentas auxiliares e comparadores

Veja também `CONTRIBUTING.md` para o fluxo de contribuição local.
