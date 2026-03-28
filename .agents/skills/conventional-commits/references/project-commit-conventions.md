# Project Commit Conventions

## Base

Este projeto usa a especificação em `docs/conventional-commits.md` com uma extensão leve para rastrear evolução por fase e por tarefa.

## Footers do projeto

Use estes footers quando a mudança fizer parte do roadmap:

- `Phase: <n>`
- `Task: <slug-curto>`

Use este footer quando houver validação útil para leitura futura:

- `Tests: <resumo curto>`

Exemplos:

```text
Phase: 0
Task: repo-governance-bootstrap
Tests: not run (docs only)
```

```text
Phase: 1
Task: lexer-keywords-and-spans
Tests: cargo test -p harbour-rust-lexer
```

## Tipos recomendados

| Tipo | Quando usar |
| --- | --- |
| `feat` | nova capacidade visível ou nova etapa funcional |
| `fix` | correção de bug ou regressão |
| `docs` | documentação somente |
| `test` | testes sem mudança funcional principal |
| `refactor` | reorganização interna sem mudança observável |
| `chore` | manutenção, bootstrap, housekeeping |
| `ci` | pipeline e automação |
| `build` | toolchain, Cargo, linker, compilação |
| `perf` | melhoria de desempenho |
| `revert` | reversão explícita |

## Scopes recomendados

| Scope | Área |
| --- | --- |
| `repo` | regras globais ou metadados do repositório |
| `workspace` | bootstrap e organização do workspace Cargo |
| `lexer` | tokenização |
| `pp` | pré-processador |
| `parser` | parser |
| `ast` | AST |
| `hir` | HIR |
| `sema` | semântica e resolução |
| `runtime` | runtime |
| `ir` | IR |
| `codegen-c` | backend C |
| `cli` | interface de linha de comando |
| `compat` | harness e matriz de compatibilidade |
| `tests` | fixtures e harness de testes |
| `rdd` | DBF/RDD |
| `docs` | documentação |
| `ci` | integração contínua |
| `release` | preparação de release |

## Regras de agrupamento

### 1. Agrupe por intenção

Bom:

- um commit para bootstrap do workspace,
- um commit para a skill de parser,
- um commit para a matriz de compatibilidade.

Ruim:

- um commit único com bootstrap, parser, runtime e CI.

### 2. Separe refactor de feature

Se foi preciso limpar estrutura antes de adicionar comportamento novo:

1. commit de `refactor`
2. commit de `feat` ou `fix`

### 3. Prefira rastreabilidade de fase

Se a mudança pertence claramente a uma fase, o footer `Phase:` deve aparecer.

### 4. Registre a menor tarefa útil

`Task:` deve representar a menor fatia auditável:

- `phase-0-workspace-bootstrap`
- `phase-0-governance-docs`
- `phase-1-lexer-keywords`
- `phase-1-lexer-invalid-token-diagnostics`

## Exemplos do harbour-rust

### Governança inicial

```text
docs(repo): add harbour-rust execution plan

Phase: 0
Task: phase-0-governance-docs
Tests: not run (docs only)
```

### Bootstrap do workspace

```text
chore(workspace): bootstrap cargo workspace and empty crates

Phase: 0
Task: phase-0-workspace-bootstrap
Tests: cargo test
```

### Lexer

```text
feat(lexer): tokenize core keywords and operators

Phase: 1
Task: phase-1-lexer-keywords-and-operators
Tests: cargo test -p harbour-rust-lexer
```

### Compatibilidade

```text
test(compat): add hello and while baseline fixtures

Phase: 1
Task: phase-1-compat-baseline-fixtures
Tests: cargo test -p harbour-rust-compat
```

### Correção de regressão

```text
fix(parser): recover after missing endif

Phase: 2
Task: phase-2-parser-endif-recovery
Tests: cargo test -p harbour-rust-parser
```

## Heurística de decisão rápida

1. A mudança altera comportamento? Use `feat` ou `fix`.
2. A mudança só reorganiza? Use `refactor`.
3. A mudança só documenta? Use `docs`.
4. A mudança só adiciona teste? Use `test`.
5. A mudança só mexe em pipeline ou toolchain? Use `ci`, `build` ou `chore`.
