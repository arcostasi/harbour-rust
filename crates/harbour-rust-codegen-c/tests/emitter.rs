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
fn emits_if_else_fixture_as_c_with_runtime_condition_checks() {
    let emitted = emit_fixture("tests/fixtures/parser/if_else.prg");

    assert!(
        emitted.errors.is_empty(),
        "unexpected codegen errors: {:?}",
        emitted.errors
    );
    assert!(
        emitted
            .source
            .contains("if (harbour_value_is_true(harbour_value_greater_than(high, harbour_value_from_integer(5LL)))) {")
    );
    assert!(emitted.source.contains("else {"));
    assert!(
        emitted
            .source
            .contains("harbour_builtin_qout((harbour_runtime_Value[]) { harbour_value_from_string_literal(\"maior\") }, 1);")
    );
    assert!(
        emitted
            .source
            .contains("harbour_builtin_qout((harbour_runtime_Value[]) { harbour_value_from_string_literal(\"menor ou igual\") }, 1);")
    );
}

#[test]
fn emits_compound_assignment_fixture_with_arithmetic_runtime_helpers() {
    let emitted = emit_fixture("tests/fixtures/parser/compound_assign.prg");

    assert!(
        emitted.errors.is_empty(),
        "unexpected codegen errors: {:?}",
        emitted.errors
    );
    assert!(
        emitted
            .source
            .contains("total = harbour_value_add(total, harbour_value_from_integer(3LL));")
    );
    assert!(emitted.source.contains(
        "harbour_static_main_factor = harbour_value_multiply(harbour_static_main_factor, total);"
    ));
    assert!(
        emitted
            .source
            .contains("return harbour_static_main_factor;")
    );
}

#[test]
fn emits_len_builtin_fixture_with_runtime_builtin_helper_calls() {
    let emitted = emit_fixture("tests/fixtures/parser/len_builtin.prg");

    assert!(
        emitted.errors.is_empty(),
        "unexpected codegen errors: {:?}",
        emitted.errors
    );
    assert!(
        emitted
            .source
            .contains("harbour_builtin_qout((harbour_runtime_Value[]) { harbour_builtin_len((harbour_runtime_Value[]) { harbour_value_from_string_literal(\"abcd\") }, 1) }, 1);")
    );
    assert!(
        emitted
            .source
            .contains("harbour_builtin_qout((harbour_runtime_Value[]) { harbour_builtin_len((harbour_runtime_Value[]) { items }, 1) }, 1);")
    );
}

#[test]
fn emits_abs_builtin_fixture_with_runtime_builtin_helper_calls() {
    let emitted = emit_fixture("tests/fixtures/parser/abs_builtin.prg");

    assert!(
        emitted.errors.is_empty(),
        "unexpected codegen errors: {:?}",
        emitted.errors
    );
    assert!(
        emitted
            .source
            .contains("harbour_builtin_qout((harbour_runtime_Value[]) { harbour_builtin_abs((harbour_runtime_Value[]) { harbour_builtin_val((harbour_runtime_Value[]) { harbour_value_from_string_literal(\"-10\") }, 1) }, 1) }, 1);")
    );
    assert!(
        emitted
            .source
            .contains("harbour_builtin_qout((harbour_runtime_Value[]) { harbour_builtin_abs((harbour_runtime_Value[]) { harbour_builtin_val((harbour_runtime_Value[]) { harbour_value_from_string_literal(\"-10.7\") }, 1) }, 1) }, 1);")
    );
}

#[test]
fn emits_sqrt_builtin_fixture_with_runtime_builtin_helper_calls() {
    let emitted = emit_fixture("tests/fixtures/parser/sqrt_builtin.prg");

    assert!(
        emitted.errors.is_empty(),
        "unexpected codegen errors: {:?}",
        emitted.errors
    );
    assert!(
        emitted
            .source
            .contains("harbour_builtin_qout((harbour_runtime_Value[]) { harbour_builtin_sqrt((harbour_runtime_Value[]) { harbour_builtin_val((harbour_runtime_Value[]) { harbour_value_from_string_literal(\"-1\") }, 1) }, 1) }, 1);")
    );
    assert!(
        emitted
            .source
            .contains("harbour_builtin_qout((harbour_runtime_Value[]) { harbour_builtin_str((harbour_runtime_Value[]) { harbour_builtin_sqrt((harbour_runtime_Value[]) { harbour_value_from_integer(10LL) }, 1), harbour_value_from_integer(10LL), harbour_value_from_integer(2LL) }, 3) }, 1);")
    );
}

