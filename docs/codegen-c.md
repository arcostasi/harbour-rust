# Backend C (codegen-c)

## Responsabilidade

Gerar código C legível e depurável a partir da IR, pronto para compilação por um compilador C host (`clang`, `gcc`, `cc`).

**Crate:** `harbour-rust-codegen-c`

## Referências upstream

- `harbour-core/src/compiler/genc.c` — backend C de referência do Harbour
- `harbour-core/include/hbpcode.h` — pcode que o backend original consome

## Pipeline

```text
IR ──codegen-c──> arquivo .c ──cc──> binário
```

## Filosofia

- **Legibilidade primeiro:** o C gerado deve ser fácil de ler e depurar com `gdb`/`lldb`.
- **Sem magia:** chamadas diretas a helpers com nomes claros, sem macros pesadas.
- **Subconjunto crescente:** começa com o procedural mínimo, expande por fase.

## Estrutura do C gerado

### Prelude

Todo arquivo gerado inclui:

```c
#include "harbour_runtime.h"
```

O header declara os tipos e helpers usados: `HarbourValue`, funções de aritmética, comparação, arrays e builtins.

### Rotinas

```c
HarbourValue procedure_main(void) {
    // corpo
    return harbour_value_nil();
}
```

- Procedures retornam `HarbourValue` (sempre `NIL`) para uniformidade
- Functions retornam o valor da expressão em `RETURN`
- Entry point: quando existe `PROCEDURE Main()`, gera wrapper `main()`:

```c
int main(void) {
    procedure_main();
    return 0;
}
```

### Variáveis locais

```c
HarbourValue x = harbour_value_nil();
HarbourValue n = harbour_value_from_int(1);
```

### Controle de fluxo

```c
// IF
if (harbour_value_is_true(condition)) { ... } else { ... }

// DO WHILE
while (harbour_value_is_truthy(condition)) { ... }

// FOR
x = harbour_value_from_int(start);
while (harbour_value_le(x, stop)) { ... x = harbour_value_add(x, step); }
```

### Builtins

```c
harbour_qout(1, &arg1);  // ? expr
```

### Arrays

```c
// Literal
HarbourValue arr = harbour_value_from_array_items(3, a, b, c);

// Leitura
HarbourValue val = harbour_value_array_get(arr, index);

// Escrita por caminho
harbour_value_array_set_path(&matrix, 2, (int[]){2, 1}, harbour_value_from_int(99));
```

## Runtime support (host C)

O crate `harbour-rust-cli` inclui um conjunto mínimo de headers e fontes C que o compilador host compila junto:

- `runtime_support.h` — declarações
- `runtime_support.c` — implementações de `HarbourValue`, aritmética, comparação, arrays, QOut

Esse suporte é mínimo e cresce com o subconjunto coberto.

## Decisões de design

### Não espelhar o pcode

O backend original (`genc.c`) gera C baseado em pcode. `harbour-rust` gera C diretamente da IR estruturada, sem passar por pcode.

### Helpers explícitos

Toda operação semântica vira chamada de helper C com nome descritivo:

- `harbour_value_add(a, b)` em vez de operador `+` direto
- `harbour_value_lt(a, b)` em vez de `<` direto
- Isso permite runtime checks e diagnósticos de tipo

### Diagnóstico de codegen

Construções da IR que o backend C ainda não suporta geram erro de codegen explícito, em vez de gerar C incorreto.

## Baselines curados

| Fixture | Comportamento |
| --- | --- |
| `examples/hello.prg` | compila e executa |
| `tests/fixtures/parser/while.prg` | DO WHILE executa |
| `tests/fixtures/parser/for_sum.prg` | FOR simples executa |
| `tests/fixtures/parser/if_else.prg` | IF/ELSE executa |
| `tests/fixtures/parser/compound_assign_run.prg` | `+= -= *= /=` executam |
| `tests/fixtures/parser/len_builtin.prg` | `Len()` em string e array executa |
| `tests/fixtures/parser/substr_builtin.prg` | `SubStr()` em string executa |
| `tests/fixtures/parser/left_right_builtin.prg` | `Left()` e `Right()` em string executam |
| `tests/fixtures/parser/upper_lower_builtin.prg` | `Upper()` e `Lower()` em string executam |
| `tests/fixtures/parser/trim_builtin.prg` | `Trim()`, `LTrim()` e `RTrim()` executam |
| `tests/fixtures/parser/at_builtin.prg` | `At()` em string executa |
| `tests/fixtures/parser/replicate_space_builtin.prg` | `Replicate()` e `Space()` executam |
| `tests/fixtures/parser/abs_builtin.prg` | `Abs()` numérico executa |
| `tests/fixtures/parser/int_builtin.prg` | `Int()` numérico executa |
| `tests/fixtures/parser/round_builtin.prg` | `Round()` numérico executa |
| `tests/fixtures/parser/mod_builtin.prg` | `Mod()` numérico executa |
| `tests/fixtures/parser/max_min_builtin.prg` | `Max()` e `Min()` executam |
| `tests/fixtures/parser/str_builtin.prg` | `Str()` numérico executa |
| `tests/fixtures/parser/val_builtin.prg` | `Val()` string->número executa |
| `tests/fixtures/parser/valtype_builtin.prg` | `ValType()` executa |
| `tests/fixtures/parser/type_builtin.prg` | `Type()` executa no recorte textual atual |
| `tests/fixtures/parser/empty_builtin.prg` | `Empty()` executa |
| `tests/fixtures/parser/arrays.prg` | gera C com array_items |
| `tests/fixtures/parser/indexing.prg` | gera C com array_get |
| `tests/fixtures/parser/indexed_assign.prg` | gera C com array_set_path + executa |
| `tests/fixtures/parser/static.prg` | gera C com storage estático persistente por rotina |

