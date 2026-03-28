# Contributing

## Fluxo local

1. Leia `AGENTS.md`, `ROADMAP.md` e `COMPATIBILITY.md`.
2. Escolha uma task pequena alinhada a uma fase.
3. Limite o diff a uma intenção por vez.
4. Execute:
   - `cargo fmt --all`
   - `cargo clippy --workspace --all-targets --all-features -- -D warnings`
   - `cargo test`
5. Atualize a documentação mínima necessária.
6. Faça commit com Conventional Commits e os footers:
   - `Phase: <n>`
   - `Task: <slug-curto>`
   - `Tests: <resumo curto>`

## Regras

- não misture refactor amplo com feature,
- preserve build verde,
- adicione testes antes de chamar uma feature de concluída,
- registre divergências em `COMPATIBILITY.md`.
