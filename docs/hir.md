# HIR — High-level Intermediate Representation

## Responsabilidade

Representação intermediária de alto nível produzida por lowering da AST. Separa sintaxe de semântica, normalizando construções e preparando a estrutura para análise semântica.

**Crate:** `harbour-rust-hir`

## Referências upstream

- `harbour-core/src/compiler/*.c` — pipeline de compilação
- `harbour-core/doc/statics.txt` — modelo de storage
- `harbour-core/doc/clipper.txt` — semântica base

## Pipeline

```text
AST ──lowering──> HIR ──sema──> HIR anotada ──lowering──> IR
```

A HIR é o formato que a análise semântica consome. Ela normaliza:

- Identificadores para símbolos (preservando case original, comparáveis case-insensitive)
- Leituras nominais para `Read(path)` explícito, separado de alvo de escrita
- Atribuição restringida a alvo nominal simples
- Sugar sintático desmontado (ex.: operadores compostos -> `Assignment + Binary`)
- `STATIC` de módulo separado das rotinas, pronto para storage compartilhado

## Nós principais

### Programa

```
HirProgram {
    module_statics: Vec<HirStaticStatement>,
    routines: Vec<HirRoutine>,
}
```

### Rotinas

```
HirRoutine {
    kind: Procedure | Function,
    name: Symbol,
    params: Vec<Symbol>,
    locals: Vec<HirLocal>,
    body: Vec<HirStatement>,
}
```

### Declarações locais

```
HirLocal {
    name: Symbol,
    initializer: Option<HirExpr>,
}

HirStatic {
    name: Symbol,
    initializer: Option<HirExpr>,
}
```

### Statements

- `Return(Option<HirExpr>)`
- `Print(Vec<HirExpr>)` — normalização de `?`
- `Expression(HirExpr)`
- `Local(Vec<HirLocal>)`
- `Static(Vec<HirStatic>)`
- `If { condition, then_body, else_body }`
- `DoWhile { condition, body }`
- `For { var, start, stop, step, body }`
- `Assign { target, value }`

### Expressões

- `Read(path)` — leitura nominal explícita; hoje começa como `ReadPath::Name(Symbol)`
- `Literal(Nil | Logical | Integer | Float | String)`
- `Binary(op, lhs, rhs)`
- `Unary(op, expr)`
- `Postfix(op, expr)`
- `Call(callee, args)`
- `Array(elements)` — literal de array
- `Index(target, indices)` — indexação de array

### Alvos de atribuição

- `AssignTarget::Symbol(name)` — variável simples
- `AssignTarget::Index(target, indices)` — atribuição indexada (`a[1] := x`)

## Lowering AST -> HIR

### O que muda

| AST | HIR |
| --- | --- |
| Identificadores como strings | Símbolos normalizados |
| Leitura de identificador | `Read(ReadPath::Name(Symbol))` |
| `+=` como operator | `Assignment + Binary` |
| `?` como statement especial | `Print(exprs)` |
| `STATIC` como keyword | `Statement::Static` |

### O que não muda ainda

- Estrutura de controle de fluxo preservada (sem flattening)
- Sem resolução de escopo (responsabilidade da sema)
- Sem tipagem

## Decisões de design

### HIR pequena e explícita

A HIR começa mínima e cresce incrementalmente. Cada nó novo precisa de justificativa semântica.

### Sem reescrita pela sema

A sema anota a HIR via side tables, sem reescrevê-la. Isso mantém a HIR estável entre fases de análise.

### Storage explícito e leitura explícita

`STATIC` não é confundido com `LOCAL` na superfície da HIR: declarações lowered viram `Statement::Static` separado de `Statement::Local`.

Leituras simples também deixam de ser um `Symbol` cru e passam a usar `Read(path)` explícito. Nesta slice, o path inicial ainda é `ReadPath::Name(Symbol)`, mas a forma já fica pronta para storage e endereçamento mais específicos nas próximas fases.

### `STATIC` de módulo explícito

`STATIC` no nível de módulo também não é achatado para dentro de uma rotina artificial. O programa lowered preserva `module_statics` separados de `routines`, o que deixa explícito o contrato necessário para storage compartilhado entre rotinas do mesmo arquivo no backend.

## Estado atual

Fase 3 + Fase 7 parcial:

- Rotinas, LOCAL, RETURN, IF, DO WHILE, FOR, `?` — completo
- STATIC same-routine e module-level como nós explícitos de HIR — lowering OK
- Leituras nominais como `Read(path)` explícito — lowering OK
- Literais de array e indexação — lowering OK
- Operadores compostos — desugaring OK
- Atribuição indexada — lowering OK
