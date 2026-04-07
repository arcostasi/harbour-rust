# harbour-rust — Copilot Instructions

## Projeto

Compilador em Rust 100% compatível com CA-Clipper/Harbour. Backend C, arquitetura limpa, foco em compatibilidade e desempenho.

## Antes de qualquer tarefa

1. Leia `AGENTS.md` — regras permanentes, fronteiras de arquitetura, políticas
2. Leia `ROADMAP.md` — fases, escopo, aceite
3. Leia `COMPATIBILITY.md` — matriz de compatibilidade viva
4. Leia a **doc temática** da camada afetada:
   - `docs/overview.md` — pipeline e mapa de crates
   - `docs/lexer.md`, `docs/preprocessor.md`, `docs/grammar.md`
   - `docs/hir.md`, `docs/sema.md`, `docs/ir.md`, `docs/codegen-c.md`
   - `docs/runtime.md`, `docs/cli.md`, `docs/rdd.md`
5. Leia a **skill** correspondente em `.agents/skills/<skill>/SKILL.md`
6. Se precisar consultar o upstream: `docs/upstream-navigator.md`
7. Se for pipeline completo: `docs/recipes.md`
8. Se for Fase 7: `docs/phase7-plan.md`

## Regras invioláveis

- **Nunca quebre build ou testes existentes.** Execute `cargo test` antes de terminar.
- **Uma intenção por mudança.** Parser separado de runtime, feature separada de refactor.
- **Toda feature precisa de teste.** Caso feliz + caso de erro mínimo.
- **Registre divergências.** Se o comportamento diferir do upstream, atualize `COMPATIBILITY.md`.
- **Não translitere C.** Use o upstream como referência semântica, implemente em Rust idiomático.
- **Preserve fronteiras de crate.** Cada crate tem responsabilidade clara; não cruze limites sem necessidade.

## Arquitetura de crates

```
cli ──> pp ──> lexer ──> parser ──> ast ──> hir ──> sema ──> ir ──> codegen-c
                                                                       │
                                                                    runtime
```

Cada crate é independente com testes próprios. Dependências fluem de cima para baixo.

## Padrões de código

### Rust

- `cargo fmt --all` antes de finalizar
- `cargo clippy --workspace --all-targets --all-features -- -D warnings` sem warnings
- Sem `panic!` para erros de entrada do usuário
- Erros estruturados com contexto (tipo `RuntimeError`, `ParseError`, etc.)
- Case-insensitive para keywords e identificadores (preserve grafia, compare normalizado)
- Testes em `#[cfg(test)]` ou em diretório `tests/`

### Fixtures .prg

- Mínimas, focadas em um comportamento
- Nomes descritivos em snake_case
- Comentários indicando saída esperada

### Commits

- Conventional Commits: `feat(parser):`, `fix(runtime):`, `test(compat):`, etc.
- Footers: `Phase: <n>`, `Task: <slug>`, `Tests: <resumo>`
- Um commit por grupo coerente de tarefa

## Pipeline de verificação

Antes de considerar uma tarefa concluída:

```bash
cargo fmt --all
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test
```

## Como encontrar referência no upstream

O `harbour-core/` está disponível localmente. Use `docs/upstream-navigator.md` para localizar:

- **Gramática:** `harbour-core/src/compiler/harbour.y`
- **PP:** `harbour-core/src/pp/ppcore.c` e `harbour-core/doc/pp.txt`
- **Runtime/VM:** `harbour-core/src/vm/` e `harbour-core/doc/vm.txt`
- **Builtins:** `harbour-core/src/rtl/` (237 arquivos organizados por função)
- **Testes:** `harbour-core/tests/*.prg` e `harbour-core/utils/hbtest/rt_*.prg`
- **RDD:** `harbour-core/src/rdd/`

## Estado atual

Fases 0–13 concluídas. Próximo: expansão do corpus de compatibilidade pós-Fase 13.

Pipeline funcional: `source -> pp -> lexer -> parser -> AST -> HIR -> sema -> IR -> codegen-c -> binário`.

Baseline executável: `hello.prg`, `while.prg`, `for_sum.prg`, arrays com indexação e atribuição, além de cobertura curada para markers avançados do pré-processador.
