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
- `tests/fixtures/parser/indexing.prg` -> `indexing.ast`
- `tests/fixtures/parser/arrays.prg` -> lowering HIR sem erro
- `tests/fixtures/parser/compound_assign.prg` -> lowering HIR estável como `Assignment + Binary`
- `tests/fixtures/parser/indexing.prg` -> lowering HIR explícito para `Index(target, indices)`
- `tests/fixtures/sema/control_flow_missing_locals.prg` -> `control_flow_missing_locals.errors`
- `tests/fixtures/sema/control_flow_missing_callables.prg` -> `control_flow_missing_callables.errors`
- `tests/fixtures/parser/static.prg` -> `tests/fixtures/sema/static_unsupported.errors`
- `tests/fixtures/parser/arrays.prg` -> lowering IR explícito para `Array(elements)` sem erro de lowering
- `tests/fixtures/parser/indexing.prg` -> lowering IR explícito para `Index(target, indices)` sem erro de lowering
- `tests/fixtures/parser/arrays.prg` -> codegen C com `harbour_value_from_array_items(...)`
- `tests/fixtures/parser/indexing.prg` -> codegen C com `harbour_value_array_get(...)`
- `tests/fixtures/parser/indexed_assign.prg` -> lowering HIR/IR e codegen C com `harbour_value_array_set_path(...)`
- `harbour-rust-runtime` -> helpers públicos de indexação 1-based com diagnóstico de bounds e tipo
- `harbour-rust-runtime` -> helpers públicos de escrita 1-based com `array_set()` e `array_set_path()`
- `harbour-rust-runtime` -> `exact_equals()` para identidade observável de arrays e helpers `array_resize()`/`array_push()`/`array_clone()`
- `harbour-rust-runtime` -> `AAdd()`/`ASize()` via `call_builtin_mut()` e erro explícito na surface imutável
- `harbour-rust-runtime` -> `AClone()` via `call_builtin()` e baseline leniente para `NIL`/não-array
- `harbour-rust-runtime` -> códigos/mensagens de `array access` e `array assign` mais próximos do baseline (`1068/1069/1132/1133`)
- `harbour-rust-runtime` -> `==` por identidade observável e `=`/`<>`/ordenação de arrays com erros `1071` a `1076`
- `tests/fixtures/compat/array_comparison_runtime.prg` -> baseline focado de arrays contra `harbour-core/utils/hbtest/rt_hvm.prg` na surface pública do runtime
- `tests/fixtures/parser/compare_ops.prg` -> codegen C com `==`, `=`, `<>`, `>` e `>=`
- `tests/fixtures/parser/array_exact_compare.prg` -> `harbour-rust-cli run` com identidade observável de arrays para `==`
- `tests/fixtures/parser/aclone.prg` -> codegen C com `harbour_builtin_aclone(...)`
- `tests/fixtures/parser/mutable_builtins.prg` -> codegen C com `harbour_builtin_aadd(&value, ...)` e `harbour_builtin_asize(&value, ...)`
- `examples/hello.prg` -> `harbour-rust-cli build/run`
- `tests/fixtures/parser/while.prg` -> `harbour-rust-cli build/run`
- `tests/fixtures/parser/for_sum.prg` -> `harbour-rust-cli build/run`
- `tests/fixtures/parser/indexed_assign.prg` -> `harbour-rust-cli run`
- `tests/fixtures/parser/aclone.prg` -> `harbour-rust-cli build/run`
- `tests/fixtures/parser/mutable_builtins.prg` -> `harbour-rust-cli build/run`
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
