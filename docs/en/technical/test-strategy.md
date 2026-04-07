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

The repository already has a practical test matrix covering compiler slices, runtime behavior, CLI execution, golden snapshots, compare tooling, benchmark smoke, and fuzz compilation checks. The compatibility corpus also includes focused runtime string fixtures for control-character trim behavior, `At()` edge cases, `SubStr()`/`Left()`/`Right()` slicing behavior, `Replicate()` overflow/preservation behavior, `Val()` parsing edges around trailing dots, repeated punctuation, repeated signs, exponent-like input, mixed punctuation such as `13.1.9`, and space-separated decimal fragments such as `12. 0`/`12 .10`, `Str()` formatting edges such as negative-width padding, width-driven rounding behavior, default-width positive large numbers, and float-literal display scale, plus NIL argument handling, focused numeric fixtures for `Round()`/`Int()` sign, scale, and large-number behavior, focused `Mod()`/`ValType()`/`Empty()` fixtures for argument errors, sign handling, array typing, codeblock typing, and emptiness edge cases, focused `Max()`/`Min()`/`Abs()` fixtures for equality, negative comparisons, magnitude, and argument-error behavior, focused `Type()`/`Len()` fixtures for trimmed source text, empty-array typing, empty strings, empty arrays, and embedded `Chr(0)` length behavior, and focused preprocessor fixtures for escaped optional replacements, selected optional-clause reordering, selected logical result markers, a minimal blockify-result `<{id}>` subset, a macro-oriented quoted-result `<"id">` subset, a macro-oriented smart-result `<(id)>` subset, a focused nested optional/list subset (`AAA`/`SET`/`AVG`/`INSERT`/`INSERT2`), repeated optional-clause expansion and continued source-line invocation for `INSERT`/`INSERT2`, and an expanded macro pattern-marker `<id:&>` subset with selected operator spillover, longer multi-segment chains, and selected `&(expr)` mixtures, all anchored to the upstream oracle when available. The executable side also includes dedicated host-C harnesses covering embedded `Chr(0)` preservation in selected string helpers and `Empty()`/`ValType()` behavior for codeblocks, arrays, and host-C error values so the support layer is exercised beyond the Rust-only runtime surface.
