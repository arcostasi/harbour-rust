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

### Programa

```
IrProgram {
    module_statics: Vec<IrStaticStatement>,
    routines: Vec<IrRoutine>,
}
```

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
- `Local(Vec<IrLocal>)`
- `Static(Vec<IrStatic>)`
- `Private(Vec<IrMemvar>)`
- `Public(Vec<IrMemvar>)`
- `Assign { target, value }`
- `Expression(IrExpr)`
- `If { condition, then_body, else_body }`
- `DoWhile { condition, body }`
- `For { var, start, stop, step, body }`

### Expressões

- `Read(path)` — leitura nominal explícita, hoje iniciando em `ReadPath::Name(Symbol)`
- `Read(path)` — leitura explícita, distinguindo `ReadPath::Name(Symbol)` e `ReadPath::Memvar(Symbol)` no subset dinâmico da Fase 8
- `Literal(Nil | Logical | Integer | Float | String)`
- `Binary(op, lhs, rhs)`
- `Unary(op, expr)`
- `Postfix(op, expr)`
- `Call(callee, args)`
- `Array(elements)` — literal de array
- `Codeblock(params, body)` — codeblock explícito
- `Macro(value)` — início do macro operator explícito
- `Assign(target, value)` — atribuição também pode existir em posição de expressão dentro de codeblocks
- `Index(target, indices)` — leitura indexada

### Alvos de atribuição

- `AssignTarget::Symbol(name)` — variável simples
- `AssignTarget::Memvar(name)` — memvar dinâmica explícita
- `AssignTarget::Index(target, indices)` — escrita indexada

## Lowering HIR -> IR

### O que muda

| HIR | IR |
| --- | --- |
| `Print(exprs)` | `BuiltinCall("QOut", exprs)` |
| `Read(ReadPath::Name(Symbol))` | `Read(ReadPath::Name(Symbol))` ou `Read(ReadPath::Memvar(Symbol))` |
| `Statement::Static` | `Statement::Static` |
| `Statement::Private/Public` | `Statement::Private/Public` |
| `Program.module_statics` | `Program.module_statics` |
| `Expression::Codeblock` | `Expression::Codeblock` |
| `Expression::Macro` | `Expression::Macro` |
| `Expression::Assign` | `Expression::Assign` |
| Expressões inválidas | Erro de lowering explícito |

### O que se preserva

- Estrutura de controle de fluxo (sem flattening para labels/gotos)
- Arrays e indexação como nós explícitos
- Atribuição indexada como `AssignTarget::Index`
- Distinção entre `Local` e `Static`
- Distinção entre `STATIC` de módulo e `STATIC` de rotina
- Leituras explícitas como `Read(path)`
- `PRIVATE`/`PUBLIC`, `codeblock` e `macro` como nós próprios da IR

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
- `STATIC` de rotina e de módulo + `Read(path)` explícitos — lowering OK
- `PRIVATE` / `PUBLIC` — lowering OK
- Literais de array — completo
- Indexação (leitura e escrita) — completo
- `Codeblock` / `macro` / memvar dinâmica explícita — lowering OK
- Flattening de controle de fluxo — não planejado para esta fase
