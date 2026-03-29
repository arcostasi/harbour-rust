---
name: implement-rdd
description: Implementar ou estender o RDD/DBF do harbour-rust com foco em compatibilidade binária com arquivos Clipper/Harbour e design trait-based.
---

# Implement RDD

## Leia antes de editar

- `AGENTS.md`
- `ROADMAP.md`
- `docs/rdd.md`
- `harbour-core/src/rdd/` (referência)
- `harbour-core/tests/rddtest/` (corpus de testes)

## Objetivo

Adicionar ou estender uma operação de acesso a dados DBF no RDD.

## Fluxo

1. Identifique a operação ou feature a implementar.
2. Consulte `harbour-core/src/rdd/` para semântica de referência.
3. Implemente contra o trait `Rdd`.
4. Teste com arquivo DBF criado pelo Harbour para garantir compatibilidade binária.
5. Adicione teste unitário e de integração.
6. Atualize `COMPATIBILITY.md` e `docs/rdd.md`.

## Não fazer

- implementar RDD antes do frontend estar estável (Fase 10+),
- alterar formato de arquivo sem documentar divergência,
- ignorar edge cases de formato DBF (registros deletados, memo, etc.),
- acoplar RDD ao parser ou ao codegen.

## Definition of done

- operação funciona para caso feliz,
- compatibilidade binária verificada com arquivo Harbour,
- erros claros para formatos não suportados,
- testes passando,
- docs atualizadas.
