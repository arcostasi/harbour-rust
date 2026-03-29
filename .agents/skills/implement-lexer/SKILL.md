---
name: implement-lexer
description: Implementar ou estender o lexer do harbour-rust com foco em tokens, spans, diagnósticos e baselines golden.
---

# Implement Lexer

## Leia antes de editar

- `AGENTS.md`
- `ROADMAP.md`
- `docs/lexer.md`
- `docs/dialect-clipper.md`

## Objetivo

Adicionar ou corrigir tokenização de uma fatia bem definida da linguagem.

## Fluxo

1. Identifique o token ou grupo de tokens a implementar.
2. Consulte `harbour-core/src/compiler/harbour.y` e `harbour-core/doc/pp.txt` para referência.
3. Implemente no lexer com spans corretos.
4. Adicione testes unitários.
5. Atualize baselines golden se necessário.
6. Atualize `COMPATIBILITY.md` e `docs/lexer.md`.

## Não fazer

- alterar o parser ou o PP sem necessidade real,
- misturar token novo com refactor amplo do lexer,
- adicionar keywords sem fixture de teste.

## Definition of done

- token produzido corretamente para inputs válidos,
- diagnóstico claro para inputs inválidos,
- span exato com linha e coluna,
- testes unitários e snapshots estáveis,
- docs atualizadas.
