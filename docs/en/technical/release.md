# Release

- [English](./release.md)
- [Português do Brasil](../../pt-BR/technical/release.md)

## Purpose

This document describes the release preparation baseline for Harbour Rust and the current expectations for an alpha release.

## Current Target

- public release line: `0.5.0-alpha`
- manifest version line: `0.5.0-alpha.0`

## Local Preflight

```text
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace
cargo check --manifest-path fuzz/Cargo.toml
cargo run -p harbour-rust-tests --bin benchmark-suite -- --fixture examples/hello.prg --iterations 1
cargo build --release -p harbour-rust-cli
```

## Release Workflow Baseline

The repository release workflow currently validates:

- formatting;
- clippy;
- workspace tests;
- fuzz harness compilation;
- benchmark smoke;
- release build of `harbour-rust-cli`;
- packaging of GitHub release assets for Windows, Linux, and macOS;
- publication of `.zip` assets plus `SHA256SUMS.txt` when the workflow runs from a tag.

The public quality workflow now also validates:

- `cargo test --workspace` on Ubuntu, Windows, and macOS;
- a `harbour-rust-cli help` smoke run on the same three platforms;
- the heavier `fmt`/`clippy`/fuzz/benchmark baseline on Ubuntu as the canonical quality gate.

## Release Discipline

- every release must point to a documented compatibility baseline;
- alpha releases may ship with documented known limits;
- public notes should describe what is implemented, what is partial, and what is intentionally out of scope.

## Related Documents

- [Governance](../../../GOVERNANCE.md)
- [Compatibility](../../../COMPATIBILITY.md)
