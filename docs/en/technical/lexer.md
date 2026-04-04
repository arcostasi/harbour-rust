# Lexer

- [English](./lexer.md)
- [Português do Brasil](../../pt-BR/technical/lexer.md)

## Role

The lexer turns `.prg` source into a token stream with spans, line/column positions, and lexical diagnostics.

## Current Baseline

The current alpha subset includes:

- case-insensitive keywords;
- identifiers;
- integers and floats;
- quoted strings;
- comments, including legacy line-comment forms;
- arithmetic, comparison, assignment, postfix, and macro-related operators.

## Design Rules

- keep lexical analysis separate from semantic concerns;
- preserve spans and source positions from the start;
- treat preprocessor handling as a separate layer;
- prefer deterministic tokenization and explicit lexical errors.

## Current Status

The lexer is stable for the currently implemented subset and already has curated golden fixtures.
