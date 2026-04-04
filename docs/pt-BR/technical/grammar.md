# Gramática e Parser

- [English](../../en/technical/grammar.md)
- [Português do Brasil](./grammar.md)

## Papel

O parser usa parsing recursivo para statements e estilo Pratt para expressões. Ele constrói a AST concreta e fornece diagnósticos sintáticos com recuperação limitada.

## Baseline Atual

O subconjunto alpha público já inclui:

- `PROCEDURE` e `FUNCTION`;
- `RETURN`;
- `LOCAL` e `STATIC`;
- `IF / ELSE / ENDIF`;
- `DO WHILE / ENDDO`;
- `FOR / NEXT`;
- sintaxe de impressão com `?`;
- arrays, operadores compostos, codeblocks, sintaxe de memvar e sintaxe inicial de macro read.

## Regras de Design

- manter a AST explícita e estável;
- preservar spans, mas não trivia, na AST;
- evitar resolução semântica dentro do parser;
- preferir diagnósticos recuperáveis a abortar cedo.

## Estado Atual

A camada de gramática já é ampla o suficiente para o pipeline alpha atual e alimenta HIR, semântica, integração com runtime e fixtures de compatibilidade.
