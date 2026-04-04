# Documentation Center

- [English](./README.md)
- [Português do Brasil](./README.pt-BR.md)

## Documentation Architecture

The public documentation model for this repository is:

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

## Language Policy

- English is the primary language.
- Portuguese (Brazil) is the official secondary language.
- Public institutional documents should exist in both languages.
- English is the source of truth for structure and intent.

## Naming Conventions

- Use lowercase kebab-case inside `docs/en/`.
- Use lowercase kebab-case or established Portuguese names inside `docs/pt-BR/`.
- At repository root, the English file keeps the default name and the Portuguese file uses `.pt-BR.md`.
- Keep mirrored files structurally aligned.

## Transitional Technical Documentation

This repository already contains technical working documents such as `docs/architecture.md`, `docs/runtime.md`, and related compiler-layer notes. Those files remain valid during the migration to a fully bilingual public documentation model.

The target approach is:

- English-first technical guides under `docs/en/technical/`;
- Portuguese mirrors under `docs/pt-BR/technical/`;
- gradual migration without breaking contributor workflows.

## Required References

- [Documentation Standards](./en/documentation-standards.md)
- [Translation Workflow](./en/translation-workflow.md)
- [Legal and Provenance Guide](./en/legal-and-provenance.md)
