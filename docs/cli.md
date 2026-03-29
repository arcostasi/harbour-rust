# CLI

## Responsabilidade

Interface de linha de comando do compilador `harbour-rust`. Orquestra o pipeline completo desde o source até o binário executável.

**Crate:** `harbour-rust-cli`

## Referências upstream

- `harbour-core/doc/cmdline.md` — opções de linha de comando do Harbour
- CLI real do `hbmk2` e `harbour`

## Comandos

### `build`

Compila um `.prg` para C.

```bash
harbour-rust build source.prg --out target/output.c
harbour-rust build source.prg -I include/ --out target/output.c
```

Pipeline: `source -> pp -> lexer -> parser -> AST -> HIR -> sema -> IR -> codegen-c -> .c`

### `run`

Compila e executa.

```bash
harbour-rust run source.prg
harbour-rust run source.prg -I include/
```

Pipeline: `build` + compilação C + execução do binário.

Detecção automática de compilador C host: `clang` > `gcc` > `cc`.

### `check` (planejado)

Verifica erros sem gerar código.

```bash
harbour-rust check source.prg
```

Pipeline: `source -> pp -> lexer -> parser -> AST -> HIR -> sema -> diagnósticos`

### `transpile` (planejado)

Gera C sem compilar.

```bash
harbour-rust transpile --to c source.prg --out output.c
```

## Opções

| Flag | Descrição |
| --- | --- |
| `--out <path>` | Caminho do arquivo de saída |
| `-I`, `--include-dir <dir>` | Diretório de includes para o PP |
| `-v`, `--verbose` | Saída detalhada |

## Pipeline interno

```text
1. Parse args
2. Read source file
3. Preprocess (PP com includes e defines)
4. Tokenize (lexer)
5. Parse (parser -> AST)
6. Lower to HIR
7. Semantic analysis (sema)
8. Lower to IR
9. Generate C (codegen-c)
10. Write .c file         [build para aqui]
11. Compile with C host   [run continua]
12. Execute binary        [run continua]
```

## Detecção de compilador C

O `run` tenta detectar um compilador C na PATH:

1. `clang`
2. `gcc`
3. `cc`

Compila o C gerado junto com os fontes de runtime support e gera um binário temporário.

## Códigos de saída

| Código | Significado |
| --- | --- |
| 0 | Sucesso |
| 1 | Erro de compilação (PP, parse, sema) |
| 2 | Erro de geração de código |
| 3 | Erro do compilador C host |
| (outro) | Código retornado pelo programa compilado |

## Decisões de design

### Runtime support embarcado

O CLI inclui em `support/` os fontes C de runtime que são compilados junto. Isso evita dependência externa e simplifica o pipeline fim a fim.

### Erros ricos

O CLI deve reportar erros com arquivo, linha, coluna e contexto. Diagnósticos internos não devem vazar como panics.

## Estado atual

Fase 5 + Fase 6:

- `build` — funcional para subconjunto procedural
- `run` — funcional com detecção de compilador C
- `-I/--include-dir` — funcional para includes do PP
- `check` — planejado para Fase 11
- `transpile` — planejado para Fase 11