## Estado atual

Fase 5 + Fase 7 parcial:

- Rotinas, RETURN, QOut — completo
- IF/ELSE simples — completo
- DO WHILE, FOR — completo
- `+= -= *= /=` em alvos nominais simples — completo
- `Len()` para string e array via dispatch de builtin — completo
- `SubStr()` para string via dispatch de builtin — parcial
- `Left()` e `Right()` para string via dispatch de builtin — parcial
- `Upper()` e `Lower()` para string via dispatch de builtin — parcial
- `Trim()`, `LTrim()` e `RTrim()` para string via dispatch de builtin — parcial
- `At()` para string via dispatch de builtin — parcial
- `Replicate()` e `Space()` para string via dispatch de builtin — parcial
- `Abs()` numérico via dispatch de builtin — parcial
- `Int()` numérico via dispatch de builtin — parcial
- `Round()` numérico via dispatch de builtin — parcial
- `Mod()` numérico via dispatch de builtin — parcial
- `Max()` e `Min()` numéricos/lógicos via dispatch de builtin — parcial
- `Str()` numérico via dispatch de builtin — parcial
- `Val()` string->número via dispatch de builtin — parcial
- `ValType()` para `NIL`, `Logical`, `Integer/Float`, `String` e `Array` via dispatch de builtin — parcial
- `Type()` no recorte textual atual (`NIL`, `.T./.F.`, número simples, string quoted, literal `{...}`, nome não resolvido) via dispatch de builtin — parcial
- `Empty()` para `NIL`, `Logical`, `Integer/Float`, `String` e `Array` via dispatch de builtin — parcial
- LOCAL com inicializador — completo
- Literais de array — completo
- Indexação (leitura) — completo
- Atribuição indexada — completo
- `%= ^=` — pendente no caminho executável
- `Len()` para hashes/objetos/codepages multibyte — pendente
- `SubStr()` para codepage multibyte, `Chr(0)` e argumentos numéricos não-inteiros — pendente
- `Left()`/`Right()` para codepage multibyte, `Chr(0)` e argumentos numéricos não-inteiros — pendente
- `Upper()`/`Lower()` para `Chr(0)`, codepage multibyte e semântica by-ref do upstream — pendente
- `Trim()`/`LTrim()`/`RTrim()` para `Chr(0)`, by-ref, whitespace não-ASCII e extensão Harbour de segundo parâmetro — pendente
- `At()` para codepage multibyte e `hb_AT()` com `start/to` — pendente
- `Replicate()`/`Space()` para overflow completo do upstream, `Chr(0)` e codepage multibyte — pendente
- `Abs()` para by-ref, handlers matemáticos do upstream e corner cases extremos — pendente
- `Int()` para by-ref, overflow extremo e corner cases mais profundos do upstream — pendente
- `Round()` para escala histórica exata, zeros à direita do upstream, by-ref e corner cases mais profundos — pendente
- `Mod()` para preservação histórica exata da representação numérica, substituição de erro no estilo original e corner cases mais profundos — pendente
- `Max()`/`Min()` para datas, datetime, by-ref e demais tipos suportados pelo upstream — pendente
- `Str()` para precisão herdada da escala original, larguras negativas e corner cases mais profundos do upstream — pendente
- `Val()` para exponentes, `Chr(0)`, pontos repetidos e corner cases mais profundos do upstream — pendente
- `ValType()` para `Date`, `Object`, `Codeblock`, `Memo`, `Hash` e tipos ainda não materializados no runtime — pendente
- `Type()` com macro evaluation completa, resolução real de nomes, datas, objetos, codeblocks, memos e demais tipos do upstream — pendente
- `Empty()` para datas, codeblocks, pointers, hashes, objetos e `Chr(0)` embutido no runtime host C — pendente
- STATIC com storage persistente no C gerado para leitura no mesmo routine — parcial
- STATIC no pipeline completo (`sema -> cli run`) — parcial
