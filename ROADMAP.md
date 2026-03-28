# ROADMAP

## Estratégia

`harbour-rust` será construído em fases curtas, cumulativas e verificáveis. O plano toma o `harbour-core/` como mapa de referência:

- frontend: `src/compiler`, `src/pp`, `doc/pp.txt`, `doc/cmdline.md`
- execução: `include/hbpcode.h`, `doc/pcode.txt`, `doc/vm.txt`, `src/vm`
- runtime: `src/rtl`, `doc/statics.txt`, `doc/clipper.txt`
- dados: `src/rdd`, `tests/rddtest`
- validação: `tests`, `tests/hbpp`, `utils/hbtest`

## Releases

| Release | Meta | Fases |
| --- | --- | --- |
| `0.1.0-alpha` | pipeline fim a fim procedural mínimo | 0 a 5 |
| `0.2.0-alpha` | compatibilidade procedural ampliada e PP inicial | 6 a 9 parcial |
| `0.3.0-alpha` | recursos dinâmicos xBase | 8 e 9 completos |
| `0.4.0-alpha` | RDD inicial, DX e regressão industrial | 10 a 12 |

## Fase 0 — Fundação do repositório

Objetivo: preparar o workspace e a governança.

Entradas do upstream:

- `README.md`
- `doc/dirstruc.txt`
- `.github/workflows/*`

Entregáveis:

- workspace Cargo,
- crates vazios,
- `AGENTS.md`, `ROADMAP.md`, `COMPATIBILITY.md`,
- `docs/`,
- CI com `fmt`, `clippy`, `test`,
- `examples/hello.prg`.

Aceite:

- `cargo test` verde,
- `cargo clippy --all-targets --all-features` verde,
- CI verde,
- docs iniciais presentes.

## Fase 1 — Lexer

Objetivo: tokenizar `.prg` com spans e diagnósticos corretos.

Entradas do upstream:

- `src/compiler/harbour.y`
- `src/pp/ppcore.c`
- `doc/pp.txt`
- `tests/hello.prg`, `tests/keywords.prg`, `tests/while.prg`

Escopo:

- keywords básicas,
- identificadores,
- strings,
- números,
- comentários,
- operadores multi-caractere,
- separadores,
- linha/coluna/span.

Fora de escopo:

- macro operator completo,
- regras avançadas de PP,
- recuperação sintática sofisticada.

Aceite:

- tokenização determinística,
- snapshots estáveis,
- erro de token inválido com linha/coluna.

## Fase 2 — AST e parser inicial

Objetivo: aceitar o subconjunto procedural mínimo.

Entradas do upstream:

- `src/compiler/harbour.y`
- `doc/clipper.txt`
- `tests/hello.prg`, `tests/while.prg`, `tests/fornext.prg` quando curado

Escopo:

- `FUNCTION`, `PROCEDURE`,
- `LOCAL`,
- `RETURN`,
- `IF/ELSE/ENDIF`,
- `DO WHILE/ENDDO`,
- `FOR/NEXT`,
- `?`,
- expressões básicas,
- chamada de função,
- atribuição.

Aceite:

- exemplos simples parseiam,
- AST estável,
- diagnósticos bons,
- snapshots de AST.

## Fase 3 — HIR e análise semântica mínima

Objetivo: separar sintaxe de semântica.

Entradas do upstream:

- `src/compiler/*.c`
- `doc/statics.txt`
- `doc/clipper.txt`

Escopo:

- lowering AST -> HIR,
- tabela de símbolos,
- resolução de função e variável local,
- erros semânticos básicos,
- infraestrutura para builtins e dialetos.

Aceite:

- símbolo ausente detectado,
- escopo local consistente,
- HIR pequena e estável.

## Fase 4 — Runtime mínimo

Objetivo: sustentar programas procedurais simples.

Entradas do upstream:

- `src/vm`
- `src/rtl`
- `doc/vm.txt`
- `utils/hbtest/rt_*.prg`

Escopo:

- `Value`,
- `Nil`, `Logical`, `Integer`, `Float`, `String`,
- conversões básicas,
- impressão,
- comparação,
- aritmética básica.

Aceite:

- soma, comparação, print e return funcionando,
- comportamento documentado,
- casos de erro cobertos.

## Fase 5 — IR e codegen C

Objetivo: gerar executável real sem entrar cedo em backend nativo.

Entradas do upstream:

- `src/compiler/genc.c`
- `include/hbpcode.h`
- `doc/pcode.txt`

