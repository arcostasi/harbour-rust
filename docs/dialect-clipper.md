# Dialeto Clipper

## Posição

O baseline inicial do `harbour-rust` é compatibilidade com um subconjunto Clipper procedural.

## Prioridades

1. estrutura de programa (`PROCEDURE`, `FUNCTION`, `RETURN`)
2. controle de fluxo básico
3. variáveis locais
4. builtins mínimos
5. `STATIC`
6. arrays
7. recursos dinâmicos clássicos

## Regras

- comportamento Clipper ganha em caso de conflito inicial,
- extensões Harbour não entram por padrão,
- qualquer desvio precisa ser listado em `COMPATIBILITY.md`.

## Oráculos

- `harbour-core/doc/pp.txt`
- `harbour-core/doc/statics.txt`
- `harbour-core/doc/pcode.txt`
- `harbour-core/tests/*` curados

## Fora do escopo inicial

- classes,
- `FOR EACH`,
- `WITH OBJECT`,
- macro avançado,
- cobertura ampla de RDD.
