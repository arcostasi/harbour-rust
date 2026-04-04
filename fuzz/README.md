# Fuzzing

Skeleton inicial de fuzzing da Fase 12.

## Targets

- `lexer`
- `parser`
- `pp`

## Como usar

```text
cargo install cargo-fuzz
cargo fuzz run lexer
cargo fuzz run parser
cargo fuzz run pp
```

Os targets aceitam qualquer entrada UTF-8 e exercitam as camadas correspondentes sem assumir sucesso semântico.
