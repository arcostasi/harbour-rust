# Test Strategy

- [English](./test-strategy.md)
- [Português do Brasil](../../pt-BR/technical/test-strategy.md)

## Principle

Compatibility is not assumed. It is measured.

## Main Layers

- unit tests for focused crate behavior;
- integration tests across the compiler pipeline;
- golden and snapshot tests for stable observable output;
- compatibility tests against curated upstream behavior;
- fuzz scaffolding for parser-related surfaces;
- benchmark smoke for repeatable baseline timing.

## Design Rules

- keep fixtures small and readable;
- promote fixed bugs into regression coverage;
- prefer curated corpus growth over uncontrolled bulk imports;
- treat compatibility claims as test-backed statements.
- let oracle-dependent compatibility tests skip cleanly in public CI when `harbour-core/` is not checked out.

## Current Status

The repository already has a practical test matrix covering compiler slices, runtime behavior, CLI execution, golden snapshots, compare tooling, benchmark smoke, and fuzz compilation checks. The compatibility corpus also includes focused runtime string fixtures for control-character trim behavior, `At()` edge cases, `SubStr()`/`Left()`/`Right()` slicing behavior, and `Replicate()` overflow/preservation behavior, including Clipper-style overflow thresholds, anchored to the upstream oracle when available. The executable side also includes a dedicated host-C harness covering embedded `Chr(0)` preservation in selected string helpers so the support layer is exercised beyond the Rust-only runtime surface.
