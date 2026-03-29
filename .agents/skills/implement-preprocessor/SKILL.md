---
name: implement-preprocessor
description: Implementar ou estender o pré-processador do harbour-rust com foco em diretivas, expansão e compatibilidade com Clipper/Harbour.
---

# Implement Preprocessor

## Leia antes de editar

- `AGENTS.md`
- `ROADMAP.md`
- `docs/preprocessor.md`
- `docs/dialect-clipper.md`
- `harbour-core/doc/pp.txt`

## Objetivo

Adicionar ou estender uma diretiva ou mecanismo de expansão do PP.

## Fluxo

1. Identifique a diretiva ou mecanismo alvo.
2. Consulte `harbour-core/doc/pp.txt` e `harbour-core/src/pp/ppcore.c`.
3. Implemente no `harbour-rust-pp` sem acoplar parser ou runtime.
4. Adicione testes unitários e de integração.
5. Se aplicável, adicione teste de compatibilidade com fixture `.prg`.
6. Atualize `COMPATIBILITY.md` e `docs/preprocessor.md`.

## Não fazer

- misturar `#define` com `#command` no mesmo PR,
- alterar o lexer principal sem necessidade,
- ignorar corner cases documentados no upstream,
- expandir escopo para diretivas que não são alvo da task.

## Definition of done

- diretiva processada corretamente para inputs válidos,
- diagnóstico claro para inputs malformados,
- mapeamento de origem preservado para diagnósticos,
- testes unitários e de integração passando,
- fixture `.prg` com golden file quando aplicável,
- docs atualizadas.