#[test]
fn emits_exp_builtin_fixture_with_runtime_builtin_helper_calls() {
    let emitted = emit_fixture("tests/fixtures/parser/exp_builtin.prg");

    assert!(
        emitted.errors.is_empty(),
        "unexpected codegen errors: {:?}",
        emitted.errors
    );
    assert!(
        emitted
            .source
            .contains("harbour_builtin_qout((harbour_runtime_Value[]) { harbour_builtin_exp((harbour_runtime_Value[]) { harbour_value_from_integer(0LL) }, 1) }, 1);")
    );
    assert!(
        emitted
            .source
            .contains("harbour_builtin_qout((harbour_runtime_Value[]) { harbour_builtin_str((harbour_runtime_Value[]) { harbour_builtin_exp((harbour_runtime_Value[]) { harbour_value_from_integer(15LL) }, 1) }, 1) }, 1);")
    );
    assert!(
        emitted
            .source
            .contains("harbour_builtin_qout((harbour_runtime_Value[]) { harbour_builtin_round((harbour_runtime_Value[]) { harbour_builtin_exp((harbour_runtime_Value[]) { harbour_value_from_integer(1LL) }, 1), harbour_value_from_integer(2LL) }, 2) }, 1);")
    );
}

#[test]
fn emits_sin_cos_builtin_fixture_with_runtime_builtin_helper_calls() {
    let emitted = emit_fixture("tests/fixtures/parser/sin_cos_builtin.prg");

    assert!(
        emitted.errors.is_empty(),
        "unexpected codegen errors: {:?}",
        emitted.errors
    );
    assert!(
        emitted
            .source
            .contains("harbour_builtin_qout((harbour_runtime_Value[]) { harbour_builtin_sin((harbour_runtime_Value[]) { harbour_value_from_integer(0LL) }, 1) }, 1);")
    );
    assert!(
        emitted
            .source
            .contains("harbour_builtin_qout((harbour_runtime_Value[]) { harbour_builtin_cos((harbour_runtime_Value[]) { harbour_value_from_integer(0LL) }, 1) }, 1);")
    );
    assert!(
        emitted
            .source
            .contains("harbour_builtin_qout((harbour_runtime_Value[]) { harbour_builtin_round((harbour_runtime_Value[]) { harbour_builtin_sin((harbour_runtime_Value[]) { harbour_value_from_integer(1LL) }, 1), harbour_value_from_integer(2LL) }, 2) }, 1);")
    );
    assert!(
        emitted
            .source
            .contains("harbour_builtin_qout((harbour_runtime_Value[]) { harbour_builtin_round((harbour_runtime_Value[]) { harbour_builtin_cos((harbour_runtime_Value[]) { harbour_value_from_integer(1LL) }, 1), harbour_value_from_integer(2LL) }, 2) }, 1);")
    );
}

#[test]
fn emits_tan_builtin_fixture_with_runtime_builtin_helper_calls() {
    let emitted = emit_fixture("tests/fixtures/parser/tan_builtin.prg");

    assert!(
        emitted.errors.is_empty(),
        "unexpected codegen errors: {:?}",
        emitted.errors
    );
    assert!(
        emitted
            .source
            .contains("harbour_builtin_qout((harbour_runtime_Value[]) { harbour_builtin_tan((harbour_runtime_Value[]) { harbour_value_from_integer(0LL) }, 1) }, 1);")
    );
    assert!(
        emitted
            .source
            .contains("harbour_builtin_qout((harbour_runtime_Value[]) { harbour_builtin_round((harbour_runtime_Value[]) { harbour_builtin_tan((harbour_runtime_Value[]) { harbour_value_from_integer(1LL) }, 1), harbour_value_from_integer(4LL) }, 2) }, 1);")
    );
}

#[test]
fn emits_log_builtin_fixture_with_runtime_builtin_helper_calls() {
    let emitted = emit_fixture("tests/fixtures/parser/log_builtin.prg");

    assert!(
        emitted.errors.is_empty(),
        "unexpected codegen errors: {:?}",
        emitted.errors
    );
    assert!(
        emitted
            .source
            .contains("harbour_builtin_qout((harbour_runtime_Value[]) { harbour_builtin_str((harbour_runtime_Value[]) { harbour_builtin_log((harbour_runtime_Value[]) { harbour_builtin_val((harbour_runtime_Value[]) { harbour_value_from_string_literal(\"-1\") }, 1) }, 1) }, 1) }, 1);")
    );
    assert!(
        emitted
            .source
            .contains("harbour_builtin_qout((harbour_runtime_Value[]) { harbour_builtin_str((harbour_runtime_Value[]) { harbour_builtin_log((harbour_runtime_Value[]) { harbour_value_from_integer(12LL) }, 1), harbour_value_from_integer(10LL), harbour_value_from_integer(2LL) }, 3) }, 1);")
    );
    assert!(
        emitted
            .source
            .contains("harbour_builtin_qout((harbour_runtime_Value[]) { harbour_builtin_str((harbour_runtime_Value[]) { harbour_builtin_log((harbour_runtime_Value[]) { harbour_builtin_val((harbour_runtime_Value[]) { harbour_value_from_string_literal(\"10\") }, 1) }, 1), harbour_value_from_integer(10LL), harbour_value_from_integer(2LL) }, 3) }, 1);")
    );
}

