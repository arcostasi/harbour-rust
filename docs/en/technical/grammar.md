# Grammar and Parser

- [English](./grammar.md)
- [Português do Brasil](../../pt-BR/technical/grammar.md)

## Role

The parser uses recursive parsing for statements and Pratt-style parsing for expressions. It builds the concrete AST and provides syntax diagnostics and limited recovery.

## Current Baseline

The public alpha subset already includes:

- `PROCEDURE` and `FUNCTION`;
- `RETURN`;
- `LOCAL` and `STATIC`;
- `IF / ELSE / ENDIF`;
- `DO WHILE / ENDDO`;
- `FOR / NEXT`;
- `?` print syntax;
- arrays, compound assignment, codeblocks, memvar syntax, and initial macro-read syntax.

## Design Rules

- keep the AST explicit and stable;
- preserve spans but not trivia in the AST;
- avoid semantic resolution in the parser;
- prefer recoverable diagnostics over early aborts.

## Current Status

The grammar layer is broad enough for the current alpha pipeline and already feeds HIR, semantics, runtime integration, and compatibility fixtures.
