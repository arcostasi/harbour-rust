# Navegador do upstream harbour-core

Guia para encontrar rapidamente a referência certa no código-fonte do Harbour original.

## Estrutura principal

```
harbour-core/
├── src/
│   ├── compiler/     # Compilador: gramática, codegen, otimização
│   ├── pp/           # Pré-processador
│   ├── vm/           # Máquina virtual: eval, memvars, arrays, GC
│   ├── rtl/          # Runtime library: 237 builtins
│   ├── rdd/          # Database drivers (DBF, NTX, CDX)
│   ├── macro/        # Compilador de macros (&)
│   ├── common/       # Utilitários compartilhados
│   ├── codepage/     # Code pages (character encoding)
│   └── debug/        # Debugger support
├── include/          # Headers (.h e .ch)
├── doc/              # Documentação técnica
├── tests/            # ~500 programas .prg de teste
└── utils/
    ├── hbtest/       # Framework de testes de compatibilidade
    └── hbmk2/        # Build tool do Harbour
```

## Mapa por tarefa

### Lexer / Tokens

| O que procurar | Onde |
| --- | --- |
| Lista completa de keywords | `src/compiler/harbour.y` (seção `%token`) |
| Tokens do PP | `src/pp/ppcore.c` (tokenizer interno) |
| Precedência de operadores | `src/compiler/harbour.y` (seção `%left`, `%right`) |
| Escape sequences em strings | `src/compiler/harbour.y` + `src/rtl/str.c` |

### Pré-processador

| O que procurar | Onde |
| --- | --- |
| Arquitetura e semântica do PP | `doc/pp.txt` (leitura obrigatória) |
| Implementação core | `src/pp/ppcore.c` |
| `#define`, `#include`, `#command`, `#translate` | `src/pp/ppcore.c` funções `hb_pp_*` |
| Testes do PP | `tests/hbpp/_pp_test.prg` |
| Headers padrão (.ch) | `include/*.ch` (ex.: `std.ch`, `hbclass.ch`) |

> **Atenção:** `doc/pp.txt` documenta que o PP do Clipper é **lexer-based** (opera em tokens, não texto). Macros como `#define a -1` têm comportamento surpreendente.

### Parser / Gramática

| O que procurar | Onde |
| --- | --- |
| Gramática completa (Bison) | `src/compiler/harbour.y` |
| Produções de statements | `harbour.y` → procure por `Statement:`, `IfStatement:`, etc. |
| Precedência e associatividade | `harbour.y` → seções `%left`, `%right`, `%nonassoc` |
| Expressões e operadores | `harbour.y` → produção `Expression:` |
| Corner cases Clipper vs Harbour | `doc/clipper.txt` |
| Gramática auxiliar | `doc/grammar.txt` |

### HIR / Análise semântica

| O que procurar | Onde |
| --- | --- |
| Compilação de expressões | `src/compiler/harbour.y` (ações semânticas) |
| Otimização de expressões | `src/compiler/expropta.c`, `exproptb.c` |
| Tabela de símbolos | `src/compiler/hbcomp.c`, `hbident.c` |
| Storage de statics | `doc/statics.txt` |
| Resolução de funções | `src/compiler/hbfunchk.c` |

### Runtime / Valores

| O que procurar | Onde |
| --- | --- |
| Modelo de VM e pcode | `doc/vm.txt`, `doc/pcode.txt` |
| API de itens (Value) | `src/vm/itemapi.c`, `include/hbapi.h` |
| Tipos e conversões | `include/hbtypes.h`, `src/vm/hvm.c` |
| Aritmética e comparação | `src/vm/hvm.c` (funções `hb_vm*Add`, `hb_vm*Equal`) |
| Garbage collector | `src/vm/garbage.c` |
| Procedure/function dispatch | `src/vm/proc.c` |

### Builtins (RTL)

| O que procurar | Onde |
| --- | --- |
| QOut / QQOut | `src/rtl/console.c` |
| Strings: SubStr, Left, Right, Trim, Upper, Lower, At | `src/rtl/str.c`, `src/rtl/at.c`, `src/rtl/substr.c`, `src/rtl/trim.c`, `src/rtl/left.c`, `src/rtl/right.c` |
| Strings: Pad, Replicate, Space, StrTran | `src/rtl/pad.c`, `src/rtl/strtran.c` |
| Math: Abs, Int, Round, Mod, Sqrt, Log, Exp | `src/rtl/math.c`, `src/rtl/round.c`, `src/rtl/mod.c`, `src/rtl/abs.c` |
| Date/Time: Date, Time, Seconds | `src/rtl/dates.c`, `src/rtl/datec.c` |
| Conversão: Val, Str, CStr, ValType | `src/rtl/val.c`, `src/rtl/str.c`, `src/rtl/valtype.c` |
| Arrays: AAdd, ASize, AClone, ADel, AIns, ASort, AScan, AEval | `src/vm/arrays.c`, `src/vm/arrayshb.c` |
| Len, Empty, Type | `src/rtl/len.c`, `src/rtl/empty.c`, `src/rtl/type.c` |
| File I/O | `src/rtl/file.c`, `src/rtl/fileio.c` |
| ErrorSys | `src/rtl/errorsys.prg`, `src/rtl/error.c` |