#[test]
fn emits_int_builtin_fixture_with_runtime_builtin_helper_calls() {
    let emitted = emit_fixture("tests/fixtures/parser/int_builtin.prg");

    assert!(
        emitted.errors.is_empty(),
        "unexpected codegen errors: {:?}",
        emitted.errors
    );
    assert!(
        emitted
            .source
            .contains("harbour_builtin_qout((harbour_runtime_Value[]) { harbour_builtin_int((harbour_runtime_Value[]) { harbour_value_from_integer(0LL) }, 1) }, 1);")
    );
    assert!(
        emitted
            .source
            .contains("harbour_builtin_qout((harbour_runtime_Value[]) { harbour_builtin_int((harbour_runtime_Value[]) { harbour_builtin_val((harbour_runtime_Value[]) { harbour_value_from_string_literal(\"-10.5\") }, 1) }, 1) }, 1);")
    );
}

#[test]
fn emits_round_builtin_fixture_with_runtime_builtin_helper_calls() {
    let emitted = emit_fixture("tests/fixtures/parser/round_builtin.prg");

    assert!(
        emitted.errors.is_empty(),
        "unexpected codegen errors: {:?}",
        emitted.errors
    );
    assert!(
        emitted
            .source
            .contains("harbour_builtin_qout((harbour_runtime_Value[]) { harbour_builtin_round((harbour_runtime_Value[]) { harbour_value_from_float_with_layout(0.5, 1U, 12U), harbour_value_from_integer(0LL) }, 2) }, 1);")
    );
    assert!(
        emitted
            .source
            .contains("harbour_builtin_qout((harbour_runtime_Value[]) { harbour_builtin_round((harbour_runtime_Value[]) { harbour_value_from_integer(50LL), harbour_builtin_val((harbour_runtime_Value[]) { harbour_value_from_string_literal(\"-2\") }, 1) }, 2) }, 1);")
    );
    assert!(
        emitted
            .source
            .contains("harbour_builtin_qout((harbour_runtime_Value[]) { harbour_builtin_round((harbour_runtime_Value[]) { harbour_builtin_val((harbour_runtime_Value[]) { harbour_value_from_string_literal(\"-0.55\") }, 1), harbour_value_from_integer(1LL) }, 2) }, 1);")
    );
}

#[test]
fn emits_mod_builtin_fixture_with_runtime_builtin_helper_calls() {
    let emitted = emit_fixture("tests/fixtures/parser/mod_builtin.prg");

    assert!(
        emitted.errors.is_empty(),
        "unexpected codegen errors: {:?}",
        emitted.errors
    );
    assert!(
        emitted
            .source
            .contains("harbour_builtin_qout((harbour_runtime_Value[]) { harbour_builtin_mod((harbour_runtime_Value[]) { harbour_value_from_integer(100LL), harbour_value_from_integer(60LL), harbour_value_from_string_literal(\"A\") }, 3) }, 1);")
    );
    assert!(
        emitted
            .source
            .contains("harbour_builtin_qout((harbour_runtime_Value[]) { harbour_builtin_mod((harbour_runtime_Value[]) { harbour_builtin_val((harbour_runtime_Value[]) { harbour_value_from_string_literal(\"-1\") }, 1), harbour_value_from_integer(3LL) }, 2) }, 1);")
    );
    assert!(
        emitted
            .source
            .contains("harbour_builtin_qout((harbour_runtime_Value[]) { harbour_builtin_mod((harbour_runtime_Value[]) { harbour_value_from_integer(1LL), harbour_builtin_val((harbour_runtime_Value[]) { harbour_value_from_string_literal(\"-3\") }, 1) }, 2) }, 1);")
    );
}

