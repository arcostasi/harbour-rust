# Diagnósticos

- [English](../../en/technical/diagnostics.md)
- [Português do Brasil](./diagnostics.md)

## Meta

Diagnósticos devem ser previsíveis, localizados e úteis desde as fases iniciais do compilador.

## Princípios

- sempre incluir arquivo e posição no source quando possível;
- preferir mensagens primárias curtas;
- evitar falhas visíveis ao usuário baseadas em `panic`;
- reduzir cascatas de ruído quando um erro-raiz já explica a falha.

## Categorias Atuais

O projeto usa agrupamentos práticos de diagnóstico para:

- erros léxicos;
- erros de pré-processador;
- erros sintáticos;
- erros semânticos;
- erros de runtime;
- falhas de geração de código;
- falhas de CLI e build.

## Estado Atual

Os diagnósticos já estão estruturados o suficiente para o fluxo alpha atual, mas snippets mais ricos e contexto multi-arquivo mais avançado ainda pertencem a trabalho futuro.
