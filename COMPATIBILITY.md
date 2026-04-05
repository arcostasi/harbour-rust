# Compatibility

- [English](./COMPATIBILITY.md)
- [Português do Brasil](./COMPATIBILITY.pt-BR.md)

## Position

Harbour Rust aims for practical compatibility with CA-Clipper and Harbour, starting with behavior that can be observed, tested, and reproduced. Compatibility claims are always subordinate to explicit tests and documented limitations.

## Current Snapshot

| Area | Status | Notes |
| --- | --- | --- |
| Lexer | stable initial baseline | spans, positions, comments, strings, numbers, keywords |
| Parser and AST | stable for current subset | procedural constructs, arrays, memvar syntax, codeblocks, macro reads |
| HIR and semantics | stable for current subset | routine resolution, local/static bindings, memvars |
| Runtime | broad alpha baseline | core values, arrays, selected string/math/conversion builtins, and growing oracle-backed string edge-case coverage for trim, search, slicing, and replication behavior |
| Preprocessor | curated advanced subset | `#define`, `#include`, `#command`, `#translate` |
| C backend | practical alpha backend | procedural flow, selected runtime helpers and dynamic features |
| CLI | usable alpha interface | `help`, `check`, `build`, `run`, `transpile --to c` |
| RDD/DBF | initial usable baseline | schema parsing, navigation, reads, append/update/delete/recall |
| Regression tooling | present | golden tests, compare tool, benchmark smoke, fuzz scaffold |

## Known Limits

The project is still alpha software. Known limits include:

- partial rather than complete xBase dialect coverage;
- selected builtins implemented only for the currently tested subset of value kinds;
- no native backend yet; C is the primary executable backend;
- compatibility gaps remain in advanced macro behavior, broader runtime fidelity, and extended RDD coverage;
- historical edge cases must be treated as unsupported until tested and documented.

## Oracle Policy

- `harbour-core` is the main behavioral oracle.
- Tests, fixtures, and observed output are preferred over assumptions.
- Source code from upstream may inform understanding, but the implementation must be original.

## Dialect Policy

- Clipper-first behavior is preferred when there is an overlap.
- Harbour-specific extensions must be explicit and documented.
- Any intentional divergence must be recorded in tests and documentation.
