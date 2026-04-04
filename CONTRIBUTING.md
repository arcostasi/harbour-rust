# Contributing

- [English](./CONTRIBUTING.md)
- [Português do Brasil](./CONTRIBUTING.pt-BR.md)

## Scope

Harbour Rust welcomes contributions to code, tests, documentation, translation, tooling, release automation, and issue triage.

## Before You Start

Read these documents first:

1. [README.md](./README.md)
2. [ROADMAP.md](./ROADMAP.md)
3. [COMPATIBILITY.md](./COMPATIBILITY.md)
4. [GOVERNANCE.md](./GOVERNANCE.md)
5. [PROVENANCE.md](./PROVENANCE.md)
6. [docs/README.md](./docs/README.md)

## Development Workflow

1. Pick one small objective at a time.
2. Avoid mixing broad refactors with new features.
3. Add or update tests for the behavior you changed.
4. Update documentation in English and Portuguese when the change is user-facing or policy-facing.
5. Keep the build green before asking for review.

Recommended validation:

```text
cargo fmt --all
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace
```

## Documentation and Translation Rules

- English is the canonical language for public documentation.
- Portuguese (Brazil) is the maintained secondary translation.
- If you change a public document in English, update the Portuguese counterpart in the same change whenever possible.
- If translation must be deferred, explicitly mark the mismatch in the pull request.
- Do not commit machine-translated text without human review.

See [docs/en/documentation-standards.md](./docs/en/documentation-standards.md) and [docs/en/translation-workflow.md](./docs/en/translation-workflow.md).

## Originality and Provenance

By contributing, you confirm that:

- your contribution is original, or
- you have the right to submit it under this repository's license, and
- you are not copying code, documentation, or proprietary materials from third parties without permission.

Using `harbour-core` as a behavior oracle is encouraged. Copying large code blocks, documentation passages, or proprietary manuals is not.

## Commit Style

Use Conventional Commits and keep each commit focused on one coherent intent.

When relevant to the roadmap, include:

- `Phase: <n>`
- `Task: <short-slug>`
- `Tests: <short-summary>`

## Conduct

All contributors are expected to follow [CODE_OF_CONDUCT.md](./CODE_OF_CONDUCT.md).
