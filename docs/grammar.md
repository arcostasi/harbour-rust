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

## Casos sensíveis

- `?` como sugar de saída.
- separador por nova linha versus `;`
- operadores multi-caractere vindos do lexer/PP
- distinção entre baseline Clipper e extensões Harbour
- interação futura entre parser e macro/codeblock
