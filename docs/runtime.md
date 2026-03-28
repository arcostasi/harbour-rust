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

## Integração com codegen C

No estágio inicial, o backend C deve gerar chamadas simples e legíveis, evitando um runtime mágico demais. O alvo é depuração fácil, não otimização.
