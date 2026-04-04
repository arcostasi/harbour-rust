# Release

## Objetivo

Ter um checklist operacional mínimo para preparar uma release alpha do `harbour-rust`.

## Release alvo atual

- release: `0.4.0-alpha`
- versão de manifests: `0.4.0-alpha.0`
- notas da release: `docs/releases/0.4.0-alpha.md`

## Pré-flight local

```text
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace
cargo check --manifest-path fuzz/Cargo.toml
cargo run -p harbour-rust-tests --bin benchmark-suite -- --fixture examples/hello.prg --iterations 1
cargo build --release -p harbour-rust-cli
```

## Workflow de release

O workflow `.github/workflows/release.yml` executa:

- `fmt`
- `clippy`
- `cargo test --workspace`
- `cargo check --manifest-path fuzz/Cargo.toml`
- geração de benchmark report
- `cargo build --release -p harbour-rust-cli`

## Artefatos atuais

- binário release de `harbour-rust-cli`
- relatório markdown de benchmark

## Critérios mínimos

- fase alvo refletida em `README.md` e `ROADMAP.md`
- `COMPATIBILITY.md` atualizada com divergências conhecidas
- `docs/test-strategy.md` coerente com a suíte real
- benchmark smoke executável
- harness de fuzz compila
- versão alvo alinhada em `Cargo.toml` do workspace e crates
- notas de release curtas registradas em `docs/releases/`
