---
name: implement-hir-sema
description: Implementar ou estender HIR lowering e análise semântica do harbour-rust com foco em escopos, resolução de símbolos e diagnósticos.
---

# Implement HIR / Sema

## Leia antes de editar

- `AGENTS.md`
- `ROADMAP.md`
- `docs/hir.md`
- `docs/sema.md`
- `docs/architecture.md`

## Objetivo

Estender o lowering AST -> HIR ou a análise semântica sobre a HIR.

## Fluxo

### Para HIR (lowering)

1. Identifique o nó AST que precisa de lowering.
2. Implemente o lowering em `harbour-rust-hir`.
3. Adicione teste de lowering com fixture `.prg`.
4. Verifique que a sema não quebrou com o nó novo.

### Para Sema (análise)

1. Identifique o diagnóstico ou resolução a implementar.
2. Implemente na sema sem reescrever a HIR.
3. Use side tables para anotações.
4. Adicione teste com fixture `.prg` e golden `.errors`.
5. Verifique que fases posteriores (IR, codegen) continuam verdes.

## Não fazer

- reescrever a HIR na sema — use side tables,
- misturar lowering novo com resolução nova no mesmo PR,
- alterar o parser sem necessidade da task,
- ignorar walk completo em novos nós (ex.: elementos de array).

## Definition of done

- lowering produz HIR correta para o novo nó,
- sema percorre e anota o novo nó,
- diagnósticos cobrem caso feliz e erro,
- testes de regressão existentes continuam verdes,
- docs atualizadas.
