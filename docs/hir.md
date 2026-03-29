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
- Atribuição restringida a alvo nominal simples
- Sugar sintático desmontado (ex.: operadores compostos -> `Assignment + Binary`)

## Nós principais

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
    storage_class: Local | Static,
    initializer: Option<HirExpr>,
}
```

### Statements

- `Return(Option<HirExpr>)`
- `Print(Vec<HirExpr>)` — normalização de `?`
- `Expression(HirExpr)`
- `If { condition, then_body, else_body }`
- `DoWhile { condition, body }`
- `For { var, start, stop, step, body }`
- `Assign { target, value }`

### Expressões

- `Symbol(name)` — referência a variável ou função
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
| `+=` como operator | `Assignment + Binary` |
| `?` como statement especial | `Print(exprs)` |
| `STATIC` como keyword | `storage_class: Static` |

### O que não muda ainda

- Estrutura de controle de fluxo preservada (sem flattening)
- Sem resolução de escopo (responsabilidade da sema)
- Sem tipagem

## Decisões de design

### HIR pequena e explícita

A HIR começa mínima e cresce incrementalmente. Cada nó novo precisa de justificativa semântica.

### Sem reescrita pela sema

A sema anota a HIR via side tables, sem reescrevê-la. Isso mantém a HIR estável entre fases de análise.

### Storage class explícito

`STATIC` não é confundido com `LOCAL` — o lowering preserva a distinção para que a sema e o codegen possam tratá-los diferentemente.

## Estado atual

Fase 3 + Fase 7 parcial:

- Rotinas, LOCAL, RETURN, IF, DO WHILE, FOR, `?` — completo
- STATIC com storage_class explícito — lowering OK, runtime pendente
- Literais de array e indexação — lowering OK
- Operadores compostos — desugaring OK
- Atribuição indexada — lowering OK
