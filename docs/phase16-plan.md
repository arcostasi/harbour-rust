# Phase 16 Plan: Runtime Fidelity

## Purpose

Phase 16 opens the post-`0.5.0-alpha` development line by shifting the first active compatibility corridor from advanced PP growth to runtime fidelity.

The goal is not broad Harbour API coverage. The goal is to add small, measurable runtime/library compatibility slices with explicit fixtures, documented limits, and stable cross-platform behavior.

## First Corridor

The first planned slice is `hb_JsonDecode`.

Acceptance for the first slice:

- define how JSON `null`, booleans, numbers, strings, arrays, and objects map into the current Harbour Rust `Value` model;
- add focused runtime unit tests for successful scalar, array, and object decoding;
- add at least one integration or compatibility fixture that exercises the public compiler/runtime path;
- document unsupported behavior, especially encoding, duplicate object keys, numeric precision edge cases, error reporting, and any Harbour-specific flags that are not implemented;
- keep the implementation independent and original while using `harbour-core` behavior as the compatibility oracle where practical.

## Expected Follow-Up Corridors

After `hb_JsonDecode`, the next candidates are:

- `hb_gzCompress`, once string/binary behavior and byte-preservation expectations are clear enough;
- `hb_processRun`, once process execution semantics, exit status handling, environment behavior, quoting, and platform differences are specified.

These should be implemented one focused fixture group at a time. They should not become broad rewrites of the runtime surface.

## Deferred Corridors

The following areas are relevant but intentionally deferred:

- sockets: `hb_socketOpen`, `hb_socketRecv`, `hb_socketSend`;
- threading and synchronization: `hb_threadStart`, `hb_mutexCreate`, `hb_mutexLock`.

They require decisions about cross-platform IO, blocking behavior, ownership, shared runtime state, scheduler expectations, and CI-safe tests. They should not be claimed as supported until those decisions are explicit and covered by tests.

## Execution Rules

- Compatibility claims must come from tests, not intention.
- Prefer one builtin or API family per slice.
- Add fixture-first tests before expanding behavior.
- Keep expected divergences in `COMPATIBILITY.md`.
- Update `docs/runtime.md` and `docs/test-strategy.md` when a slice changes runtime behavior or validation policy.
- Avoid introducing native backend assumptions; the executable path remains the C backend for this phase.

## Validation

Each phase 16 runtime slice should normally run:

```text
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace
```

When a fixture can be compared with Harbour, also run the compatibility comparator with an available Harbour runner.
