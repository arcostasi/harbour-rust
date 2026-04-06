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
- compatibility-oriented diagnostics for selected array and numeric operations;
- Clipper-style string overflow limits for `Replicate()` and `Space()`.
- oracle-backed `SubStr()`/`Right()` leniency and host-C preservation of embedded `Chr(0)` in selected executable string helpers.
- default-width `Str()` formatting now aligned for positive large numbers and source-level float-literal display scale in the executable C path, while explicit negative-width padding and width-driven `Str()` rounding also follow the oracle through half-away-from-zero behavior.
- executable `Round()` output for large floats now stays in plain decimal form instead of collapsing into scientific notation in the host-C path.
- type-oriented executable behavior now covers `ValType()` on codeblocks and `Empty()` on codeblocks and host-C error values with oracle-backed expectations.

## Known Limits

- not all historical xBase value kinds exist yet;
- some builtins only cover the currently tested subset of arguments;
- `Val()` now follows the oracle for trailing-dot continuations such as `1..`, `1...`, `..`, and `-..`; the current ASCII subset also matches repeated-sign and exponent-like stop conditions, mixed punctuation such as `13.1.9`, and space-separated fragments after the decimal separator such as `12. 0` and `12 .10`; the remaining divergence is tied to source-level embedded `Chr(0)` construction in the current frontend/codegen path;
- source-level construction of embedded `Chr(0)` strings is still limited in the current frontend/codegen path even though the host-C runtime preserves them in selected helpers once present;
- exact historical formatting still differs in selected edge cases, especially some default-width large negative numeric expressions.

## Related Documents

- [Architecture](./architecture.md)
- [CLI](./cli.md)
