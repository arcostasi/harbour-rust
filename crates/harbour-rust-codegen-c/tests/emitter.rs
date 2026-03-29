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

#[test]
fn emits_for_sum_fixture_as_c_with_for_loop_helpers() {
    let emitted = emit_fixture("tests/fixtures/parser/for_sum.prg");

    assert!(
        emitted.errors.is_empty(),
        "unexpected codegen errors: {:?}",
        emitted.errors
    );
    assert!(emitted.source.contains("while (harbour_value_is_true("));
    assert!(emitted.source.contains("harbour_value_less_than_or_equal("));
    assert!(emitted.source.contains("sum = harbour_value_add(sum, n);"));
    assert!(
        emitted
            .source
            .contains("n = harbour_value_add(n, harbour_value_from_integer(1LL));")
    );
}

#[test]
fn emits_array_runtime_helper_declarations_in_c_prelude() {
    let emitted = emit_fixture("tests/fixtures/parser/hello.prg");

    assert!(
        emitted
            .source
            .contains("extern harbour_runtime_Value harbour_value_from_array_items(const harbour_runtime_Value *items, size_t length);")
    );
    assert!(
        emitted
            .source
            .contains("extern size_t harbour_value_array_len(harbour_runtime_Value value);")
    );
    assert!(
        emitted
            .source
            .contains("extern harbour_runtime_Value harbour_value_array_get(harbour_runtime_Value value, harbour_runtime_Value index);")
    );
    assert!(
        emitted
            .source
            .contains("extern harbour_runtime_Value harbour_value_array_set_path(harbour_runtime_Value *value, const harbour_runtime_Value *indices, size_t index_count, harbour_runtime_Value assigned);")
    );
}

#[test]
fn emits_arrays_fixture_with_array_constructor_helpers() {
    let emitted = emit_fixture("tests/fixtures/parser/arrays.prg");

    assert!(
        emitted.errors.is_empty(),
        "unexpected codegen errors: {:?}",
        emitted.errors
    );
    assert!(
        emitted
            .source
            .contains("harbour_value_from_array_items(NULL, 0)")
    );
    assert!(
        emitted
            .source
            .contains("harbour_value_from_array_items((harbour_runtime_Value[]) { harbour_value_from_integer(1LL), harbour_value_from_string_literal(\"x\"), cache }, 3)")
    );
}

#[test]
fn emits_indexing_fixture_with_array_get_helpers() {
    let emitted = emit_fixture("tests/fixtures/parser/indexing.prg");

    assert!(
        emitted.errors.is_empty(),
        "unexpected codegen errors: {:?}",
        emitted.errors
    );
    assert!(
        emitted
            .source
            .contains("harbour_runtime_Value matrix = harbour_value_from_array_items((harbour_runtime_Value[]) { harbour_value_from_array_items((harbour_runtime_Value[]) { harbour_value_from_integer(10LL), harbour_value_from_integer(20LL) }, 2), harbour_value_from_array_items((harbour_runtime_Value[]) { harbour_value_from_integer(30LL), harbour_value_from_integer(40LL) }, 2) }, 2);")
    );
    assert!(
        emitted
            .source
            .contains("return harbour_value_array_get(harbour_value_array_get(matrix, row), harbour_value_add(harbour_value_from_integer(1LL), col));")
    );
}

#[test]
fn emits_indexed_assignment_fixture_with_array_set_path_helper() {
    let emitted = emit_fixture("tests/fixtures/parser/indexed_assign.prg");

    assert!(
        emitted.errors.is_empty(),
        "unexpected codegen errors: {:?}",
        emitted.errors
    );
    assert!(
        emitted
            .source
            .contains("harbour_runtime_Value matrix = harbour_value_from_array_items((harbour_runtime_Value[]) { harbour_value_from_array_items((harbour_runtime_Value[]) { harbour_value_from_integer(10LL), harbour_value_from_integer(20LL) }, 2), harbour_value_from_array_items((harbour_runtime_Value[]) { harbour_value_from_integer(30LL), harbour_value_from_integer(40LL) }, 2) }, 2);")
    );
    assert!(
        emitted
            .source
            .contains("(void) harbour_value_array_set_path(&matrix, (harbour_runtime_Value[]) { harbour_value_from_integer(2LL), harbour_value_from_integer(1LL) }, 2, harbour_value_from_integer(99LL));")
    );
    assert!(
        emitted
            .source
            .contains("harbour_builtin_qout((harbour_runtime_Value[]) { harbour_value_array_get(harbour_value_array_get(matrix, harbour_value_from_integer(2LL)), harbour_value_from_integer(1LL)) }, 1);")
    );
}
