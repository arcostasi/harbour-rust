# Fluxo de commits

## Objetivo

Manter o histórico do `harbour-rust` auditável desde o primeiro commit, alinhado ao roadmap por fase, tarefa e camada.

## Regra do upstream

`harbour-core/` é um checkout local de referência e não entra no histórico principal por padrão.

Se no futuro for necessário versionar o upstream dentro deste projeto, isso deve acontecer por decisão explícita e em commit próprio, preferencialmente como submódulo ou snapshot curado.

## Sequência recomendada para o estado atual

### Commit 1

```text
chore(repo): ignore local harbour-core checkout

Phase: 0
Task: phase-0-repo-bootstrap
Tests: not run (repo hygiene only)
```

Inclui:

- `.gitignore`

### Commit 2

```text
docs(repo): add harbour-rust execution plan and architecture docs

Phase: 0
Task: phase-0-governance-docs
Tests: not run (docs only)
```

Inclui:

- `README.md`
- `ROADMAP.md`
- `COMPATIBILITY.md`
- `docs/architecture.md`
- `docs/grammar.md`
- `docs/runtime.md`
- `docs/diagnostics.md`
- `docs/test-strategy.md`
- `docs/dialect-clipper.md`
- `docs/dialect-harbour.md`

### Commit 3

```text
docs(repo): define codex workflow and commit conventions

Phase: 0
Task: phase-0-codex-workflow-docs
Tests: not run (docs only)
```

Inclui:

- `AGENTS.md`
- `docs/commit-flow.md`
- `.agents/skills/implement-parser/SKILL.md`
- `.agents/skills/add-runtime-builtin/SKILL.md`
- `.agents/skills/add-compat-test/SKILL.md`
- `.agents/skills/investigate-regression/SKILL.md`
- `.agents/skills/release-checklist/SKILL.md`
- `.agents/skills/conventional-commits/SKILL.md`
- `.agents/skills/conventional-commits/references/project-commit-conventions.md`

## Regra para fases futuras

Cada fase deve terminar com:

1. build verde,
2. testes relevantes verdes,
3. docs mínimas atualizadas,
4. commit de fechamento da fase ou do slice concluído.

## Granularidade por fase

### Fase 0

- `chore(workspace)`: bootstrap do workspace Cargo
- `ci(ci)`: pipeline inicial
- `docs(repo)`: instruções de desenvolvimento

### Fase 1

- `feat(lexer)`: keywords e operadores
- `feat(lexer)`: strings e números
- `fix(lexer)`: diagnósticos e spans
- `test(compat)`: fixtures léxicos ou baseline

### Fase 2

- `feat(ast)`: nós mínimos
- `feat(parser)`: statements procedurais
- `fix(parser)`: recuperação e erros
- `test(parser)`: snapshots e fixtures

### Fase 3

- `feat(hir)`: lowering inicial
- `feat(sema)`: resolução de símbolos
- `fix(sema)`: escopo e diagnósticos

### Fase 4

- `feat(runtime)`: `Value` e builtins mínimos
- `fix(runtime)`: semântica básica
- `test(compat)`: comparação de comportamento

### Fase 5

- `feat(ir)`: IR inicial
- `feat(codegen-c)`: backend C
- `feat(cli)`: `build`, `run`, `check`

### Fases 6 a 12

Repita a mesma disciplina:

- feature nova em commit próprio,
- refactor separado,
- testes em commit próprio quando fizer sentido,
- fechamento da fase com commit rastreável por `Phase:` e `Task:`.

## Regra de fechamento de fase

Quando a fase inteira for concluída, prefira um último commit pequeno de ajuste final, por exemplo:

```text
docs(repo): close phase 1 acceptance checklist

Phase: 1
Task: phase-1-closeout
Tests: cargo test -p harbour-rust-lexer
```

Esse commit final não substitui os commits funcionais da fase; ele apenas fecha documentação, checklist e compatibilidade.
