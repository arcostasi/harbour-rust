---
name: add-compat-test
description: Adicionar um teste de compatibilidade pequeno e focado para behaviour Clipper/Harbour, usando harbour-core como oráculo quando possível.
---

# Add Compat Test

## Leia antes de editar

- `AGENTS.md`
- `COMPATIBILITY.md`
- `docs/test-strategy.md`

## Fluxo

1. Escolha um comportamento específico.
2. Crie fixture `.prg` mínima.
3. Defina saída esperada ou diferença documentada.
4. Adicione teste automatizado.
5. Atualize `COMPATIBILITY.md`.

## Não fazer

- empacotar várias features no mesmo teste,
- importar corpus grande sem curadoria,
- corrigir múltiplas features na mesma task sem necessidade.
