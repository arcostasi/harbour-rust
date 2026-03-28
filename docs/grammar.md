# Gramática

## Estratégia

Usar parser recursivo para statements e Pratt parser para expressões.

Referências principais:

- `harbour-core/src/compiler/harbour.y`
- `harbour-core/doc/clipper.txt`
- corpus de `harbour-core/tests/*.prg`

## Ordem de implementação

### Slice 1

- `PROCEDURE`
- `FUNCTION`
- `RETURN`
- `?`
- literals básicos
- chamada simples
- atribuição

### Slice 2

- `LOCAL`
- blocos de statements
- `IF / ELSE / ENDIF`
- `DO WHILE / ENDDO`
- `FOR / NEXT`

### Slice 3

- `STATIC`
- arrays
- operadores compostos
- chamadas com argumentos variados

### Slice 4

- codeblocks
- memvars
- extensões Harbour com flag

## Precedência inicial de expressões

Espelhar a ordem histórica observada em `harbour.y`:

1. pós-incremento/decremento
2. atribuição
3. `OR`
4. `AND`
5. `NOT`
6. relacionais
7. `+ -`
8. `* / %`
9. potência
10. unários

## Regras de design

- AST deve ser estável e explícita.
- Trivia não entra na AST, mas spans entram.
- Parser não consulta runtime.
- Parser não resolve símbolo.
- Recuperação de erro deve permitir múltiplos diagnósticos por arquivo.

## AST baseline da Fase 2

Primeira slice da AST procedural:

- `Program`
- `Item::Routine`
- `RoutineKind::{Procedure, Function}`
- `Statement::{Return, Print, Expression}`
- `Expression::{Identifier, Nil, Logical, Integer, Float, String, Call, Assignment}`

Esses nós existem para sustentar o parser inicial sem acoplar semântica ou runtime.

Extensão da terceira slice:

- `Statement::{Local, If, DoWhile, For}`
- `Expression::{Binary, Unary, Postfix}`
- operadores binários, unários e pós-fixos mínimos para controle de fluxo procedural

Primeira slice da Fase 7:

- `Statement::Static(StaticStatement)`
- `StorageClass::{Local, Static}` como hook explícito de storage no AST
- `LocalStatement` passa a carregar `storage_class`

Nesta etapa, o objetivo ainda não é parsear `STATIC`, mas estabilizar o modelo sintático para as slices seguintes de parser e semântica.

Segunda slice da Fase 7:

- `STATIC <id> [:= <expr>] [, ...]`
- initializer list espelhando a surface de `LOCAL`
- snapshot dedicada para `STATIC`

Nesta etapa, `STATIC` já entra no parser e na AST concreta, mas o lowering e a semântica de storage continuam explícita e deliberadamente pendentes.

Terceira slice da Fase 7:

- array literal vazio: `{}`
- array literal com elementos: `{ expr, expr, ... }`
- elementos como expressões arbitrárias do subconjunto atual
- snapshot dedicada para arrays

Nesta etapa, arrays já entram na AST e no parser como literal explícito, mas lowering, semântica de acesso/indexação e execução continuam pendentes.

## Parser baseline da Fase 2

Segunda slice já coberta pelo parser:

- `PROCEDURE <id>(...)`
- `FUNCTION <id>(...)`
- `RETURN`
- `RETURN <expr>`
- `? <expr-list>`
- statement de expressão para chamadas simples
- expressões primárias:
  - identificador,
  - `NIL`,
  - lógico,
  - inteiro,
  - float,
  - string,
  - chamada,
  - atribuição com `:=`

Terceira slice já coberta pelo parser:

- `LOCAL <id> [:= <expr>] [, ...]`
- `IF <expr> ... [ELSE ...] ENDIF`
- `DO WHILE <expr> ... ENDDO`
- `FOR <id> := <expr> TO <expr> [STEP <expr>] ... NEXT`
- expressões adicionais:
  - binárias para comparação, aritmética básica e `AND`/`OR`
  - unárias para `+`, `-`, `NOT`
  - pós-fixas para `++` e `--`

Slice de recovery e diagnósticos já coberta:

- recuperação de `IF`, `DO WHILE` e `FOR` quando `ENDIF`, `ENDDO` ou `NEXT` faltam antes da próxima rotina
- mensagens sintáticas no formato `expected ...; found ...`
- linha e coluna expostas diretamente em `ParseError`

## Casos sensíveis

- `?` como sugar de saída.
- separador por nova linha versus `;`
- operadores multi-caractere vindos do lexer/PP
- distinção entre baseline Clipper e extensões Harbour
- interação futura entre parser e macro/codeblock
