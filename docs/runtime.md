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

Na slice seguinte da Fase 7, entram `Upper()` e `Lower()` como builtins imutáveis de string:

- `upper()` cobre o baseline inicial ASCII de `Upper( cText )`,
- `lower()` cobre o baseline inicial ASCII de `Lower( cText )`,
- ambos preservam bytes não-ASCII no recorte atual e transformam apenas letras ASCII,
- `Upper()` agora emite `BASE 1102 Argument error (UPPER)` para argumentos inválidos,
- `Lower()` agora emite `BASE 1103 Argument error (LOWER)` para argumentos inválidos,
- nesta fase ambos continuam parciais: `Chr(0)`, codepage multibyte e by-ref observados no upstream continuam pendentes.

Na slice seguinte da Fase 7, entram `Trim()`, `LTrim()` e `RTrim()` como builtins imutáveis de string:

- `trim()` segue o baseline histórico de sinônimo de `rtrim()`,
- `rtrim()` remove apenas espaços `' '` à direita no recorte atual,
- `ltrim()` remove whitespace ASCII à esquerda no recorte atual,
- `Trim()` e `RTrim()` agora emitem `BASE 1100 Argument error (TRIM)` para argumentos inválidos,
- `LTrim()` agora emite `BASE 1101 Argument error (LTRIM)` para argumentos inválidos,
- nesta fase os três continuam parciais: `Chr(0)`, regras completas de whitespace/codepage e by-ref observados no upstream continuam pendentes.

Na slice seguinte da Fase 7, entra `At()` como builtin imutável de busca em string:

- `at()` cobre o recorte clássico de `At( cNeedle, cHaystack )` com retorno 1-based,
- substring ausente retorna `0`,
- string vazia também retorna `0` no baseline Harbour atual usado como oráculo,
- argumentos inválidos agora emitem `BASE 1108 Argument error (AT)`,
- nesta fase o builtin continua parcial: `hb_AT()` com `start/to`, codepage multibyte e as divergências históricas do otimizador Clipper para string vazia continuam pendentes.

Na slice seguinte da Fase 7, entram `Replicate()` e `Space()` como builtins imutáveis de construção de string:

- `replicate()` cobre o recorte inicial de `Replicate( cText, nCount )`,
- `space()` cobre o recorte inicial de `Space( nCount )`,
- `nCount` aceita `Integer` e `Float`, com truncamento para zero casas decimais no baseline atual,
- valores `<= 0` retornam string vazia,
- `Replicate()` agora emite `BASE 1106 Argument error (REPLICATE)` para argumentos inválidos,
- `Space()` agora emite `BASE 1105 Argument error (SPACE)` para argumentos inválidos,
- nesta fase ambos continuam parciais: overflow completo do upstream, `Chr(0)` em `Replicate()` e codepage multibyte no host C continuam pendentes.

Na slice seguinte da Fase 7, entra `Str()` como builtin imutável de conversão numérica para string:

- `str_value()` cobre o recorte inicial de `Str( nValue, [nWidth], [nDecimals] )` para `Integer` e `Float`,
- sem largura explícita o baseline atual usa largura mínima 10, mas deixa o texto crescer quando necessário,
- com largura explícita e sem decimais o recorte atual arredonda para inteiro, alinhado aos casos básicos do upstream usados nesta slice,
- com largura + decimais explícitos o builtin usa formatação fixa e retorna `*****` quando o resultado não cabe na largura pedida,
- argumentos inválidos agora emitem `BASE 1099 Argument error (STR)`,
- nesta fase o builtin continua parcial: a precisão histórica derivada de escala original do número, larguras negativas e corner cases mais profundos do upstream continuam pendentes.

Na slice seguinte da Fase 7, entra `Val()` como builtin imutável de conversão string para número:

- `val()` cobre o recorte inicial de `Val( cText )` para `String`,
- o baseline atual ignora whitespace ASCII à esquerda,
- aceita sinal simples no início,
- reconhece inteiro e decimal básico com lixo à direita ignorado,
- retorna `0` quando a string não começa com número reconhecível,
- argumentos inválidos agora emitem `BASE 1098 Argument error (VAL)`,
- nesta fase o builtin continua parcial: exponentes, `Chr(0)`, pontos repetidos e corner cases mais profundos observados em `rt_str.prg` continuam pendentes.

