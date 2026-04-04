# Análise Semântica

> Nota de transição: a versão pública bilíngue deste conteúdo está sendo migrada para [docs/en/technical/sema.md](./en/technical/sema.md) e [docs/pt-BR/technical/sema.md](./pt-BR/technical/sema.md).

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
- Tabelas de memvars por rotina (`PRIVATE`, `PUBLIC`)
- Diagnósticos de resolução

## Escopos

### Tabela global

Contém todas as rotinas (`PROCEDURE` e `FUNCTION`) declaradas no programa. A resolução de chamadas consulta esta tabela de forma case-insensitive.

### Tabela local

Cada rotina mantém uma tabela com:

- Parâmetros formais
- Variáveis `LOCAL`
- Variáveis `STATIC`
- Parâmetros de codeblock em escopos aninhados

### Tabela de memvars

Cada rotina também mantém uma tabela explícita de memvars declaradas com:

- `PRIVATE`
- `PUBLIC`

No baseline atual da Fase 8, leituras e escritas nominais ainda tentam resolver:

1. escopo local mais interno
2. `STATIC` de módulo
3. memvar declarada na rotina
4. memvar dinâmica implícita, quando o programa entra em modo dinâmico xBase

Isso preserva os diagnósticos estritos das fases anteriores para programas puramente procedurais, mas permite o primeiro recorte de memvars dinâmicas quando `PRIVATE`, `PUBLIC`, codeblocks ou macro read entram no programa.

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
- Corpos e parâmetros de codeblocks
- Expressões de macro read
- Targets e índices de indexação
- Argumentos de chamadas

## Baselines curados

| Fixture | Golden |
| --- | --- |
| `tests/fixtures/sema/control_flow_missing_locals.prg` | `.errors` |
| `tests/fixtures/sema/control_flow_missing_callables.prg` | `.errors` |
| `tests/fixtures/parser/static.prg` | sucesso semântico com bindings `STATIC` same-routine |
| `tests/fixtures/parser/static_module.prg` | sucesso semântico com bindings `STATIC` compartilhados |
| `tests/fixtures/parser/memvars.prg` | sucesso semântico com bindings `PRIVATE`/`PUBLIC` explícitos |
| `tests/fixtures/parser/private_dynamic.prg` | sucesso semântico com fallback de memvar dinâmica entre rotinas |
| `tests/fixtures/parser/codeblock.prg` | sucesso semântico com parâmetros de codeblock e resolução em escopo aninhado |

## Próximos passos (Fase 7+)

- Resolução mais fina de tipos
- Diagnósticos de type mismatch quando viável
- Refinar fallback de memvar dinâmica contra o oracle do `harbour-core`
- Separar melhor leitura local, memvar explícita e memvar dinâmica na IR/runtime

## Estado atual

Fase 3 + Fase 7 parcial:

- Tabela global de rotinas — completo
- Tabela local com LOCAL e parâmetros — completo
- Resolução case-insensitive — completo
- Diagnósticos de símbolo ausente — completo
- STATIC declarado e resolvido na sema — completo
- STATIC de módulo compartilhado entre rotinas — completo no mesmo arquivo
- Walk de arrays e indexação — completo
- `PRIVATE` / `PUBLIC` como memvars explícitas — completo no recorte atual
- Parâmetros de codeblock como escopo aninhado — completo no recorte atual
- Fallback de memvar dinâmica entre rotinas com features da Fase 8 — parcial e documentado
