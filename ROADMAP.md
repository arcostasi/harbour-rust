# Roadmap

- [English](./ROADMAP.md)
- [Português do Brasil](./ROADMAP.pt-BR.md)

## Purpose

This roadmap organizes Harbour Rust into small, cumulative, verifiable milestones. It is intentionally compatibility-first and uses `harbour-core` as a reference for behavior, not as a source for transliteration.

## Release Milestones

| Release | Focus | Status |
| --- | --- | --- |
| `0.1.0-alpha` | minimal end-to-end procedural pipeline | completed |
| `0.2.0-alpha` | expanded procedural compatibility and initial preprocessor support | completed |
| `0.3.0-alpha` | dynamic xBase behavior | completed |
| `0.4.0-alpha` | RDD foundation, CLI/DX, regression and release tooling | completed |
| `0.5.0-alpha` | curated phase 15 compatibility expansion, focused advanced PP corpus growth | completed |
| `0.6.0-alpha` | phase 16 runtime fidelity, starting with focused Harbour runtime/library builtins | planned |

## Phase Snapshot

| Phase | Theme | Status |
| --- | --- | --- |
| 0 | repository foundation | completed |
| 1 | lexer | completed |
| 2 | AST and parser | completed |
| 3 | HIR and basic semantics | completed |
| 4 | minimum runtime | completed |
| 5 | IR and C backend | completed |
| 6 | initial preprocessor | completed |
| 7 | expanded procedural compatibility | completed |
| 8 | dynamic xBase features | completed |
| 9 | advanced preprocessor | completed |
| 10 | DBF/RDD foundation | completed |
| 11 | diagnostics, CLI, DX | completed |
| 12 | quality and release readiness | completed |
| 13 | oracle-backed advanced preprocessor markers | completed |
| 14 | curated compatibility corpus expansion | completed |
| 15 | post-0.4 compatibility expansion | first release slice completed |
| 16 | post-0.5 runtime fidelity | planned |

## Near-Term Priorities

After the `0.5.0-alpha` release, the next priority is phase 16 runtime fidelity.

The first planned corridor is:

1. implement the smallest oracle-backed `hb_JsonDecode` slice that can map JSON scalars, arrays, and objects into the current runtime value model;
2. document unsupported JSON/value edge cases explicitly instead of implying full Harbour API coverage;
3. use the same pattern for later `hb_gzCompress` and `hb_processRun` slices only after the value and string/binary behavior is stable enough;
4. defer sockets and threading until the runtime has explicit cross-platform IO, ownership, and concurrency decisions.

Secondary priorities remain:

1. larger compatibility corpus;
2. broader DBF/RDD coverage;
3. performance and memory profiling;
4. selective architectural hardening without losing readability.

## Planning Rules

- Prefer small, reversible increments.
- Keep behavior measurable with tests and fixtures.
- Separate parser, semantics, runtime, code generation, and RDD work when possible.
- Document known incompatibilities instead of hiding them.
- Treat English as the canonical language for roadmap updates and keep the Portuguese version aligned.
