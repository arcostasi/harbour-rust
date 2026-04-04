# Centro de Documentação

- [English](./README.md)
- [Português do Brasil](./README.pt-BR.md)

## Arquitetura da Documentação

O modelo de documentação pública deste repositório é:

```text
/
├─ README.md
├─ README.pt-BR.md
├─ CONTRIBUTING.md
├─ CONTRIBUTING.pt-BR.md
├─ ROADMAP.md
├─ ROADMAP.pt-BR.md
├─ COMPATIBILITY.md
├─ COMPATIBILITY.pt-BR.md
├─ GOVERNANCE.md
├─ GOVERNANCE.pt-BR.md
├─ CODE_OF_CONDUCT.md
├─ CODE_OF_CONDUCT.pt-BR.md
├─ SECURITY.md
├─ SECURITY.pt-BR.md
├─ SUPPORT.md
├─ SUPPORT.pt-BR.md
├─ PROVENANCE.md
├─ PROVENANCE.pt-BR.md
└─ docs/
   ├─ README.md
   ├─ README.pt-BR.md
   ├─ en/
   │  ├─ documentation-standards.md
   │  ├─ translation-workflow.md
   │  ├─ legal-and-provenance.md
   │  └─ technical/README.md
   └─ pt-BR/
      ├─ padroes-de-documentacao.md
      ├─ fluxo-de-traducao.md
      ├─ legal-e-proveniencia.md
      └─ technical/README.md
```

## Política de Idioma

- Inglês é o idioma principal.
- Português do Brasil é o idioma secundário oficial.
- Documentos institucionais públicos devem existir nos dois idiomas.
- Inglês é a fonte da verdade para estrutura e intenção.

## Convenções de Nomenclatura

- Use lowercase kebab-case dentro de `docs/en/`.
- Use lowercase kebab-case ou nomes portugueses estabelecidos dentro de `docs/pt-BR/`.
- Na raiz do repositório, o arquivo em Inglês mantém o nome padrão e o arquivo em Português usa `.pt-BR.md`.
- Mantenha arquivos espelhados estruturalmente alinhados.

## Documentação Técnica em Transição

Este repositório já contém documentos técnicos de trabalho, como `docs/architecture.md`, `docs/runtime.md` e outras notas por camada do compilador. Esses arquivos continuam válidos durante a migração para um modelo público totalmente bilíngue.

A abordagem-alvo é:

- guias técnicos em Inglês sob `docs/en/technical/`;
- espelhos em Português sob `docs/pt-BR/technical/`;
- migração gradual sem quebrar o fluxo de quem contribui.

## Referências Obrigatórias

- [Padrões de Documentação](./pt-BR/padroes-de-documentacao.md)
- [Fluxo de Tradução](./pt-BR/fluxo-de-traducao.md)
- [Guia Legal e de Proveniência](./pt-BR/legal-e-proveniencia.md)
