# IR

- [English](./ir.md)
- [Português do Brasil](../../pt-BR/technical/ir.md)

## Role

The IR is the backend-facing representation between semantic analysis and code generation. It remains target-independent while being closer to executable lowering than HIR.

## Current Baseline

The current IR preserves:

- routines and structured control flow;
- explicit reads and assignment targets;
- `STATIC`, `PRIVATE`, and `PUBLIC` statements;
- arrays, indexing, codeblocks, macro expressions, and selected dynamic operations.

## Design Rules

- keep the IR backend-agnostic;
- preserve structure until flattening is actually needed;
- emit explicit lowering errors instead of silently dropping unsupported constructs.

## Current Status

The IR is already strong enough to feed the current C backend for the alpha subset and acts as a clear boundary between frontend semantics and executable lowering.
