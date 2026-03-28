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