Na slice seguinte da Fase 7, entra `Abs()` como builtin imutável de valor absoluto:

- `abs()` cobre o recorte inicial de `Abs( nValue )` para `Integer` e `Float`,
- números inteiros permanecem inteiros quando o resultado cabe nesse formato,
- números de ponto flutuante usam o valor absoluto padrão,
- argumentos inválidos agora emitem `BASE 1089 Argument error (ABS)`,
- nesta fase o builtin continua parcial: by-ref, handlers matemáticos do upstream e corner cases extremos fora do recorte atual continuam pendentes.

Na slice seguinte da Fase 7, entra `Sqrt()` como builtin imutável de raiz quadrada:

- `sqrt_value()` cobre o recorte inicial de `Sqrt( nValue )` para `Integer` e `Float`,
- valores `<= 0` retornam `0`, alinhado ao baseline observado em `rt_math.prg`,
- valores positivos usam `sqrt()` padrão sobre `f64`,
- argumentos inválidos agora emitem `BASE 1097 Argument error (SQRT)`,
- nesta fase o builtin continua parcial: handlers matemáticos do upstream, escala histórica fina e corner cases mais profundos permanecem pendentes.

Na slice seguinte da Fase 7, entra `Log()` como builtin imutável de logaritmo natural:

- `log_value()` cobre o recorte inicial de `Log( nValue )` para `Integer` e `Float`,
- valores `<= 0` retornam `-infinity` no runtime numérico e o caminho `Str( Log(...) )` agora materializa placeholder de overflow com `*`, alinhado ao baseline observado em `rt_math.prg`,
- valores positivos usam `ln()` padrão sobre `f64`,
- argumentos inválidos agora emitem `BASE 1095 Argument error (LOG)`,
- nesta fase o builtin continua parcial: handlers matemáticos do upstream, substituição de erro histórica completa e corner cases mais profundos permanecem pendentes.

Na slice seguinte da Fase 7, entra `Exp()` como builtin imutável de exponencial:

- `exp_value()` cobre o recorte inicial de `Exp( nValue )` para `Integer` e `Float`,
- o baseline atual usa `exp()` padrão sobre `f64`,
- overflow numérico permanece como `+infinity` no runtime numérico e o caminho `Str( Exp(...) )` já reaproveita o placeholder de overflow com `*`,
- argumentos inválidos agora emitem `BASE 1096 Argument error (EXP)`,
- nesta fase o builtin continua parcial: `Str( Exp(...) )` ainda herda a largura/escala simplificada do `Str()` atual e por isso diverge do `harbour-core` em casos como `Str( Exp( 15 ) )`; handlers matemáticos do upstream, substituição de erro histórica completa e corner cases mais profundos permanecem pendentes.

Na slice seguinte da Fase 7, entram `Sin()` e `Cos()` como builtins imutáveis trigonométricos:

- `sin_value()` cobre o recorte inicial de `Sin( nValue )` para `Integer` e `Float`,
- `cos_value()` cobre o recorte inicial de `Cos( nValue )` para `Integer` e `Float`,
- ambos usam `f64::sin()` e `f64::cos()` como baseline numérico atual,
- argumentos inválidos agora emitem `BASE 1091 Argument error (SIN)` e `BASE 1091 Argument error (COS)`,
- nesta fase os dois continuam parciais: o checkout local do upstream não traz fixture direta em `utils/hbtest` para `Sin()`/`Cos()`, então o baseline atual é provisório e documentado por fixture local de compatibilidade até que um oracle melhor seja curado.

Na slice seguinte da Fase 7, entra `Int()` como builtin imutável de truncamento numérico:

- `int()` cobre o recorte inicial de `Int( nValue )` para `Integer` e `Float`,
- `Integer` permanece inteiro sem alteração,
- `Float` usa truncamento toward-zero, alinhado ao baseline observado em `rt_math.prg`,
- argumentos inválidos agora emitem `BASE 1090 Argument error (INT)`,
- nesta fase o builtin continua parcial: by-ref, overflow extremo e corner cases mais profundos do upstream continuam pendentes.

Na slice seguinte da Fase 7, entra `Round()` como builtin imutável de arredondamento numérico:

