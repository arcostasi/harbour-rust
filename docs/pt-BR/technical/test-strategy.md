# Estratégia de Testes

- [English](../../en/technical/test-strategy.md)
- [Português do Brasil](./test-strategy.md)

## Princípio

Compatibilidade não é presumida. Ela é medida.

## Camadas Principais

- testes unitários para comportamento focado de cada crate;
- testes de integração ao longo do pipeline do compilador;
- testes golden e snapshot para saída observável estável;
- testes de compatibilidade contra comportamento curado do upstream;
- scaffold de fuzzing para superfícies ligadas a parsing;
- benchmark smoke para baseline reproduzível de tempo.

## Regras de Design

- manter fixtures pequenas e legíveis;
- promover bugs corrigidos para cobertura de regressão;
- preferir crescimento curado de corpus em vez de importações massivas sem controle;
- tratar alegações de compatibilidade como afirmações apoiadas por testes.
- permitir que testes de compatibilidade dependentes de oráculo sejam pulados de forma limpa na CI pública quando `harbour-core/` não estiver disponível.

## Estado Atual

O repositório já possui uma matriz prática de testes cobrindo slices do compilador, comportamento de runtime, execução via CLI, snapshots golden, compare tooling, benchmark smoke e checks de compilação de fuzzing. O corpus de compatibilidade também inclui fixtures focadas de strings para comportamento de trim com caracteres de controle, edge cases de `At()`, recorte em `SubStr()`/`Left()`/`Right()`, overflow/preservação em `Replicate()`, parsing de `Val()` em casos com ponto final sem fração, pontuação repetida, sinais repetidos e entradas parecidas com expoente, e edge cases de formatação de `Str()` como largura negativa e tratamento de argumentos `NIL`, ancoradas no oráculo do upstream quando disponível. O lado executável também inclui um harness dedicado de host C cobrindo a preservação de `Chr(0)` embutido em helpers selecionados de string, para exercitar essa camada além da surface apenas em Rust.
