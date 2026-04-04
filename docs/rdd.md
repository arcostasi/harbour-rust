# RDD — Replaceable Database Driver

## Responsabilidade

Implementar acesso a arquivos de dados DBF (dBASE III+/Clipper) com arquitetura de driver substituível, compatível com o modelo RDD do Clipper/Harbour.

**Crate:** `harbour-rust-rdd`

## Referências upstream

- `harbour-core/src/rdd/` — implementação de referência
- `harbour-core/tests/rdd.prg` — testes básicos
- `harbour-core/tests/rddtest/` — corpus de testes RDD

## Conceito

O RDD é a abstração de acesso a dados do Clipper. Toda operação de banco de dados (abrir, ler, escrever, indexar) passa pelo RDD, que pode ser substituído por drivers diferentes:

- **DBFNTX** — driver padrão Clipper (DBF + NTX para índices)
- **DBFCDX** — driver Harbour (DBF + CDX para índices)
- **DBFFPT** — campos memo
- Drivers customizados

## Escopo planejado (Fase 10)

### Operações iniciais

| Operação | Descrição |
| --- | --- |
| `USE arquivo` | Abrir tabela DBF |
| `CLOSE` | Fechar tabela |
| `GOTO n` | Ir para registro N |
| `SKIP [n]` | Avançar/retroceder N registros |
| `BOF()` / `EOF()` | Início/fim do arquivo |
| `RECNO()` | Número do registro atual |
| `RECCOUNT()` | Total de registros |
| `FIELD->nome` | Acesso a campo |
| `REPLACE campo WITH valor` | Escrita em campo |
| `APPEND BLANK` | Novo registro |
| `DELETE` / `RECALL` | Marcar/desmarcar exclusão |

### Formato DBF

- Header com metadados da tabela
- Descritor de campos com nome, tipo, tamanho, decimais
- Registros de tamanho fixo
- Tipos: Character, Numeric, Date, Logical, Memo

### Fora do escopo inicial

- Índices (NTX, CDX)
- Memo fields (FPT/DBT)
- Locking de rede
- SET FILTER, SET RELATION
- LOCATE, SEEK
- Drivers customizados

## Arquitetura planejada

```text
┌──────────────┐
│  Comandos    │  USE, SKIP, REPLACE, etc.
│  xBase       │
└──────┬───────┘
       v
┌──────────────┐
│  RDD trait   │  Interface abstrata
└──────┬───────┘
       v
┌──────────────┐
│  DBFNTX      │  Implementação padrão
│  driver      │
└──────┬───────┘
       v
┌──────────────┐
│  DBF file    │  Arquivo físico
│  I/O         │
└──────────────┘
```

### Trait RDD

```rust
trait Rdd {
    fn open(&mut self, path: &Path) -> Result<()>;
    fn close(&mut self) -> Result<()>;
    fn go_to(&mut self, recno: usize) -> Result<()>;
    fn skip(&mut self, count: i32) -> Result<()>;
    fn bof(&self) -> bool;
    fn eof(&self) -> bool;
    fn recno(&self) -> usize;
    fn rec_count(&self) -> usize;
    fn field_get(&self, name: &str) -> Result<Value>;
    fn field_put(&mut self, name: &str, value: Value) -> Result<()>;
    fn append_blank(&mut self) -> Result<()>;
    fn deleted(&self) -> bool;
    fn delete(&mut self) -> Result<()>;
    fn recall(&mut self) -> Result<()>;
}
```

## Decisões de design

### Trait-based

O RDD é um trait em Rust para permitir drivers plugáveis, assim como no Harbour original.

### Só após frontend estável

O RDD só entra na Fase 10, depois que o frontend (PP, parser, sema) e o pipeline (IR, codegen, CLI) estiverem estáveis.

### Compatibilidade com DBF existente

O driver deve ler e escrever arquivos DBF criados pelo Clipper/Harbour original sem corrupção.

## Estado atual

Fase 10 iniciada:

- trait `Rdd` definida com a superfície mínima de navegação e mutação planejada,
- modelos `DbfHeader`, `FieldDescriptor` e `DbfSchema` implementados,
- parser binário inicial de header e descritores de campo validado contra DBFs reais do `harbour-core`,
- navegação inicial com `GOTO`, `SKIP`, `RECNO`, `RECCOUNT`, `BOF` e `EOF` implementada para DBF,
- leitura inicial de campos `C`, `N`, `L` e `D` implementada em `field_get()` e `snapshot()`,
- `APPEND BLANK` e `field_put()` já persistem em disco para campos `C`, `N`, `L` e `D`,
- `DELETE` e `RECALL` já persistem a flag de exclusão no arquivo DBF,
- testes cobrem `users.dbf`, `carts.dbf`, `items.dbf` e `test.dbf` vindos do `harbour-core`,
- suporte atual focado em tabelas DBF dBASE III (`0x03`) com campos `C`, `N`, `L` e `D`,
- índices, memo, locking e integração com comandos xBase ainda ficam fora do recorte inicial da fase.
