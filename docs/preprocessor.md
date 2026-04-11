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
- **Macros parametrizadas:** subset focado de call-site em linhas normais já satura o corredor correspondente do `_pp_test.prg` com `clas(x)`, `DATEOLD(x)`, `datediff(x,y)`, `F1` e `F3`, com nome da macro case-insensitive, passes focados repetidos, wrapper `clas( x )   (x)` antes de `#xtranslate` e substituição de parâmetros case-sensitive dentro do replacement

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
- corpo de resultado multi-linha quando a regra termina a linha exatamente em `=>`

Limitações ainda abertas nesta fase:

- sem semântica mais ampla de `<{id}>`, `<"id">` e `<(id)>` em capturas com múltiplas expressões, strings e macros, além de behavior mais amplo de pattern markers de macro além do subset atual `<id:&>` com spillover em operadores, cadeias longas multi-segmento e misturas selecionadas com `&(expr)`
- sem macro markers `:<&>` e variantes mais complexas do upstream
- sem nested optional/list expansion completa do `_pp_test.prg` além do subset focado `AAA`/`SET`/`AVG`/`INSERT`/`INSERT2`, agora incluindo também as declarações multi-linha de `SET`/`AVG` em `hbpptest.prg`
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
| `tests/fixtures/pp/logical_marker_root.prg` | golden de result marker lógico `<.id.>` |
| `tests/fixtures/pp/blockify_marker_root.prg` | golden do subset mínimo de blockify result marker `<{id}>` |
| `tests/fixtures/pp/quoted_marker_root.prg` | golden do subset mínimo de result marker `<"id">` |
| `tests/fixtures/pp/quoted_macro_marker_root.prg` | golden do subset macro-orientado de result marker `<"id">` |
| `tests/fixtures/pp/smart_marker_root.prg` | golden do subset mínimo de smart result marker `<(id)>` |
| `tests/fixtures/pp/smart_marker_macro_root.prg` | golden do subset macro-orientado de smart result marker `<(id)>` |
| `tests/fixtures/pp/macro_pattern_translate_root.prg` | golden do subset mínimo de pattern marker de macro `<id:&>` em `#translate` |
| `tests/fixtures/pp/macro_pattern_command_root.prg` | golden do subset mínimo de pattern marker de macro `<id:&>` em `#command` |
| `tests/fixtures/pp/nested_optional_list_root.prg` | golden do subset focado de expansão de resultado com opcionais/listas (`SET`/`AVG`) |
| `tests/fixtures/pp/multiline_nested_optional_list_root.prg` | golden do mesmo subset `SET`/`AVG` com diretivas multi-linha após `=>` |
| `tests/fixtures/pp/insert_rule_root.prg` | golden do subset focado de cláusulas opcionais repetidas e continuação de source line (`INSERT`/`INSERT2`) |
| `tests/fixtures/pp/multiline_result_rule_root.prg` | golden do subset focado de corpo multi-linha após `=>` (`INSERT2`/`MYCOMMAND2`/`MYCOMMAND3`) |
| `tests/fixtures/pp/multiline_pattern_rule_root.prg` | golden do subset focado de padrão multi-linha antes do `=>` (`MYCOMMAND2`) |
| `tests/fixtures/pp/xtrans_match_root.prg` | golden do subset focado de `XTRANS(<x>(` / `XTRANS(<x:&>(`, incluindo variantes com ponto, indexação, operadores lógicos e preservação literal |
| `tests/fixtures/pp/xtrans_macro_chain_root.prg` | golden do subset focado de `XTRANS(<x:&>(` com cadeias concatenadas `&id&id` e `&id.&id[.]` |
| `tests/fixtures/pp/xtrans_full_root.prg` | golden consolidado do bloco completo `XTRANS` do `_pp_test.prg` |
| `tests/fixtures/pp/macro_call_root.prg` | golden do subset focado de macro-calls adjacentes `MXCALL`/`MYCALL`/`MZCALL` |
| `tests/fixtures/pp/macro_pair_root.prg` | golden do subset focado de macros pareadas adjacentes `FOO ... FOO ...` / `BAR ... BAR ...` |
| `tests/fixtures/pp/mxcall_post_root.prg` | golden do subset focado das formas pós-expansão de `MXCALL` com `()`, `++`, parênteses e `.1` |
| `tests/fixtures/pp/macro_command_operator_root.prg` | golden do subset focado das variantes dot/operator de `MCOMMAND` |
| `tests/fixtures/pp/define_window_root.prg` | golden do subset focado de `DEFINE WINDOW` com `ON INIT` e property translation sem espaços ao redor de `.` |
| `tests/fixtures/pp/property_translate_root.prg` | golden do subset focado do mesmo `#xtranslate` de propriedade fora do wrapper `DEFINE WINDOW` |
| `tests/fixtures/pp/constructor_translate_root.prg` | golden do subset focado de constructor-style `#xtranslate` com padrão composto `(<name>{ [<p,...>] }` |
| `tests/fixtures/pp/constructor_identifier_translate_root.prg` | golden do subset Harbour-only de constructor-style `#xtranslate` com marker identificador `<!name!>` |
| `tests/fixtures/pp/regular_marker_compound_root.prg` | golden do subset focado de padrão composto `_REGULAR_(<z>)` sem espaços ao redor dos delimitadores |
| `tests/fixtures/pp/normal_marker_compound_root.prg` | golden do subset focado de normal stringify `_NORMAL_M(<z>)` sem espaços ao redor dos delimitadores, incluindo strings e quoted literals |
| `tests/fixtures/pp/smart_marker_compound_root.prg` | golden do subset focado de smart stringify `_SMART_M(<z>)` sem espaços ao redor dos delimitadores, incluindo strings e quoted literals |
| `tests/fixtures/pp/dumb_marker_compound_root.prg` | golden do subset focado de dumb stringify `_DUMB_M(<z>)` sem espaços ao redor dos delimitadores, incluindo strings e quoted literals |
| `tests/fixtures/pp/regular_list_compound_root.prg` | golden do subset focado de `_REGULAR_L(<z,...>)` com normalização de item literal e preservação de whitespace na lista recomposta |
| `tests/fixtures/pp/normal_list_compound_root.prg` | golden do subset focado de `_NORMAL_L(<z,...>)` com renderização quoted por item e separadores preservados |
| `tests/fixtures/pp/smart_list_compound_root.prg` | golden do subset focado de `_SMART_L(<z,...>)` com renderização smart por item e separadores preservados |
| `tests/fixtures/pp/dumb_list_compound_root.prg` | golden do subset focado de `_DUMB_L(<z,...>)` com stringify da lista inteira e separadores preservados |
| `tests/fixtures/pp/index_preserve_spaces_root.prg` | golden do subset focado de `INDEX ON <key> TO <(file)>` preservando espaços internos da expressão em `<"key">` e `<{key}>` |
| `tests/fixtures/pp/function_like_define_case_root.prg` | golden do subset focado de `#define` parametrizado `F1`/`F3` com substituição case-sensitive dos parâmetros |
| `tests/fixtures/pp/nested_function_like_define_root.prg` | golden do subset focado de `#define` parametrizado nested `datediff -> DATEOLD -> DATENEW` |
| `tests/fixtures/pp/constructor_wrapper_function_like_define_root.prg` | golden do último caso do corredor `_pp_test` de `#define` parametrizado, com `clas( x )   (x)` antes do subset constructor-style |
| `tests/fixtures/pp/tooltip_command_root.prg` | golden do subset focado de `SET TOOLTIP TO <color> OF <form>` com arrays escapados e cláusula opcional no replacement |
| `tests/fixtures/pp/zzz_escape_root.prg` | golden do subset focado da regra exata `ZZZ [<v>] => QOUT([<v>\[1\]])` de `hbpptest.prg`, incluindo os casos `a`, vazio e `a[1]+2` |
| `tests/fixtures/pp/hmg_escape_translate_root.prg` | golden do subset focado de `#xtranslate _HMG_a => _HMG\[137\]` em source indexado de `hbpptest.prg` |
| `tests/fixtures/pp/set_filter_macro_root.prg` | golden do subset focado de `SET FILTER TO <x:&>` derivado de `std.ch` com os três casos observáveis de `hbpptest.prg` |
| `tests/fixtures/pp/copy_structure_extended_root.prg` | golden do subset focado de `COPY [STRUCTURE] [EXTENDED] [TO <(f)>]` com o caso adjacente `COPY STRUCTURE EXTENDED TO teststru` de `hbpptest.prg` |
| `tests/fixtures/pp/get_command_base_root.prg` | golden do primeiro subset focado de `@ <row>, <col> GET <var>` com o caso-base `@ 0,1 GET a` de `hbpptest.prg` |
| `tests/fixtures/pp/optional_reorder_root.prg` | golden do subset focado de reordenação de cláusulas opcionais multi-linha com lista (`MYCOMMAND3`) |
| `tests/fixtures/pp/nested_optional_match_root.prg` | golden do subset focado de nested optional match (`AAA`) |
| `tests/fixtures/pp/multiline_command_root.prg` | golden de diretiva multi-linha com `;` |
| `tests/fixtures/pp/malformed_rule_root.prg` | erro explícito de regra malformada |
| `tests/fixtures/pp/phase9_acceptance.prg` | `harbour-rust-cli build/run` com `#command` + `#translate` no pipeline completo |
| `tests/fixtures/pp/phase9_preprocess_error.prg` | erro explícito de preprocessamento no CLI para regra malformada |

