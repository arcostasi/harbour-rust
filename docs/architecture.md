# Arquitetura

## Meta

Entregar um compilador compatível com CA-Clipper/Harbour em Rust sem copiar a estrutura monolítica do upstream.

## Mapa upstream -> harbour-rust

| Upstream | Papel | Destino em `harbour-rust` |
| --- | --- | --- |
| `src/compiler/harbour.y` | gramática, precedência, construções | `harbour-rust-parser`, `harbour-rust-ast` |
| `src/compiler/genc.c` | geração de C | `harbour-rust-codegen-c` |
| `include/hbpcode.h` | modelo histórico de execução | referência para `harbour-rust-ir` |
| `src/pp/ppcore.c` e `doc/pp.txt` | pré-processador compatível | `harbour-rust-pp` |
| `src/vm` e `doc/vm.txt` | runtime/semântica de execução | `harbour-rust-runtime` |
| `src/rtl` | builtins e funções padrão | `harbour-rust-runtime` |
| `src/rdd` | DBF/RDD | `harbour-rust-rdd` |
| `tests`, `tests/hbpp`, `utils/hbtest` | compatibilidade e regressão | `harbour-rust-compat`, `harbour-rust-tests` |

## Pipeline recomendado

1. `source`
2. `preprocessor`
3. `lexer`
4. `parser`
5. `AST`
6. `HIR`
7. `sema`
8. `IR`
9. `codegen-c`
10. compilador C host
11. binário final

## Crates

| Crate | Responsabilidade |
| --- | --- |
| `harbour-rust-cli` | CLI `check`, `build`, `run`, `transpile` |
| `harbour-rust-lexer` | tokens, spans, trivia, erros léxicos |
| `harbour-rust-pp` | diretivas e expansão token-based |
| `harbour-rust-parser` | parser recursivo + Pratt parser de expressões |
| `harbour-rust-ast` | AST concreta e estável |
| `harbour-rust-hir` | lowering semanticamente útil |
| `harbour-rust-sema` | escopos, resolução, builtins, diagnósticos semânticos |
| `harbour-rust-ir` | IR própria, mais simples que pcode |
| `harbour-rust-codegen-c` | C legível e depurável |
| `harbour-rust-runtime` | `Value`, ambiente, builtins, helpers |
| `harbour-rust-rdd` | DBF/RDD |
| `harbour-rust-compat` | harness de comparação com Harbour |
| `harbour-rust-tests` | fixtures, snapshots, golden tests |

## Decisões arquiteturais

### 1. Não espelhar pcode primeiro

O upstream é fortemente centrado em VM/pcode. Para `harbour-rust`, o caminho inicial será:

- AST/HIR própria,
- IR pequena,
- codegen C,
- runtime em Rust consumido pelo C gerado quando necessário.

O `include/hbpcode.h` e os docs de VM servem como referência semântica, não como contrato obrigatório da primeira versão.

### 1.1. HIR inicial pequena e explícita

Na Fase 3, a HIR começa como lowering direto da AST procedural já suportada:

- rotinas, `LOCAL`, `RETURN`, `IF`, `DO WHILE`, `FOR` e `?`,
- identificadores normalizados como símbolos,
- atribuição já restringida a alvo nominal simples,
- sem resolução de escopo ainda.

Isso mantém a HIR útil para a semântica sem antecipar tabela de símbolos ou tipagem.

### 1.2. Sema inicial com side tables

O primeiro slice de `sema` trabalha sobre a HIR sem reescrevê-la:

- tabela global de rotinas,
- tabela local por rotina para parâmetros e `LOCAL`,
- resolução básica case-insensitive de símbolos,
- diagnósticos para símbolo ausente e duplicação local.

As decisões de binding ficam em side tables para manter a HIR pequena e estável nesta fase.

### 1.3. IR inicial ainda estruturada

Na primeira slice da Fase 5, a IR começa como lowering de HIR para uma forma mais próxima do backend:

- rotinas ainda preservam estrutura de `IF`, `DO WHILE` e `FOR`,
- `?` já baixa para `BuiltinCall(QOut)`,
- atribuição em posição de statement já baixa para `Assign`,
- expressões inválidas da HIR viram erro explícito de lowering.

O objetivo desta etapa é estabilizar a superfície de lowering antes de introduzir flattening de controle de fluxo ou detalhes de codegen C.

### 1.4. Backend C começa pelo subconjunto observável

Na segunda slice da Fase 5, `harbour-rust-codegen-c` começa emitindo C legível para:

- rotinas procedurais,
- `RETURN`,
- `?` já baixado como `BuiltinCall(QOut)`,
- wrapper `main()` quando houver `PROCEDURE Main()`.

Controle de fluxo estruturado e expressões mais ricas ainda produzem diagnóstico explícito de codegen nesta etapa, em vez de expansão parcial silenciosa.

Na slice seguinte, o backend C passa a cobrir o primeiro loop executável do projeto:

- `LOCAL` com inicializador inteiro,
- `DO WHILE` emitido como `while`,
- comparação `<` via helper de runtime,
- `x++` em condição via helper de postfix increment.

Na slice seguinte, o backend C passa a cobrir o primeiro `FOR` executável:

- inicialização explícita da variável de loop,
- condição `<=` via helper de runtime,
- atualização do índice por passo implícito `1`,
- atribuição `sum := sum + n` via helper de soma.

### 2. Parser manual, não porta de Bison

`harbour.y` é útil como oráculo de:

- tokens,
- precedência,
- construções,
- corner cases.

Mas a implementação em Rust deve privilegiar:

- diagnósticos melhores,
- recuperação de erro mais clara,
- módulos menores,
- testes direcionados por produção.

### 3. Pré-processador token-based

`doc/pp.txt` deixa claro que compatibilidade real com Clipper depende de operar em tokens, não apenas em texto. Portanto:

- o PP precisa ter seu próprio tokenizer,
- spans do PP devem ser preservados para diagnóstico,
- `#define` e `#include` entram antes de `#command` e `#translate`.

### 4. Compatibilidade incremental por dialeto

- baseline inicial: subconjunto Clipper procedural,
- extensões Harbour atrás de flags,
- divergências explícitas e testadas.

## Política de derivação e licença

O upstream contém código GPL/GPL+exception e documentação sob licenças variadas. Este projeto deve:

- usar comportamento, docs e testes como referência,
- evitar copiar blocos grandes de implementação,
- registrar a proveniência de fixtures importadas,
- tratar qualquer reaproveitamento literal de código com revisão específica.

Isto é uma regra de engenharia e higiene de projeto, não parecer jurídico.
