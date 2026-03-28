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

Na Fase 5, o baseline executável passa a validar o pipeline completo:

- `parser -> hir -> sema -> ir -> codegen-c`,
- compilação do C gerado com compilador host,
- comparação de sucesso/erro no nível do CLI.

Na Fase 6, o baseline de PP passa a validar também:

- `source -> pp -> parser` antes do restante do pipeline,
- `build/run` com `-I/--include-dir`,
- erro explícito de preprocessamento quando o include não é resolvido.

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
- `tests/fixtures/parser/static.prg` -> `static.ast`
- `tests/fixtures/parser/arrays.prg` -> `arrays.ast`
- `tests/fixtures/parser/compound_assign.prg` -> `compound_assign.ast`
- `tests/fixtures/parser/arrays.prg` -> lowering HIR sem erro
- `tests/fixtures/parser/compound_assign.prg` -> lowering HIR estável como `Assignment + Binary`
- `tests/fixtures/sema/control_flow_missing_locals.prg` -> `control_flow_missing_locals.errors`
- `tests/fixtures/sema/control_flow_missing_callables.prg` -> `control_flow_missing_callables.errors`
- `tests/fixtures/parser/static.prg` -> `tests/fixtures/sema/static_unsupported.errors`
- `tests/fixtures/parser/arrays.prg` -> diagnóstico estável de IR para array ainda não suportado
- `examples/hello.prg` -> `harbour-rust-cli build/run`
- `tests/fixtures/parser/while.prg` -> `harbour-rust-cli build/run`
- `tests/fixtures/parser/for_sum.prg` -> `harbour-rust-cli build/run`
- `tests/fixtures/pp/include_root.prg` -> preprocessamento com `#define` e `#include` simples
- `tests/fixtures/pp/define_root.prg` -> expansão simples de `#define` objeto
- `tests/fixtures/pp/recursive_define_root.prg` -> expansão recursiva de `#define` objeto
- `tests/fixtures/pp/cyclic_define_root.prg` -> erro de ciclo em expansão recursiva
- `tests/fixtures/pp/quoted_search_path_root.prg` -> `#include "..."` com fallback para search path
- `tests/fixtures/pp/angle_search_path_root.prg` -> `#include <...>` resolvido por search path
- `tests/fixtures/pp/angle_search_path_root.prg` -> `harbour-rust-cli build/run` com `-I/--include-dir`

## Critérios por PR

- ao menos um teste novo ou atualizado,
- caso feliz e caso de erro quando couber,
- atualização de `COMPATIBILITY.md` se a semântica mudou.
