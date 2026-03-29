# Receitas fim a fim

Receitas passo a passo para as operações mais comuns no `harbour-rust`. Cada receita lista os arquivos a tocar, a ordem correta e os testes a executar.

## Receita 1: Adicionar novo nó AST + parser

**Exemplo:** adicionar `SWITCH/CASE/ENDSWITCH`

### Passo a passo

1. **Consulte o upstream:** `harbour-core/src/compiler/harbour.y` para a produção gramatical.

2. **Atualize o AST** (`crates/harbour-rust-ast/src/`):
   ```rust
   // Adicione o novo statement ao enum
   Statement::Switch { expr, cases, otherwise }
   ```

3. **Atualize o parser** (`crates/harbour-rust-parser/src/`):
   - Adicione keyword no reconhecimento
   - Implemente `parse_switch()` com recuperação de erro
   - Registre a produção na precedência se necessário

4. **Crie fixture** (`tests/fixtures/parser/switch.prg`):
   ```clipper
   PROCEDURE Main()
      LOCAL x := 2
      SWITCH x
      CASE 1
         ? "um"
      CASE 2
         ? "dois"
      OTHERWISE
         ? "outro"
      ENDSWITCH
   RETURN
   ```

5. **Crie golden** (`tests/fixtures/parser/switch.ast`):
   - Execute o parser, capture a saída, revise manualmente
   - Salve como baseline

6. **Adicione teste unitário** no crate do parser

7. **Atualize docs:**
   - `docs/grammar.md` — nova produção
   - `COMPATIBILITY.md` — status `partial`, fase correspondente

8. **Verifique:**
   ```bash
   cargo test -p harbour-rust-parser
   cargo test
   ```

### Arquivos tocados
- `crates/harbour-rust-ast/src/lib.rs` (ou módulo relevante)
- `crates/harbour-rust-parser/src/` (parsing functions)
- `tests/fixtures/parser/switch.prg`
- `tests/fixtures/parser/switch.ast`
- `docs/grammar.md`
- `COMPATIBILITY.md`

---

## Receita 2: Baixar novo nó da AST até C gerado (pipeline completo)

**Exemplo:** o nó `Switch` da receita anterior precisa funcionar fim a fim.

### Passo a passo

1. **HIR** (`crates/harbour-rust-hir/src/`):
   - Adicione `HirStatement::Switch { ... }` ou desugar para `If` chains
   - Implemente lowering em `lower_statement()`

2. **Sema** (`crates/harbour-rust-sema/src/`):
   - Atualize o walk para visitar o novo nó
   - Resolva símbolos nas expressões dos cases

3. **IR** (`crates/harbour-rust-ir/src/`):
   - Adicione `IrStatement::Switch { ... }` ou use a forma desugared
   - Implemente lowering HIR → IR

4. **Codegen-C** (`crates/harbour-rust-codegen-c/src/`):
   - Implemente emissão de C para o novo nó (ex.: `switch` ou cadeia de `if/else if`)
   - Se precisar de helper C: atualize `crates/harbour-rust-cli/support/`

5. **Teste de integração:**
   - Use `cargo run -p harbour-rust-cli -- build switch.prg --out target/switch.c`
   - Verifique C gerado
   - Use `cargo run -p harbour-rust-cli -- run switch.prg` para executar

6. **Atualize docs:** `docs/hir.md`, `docs/ir.md`, `docs/codegen-c.md`

### Verificação
```bash
cargo test -p harbour-rust-hir
cargo test -p harbour-rust-sema
cargo test -p harbour-rust-ir
cargo test -p harbour-rust-codegen-c
cargo test -p harbour-rust-cli
cargo test
```

---

## Receita 3: Adicionar um builtin ao runtime

**Exemplo:** adicionar `Upper()`

### Passo a passo

1. **Consulte o upstream:** `harbour-core/src/rtl/str.c` para semântica de `Upper()`.

2. **Consulte testes upstream:** `harbour-core/utils/hbtest/rt_str.prg` para corner cases.

3. **Implemente no runtime** (`crates/harbour-rust-runtime/src/`):
   ```rust
   pub fn upper(args: &[Value]) -> Result<Value, RuntimeError> {
       // Validar argumento: precisa ser String
       // Retornar String uppercase
       // Erro se não for String
   }
   ```

4. **Registre no lookup de builtins:**
   - Adicione `"UPPER"` → `upper` no `Builtin::lookup()`
   - Caso imutável: adicione em `call_builtin()`
   - Caso mutante (raro): adicione em `call_builtin_mut()`