- `round_value()` cobre o recorte inicial de `Round( nValue, nDecimals )` para `Integer` e `Float`,
- `nDecimals` é obrigatório e aceita `Integer` ou `Float` truncado para inteiro,
- o baseline atual usa arredondamento half-away-from-zero e aceita decimais negativos,
- com `nDecimals <= 0` o runtime retorna `Integer` quando o resultado cabe nesse formato,
- argumentos inválidos agora emitem `BASE 1094 Argument error (ROUND)`,
- nesta fase o builtin continua parcial: escala histórica do item numérico, zeros à direita observáveis no upstream, by-ref e corner cases mais profundos continuam pendentes.

Na slice seguinte da Fase 7, entra `Mod()` como builtin imutável de resto numérico:

- `mod_value()` cobre o recorte inicial de `Mod( nValue, nBase )` para `Integer` e `Float`,
- o baseline atual ajusta o resto ao sinal do divisor, alinhado ao comportamento observado em `src/rtl/mod.c` e `rt_math.prg`,
- argumentos extras continuam ignorados na surface do builtin, como no upstream,
- argumentos inválidos agora emitem `BASE 1085 Argument error (%)`,
- divisor zero agora emite `BASE 1341 Zero divisor (%)`,
- nesta fase o builtin continua parcial: preservação histórica exata da representação numérica e do item original quando a substituição de erro ocorre no upstream continuam pendentes.

Na slice seguinte da Fase 7, entra `ValType()` como builtin imutável de introspecção leve:

- `valtype()` cobre o recorte inicial dos tipos já materializados no runtime atual,
- `Nil` e ausência de argumento retornam `"U"`,
- `Logical` retorna `"L"`,
- `Integer` e `Float` retornam `"N"`,
- `String` retorna `"C"`,
- `Array` retorna `"A"`,
- nesta fase o builtin continua parcial: `Date`, `Object`, `Codeblock`, `Memo`, `Hash` e outros tipos ainda não existem no runtime, então seus códigos permanecem pendentes.

Na slice seguinte da Fase 7, entra `Type()` como builtin imutável de introspecção textual:

- `type_value()` exige argumento `String` e agora emite `BASE 1121 Argument error (TYPE)` para ausência de argumento ou tipo inválido,
- o recorte atual interpreta o texto da string como origem de expressão apenas em um subconjunto pequeno e explícito,
- `NIL` retorna `"U"`,
- `.T.` e `.F.` retornam `"L"`,
- números ASCII simples retornam `"N"`,
- literais quoted (`'abc'`, `"abc"`) retornam `"C"`,
- literais `{ ... }` retornam `"A"`,
- nomes textuais não resolvidos retornam `"U"`,
- nesta fase o builtin continua parcial: macro evaluation completa, resolução de nomes, `Date`, `Object`, `Codeblock`, `Memo`, `Hash` e os demais tipos do upstream continuam pendentes.

Na slice seguinte da Fase 7, entram `Max()` e `Min()` como builtins imutáveis de comparação leve:

- `max_value()` cobre o recorte inicial de `Max( xLeft, xRight )` para `Integer`, `Float` e `Logical`,
- `min_value()` cobre o mesmo recorte para `Min( xLeft, xRight )`,
- comparações numéricas mistas usam promoção para `Float`, mas o valor retornado preserva o item original vencedor,
- em empate o baseline atual preserva o primeiro argumento, alinhado ao comportamento documentado no upstream,
- `Max()` agora emite `BASE 1093 Argument error (MAX)` para argumentos inválidos,
- `Min()` agora emite `BASE 1092 Argument error (MIN)` para argumentos inválidos,
- nesta fase ambos continuam parciais: `Date`, `DateTime`, by-ref e demais tipos suportados pelo upstream permanecem pendentes.

Na slice seguinte da Fase 7, entra `Empty()` como builtin imutável de emptiness em estilo xBase:

- `empty()` segue o baseline leniente do upstream e não emite erro para os tipos hoje materializados no runtime,
- `Nil` e ausência de argumento retornam `.T.`,
- `Logical` retorna o inverso do valor (`.F.` é vazio, `.T.` não é),
- `Integer` e `Float` retornam `.T.` apenas quando o valor observado é zero,
- `String` retorna `.T.` apenas quando contém whitespace ASCII e nenhum outro byte observável,
- `Array` retorna `.T.` apenas quando vazia,
- nesta fase o builtin continua parcial: `Date`, `Object`, `Codeblock`, `Memo`, `Hash`, pointers e o caminho host C com `Chr(0)` embutido permanecem pendentes.

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
