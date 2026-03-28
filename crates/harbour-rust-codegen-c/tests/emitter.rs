use std::{fs, path::PathBuf};

use harbour_rust_codegen_c::emit_program;
use harbour_rust_hir::lower_program as lower_hir_program;
use harbour_rust_ir::lower_program as lower_ir_program;
use harbour_rust_parser::parse;

fn workspace_fixture(path: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join(path)
}

fn emit_fixture(path: &str) -> harbour_rust_codegen_c::CodegenOutput {
    let source = fs::read_to_string(workspace_fixture(path)).expect("fixture source");
    let parsed = parse(&source);
    assert!(
        parsed.errors.is_empty(),
        "unexpected parse errors: {:?}",
        parsed.errors
    );

    let hir = lower_hir_program(&parsed.program);
    assert!(
        hir.errors.is_empty(),
        "unexpected hir lowering errors: {:?}",
        hir.errors
    );

    let ir = lower_ir_program(&hir.program);
    assert!(
        ir.errors.is_empty(),
        "unexpected ir lowering errors: {:?}",
        ir.errors
    );

    emit_program(&ir.program)
}

#[test]
fn emits_hello_fixture_as_c_with_qout_and_main_wrapper() {
    let emitted = emit_fixture("tests/fixtures/parser/hello.prg");

    assert!(
        emitted.errors.is_empty(),
        "unexpected codegen errors: {:?}",
        emitted.errors
    );
    assert!(emitted.source.contains("int main(void)"));
    assert!(emitted.source.contains("harbour_builtin_qout("));
    assert!(
        emitted
            .source
            .contains("harbour_value_from_string_literal(\"Hello, world!\")")
    );
    assert!(emitted.source.contains("return harbour_value_nil();"));
}

#[test]
fn emits_while_fixture_as_c_with_runtime_loop_helpers() {
    let emitted = emit_fixture("tests/fixtures/parser/while.prg");

    assert!(
        emitted.errors.is_empty(),
        "unexpected codegen errors: {:?}",
        emitted.errors
    );
    assert!(emitted.source.contains("while (harbour_value_is_true("));
    assert!(
        emitted
            .source
            .contains("harbour_value_postfix_increment(&x)")
    );
    assert!(emitted.source.contains("harbour_value_less_than("));
}
