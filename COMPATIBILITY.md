# COMPATIBILITY

Status:

- `planned`: ainda não iniciado
- `partial`: implementado parcialmente
- `done`: coberto e testado
- `deferred`: adiado para release futura

## Matriz viva

| Recurso | Dialeto | Status | Fase | Oráculo upstream | Teste esperado | Notas |
| --- | --- | --- | --- | --- | --- | --- |
| `PROCEDURE Main()` | Clipper | partial | 2 | `tests/hello.prg` | parser + snapshot | sintaxe procedural mínima e AST cobertas na Fase 2; integração e execução ainda pendentes |
| `FUNCTION` | Clipper | partial | 2/3 | `src/compiler/harbour.y` | unit + snapshot | assinatura simples parseia; resolução básica de chamadas por nome entra na Fase 3 |
| `LOCAL` | Clipper | partial | 2/3 | `harbour.y` | parser + sema | sintaxe e AST cobertas na Fase 2; resolução básica de parâmetros e `LOCAL` entra na Fase 3 |
| `RETURN` | Clipper | partial | 2 | `tests/returns.prg` | parser + snapshot | `RETURN` vazio e com expressão parseiam; integração e runtime ainda pendentes |
| `?` / `QOut()` | Clipper | partial | 2/4 | `doc/pcode.txt`, `tests/hello.prg` | parser + runtime | sintaxe de `?` coberta na Fase 2; builtin e execução seguem para runtime |
| `IF / ELSE / ENDIF` | Clipper | partial | 2 | `harbour.y` | parser + run | sintaxe e AST cobertas na Fase 2; execução ainda pendente |
| `DO WHILE / ENDDO` | Clipper | partial | 2 | `tests/while.prg` | integração | sintaxe e AST cobertas na Fase 2; execução ainda pendente |
| `FOR / NEXT` | Clipper | partial | 2 | `harbour.y` | integração | sintaxe e AST cobertas na Fase 2; sem `FOR EACH` e sem execução ainda |
| operadores básicos | Clipper | partial | 1/2/4 | `ppcore.c`, `harbour.y` | unit + runtime | tokenização base e multi-caractere cobertas na Fase 1; semântica ainda pendente |
| strings | Clipper | partial | 1/4 | `doc/pp.txt` | unit + golden | literais e erro de string não terminada cobertos na Fase 1; `[]` segue para PP |
| números | Clipper | partial | 1/4 | `doc/pp.txt` | unit + runtime | inteiros e floats decimais tokenizados na Fase 1; semântica e corner cases pendentes |
| comentários | Clipper | partial | 1 | `tests/hello.prg`, `tests/while.prg` | unit + golden | `//`, `&&`, `/* */` e `*` em início de linha cobertos pelo lexer |
| `STATIC` | Clipper | planned | 7 | `doc/statics.txt`, `tests/statics*.prg` | compat | modelar storage separado |
| arrays | Clipper | planned | 7 | `src/vm/arrays.c` | runtime + compat | depois do procedural mínimo |
| builtins de string | Clipper | planned | 7 | `src/rtl`, `utils/hbtest/rt_str.prg` | compat | por prioridade, não em lote |
| builtins matemáticos | Clipper | planned | 7 | `utils/hbtest/rt_math.prg` | compat | |
| `#define` | Clipper | planned | 6 | `doc/pp.txt`, `tests/pp.prg` | golden | token-based, não texto puro |
| `#include` | Clipper | planned | 6 | `ppcore.c` | golden | incluir mapeamento de spans |
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

## Regras

- Nenhum recurso vai para `done` sem teste automatizado.
- `partial` exige nota de limitação e fixture explícita.
- Toda divergência observada contra `harbour-core` deve ser registrada aqui.
