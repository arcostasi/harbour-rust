# Estratégia de testes

## Princípio

Compatibilidade não será assumida; será medida.

## Camadas

### Unitários

- lexer
- pp
- parser
- sema
- runtime
- codegen pontual

### Integração

- `.prg -> check`
- `.prg -> C`
- `.prg -> binário -> stdout/stderr`

### Snapshot/golden

- tokens
- AST
- HIR
- C gerado
- stdout de fixtures

### Compatibilidade

Executar o mesmo fixture com `harbour-rust` e `harbour-core` quando aplicável e comparar:

- código de saída,
- stdout,
- stderr,
- eventualmente output intermediário do PP.

### Fuzz

- lexer
- parser
- pp

## Fontes do upstream

- `harbour-core/tests/*.prg`
- `harbour-core/tests/hbpp/*`
- `harbour-core/utils/hbtest/rt_*.prg`
- `harbour-core/tests/rddtest/*`

## Política de corpus

- começar com corpus pequeno e curado,
- importar primeiro exemplos mínimos e comportamentos centrais,
- promover bugs corrigidos para regressão permanente,
- não despejar o corpus completo antes de termos harness confiável.

## Seeds recomendados para o início

- `tests/hello.prg`
- `tests/while.prg`
- `tests/returns.prg`
- `tests/memvar.prg` quando a fase chegar
- `tests/hbpp/_pp_test.prg` por recorte, não inteiro de uma vez

## Baselines já curados

- `tests/fixtures/lexer/hello.prg` -> `hello.tokens`
- `tests/fixtures/lexer/while.prg` -> `while.tokens`
- `tests/fixtures/parser/hello.prg` -> `hello.ast`
- `tests/fixtures/parser/while.prg` -> `while.ast`

## Critérios por PR

- ao menos um teste novo ou atualizado,
- caso feliz e caso de erro quando couber,
- atualização de `COMPATIBILITY.md` se a semântica mudou.