## Estado atual

Fases 6, 9 e 13 concluídas:

- `#define` objeto com expansão recursiva e detecção de ciclo
- subset focado de `#define` parametrizado que já satura o corredor correspondente do `_pp_test.prg` (`clas`, `DATEOLD`, `datediff`, `F1`, `F3`), com expansão só em call-site, passes focados repetidos, wrapper de construtor preservado e sensibilidade de maiúsculas/minúsculas preservada nos nomes de parâmetros
- subset focado do comando `SET TOOLTIP TO <color> OF <form>` de `hbpptest.prg`, cobrindo arrays escapados `\[...\]` no replacement e a cláusula opcional final do `RGB(...)`
- subset focado da regra exata `ZZZ [<v>] => QOUT([<v>\[1\]])` de `hbpptest.prg`, alinhando os casos `a`, vazio e `a[1]+2` sem os espaços extras do fixture antigo de `optional_stringify`
- subset focado de `#xtranslate _HMG_a => _HMG\[137\]` de `hbpptest.prg`, cobrindo replacement escapado dentro do source indexado `v:= _bro[ a( _HMG_a [i] ) ]`
- subset focado de `SET FILTER TO <x:&>` derivado de `std.ch`, cobrindo os casos adjacentes `&cVar.`, `&(cVar .AND. &cVar)` e fallback `&cVar. .AND. cVar` observados em `hbpptest.prg`
- subset focado de `COPY [STRUCTURE] [EXTENDED] [TO <(f)>]` derivado de `std.ch`, cobrindo o caso adjacente `COPY STRUCTURE EXTENDED TO teststru` observado em `hbpptest.prg`
- primeiro subset focado do bloco maior `@ <row>, <col> GET <var>` de `hbpptest.prg`, cobrindo apenas o caso-base `@ 0,1 GET a` antes das variantes com `PICTURE`, `VALID` e `WHEN`
- `#include` com quoted e angle-bracket, search paths configuráveis
- Handoff `pp -> parser` no CLI com `-I/--include-dir`
- `#command`/`#translate` já cobrem o primeiro subset com marcadores regulares, listas, restrições, opcionais, stringify, continuação por `;` e um subset focado de corpo multi-linha quando o resultado começa na linha seguinte ao `=>`
- baseline de compatibilidade focado contra `doc/pp.txt`, `tests/hbpp/_pp_test.prg` e `tests/hbpp/hbpptest.prg`, incluindo replacements com `\[`/`\]`, reordenação selecionada de cláusulas opcionais contíguas, um subset focado `AAA`/`SET`/`AVG`/`INSERT`/`INSERT2` para nested optional/list e cláusulas repetidas, agora também com as declarações multi-linha de `SET`/`AVG` em `hbpptest.prg`, um subset focado de reordenação de cláusulas opcionais multi-linha derivado de `MYCOMMAND3`, um subset focado de padrão e corpo multi-linha em `MYCOMMAND2`/`INSERT2`/`MYCOMMAND3`, incluindo a declaração `MYCOMMAND2 [<myList,...>]` repartida antes do `=>` e a permutação `MYCOMMAND2 MYCLAUSE 321 ALL "HELLO"`, um subset focado de `XTRANS(<x>(` / `XTRANS(<x:&>(` com diferenciação entre match regular e macro, result markers lógicos `<.id.>`, um subset mínimo de `<{id}>`, um subset macro-orientado de `<"id">`, um subset macro-orientado de `<(id)>` e um subset expandido de pattern marker `<id:&>` com spillover em operadores, cadeias longas multi-segmento e misturas selecionadas com `&(expr)`
- baseline de compatibilidade focado contra `doc/pp.txt`, `tests/hbpp/_pp_test.prg` e `tests/hbpp/hbpptest.prg`, incluindo replacements com `\[`/`\]`, reordenação selecionada de cláusulas opcionais contíguas, um subset focado `AAA`/`SET`/`AVG`/`INSERT`/`INSERT2` para nested optional/list e cláusulas repetidas, agora também com as declarações multi-linha de `SET`/`AVG` em `hbpptest.prg`, um subset focado de reordenação de cláusulas opcionais multi-linha derivado de `MYCOMMAND3`, um subset focado de padrão e corpo multi-linha em `MYCOMMAND2`/`INSERT2`/`MYCOMMAND3`, incluindo a declaração `MYCOMMAND2 [<myList,...>]` repartida antes do `=>` e a permutação `MYCOMMAND2 MYCLAUSE 321 ALL "HELLO"`, um subset focado de `XTRANS(<x>(` / `XTRANS(<x:&>(` com diferenciação entre match regular, macro, variantes com ponto, indexação, operadores lógicos e preservação literal, result markers lógicos `<.id.>`, um subset mínimo de `<{id}>`, um subset macro-orientado de `<"id">`, um subset macro-orientado de `<(id)>` e um subset expandido de pattern marker `<id:&>` com spillover em operadores, cadeias longas multi-segmento e misturas selecionadas com `&(expr)`
- fixture executável `tests/fixtures/pp/phase9_acceptance.prg` já valida o caminho completo `pp -> parser -> runtime`
- semântica mais ampla de markers/result markers avançados e compatibilidade com corpus maior do `tests/hbpp/_pp_test.prg` continuam pendentes
