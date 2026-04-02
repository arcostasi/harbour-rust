# Fase 7 — Plano detalhado de execução

> Compatibilidade procedural ampliada: arrays completos, STATIC com runtime, operadores compostos, builtins essenciais.

## Pré-requisitos

- Fases 0–6 concluídas e verdes
- Pipeline fim a fim funcional: `source -> pp -> lexer -> parser -> AST -> HIR -> sema -> IR -> codegen-c -> binário`
- Baselines de `hello.prg`, `while.prg`, `for_sum.prg` executáveis

## Objetivo da fase

Após esta fase, programas procedurais reais de pequeno porte devem compilar e executar, incluindo:

- variáveis `STATIC` com storage persistente
- arrays criados, indexados, modificados e passados como argumento
- operadores compostos (`+=`, `-=`, etc.)
- builtins essenciais de string, math e conversão

## Slices de execução

### Slice 7.1 — STATIC com runtime completo

**Status:** parcial (parse + HIR + IR + sema + codegen same-routine ✓, storage compartilhado entre rotinas + runtime/CLI mais amplo ✗)

**Escopo:**
- Runtime: storage persistente para STATIC (inicializado uma vez)
- IR: lowering de STATIC com distinção de LOCAL
- Codegen-C: declaração `static HarbourValue` no C gerado
- Sema: remover diagnóstico de "unsupported", ativar resolução normal

**Referências upstream:**
- `harbour-core/doc/statics.txt`
- `harbour-core/tests/statics*.prg`
- `harbour-core/src/vm/hvm.c` (inicialização de statics)

**Fixture:**
```clipper
STATIC s_count := 0

PROCEDURE Main()
   Increment()
   Increment()
   Increment()
   ? s_count        // esperado: 3
RETURN

PROCEDURE Increment()
   s_count := s_count + 1
RETURN
```

**Aceite:**
- [ ] `STATIC` com inicializador persiste entre chamadas
- [ ] `STATIC` sem inicializador começa como NIL
- [ ] `STATIC` no nível de módulo acessível por rotinas do mesmo arquivo
- [ ] Testes unitários + integração + CLI run

---

### Slice 7.2 — Arrays: construtores e travessia completa

**Status:** parcial (parse + HIR + IR + codegen para literais e indexação ✓, `Len()` para arrays ✓)

**Escopo:**
- Runtime: `Array` como tipo passável por referência
- Comparação de arrays: `=` e `<>` (por referência, não deep)
- Multidimensional estável
- `Len()` para arrays
- Teste fim a fim com arrays como argumento de função

**Referências upstream:**
- `harbour-core/src/vm/arrays.c`
- `harbour-core/utils/hbtest/rt_array.prg`

**Fixture:**
```clipper
PROCEDURE Main()
   LOCAL a := {10, 20, 30}
   ? Len(a)           // esperado: 3
   ? a[2]             // esperado: 20
   a[2] := 99
   ? a[2]             // esperado: 99
   ShowArray(a)
RETURN

PROCEDURE ShowArray(arr)
   LOCAL i
   FOR i := 1 TO Len(arr)
      ? arr[i]
   NEXT
RETURN
```

**Aceite:**
- [ ] Literais, indexação (leitura/escrita) e passagem como argumento
- [x] `Len()` funciona para arrays
- [ ] Multidimensional executável
- [ ] Testes unitários + integração + CLI run

---

### Slice 7.3 — Builtins de array completos

**Status:** parcial (AAdd, ASize, AClone ✓ no runtime)

**Escopo:**
- `ADel()`, `AIns()` — remoção e inserção de elementos
- `ASort()` — ordenação (comparação padrão)
- `AScan()` — busca em array
- `AEval()` — iteração com codeblock (pode ser simplificada sem codeblock real)
- `AFill()` — preenchimento
- `ACopy()` — cópia parcial

**Referências upstream:**
- `harbour-core/src/vm/arrays.c`
- `harbour-core/utils/hbtest/rt_array.prg`

**Aceite:**
- [ ] Cada builtin com teste unitário de caso feliz e erro
- [ ] `COMPATIBILITY.md` atualizada por builtin
- [ ] Pelo menos `ADel`, `AIns`, `AScan` funcionais

---

### Slice 7.4 — Operadores compostos no caminho executável

**Status:** parse + HIR + codegen-c + cli run ✓ para `+= -= *= /=`; `%= ^=` ainda pendentes

**Escopo:**
- Codegen-C: garantir que `x += 1` gere `x = harbour_value_add(x, 1)`
- Teste fim a fim com `+=`, `-=`, `*=`, `/=`

**Referências upstream:**
- `harbour-core/src/compiler/harbour.y` (seção de assignment operators)

**Fixture:**
```clipper
PROCEDURE Main()
   LOCAL n := 10
   n += 5
   ? n        // esperado: 15
   n -= 3
   ? n        // esperado: 12
   n *= 2
   ? n        // esperado: 24
   n /= 4
   ? n        // esperado: 6
RETURN
```

**Aceite:**
- [x] `+=`, `-=`, `*=`, `/=` executam corretamente
- [x] Teste de integração CLI run
- [x] Sem regressão nos testes existentes

---

### Slice 7.5 — Builtins essenciais de string

**Status:** parcial (`Len()` ✓, `SubStr()` ✓, `Left()` ✓, `Right()` ✓, `Upper()` ✓, `Lower()` ✓, `Trim()` ✓, `LTrim()` ✓, `RTrim()` ✓, `At()` ✓, `Replicate()` ✓, `Space()` ✓)