5. **Adicione testes unitários** (`crates/harbour-rust-runtime/tests/`):
   ```rust
   #[test]
   fn upper_basic() {
       let result = upper(&[Value::from("hello")]);
       assert_eq!(result.unwrap(), Value::from("HELLO"));
   }

   #[test]
   fn upper_type_error() {
       let result = upper(&[Value::Integer(42)]);
       assert!(result.is_err());
   }
   ```

6. **Teste de integração** (fixture `.prg`):
   ```clipper
   PROCEDURE Main()
      ? Upper("hello")     // esperado: HELLO
      ? Upper("WORLD")     // esperado: WORLD
      ? Upper("")          // esperado: (vazio)
   RETURN
   ```

7. **Atualize docs:**
   - `docs/runtime.md` — adicione na lista de builtins
   - `COMPATIBILITY.md` — `Upper()` status `done` ou `partial`

### Arquivos tocados
- `crates/harbour-rust-runtime/src/` (implementação + lookup)
- `crates/harbour-rust-runtime/tests/` (testes unitários)
- `tests/fixtures/` (fixture .prg se houver integração)
- `docs/runtime.md`
- `COMPATIBILITY.md`

---

## Receita 4: Adicionar teste de compatibilidade

**Exemplo:** validar que `SubStr()` se comporta igual ao Harbour

### Passo a passo

1. **Crie fixture mínima** (`tests/fixtures/compat/substr.prg`):
   ```clipper
   PROCEDURE Main()
      ? SubStr("Hello", 2, 3)   // esperado: ell
      ? SubStr("Hello", 2)      // esperado: ello
      ? SubStr("Hello", -2)     // esperado: lo
   RETURN
   ```

2. **Execute com harbour-core** (se disponível):
   ```bash
   cd harbour-core && harbour tests/fixtures/compat/substr.prg
   # capture stdout como golden
   ```

3. **Execute com harbour-rust:**
   ```bash
   cargo run -p harbour-rust-cli -- run tests/fixtures/compat/substr.prg
   ```

4. **Compare saídas.**

5. **Adicione teste automatizado** em `crates/harbour-rust-compat/`:
   ```rust
   #[test]
   fn compat_substr() {
       // Executar fixture e comparar com golden
   }
   ```

6. **Atualize `COMPATIBILITY.md`.**

---

## Receita 5: Estender o pré-processador

**Exemplo:** adicionar `#ifdef` / `#endif`

### Passo a passo

1. **Consulte upstream:** `harbour-core/doc/pp.txt` e `harbour-core/src/pp/ppcore.c`.

2. **Implemente no PP** (`crates/harbour-rust-pp/src/`):
   - Reconheça a diretiva na linha
   - Mantenha stack de condicionais
   - Emita ou suprima linhas conforme a condição

3. **Crie fixtures:**
   - `tests/fixtures/pp/ifdef_true.prg` — define ativo
   - `tests/fixtures/pp/ifdef_false.prg` — define inativo
   - Golden files `.out` correspondentes

4. **Teste unitário** no crate do PP

5. **Teste de integração** via CLI:
   ```bash
   cargo run -p harbour-rust-cli -- build tests/fixtures/pp/ifdef_true.prg --out target/ifdef.c
   ```

6. **Atualize:** `docs/preprocessor.md`, `COMPATIBILITY.md`

---

## Receita 6: Investigar e corrigir regressão

### Passo a passo

1. **Reproduza:** execute `cargo test` e identifique a falha.

2. **Reduza:** isole o menor `.prg` que reproduz o problema.

3. **Localize a camada:** execute cada estágio do pipeline manualmente:
   - PP: fonte expandido correto?
   - Lexer: tokens corretos?
   - Parser: AST correta?
   - HIR: lowering correto?
   - Sema: resolução correta?
   - IR: lowering correto?
   - Codegen: C correto?

4. **Corrija** apenas na camada afetada, sem misturar refactor.

5. **Adicione teste de regressão** com o fixture reduzido.

6. **Atualize `COMPATIBILITY.md`** se o status de algum recurso mudou.

---

## Checklist rápido universal

Para qualquer mudança, antes de commitar:

- [ ] `cargo fmt --all`
- [ ] `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- [ ] `cargo test`
- [ ] Pelo menos um teste novo ou atualizado
- [ ] Caso feliz + caso de erro cobertos
- [ ] `COMPATIBILITY.md` atualizada se semântica mudou
- [ ] Doc temática atualizada
- [ ] Commit com Conventional Commits + footers `Phase:` e `Task:`
