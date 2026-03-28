---
name: implement-parser
description: Implementar ou estender o parser do harbour-rust sem misturar runtime, CLI ou codegen, com foco em gramática, AST, diagnósticos e testes.
---

# Implement Parser

## Leia antes de editar

- `AGENTS.md`
- `ROADMAP.md`
- `COMPATIBILITY.md`
- `docs/grammar.md`
- `docs/dialect-clipper.md` ou `docs/dialect-harbour.md`

## Objetivo

Adicionar ou estender parsing de uma fatia bem definida da linguagem.

## Fluxo

1. Defina a menor fatia sintática possível.
2. Mapeie a produção correspondente em `harbour-core/src/compiler/harbour.y`.
3. Adicione ou ajuste nós de AST.
4. Implemente parser e recuperação de erro mínima.
5. Adicione testes unitários e snapshots.
6. Atualize `COMPATIBILITY.md` e `docs/grammar.md`.

## Não fazer

- alterar runtime sem necessidade real,
- refactor amplo fora do parser,
- misturar nova sintaxe com reestruturação grande.

## Definition of done

- parsing da fatia funcionando,
- diagnóstico básico coberto,
- snapshots estáveis,
- docs atualizadas.
