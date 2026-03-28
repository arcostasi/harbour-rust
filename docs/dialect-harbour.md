# Dialeto Harbour

## Posição

Recursos específicos de Harbour entram depois da estabilidade do baseline procedural e sempre atrás de flag explícita de dialeto.

## Recursos previstos

- `FOR EACH`
- `WITH OBJECT`
- macro mais amplo
- extensões de expressão
- recursos adicionais de preprocessor

## Fontes upstream

- `harbour-core/doc/clipper.txt`
- `harbour-core/src/compiler/harbour.y`
- `harbour-core/src/vm/*`

## Regras

- nenhum recurso Harbour deve bloquear o caminho Clipper inicial,
- toda extensão deve ter teste dedicado,
- documentação deve indicar claramente quando algo é Harbour-only.
