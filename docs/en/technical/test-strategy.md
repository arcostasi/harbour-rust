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

## Current Status

The repository already has a practical test matrix covering compiler slices, runtime behavior, CLI execution, golden snapshots, compare tooling, benchmark smoke, and fuzz compilation checks.
