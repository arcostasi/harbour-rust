# CLI

- [English](../../en/technical/cli.md)
- [Português do Brasil](./cli.md)

## Papel

`harbour-rust-cli` é o ponto de entrada para usuários do compilador. Ele orquestra pré-processamento, parsing, análise semântica, lowering para IR, emissão de C, compilação host e execução, conforme o comando selecionado.

## Comandos Atuais

| Comando | Propósito |
| --- | --- |
| `help` | mostrar uso geral ou específico de um comando |
| `check` | validar um arquivo-fonte pelo frontend e pela semântica |
| `build` | gerar saída em C |
| `transpile --to c` | emitir C explicitamente sem compilar |
| `run` | gerar, compilar com compilador C host e executar |

## Códigos de Saída

| Código | Significado |
| --- | --- |
| `0` | sucesso |
| `1` | erro de frontend ou uso |
| `2` | erro de geração de código |
| `3` | erro de compilador C host ou da infraestrutura de execução |
| outro | código propagado pelo programa executado em `run` |

## Prioridades de Design

- estágios previsíveis de pipeline;
- diagnósticos legíveis para usuários;
- tratamento explícito de include paths e descoberta de compilador C host;
- mínima surpresa entre `check`, `build`, `transpile` e `run`.

## Estado Atual

A CLI já cobre o fluxo alpha esperado pelo projeto:

- `help` consistente;
- `check` para após frontend e semântica;
- `build` e `transpile` emitem C;
- `run` compila o C gerado e propaga o exit code do programa;
- diretórios de include são suportados com `-I` / `--include-dir`.

## Documentos Relacionados

- [Technical Overview](./overview.md)
- [Release](./release.md)
