use std::{fs, path::PathBuf};

use harbour_rust_hir::lower_program as lower_hir_program;
use harbour_rust_ir::{Builtin, Expression, ReturnStatement, Statement, lower_program};
use harbour_rust_parser::parse;

fn workspace_fixture(path: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join(path)
}

fn lower_fixture(path: &str) -> harbour_rust_ir::LoweringOutput {
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

    lower_program(&hir.program)
}

#[test]
fn lowers_hello_fixture_to_builtin_print_ir() {
    let lowered = lower_fixture("tests/fixtures/parser/hello.prg");
    assert!(
        lowered.errors.is_empty(),
        "unexpected ir lowering errors: {:?}",
        lowered.errors
    );

    assert_eq!(lowered.program.routines.len(), 1);
    assert_eq!(lowered.program.routines[0].name.text, "Main");
    assert!(matches!(
        lowered.program.routines[0].body[0],
        Statement::BuiltinCall(ref statement)
            if statement.builtin == Builtin::QOut && statement.arguments.len() == 1
    ));
}

#[test]
fn lowers_while_fixture_without_ir_errors() {
    let lowered = lower_fixture("tests/fixtures/parser/while.prg");
    assert!(
        lowered.errors.is_empty(),
        "unexpected ir lowering errors: {:?}",
        lowered.errors
    );

    assert_eq!(lowered.program.routines.len(), 1);
    assert_eq!(lowered.program.routines[0].body.len(), 3);
    assert!(matches!(
        lowered.program.routines[0].body[1],
        Statement::DoWhile(_)
    ));
}

#[test]
fn reports_arrays_fixture_as_ir_placeholder_error() {
    let lowered = lower_fixture("tests/fixtures/parser/arrays.prg");
    assert_eq!(lowered.errors.len(), 1);
    assert_eq!(
        lowered.errors[0].message,
        "array literals are not supported in IR yet"
    );
}

#[test]
fn lowers_indexing_fixture_to_explicit_ir_index_nodes() {
    let lowered = lower_fixture("tests/fixtures/parser/indexing.prg");
    assert_eq!(lowered.errors.len(), 1);
    assert_eq!(
        lowered.errors[0].message,
        "array literals are not supported in IR yet"
    );

    let Statement::Return(ReturnStatement {
        value: Some(expression),
        ..
    }) = &lowered.program.routines[0].body[1]
    else {
        panic!("expected return statement with indexing expression");
    };

    let Expression::Index(outer_index) = expression else {
        panic!("expected outer IR index expression");
    };
    let Expression::Index(inner_index) = outer_index.target.as_ref() else {
        panic!("expected nested IR index expression");
    };
    assert!(matches!(inner_index.target.as_ref(), Expression::Symbol(_)));
    assert_eq!(inner_index.indices.len(), 1);
    assert!(matches!(inner_index.indices[0], Expression::Symbol(_)));
    assert_eq!(outer_index.indices.len(), 1);
    assert!(matches!(outer_index.indices[0], Expression::Binary(_)));
}
