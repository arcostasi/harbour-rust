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

The preprocessor already supports the project's current alpha baseline, but advanced marker forms and broader corpus fidelity remain future work.