**Escopo (por prioridade):**

| Builtin | Referência | Prioridade |
| --- | --- | --- |
| `Len()` (string) | `src/rtl/len.c` | alta |
| `SubStr()` | `src/rtl/substr.c` | alta |
| `Left()`, `Right()` | `src/rtl/left.c`, `src/rtl/right.c` | alta |
| `Upper()`, `Lower()` | `src/rtl/str.c` | alta |
| `Trim()`, `LTrim()`, `RTrim()` | `src/rtl/trim.c` | alta |
| `At()` | `src/rtl/at.c` | média |
| `Replicate()` | `src/rtl/str.c` | média |
| `Space()` | `src/rtl/str.c` | média |
| `StrTran()` | `src/rtl/strtran.c` | baixa |
| `Pad*()` | `src/rtl/pad.c` | baixa |

**Aceite:**
- [ ] Pelo menos `Len`, `SubStr`, `Left`, `Right`, `Upper`, `Lower`, `Trim` implementados
- [ ] Cada um com teste unitário e de compatibilidade
- [ ] `COMPATIBILITY.md` atualizada

---

### Slice 7.6 — Builtins essenciais de math e conversão

**Status:** parcial (`Abs()` ✓, `Int()` ✓, `Str()` ✓, `Val()` ✓, `ValType()` ✓)

**Escopo (por prioridade):**

| Builtin | Referência | Prioridade |
| --- | --- | --- |
| `Abs()` | `src/rtl/abs.c` | alta |
| `Int()` | `src/rtl/math.c` | alta |
| `Round()` | `src/rtl/round.c` | alta |
| `Mod()` | `src/rtl/mod.c` | alta |
| `Val()` | `src/rtl/val.c` | alta |
| `Str()` | `src/rtl/str.c` | alta |
| `ValType()` | `src/rtl/valtype.c` | alta |
| `Empty()` | `src/rtl/empty.c` | média |
| `Type()` | `src/rtl/type.c` | média |
| `Sqrt()`, `Log()`, `Exp()` | `src/rtl/math.c` | baixa |
| `Max()`, `Min()` | `src/rtl/math.c` | baixa |

**Aceite:**
- [ ] Pelo menos `Abs`, `Int`, `Round`, `Val`, `Str`, `ValType` implementados
- [ ] Cada um com teste unitário e de compatibilidade
- [ ] `COMPATIBILITY.md` atualizada

---

### Slice 7.7 — IF no caminho executável

**Status:** parse + HIR + IR + codegen-c + cli run ✓

**Escopo:**
- Codegen-C: gerar `if/else` em C
- Suporte a condições com expressões binárias
- Teste fim a fim com IF/ELSE/ENDIF

**Fixture:**
```clipper
PROCEDURE Main()
   LOCAL x := 10
   IF x > 5
      ? "maior"
   ELSE
      ? "menor ou igual"
   ENDIF
RETURN
```

**Aceite:**
- [x] IF/ELSE/ENDIF executa corretamente via CLI run
- [ ] Testes existentes continuam verdes

---

### Slice 7.8 — Comparações mais fiéis

**Escopo:**
- Comparação de strings case-insensitive (SET EXACT OFF como padrão Clipper)
- `==` exato para strings
- Comparação de tipos mistos (comportamento Clipper documentado)

**Referências upstream:**
- `harbour-core/src/vm/hvm.c` (funções de comparação)
- `harbour-core/utils/hbtest/rt_str.prg`

**Aceite:**
- [ ] String comparison rules documentadas e testadas
- [ ] `==` e `=` com semântica distinta
- [ ] `COMPATIBILITY.md` atualizada

---

## Ordem recomendada

```
7.7 IF executável (desbloqueia muitos programas)
 │
 v
7.4 Operadores compostos executáveis
 │
 v
7.1 STATIC com runtime
 │
 v
7.2 Arrays completos
 │
 v
7.3 Builtins de array
 │
 v
7.5 Builtins de string ──> 7.6 Builtins de math/conversão
                                │
                                v
                           7.8 Comparações fiéis
```

## Critério de fechamento da Fase 7

- [ ] Programa com IF, FOR, STATIC, arrays e builtins essenciais compila e executa
- [ ] Nenhuma regressão nas fases anteriores
- [ ] `COMPATIBILITY.md` reflete todos os recursos implementados
- [ ] Docs temáticas atualizadas
- [ ] Pronto para congelar `0.2.0-alpha` após Fases 8 e 9

## Fixture de aceite da fase

```clipper
// phase7_acceptance.prg — programa de aceite da Fase 7
STATIC s_total := 0

PROCEDURE Main()
   LOCAL names := {"Alice", "Bob", "Charlie"}
   LOCAL i

   FOR i := 1 TO Len(names)
      IF Len(names[i]) > 3
         s_total += 1
         ? Upper(names[i]) + " tem nome longo"
      ELSE
         ? names[i] + " tem nome curto"
      ENDIF
   NEXT

   ? "Total de nomes longos: " + Str(s_total)
   ? "Tipo do array: " + ValType(names)
RETURN
```

Saída esperada:
```
ALICE tem nome longo
Bob tem nome curto
CHARLIE tem nome longo
Total de nomes longos: 2
Tipo do array: A
```
