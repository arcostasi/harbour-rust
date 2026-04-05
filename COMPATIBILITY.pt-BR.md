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
| Runtime | baseline alpha amplo | valores centrais, arrays, builtins selecionados de string/matemática/conversão, cobertura de edge cases de strings guiada por oráculo para trim, busca, recorte e replicação, e limites de overflow de string ao estilo Clipper em `Replicate()`/`Space()` |
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
- edge cases históricos devem ser tratados como não suportados até serem testados e documentados.

## Política de Oráculo

- `harbour-core` é o principal oráculo de comportamento.
- Testes, fixtures e saída observada têm prioridade sobre suposições.
- O código-fonte do upstream pode ajudar no entendimento, mas a implementação precisa ser original.

## Política de Dialeto

- Comportamento Clipper-first é preferido quando há sobreposição.
- Extensões específicas de Harbour devem ser explícitas e documentadas.
- Toda divergência intencional precisa ser registrada em testes e documentação.