#[test]
fn emits_str_builtin_fixture_with_runtime_builtin_helper_calls() {
    let emitted = emit_fixture("tests/fixtures/parser/str_builtin.prg");

    assert!(
        emitted.errors.is_empty(),
        "unexpected codegen errors: {:?}",
        emitted.errors
    );
    assert!(
        emitted
            .source
            .contains("harbour_builtin_qout((harbour_runtime_Value[]) { harbour_builtin_str((harbour_runtime_Value[]) { harbour_value_from_integer(10LL) }, 1) }, 1);")
    );
    assert!(
        emitted
            .source
            .contains("harbour_builtin_qout((harbour_runtime_Value[]) { harbour_builtin_str((harbour_runtime_Value[]) { harbour_value_from_float_with_layout(10.6, 1U, 12U), harbour_value_from_integer(5LL) }, 2) }, 1);")
    );
    assert!(
        emitted
            .source
            .contains("harbour_value_from_float_with_layout(5000000000.0, 1U, 12U)")
    );
    assert!(
        emitted
            .source
            .contains("harbour_value_from_float_with_layout(10.0, 1U, 12U)")
    );
    assert!(
        emitted
            .source
            .contains("harbour_value_from_float_with_layout(10.00, 2U, 13U)")
    );
    assert!(
        emitted
            .source
            .contains("harbour_value_from_float_with_layout(10.50, 2U, 13U)")
    );
    assert!(
        emitted
            .source
            .contains("harbour_builtin_qout((harbour_runtime_Value[]) { harbour_builtin_str((harbour_runtime_Value[]) { harbour_value_from_float_with_layout(3.125, 3U, 14U), harbour_value_from_integer(8LL), harbour_value_from_integer(2LL) }, 3) }, 1);")
    );
}

#[test]
fn emits_val_builtin_fixture_with_runtime_builtin_helper_calls() {
    let emitted = emit_fixture("tests/fixtures/parser/val_builtin.prg");

    assert!(
        emitted.errors.is_empty(),
        "unexpected codegen errors: {:?}",
        emitted.errors
    );
    assert!(
        emitted
            .source
            .contains("harbour_builtin_qout((harbour_runtime_Value[]) { harbour_builtin_val((harbour_runtime_Value[]) { harbour_value_from_string_literal(\"10\") }, 1) }, 1);")
    );
    assert!(
        emitted
            .source
            .contains("harbour_builtin_qout((harbour_runtime_Value[]) { harbour_builtin_val((harbour_runtime_Value[]) { harbour_value_from_string_literal(\"15.001 \") }, 1) }, 1);")
    );
}

#[test]
fn emits_valtype_builtin_fixture_with_runtime_builtin_helper_calls() {
    let emitted = emit_fixture("tests/fixtures/parser/valtype_builtin.prg");

    assert!(
        emitted.errors.is_empty(),
        "unexpected codegen errors: {:?}",
        emitted.errors
    );
    assert!(emitted.source.contains(
        "harbour_builtin_qout((harbour_runtime_Value[]) { harbour_builtin_valtype(NULL, 0) }, 1);"
    ));
    assert!(
        emitted
            .source
            .contains("harbour_builtin_qout((harbour_runtime_Value[]) { harbour_builtin_valtype((harbour_runtime_Value[]) { names }, 1) }, 1);")
    );
}

#[test]
fn emits_type_builtin_fixture_with_runtime_builtin_helper_calls() {
    let emitted = emit_fixture("tests/fixtures/parser/type_builtin.prg");

    assert!(
        emitted.errors.is_empty(),
        "unexpected codegen errors: {:?}",
        emitted.errors
    );
    assert!(
        emitted
            .source
            .contains("harbour_builtin_qout((harbour_runtime_Value[]) { harbour_builtin_type((harbour_runtime_Value[]) { harbour_value_from_string_literal(\"NIL\") }, 1) }, 1);")
    );
    assert!(
        emitted
            .source
            .contains("harbour_builtin_qout((harbour_runtime_Value[]) { harbour_builtin_type((harbour_runtime_Value[]) { harbour_value_from_string_literal(\"{ 1, 2 }\") }, 1) }, 1);")
    );
}

#[test]
fn emits_max_min_builtin_fixture_with_runtime_builtin_helper_calls() {
    let emitted = emit_fixture("tests/fixtures/parser/max_min_builtin.prg");

    assert!(
        emitted.errors.is_empty(),
        "unexpected codegen errors: {:?}",
        emitted.errors
    );
    assert!(
        emitted
            .source
            .contains("harbour_builtin_qout((harbour_runtime_Value[]) { harbour_builtin_max((harbour_runtime_Value[]) { harbour_value_from_integer(10LL), harbour_value_from_integer(5LL) }, 2) }, 1);")
    );
    assert!(
        emitted
            .source
            .contains("harbour_builtin_qout((harbour_runtime_Value[]) { harbour_builtin_min((harbour_runtime_Value[]) { harbour_value_from_logical(false), harbour_value_from_logical(true) }, 2) }, 1);")
    );
}

