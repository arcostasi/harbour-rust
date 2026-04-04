# Technical Overview

- [English](./overview.md)
- [Português do Brasil](../../pt-BR/technical/overview.md)

## Purpose

This document describes the current compiler pipeline, crate boundaries, and the role each major component plays in Harbour Rust.

## What Harbour Rust Is

Harbour Rust is a Rust compiler project for CA-Clipper and Harbour compatibility. The current strategy is to build a practical, test-driven, compatibility-first compiler with a readable C backend before attempting a native backend.

## Compilation Pipeline

```text
source
  -> preprocessor
  -> lexer
  -> parser
  -> AST
  -> HIR
  -> semantic analysis
  -> IR
  -> C code generation
  -> host C compiler
  -> executable
```

## Workspace Crates

| Crate | Responsibility |
| --- | --- |
| `harbour-rust-cli` | user-facing commands and pipeline orchestration |
| `harbour-rust-pp` | preprocessor directives and expansion |
| `harbour-rust-lexer` | tokenization, spans, lexical diagnostics |
| `harbour-rust-parser` | parsing and syntax diagnostics |
| `harbour-rust-ast` | concrete syntax tree structures |
| `harbour-rust-hir` | high-level lowered representation |
| `harbour-rust-sema` | scope resolution, symbol checks, semantic diagnostics |
| `harbour-rust-ir` | backend-facing intermediate representation |
| `harbour-rust-codegen-c` | readable C emission |
| `harbour-rust-runtime` | runtime values, builtins, execution helpers |
| `harbour-rust-rdd` | DBF/RDD support |
| `harbour-rust-compat` | compatibility-oriented tests and helpers |
| `harbour-rust-tests` | golden, compare, and benchmark harnesses |

## Design Priorities

- compatibility first;
- incremental slices with tests;
- explicit documentation of known limits;
- architectural separation between frontend, semantics, runtime, backend, and RDD;
- long-term maintainability over short-term cleverness.

## Current State

The project already covers:

- a practical procedural subset;
- arrays, statics, memvars, codeblocks, and selected dynamic xBase features;
- an advanced but curated preprocessor subset;
- a working CLI and release-quality test scaffolding;
- an initial DBF/RDD baseline.

## Related Documents

- [Architecture](./architecture.md)
- [Runtime](./runtime.md)
- [CLI](./cli.md)
- [Release](./release.md)
