# Backend C

- [English](../../en/technical/codegen-c.md)
- [Português do Brasil](./codegen-c.md)

## Papel

O backend C emite C legível a partir da IR e depende de um compilador C host para produzir executáveis.

## Prioridades de Design

- legibilidade acima de esperteza;
- chamadas explícitas para helpers de runtime;
- crescimento do backend em slices pequenos apoiados por compatibilidade;
- diagnósticos claros quando construções da IR ainda não são executáveis.

## Baseline Atual

O backend já suporta:

- rotinas procedurais e returns;
- controle de fluxo como `IF`, `DO WHILE` e `FOR`;
- storage `STATIC` no subconjunto atualmente suportado;
- arrays, leituras e escritas indexadas e builtins selecionados;
- helpers ligados a memvar, macro reads e caminhos de execução com codeblocks sem captura.

## Estado Atual

O backend C é o backend executável principal do projeto e a ponte mais importante entre pesquisa de compilador e compatibilidade prática executável.