#[test]
fn emits_empty_builtin_fixture_with_runtime_builtin_helper_calls() {
    let emitted = emit_fixture("tests/fixtures/parser/empty_builtin.prg");

    assert!(
        emitted.errors.is_empty(),
        "unexpected codegen errors: {:?}",
        emitted.errors
    );
    assert!(emitted.source.contains(
        "harbour_builtin_qout((harbour_runtime_Value[]) { harbour_builtin_empty(NULL, 0) }, 1);"
    ));
    assert!(
        emitted
            .source
            .contains("harbour_builtin_qout((harbour_runtime_Value[]) { harbour_builtin_empty((harbour_runtime_Value[]) { empty_items }, 1) }, 1);")
    );
    assert!(
        emitted
            .source
            .contains("harbour_builtin_qout((harbour_runtime_Value[]) { harbour_builtin_empty((harbour_runtime_Value[]) { filled_items }, 1) }, 1);")
    );
}

#[test]
fn emits_substr_builtin_fixture_with_runtime_builtin_helper_calls() {
    let emitted = emit_fixture("tests/fixtures/parser/substr_builtin.prg");

    assert!(
        emitted.errors.is_empty(),
        "unexpected codegen errors: {:?}",
        emitted.errors
    );
    assert!(
        emitted
            .source
            .contains("harbour_builtin_qout((harbour_runtime_Value[]) { harbour_builtin_substr((harbour_runtime_Value[]) { harbour_value_from_string_literal(\"abcdef\"), harbour_value_from_integer(0LL), harbour_value_from_integer(1LL) }, 3) }, 1);")
    );
    assert!(
        emitted
            .source
            .contains("harbour_builtin_qout((harbour_runtime_Value[]) { harbour_builtin_substr((harbour_runtime_Value[]) { harbour_value_from_string_literal(\"abcdef\"), harbour_value_from_integer(10LL) }, 2) }, 1);")
    );
}

#[test]
fn emits_left_right_builtin_fixture_with_runtime_builtin_helper_calls() {
    let emitted = emit_fixture("tests/fixtures/parser/left_right_builtin.prg");

    assert!(
        emitted.errors.is_empty(),
        "unexpected codegen errors: {:?}",
        emitted.errors
    );
    assert!(
        emitted
            .source
            .contains("harbour_builtin_qout((harbour_runtime_Value[]) { harbour_builtin_left((harbour_runtime_Value[]) { harbour_value_from_string_literal(\"abcdef\"), harbour_value_from_integer(2LL) }, 2) }, 1);")
    );
    assert!(
        emitted
            .source
            .contains("harbour_builtin_qout((harbour_runtime_Value[]) { harbour_builtin_right((harbour_runtime_Value[]) { harbour_value_from_string_literal(\"abcdef\"), harbour_value_from_integer(10LL) }, 2) }, 1);")
    );
}

#[test]
fn emits_upper_lower_builtin_fixture_with_runtime_builtin_helper_calls() {
    let emitted = emit_fixture("tests/fixtures/parser/upper_lower_builtin.prg");

    assert!(
        emitted.errors.is_empty(),
        "unexpected codegen errors: {:?}",
        emitted.errors
    );
    assert!(
        emitted
            .source
            .contains("harbour_builtin_qout((harbour_runtime_Value[]) { harbour_builtin_upper((harbour_runtime_Value[]) { harbour_value_from_string_literal(\"aAZAZa\") }, 1) }, 1);")
    );
    assert!(
        emitted
            .source
            .contains("harbour_builtin_qout((harbour_runtime_Value[]) { harbour_builtin_lower((harbour_runtime_Value[]) { harbour_value_from_string_literal(\"AazazA\") }, 1) }, 1);")
    );
}

#[test]
fn emits_trim_builtin_fixture_with_runtime_builtin_helper_calls() {
    let emitted = emit_fixture("tests/fixtures/parser/trim_builtin.prg");

    assert!(
        emitted.errors.is_empty(),
        "unexpected codegen errors: {:?}",
        emitted.errors
    );
    assert!(
        emitted
            .source
            .contains("harbour_builtin_qout((harbour_runtime_Value[]) { harbour_builtin_trim((harbour_runtime_Value[]) { harbour_value_from_string_literal(\"UA   \") }, 1) }, 1);")
    );
    assert!(
        emitted
            .source
            .contains("harbour_builtin_qout((harbour_runtime_Value[]) { harbour_builtin_ltrim((harbour_runtime_Value[]) { harbour_value_from_string_literal(\"   UA  \") }, 1) }, 1);")
    );
    assert!(
        emitted
            .source
            .contains("harbour_builtin_qout((harbour_runtime_Value[]) { harbour_builtin_rtrim((harbour_runtime_Value[]) { harbour_value_from_string_literal(\"   UA  \") }, 1) }, 1);")
    );
}

