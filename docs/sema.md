# Análise Semântica

## Responsabilidade

Validar e anotar a HIR com informações de escopo, resolução de símbolos e diagnósticos semânticos.

**Crate:** `harbour-rust-sema`

## Referências upstream

- `harbour-core/src/compiler/*.c` — resolução de símbolos
- `harbour-core/doc/statics.txt` — modelo de storage de statics
- `harbour-core/doc/clipper.txt` — semântica Clipper

## Pipeline

```text
HIR ──sema──> HIR + side tables (símbolos resolvidos, diagnósticos)
```

A sema **não reescreve** a HIR. Ela produz:

- Tabela global de rotinas
- Tabela global de `STATIC` de módulo
- Tabelas locais por rotina (parâmetros, `LOCAL`, `STATIC`)
- Diagnósticos de resolução

## Escopos

### Tabela global

Contém todas as rotinas (`PROCEDURE` e `FUNCTION`) declaradas no programa. A resolução de chamadas consulta esta tabela de forma case-insensitive.

### Tabela local

Cada rotina mantém uma tabela com:

- Parâmetros formais
- Variáveis `LOCAL`
- Variáveis `STATIC`

### Tabela de `STATIC` de módulo

O programa também mantém uma tabela global separada para `STATIC` declarados fora de rotinas. Leituras e escritas nominais dentro de rotinas consultam:

1. bindings locais e parâmetros
2. `STATIC` de módulo
3. erro de símbolo não resolvido

Isso permite o baseline atual de storage compartilhado entre rotinas do mesmo arquivo.

### Resolução case-insensitive

Consistente com Clipper/Harbour: `myFunc`, `MYFUNC` e `MyFunc` resolvem para a mesma entrada.

## Diagnósticos

| Código | Mensagem | Contexto |
| --- | --- | --- |
| — | Unresolved symbol | Variável ou função usada sem declaração visível |
| — | Duplicate local | Variável declarada mais de uma vez no mesmo escopo |
| — | Duplicate module static | `STATIC` de módulo declarado mais de uma vez |
| — | Unresolved callable | Chamada a função/procedimento não declarado |

## Decisões de design

### Side tables, não reescrita

As decisões de binding ficam em side tables separadas. Isso mantém a HIR pequena e estável, facilita testes incrementais e evita dependências circulares.

### Builtins como lookup especial

Builtins (`QOut`, `AAdd`, `ASize`, `AClone`, etc.) são registrados separadamente e consultados quando a tabela global não resolve um nome.

### Walk completo

A sema percorre todos os nós da HIR, incluindo:

- Condições de `IF`, `DO WHILE`, `FOR`
- Elementos de literais de array
- Targets e índices de indexação
- Argumentos de chamadas

## Baselines curados

| Fixture | Golden |
| --- | --- |
| `tests/fixtures/sema/control_flow_missing_locals.prg` | `.errors` |
| `tests/fixtures/sema/control_flow_missing_callables.prg` | `.errors` |
| `tests/fixtures/parser/static.prg` | sucesso semântico com bindings `STATIC` same-routine |
| `tests/fixtures/parser/static_module.prg` | sucesso semântico com bindings `STATIC` compartilhados |

## Próximos passos (Fase 7+)

- Resolução mais fina de tipos
- Diagnósticos de type mismatch quando viável
- Suporte a memvars (PRIVATE/PUBLIC) na Fase 8

## Estado atual

Fase 3 + Fase 7 parcial:

- Tabela global de rotinas — completo
- Tabela local com LOCAL e parâmetros — completo
- Resolução case-insensitive — completo
- Diagnósticos de símbolo ausente — completo
- STATIC declarado e resolvido na sema — completo
- STATIC de módulo compartilhado entre rotinas — completo no mesmo arquivo
- Walk de arrays e indexação — completo
