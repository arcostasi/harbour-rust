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
- `tests/fixtures/parser/if_else.prg` -> `harbour-rust-cli build/run` com IF/ELSE executável
- `tests/fixtures/parser/arrays.prg` -> `arrays.ast`
- `tests/fixtures/parser/compound_assign.prg` -> `compound_assign.ast`
- `tests/fixtures/parser/compound_assign_run.prg` -> `harbour-rust-cli build/run` com `+= -= *= /=`
- `tests/fixtures/parser/len_builtin.prg` -> `harbour-rust-cli build/run` com `Len()` para string e array
- `tests/fixtures/parser/substr_builtin.prg` -> `harbour-rust-cli build/run` com `SubStr()` para string
- `tests/fixtures/parser/left_right_builtin.prg` -> `harbour-rust-cli build/run` com `Left()` e `Right()` para string
- `tests/fixtures/parser/upper_lower_builtin.prg` -> `harbour-rust-cli build/run` com `Upper()` e `Lower()` para string
- `tests/fixtures/parser/trim_builtin.prg` -> `harbour-rust-cli build/run` com `Trim()`, `LTrim()` e `RTrim()` para string
- `tests/fixtures/parser/at_builtin.prg` -> `harbour-rust-cli build/run` com `At()` para string
- `tests/fixtures/parser/replicate_space_builtin.prg` -> `harbour-rust-cli build/run` com `Replicate()` e `Space()` para string
- `tests/fixtures/parser/string_compare.prg` -> `harbour-rust-cli build/run` com `=`, `==` e `!=` de string no baseline EXACT OFF
- `tests/fixtures/parser/string_concat.prg` -> `harbour-rust-cli build/run` com concatenação `String + String` no host C
- `tests/fixtures/parser/abs_builtin.prg` -> `harbour-rust-cli build/run` com `Abs()` numérico
- `tests/fixtures/parser/sqrt_builtin.prg` -> `harbour-rust-cli build/run` com `Sqrt()` numérico
- `tests/fixtures/parser/exp_builtin.prg` -> `harbour-rust-cli build/run` com `Exp()` numérico
- `tests/fixtures/parser/sin_cos_builtin.prg` -> `harbour-rust-cli build/run` com `Sin()` e `Cos()` numéricos
- `tests/fixtures/parser/tan_builtin.prg` -> `harbour-rust-cli build/run` com `Tan()` numérico
- `tests/fixtures/parser/log_builtin.prg` -> `harbour-rust-cli build/run` com `Log()` numérico
- `tests/fixtures/parser/int_builtin.prg` -> `harbour-rust-cli build/run` com `Int()` numérico
- `tests/fixtures/parser/round_builtin.prg` -> `harbour-rust-cli build/run` com `Round()` numérico
- `tests/fixtures/parser/mod_builtin.prg` -> `harbour-rust-cli build/run` com `Mod()` numérico
- `tests/fixtures/parser/str_builtin.prg` -> `harbour-rust-cli build/run` com `Str()` numérico
- `tests/fixtures/parser/val_builtin.prg` -> `harbour-rust-cli build/run` com `Val()` string->número
- `tests/fixtures/parser/valtype_builtin.prg` -> `harbour-rust-cli build/run` com `ValType()`
- `tests/fixtures/parser/type_builtin.prg` -> `harbour-rust-cli build/run` com `Type()` no recorte textual atual
- `tests/fixtures/parser/max_min_builtin.prg` -> `harbour-rust-cli build/run` com `Max()` e `Min()`
- `tests/fixtures/parser/empty_builtin.prg` -> `harbour-rust-cli build/run` com `Empty()`
- `tests/fixtures/parser/indexing.prg` -> `indexing.ast`
- `tests/fixtures/parser/memvars.prg` -> `memvars.ast`
- `tests/fixtures/parser/codeblock.prg` -> `codeblock.ast`
- `tests/fixtures/parser/macro_read.prg` -> `macro_read.ast`
- `tests/fixtures/parser/private_dynamic.prg` -> sema sem erro com fallback de memvar dinâmica entre rotinas
- `tests/fixtures/parser/memvars.prg` -> lowering IR com `Statement::Private` e `Statement::Public`
- `tests/fixtures/parser/codeblock.prg` -> lowering IR com `Expression::Codeblock`
- `tests/fixtures/parser/macro_read.prg` -> lowering IR com `Expression::Macro`
- `tests/fixtures/parser/private_dynamic.prg` -> lowering IR com `ReadPath::Memvar` e `AssignTarget::Memvar`
- `harbour-rust-runtime` -> `Value::Codeblock`, `Eval()` mínimo e `ValType()/Empty()` para codeblocks
- `harbour-rust-runtime` -> `RuntimeContext` com storage dinâmico para `PRIVATE`/`PUBLIC`, leitura e atribuição com precedência privada
- `tests/fixtures/parser/eval_codeblock.prg` -> `harbour-rust-cli build/run` com `Eval()` e codeblock não-capturante
- `tests/fixtures/parser/eval_memvar_codeblock.prg` -> `harbour-rust-cli build/run` com `Eval()` e codeblock lendo memvar
- `tests/fixtures/parser/eval_capture_error.prg` -> `harbour-rust-cli build` com erro explícito de captura lexical ainda não suportada
- `tests/fixtures/parser/phase8_acceptance.prg` -> `harbour-rust-cli build/run` com `PRIVATE`, `PUBLIC`, memvars, `Eval()` e macro read mínima no baseline de aceite da Fase 8
- `tests/fixtures/compat/phase8_dynamic_runtime.prg` -> baseline focado de memvars dinâmicas e `Eval()`/codeblocks contra `harbour-core/tests/memvar.prg` e `harbour-core/doc/codebloc.txt`
- `crates/harbour-rust-cli/tests/commands.rs` -> `help`, `check`, `transpile --to c` e códigos de saída coerentes do CLI na Fase 11
- `tests/fixtures/parser/arrays.prg` -> lowering HIR sem erro
- `tests/fixtures/parser/static.prg` -> lowering HIR com `Statement::Static`
- leituras nominais manuais no crate `harbour-rust-hir` -> lowering HIR como `Read(path)` explícito
- `tests/fixtures/parser/compound_assign.prg` -> lowering HIR estável como `Assignment + Binary`
- `tests/fixtures/parser/compound_assign_mod.prg` -> erro explícito de `codegen-c` para operador ainda não executável
- `tests/fixtures/compat/len_runtime.prg` -> baseline focado de `Len()` contra `harbour-core/utils/hbtest/rt_hvma.prg`
- `tests/fixtures/compat/substr_runtime.prg` -> baseline focado de `SubStr()` contra `harbour-core/utils/hbtest/rt_str.prg`
- `tests/fixtures/compat/left_right_runtime.prg` -> baseline focado de `Left()`/`Right()` contra `harbour-core/utils/hbtest/rt_str.prg`
- `tests/fixtures/compat/upper_lower_runtime.prg` -> baseline focado de `Upper()`/`Lower()` contra `harbour-core/utils/hbtest/rt_str.prg`
- `tests/fixtures/compat/trim_runtime.prg` -> baseline focado de `Trim()`/`LTrim()`/`RTrim()` contra `harbour-core/utils/hbtest/rt_str.prg`
- `tests/fixtures/compat/at_runtime.prg` -> baseline focado de `At()` contra `harbour-core/utils/hbtest/rt_str.prg`
- `tests/fixtures/compat/replicate_space_runtime.prg` -> baseline focado de `Replicate()`/`Space()` contra `harbour-core/utils/hbtest/rt_str.prg`
- `tests/fixtures/compat/string_compare_runtime.prg` -> baseline focado de `=`, `==` e `!=` de string contra `harbour-core/utils/hbtest/rt_hvm.prg`
- `tests/fixtures/compat/abs_runtime.prg` -> baseline focado de `Abs()` contra `harbour-core/utils/hbtest/rt_math.prg`
- `tests/fixtures/compat/sqrt_runtime.prg` -> baseline focado de `Sqrt()` contra `harbour-core/utils/hbtest/rt_math.prg`
- `tests/fixtures/compat/exp_runtime.prg` -> baseline focado de `Exp()` contra `harbour-core/utils/hbtest/rt_math.prg`
- `tests/fixtures/compat/sin_cos_runtime.prg` -> baseline focado local de `Sin()`/`Cos()` ancorado em `harbour-core/src/rtl/math.c` e `harbour-core/doc/c_std.txt` por falta de fixture direta em `utils/hbtest`
- `tests/fixtures/compat/tan_runtime.prg` -> baseline focado de `Tan()` ancorado em `harbour-core/contrib/hbct/trig.c`, `contrib/hbct/tests/test.prg` e `contrib/hbct/doc/en/trig.txt`
- `tests/fixtures/compat/log_runtime.prg` -> baseline focado de `Log()` contra `harbour-core/utils/hbtest/rt_math.prg`
- `tests/fixtures/compat/int_runtime.prg` -> baseline focado de `Int()` contra `harbour-core/utils/hbtest/rt_math.prg`
- `tests/fixtures/compat/round_runtime.prg` -> baseline focado de `Round()` contra `harbour-core/utils/hbtest/rt_math.prg`
- `tests/fixtures/compat/mod_runtime.prg` -> baseline focado de `Mod()` contra `harbour-core/utils/hbtest/rt_math.prg`
- `tests/fixtures/compat/str_runtime.prg` -> baseline focado de `Str()` contra `harbour-core/utils/hbtest/rt_stra.prg`, `rt_hvma.prg` e `rt_math.prg`
- `tests/fixtures/compat/val_runtime.prg` -> baseline focado de `Val()` contra `harbour-core/utils/hbtest/rt_str.prg` e `rt_math.prg`
- `tests/fixtures/compat/valtype_runtime.prg` -> baseline focado de `ValType()` contra `harbour-core/utils/hbtest/rt_hvm.prg`
- `tests/fixtures/compat/type_runtime.prg` -> baseline focado de `Type()` contra `harbour-core/utils/hbtest/rt_hvm.prg`
- `tests/fixtures/compat/max_min_runtime.prg` -> baseline focado de `Max()` e `Min()` contra `harbour-core/utils/hbtest/rt_math.prg`
- `tests/fixtures/compat/empty_runtime.prg` -> baseline focado de `Empty()` contra `harbour-core/utils/hbtest/rt_hvma.prg`
- `tests/fixtures/parser/indexing.prg` -> lowering HIR explícito para `Index(target, indices)`
- `tests/fixtures/parser/static.prg` -> lowering IR com `Statement::Static`
- leituras nominais manuais no crate `harbour-rust-ir` -> lowering IR como `Read(path)` explícito
- `tests/fixtures/sema/control_flow_missing_locals.prg` -> `control_flow_missing_locals.errors`
- `tests/fixtures/sema/control_flow_missing_callables.prg` -> `control_flow_missing_callables.errors`
- `tests/fixtures/parser/static.prg` -> sema sem erro com bindings `STATIC`
- `tests/fixtures/parser/static_counter.prg` -> `harbour-rust-cli build/run` com persistência same-routine de `STATIC`
- `tests/fixtures/parser/phase7_acceptance.prg` -> `harbour-rust-cli build/run` com IF, FOR, STATIC, arrays e builtins essenciais no baseline de aceite da Fase 7
- `tests/fixtures/parser/private_dynamic.prg` -> `harbour-rust-cli build/run` com `PRIVATE` dinâmica entre rotinas
- `tests/fixtures/parser/public_dynamic.prg` -> `harbour-rust-cli run` com `PUBLIC` compartilhado
- `tests/fixtures/parser/macro_memvar.prg` -> `harbour-rust-cli build/run` com `&name` e `&(expr)` como macro read mínima de memvar
- `tests/fixtures/parser/arrays.prg` -> lowering IR explícito para `Array(elements)` sem erro de lowering
- `tests/fixtures/parser/indexing.prg` -> lowering IR explícito para `Index(target, indices)` sem erro de lowering
- `tests/fixtures/parser/arrays.prg` -> codegen C com `harbour_value_from_array_items(...)`
- `tests/fixtures/parser/indexing.prg` -> codegen C com `harbour_value_array_get(...)`
- `tests/fixtures/parser/static.prg` -> codegen C com storage estático persistente por rotina
- `tests/fixtures/parser/indexed_assign.prg` -> lowering HIR/IR e codegen C com `harbour_value_array_set_path(...)`
- `harbour-rust-runtime` -> helpers públicos de indexação 1-based com diagnóstico de bounds e tipo
- `harbour-rust-runtime` -> helpers públicos de escrita 1-based com `array_set()` e `array_set_path()`
- `harbour-rust-runtime` -> `exact_equals()` para identidade observável de arrays e helpers `array_resize()`/`array_push()`/`array_clone()`
- `harbour-rust-runtime` -> `AAdd()`/`ASize()` via `call_builtin_mut()` e erro explícito na surface imutável
- `harbour-rust-runtime` -> `AClone()` via `call_builtin()` e baseline leniente para `NIL`/não-array
- `harbour-rust-runtime` -> `ADel()`/`AIns()` via `call_builtin_mut()` com semântica leniente de posição e comprimento preservado
- `harbour-rust-runtime` -> `AScan()` via `call_builtin()` com busca 1-based e match prefixo para strings no baseline atual
- `harbour-rust-runtime` -> códigos/mensagens de `array access` e `array assign` mais próximos do baseline (`1068/1069/1132/1133`)
- `harbour-rust-runtime` -> `==` por identidade observável e `=`/`<>`/ordenação de arrays com erros `1071` a `1076`
- `tests/fixtures/compat/array_comparison_runtime.prg` -> baseline focado de arrays contra `harbour-core/utils/hbtest/rt_hvm.prg` na surface pública do runtime
- `tests/fixtures/parser/compare_ops.prg` -> codegen C com `==`, `=`, `<>`, `>` e `>=`
- `tests/fixtures/parser/compare_ops_lt.prg` -> codegen C com `<` e `<=`
- `tests/fixtures/parser/array_exact_compare.prg` -> `harbour-rust-cli run` com identidade observável de arrays para `==`
- `tests/fixtures/parser/compare_ops.prg` -> `harbour-rust-cli run` com mensagens observáveis `BASE 1071/1072/1075/1076`
- `tests/fixtures/parser/compare_ops_lt.prg` -> `harbour-rust-cli run` com mensagens observáveis `BASE 1073/1074`
- `tests/fixtures/parser/aclone.prg` -> codegen C com `harbour_builtin_aclone(...)`
- `tests/fixtures/parser/mutable_builtins.prg` -> codegen C com `harbour_builtin_aadd(&value, ...)` e `harbour_builtin_asize(&value, ...)`
- `tests/fixtures/compat/array_builtins_runtime.prg` -> baseline focado de `ADel()`/`AIns()`/`AScan()` contra `harbour-core/utils/hbtest/rt_array.prg`
- `examples/hello.prg` -> `harbour-rust-cli build/run`
- `tests/fixtures/parser/while.prg` -> `harbour-rust-cli build/run`
- `tests/fixtures/parser/for_sum.prg` -> `harbour-rust-cli build/run`
- `tests/fixtures/parser/indexed_assign.prg` -> `harbour-rust-cli run`
- `tests/fixtures/parser/aclone.prg` -> `harbour-rust-cli build/run`
- `tests/fixtures/parser/mutable_builtins.prg` -> `harbour-rust-cli build/run`
- `tests/fixtures/parser/array_builtins.prg` -> `harbour-rust-cli build/run` com `ADel()`, `AIns()` e `AScan()`
- `tests/fixtures/parser/array_args.prg` -> `harbour-rust-cli build/run` com array passado como argumento
- `tests/fixtures/parser/array_matrix.prg` -> `harbour-rust-cli build/run` com leitura/escrita multidimensional
- `tests/fixtures/pp/include_root.prg` -> preprocessamento com `#define` e `#include` simples
- `tests/fixtures/pp/define_root.prg` -> expansão simples de `#define` objeto
- `tests/fixtures/pp/recursive_define_root.prg` -> expansão recursiva de `#define` objeto
- `tests/fixtures/pp/cyclic_define_root.prg` -> erro de ciclo em expansão recursiva
- `tests/fixtures/pp/quoted_search_path_root.prg` -> `#include "..."` com fallback para search path
- `tests/fixtures/pp/angle_search_path_root.prg` -> `#include <...>` resolvido por search path
- `tests/fixtures/pp/angle_search_path_root.prg` -> `harbour-rust-cli build/run` com `-I/--include-dir`
- `tests/fixtures/pp/command_translate_root.prg` -> golden de `#command` + `#translate`
- `tests/fixtures/pp/rule_markers_root.prg` -> golden de opcionais, lista, restrição e stringify
- `tests/fixtures/pp/multiline_command_root.prg` -> golden de diretiva multi-linha com `;`
- `tests/fixtures/pp/malformed_rule_root.prg` -> erro explícito de regra malformada
- `crates/harbour-rust-compat/tests/pp_phase9_rules.rs` -> baseline focado do recorte de `#command`/`#translate` ancorado em `harbour-core/doc/pp.txt` e `harbour-core/tests/hbpp/_pp_test.prg`
- `tests/fixtures/pp/phase9_acceptance.prg` -> `harbour-rust-cli build/run` com `#command` + `#translate` no pipeline completo
- `tests/fixtures/pp/phase9_preprocess_error.prg` -> erro explícito de preprocessamento no CLI para regra malformada
- Fase 9 fechada no subset acima; compatibilidade completa com `ppcore.c` e corpus maior de `tests/hbpp` segue como expansão futura, não como bloqueio do aceite atual
- `crates/harbour-rust-rdd/tests/schema.rs` -> leitura de schema e navegação sobre `users.dbf`, `carts.dbf`, `items.dbf` e `test.dbf` do `harbour-core`
- `crates/harbour-rust-rdd/tests/schema.rs` -> roundtrip temporário de `APPEND BLANK`, `REPLACE`, `DELETE` e `RECALL` sobre cópias de DBFs reais do upstream
- `crates/harbour-rust-cli/tests/commands.rs` -> `check` bem-sucedido para `examples/hello.prg`, erro de PP com exit code `1`, `transpile --to c` com geração de arquivo e erro de `codegen-c` com exit code `2`

## Critérios por PR

- ao menos um teste novo ou atualizado,
- caso feliz e caso de erro quando couber,
- atualização de `COMPATIBILITY.md` se a semântica mudou.
