# Runtime

## Objetivo

Fornecer semântica suficiente para executar o subconjunto inicial sem comprometer a evolução para recursos dinâmicos de xBase.

Referências principais:

- `harbour-core/src/vm`
- `harbour-core/src/rtl`
- `harbour-core/doc/vm.txt`
- `harbour-core/doc/statics.txt`
- `harbour-core/utils/hbtest`

## Modelo inicial de valor

```rust
enum Value {
    Nil,
    Logical(bool),
    Integer(i64),
    Float(f64),
    String(String),
    Array(Vec<Value>),
    Codeblock(CodeblockId),
}
```

O conjunto acima é incremental. `Array` e `Codeblock` entram em fases posteriores, mas o enum deve ser desenhado para isso desde o início.

Na primeira slice da Fase 4, o runtime cobre:

- `Value::{Nil, Logical, Integer, Float, String}`,
- `ValueKind` para diagnóstico e dispatch leve,
- conversões estritas por tipo,
- promoção de `Integer` para `Float`,
- formatação básica de saída para `NIL`, `.T.`, `.F.`, números e strings.

Na segunda slice da Fase 4, entram operações básicas:

- `+`, `-`, `*`, `/` para números,
- concatenação `String + String`,
- comparações `=`, `<>`, `<`, `<=`, `>`, `>=` para números,
- igualdade e ordenação léxica básica para strings,
- erro estruturado para combinações ainda não suportadas e divisão por zero.

Na terceira slice da Fase 4, entram saída mínima e builtin inicial:

- `to_print_string()` como formatter básico orientado a `QOut`,
- `OutputBuffer` simples para testes e integração inicial,
- `qout()` retornando `NIL`,
- emissão de linha única com argumentos separados por espaço,
- linha em branco quando chamado sem argumentos.

Na quarta slice da Fase 4, entra o dispatch mínimo de builtins de impressão:

- `RuntimeContext` com saída explícita e testável,
- `Builtin::lookup()` case-insensitive para `QOut`,
- `call_builtin()` como superfície inicial para integração com parser/codegen,
- erro estruturado para builtin desconhecido.

Na primeira slice de arrays da Fase 7, entra a superfície mínima de coleção:

- `ValueKind::Array` e `Value::Array(Vec<Value>)`,
- `Value::array(...)`, `Value::empty_array()` e `Value::array_with_len(...)`,
- acesso estrito com `as_array()` e `TryFrom<&Value> for Vec<Value>`,
- formatter basal `"{ Array(n) }"` para tornar snapshots e diagnósticos previsíveis.

Na slice seguinte da Fase 7, entram helpers mínimos de indexação:

- `array_len()` para expor o tamanho do contêiner,
- `array_get()` e `array_get_owned()` com índice 1-based, alinhado ao baseline xBase,
- `array_get_path()` para navegação sequencial em indexação encadeada,
- diagnóstico estruturado para alvo não-array, índice não-inteiro e bounds inválidos.

Na slice seguinte da Fase 7, entram helpers mínimos de escrita e groundwork de atribuição:

- `as_array_mut()` e `array_get_mut()` como superfície controlada de mutação,
- `array_set()` retornando o valor atribuído para preparar semântica de assignment expression,
- `array_set_path()` para `matrix[i][j] := value` sem acoplar parser e codegen ainda,
- diagnóstico estruturado para caminho de atribuição vazio e alvo intermediário não-array.

## Ambientes

Precisaremos de pelo menos:

- frame local,
- tabela de funções,
- storage de statics,
- storage de memvars,
- builtins,
- contexto de saída/IO.

## Decisões

### Statics

Seguir o insight do upstream descrito em `doc/statics.txt`: tratar statics como storage separado do frame local.

### Memvars

Entram depois do procedural mínimo e precisam de escopo dinâmico explícito. Não simular memvars como simples globais.

### Builtins

Implementar por prioridade e sempre com teste de compatibilidade:

1. `QOut` e equivalentes mínimos de saída
2. conversões básicas
3. aritmética e comparação
4. strings
5. arrays

Nesta primeira entrada de arrays, o objetivo ainda não é semântica completa de xBase. O runtime só materializa o contêiner, seu tamanho inicial e uma surface pública pequena o bastante para parser, builtins e testes evoluírem sem inventar indexação, mutação ou comparação antes da hora.

Com a slice seguinte, o runtime passa a aceitar leitura básica de arrays, mas ainda não implementa:

- escrita por índice,
- comparação profunda de arrays,
- integração fim a fim com IR/codegen,
- mensagens completas no formato histórico de erro xBase.

Com a slice seguinte, o runtime passa a aceitar também escrita básica por índice e por caminho, mas ainda não implementa:

- atribuição indexada fim a fim no pipeline parser -> codegen,
- crescimento automático compatível com todas as variantes históricas de xBase,
- comparação profunda de arrays,
- mensagens completas no formato histórico de erro xBase.

Na slice seguinte da Fase 7, o runtime ganha helpers mais ricos de coleção e comparação exata:

