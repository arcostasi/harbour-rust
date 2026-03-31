# COMPATIBILITY

Status:

- `planned`: ainda não iniciado
- `partial`: implementado parcialmente
- `done`: coberto e testado
- `deferred`: adiado para release futura

## Matriz viva

| Recurso | Dialeto | Status | Fase | Oráculo upstream | Teste esperado | Notas |
| --- | --- | --- | --- | --- | --- | --- |
| `PROCEDURE Main()` | Clipper | partial | 2/5 | `tests/hello.prg` | parser + snapshot + cli run | sintaxe procedural mínima e AST cobertas na Fase 2; `hello.prg` já compila e executa via CLI na Fase 5; cobertura ainda parcial fora do caminho feliz |
| `FUNCTION` | Clipper | partial | 2/3 | `src/compiler/harbour.y` | unit + snapshot + sema | assinatura simples parseia; resolução básica de chamadas por nome coberta na Fase 3 |
| `LOCAL` | Clipper | partial | 2/3 | `harbour.y` | parser + sema | sintaxe e AST cobertas na Fase 2; resolução básica de parâmetros e `LOCAL` coberta na Fase 3 |
| `RETURN` | Clipper | partial | 2/4/5 | `tests/returns.prg` | parser + snapshot + runtime + cli run | `RETURN` vazio e com expressão parseiam; valores de retorno e surface de runtime existem; `RETURN` vazio participa do caminho executável de `hello.prg` na Fase 5 |
| `?` / `QOut()` | Clipper | partial | 2/4/5 | `doc/pcode.txt`, `tests/hello.prg` | parser + runtime + cli run | sintaxe de `?` coberta na Fase 2; formatter, `QOut()` mínimo e dispatch case-insensitive por nome entram na Fase 4; `hello.prg` já executa `QOut()` via compilador C host na Fase 5 |
| `IF / ELSE / ENDIF` | Clipper | partial | 2/7 | `harbour.y` | parser + codegen-c + cli run | sintaxe e AST cobertas na Fase 2; `tests/fixtures/parser/if_else.prg` agora cobre o primeiro caminho executável no backend C e no `cli run`; `ELSEIF`, combinações mais amplas de expressão e regressões em programas maiores continuam pendentes |
| `DO WHILE / ENDDO` | Clipper | partial | 2/5 | `tests/while.prg` | integração + cli run | sintaxe e AST cobertas na Fase 2; `while.prg` já compila e executa no caminho inicial da Fase 5; outras formas de condição e corpo ainda pendentes |
| `FOR / NEXT` | Clipper | partial | 2/5 | `harbour.y`, `tests/fornext.prg` | integração + cli run | sintaxe e AST cobertas na Fase 2; `for_sum.prg` já compila e executa no caminho inicial da Fase 5; sem `FOR EACH`, sem passo negativo e sem variantes mais amplas |
| operadores básicos | Clipper | partial | 1/2/3/4/5/7 | `ppcore.c`, `harbour.y` | unit + parser + sema + runtime + cli run | tokenização base e multi-caractere cobertas na Fase 1; parsing e resolução básica cobertas até a Fase 3; aritmética e comparação básicas cobertas no runtime da Fase 4; `<`, `<=`, `+` e `x++` entram no caminho executável inicial da Fase 5; `+= -= *= /=` agora atravessam `parser -> hir -> ir -> codegen-c -> cli run` para alvos nominais simples; `%= ^=` continuam apenas no parsing/lowering com erro explícito de codegen nesta fase |
| strings | Clipper | partial | 1/4 | `doc/pp.txt` | unit + runtime | literais e erro de string não terminada cobertos na Fase 1; concatenação, comparação léxica básica e saída orientada a print entram na Fase 4; `[]` segue para PP |
| números | Clipper | partial | 1/4 | `doc/pp.txt` | unit + runtime | inteiros e floats decimais tokenizados na Fase 1; aritmética, promoção numérica e formatação básica entram na Fase 4; corner cases ainda pendentes |
| comentários | Clipper | partial | 1 | `tests/hello.prg`, `tests/while.prg` | unit + golden | `//`, `&&`, `/* */` e `*` em início de linha cobertos pelo lexer |
| `STATIC` | Clipper | partial | 7 | `doc/statics.txt`, `tests/statics*.prg` | parser + hir + sema + ir + codegen-c + cli run | `STATIC` já parseia com lista inicial de inicializadores, baixa para HIR e IR como `Statement::Static`, e leituras nominais já entram como `Read(path)` explícito nas duas camadas; a sema agora aceita declarações `STATIC` e resolve leituras/escritas nominais no mesmo routine sem diagnóstico placeholder; o `codegen-c` já gera storage C persistente por rotina com guard de inicialização para esse caminho same-routine, e `tests/fixtures/parser/static_counter.prg` agora valida esse baseline via `harbour-rust-cli run`; storage compartilhado entre rotinas do mesmo módulo, semântica completa de inicialização e suporte de runtime mais amplo continuam pendentes |
| arrays | Clipper | partial | 7 | `src/vm/arrays.c`, `utils/hbtest/rt_hvm.prg`, `utils/hbtest/rt_hvma.prg`, `utils/hbtest/rt_array.prg` | parser + hir + runtime + compat + cli run | `Value::Array` e construtores básicos já existem no runtime; literais `{}` e `{ expr, ... }` já parseiam e agora baixam para HIR e IR explícitas; `expr[expr]`, `expr[expr, ...]` e encadeamento como `expr[expr][expr]` também já baixam para HIR e IR explícitas na Fase 7; o runtime Rust já expõe leitura e escrita 1-based com diagnóstico de bounds, tipo e caminho vazio; o backend C agora emite `harbour_value_from_array_items(...)`, `harbour_value_array_get(...)`, `harbour_value_array_set_path(...)`, `harbour_value_exact_equals(...)`, `harbour_value_equals(...)`, `harbour_value_not_equals(...)`, `harbour_value_less_than(...)`, `harbour_value_less_than_or_equal(...)`, `harbour_value_greater_than(...)` e `harbour_value_greater_than_or_equal(...)`, e o suporte host já materializa `Array`, leitura e escrita indexadas 1-based, identidade observável de arrays para `==` e mensagens observáveis `BASE 1071` a `1076` no caminho `cli run`; `exact_equals()` já modela o baseline observável de `==` para arrays por identidade do valor observado, `=`/`<>` e ordenação com arrays agora já usam mensagens/códigos mais próximos do baseline xBase (`1071` a `1076`), e o fixture focado `tests/fixtures/compat/array_comparison_runtime.prg` agora fixa esse baseline diretamente contra `utils/hbtest/rt_hvm.prg` na surface pública do runtime; `tests/fixtures/parser/array_exact_compare.prg` agora também valida `==` de arrays no caminho executável `cli run`, `tests/fixtures/parser/compare_ops.prg` valida os diagnósticos executáveis de `=`/`<>`/`>`/`>=`, e `tests/fixtures/parser/compare_ops_lt.prg` valida `<`/`<=`; `aadd()` e `asize()` já existem sobre `array_push()`/`array_resize()` via `call_builtin_mut()`, e `aclone()` agora expõe `AClone()` na surface imutável sobre `array_clone()`; `AClone()` já atravessa `codegen-c` e `cli run` via `harbour_builtin_aclone(...)`, e agora `AAdd()`/`ASize()` também atravessam `codegen-c` e `cli run` quando o primeiro argumento é símbolo endereçável simples, via `harbour_builtin_aadd(&value, ...)` e `harbour_builtin_asize(&value, ...)`; primeiros argumentos não endereçáveis continuam com erro explícito de codegen nesta fase; leitura e escrita indexadas agora já usam mensagens/códigos mais próximos do baseline xBase (`1068/1069/1132/1133`); semântica mais ampla para objetos, codeblocks e outras combinações continua pendente |
| `Len()` | Clipper | partial | 7 | `src/rtl/len.c`, `utils/hbtest/rt_hvma.prg` | runtime + compat + codegen-c + cli run | `Len()` agora retorna tamanho para `String` e `Array` no runtime Rust, no runtime host C e no dispatch do `codegen-c`; `tests/fixtures/parser/len_builtin.prg` cobre o caminho executável e `tests/fixtures/compat/len_runtime.out` ancora o baseline observado para `String`, `Array`, `NIL` e `Integer`; hashes, objetos e contagem por codepage multibyte continuam pendentes |
| builtins de string | Clipper | planned | 7 | `src/rtl`, `utils/hbtest/rt_str.prg` | compat | por prioridade, não em lote |
| builtins matemáticos | Clipper | planned | 7 | `utils/hbtest/rt_math.prg` | compat | |
| `#define` | Clipper | partial | 6 | `doc/pp.txt`, `tests/pp.prg` | unit + integração | parsing inicial de diretiva, registro de defines e expansão recursiva case-insensitive de macros objeto em linhas normais, com diagnóstico de ciclo; macros parametrizadas e expansão token-based ainda pendentes |
| `#include` | Clipper | partial | 6 | `ppcore.c` | integração + cli build/run | resolução inicial relativa ao arquivo atual, fallback por search paths configuráveis, suporte inicial a `<...>` e handoff `pp -> parser` no CLI; spans finos e política completa de busca ainda pendentes |
| `#command` | Clipper | planned | 9 | `tests/hbpp/_pp_test.prg` | compat | implementação incremental |
| `#translate` | Clipper | planned | 9 | `tests/hbpp/_pp_test.prg` | compat | |
| `PRIVATE` | xBase | planned | 8 | `src/vm/memvars.c`, `tests/memvar.prg` | compat | escopo dinâmico |
| `PUBLIC` | xBase | planned | 8 | `src/vm/memvars.c`, `tests/memvar.prg` | compat | |
| memvars | xBase | planned | 8 | `src/vm/memvars.c` | compat | |
| codeblocks | Clipper/Harbour | planned | 8 | `src/vm/codebloc.c`, `doc/codebloc.txt` | compat | |
| macro operator | Harbour | planned | 8/9 | `src/vm/macro.c`, `tests/macro.prg` | compat | começar parcial |
| `FOR EACH` | Harbour | deferred | pós-0.2 | `doc/clipper.txt` | compat | extensão Harbour, não baseline |
| `WITH OBJECT` | Harbour | deferred | pós-0.2 | `doc/clipper.txt` | compat | |
| RDD/DBF | Clipper/Harbour | planned | 10 | `src/rdd`, `tests/rddtest` | integração | só após frontend estável |

## Marco atual

O aceite da Fase 5 está fechado com o pipeline procedural fim a fim:

- `examples/hello.prg` compila e executa via CLI,
- `tests/fixtures/parser/while.prg` cobre o primeiro caminho executável com `DO WHILE`,
- `tests/fixtures/parser/for_sum.prg` cobre o primeiro caminho executável com `FOR` simples.

O aceite da Fase 6 está fechado com o baseline inicial de pré-processamento:

- `#define` objeto já expande em linhas normais, inclusive de forma recursiva,
- ciclos de macro objeto geram diagnóstico explícito,
- `#include "..."` e `#include <...>` já entram no caminho `pp -> parser` do CLI com `-I/--include-dir`.

## Regras

- Nenhum recurso vai para `done` sem teste automatizado.
- `partial` exige nota de limitação e fixture explícita.
- Toda divergência observada contra `harbour-core` deve ser registrada aqui.
