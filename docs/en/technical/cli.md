# CLI

- [English](./cli.md)
- [Português do Brasil](../../pt-BR/technical/cli.md)

## Role

`harbour-rust-cli` is the user-facing entry point for the compiler. It orchestrates preprocessing, parsing, semantic analysis, IR lowering, C emission, host compilation, and execution depending on the selected command.

## Current Commands

| Command | Purpose |
| --- | --- |
| `help` | show top-level or command-specific usage |
| `check` | validate a source file through frontend and semantics |
| `build` | generate C output |
| `transpile --to c` | explicitly emit C without compiling it |
| `run` | build, compile with a host C compiler, and execute |

## Exit Codes

| Code | Meaning |
| --- | --- |
| `0` | success |
| `1` | frontend or usage error |
| `2` | code generation error |
| `3` | host C compiler or execution infrastructure error |
| other | propagated program exit code in `run` |

## Design Priorities

- predictable pipeline stages;
- user-readable diagnostics;
- explicit handling of include paths and host compiler discovery;
- minimal surprise between `check`, `build`, `transpile`, and `run`.

## Current State

The CLI already covers the alpha workflow expected by the project:

- `help` is consistent;
- `check` stops after frontend and semantics;
- `build` and `transpile` emit C;
- `run` compiles generated C and propagates the program exit code;
- include directories are supported through `-I` / `--include-dir`.

## Related Documents

- [Technical Overview](./overview.md)
- [Release](./release.md)
