# Runtime

- [English](../../en/technical/runtime.md)
- [Português do Brasil](./runtime.md)

## Papel

O runtime fornece o modelo de execução usado pelo C gerado e pelos testes orientados à execução. Ele é responsável por valores, builtins, helpers de storage dinâmico e diagnósticos do lado da execução.

## Modelo Central de Valores

O baseline atual do runtime inclui:

- `Nil`
- `Logical`
- `Integer`
- `Float`
- `String`
- `Array`
- `Codeblock`

Nem todo tipo de valor de xBase já foi implementado, mas o modelo foi desenhado para ser extensível.

## Responsabilidades Principais

- armazenamento de valores com noção de tipo;
- conversões e aritmética;
- builtins de string, matemática e conversão;
- helpers de array para leitura, escrita, clone, resize e comportamento de busca;
- base para memvars e escopo dinâmico;
- formatação de saída e `QOut`;
- erros de runtime estruturados.

## Regras de Design

- sem `panic` para erros previsíveis que vêm do usuário;
- preferir helpers explícitos a mágica escondida;
- manter a semântica testável a partir de Rust e do caminho do CLI;
- documentar qualquer comportamento parcial ou leniente que ainda não reproduza exatamente runtimes históricos.

## Estado Atual

O runtime já suporta:

- um subconjunto alpha amplo de builtins;
- arrays com indexação 1-based;
- comportamento executável relacionado a `STATIC` pelo caminho do backend;
- base de contexto de memvar e avaliação de codeblocks;
- diagnósticos orientados à compatibilidade para operações selecionadas de arrays e números;
- limites de overflow de string ao estilo Clipper para `Replicate()` e `Space()`.
- leniência guiada por oráculo em `SubStr()`/`Right()` e preservação de `Chr(0)` embutido em helpers selecionados do runtime executável em C.
- padding em campo padrão para `Str()` com largura negativa explícita, com diferenças remanescentes de arredondamento guiado por largura documentadas separadamente.

## Limites Conhecidos

- ainda não existem todos os tipos históricos de valor de xBase;
- alguns builtins cobrem apenas o subconjunto de argumentos já testado;
- `Val()` agora segue o oráculo em continuações com ponto final como `1..`, `1...`, `..` e `-..`, mas a escala decimal histórica em pontuação mista e entradas parecidas com expoente ainda divergem em alguns edge cases;
- a construção de strings com `Chr(0)` embutido a partir do código-fonte ainda é limitada no caminho atual de frontend/codegen, mesmo com o runtime host C já preservando esses bytes em helpers selecionados quando eles existem;
- a formatação histórica exata ainda diverge em alguns edge cases.

## Documentos Relacionados

- [Architecture](./architecture.md)
- [CLI](./cli.md)
