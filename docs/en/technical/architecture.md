# Architecture

- [English](./architecture.md)
- [Português do Brasil](../../pt-BR/technical/architecture.md)

## Goal

Harbour Rust aims to reimplement CA-Clipper/Harbour behavior in Rust without reproducing the upstream monolith.

## Upstream-to-Local Mapping

| Upstream area | Role | Harbour Rust destination |
| --- | --- | --- |
| `src/compiler/harbour.y` | grammar and precedence | parser and AST |
| `src/compiler/genc.c` | C backend reference | `harbour-rust-codegen-c` |
| `src/pp/ppcore.c` and `doc/pp.txt` | preprocessor behavior | `harbour-rust-pp` |
| `src/vm`, `src/rtl`, `doc/vm.txt` | runtime behavior | `harbour-rust-runtime` |
| `src/rdd` | DBF/RDD model | `harbour-rust-rdd` |
| `tests`, `tests/hbpp`, `utils/hbtest` | compatibility corpus | compat and test harnesses |

## Architectural Rules

- keep lexer separate from preprocessor;
- keep parser separate from semantics;
- keep runtime separate from frontend;
- keep IR separate from code generation;
- use C as the initial executable backend;
- treat a native backend as a later stage, not an initial target.

## Intentional Deviations

Harbour Rust does not begin by reproducing historical pcode or VM internals one-to-one. Instead it uses:

- a dedicated AST and HIR;
- a simpler backend-oriented IR;
- a readable C backend;
- a Rust runtime model with explicit tests.

This is an implementation strategy, not a rejection of upstream semantics.

## Compatibility Policy

- use upstream behavior as an oracle;
- document gaps instead of hiding them;
- prefer incremental test-backed fidelity over speculative redesign;
- keep ownership of the implementation fully original.

## Current Architectural State

The repository already includes:

- separated frontend layers;
- semantic analysis and runtime scaffolding;
- a practical C backend;
- dynamic feature groundwork;
- DBF/RDD infrastructure and quality tooling.

## Related Documents

- [Technical Overview](./overview.md)
- [Runtime](./runtime.md)
- [CLI](./cli.md)
