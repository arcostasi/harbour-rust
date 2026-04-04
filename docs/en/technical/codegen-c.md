# C Backend

- [English](./codegen-c.md)
- [Português do Brasil](../../pt-BR/technical/codegen-c.md)

## Role

The C backend emits explicit C from the IR and relies on a host C compiler to produce executables.

## Design Priorities

- explicitness and debuggability over cleverness;
- explicit runtime helper calls;
- backend growth in small compatibility-backed slices;
- clear diagnostics when IR constructs are not yet executable.

## Current Baseline

The backend already supports:

- procedural routines and returns;
- control flow such as `IF`, `DO WHILE`, and `FOR`;
- `STATIC` storage in the currently supported subset;
- arrays, indexed reads and writes, and selected builtins;
- memvar-related helpers, macro reads, and non-capturing codeblock execution paths.

## Current Status

The C backend is the primary executable backend of the project and the main bridge between compiler research and practical runnable compatibility.
