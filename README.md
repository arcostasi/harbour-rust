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
4. `docs/overview.md` — visão geral, mapa de crates e pipeline
5. `docs/architecture.md` — decisões arquiteturais
6. `docs/test-strategy.md`
7. Docs temáticas por camada: `docs/lexer.md`, `docs/preprocessor.md`, `docs/grammar.md`, `docs/hir.md`, `docs/sema.md`, `docs/ir.md`, `docs/codegen-c.md`, `docs/runtime.md`, `docs/cli.md`, `docs/rdd.md`

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

As Fases 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11 e 12 estão concluídas:

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
- formatação orientada a impressão com `QOut()` mínimo e dispatch case-insensitive de builtins de saída,
- IR procedural inicial implementada com lowering HIR -> IR,
- backend C inicial implementado para rotinas procedurais, `RETURN`, `QOut()`, `DO WHILE` e `FOR` simples,
- CLI com `build` e `run` fim a fim via compilador C host,
- caminhos executáveis curados para `examples/hello.prg`, `tests/fixtures/parser/while.prg` e `tests/fixtures/parser/for_sum.prg`,
- pré-processador inicial com `#define` e `#include`,
- expansão recursiva case-insensitive de macros objeto com diagnóstico de ciclo,
- resolução de includes com quoted include, angle-bracket include e search paths configuráveis,
- handoff explícito `pp -> parser` no CLI com `-I/--include-dir`,
- `STATIC` same-routine e de módulo executáveis no caminho `cli run`,
- arrays com leitura/escrita indexada, builtins essenciais e comparação observável básica,
- operadores compostos `+= -= *= /=` executáveis,
- builtins essenciais de string, math e conversão executáveis no caminho `cli run`,
- fixture de aceite da Fase 7 curado em `tests/fixtures/parser/phase7_acceptance.prg`,
- `PRIVATE`, `PUBLIC`, leitura/atribuição dinâmica de memvar e macro read mínima executáveis no caminho `cli run`,
- `Eval()` com codeblocks não-capturantes e codeblocks lendo memvar executável no caminho `cli run`,
- fixture de aceite da Fase 8 curado em `tests/fixtures/parser/phase8_acceptance.prg`,
- `#command`, `#translate`, markers de lista/restrição/opcional/stringify e continuação por `;` atravessando `pp -> parser -> cli run`,
- fixture de aceite da Fase 9 curado em `tests/fixtures/pp/phase9_acceptance.prg`,
- crate `harbour-rust-rdd` com trait `Rdd`, schema DBF, navegação, leitura de campos e mutação persistente básica,
- leitura validada contra `harbour-core/contrib/hbhttpd/tests/users.dbf`, `carts.dbf`, `items.dbf` e `harbour-core/tests/test.dbf`,
- escrita básica validada em roundtrip temporário para `APPEND BLANK`, `REPLACE`, `DELETE` e `RECALL`,
- CLI com help consistente por comando, `check`, `transpile --to c` e códigos de saída coerentes entre frontend, codegen e compilador C host,
- cobertura de integração do CLI para help, `check`, `transpile` e falhas de `codegen-c`,
- crate `harbour-rust-tests` convertido em harness de golden tests executáveis e comparador com runner Harbour externo,
- benchmark suite inicial em markdown para `check`, `transpile` e `run`,
- skeleton de `cargo fuzz` para lexer, parser e PP,
- workflows de qualidade e release com preflight, benchmark smoke e artefatos de release.

O próximo passo técnico é preparar a release `0.4.0-alpha` com o checklist de release da Fase 12 e, a partir daí, ampliar corpus, compatibilidade e performance sobre a base já estabilizada.

O baseline fim a fim atual oferece geração de C:

```text
cargo run -p harbour-rust-cli -- build examples/hello.prg --out target/hello.c
```

O pipeline atual valida parse, HIR, sema, IR e `codegen-c`, e escreve o `.c` gerado.

O mesmo baseline já executa `hello.prg` via compilador C host:

```text
cargo run -p harbour-rust-cli -- run examples/hello.prg
```

Nesta etapa o `run` detecta `clang`, `gcc` ou `cc`, compila o C gerado com um suporte mínimo de runtime e executa o binário resultante. O suporte de codegen continua parcial fora do subconjunto procedural já coberto, mas a Fase 5 já atende o aceite com `hello.prg` e um programa com `FOR` simples executáveis.

Na Fase 6, o mesmo pipeline já aceita preprocessamento configurável no CLI:

```text
cargo run -p harbour-rust-cli -- build tests/fixtures/pp/angle_search_path_root.prg --include-dir tests/fixtures/pp/include-path --out target/angle_search.c
cargo run -p harbour-rust-cli -- run tests/fixtures/pp/angle_search_path_root.prg -I tests/fixtures/pp/include-path
```

Na Fase 11, o CLI passa a expor o fluxo de uso mínimo completo:

```text
cargo run -p harbour-rust-cli -- check examples/hello.prg
cargo run -p harbour-rust-cli -- transpile --to c examples/hello.prg --out target/hello.c
cargo run -p harbour-rust-cli -- help
```

O modelo atual de saída é:

- `0` para sucesso,
- `1` para erro de entrada/frontend (`pp`, parser, HIR, sema),
- `2` para erro de `codegen-c`,
- `3` para erro do compilador C host,
- qualquer outro código para o exit status real do programa executado por `run`.

Na Fase 12, a infraestrutura de qualidade mínima passa a incluir:

```text
cargo test -p harbour-rust-tests
cargo run -p harbour-rust-tests --bin compare-harbour -- --fixture examples/hello.prg --harbour-runner /caminho/para/runner-harbour
cargo run -p harbour-rust-tests --bin benchmark-suite -- --fixture examples/hello.prg --iterations 1
cargo check --manifest-path fuzz/Cargo.toml
```

Esse recorte fecha:

- golden tests de stdout para fixtures de aceite,
- comparador operacional inicial contra runner Harbour externo,
- benchmark smoke reproduzível em markdown,
- skeleton de fuzzing para `lexer`, `parser` e `pp`,
- workflow de release com build release e benchmark report.

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
