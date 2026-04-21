# Release

- [English](../../en/technical/release.md)
- [Português do Brasil](./release.md)

## Propósito

Este documento descreve o baseline de preparação de release do Harbour Rust e as expectativas atuais para uma release alpha.

## Alvo Atual

- linha de release pública: `0.5.0-alpha`
- linha de versão dos manifests: `0.5.0-alpha.0`

## Preflight Local

```text
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace
cargo check --manifest-path fuzz/Cargo.toml
cargo run -p harbour-rust-tests --bin benchmark-suite -- --fixture examples/hello.prg --iterations 1
cargo build --release -p harbour-rust-cli
```

## Baseline do Workflow de Release

O workflow de release do repositório atualmente valida:

- formatação;
- clippy;
- testes do workspace;
- compilação do harness de fuzzing;
- benchmark smoke;
- build release de `harbour-rust-cli`;
- empacotamento de assets da GitHub Release para Windows, Linux e macOS;
- publicação de assets `.zip` mais `SHA256SUMS.txt` quando o workflow roda a partir de uma tag.

O workflow público de quality agora também valida:

- `cargo test --workspace` em Ubuntu, Windows e macOS;
- um smoke run de `harbour-rust-cli help` nessas mesmas três plataformas;
- o baseline mais pesado de `fmt`/`clippy`/fuzz/benchmark no Ubuntu como gate canônico de qualidade.

## Disciplina de Release

- toda release precisa apontar para um baseline de compatibilidade documentado;
- releases alpha podem sair com limites conhecidos documentados;
- notas públicas devem descrever o que já está implementado, o que é parcial e o que está intencionalmente fora de escopo.

## Documentos Relacionados

- [Governance](../../../GOVERNANCE.pt-BR.md)
- [Compatibility](../../../COMPATIBILITY.pt-BR.md)