#[test]
fn emits_at_builtin_fixture_with_runtime_builtin_helper_calls() {
    let emitted = emit_fixture("tests/fixtures/parser/at_builtin.prg");

    assert!(
        emitted.errors.is_empty(),
        "unexpected codegen errors: {:?}",
        emitted.errors
    );
    assert!(
        emitted
            .source
            .contains("harbour_builtin_qout((harbour_runtime_Value[]) { harbour_builtin_at((harbour_runtime_Value[]) { harbour_value_from_string_literal(\"\"), harbour_value_from_string_literal(\"\") }, 2) }, 1);")
    );
    assert!(
        emitted
            .source
            .contains("harbour_builtin_qout((harbour_runtime_Value[]) { harbour_builtin_at((harbour_runtime_Value[]) { harbour_value_from_string_literal(\"AB\"), harbour_value_from_string_literal(\"AAB\") }, 2) }, 1);")
    );
}

#[test]
fn emits_replicate_space_builtin_fixture_with_runtime_builtin_helper_calls() {
    let emitted = emit_fixture("tests/fixtures/parser/replicate_space_builtin.prg");

    assert!(
        emitted.errors.is_empty(),
        "unexpected codegen errors: {:?}",
        emitted.errors
    );
    assert!(
        emitted
            .source
            .contains("harbour_builtin_qout((harbour_runtime_Value[]) { harbour_builtin_replicate((harbour_runtime_Value[]) { harbour_value_from_string_literal(\"HE\"), harbour_value_from_float_with_layout(3.7, 1U, 12U) }, 2) }, 1);")
    );
    assert!(
        emitted
            .source
            .contains("harbour_builtin_qout((harbour_runtime_Value[]) { harbour_builtin_space((harbour_runtime_Value[]) { harbour_value_from_float_with_layout(3.7, 1U, 12U) }, 1) }, 1);")
    );
}

#[test]
fn emits_static_fixture_with_persistent_c_storage() {
    let emitted = emit_fixture("tests/fixtures/parser/static.prg");

    assert!(
        emitted.errors.is_empty(),
        "unexpected codegen errors: {:?}",
        emitted.errors
    );
    assert!(
        emitted
            .source
            .contains("static harbour_runtime_Value harbour_static_main_cache;")
    );
    assert!(
        emitted
            .source
            .contains("static harbour_runtime_Value harbour_static_main_hits;")
    );
    assert!(
        emitted
            .source
            .contains("if (!harbour_static_main_cache__initialized) {")
    );
    assert!(
        emitted
            .source
            .contains("harbour_static_main_cache = harbour_value_from_string_literal(\"memo\");")
    );
    assert!(
        emitted
            .source
            .contains("harbour_static_main_hits = harbour_value_from_integer(0LL);")
    );
    assert!(emitted.source.contains("return harbour_static_main_cache;"));
}