- `exact_equals()` e `exact_not_equals()` como superfície explícita para a semântica de `==`,
- arrays usam identidade do valor observado, então a mesma referência retorna `.T.` e clones seguem `.F.`,
- `array_resize()`, `array_push()` e `array_clone()` preparam o terreno para `ASize()`, `AAdd()` e `AClone()`,
- a comparação comum `=`/`<>` e ordenação continuam fora da semântica de arrays nesta fase.

Na slice seguinte da Fase 7, entram os primeiros builtins de array sobre essa infraestrutura:

- `aadd()` usa `array_push()` e retorna o valor adicionado,
- `asize()` usa `array_resize()` e retorna o array ajustado,
- `call_builtin_mut()` passa a existir como surface separada para builtins mutantes,
- `call_builtin()` continua atendendo builtins imutáveis e reporta erro explícito se `AAdd` ou `ASize` forem chamados pela surface errada.

Na slice seguinte da Fase 7, entra `Len()` como builtin imutável compartilhado entre strings e arrays:

- `len()` retorna `Integer` para `String` e `Array`,
- `Len(NIL)` e `Len(123)` agora produzem `BASE 1111 Argument error (LEN)` no baseline atual,
- `call_builtin()` e `call_builtin_mut()` passam a despachar `LEN` de forma case-insensitive,
- nesta fase o builtin ainda não cobre hashes, objetos nem semântica de codepage multibyte observada no upstream completo.

Na slice seguinte da Fase 7, entra `SubStr()` como builtin imutável de string:

- `substr()` cobre o baseline inicial de `SubStr( cText, nStart, [nCount] )` para `String`,
- `nStart` aceita `0`, positivos e negativos, com clipping alinhado ao recorte observado em `rt_str.prg`,
- `nCount <= 0` retorna string vazia, ausência de `nCount` devolve a cauda da string,
- argumentos inválidos agora produzem `BASE 1110 Argument error (SUBSTR)`,
- nesta fase o builtin continua parcial: `start/count` ainda exigem `Integer`, e codepage multibyte + `Chr(0)` no host C permanecem pendentes.

Na slice seguinte da Fase 7, entram `Left()` e `Right()` como builtins imutáveis de string:

- `left()` cobre o baseline inicial de `Left( cText, nCount )` para `String`,
- `right()` cobre o baseline inicial de `Right( cText, nCount )` para `String`,
- `nCount <= 0` retorna string vazia e valores acima do tamanho fazem clipping para a string inteira,
- `Left()` agora emite `BASE 1124 Argument error (LEFT)` para argumentos inválidos,
- `Right()` segue o recorte leniente observado em `rt_str.prg` e retorna string vazia para argumentos inválidos,
- nesta fase ambos continuam parciais: `count` ainda exige `Integer`, e codepage multibyte + `Chr(0)` no host C permanecem pendentes.

Na slice seguinte da Fase 7, entra `AClone()` como builtin imutável de array:

- `aclone()` usa `array_clone()` e retorna cópia estrutural do array,
- `AClone()` permanece na surface imutável `call_builtin()`,
- `AClone(NIL)` e argumentos não-array retornam `NIL` no baseline atual,
- a slice evita introduzir ainda semântica mais ampla de cópia para tipos complexos além de `Array`.

Na slice seguinte da Fase 7, os diagnósticos de acesso e atualização de arrays ficam mais próximos do baseline xBase:

- leitura usa mensagens/códigos alinhados a `array access` (`1068` e `1132`),
- escrita usa mensagens/códigos alinhados a `array assign` (`1069` e `1133`),
- o runtime continua estruturado em `RuntimeError`, mas a mensagem primária já preserva o código estável esperado,
- erros genéricos de conversão continuam reservados para APIs que não representam acesso/atribuição indexada.

Na slice seguinte da Fase 7, comparações de arrays ficam mais próximas do baseline xBase:

- `==` continua modelado por identidade observável do valor, então a mesma referência retorna `.T.` e clones seguem `.F.`,
- `=` e `<>` deixam de cair no mismatch genérico e passam a emitir `BASE 1071` e `BASE 1072`,
- `exact_not_equals()` continua sendo a negação da surface exata atual, preservando a API interna usada pelos testes,
- `<`, `<=`, `>` e `>=` com arrays passam a emitir `BASE 1073` a `BASE 1076`,
- a slice continua restrita a arrays; semântica equivalente para objetos e codeblocks permanece fora do escopo atual.

### Erros de runtime

- nada de `panic!` para erro de usuário,
- usar tipo de erro estruturado,
- superfície amigável para CLI e testes.

Na base inicial, erros de conversão usam `RuntimeError` com mensagem e tipo real encontrado.

## Integração com codegen C

No estágio inicial, o backend C deve gerar chamadas simples e legíveis, evitando um runtime mágico demais. O alvo é depuração fácil, não otimização.

## Fechamento da Fase 4

Com a quarta slice, a Fase 4 fecha no nível do crate de runtime com:

- modelo básico de `Value`,
- conversões públicas e diagnósticos estruturados,
- aritmética e comparação para o subconjunto procedural inicial,
- formatação de saída,
- `QOut()` mínimo,
- dispatch de builtin de impressão por nome.

Continua pendente para a Fase 5 a integração fim a fim entre parser, HIR, IR, backend C e execução observável de `RETURN` e `?`.
