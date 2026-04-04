# Diagnostics

- [English](./diagnostics.md)
- [Português do Brasil](../../pt-BR/technical/diagnostics.md)

## Goal

Diagnostics should be predictable, localized, and useful from the earliest compiler stages onward.

## Principles

- always include file and source position when possible;
- prefer short primary messages;
- avoid panic-based user-facing failures;
- reduce cascaded noise when one root error explains the failure.

## Current Categories

The project currently uses practical diagnostic groupings for:

- lexical errors;
- preprocessor errors;
- syntax errors;
- semantic errors;
- runtime errors;
- code generation failures;
- CLI and build failures.

## Current Status

Diagnostics are already structured enough for the current alpha workflow, but richer snippets and more advanced multi-file context remain future work.
