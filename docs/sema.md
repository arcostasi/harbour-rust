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

### Resolução case-insensitive

Consistente com Clipper/Harbour: `myFunc`, `MYFUNC` e `MyFunc` resolvem para a mesma entrada.

## Diagnósticos

| Código | Mensagem | Contexto |
| --- | --- | --- |
| — | Unresolved symbol | Variável ou função usada sem declaração visível |
| — | Duplicate local | Variável declarada mais de uma vez no mesmo escopo |
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
| `tests/fixtures/parser/static.prg` | sucesso semântico com bindings `STATIC` |

## Próximos passos (Fase 7+)

- Storage de `STATIC` compartilhado entre rotinas do mesmo módulo
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
- Walk de arrays e indexação — completo
