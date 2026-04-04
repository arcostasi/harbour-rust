# Semantic Analysis

- [English](./sema.md)
- [Português do Brasil](../../pt-BR/technical/sema.md)

## Role

Semantic analysis validates the HIR, resolves symbols, manages scope tables, and produces semantic diagnostics without rewriting the HIR itself.

## Current Baseline

The current alpha subset includes:

- routine-level symbol tables;
- local and static bindings;
- module-level statics;
- memvar-related resolution groundwork;
- nested scope handling for codeblock parameters;
- unresolved-symbol and duplicate-binding diagnostics.

## Design Rules

- keep semantic decisions in side tables when possible;
- preserve case-insensitive resolution behavior;
- walk all relevant expressions, including arrays, indexing, macro reads, and codeblocks;
- prefer explicit diagnostics over implicit fallback when behavior is not yet supported.

## Current Status

The semantic layer is sufficient for the project's current alpha pipeline and already supports both procedural and early dynamic behavior.
