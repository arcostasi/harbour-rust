# CLI

> Nota de transição: a versão pública bilíngue deste conteúdo está sendo migrada para [docs/en/technical/cli.md](./en/technical/cli.md) e [docs/pt-BR/technical/cli.md](./pt-BR/technical/cli.md).

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

### `check`

Verifica erros sem gerar código.

```bash
harbour-rust-cli check source.prg
```

Pipeline: `source -> pp -> lexer -> parser -> AST -> HIR -> sema -> diagnósticos`

### `transpile`

Gera C sem compilar.

```bash
harbour-rust-cli transpile --to c source.prg --out output.c
```

Pipeline: `source -> pp -> lexer -> parser -> AST -> HIR -> sema -> IR -> codegen-c -> .c`

### `help`

Mostra help geral ou de um comando específico.

```bash
harbour-rust-cli help
harbour-rust-cli help check
harbour-rust-cli transpile --help
```

## Opções

| Flag | Descrição |
| --- | --- |
| `--out <path>` | Caminho do arquivo de saída |
| `-I`, `--include-dir <dir>` | Diretório de includes para o PP |
| `--to c` | Target do `transpile` atual |
| `-h`, `--help` | Help geral ou help do comando |

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

Saída atual por categoria:

- `B001` para falha de leitura do source,
- `preprocess failed for <arquivo>` para falha de PP,
- `parse failed for <arquivo>` para falha sintática,
- `hir lowering failed for <arquivo>` para falha de lowering,
- `semantic analysis failed for <arquivo>` para falha de sema,
- `ir lowering failed for <arquivo>` para falha de IR,
- `codegen-c failed for <arquivo>` para falha de backend,
- `B002` quando nenhum compilador C host é encontrado,
- `B003` quando o compilador C host falha.

## Decisões de design

### Runtime support embarcado

O CLI inclui em `support/` os fontes C de runtime que são compilados junto. Isso evita dependência externa e simplifica o pipeline fim a fim.

### Erros ricos

O CLI deve reportar erros com arquivo, linha, coluna e contexto. Diagnósticos internos não devem vazar como panics.

## Estado atual

Fase 11:

- `build` — funcional para o subset atual do frontend/backend C
- `check` — funcional até `pp -> parser -> hir -> sema`
- `run` — funcional com detecção de compilador C e propagação de exit code do programa
- `transpile --to c` — funcional como wrapper explícito do caminho de geração de C
- `help`, `-h`, `--help` — funcionais no topo e por comando
- `-I/--include-dir` — funcional em `build`, `check`, `run` e `transpile`