### Memvars e escopo dinâmico

| O que procurar | Onde |
| --- | --- |
| PRIVATE / PUBLIC | `src/vm/memvars.c` |
| Clipper-style memvars | `src/vm/memvclip.c` |
| Semântica de escopo dinâmico | `doc/vm.txt` |
| Testes de memvar | `tests/memvar.prg` |

### Codeblocks

| O que procurar | Onde |
| --- | --- |
| Implementação | `src/vm/codebloc.c` |
| Documentação | `doc/codebloc.txt` |
| Testes | `tests/cblock*.prg` |

### Macro operator (`&`)

| O que procurar | Onde |
| --- | --- |
| Compilador de macros | `src/macro/` |
| Documentação | `doc/clipper.txt` (seção de macro) |
| Testes | `tests/macro.prg` |

### Arrays

| O que procurar | Onde |
| --- | --- |
| Implementação VM | `src/vm/arrays.c`, `src/vm/arrayshb.c` |
| Builtins de array | `src/vm/arrays.c` (AAdd, ASize, etc.) |
| Hashes (Harbour ext.) | `src/vm/hashes.c`, `src/vm/hashfunc.c` |
| Testes | `tests/arrays*.prg`, `utils/hbtest/rt_array.prg`, `utils/hbtest/rt_hvm.prg` |

### RDD / DBF

| O que procurar | Onde |
| --- | --- |
| Abstração RDD | `src/rdd/dbcmd.c`, `src/rdd/workarea.c` |
| Driver DBF base | `src/rdd/dbf1.c` |
| Driver DBFNTX (índices) | `src/rdd/dbfntx/` |
| Driver DBFCDX (Harbour) | `src/rdd/dbfcdx/` |
| Memo fields | `src/rdd/dbffpt/` |
| Template para RDD custom | `src/rdd/usrrdd/` |
| Testes | `tests/rddtest/`, `tests/rdd.prg` |

### Backend C (codegen)

| O que procurar | Onde |
| --- | --- |
| Geração de C original | `src/compiler/genc.c` |
| Geração de C++ | `src/compiler/gencc.c` |
| Geração de HRB (bytecode) | `src/compiler/genhrb.c` |
| Formato do pcode | `include/hbpcode.h`, `doc/pcode.txt` |

### Testes de compatibilidade

| O que procurar | Onde |
| --- | --- |
| Testes principais | `tests/*.prg` (~500 arquivos) |
| Testes do PP | `tests/hbpp/_pp_test.prg` |
| Harness de teste | `utils/hbtest/` |
| Testes de runtime | `utils/hbtest/rt_*.prg` |
| Testes de RDD | `tests/rddtest/` |
| Testes de multi-thread | `tests/mt/` |

## Documentação upstream essencial

| Arquivo | Conteúdo | Quando ler |
| --- | --- | --- |
| `doc/pp.txt` | Arquitetura do PP, por que é lexer-based | Qualquer trabalho no PP |
| `doc/clipper.txt` | Diferenças Harbour vs Clipper | Qualquer decisão de compatibilidade |
| `doc/pcode.txt` | Modelo de pcode e bytecode | IR, codegen, VM |
| `doc/vm.txt` | Semântica da VM | Runtime, avaliação |
| `doc/statics.txt` | Storage de variáveis STATIC | STATIC, sema |
| `doc/codebloc.txt` | Semântica de codeblocks | Fase 8 |
| `doc/cmdline.md` | Opções de CLI do Harbour | CLI |
| `doc/grammar.txt` | Gramática auxiliar | Parser |
| `doc/pragma.txt` | Diretivas de pragma | PP avançado |

## Regras de uso

1. **Nunca translitere C para Rust** — use o upstream para entender semântica, depois reimplemente em Rust idiomático.
2. **Leituras pontuais** — consulte funções específicas, não copie módulos inteiros.
3. **Testes como oráculo** — execute `.prg` com `harbour-core` para validar comportamento antes de implementar.
4. **Documente divergências** — qualquer comportamento que difira do upstream deve ir para `COMPATIBILITY.md`.
