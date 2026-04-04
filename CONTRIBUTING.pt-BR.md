# Contribuição

- [English](./CONTRIBUTING.md)
- [Português do Brasil](./CONTRIBUTING.pt-BR.md)

## Escopo

Harbour Rust aceita contribuições em código, testes, documentação, tradução, ferramentas, automação de release e triagem de issues.

## Antes de Começar

Leia estes documentos primeiro:

1. [README.pt-BR.md](./README.pt-BR.md)
2. [ROADMAP.pt-BR.md](./ROADMAP.pt-BR.md)
3. [COMPATIBILITY.pt-BR.md](./COMPATIBILITY.pt-BR.md)
4. [GOVERNANCE.pt-BR.md](./GOVERNANCE.pt-BR.md)
5. [PROVENANCE.pt-BR.md](./PROVENANCE.pt-BR.md)
6. [docs/README.pt-BR.md](./docs/README.pt-BR.md)

## Fluxo de Desenvolvimento

1. Escolha um objetivo pequeno por vez.
2. Evite misturar refactors amplos com features novas.
3. Adicione ou atualize testes para o comportamento alterado.
4. Atualize a documentação em Inglês e em Português quando a mudança for visível para usuários ou relevante como política.
5. Mantenha o build verde antes de pedir revisão.

Validação recomendada:

```text
cargo fmt --all
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace
```

## Regras de Documentação e Tradução

- Inglês é o idioma canônico da documentação pública.
- Português do Brasil é a tradução secundária mantida.
- Se você alterar um documento público em Inglês, atualize a contraparte em Português na mesma mudança sempre que possível.
- Se a tradução precisar ser adiada, registre isso explicitamente no pull request.
- Não faça commit de texto traduzido por máquina sem revisão humana.

Consulte [docs/en/documentation-standards.md](./docs/en/documentation-standards.md) e [docs/pt-BR/fluxo-de-traducao.md](./docs/pt-BR/fluxo-de-traducao.md).

## Originalidade e Proveniência

Ao contribuir, você confirma que:

- sua contribuição é original, ou
- você tem direito de submetê-la sob a licença deste repositório, e
- você não está copiando código, documentação ou material proprietário de terceiros sem permissão.

Usar `harbour-core` como oráculo de comportamento é incentivado. Copiar grandes blocos de código, trechos extensos de documentação ou manuais proprietários não é.

## Estilo de Commit

Use Conventional Commits e mantenha cada commit focado em uma intenção coerente.

Quando fizer sentido para o roadmap, inclua:

- `Phase: <n>`
- `Task: <slug-curto>`
- `Tests: <resumo-curto>`

## Conduta

Todas as pessoas contribuidoras devem seguir [CODE_OF_CONDUCT.pt-BR.md](./CODE_OF_CONDUCT.pt-BR.md).
