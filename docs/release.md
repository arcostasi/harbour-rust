# Release

> Nota de transição: a versão pública bilíngue deste conteúdo está sendo migrada para [docs/en/technical/release.md](./en/technical/release.md) e [docs/pt-BR/technical/release.md](./pt-BR/technical/release.md).

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
- build matrix para Windows, Linux e macOS
- empacotamento automático de assets `.zip`
- geração de `SHA256SUMS.txt`
- publicação automática dos assets na GitHub Release da tag

Além disso, o workflow `.github/workflows/quality.yml` passa a validar:

- `cargo test --workspace` em Ubuntu, Windows e macOS
- smoke run de `harbour-rust-cli -- help` nas três plataformas
- baseline canônico de `fmt`, `clippy`, fuzz e benchmark smoke no Ubuntu

## Artefatos atuais

- assets `.zip` do `harbour-rust-cli` para Windows, Linux e macOS
- `SHA256SUMS.txt`
- relatório markdown de benchmark

## Critérios mínimos

- fase alvo refletida em `README.md` e `ROADMAP.md`
- `COMPATIBILITY.md` atualizada com divergências conhecidas
- `docs/test-strategy.md` coerente com a suíte real
- benchmark smoke executável
- harness de fuzz compila
- versão alvo alinhada em `Cargo.toml` do workspace e crates
- notas de release curtas registradas em `docs/releases/`
