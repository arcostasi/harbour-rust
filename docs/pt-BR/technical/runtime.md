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
- `Str()` em largura default agora está alinhado para números positivos grandes e para a escala visual de literais float no caminho executável em C; além disso, o padding com largura negativa explícita e o arredondamento guiado por largura também seguem o oráculo com comportamento half-away-from-zero.
- saída executável de `Round()` com floats grandes agora preservada em decimal simples, sem colapsar para notação científica no caminho host C.

## Limites Conhecidos

- ainda não existem todos os tipos históricos de valor de xBase;
- alguns builtins cobrem apenas o subconjunto de argumentos já testado;
- `Val()` agora segue o oráculo em continuações com ponto final como `1..`, `1...`, `..` e `-..`; o subconjunto ASCII atual também já bate em sinais repetidos, paradas estilo expoente, pontuação mista como `13.1.9` e fragmentos separados por espaço após o separador decimal como `12. 0` e `12 .10`; a divergência remanescente ficou ligada à construção de `Chr(0)` embutido a partir do código-fonte no caminho atual de frontend/codegen;
- a construção de strings com `Chr(0)` embutido a partir do código-fonte ainda é limitada no caminho atual de frontend/codegen, mesmo com o runtime host C já preservando esses bytes em helpers selecionados quando eles existem;
- a formatação histórica exata ainda diverge em alguns edge cases, especialmente em algumas expressões numéricas negativas grandes com largura default.

## Documentos Relacionados

- [Architecture](./architecture.md)
- [CLI](./cli.md)
