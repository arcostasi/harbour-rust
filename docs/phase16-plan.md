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

## Delivered Follow-Up Slice

The first adjacent follow-up slice now delivered is `hb_gzCompressBound`.

Acceptance for this slice:

- accept either source text or a numeric source length;
- return the maximum compressed-size estimate as an integer using the observable upstream gzip-bound path;
- report argument errors for unsupported types instead of implying broad compression support;
- exercise the runtime surface, the public compiler/codegen path, and a focused compatibility baseline;
- keep full `hb_gzCompress` output semantics explicitly out of scope for this step.

The adjacent raw-zlib bound slice now delivered is `hb_ZCompressBound`.

Acceptance for this slice:

- accept either source text or a numeric source length;
- return the raw zlib maximum compressed-size estimate without the gzip header adjustment;
- report argument errors for unsupported types;
- exercise the runtime surface, the public compiler/codegen path, and a focused compatibility baseline;
- keep raw `hb_ZCompress` and decompression APIs out of scope.

The next delivered slice in the same corridor is the minimal one-argument `hb_gzCompress`.

Acceptance for this slice:

- accept only the direct `hb_gzCompress( cData )` form;
- preserve binary output through the current runtime string model so the result stays observable in tests;
- emit a valid gzip stream without requiring host zlib linkage in the CLI executable path;
- keep destination buffers, by-reference result reporting, compression-level selection, and byte-for-byte parity with upstream explicitly out of scope;
- cover runtime unit tests, public compiler/runtime execution, and a focused compatibility baseline.

The next adjacent slice now delivered in the same corridor is observable `@nResult` writeback for `hb_gzCompress`.

Acceptance for this slice:

- accept the focused `hb_gzCompress( cData, NIL, @nResult )` form without implying generic by-reference call support;
- write back `0` to `@nResult` on the current successful path, including empty-string input;
- cover runtime unit tests, public compiler/runtime execution, and a focused compatibility baseline;
- keep destination buffers, compression level, and broader by-reference semantics explicitly out of scope.

The next adjacent slice now delivered is numeric destination-length handling for the same builtin.

Acceptance for this slice:

- accept `hb_gzCompress( cData, nDstBufLen )` and `hb_gzCompress( cData, nDstBufLen, @nResult )` for numeric destination sizes;
- return a compressed binary string and write `0` when the current deterministic gzip output fits;
- return `NIL` and write `-5` when the supplied size is too small, matching the observable `Z_BUF_ERROR` branch;
- preserve argument errors for unsupported second-slot types and keep `@cBuffer` plus compression-level selection out of scope;
- cover runtime unit tests, public compiler/runtime execution, and a focused compatibility baseline.

The next adjacent zlib slice now delivered is destination-buffer writeback for the same builtin.

Acceptance for this slice:

- accept `hb_gzCompress( cData, @cBuffer )` and `hb_gzCompress( cData, @cBuffer, @nResult )` for a preallocated string buffer;
- use the current buffer length as the destination limit;
- write the compressed string back to `@cBuffer` and `@nResult := 0` when the output fits;
- return `NIL`, preserve the current buffer value, and write `@nResult := -5` when the buffer is too small;
- keep compression-level selection, zlib byte-for-byte parity, and generic by-reference call semantics out of scope.

The next adjacent zlib slice now delivered is `hb_ZError`.

Acceptance for this slice:

- accept a numeric error code and return the stable message text for common zlib codes;
- cover the observable empty-string result for `0` and selected negative error codes such as `-5` and `-6`;
- report a focused argument error for missing or non-numeric input in the current runtime model;
- cover runtime unit tests, public compiler/runtime execution, and a focused compatibility baseline tied to `hbzlib.c`;
- keep host-zlib linkage and undefined out-of-table behavior out of scope.

The first process-execution slice now delivered is minimal `hb_processRun`.

Acceptance for this slice:

- accept only `hb_processRun( cCommand )`;
- execute the command through the host shell in a CI-safe way for the public executable path;
- return the normalized integer exit status for normally terminated commands;
- report `BASE 4001` argument errors for missing or non-string input;
- keep stdin, stdout/stderr capture by reference, detach/async behavior, environment customization, and advanced quoting out of scope;
- cover runtime unit tests, public compiler/runtime execution, and a focused compatibility baseline tied to `hbprocfn.c`.

The next adjacent process-execution slice now delivered is stdout capture.

Acceptance for this slice:

- accept the focused `hb_processRun( cCommand, NIL, @cStdOut )` form without implying generic by-reference call support;
- write the captured stdout bytes back to the third argument slot as a runtime string;
- keep the second argument restricted to `NIL` in this slice;
- keep stdin, stderr capture, merged streams, detach/async behavior, environment customization, and advanced quoting out of scope;
- cover runtime unit tests, public compiler/runtime execution, and the focused compatibility baseline.

## Expected Follow-Up Corridors

After `hb_JsonDecode`, the next candidates are:

- the remaining zlib surface, especially raw `hb_ZCompress`, decompression APIs, `hb_gzCompress` compression-level selection, zlib byte-for-byte parity, and APIs beyond the delivered `hb_ZCompressBound`/`hb_ZError` slices;
- the remaining `hb_processRun` surface, especially stdin, stderr capture, merged streams, detach behavior, environment behavior, quoting, and platform differences.

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
