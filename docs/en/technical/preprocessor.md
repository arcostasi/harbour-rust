# Preprocessor

- [English](./preprocessor.md)
- [Português do Brasil](../../pt-BR/technical/preprocessor.md)

## Role

The preprocessor handles compile-time directives before the main lexer and parser. Its purpose is to support a practical compatibility subset of Clipper/Harbour preprocessing.

## Current Baseline

Implemented areas include:

- object-like `#define`;
- recursive define expansion with cycle detection;
- `#include` with quoted and angle-bracket lookup;
- configurable include search paths;
- curated subsets of `#command`, `#translate`, `#xcommand`, and `#xtranslate`.

## Design Rules

- keep preprocessing separate from the main lexer;
- preserve source-origin information for diagnostics;
- grow toward stronger token-aware behavior over time;
- measure compatibility with focused fixtures instead of vague claims.

## Current Status

The preprocessor already supports the project's current alpha baseline. Focused compatibility fixtures now cover escaped-bracket optional replacements, selected reordering across contiguous optional clauses, a focused nested optional/list subset drawn from the upstream `AAA`, `SET`, `AVG`, `INSERT`, and `INSERT2` rules, repeated optional-clause expansion and continued source-line invocation for the `INSERT`/`INSERT2` subset, a focused multiline optional-clause reordering subset from the upstream `_pp_test` `MYCOMMAND3` rule, a focused multiline result-body subset where the replacement starts on the next line after `=>` as exercised by the upstream `INSERT2`, `MYCOMMAND2`, and `MYCOMMAND3` rules, including the `MYCOMMAND2` `ALL`-before-list permutation, selected logical result markers such as `<.id.>`, a minimal blockify-result subset for `<{id}>`, a macro-oriented quoted-result subset for `<"id">`, a macro-oriented smart-result subset for `<(id)>`, and an expanded macro pattern-marker subset for `<id:&>` drawn from the upstream hbpp corpus and `pp.txt`, including selected operator spillover, longer multi-segment chains, and selected `&(expr)` mixtures. Broader nested optional/list combinations, dumb-stringify edge cases, broader macro pattern-marker behavior, and broader marker semantics still remain future work.
