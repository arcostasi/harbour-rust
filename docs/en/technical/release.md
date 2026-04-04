# Release

- [English](./release.md)
- [Português do Brasil](../../pt-BR/technical/release.md)

## Purpose

This document describes the release preparation baseline for Harbour Rust and the current expectations for an alpha release.

## Current Target

- public release line: `0.4.0-alpha`
- manifest version line: `0.4.0-alpha.0`

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
- release build of `harbour-rust-cli`.

## Release Discipline

- every release must point to a documented compatibility baseline;
- alpha releases may ship with documented known limits;
- public notes should describe what is implemented, what is partial, and what is intentionally out of scope.

## Related Documents

- [Governance](../../../GOVERNANCE.md)
- [Compatibility](../../../COMPATIBILITY.md)
