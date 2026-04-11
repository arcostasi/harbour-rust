# Compatibilidade

- [English](./COMPATIBILITY.md)
- [Português do Brasil](./COMPATIBILITY.pt-BR.md)

## Posicionamento

Harbour Rust busca compatibilidade prática com CA-Clipper e Harbour, começando por comportamentos que possam ser observados, testados e reproduzidos. Alegações de compatibilidade estão sempre subordinadas a testes explícitos e limitações documentadas.

## Panorama Atual

| Área | Status | Observações |
| --- | --- | --- |
| Lexer | baseline inicial estável | spans, posições, comentários, strings, números, keywords |
| Parser e AST | estável para o subconjunto atual | construções procedurais, arrays, sintaxe de memvar, codeblocks, macro reads |
| HIR e semântica | estável para o subconjunto atual | resolução de rotinas, bindings locais/`STATIC`, memvars |
| Runtime | baseline alpha amplo | valores centrais, arrays, builtins selecionados de string/matemática/conversão, cobertura de edge cases de strings guiada por oráculo para trim, busca, recorte, replicação, parsing de `Val()`, formatação de `Str()`, edge cases numéricos focados de `Round()`/`Int()`, saída executável de `Round()` com floats grandes alinhada para evitar notação científica, edge cases focados de compatibilidade em `Mod()`/`ValType()`/`Empty()` incluindo codeblocks e valores de erro no host C, edge cases focados de `Max()`/`Min()` e `Abs()`, edge cases focados de `Type()`/`Len()`, limites de overflow de string ao estilo Clipper em `Replicate()`/`Space()` e preservação executável de `Chr(0)` embutido em helpers selecionados do runtime host C |
| Pré-processador | subconjunto avançado curado | `#define`, `#include`, `#command`, `#translate`, além de cobertura ancorada no oráculo para replacements opcionais escapados, reordenação selecionada de cláusulas opcionais, um subconjunto focado de opcionais/listas nested derivado das regras `AAA`, `SET`, `AVG`, `INSERT` e `INSERT2` do upstream, incluindo também as declarações multi-linha de `SET`/`AVG` exercitadas em `hbpptest.prg`, incluindo cláusulas opcionais repetidas e invocação multi-linha no source para `INSERT`/`INSERT2`, um subconjunto focado de reordenação de cláusulas opcionais multi-linha derivado da regra upstream `_pp_test` `MYCOMMAND3`, um subconjunto focado de diretivas com padrão e corpo multi-linha derivado de `INSERT2`/`MYCOMMAND2`/`MYCOMMAND3`, incluindo a declaração multi-linha de `MYCOMMAND2` e a permutação com `ALL` antes da lista, result markers lógicos como `<.id.>`, um subconjunto mínimo de blockify `<{id}>`, um subconjunto ampliado de quoted-result para `<"id">`, incluindo os casos de string e quoted literals do `_NORMAL_M(<z>)` no `_pp_test`, um subconjunto ampliado de smart-result para `<(id)>`, incluindo os casos de string e quoted literals do `_SMART_M(<z>)` no `_pp_test`, um subconjunto ampliado de dumb-stringify para `#<id>`, incluindo os casos de string e quoted literals do `_DUMB_M(<z>)` no `_pp_test`, um subconjunto ampliado de captura de lista cobrindo `_REGULAR_L(<z,...>)` com normalização de item literal e preservação de whitespace na lista bruta, um subconjunto ampliado de result markers sobre lista cobrindo `_NORMAL_L(<z,...>)`, `_SMART_L(<z,...>)` e `_DUMB_L(<z,...>)` com separadores preservados e renderização por item ou da lista inteira alinhada ao upstream, um subconjunto focado de comando cobrindo `INDEX ON <key> TO <(file)>` com preservação dos espaços internos da expressão em `<"key">` e `<{key}>`, um subconjunto focado de resultado com array escapado cobrindo o comando adjacente `SET TOOLTIP TO <color> OF <form>` de `hbpptest.prg`, com preservação dos literais indexados `\[...\]` e renderização da cláusula opcional no replacement, um subconjunto adjacente focado de colchetes escapados cobrindo a regra exata `ZZZ [<v>] => QOUT([<v>\[1\]])` e os casos `a` / vazio / `a[1]+2` de `hbpptest.prg`, um subconjunto adjacente focado de translate escapado cobrindo `#xtranslate _HMG_a => _HMG\[137\]` dentro de source indexado em `hbpptest.prg`, um subconjunto focado de `#define` parametrizado saturando o corredor do `_pp_test.prg` com `clas(x)`, `DATEOLD(x)`, `datediff(x,y)`, `F1` e `F3`, com nomes de macro case-insensitive, expansão no call-site, passes focados repetidos, expansão de wrapper de construtor e substituição case-sensitive dos parâmetros no replacement, um subconjunto ampliado de pattern marker de macro para `<id:&>`, incluindo spillover em operadores, cadeias longas com múltiplos segmentos, misturas selecionadas com `&(expr)`, a sintaxe focada de match `XTRANS(<x>(` / `XTRANS(<x:&>(` já saturando o bloco completo de `XTRANS` do `_pp_test.prg`, um subconjunto adjacente focado de macro-call cobrindo `MXCALL`/`MYCALL`/`MZCALL` incluindo formas pós-expansão de `MXCALL` com `()`, `++`, parênteses e `.1`, o subconjunto adjacente de macros pareadas cobrindo `FOO ... FOO ...` / `BAR ... BAR ...`, o subconjunto adjacente restante de operadores/dot em `MCOMMAND` derivado de `hbpptest.prg`, o subconjunto adjacente de padrão composto com marker regular cobrindo `_REGULAR_(<z>)`, o subconjunto adjacente de padrão composto com normal stringify cobrindo `_NORMAL_M(<z>)`, o subconjunto adjacente de padrão composto com smart stringify cobrindo `_SMART_M(<z>)`, o subconjunto adjacente de padrão composto com dumb stringify cobrindo `_DUMB_M(<z>)`, e o subconjunto focado do `DEFINE WINDOW`/`#xtranslate` em estilo propriedade e construtor, incluindo `ON INIT`, traduções com e sem espaços ao redor de `.`, rewrites `(<name>{ ... }` e a variante Harbour-only de marker identificador `<!name!>` |
| Backend C | backend alpha prático | fluxo procedural, helpers de runtime selecionados e recursos dinâmicos |
| CLI | interface alpha utilizável | `help`, `check`, `build`, `run`, `transpile --to c` |
| RDD/DBF | baseline inicial utilizável | parsing de schema, navegação, leitura, append/update/delete/recall |
| Tooling de regressão | presente | golden tests, compare tool, benchmark smoke, scaffold de fuzzing |

