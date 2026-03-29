# Diagnósticos

## Meta

Mensagens previsíveis, localizadas e úteis desde a Fase 1.

## Regras

- todo diagnóstico tem arquivo, linha e coluna,
- mensagem principal curta,
- nota adicional quando houver contexto útil,
- sem `panic!` para erro de entrada do usuário,
- o mesmo erro não deve ser emitido em cascata sem valor.

## Categorias e códigos

### Léxicos (L)

| Código | Mensagem | Descrição |
| --- | --- | --- |
| L001 | Invalid token | Caractere não reconhecido |
| L002 | Unterminated string | String aberta sem fechar aspas |
| L003 | Unterminated comment | `/*` sem `*/` correspondente |
| L004 | Invalid number literal | Formato numérico malformado |

### Pré-processador (P)

| Código | Mensagem | Descrição |
| --- | --- | --- |
| P001 | Include file not found | Arquivo de `#include` não localizado |
| P002 | Cyclic macro expansion | `#define` com referência circular |
| P003 | Invalid directive | Diretiva PP não reconhecida |
| P004 | Unterminated conditional | `#ifdef` sem `#endif` |

### Sintáticos (S)

| Código | Mensagem | Descrição |
| --- | --- | --- |
| S001 | Expected token | Token esperado não encontrado |
| S002 | Unexpected token | Token inesperado no contexto |
| S003 | Unterminated block | `IF` sem `ENDIF`, `DO WHILE` sem `ENDDO`, etc. |
| S004 | Invalid expression | Expressão malformada |

### Semânticos (E)

| Código | Mensagem | Descrição |
| --- | --- | --- |
| E001 | Unresolved symbol | Variável ou função não declarada |
| E002 | Duplicate local | Variável declarada duas vezes no mesmo escopo |
| E003 | Unresolved callable | Chamada a função/procedimento não encontrado |
| E004 | Static storage unsupported | `STATIC` reconhecido mas sem runtime ainda |
| E005 | Type mismatch | Operação com tipos incompatíveis (futuro) |

### Runtime (R)

| Código | Mensagem | Descrição |
| --- | --- | --- |
| R001 | Type error | Operação inválida para o tipo do valor |
| R002 | Division by zero | Divisão por zero |
| R003 | Array bounds | Índice fora dos limites do array |
| R004 | Invalid argument | Argumento inválido para builtin |
| R005 | Unknown builtin | Builtin não implementado |

### Codegen (G)

| Código | Mensagem | Descrição |
| --- | --- | --- |
| G001 | Unsupported construct | Construção IR sem suporte no backend |
| G002 | Codegen error | Erro genérico na geração de código |

### CLI/Build (B)

| Código | Mensagem | Descrição |
| --- | --- | --- |
| B001 | File not found | Arquivo fonte não encontrado |
| B002 | C compiler not found | Compilador C host não detectado |
| B003 | C compilation failed | Compilador C retornou erro |

## Estrutura recomendada

```rust
struct Diagnostic {
    severity: Severity,
    code: Option<&'static str>,
    message: String,
    primary_span: Span,
    notes: Vec<String>,
}
```

## Fases

### Inicial

- spans corretos,
- linha/coluna,
- mensagem primária legível.
- parser recupera blocos procedurais básicos ao encontrar a próxima rotina ou `EOF`.

### Intermediária

- notas contextuais,
- agrupamento por arquivo,
- códigos estáveis.

### Avançada

- snippets de source,
- dicas de correção,
- múltiplos arquivos e includes.

## Casos prioritários

- token inválido,
- string não terminada,
- include não encontrado,
- símbolo ausente,
- uso fora de escopo,
- dialeto desabilitado,
- builtin ausente.
