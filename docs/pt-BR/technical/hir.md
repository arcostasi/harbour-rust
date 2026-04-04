# HIR

- [English](../../en/technical/hir.md)
- [Português do Brasil](./hir.md)

## Papel

A HIR é a representação lowered de alto nível entre AST e análise semântica. Ela remove açúcar sintático e prepara estruturas mais fáceis de analisar.

## Responsabilidades Principais

- normalizar identificadores em símbolos;
- separar leituras de alvos de escrita;
- preservar categorias explícitas de storage como `LOCAL`, `STATIC` e statics de módulo;
- representar arrays, indexação, memvars, codeblocks e macro reads de forma amigável para análise.

## Regras de Design

- manter a HIR pequena, mas semanticamente útil;
- evitar que a análise semântica reescreva a representação central;
- introduzir nós apenas quando carregarem valor semântico real.

## Estado Atual

A HIR já cobre o subconjunto alpha procedural e dinâmico atualmente implementado e atua como handoff estável para a análise semântica.
