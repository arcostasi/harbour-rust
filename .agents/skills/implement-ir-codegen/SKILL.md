---
name: implement-ir-codegen
description: Implementar ou estender IR lowering e geração de código C do harbour-rust com foco em C legível, helpers explícitos e caminho executável.
---

# Implement IR / Codegen-C

## Leia antes de editar

- `AGENTS.md`
- `ROADMAP.md`
- `docs/ir.md`
- `docs/codegen-c.md`
- `docs/architecture.md`
- `harbour-core/src/compiler/genc.c` (referência pontual)

## Objetivo

Estender o lowering HIR -> IR ou a geração de código C a partir da IR.

## Fluxo

### Para IR (lowering)

1. Identifique o nó HIR que precisa de lowering para IR.
2. Implemente o lowering em `harbour-rust-ir`.
3. Adicione teste de lowering com fixture.
4. Verifique que o codegen-c não quebrou.

### Para Codegen-C (geração)

1. Identifique o nó IR que precisa gerar C.
2. Implemente em `harbour-rust-codegen-c`.
3. Se necessário, adicione helpers ao runtime support C.
4. Adicione teste de geração de C.
5. Se aplicável, verifique caminho executável com `cargo run -p harbour-rust-cli -- run`.

## Não fazer

- gerar C para nós que a IR ainda não suporta — implemente na IR primeiro,
- adicionar helpers C sem teste,
- misturar IR nova com codegen novo no mesmo PR sem dependência mínima,
- gerar C silenciosamente incorreto — use erro de codegen explícito.

## Definition of done

- IR representa o novo nó corretamente,
- C gerado compila com compilador host,
- se executável: executa corretamente,
- erros de codegen explícitos para nós não suportados,
- testes de geração e/ou integração passando,
- docs atualizadas.
