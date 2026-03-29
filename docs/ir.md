# IR — Intermediate Representation

## Responsabilidade

Representação intermediária entre a HIR e o backend de geração de código. Mais próxima do código gerado do que a HIR, mas ainda independente de target.

**Crate:** `harbour-rust-ir`

## Referências upstream

- `harbour-core/include/hbpcode.h` — modelo pcode histórico (referência, não contrato)
- `harbour-core/doc/pcode.txt` — instrução set do pcode
- `harbour-core/doc/vm.txt` — modelo de execução

## Pipeline

```text
HIR (anotada pela sema) ──lowering──> IR ──codegen──> C / futuro backend nativo
```

## Decisão: IR própria, não pcode

O upstream usa pcode + VM stack-based. `harbour-rust` usa uma IR estruturada própria porque:

- Permite backends diferentes (C hoje, nativo depois)
- Mais fácil de depurar e testar
- Não fecha o design numa VM específica
- pcode serve como referência semântica, não como contrato

## Nós principais

### Rotinas

```
IrRoutine {
    kind: Procedure | Function,
    name: String,
    params: Vec<String>,
    locals: Vec<IrLocal>,
    body: Vec<IrStatement>,
}
```

### Statements

- `Return(Option<IrExpr>)`
- `BuiltinCall(name, args)` — ex.: `QOut`
- `Assign { target, value }`
- `Expression(IrExpr)`
- `If { condition, then_body, else_body }`
- `DoWhile { condition, body }`
- `For { var, start, stop, step, body }`

### Expressões

- `Symbol(name)`
- `Literal(Nil | Logical | Integer | Float | String)`
- `Binary(op, lhs, rhs)`
- `Unary(op, expr)`
- `Postfix(op, expr)`
- `Call(callee, args)`
- `Array(elements)` — literal de array
- `Index(target, indices)` — leitura indexada

### Alvos de atribuição

- `AssignTarget::Symbol(name)` — variável simples
- `AssignTarget::Index(target, indices)` — escrita indexada

## Lowering HIR -> IR

### O que muda

| HIR | IR |
| --- | --- |
| `Print(exprs)` | `BuiltinCall("QOut", exprs)` |
| Símbolos | Strings normalizadas |
| Expressões inválidas | Erro de lowering explícito |

### O que se preserva

- Estrutura de controle de fluxo (sem flattening para labels/gotos)
- Arrays e indexação como nós explícitos
- Atribuição indexada como `AssignTarget::Index`

## Decisões de design

### Ainda estruturada

A IR desta fase preserva controle de fluxo estruturado. Flattening para labels/gotos virá quando houver necessidade real (otimização ou backend nativo).

### Erros de lowering explícitos

Expressões ou construções da HIR que não têm representação na IR ainda geram erro de lowering visível, em vez de silêncio ou panic.

### Backend-agnóstica

A IR não referencia detalhes do backend C (nomes de helpers, formato de output). O `codegen-c` é responsável por mapear nós IR para chamadas C específicas.

## Estado atual

Fase 5 + Fase 7 parcial:

- Rotinas, RETURN, BuiltinCall(QOut) — completo
- IF, DO WHILE, FOR — completo
- Atribuição simples — completo
- Literais de array — completo
- Indexação (leitura e escrita) — completo
- Flattening de controle de fluxo — não planejado para esta fase
