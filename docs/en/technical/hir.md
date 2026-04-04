# HIR

- [English](./hir.md)
- [Português do Brasil](../../pt-BR/technical/hir.md)

## Role

HIR is the high-level lowered representation between the AST and semantic analysis. It removes syntax sugar and prepares structures that are easier to analyze.

## Main Responsibilities

- normalize identifiers into symbols;
- separate reads from write targets;
- preserve explicit storage categories such as `LOCAL`, `STATIC`, and module statics;
- represent arrays, indexing, memvars, codeblocks, and macro reads in an analysis-friendly form.

## Design Rules

- keep the HIR small but semantically meaningful;
- avoid letting semantic analysis rewrite the core representation;
- introduce nodes only when they carry real semantic value.

## Current Status

HIR already covers the currently implemented procedural and dynamic alpha subset and acts as the stable handoff into semantic analysis.
