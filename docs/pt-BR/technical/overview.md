# Visão Técnica Geral

- [English](../../en/technical/overview.md)
- [Português do Brasil](./overview.md)

## Propósito

Este documento descreve o pipeline atual do compilador, os limites entre crates e o papel de cada componente principal do Harbour Rust.

## O Que é o Harbour Rust

Harbour Rust é um projeto de compilador em Rust voltado à compatibilidade com CA-Clipper e Harbour. A estratégia atual é construir um compilador prático, guiado por testes e orientado à compatibilidade, com backend C legível antes de tentar um backend nativo.

## Pipeline de Compilação

```text
source
  -> preprocessor
  -> lexer
  -> parser
  -> AST
  -> HIR
  -> semantic analysis
  -> IR
  -> geração de código C
  -> compilador C host
  -> executável
```

## Crates do Workspace

| Crate | Responsabilidade |
| --- | --- |
| `harbour-rust-cli` | comandos para usuários e orquestração do pipeline |
| `harbour-rust-pp` | diretivas e expansão do pré-processador |
| `harbour-rust-lexer` | tokenização, spans e diagnósticos léxicos |
| `harbour-rust-parser` | parsing e diagnósticos sintáticos |
| `harbour-rust-ast` | estruturas da árvore sintática concreta |
| `harbour-rust-hir` | representação lowered de alto nível |
| `harbour-rust-sema` | resolução de escopos, checagem de símbolos e diagnósticos semânticos |
| `harbour-rust-ir` | representação intermediária voltada ao backend |
| `harbour-rust-codegen-c` | emissão de C legível |
| `harbour-rust-runtime` | valores de runtime, builtins e helpers de execução |
| `harbour-rust-rdd` | suporte a DBF/RDD |
| `harbour-rust-compat` | testes e helpers voltados à compatibilidade |
| `harbour-rust-tests` | harnesses de golden, compare e benchmark |

## Prioridades de Design

- compatibilidade primeiro;
- slices incrementais com testes;
- documentação explícita de limites conhecidos;
- separação arquitetural entre frontend, semântica, runtime, backend e RDD;
- manutenibilidade de longo prazo acima de esperteza de curto prazo.

## Estado Atual

O projeto já cobre:

- um subconjunto procedural prático;
- arrays, statics, memvars, codeblocks e recursos dinâmicos xBase selecionados;
- um subconjunto avançado, porém curado, de pré-processador;
- uma CLI funcional e uma base de testes preparada para release;
- um baseline inicial de DBF/RDD.

## Documentos Relacionados

- [Architecture](./architecture.md)
- [Runtime](./runtime.md)
- [CLI](./cli.md)
- [Release](./release.md)
