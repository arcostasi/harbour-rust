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
| `0.4.0-alpha` | base de RDD, CLI/DX, regressão e tooling de release | concluída |
| `0.5.0-alpha` | expansão curada de compatibilidade da fase 15, com crescimento focado do corpus avançado de PP | concluída |
| `0.6.0-alpha` | fidelidade de runtime da fase 16, começando por builtins focados de runtime/biblioteca Harbour | planejada |

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
| 13 | marcadores avançados de pré-processador ancorados no oráculo | concluída |
| 14 | expansão curada do corpus de compatibilidade | concluída |
| 15 | expansão de compatibilidade pós-0.4 | primeiro slice de release concluído |
| 16 | fidelidade de runtime pós-0.5 | planejada |

## Prioridades de Curto Prazo

Depois da release `0.5.0-alpha`, a próxima prioridade é a fidelidade de runtime da fase 16.

O primeiro corredor planejado é:

1. implementar o menor slice de `hb_JsonDecode` ancorado em oráculo que consiga mapear escalares, arrays e objetos JSON para o modelo atual de valores do runtime;
2. documentar explicitamente os edge cases JSON/valor ainda não suportados, sem sugerir cobertura completa da API Harbour;
3. aplicar o mesmo padrão a slices posteriores de `hb_gzCompress` e `hb_processRun` apenas depois de estabilizar o comportamento de valores e strings/binários;
4. adiar sockets e threading até o runtime ter decisões explícitas de IO, ownership e concorrência multiplataforma.

Prioridades secundárias permanecem:

1. corpus de compatibilidade maior;
2. cobertura mais ampla de DBF/RDD;
3. profiling de performance e memória;
4. endurecimento arquitetural seletivo sem perder legibilidade.

## Regras de Planejamento

- Prefira incrementos pequenos e reversíveis.
- Mantenha o comportamento mensurável com testes e fixtures.
- Separe trabalho de parser, semântica, runtime, geração de código e RDD sempre que possível.
- Documente incompatibilidades conhecidas em vez de escondê-las.
- Trate o Inglês como idioma canônico para atualizações do roadmap e mantenha a versão em Português alinhada.
