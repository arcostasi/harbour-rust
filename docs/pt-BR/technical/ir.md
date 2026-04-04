# IR

- [English](../../en/technical/ir.md)
- [Português do Brasil](./ir.md)

## Papel

A IR é a representação voltada ao backend entre análise semântica e geração de código. Ela continua independente de target, mas fica mais próxima da forma executável do que a HIR.

## Baseline Atual

A IR atual preserva:

- rotinas e controle de fluxo estruturado;
- leituras explícitas e alvos de atribuição;
- statements `STATIC`, `PRIVATE` e `PUBLIC`;
- arrays, indexação, codeblocks, expressões de macro e operações dinâmicas selecionadas.

## Regras de Design

- manter a IR agnóstica ao backend;
- preservar estrutura até que flattening seja realmente necessário;
- emitir erros explícitos de lowering em vez de descartar construções não suportadas silenciosamente.

## Estado Atual

A IR já é suficiente para alimentar o backend C atual no subconjunto alpha e funciona como uma fronteira clara entre semântica de frontend e lowering executável.
