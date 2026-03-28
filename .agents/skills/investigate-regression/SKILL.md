---
name: investigate-regression
description: Investigar regressões do harbour-rust com foco em bisseção local, redução de caso, atualização de diagnóstico e adição de teste de regressão.
---

# Investigate Regression

## Leia antes de editar

- `AGENTS.md`
- `COMPATIBILITY.md`
- `docs/test-strategy.md`
- doc temática da camada afetada

## Fluxo

1. Reproduza a falha.
2. Reduza para o menor fixture possível.
3. Determine a camada responsável: `pp`, `lexer`, `parser`, `sema`, `runtime`, `codegen`.
4. Corrija sem misturar melhorias não relacionadas.
5. Adicione teste de regressão.
6. Atualize `COMPATIBILITY.md` se o status mudou.

## Não fazer

- refactor oportunista,
- correção sem teste,
- alterar múltiplas camadas sem necessidade explícita.
