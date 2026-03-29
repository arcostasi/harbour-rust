# AGENTS

## Objetivo do projeto

Construir o `harbour-rust`, um compilador em Rust compatível com CA-Clipper/Harbour, com foco inicial em compatibilidade procedural, backend C e validação contínua contra `harbour-core/`.

## Regras permanentes

- Sempre comece lendo `README.md`, `ROADMAP.md`, `COMPATIBILITY.md` e a doc temática relevante.
- Não implemente escopo extra.
- Não misture refactor amplo com feature nova.
- Preserve build, lint e testes verdes.
- Toda feature nova exige:
  - teste unitário,
  - teste de integração,
  - teste de compatibilidade quando aplicável,
  - atualização mínima de documentação.
- Toda divergência de comportamento precisa ser documentada.
- Nunca quebre fases anteriores sem atualizar roadmap, compatibilidade e testes.

## Fronteiras de arquitetura

- `lexer` separado de `pp`.
- `pp` separado de `parser`.
- `ast` separado de `hir`.
- `sema` separado de `runtime`.
- `ir` separado de `codegen`.
- Backend inicial em C.
- Backend nativo só depois da estabilidade do frontend e do runtime.

## Regra de compatibilidade

- O oráculo principal é o comportamento do `harbour-core/`.
- Clipper primeiro, Harbour depois.
- Extensões Harbour entram com flag explícita de dialeto.
- Compatibilidade vale mais que elegância interna nas fases iniciais.

## Regra de teste

- Toda alteração relevante deve incluir caso feliz e caso de erro.
- Testes devem preferir fixtures `.prg` pequenas e legíveis.
- Sempre que viável, compare saída de `harbour-rust` com `harbour-core`.
- Bugs corrigidos viram testes de regressão.

## Definition of Done por PR

- compila,
- testes verdes,
- docs atualizadas,
- não quebra API sem justificativa,
- inclui caso feliz,
- inclui caso de erro,
- registra impacto na fase atual,
- mantém a mudança pequena e reversível.

## Política de PR

- Uma intenção por PR.
- Sem “faça tudo”.
- Mudanças em parser, sema, runtime e codegen devem ficar em PRs separados, salvo dependência mínima inevitável.

## Política de commit

- Use Conventional Commits.
- Um commit por grupo coerente de tarefa.
- Não misture feature, refactor e correção no mesmo commit.
- Prefira `scope` alinhado à camada afetada: `workspace`, `lexer`, `pp`, `parser`, `ast`, `hir`, `sema`, `runtime`, `ir`, `codegen-c`, `cli`, `compat`, `tests`, `rdd`, `docs`, `ci`, `release`.
- Sempre que a mudança se alinhar ao roadmap, inclua os footers:
  - `Phase: <n>`
  - `Task: <slug-curto>`
- Se houver teste executado relevante, inclua `Tests: <resumo curto>`.
- Ao concluir uma fase ou slice verificável, faça um commit de fechamento pequeno com docs e aceite atualizados.

## Política de derivação do upstream

- Use o `harbour-core/` como referência de comportamento, arquitetura e teste.
- Não translitere grandes blocos de C.
- Prefira reexpressar regras em Rust com testes cobrindo a mesma semântica.

## Documentação

A documentação do projeto está organizada em:

- `docs/overview.md` — visão geral, mapa de crates e pipeline de compilação
- `docs/architecture.md` — decisões arquiteturais e mapeamento do upstream
- Docs por camada: `lexer.md`, `preprocessor.md`, `grammar.md`, `hir.md`, `sema.md`, `ir.md`, `codegen-c.md`, `runtime.md`, `cli.md`, `rdd.md`
- `docs/diagnostics.md` — estrutura, categorias e códigos de mensagens de erro
- `docs/test-strategy.md` — estratégia de testes e baselines
- `docs/dialect-clipper.md` / `docs/dialect-harbour.md` — regras de compatibilidade por dialeto
- `docs/upstream-navigator.md` — guia para encontrar referências no harbour-core
- `docs/recipes.md` — receitas passo a passo para operações comuns no compilador
- `docs/phase7-plan.md` — plano detalhado da Fase 7 com slices, fixtures e aceite
- `docs/commit-flow.md` / `docs/conventional-commits.md` — convenções de commit

Ao trabalhar em qualquer camada, leia primeiro a doc temática correspondente.

## Skills disponíveis

| Skill | Quando usar |
| --- | --- |
| `implement-lexer` | Adicionar tokens, spans ou diagnósticos léxicos |
| `implement-preprocessor` | Estender PP (diretivas, expansão) |
| `implement-parser` | Estender parser/AST |
| `implement-hir-sema` | Lowering HIR ou análise semântica |
| `implement-ir-codegen` | IR lowering ou geração de código C |
| `add-runtime-builtin` | Adicionar builtin ao runtime |
| `implement-cli` | Comandos, opções ou pipeline do CLI |
| `implement-rdd` | Acesso a dados DBF/RDD |
| `add-compat-test` | Teste de compatibilidade Clipper/Harbour |
| `investigate-regression` | Investigar e corrigir regressão |
| `release-checklist` | Preparar release |
| `conventional-commits` | Organizar commits |
