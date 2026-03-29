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

Na Fase 7, `STATIC` entra nessa mesma superfície como placeholder explícito:

- o lowering preserva `storage_class` em declarações lowered,
- `STATIC` não é mais confundido silenciosamente com `LOCAL`,
- a semântica ainda não implementa storage persistente, mas já emite diagnóstico explícito e mantém os símbolos declarados para evitar falsos `unresolved`.

Na slice seguinte da Fase 7, arrays deixam de morrer cedo no lowering:

- literais de array entram como nó explícito da HIR,
- a semântica passa a caminhar pelos elementos para preservar resolução e regressões,
- a IR agora também preserva literais de array como nós explícitos,
- o gargalo remanescente deixa de estar no lowering AST -> HIR -> IR e passa a se concentrar na semântica executável mais completa de arrays.

Na slice seguinte da Fase 7, operadores compostos passam a ter superfície estável no lowering:

- o parser dessuga `+= -= *= /= %= ^=` para `Assignment + Binary`,
- a HIR preserva esse shape sem introduzir nó especial ainda,
- fixtures com `LOCAL` e `STATIC` agora validam explicitamente esse lowering.

Na slice seguinte da Fase 7, indexação de array deixa de morrer no AST -> HIR:

- `expr[expr]`, `expr[expr, ...]` e encadeamento entram como `Index(target, indices)` na HIR,
- a semântica passa a caminhar por `target` e por cada índice,
- a IR agora também preserva `Index(target, indices)` de forma explícita,
- escrita por índice e semântica mais completa de arrays continuam como limitações separadas e diagnosticáveis.

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

Na slice seguinte da Fase 7, o backend C ganha a primeira surface de arrays sem ligar lowering ainda:

- o prelude de `codegen-c` passa a declarar helpers de array no runtime C,
- o suporte host em `runtime_support.{h,c}` ganha `Value::Array` mínima, construtor por itens, `array_len` e `array_get`,
- literais `{}` e `expr[...]` deixam de estar bloqueados na IR e passam a ficar bloqueados apenas no codegen executável, enquanto a infraestrutura do lado C deixa de ser o próximo gargalo.

Na slice seguinte da Fase 7, o backend C passa a usar essa surface explicitamente:

- literais de array baixam para `harbour_value_from_array_items(...)`,
- indexação baixa para chamadas encadeadas de `harbour_value_array_get(...)`,
- fixtures de arrays e indexação já conseguem gerar C sem erro de codegen,
- escrita por índice e semântica mais completa de arrays continuam pendentes.

Na slice seguinte da Fase 7, a escrita por índice entra no caminho executável inicial:

- alvos como `matrix[2][1] := 99` baixam para `AssignTarget::Index` em HIR e IR,
- o backend C emite `harbour_value_array_set_path(&matrix, indices..., assigned)`,
- o suporte host em C passa a materializar atualização 1-based por caminho,
- a leitura imediata do mesmo caminho já consegue validar a mutação no pipeline `run`.

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

Na primeira slice da Fase 6, `harbour-rust-pp` começa com uma superfície explícita de source graph:

- `SourceFile` como unidade de entrada do PP,
- resolver de includes separado da lógica principal,
- registro estruturado de `#define`,
- mapeamento de linhas de saída para o arquivo e a linha de origem,
- inclusão textual simples antes da futura expansão token-based.

Na slice seguinte, o PP passa a expandir `#define` objeto no source normal:

- expansão case-insensitive por identificador inteiro,
- sem tocar em strings e comentários de linha,
- sem expandir macros parametrizadas ainda.

Na slice seguinte, a expansão de `#define` objeto ganha recursão controlada:

- cadeias como `A -> B -> "x"` passam a resolver até o valor final,
- ciclos como `A -> B -> A` geram diagnóstico explícito,
- o erro é reportado no ponto de uso da linha preprocessada, não como panic interno.

Na slice seguinte, o resolver de includes ganha política inicial de busca:

- `#include "x.ch"` tenta primeiro o diretório do arquivo atual e depois search paths configuráveis,
- `#include <x.ch>` usa search paths configuráveis como primeira política,
- a lógica de busca continua isolada em `FileSystemIncludeResolver`.

Na slice seguinte, o `harbour-rust-cli` ganha a primeira superfície de handoff do PP:

- `build` e `run` aceitam `-I/--include-dir`,
- o CLI materializa um `preprocess -> parse` explícito antes do restante do pipeline,
- a integração continua mínima: options de PP no CLI, sem dialetos nem `#command`.

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
