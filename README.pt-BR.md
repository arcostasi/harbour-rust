# Harbour Rust

[![CI](https://github.com/arcostasi/harbour-rust/actions/workflows/quality.yml/badge.svg)](https://github.com/arcostasi/harbour-rust/actions/workflows/quality.yml)
[![Release](https://github.com/arcostasi/harbour-rust/actions/workflows/release.yml/badge.svg)](https://github.com/arcostasi/harbour-rust/actions/workflows/release.yml)
[![Latest Pre-release](https://img.shields.io/github/v/release/arcostasi/harbour-rust?include_prereleases&label=latest%20pre-release)](https://github.com/arcostasi/harbour-rust/releases)
[![License: Apache-2.0](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](./LICENSE)

![Harbour Rust social preview](./docs/assets/harbour-rust-social-preview.png)

Projeto de compilador em Rust para compatibilidade com CA-Clipper/Harbour, com backend C executável e pragmático, CLI moderna e foco em modernização de sistemas xBase.

[English](./README.md) | [Português do Brasil](./README.pt-BR.md)

[Release atual](https://github.com/arcostasi/harbour-rust/releases/tag/0.5.0-alpha) | [Todas as releases](https://github.com/arcostasi/harbour-rust/releases) | [Documentação](./docs/README.pt-BR.md) | [Contribuição](./CONTRIBUTING.pt-BR.md)

Harbour Rust é um projeto de compilador open source, independente e orientado pela comunidade, escrito em Rust e voltado à compatibilidade com CA-Clipper e Harbour.

Este repositório é mantido como uma contribuição pública e sem fins lucrativos para a comunidade de software. Ele não é afiliado, endossado nem patrocinado pelo Harbour Project, xHarbour, CA-Clipper ou por titulares de marcas relacionadas a esses nomes. Esses nomes são usados apenas para descrever metas de compatibilidade e contexto histórico.

O projeto também tem uma origem pessoal: xBase foi o primeiro ambiente de programação estudado de forma mais séria pelo mantenedor e teve papel importante na sua trajetória como desenvolvedor. Esse contexto inspira o trabalho, mas o repositório foi pensado para permanecer tecnicamente rigoroso, orientado à comunidade e documentado de forma profissional.

## Objetivos

- oferecer uma implementação moderna em Rust para uma família de linguagens xBase historicamente importante;
- priorizar compatibilidade observável antes de elegância interna nas fases iniciais;
- manter a arquitetura do compilador compreensível, testável e evolutiva;
- desenvolver o projeto em público com documentação, testes e governança adequados para colaboração de longo prazo.

## Estado Atual

O repositório concluiu as fases 0 a 14 do roadmap atual e empacotou uma primeira expansão de compatibilidade da fase 15 para a linha de release `0.5.0-alpha`.

Destaques atuais:

- pipelines de parser, HIR, sema, runtime, IR e backend C executável atual implementados;
- compatibilidade procedural, arrays, `STATIC`, memvars, codeblocks e marcadores avançados selecionados do pré-processador disponíveis;
- base inicial de DBF/RDD presente;
- CLI, harnesses de regressão, benchmarks, scaffold de fuzzing, workflows de release e validação de CI multiplataforma configurados.

## Releases

- Pre-release atual: [0.5.0-alpha](https://github.com/arcostasi/harbour-rust/releases/tag/0.5.0-alpha)
- Todas as releases: [github.com/arcostasi/harbour-rust/releases](https://github.com/arcostasi/harbour-rust/releases)
- Assets da pre-release atual:
  - [Linux x86_64](https://github.com/arcostasi/harbour-rust/releases/download/0.5.0-alpha/harbour-rust-cli-0.5.0-alpha-linux-x86_64.zip)
  - [macOS aarch64](https://github.com/arcostasi/harbour-rust/releases/download/0.5.0-alpha/harbour-rust-cli-0.5.0-alpha-macos-aarch64.zip)
  - [Windows x86_64](https://github.com/arcostasi/harbour-rust/releases/download/0.5.0-alpha/harbour-rust-cli-0.5.0-alpha-windows-x86_64.zip)
  - [SHA256SUMS.txt](https://github.com/arcostasi/harbour-rust/releases/download/0.5.0-alpha/SHA256SUMS.txt)
  - [benchmark-report.md](https://github.com/arcostasi/harbour-rust/releases/download/0.5.0-alpha/benchmark-report.md)

## Documentação

Comece por aqui:

- [Roadmap](./ROADMAP.pt-BR.md)
- [Compatibilidade](./COMPATIBILITY.pt-BR.md)
- [Contribuição](./CONTRIBUTING.pt-BR.md)
- [Governança](./GOVERNANCE.pt-BR.md)
- [Segurança](./SECURITY.pt-BR.md)
- [Suporte](./SUPPORT.pt-BR.md)
- [Política de Proveniência e Copyright](./PROVENANCE.pt-BR.md)
- [Centro de Documentação](./docs/README.pt-BR.md)

Guias técnicos:

- [Visão Geral Técnica](./docs/pt-BR/technical/overview.md)
- [Arquitetura](./docs/pt-BR/technical/architecture.md)
- [Runtime](./docs/pt-BR/technical/runtime.md)
- [CLI](./docs/pt-BR/technical/cli.md)
- [Estratégia de Testes](./docs/pt-BR/technical/test-strategy.md)

## Início Rápido

```text
cargo fmt --all
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace
cargo run -p harbour-rust-cli -- help
```

## Posicionamento Open Source

Harbour Rust é:

- open source;
- orientado à comunidade;
- sem fins lucrativos como iniciativa de projeto;
- mantido de forma independente;
- pensado como uma contribuição técnica respeitosa para um ecossistema clássico.

Contribuições são bem-vindas, mas todo material enviado precisa ser original ou legalmente reutilizável. Consulte [PROVENANCE.pt-BR.md](./PROVENANCE.pt-BR.md) para ver a política do repositório sobre originalidade, referências ao upstream, tradução e uso de material de terceiros.

## Comunidade

- Use GitHub Issues para bugs focados, problemas de compatibilidade e itens de trabalho com escopo claro.
- Use GitHub Discussions para dúvidas, ideias e conversas de design mais amplas quando Discussions estiver habilitado no repositório.
- Use pull requests para mudanças concretas e revisáveis, com testes e documentação sincronizada.

Os templates em `.github/` foram pensados para atender contribuidores em Inglês e Português sem alterar a política canônica do projeto.

## Licença

Este repositório é distribuído sob a [Apache License 2.0](./LICENSE).
