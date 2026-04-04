# Análise Semântica

- [English](../../en/technical/sema.md)
- [Português do Brasil](./sema.md)

## Papel

A análise semântica valida a HIR, resolve símbolos, gerencia tabelas de escopo e produz diagnósticos semânticos sem reescrever a própria HIR.

## Baseline Atual

O subconjunto alpha atual inclui:

- tabelas de símbolos por rotina;
- bindings locais e `STATIC`;
- statics em nível de módulo;
- base de resolução relacionada a memvars;
- tratamento de escopo aninhado para parâmetros de codeblock;
- diagnósticos de símbolo não resolvido e binding duplicado.

## Regras de Design

- manter decisões semânticas em side tables sempre que possível;
- preservar comportamento de resolução case-insensitive;
- percorrer todas as expressões relevantes, incluindo arrays, indexação, macro reads e codeblocks;
- preferir diagnósticos explícitos a fallback implícito quando o comportamento ainda não estiver suportado.

## Estado Atual

A camada semântica é suficiente para o pipeline alpha atual do projeto e já suporta comportamento procedural e dinâmico inicial.
