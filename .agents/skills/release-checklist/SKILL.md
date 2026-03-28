---
name: release-checklist
description: Preparar uma release do harbour-rust verificando escopo da milestone, matriz de compatibilidade, testes, documentação e pendências conhecidas.
---

# Release Checklist

## Leia antes de editar

- `ROADMAP.md`
- `COMPATIBILITY.md`
- `docs/test-strategy.md`

## Fluxo

1. Confirme a milestone alvo.
2. Verifique se todas as fases prometidas estão refletidas em docs e testes.
3. Revise `COMPATIBILITY.md` para gaps e divergências abertas.
4. Execute a suíte necessária.
5. Atualize notas de release internas.
6. Liste riscos remanescentes de forma objetiva.

## Não fazer

- promover release com status implícito,
- esconder testes faltantes,
- mudar escopo da milestone no fim do processo.
