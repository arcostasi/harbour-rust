# Pré-processador

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

- Parsing de regras com padrão e substituição
- Expansão parametrizada
- Corner cases documentados no upstream
- **Não implementado ainda**

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

## Estado atual

Fase 6 concluída:

- `#define` objeto com expansão recursiva e detecção de ciclo
- `#include` com quoted e angle-bracket, search paths configuráveis
- Handoff `pp -> parser` no CLI com `-I/--include-dir`
- Macros parametrizadas e `#command`/`#translate` pendentes para Fase 9
