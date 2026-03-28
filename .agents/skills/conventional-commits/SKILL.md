---
name: conventional-commits
description: Organizar commits do harbour-rust com Conventional Commits, agrupando alterações por fase, tarefa e camada do compilador, e produzindo histórico limpo desde o início do projeto.
---

# Conventional Commits

## Leia antes de agir

- `AGENTS.md`
- `ROADMAP.md`
- `COMPATIBILITY.md`
- `docs/conventional-commits.md`
- `references/project-commit-conventions.md`

## Objetivo

Produzir commits pequenos, coerentes e rastreáveis por fase e por grupo de tarefa do `harbour-rust`.

## Quando usar

- ao preparar commits,
- ao propor agrupamento de mudanças em commits,
- ao revisar se um diff deve ser dividido antes de commitar,
- ao escrever mensagem de squash ou merge alinhada ao projeto.

## Fluxo

1. Leia `git status` e `git diff` antes de sugerir qualquer mensagem.
2. Separe mudanças por intenção única, não por quantidade de arquivos.
3. Nunca misture feature nova com refactor amplo no mesmo commit.
4. Escolha `type` e `scope` conforme a camada principal afetada.
5. Escreva um subject curto, direto e no imperativo.
6. Se a mudança mapear para uma fase do roadmap, inclua:
   - `Phase: <n>`
   - `Task: <slug-curto>`
7. Se testes relevantes foram executados, inclua `Tests: <resumo curto>`.
8. Se o diff ainda estiver misturado, pare e proponha a divisão antes de commitar.

## Formato preferido

```text
<type>(<scope>): <descrição curta>

[corpo opcional]

Phase: <n>
Task: <slug-curto>
Tests: <resumo curto>
```

## Regras do projeto

- Use `feat` para capacidade nova, `fix` para correção, `docs` para documentação, `test` para testes, `refactor` para reorganização sem mudança funcional, `chore` para manutenção, `ci` para pipeline, `build` para toolchain/build.
- Prefira o scope mais específico possível.
- Para bootstrap inicial e governança, use `workspace` ou `repo`.
- Para mudanças só de documentação, use `docs`.
- Para mudanças multi-camada guiadas por um único objetivo, use o scope da camada dominante; se não houver uma dominante, use `workspace`.
- Só use `!` ou `BREAKING CHANGE:` quando a quebra for real e explícita.

## Não fazer

- criar commit “misc”,
- agrupar mudanças não relacionadas para “limpar rápido”,
- inventar `scope` vago quando existe um scope de camada claro,
- commitar alterações do usuário não relacionadas,
- reescrever histórico sem pedido explícito.
