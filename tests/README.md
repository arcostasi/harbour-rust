# Tests

Fixtures, golden tests e testes de integração do `harbour-rust`.

Veja [docs/test-strategy.md](../docs/test-strategy.md) para a estratégia completa.

## Estrutura

```
tests/
├── fixtures/
│   ├── lexer/          # Tokens e baselines léxicos
│   │   ├── hello.prg / hello.tokens
│   │   └── while.prg / while.tokens
│   ├── parser/         # AST snapshots e fixtures de parsing
│   │   ├── hello.prg / hello.ast
│   │   ├── while.prg / while.ast
│   │   ├── static.prg / static.ast
│   │   ├── arrays.prg / arrays.ast
│   │   ├── compound_assign.prg / compound_assign.ast
│   │   ├── indexing.prg / indexing.ast
│   │   ├── indexed_assign.prg
│   │   └── for_sum.prg
│   ├── sema/           # Diagnósticos semânticos esperados
│   │   ├── control_flow_missing_locals.prg / .errors
│   │   ├── control_flow_missing_callables.prg / .errors
│   │   └── static_unsupported.errors
│   └── pp/             # Fixtures do pré-processador
│       ├── define_root.prg / .out
│       ├── include_root.prg / .out
│       ├── recursive_define_root.prg / .out
│       ├── cyclic_define_root.prg
│       ├── quoted_search_path_root.prg / .out
│       ├── angle_search_path_root.prg / .out
│       ├── shared.ch
│       └── include-path/
└── README.md
```

## Convenções

### Fixtures `.prg`

- Programas mínimos e focados em um comportamento específico.
- Nomes descritivos: `control_flow_missing_locals.prg`, não `test1.prg`.
- Um fixture por comportamento; não empacotar múltiplas features.

### Golden files

- `.tokens` — saída esperada de tokenização
- `.ast` — snapshot de AST
- `.errors` — diagnósticos esperados
- `.out` — saída esperada do PP

### Testes de integração (crates)

Testes que validam o pipeline completo ficam nos crates:

- `harbour-rust-tests` — integração geral
- `harbour-rust-compat` — comparação com harbour-core
- `harbour-rust-cli/tests/` — testes de CLI

## Como adicionar um teste

1. Crie fixture em `tests/fixtures/<camada>/`
2. Se golden: crie o arquivo `.tokens`, `.ast`, `.errors` ou `.out` correspondente
3. Adicione test function no crate relevante
4. Execute `cargo test` para validar
5. Atualize a lista de baselines em `docs/test-strategy.md`

## Fontes do upstream

Seeds curados vêm de:

- `harbour-core/tests/*.prg`
- `harbour-core/tests/hbpp/*`
- `harbour-core/utils/hbtest/rt_*.prg`
- `harbour-core/tests/rddtest/*`

Importação sempre por curadoria, nunca em massa.
