# RDD e DBF

- [English](../../en/technical/rdd.md)
- [Português do Brasil](./rdd.md)

## Papel

A camada de RDD fornece o modelo inicial de acesso a dados DBF do projeto, inspirado na arquitetura histórica de Replaceable Database Driver.

## Baseline Atual

O subconjunto alpha atual inclui:

- um trait `Rdd` central;
- parsing de schema DBF;
- primitivas de navegação;
- leitura de campos para os tipos atualmente suportados;
- persistência de append, update, delete e recall no subconjunto DBF suportado.

## Regras de Design

- manter preocupações de driver/storage separadas das camadas de frontend/compilador;
- começar com um baseline DBF mínimo e testável;
- preservar espaço para futura substituição de drivers e suporte a índices.

## Estado Atual

O suporte a RDD está presente como fundação, não como camada completa de banco xBase. Ele é deliberadamente pequeno, prático e documentado dessa forma.
