# Harbour Rust

- [English](./README.md)
- [Português do Brasil](./README.pt-BR.md)

Harbour Rust é um projeto de compilador open source, independente e orientado pela comunidade, escrito em Rust e voltado à compatibilidade com CA-Clipper e Harbour.

Este repositório é mantido como uma contribuição pública e sem fins lucrativos para a comunidade de software. Ele não é afiliado, endossado nem patrocinado pelo Harbour Project, xHarbour, CA-Clipper ou por titulares de marcas relacionadas a esses nomes. Esses nomes são usados apenas para descrever metas de compatibilidade e contexto histórico.

O projeto também tem uma origem pessoal: xBase foi o primeiro ambiente de programação estudado de forma mais séria pelo mantenedor e teve papel importante na sua trajetória como desenvolvedor. Esse contexto inspira o trabalho, mas o repositório foi pensado para permanecer tecnicamente rigoroso, orientado à comunidade e documentado de forma profissional.

## Objetivos

- oferecer uma implementação moderna em Rust para uma família de linguagens xBase historicamente importante;
- priorizar compatibilidade observável antes de elegância interna nas fases iniciais;
- manter a arquitetura do compilador compreensível, testável e evolutiva;
- desenvolver o projeto em público com documentação, testes e governança adequados para colaboração de longo prazo.

## Estado Atual

O repositório concluiu as fases 0 a 12 do roadmap inicial e está em preparação para a linha de release `0.4.0-alpha`.

Destaques atuais:

- pipelines de parser, HIR, sema, runtime, IR e geração de código C implementados;
- compatibilidade procedural, arrays, `STATIC`, memvars, codeblocks e parte do pré-processador disponíveis;
- base inicial de DBF/RDD presente;
- CLI, harnesses de regressão, benchmarks, scaffold de fuzzing e workflows de release configurados.

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

## Licença

Este repositório é distribuído sob a [Apache License 2.0](./LICENSE).
