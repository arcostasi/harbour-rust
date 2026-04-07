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
| Runtime | broad alpha baseline | core values, arrays, selected string/math/conversion builtins, oracle-backed string edge-case coverage for trim, search, slicing, replication, `Val()` parsing, `Str()` formatting behavior, focused `Round()`/`Int()` numeric edge cases, large-float executable `Round()` output aligned away from scientific notation, focused `Mod()`/`ValType()`/`Empty()` compatibility edges including codeblocks and host-C error values, focused `Max()`/`Min()`/`Abs()` edge cases, focused `Type()`/`Len()` edge cases, Clipper-style string overflow limits in `Replicate()`/`Space()`, and executable host-C preservation of embedded `Chr(0)` in selected string helpers |
| Preprocessor | curated advanced subset | `#define`, `#include`, `#command`, `#translate`, plus oracle-backed coverage for escaped optional replacements, selected optional-clause reordering, selected logical result markers like `<.id.>`, a minimal blockify-result subset for `<{id}>`, a macro-oriented quoted-result subset for `<"id">`, a macro-oriented smart-result subset for `<(id)>`, and a minimal macro pattern-marker subset for `<id:&>` including operator spillover and chained `&(expr)` suffixes |
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
- advanced preprocessor cases still remain in nested optional/list expansion, broader dumb-stringify combinations, broader pattern-level macro marker behavior beyond the current `<id:&>` subset, and broader marker semantics beyond the current oracle-backed subset such as multi-expression `<{id}>` blockify behavior and additional `<"id">`/`<(id)>` handling around strings, macros, and quoted literals;
- `Val()` now follows the current oracle-backed ASCII subset for trailing-dot continuations, repeated signs, exponent-like stop conditions, mixed punctuation, and space-separated fragments after the decimal separator; the remaining divergence is tied to source-level embedded `Chr(0)` construction in the current frontend/codegen path;
- `Str()` now follows the current oracle-backed baseline for width-driven half-away-from-zero rounding, negative-width padding, default-width positive large numbers, and source-level float-literal display scale in the executable C path; the remaining documented gap is default-width formatting for selected large negative numeric expressions;
- source-level construction of embedded `Chr(0)` strings is still limited in the current frontend/codegen path even though the executable host-C runtime now preserves them in selected helpers once present;
- historical edge cases must be treated as unsupported until tested and documented.

## Oracle Policy

- `harbour-core` is the main behavioral oracle.
- Tests, fixtures, and observed output are preferred over assumptions.
- Source code from upstream may inform understanding, but the implementation must be original.

## Dialect Policy

- Clipper-first behavior is preferred when there is an overlap.
- Harbour-specific extensions must be explicit and documented.
- Any intentional divergence must be recorded in tests and documentation.
