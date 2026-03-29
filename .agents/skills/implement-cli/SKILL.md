---
name: implement-cli
description: Implementar ou estender a CLI do harbour-rust com foco em pipeline, opções, diagnósticos e experiência do desenvolvedor.
---

# Implement CLI

## Leia antes de editar

- `AGENTS.md`
- `ROADMAP.md`
- `docs/cli.md`
- `docs/diagnostics.md`

## Objetivo

Adicionar ou melhorar um comando, opção ou fluxo de pipeline do CLI.

## Fluxo

1. Identifique o comando ou opção a implementar.
2. Verifique se todas as fases do pipeline necessárias já estão implementadas.
3. Implemente no `harbour-rust-cli`.
4. Adicione teste de integração.
5. Atualize help/usage se aplicável.
6. Atualize `docs/cli.md`.

## Não fazer

- implementar pipeline no CLI que deveria estar em crate separado,
- esconder erros de compilação com mensagens genéricas,
- adicionar opção sem teste,
- alterar comportamento de comandos existentes sem documentar.

## Definition of done

- comando funciona para caso feliz,
- diagnósticos claros para erros,
- teste de integração cobrindo o novo fluxo,
- help/usage consistente,
- docs atualizadas.
