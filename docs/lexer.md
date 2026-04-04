# Lexer

> Nota de transição: a versão pública bilíngue deste conteúdo está sendo migrada para [docs/en/technical/lexer.md](./en/technical/lexer.md) e [docs/pt-BR/technical/lexer.md](./pt-BR/technical/lexer.md).

## Responsabilidade

Tokenizar fonte `.prg` produzindo um stream de tokens com spans, posições (linha/coluna) e diagnósticos precisos.

**Crate:** `harbour-rust-lexer`

## Referências upstream

- `harbour-core/src/compiler/harbour.y` — tokens e keywords
- `harbour-core/src/pp/ppcore.c` — tokens que o PP espera
- `harbour-core/doc/pp.txt` — interação léxica com PP

## Tokens suportados

### Keywords

Keywords são case-insensitive. A lista inclui:

- Controle de fluxo: `IF`, `ELSE`, `ELSEIF`, `ENDIF`, `DO`, `WHILE`, `ENDDO`, `FOR`, `TO`, `STEP`, `NEXT`
- Procedural: `FUNCTION`, `PROCEDURE`, `RETURN`
- Declaração: `LOCAL`, `STATIC`, `PRIVATE`, `PUBLIC`
- Lógicos: `AND`, `OR`, `NOT`, `.T.`, `.F.`, `NIL`
- PP: `#define`, `#include`, `#ifdef`, `#ifndef`, `#else`, `#endif`, `#command`, `#translate`

### Literais

- Inteiros: `42`, `0`
- Floats: `3.14`, `.5`
- Strings: `"texto"`, `'texto'`
- Lógicos: `.T.`, `.F.`

### Operadores

- Aritméticos: `+`, `-`, `*`, `/`, `%`, `^`, `**`
- Comparação: `=`, `==`, `!=`, `<>`, `#`, `<`, `<=`, `>`, `>=`
- Lógicos: `.AND.`, `.OR.`, `.NOT.`, `!`
- Atribuição: `:=`, `+=`, `-=`, `*=`, `/=`, `%=`, `^=`
- Pós-fixo: `++`, `--`
- String: `+` (concatenação), `$` (substring)
- Macro: `&`

### Comentários

- `//` até fim de linha
- `&&` até fim de linha
- `/* ... */` multi-linha
- `*` em início de linha (legado Clipper)

## Spans

Todo token carrega:

- `start`: offset em bytes no source
- `end`: offset em bytes no source
- Linha e coluna deriváveis

## Diagnósticos

- Token inválido com posição exata
- String não terminada
- Comentário não terminado
- Caractere inesperado

## Decisões de design

### Separação de PP

O lexer tokeniza o resultado do PP, não o source bruto. O PP tem sua própria tokenização leve para diretivas.

### Case-insensitivity

Keywords são reconhecidas de forma case-insensitive. Identificadores preservam a grafia original mas são comparáveis case-insensitive em fases posteriores.

### Sem resolução semântica

O lexer não sabe se um identificador é variável, função ou tipo. Isso é responsabilidade do parser e da sema.

## Baselines curados

| Fixture | Golden |
| --- | --- |
| `tests/fixtures/lexer/hello.prg` | `hello.tokens` |
| `tests/fixtures/lexer/while.prg` | `while.tokens` |

## Estado atual

Fase 1 concluída:

- keywords, identificadores, strings, números, comentários, operadores
- spans e posições
- diagnósticos básicos de token inválido e string não terminada
- snapshots estáveis
