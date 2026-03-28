# Diagnósticos

## Meta

Mensagens previsíveis, localizadas e úteis desde a Fase 1.

## Regras

- todo diagnóstico tem arquivo, linha e coluna,
- mensagem principal curta,
- nota adicional quando houver contexto útil,
- sem `panic!` para erro de entrada do usuário,
- o mesmo erro não deve ser emitido em cascata sem valor.

## Categorias

- léxico
- pré-processador
- sintático
- semântico
- runtime
- CLI/build

## Estrutura recomendada

```rust
struct Diagnostic {
    severity: Severity,
    code: Option<&'static str>,
    message: String,
    primary_span: Span,
    notes: Vec<String>,
}
```

## Fases

### Inicial

- spans corretos,
- linha/coluna,
- mensagem primária legível.
- parser recupera blocos procedurais básicos ao encontrar a próxima rotina ou `EOF`.

### Intermediária

- notas contextuais,
- agrupamento por arquivo,
- códigos estáveis.

### Avançada

- snippets de source,
- dicas de correção,
- múltiplos arquivos e includes.

## Casos prioritários

- token inválido,
- string não terminada,
- include não encontrado,
- símbolo ausente,
- uso fora de escopo,
- dialeto desabilitado,
- builtin ausente.