## Limites Conhecidos

O projeto ainda está em alpha. Limites conhecidos incluem:

- cobertura parcial, não total, dos dialetos xBase;
- builtins selecionados implementados apenas para o subconjunto de tipos atualmente testado;
- ainda não existe backend nativo; C é o backend executável principal;
- ainda há lacunas de compatibilidade em comportamento avançado de macro, fidelidade mais ampla de runtime e cobertura estendida de RDD;
- casos avançados de pré-processador ainda permanecem em expansão nested de opcionais/listas além do subconjunto focado atual `AAA`/`SET`/`AVG`/`INSERT`/`INSERT2`/`MYCOMMAND3` multiline-reorder/multiline-result, agora incluindo as declarações multi-linha de `SET`/`AVG` exercitadas em `hbpptest.prg`, comportamento mais amplo de `#define` parametrizado além do subconjunto atualmente saturado do corredor `_pp_test` (`clas`, `DATEOLD`, `datediff`, `F1`, `F3`), comportamento mais amplo de pattern markers de macro além do subconjunto ampliado atual `<id:&>` e semântica mais ampla de marcadores como `<{id}>`, `<"id">` e `<(id)>` além dos subconjuntos atuais `_NORMAL_M(<z>)`/`_SMART_M(<z>)`/`_DUMB_M(<z>)` e `_NORMAL_L(<z,...>)`/`_SMART_L(<z,...>)`/`_DUMB_L(<z,...>)`;
- `Val()` agora segue o subconjunto ASCII atual guiado por oráculo para continuações com ponto final, sinais repetidos, paradas estilo expoente, pontuação mista e fragmentos separados por espaço após o separador decimal; a divergência remanescente ficou ligada à construção de `Chr(0)` embutido a partir do código-fonte no caminho atual de frontend/codegen;
- `Str()` agora segue o baseline atual guiado por oráculo para arredondamento half-away-from-zero guiado por largura, padding com largura negativa, números positivos grandes em largura default e preservação da escala visual de literais float no caminho executável em C; a lacuna documentada remanescente está na formatação em largura default de alguns números negativos grandes produzidos por expressão;
- a construção de strings com `Chr(0)` embutido a partir do código-fonte ainda é limitada no caminho atual de frontend/codegen, mesmo com o runtime executável em C já preservando esses bytes em helpers selecionados quando eles existem;
- edge cases históricos devem ser tratados como não suportados até serem testados e documentados.

## Política de Oráculo

- `harbour-core` é o principal oráculo de comportamento.
- Testes, fixtures e saída observada têm prioridade sobre suposições.
- O código-fonte do upstream pode ajudar no entendimento, mas a implementação precisa ser original.

## Política de Dialeto

- Comportamento Clipper-first é preferido quando há sobreposição.
- Extensões específicas de Harbour devem ser explícitas e documentadas.
- Toda divergência intencional precisa ser registrada em testes e documentação.
