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
| Runtime | baseline alpha amplo | valores centrais, arrays, builtins selecionados de string/matemática/conversão, cobertura de edge cases de strings guiada por oráculo para trim, busca, recorte, replicação, parsing de `Val()`, formatação de `Str()`, edge cases numéricos focados de `Round()`/`Int()`, saída executável de `Round()` com floats grandes alinhada para evitar notação científica, edge cases focados de compatibilidade em `Mod()`/`ValType()`/`Empty()`, edge cases focados de `Max()`/`Min()` e `Abs()`, edge cases focados de `Type()`/`Len()`, limites de overflow de string ao estilo Clipper em `Replicate()`/`Space()` e preservação executável de `Chr(0)` embutido em helpers selecionados do runtime host C |
| Pré-processador | subconjunto avançado curado | `#define`, `#include`, `#command`, `#translate` |
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
- `Val()` agora segue o subconjunto ASCII atual guiado por oráculo para continuações com ponto final, sinais repetidos, paradas estilo expoente, pontuação mista e fragmentos separados por espaço após o separador decimal; a divergência remanescente ficou ligada à construção de `Chr(0)` embutido a partir do código-fonte no caminho atual de frontend/codegen;
- `Str()` ainda tem lacunas documentadas em alguns detalhes históricos de formatação, mas o arredondamento half-away-from-zero guiado por largura e o padding com largura negativa agora seguem o baseline atual guiado por oráculo;
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
