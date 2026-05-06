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
| `0.6.0-alpha` | fidelidade de runtime da fase 16, começando por builtins focados de runtime/biblioteca Harbour | concluída |
| `0.7.0-alpha` | fidelidade de runtime da fase 16, com expansão de slices zlib e process-run | concluída |

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
| 16 | fidelidade de runtime pós-0.5 | segundo slice de release concluído |

## Prioridades de Curto Prazo

Depois da release `0.7.0-alpha`, a próxima prioridade é continuar a fidelidade de runtime da fase 16 sem ampliar alegações de compatibilidade além de slices de runtime/biblioteca testados.

O corredor entregue em `0.7.0-alpha` cobre:

1. o menor slice de `hb_JsonDecode` ancorado em oráculo que mapeia escalares, arrays e objetos JSON para o modelo atual de valores do runtime;
2. documentação explícita dos edge cases JSON/valor ainda não suportados, sem sugerir cobertura completa da API Harbour;
3. slices focados de zlib cobrindo `hb_gzCompressBound()`, `hb_ZCompressBound()`, `hb_gzCompress()` com saída direta, `@nResult`, `nDstBufLen` numérico, writeback de buffer, `hb_ZError()` e limites de compressão documentados;
4. tratamento focado de exit status em `hb_processRun( cCommand )` e captura de stdout em `hb_processRun( cCommand, NIL, @cStdOut )`, deixando stdin, stderr, detach, ambiente e quoting para oráculos posteriores;
5. adiamento de sockets e threading até o runtime ter decisões explícitas de IO, ownership e concorrência multiplataforma.

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
