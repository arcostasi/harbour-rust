# Benchmarks

## Objetivo

Documentar um baseline pequeno, reproduzível e cumulativo para:

- `check`
- `transpile`
- `run`

sobre um corpus curado de fixtures.

## Runner atual

O benchmark inicial da Fase 12 está disponível via:

```text
cargo run -p harbour-rust-tests --bin benchmark-suite -- --fixture examples/hello.prg --fixture tests/fixtures/parser/phase7_acceptance.prg --iterations 3
```

O relatório sai em markdown com uma tabela por fixture.

## Baseline operacional

Nesta fase o baseline fica documentado como recorte de smoke test:

- `examples/hello.prg`
- `tests/fixtures/parser/phase7_acceptance.prg`

O objetivo inicial não é comparar números absolutos entre máquinas, e sim:

- registrar tendências,
- detectar regressões grosseiras,
- manter um caminho reproduzível para coleta de timings.

## Próximos passos

- aumentar o corpus com fixtures de PP e dinâmica xBase,
- separar métricas de frontend e backend host C,
- comparar `harbour-rust` e `harbour-core` no mesmo runner quando o toolchain upstream estiver disponível.