Escopo:

- IR simples independente de pcode,
- lowering HIR -> IR,
- backend C,
- integração com compilador C,
- CLI `build`, `run`, `check`.

Aceite:

- `hello.prg` compila e executa,
- programa com `FOR` simples compila e executa,
- C gerado é legível para debug.

## Fase 6 — Pré-processador inicial

Objetivo: iniciar compatibilidade textual.

Entradas do upstream:

- `src/pp/ppcore.c`
- `doc/pp.txt`
- `tests/hbpp`

Escopo:

- `#define`,
- `#include`,
- expansão textual simples,
- arquitetura para `#command` e `#translate`.

Aceite:

- include simples,
- define simples,
- pipeline `pp -> lexer -> parser` bem definido.

## Fase 7 — Compatibilidade procedural ampliada

Objetivo: cobrir o miolo útil do procedural.

Entradas do upstream:

- `src/rtl`
- `src/vm/arrays.c`
- `doc/statics.txt`
- `tests/statics*.prg`, `tests/op.prg`, `tests/arrays*`

Escopo:

- arrays,
- `STATIC`,
- operadores compostos,
- builtins essenciais,
- comparações mais fiéis,
- strings mais completas.

Aceite:

- utilitários pequenos rodam,
- regressões básicas cobertas,
- compatibilidade documentada.

## Fase 8 — Compatibilidade dinâmica xBase

Objetivo: suportar semântica dinâmica que diferencia xBase.

Entradas do upstream:

- `src/vm/memvars.c`
- `src/vm/codebloc.c`
- `src/vm/macro.c`
- `tests/memvar.prg`, `tests/macro.prg`

Escopo:

- `PRIVATE`,
- `PUBLIC`,
- memvars,
- codeblocks,
- avaliação dinâmica,
- começo do macro operator.

Aceite:

- semântica documentada,
- divergências conhecidas registradas,
- testes de regressão cobrindo os casos escolhidos.

## Fase 9 — Pré-processador avançado

Objetivo: cobrir `#command` e `#translate`.

Entradas do upstream:

- `src/pp/ppcore.c`
- `doc/pp.txt`
- `tests/hbpp/_pp_test.prg`

Escopo:

- parsing das regras,
- expansão parametrizada,
- corner cases mais relevantes,
- diagnósticos do PP.

Aceite:

- corpus curado processa corretamente,
- regressões do frontend não quebram.

## Fase 10 — DBF/RDD inicial

Objetivo: iniciar utilidade prática para sistemas legados.

Entradas do upstream:

- `src/rdd`
- `tests/rdd.prg`
- `tests/rddtest`

Escopo:

- abstração `RDD`,
- abrir tabela,
- navegar registros,
- leitura/escrita básica.

Aceite:

- abrir, ler e gravar DBF básico,
- API clara para extensão futura.

## Fase 11 — Diagnósticos, CLI e DX

Objetivo: transformar o compilador em ferramenta utilizável.

Entradas do upstream:

- `doc/cmdline.md`
- CLI atual do `harbour-core`

Escopo:

- help consistente,
- `check`,
- `transpile --to c`,
- melhores mensagens,
- códigos de saída coerentes.

Aceite:

- UX previsível,
- exemplos reproduzíveis,
- docs de uso atualizadas.

## Fase 12 — Qualidade industrial

Objetivo: blindar regressão e preparar releases.

Entradas do upstream:

- `tests`
- `tests/hbpp`
- `utils/hbtest`

Escopo:

- golden tests,
- comparador automático com Harbour,
- fuzzing,
- benchmarks,
- pipeline de release.

Aceite:

- regressão automatizada robusta,
- baseline de performance documentada,
- release checklist operacional.

## Ordem de execução recomendada

1. Fase 0
2. Fase 1
3. Fase 2
4. Fase 3
5. Fase 4
6. Fase 5
7. congelar `0.1.0-alpha`
8. Fase 6
9. Fase 7
10. Fase 8
11. Fase 9
12. congelar `0.2.0-alpha`
13. Fase 10
14. Fase 11
15. Fase 12

## Primeiro pacote de tasks

1. Criar a Fase 0 inteira.
2. Implementar lexer com testes.
3. Implementar AST + parser de `FUNCTION`, `LOCAL`, `RETURN`, `?`.
4. Implementar runtime mínimo.
5. Implementar backend C para `hello.prg`.
6. Adicionar `harbour-rust-cli run`.
