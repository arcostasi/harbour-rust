# Lexer

- [English](../../en/technical/lexer.md)
- [Português do Brasil](./lexer.md)

## Papel

O lexer transforma source `.prg` em um stream de tokens com spans, posições de linha/coluna e diagnósticos léxicos.

## Baseline Atual

O subconjunto alpha atual inclui:

- keywords case-insensitive;
- identificadores;
- inteiros e floats;
- strings com aspas;
- comentários, incluindo formas legadas de comentário de linha;
- operadores aritméticos, de comparação, atribuição, pós-fixo e macro.

## Regras de Design

- manter análise léxica separada de preocupações semânticas;
- preservar spans e posições desde o começo;
- tratar o pré-processador como camada separada;
- preferir tokenização determinística e erros léxicos explícitos.

## Estado Atual

O lexer está estável para o subconjunto atualmente implementado e já possui fixtures golden curados.