#[test]
fn emits_module_static_fixture_with_shared_storage_and_init_helper() {
    let emitted = emit_fixture("tests/fixtures/parser/static_module.prg");

    assert!(
        emitted.errors.is_empty(),
        "unexpected codegen errors: {:?}",
        emitted.errors
    );
    assert!(
        emitted
            .source
            .contains("static harbour_runtime_Value harbour_static_s_count;")
    );
    assert!(
        emitted
            .source
            .contains("static harbour_runtime_Value harbour_static_s_cache;")
    );
    assert!(
        emitted
            .source
            .contains("static void harbour_module_init_statics(void) {")
    );
    assert!(
        emitted
            .source
            .contains("harbour_static_s_count = harbour_value_from_integer(0LL);")
    );
    assert!(
        emitted
            .source
            .contains("harbour_static_s_cache = harbour_value_nil();")
    );
    assert!(emitted.source.contains("harbour_module_init_statics();"));
    assert!(emitted.source.contains("harbour_static_s_count = harbour_value_add(harbour_static_s_count, harbour_value_from_integer(1LL));"));
    assert!(emitted.source.contains(
        "return harbour_builtin_valtype((harbour_runtime_Value[]) { harbour_static_s_cache }, 1);"
    ));
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
    assert!(
        emitted
            .source
            .contains("extern harbour_runtime_Value harbour_builtin_len(const harbour_runtime_Value *arguments, size_t argument_count);")
    );
    assert!(
        emitted
            .source
            .contains("extern harbour_runtime_Value harbour_builtin_abs(const harbour_runtime_Value *arguments, size_t argument_count);")
    );
    assert!(
        emitted
            .source
            .contains("extern harbour_runtime_Value harbour_builtin_int(const harbour_runtime_Value *arguments, size_t argument_count);")
    );
    assert!(
        emitted
            .source
            .contains("extern harbour_runtime_Value harbour_builtin_round(const harbour_runtime_Value *arguments, size_t argument_count);")
    );
    assert!(
        emitted
            .source
            .contains("extern harbour_runtime_Value harbour_builtin_mod(const harbour_runtime_Value *arguments, size_t argument_count);")
    );
    assert!(
        emitted
            .source
            .contains("extern harbour_runtime_Value harbour_builtin_str(const harbour_runtime_Value *arguments, size_t argument_count);")
    );
    assert!(
        emitted
            .source
            .contains("extern harbour_runtime_Value harbour_builtin_val(const harbour_runtime_Value *arguments, size_t argument_count);")
    );
    assert!(
        emitted
            .source
            .contains("extern harbour_runtime_Value harbour_builtin_valtype(const harbour_runtime_Value *arguments, size_t argument_count);")
    );
    assert!(
        emitted
            .source
            .contains("extern harbour_runtime_Value harbour_builtin_type(const harbour_runtime_Value *arguments, size_t argument_count);")
    );
    assert!(
        emitted
            .source
            .contains("extern harbour_runtime_Value harbour_builtin_substr(const harbour_runtime_Value *arguments, size_t argument_count);")
    );
    assert!(
        emitted
            .source
            .contains("extern harbour_runtime_Value harbour_builtin_left(const harbour_runtime_Value *arguments, size_t argument_count);")
    );
    assert!(
        emitted
            .source
            .contains("extern harbour_runtime_Value harbour_builtin_right(const harbour_runtime_Value *arguments, size_t argument_count);")
    );
    assert!(
        emitted
            .source
            .contains("extern harbour_runtime_Value harbour_builtin_upper(const harbour_runtime_Value *arguments, size_t argument_count);")
    );
    assert!(
        emitted
            .source
            .contains("extern harbour_runtime_Value harbour_builtin_lower(const harbour_runtime_Value *arguments, size_t argument_count);")
    );
    assert!(
        emitted
            .source
            .contains("extern harbour_runtime_Value harbour_builtin_trim(const harbour_runtime_Value *arguments, size_t argument_count);")
    );
    assert!(
        emitted
            .source
            .contains("extern harbour_runtime_Value harbour_builtin_ltrim(const harbour_runtime_Value *arguments, size_t argument_count);")
    );
    assert!(
        emitted
            .source
            .contains("extern harbour_runtime_Value harbour_builtin_rtrim(const harbour_runtime_Value *arguments, size_t argument_count);")
    );
    assert!(
        emitted
            .source
            .contains("extern harbour_runtime_Value harbour_builtin_replicate(const harbour_runtime_Value *arguments, size_t argument_count);")
    );
    assert!(
        emitted
            .source
            .contains("extern harbour_runtime_Value harbour_builtin_space(const harbour_runtime_Value *arguments, size_t argument_count);")
    );
    assert!(
        emitted
            .source
            .contains("extern harbour_runtime_Value harbour_builtin_aclone(const harbour_runtime_Value *arguments, size_t argument_count);")
    );
    assert!(
        emitted
            .source
            .contains("extern harbour_runtime_Value harbour_builtin_aadd(harbour_runtime_Value *array, const harbour_runtime_Value *arguments, size_t argument_count);")
    );
    assert!(
        emitted
            .source
            .contains("extern harbour_runtime_Value harbour_builtin_asize(harbour_runtime_Value *array, const harbour_runtime_Value *arguments, size_t argument_count);")
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

#[test]
fn emits_aclones_fixture_with_runtime_builtin_helper_calls() {
    let emitted = emit_fixture("tests/fixtures/parser/aclone.prg");

    assert!(
        emitted.errors.is_empty(),
        "unexpected codegen errors: {:?}",
        emitted.errors
    );
    assert!(
        emitted
            .source
            .contains("harbour_runtime_Value source = harbour_value_from_array_items((harbour_runtime_Value[]) { harbour_value_from_integer(1LL), harbour_value_from_array_items((harbour_runtime_Value[]) { harbour_value_from_integer(2LL) }, 1) }, 2);")
    );
    assert!(
        emitted
            .source
            .contains("harbour_builtin_qout((harbour_runtime_Value[]) { harbour_builtin_aclone((harbour_runtime_Value[]) { source }, 1) }, 1);")
    );
}

#[test]
fn emits_mutable_builtins_fixture_with_addressable_runtime_helper_calls() {
    let emitted = emit_fixture("tests/fixtures/parser/mutable_builtins.prg");

    assert!(
        emitted.errors.is_empty(),
        "unexpected codegen errors: {:?}",
        emitted.errors
    );
    assert!(
        emitted
            .source
            .contains("harbour_builtin_qout((harbour_runtime_Value[]) { harbour_builtin_aadd(&items, (harbour_runtime_Value[]) { harbour_value_from_integer(7LL) }, 1) }, 1);")
    );
    assert!(
        emitted
            .source
            .contains("harbour_builtin_qout((harbour_runtime_Value[]) { harbour_builtin_asize(&items, (harbour_runtime_Value[]) { harbour_value_from_integer(3LL) }, 1) }, 1);")
    );
}

#[test]
fn emits_array_builtins_fixture_with_runtime_array_helper_calls() {
    let emitted = emit_fixture("tests/fixtures/parser/array_builtins.prg");

    assert!(
        emitted.errors.is_empty(),
        "unexpected codegen errors: {:?}",
        emitted.errors
    );
    assert!(
        emitted
            .source
            .contains("harbour_builtin_ascan((harbour_runtime_Value[]) { values, harbour_value_from_integer(20LL) }, 2)")
    );
    assert!(
        emitted
            .source
            .contains("harbour_builtin_ascan((harbour_runtime_Value[]) { words, harbour_value_from_string_literal(\"HELL\") }, 2)")
    );
    assert!(
        emitted
            .source
            .contains("harbour_builtin_ains(&values, (harbour_runtime_Value[]) { harbour_value_from_integer(2LL) }, 1)")
    );
    assert!(
        emitted
            .source
            .contains("harbour_builtin_adel(&values, (harbour_runtime_Value[]) { harbour_value_from_integer(1LL) }, 1)")
    );
}

#[test]
fn emits_array_args_fixture_with_array_argument_and_index_helpers() {
    let emitted = emit_fixture("tests/fixtures/parser/array_args.prg");

    assert!(
        emitted.errors.is_empty(),
        "unexpected codegen errors: {:?}",
        emitted.errors
    );
    assert!(emitted.source.contains("harbour_routine_showarray(items);"));
    assert!(
        emitted
            .source
            .contains("harbour_builtin_len((harbour_runtime_Value[]) { items }, 1)")
    );
    assert!(
        emitted
            .source
            .contains("harbour_value_array_get(items, harbour_value_from_integer(2LL))")
    );
}

#[test]
fn emits_array_matrix_fixture_with_nested_array_helpers() {
    let emitted = emit_fixture("tests/fixtures/parser/array_matrix.prg");

    assert!(
        emitted.errors.is_empty(),
        "unexpected codegen errors: {:?}",
        emitted.errors
    );
    assert!(emitted.source.contains(
        "harbour_value_from_array_items((harbour_runtime_Value[]) { harbour_value_from_integer(1LL), harbour_value_from_integer(2LL) }, 2)"
    ));
    assert!(emitted.source.contains(
        "harbour_value_array_get(harbour_value_array_get(matrix, harbour_value_from_integer(1LL)), harbour_value_from_integer(2LL))"
    ));
    assert!(emitted.source.contains(
        "harbour_value_array_set_path(&matrix, (harbour_runtime_Value[]) { harbour_value_from_integer(2LL), harbour_value_from_integer(1LL) }, 2, harbour_value_from_integer(99LL));"
    ));
}

#[test]
fn emits_compare_ops_fixture_with_runtime_comparison_helpers() {
    let emitted = emit_fixture("tests/fixtures/parser/compare_ops.prg");

    assert!(
        emitted.errors.is_empty(),
        "unexpected codegen errors: {:?}",
        emitted.errors
    );
    assert!(
        emitted
            .source
            .contains("harbour_builtin_qout((harbour_runtime_Value[]) { harbour_value_exact_equals(same, same) }, 1);")
    );
    assert!(emitted.source.contains(
        "harbour_builtin_qout((harbour_runtime_Value[]) { harbour_value_equals(same, other) }, 1);"
    ));
    assert!(
        emitted
            .source
            .contains("harbour_builtin_qout((harbour_runtime_Value[]) { harbour_value_not_equals(same, other) }, 1);")
    );
    assert!(
        emitted
            .source
            .contains("harbour_builtin_qout((harbour_runtime_Value[]) { harbour_value_greater_than(same, other) }, 1);")
    );
    assert!(
        emitted
            .source
            .contains("harbour_builtin_qout((harbour_runtime_Value[]) { harbour_value_greater_than_or_equal(same, other) }, 1);")
    );
}
