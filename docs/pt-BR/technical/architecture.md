# Arquitetura

- [English](../../en/technical/architecture.md)
- [Português do Brasil](./architecture.md)

## Meta

Harbour Rust busca reimplementar o comportamento de CA-Clipper/Harbour em Rust sem reproduzir o monólito do upstream.

## Mapeamento Upstream para Local

| Área do upstream | Papel | Destino no Harbour Rust |
| --- | --- | --- |
| `src/compiler/harbour.y` | gramática e precedência | parser e AST |
| `src/compiler/genc.c` | referência do backend C | `harbour-rust-codegen-c` |
| `src/pp/ppcore.c` e `doc/pp.txt` | comportamento do pré-processador | `harbour-rust-pp` |
| `src/vm`, `src/rtl`, `doc/vm.txt` | comportamento de runtime | `harbour-rust-runtime` |
| `src/rdd` | modelo de DBF/RDD | `harbour-rust-rdd` |
| `tests`, `tests/hbpp`, `utils/hbtest` | corpus de compatibilidade | harnesses de compat e testes |

## Regras Arquiteturais

- manter lexer separado do pré-processador;
- manter parser separado da semântica;
- manter runtime separado do frontend;
- manter IR separada da geração de código;
- usar C como backend executável inicial;
- tratar backend nativo como etapa posterior, não como alvo inicial.

## Desvios Intencionais

Harbour Rust não começa reproduzindo pcode ou internas da VM histórica de forma um-para-um. Em vez disso, usa:

- AST e HIR dedicadas;
- uma IR mais simples e voltada ao backend;
- backend C executável e pragmático;
- modelo de runtime em Rust com testes explícitos.

Isso é uma estratégia de implementação, não uma rejeição da semântica do upstream.

## Política de Compatibilidade

- usar o comportamento do upstream como oráculo;
- documentar lacunas em vez de escondê-las;
- preferir fidelidade incremental apoiada por testes em vez de redesign especulativo;
- manter a implementação integralmente original.

## Estado Arquitetural Atual

O repositório já inclui:

- camadas de frontend separadas;
- análise semântica e base de runtime;
- backend C prático;
- base para recursos dinâmicos;
- infraestrutura de DBF/RDD e tooling de qualidade.

## Documentos Relacionados

- [Technical Overview](./overview.md)
- [Runtime](./runtime.md)
- [CLI](./cli.md)
