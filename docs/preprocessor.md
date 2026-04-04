# Pré-processador

> Nota de transição: a versão pública bilíngue deste conteúdo está sendo migrada para [docs/en/technical/preprocessor.md](./en/technical/preprocessor.md) e [docs/pt-BR/technical/preprocessor.md](./pt-BR/technical/preprocessor.md).

## Responsabilidade

Processar diretivas de compilação (`#define`, `#include`, `#command`, `#translate`) antes da tokenização principal, mantendo compatibilidade com o PP do Clipper/Harbour.

**Crate:** `harbour-rust-pp`

## Referências upstream

- `harbour-core/src/pp/ppcore.c` — implementação de referência do PP
- `harbour-core/doc/pp.txt` — especificação detalhada do PP Clipper
- `harbour-core/tests/hbpp/` — corpus de testes do PP

## Pipeline do PP

```text
source .prg
    │
    v
┌──────────────────────┐
│ Diretivas (#define,  │
│ #include, #command,  │
│ #translate)          │
└──────────────────────┘
    │
    v
source expandido ──> lexer ──> parser
```

O PP opera antes do lexer principal. O handoff é:

1. PP recebe o source original
2. PP resolve includes, expande defines e regras de tradução
3. PP emite source expandido com mapeamento de origem
4. Lexer tokeniza o resultado

## Diretivas suportadas

### `#define` (Fase 6)

- **Macros objeto:** `#define NOME valor`
- Expansão case-insensitive por identificador inteiro
- Expansão recursiva: `A -> B -> "x"` resolve até o valor final
- Detecção de ciclo: `A -> B -> A` gera diagnóstico explícito
- Não expande dentro de strings e comentários
- **Macros parametrizadas:** pendente

### `#include` (Fase 6)

- **Quoted:** `#include "arquivo.ch"` — busca relativa ao arquivo atual, depois search paths
- **Angle-bracket:** `#include <arquivo.ch>` — busca direto nos search paths
- Search paths configuráveis via `-I/--include-dir` no CLI
- Resolver de includes isolado em `FileSystemIncludeResolver`

### `#command` / `#translate` (Fase 9)

Recorte atual já implementado:

- `#command`, `#xcommand`, `#translate` e `#xtranslate`
- parsing de padrão `... => ...`
- marcador regular `<id>`
- marcador de lista `<id,...>`
- marcador restrito `<id:ON,OFF>`
- cláusulas opcionais `[ ... ]` no padrão e no resultado
- stringify `#<id>` no resultado
- continuação de diretiva em múltiplas linhas com `;`

Limitações ainda abertas nesta fase:

- sem markers avançados como `<(id)>`, `<"id">`, `<{id}>` e `<.id.>`
- sem macro markers `:<&>` e variantes mais complexas do upstream
- sem nested optional/list expansion completa do `_pp_test.prg`
- sem engine token-based fiel ao `ppcore.c`; o recorte atual continua tokenização leve sobre source textual

### Outras diretivas (futuras)

- `#ifdef`, `#ifndef`, `#else`, `#endif`
- `#stdout`, `#error`
- `#pragma`

## Source graph

O PP mantém um grafo de fontes para rastrear origem:

- `SourceFile` como unidade de entrada
- Mapeamento de linhas de saída para arquivo e linha de origem
- Suporte a diagnósticos que apontem para o arquivo `.ch` incluído

## Decisões de design

### Token-based vs text-based

`doc/pp.txt` especifica que compatibilidade real com Clipper exige PP token-based. A implementação atual começa text-based para destravar o pipeline, com migração planejada para token-based.

### Resolver isolado

A lógica de busca de arquivos está isolada em `FileSystemIncludeResolver`, facilitando testes unitários com filesystem mockado e políticas alternativas de busca.

### Expansão case-insensitive

Consistente com o restante do Clipper/Harbour: `#define FOO 1` expande tanto `FOO` quanto `foo` quanto `Foo`.

## Baselines curados

| Fixture | Golden / Comportamento |
| --- | --- |
| `tests/fixtures/pp/define_root.prg` | `define_root.out` |
| `tests/fixtures/pp/include_root.prg` | `include_root.out` |
| `tests/fixtures/pp/recursive_define_root.prg` | `recursive_define_root.out` |
| `tests/fixtures/pp/cyclic_define_root.prg` | erro de ciclo |
| `tests/fixtures/pp/quoted_search_path_root.prg` | fallback para search path |
| `tests/fixtures/pp/angle_search_path_root.prg` | resolvido por search path |
| `tests/fixtures/pp/command_translate_root.prg` | golden de `#command` + `#translate` |
| `tests/fixtures/pp/rule_markers_root.prg` | golden de opcionais, lista, restrição e stringify |
| `tests/fixtures/pp/multiline_command_root.prg` | golden de diretiva multi-linha com `;` |
| `tests/fixtures/pp/malformed_rule_root.prg` | erro explícito de regra malformada |
| `tests/fixtures/pp/phase9_acceptance.prg` | `harbour-rust-cli build/run` com `#command` + `#translate` no pipeline completo |
| `tests/fixtures/pp/phase9_preprocess_error.prg` | erro explícito de preprocessamento no CLI para regra malformada |

## Estado atual

Fases 6 e 9 concluídas:

- `#define` objeto com expansão recursiva e detecção de ciclo
- `#include` com quoted e angle-bracket, search paths configuráveis
- Handoff `pp -> parser` no CLI com `-I/--include-dir`
- `#command`/`#translate` já cobrem o primeiro subset com marcadores regulares, listas, restrições, opcionais, stringify e continuação por `;`
- baseline de compatibilidade focado contra `doc/pp.txt` e `tests/hbpp/_pp_test.prg`
- fixture executável `tests/fixtures/pp/phase9_acceptance.prg` já valida o caminho completo `pp -> parser -> runtime`
- markers/result markers avançados e compatibilidade com corpus maior do `tests/hbpp/_pp_test.prg` continuam pendentes
