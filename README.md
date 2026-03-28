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

As Fases 0, 1, 2, 3 e 4 estão concluídas:

- workspace Cargo criado,
- crates iniciais criados,
- estrutura base de `examples/`, `tests/` e `tools/` criada,
- workflow de CI configurado para `fmt`, `clippy` e `test`,
- lexer inicial implementado com spans, posições, keywords, operadores, strings, números, comentários e diagnósticos básicos,
- baselines léxicos curados para `hello.prg` e `while.prg`,
- AST procedural inicial implementada para `PROCEDURE`, `FUNCTION`, `LOCAL`, `RETURN`, `IF`, `DO WHILE`, `FOR` e `?`,
- parser inicial com recuperação básica de blocos e diagnósticos sintáticos melhores,
- snapshots de AST curados para `hello.prg` e `while.prg`,
- HIR procedural inicial implementada com lowering AST -> HIR para rotinas, statements e expressões básicas,
- análise semântica mínima implementada com tabela global de rotinas, tabela local por rotina, resolução case-insensitive e diagnósticos de símbolo ausente,
- regressões de sema curadas para símbolos locais e callables ausentes em `IF`, `DO WHILE` e `FOR`,
- runtime mínimo implementado com `Value::{Nil, Logical, Integer, Float, String}`,
- conversões básicas, aritmética e comparação com erros estruturados,
- formatação orientada a impressão com `QOut()` mínimo e dispatch case-insensitive de builtins de saída.

O próximo passo técnico é iniciar a Fase 5 com IR e backend C para integração fim a fim.

Neste ponto, o primeiro slice de CLI para a Fase 5 já oferece geração de C:

```text
cargo run -p harbour-rust-cli -- build examples/hello.prg --out target/hello.c
```

O pipeline atual valida parse, HIR, sema, IR e `codegen-c`, e escreve o `.c` gerado. A compilação com compilador C host entra na próxima slice.

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
