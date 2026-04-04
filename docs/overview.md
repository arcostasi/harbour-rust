# Visão geral do harbour-rust

> Nota de transição: a versão pública bilíngue deste conteúdo está sendo migrada para [docs/en/technical/overview.md](./en/technical/overview.md) e [docs/pt-BR/technical/overview.md](./pt-BR/technical/overview.md).

## O que é

`harbour-rust` é um compilador moderno escrito em Rust, 100% compatível com CA-Clipper e Harbour. O objetivo é oferecer:

- compatibilidade total com o legado xBase/Clipper,
- desempenho superior ao compilador original,
- mensagens de erro modernas e acionáveis,
- backend C legível para interoperabilidade e depuração,
- arquitetura limpa, testável e extensível.

## Pipeline de compilação

```text
 ┌─────────┐    ┌────┐    ┌───────┐    ┌────────┐    ┌─────┐
 │ source  │───>│ PP │───>│ Lexer │───>│ Parser │───>│ AST │
 └─────────┘    └────┘    └───────┘    └────────┘    └─────┘
                                                        │
                                                        v
┌──────────┐    ┌────┐    ┌──────┐    ┌──────┐    ┌─────────┐
│ binário  │<───│ cc │<───│ C    │<───│  IR  │<───│   HIR   │
└──────────┘    └────┘    └──────┘    └──────┘    │ + sema  │
                                                  └─────────┘
```

## Mapa de crates

| Crate | Diretório | Responsabilidade | Doc |
| --- | --- | --- | --- |
| `harbour-rust-cli` | `crates/harbour-rust-cli/` | Ponto de entrada: `check`, `build`, `run` | [cli.md](cli.md) |
| `harbour-rust-pp` | `crates/harbour-rust-pp/` | Pré-processador: `#define`, `#include`, `#command` | [preprocessor.md](preprocessor.md) |
| `harbour-rust-lexer` | `crates/harbour-rust-lexer/` | Tokenização com spans e diagnósticos | [lexer.md](lexer.md) |
| `harbour-rust-parser` | `crates/harbour-rust-parser/` | Parser recursivo + Pratt de expressões | [grammar.md](grammar.md) |
| `harbour-rust-ast` | `crates/harbour-rust-ast/` | Árvore sintática concreta e estável | [grammar.md](grammar.md) |
| `harbour-rust-hir` | `crates/harbour-rust-hir/` | Representação intermediária de alto nível | [hir.md](hir.md) |
| `harbour-rust-sema` | `crates/harbour-rust-sema/` | Análise semântica: escopos, resolução, diagnósticos | [sema.md](sema.md) |
| `harbour-rust-ir` | `crates/harbour-rust-ir/` | Representação intermediária para backends | [ir.md](ir.md) |
| `harbour-rust-codegen-c` | `crates/harbour-rust-codegen-c/` | Geração de código C legível | [codegen-c.md](codegen-c.md) |
| `harbour-rust-runtime` | `crates/harbour-rust-runtime/` | Valores, builtins, ambiente de execução | [runtime.md](runtime.md) |
| `harbour-rust-rdd` | `crates/harbour-rust-rdd/` | DBF/RDD (futuro) | [rdd.md](rdd.md) |
| `harbour-rust-compat` | `crates/harbour-rust-compat/` | Harness de comparação com harbour-core | [test-strategy.md](test-strategy.md) |
| `harbour-rust-tests` | `crates/harbour-rust-tests/` | Fixtures, snapshots, golden tests | [test-strategy.md](test-strategy.md) |

## Dependências entre crates

```text
cli ──> pp ──> lexer
 │      │
 │      v
 ├──> parser ──> ast
 │      │
 │      v
 ├──> hir <──── ast
 │      │
 │      v
 ├──> sema ──> hir
 │      │
 │      v
 ├──> ir <──── hir
 │      │
 │      v
 ├──> codegen-c ──> ir
 │
 ├──> runtime
 │
 └──> rdd ──> runtime
```

## Fluxo de dados por fase

| Fase | Entrada | Saída | Crates principais |
| --- | --- | --- | --- |
| Preprocessamento | `.prg` + `.ch` | source expandido | `pp` |
| Tokenização | source expandido | stream de tokens | `lexer` |
| Parsing | stream de tokens | AST | `parser`, `ast` |
| Lowering | AST | HIR | `hir` |
| Análise semântica | HIR | HIR anotada + side tables | `sema` |
| Lowering IR | HIR anotada | IR | `ir` |
| Geração de código | IR | arquivo `.c` | `codegen-c` |
| Compilação | `.c` | binário | compilador C host |

## Documentação de referência

| Documento | Conteúdo |
| --- | --- |
| [architecture.md](architecture.md) | Decisões arquiteturais e mapeamento do upstream |
| [grammar.md](grammar.md) | Gramática, precedência, slices do parser |
| [runtime.md](runtime.md) | Modelo de valores, builtins, ambiente |
| [diagnostics.md](diagnostics.md) | Estrutura, categorias e códigos de mensagens de erro |
| [test-strategy.md](test-strategy.md) | Testes, fixtures, compatibilidade |
| [dialect-clipper.md](dialect-clipper.md) | Baseline Clipper |
| [dialect-harbour.md](dialect-harbour.md) | Extensões Harbour |
| [preprocessor.md](preprocessor.md) | Design do pré-processador |
| [lexer.md](lexer.md) | Design do lexer |
| [hir.md](hir.md) | Design da HIR |
| [sema.md](sema.md) | Design da análise semântica |
| [ir.md](ir.md) | Design da IR |
| [codegen-c.md](codegen-c.md) | Design do backend C |
| [cli.md](cli.md) | Interface de linha de comando |
| [rdd.md](rdd.md) | Design do RDD/DBF |
| [upstream-navigator.md](upstream-navigator.md) | Guia para encontrar referências no harbour-core |
| [recipes.md](recipes.md) | Receitas passo a passo para operações comuns |
| [phase7-plan.md](phase7-plan.md) | Plano detalhado da Fase 7 |
| [rdd.md](rdd.md) | Design do RDD/DBF |
| [commit-flow.md](commit-flow.md) | Convenções de commit |
| [conventional-commits.md](conventional-commits.md) | Especificação Conventional Commits |

## Comandos essenciais

```bash
# Verificar estilo e lint
cargo fmt --all
cargo clippy --workspace --all-targets --all-features -- -D warnings

# Executar todos os testes
cargo test

# Compilar um programa .prg para C
cargo run -p harbour-rust-cli -- build examples/hello.prg --out target/hello.c

# Compilar e executar
cargo run -p harbour-rust-cli -- run examples/hello.prg

# Compilar com include paths
cargo run -p harbour-rust-cli -- run program.prg -I include/
```
