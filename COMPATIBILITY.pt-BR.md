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
| Pré-processador | subconjunto avançado curado | `#define`, `#include`, `#command`, `#translate`, além de cobertura ancorada no oráculo para replacements opcionais escapados, reordenação selecionada de cláusulas opcionais, um subconjunto focado de opcionais/listas nested derivado das regras `AAA`, `SET`, `AVG`, `INSERT` e `INSERT2` do upstream, incluindo também as declarações multi-linha de `SET`/`AVG` exercitadas em `hbpptest.prg`, incluindo cláusulas opcionais repetidas e invocação multi-linha no source para `INSERT`/`INSERT2`, um subconjunto focado de reordenação de cláusulas opcionais multi-linha derivado da regra upstream `_pp_test` `MYCOMMAND3`, um subconjunto focado de diretivas com padrão e corpo multi-linha derivado de `INSERT2`/`MYCOMMAND2`/`MYCOMMAND3`, incluindo a declaração multi-linha de `MYCOMMAND2` e a permutação com `ALL` antes da lista, result markers lógicos como `<.id.>`, um subconjunto mínimo de blockify `<{id}>`, um subconjunto quoted-result orientado a macros para `<"id">`, um subconjunto smart-result orientado a macros para `<(id)>`, um subconjunto ampliado de pattern marker de macro para `<id:&>`, incluindo spillover em operadores, cadeias longas com múltiplos segmentos, misturas selecionadas com `&(expr)`, a sintaxe focada de match `XTRANS(<x>(` / `XTRANS(<x:&>(` já saturando o bloco completo de `XTRANS` do `_pp_test.prg`, um subconjunto adjacente focado de macro-call cobrindo `MXCALL`/`MYCALL`/`MZCALL` incluindo formas pós-expansão de `MXCALL` com `()`, `++`, parênteses e `.1`, e o subconjunto adjacente de macros pareadas cobrindo `FOO ... FOO ...` / `BAR ... BAR ...` |
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
- casos avançados de pré-processador ainda permanecem em expansão para combinações mais amplas de dumb-stringify, expansão nested de opcionais/listas além do subconjunto focado atual `AAA`/`SET`/`AVG`/`INSERT`/`INSERT2`/`MYCOMMAND3` multiline-reorder/multiline-result, agora incluindo as declarações multi-linha de `SET`/`AVG` exercitadas em `hbpptest.prg`, comportamento mais amplo de pattern markers de macro além do subconjunto ampliado atual `<id:&>` e semântica mais ampla de marcadores como `<{id}>`, `<"id">` e `<(id)>` em cenários com múltiplas expressões, strings e macros;
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
