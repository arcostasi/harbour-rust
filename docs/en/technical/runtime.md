# Runtime

- [English](./runtime.md)
- [Português do Brasil](../../pt-BR/technical/runtime.md)

## Role

The runtime provides the execution model used by generated C code and by runtime-oriented tests. It is responsible for values, builtins, dynamic storage helpers, and execution-side diagnostics.

## Core Value Model

The current runtime baseline includes:

- `Nil`
- `Logical`
- `Integer`
- `Float`
- `String`
- `Array`
- `Codeblock`

Not every xBase value kind is implemented yet, but the model is intentionally extensible.

## Main Responsibilities

- type-aware value storage;
- conversions and arithmetic;
- string, math, and conversion builtins;
- array helpers for read, write, clone, resize, and search-related behavior;
- memvar and dynamic-scope groundwork;
- output formatting and `QOut`;
- structured runtime errors.

## Design Rules

- no panics for predictable user-facing runtime errors;
- prefer explicit helpers over hidden magic;
- keep semantics testable from Rust and from the CLI path;
- document any lenient or partial behavior that does not yet match historical runtimes exactly.

## Current State

The runtime already supports:

- a broad alpha subset of builtins;
- arrays with one-based indexing;
- `STATIC`-related executable behavior through the backend path;
- memvar context groundwork and codeblock evaluation;
- compatibility-oriented diagnostics for selected array and numeric operations.

## Known Limits

- not all historical xBase value kinds exist yet;
- some builtins only cover the currently tested subset of arguments;
- exact historical formatting still differs in selected edge cases.

## Related Documents

- [Architecture](./architecture.md)
- [CLI](./cli.md)
