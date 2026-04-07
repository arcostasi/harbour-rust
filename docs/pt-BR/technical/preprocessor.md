# Pré-processador

- [English](../../en/technical/preprocessor.md)
- [Português do Brasil](./preprocessor.md)

## Papel

O pré-processador trata diretivas de compilação antes do lexer e do parser principais. Seu objetivo é suportar um subconjunto prático de compatibilidade com o pré-processamento de Clipper/Harbour.

## Baseline Atual

As áreas implementadas incluem:

- `#define` objeto;
- expansão recursiva de define com detecção de ciclo;
- `#include` com busca quoted e angle-bracket;
- search paths configuráveis para include;
- subconjuntos curados de `#command`, `#translate`, `#xcommand` e `#xtranslate`.

## Regras de Design

- manter o pré-processamento separado do lexer principal;
- preservar informação de origem para diagnósticos;
- evoluir gradualmente para comportamento mais orientado a tokens;
- medir compatibilidade com fixtures focados, não com promessas vagas.

## Estado Atual

O pré-processador já suporta o baseline alpha atual do projeto. Fixtures focadas de compatibilidade agora cobrem replacements opcionais com colchetes escapados, reordenação selecionada de cláusulas opcionais contíguas, result markers lógicos como `<.id.>`, um subconjunto mínimo de blockify `<{id}>`, um subconjunto mínimo de quoted-result `<"id">` e um subconjunto mínimo de smart-result `<(id)>` ancorados no corpus hbpp do upstream. Combinações mais amplas de opcionais/listas, edge cases de dumb-stringify e semântica mais ampla desses markers continuam como trabalho futuro.
