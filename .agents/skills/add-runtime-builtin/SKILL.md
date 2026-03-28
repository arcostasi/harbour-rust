---
name: add-runtime-builtin
description: Adicionar um builtin ao runtime do harbour-rust com comportamento compatível, testes direcionados e documentação mínima.
---

# Add Runtime Builtin

## Leia antes de editar

- `AGENTS.md`
- `ROADMAP.md`
- `COMPATIBILITY.md`
- `docs/runtime.md`
- `docs/test-strategy.md`

## Objetivo

Implementar um builtin por vez, com comportamento testado contra o upstream quando possível.

## Fluxo

1. Identifique o builtin e a fase alvo.
2. Procure referência em `harbour-core/src/rtl` e `harbour-core/utils/hbtest`.
3. Especifique comportamento feliz, erro e corner case mínimo.
4. Implemente no runtime sem ampliar escopo.
5. Adicione teste unitário, integração e compatibilidade se aplicável.
6. Atualize `COMPATIBILITY.md`.

## Não fazer

- adicionar muitos builtins na mesma task,
- esconder divergência conhecida,
- acoplar builtin novo a refactor amplo.
