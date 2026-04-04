# Roadmap

- [English](./ROADMAP.md)
- [Português do Brasil](./ROADMAP.pt-BR.md)

## Propósito

Este roadmap organiza o Harbour Rust em marcos pequenos, cumulativos e verificáveis. Ele é deliberadamente orientado à compatibilidade e usa `harbour-core` como referência de comportamento, não como fonte para transliteração.

## Marcos de Release

| Release | Foco | Status |
| --- | --- | --- |
| `0.1.0-alpha` | pipeline procedural mínimo ponta a ponta | concluída |
| `0.2.0-alpha` | compatibilidade procedural ampliada e suporte inicial de pré-processador | concluída |
| `0.3.0-alpha` | comportamento dinâmico xBase | concluída |
| `0.4.0-alpha` | base de RDD, CLI/DX, regressão e tooling de release | em preparação de release |

## Panorama das Fases

| Fase | Tema | Status |
| --- | --- | --- |
| 0 | fundação do repositório | concluída |
| 1 | lexer | concluída |
| 2 | AST e parser | concluída |
| 3 | HIR e semântica básica | concluída |
| 4 | runtime mínimo | concluída |
| 5 | IR e backend C | concluída |
| 6 | pré-processador inicial | concluída |
| 7 | compatibilidade procedural ampliada | concluída |
| 8 | recursos dinâmicos de xBase | concluída |
| 9 | pré-processador avançado | concluída |
| 10 | base de DBF/RDD | concluída |
| 11 | diagnósticos, CLI, DX | concluída |
| 12 | qualidade e prontidão para release | concluída |

## Prioridades de Curto Prazo

Depois do congelamento da release `0.4.0-alpha`, as próximas prioridades esperadas são:

1. corpus de compatibilidade maior;
2. fidelidade de runtime mais próxima do comportamento histórico;
3. cobertura mais ampla de DBF/RDD;
4. profiling de performance e memória;
5. endurecimento arquitetural seletivo sem perder legibilidade.

## Regras de Planejamento

- Prefira incrementos pequenos e reversíveis.
- Mantenha o comportamento mensurável com testes e fixtures.
- Separe trabalho de parser, semântica, runtime, geração de código e RDD sempre que possível.
- Documente incompatibilidades conhecidas em vez de escondê-las.
- Trate o Inglês como idioma canônico para atualizações do roadmap e mantenha a versão em Português alinhada.
