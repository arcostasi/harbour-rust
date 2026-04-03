use std::{fs, path::PathBuf};

use harbour_rust_hir::lower_program as lower_hir_program;
use harbour_rust_ir::{
    AssignTarget, Builtin, Expression, ReadPath, ReturnStatement, Statement, lower_program,
};
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
fn lowers_arrays_fixture_to_explicit_ir_array_nodes() {
    let lowered = lower_fixture("tests/fixtures/parser/arrays.prg");
    assert!(
        lowered.errors.is_empty(),
        "unexpected ir lowering errors: {:?}",
        lowered.errors
    );

    let Statement::Return(ReturnStatement {
        value: Some(expression),
        ..
    }) = &lowered.program.routines[0].body[0]
    else {
        panic!("expected return statement with array expression");
    };

    let Expression::Array(outer_array) = expression else {
        panic!("expected outer IR array literal");
    };
    assert_eq!(outer_array.elements.len(), 2);
    assert!(matches!(outer_array.elements[0], Expression::Array(_)));
    let Expression::Array(inner_array) = &outer_array.elements[1] else {
        panic!("expected nested array literal");
    };
    assert_eq!(inner_array.elements.len(), 3);
    assert!(matches!(inner_array.elements[0], Expression::Integer(_)));
    assert!(matches!(inner_array.elements[1], Expression::String(_)));
    assert!(matches!(inner_array.elements[2], Expression::Read(_)));
}

#[test]
fn lowers_static_fixture_to_explicit_ir_static_nodes() {
    let lowered = lower_fixture("tests/fixtures/parser/static.prg");
    assert!(
        lowered.errors.is_empty(),
        "unexpected ir lowering errors: {:?}",
        lowered.errors
    );

    let body = &lowered.program.routines[0].body;
    assert_eq!(body.len(), 2);

    let Statement::Static(statement) = &body[0] else {
        panic!("expected explicit IR static statement");
    };
    assert_eq!(statement.bindings.len(), 2);
    assert_eq!(statement.bindings[0].name.text, "cache");
    assert_eq!(statement.bindings[1].name.text, "hits");

    let Statement::Return(ReturnStatement {
        value: Some(Expression::Read(read)),
        ..
    }) = &body[1]
    else {
        panic!("expected return with explicit IR read");
    };
    assert!(matches!(
        &read.path,
        ReadPath::Name(symbol) if symbol.text == "cache"
    ));
}

#[test]
fn lowers_module_static_fixture_to_program_level_ir_statics() {
    let lowered = lower_fixture("tests/fixtures/parser/static_module.prg");
    assert!(
        lowered.errors.is_empty(),
        "unexpected ir lowering errors: {:?}",
        lowered.errors
    );

    assert_eq!(lowered.program.module_statics.len(), 2);
    assert_eq!(
        lowered.program.module_statics[0].bindings[0].name.text,
        "s_count"
    );
    assert_eq!(
        lowered.program.module_statics[1].bindings[0].name.text,
        "s_cache"
    );
    assert_eq!(lowered.program.routines.len(), 3);
}

#[test]
fn lowers_indexing_fixture_to_explicit_ir_index_nodes() {
    let lowered = lower_fixture("tests/fixtures/parser/indexing.prg");
    assert!(
        lowered.errors.is_empty(),
        "unexpected ir lowering errors: {:?}",
        lowered.errors
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
    assert!(matches!(inner_index.target.as_ref(), Expression::Read(_)));
    assert_eq!(inner_index.indices.len(), 1);
    assert!(matches!(inner_index.indices[0], Expression::Read(_)));
    assert_eq!(outer_index.indices.len(), 1);
    assert!(matches!(outer_index.indices[0], Expression::Binary(_)));
}

#[test]
fn lowers_indexed_assignment_fixture_to_explicit_ir_assign_target() {
    let lowered = lower_fixture("tests/fixtures/parser/indexed_assign.prg");
    assert!(
        lowered.errors.is_empty(),
        "unexpected ir lowering errors: {:?}",
        lowered.errors
    );

    let body = &lowered.program.routines[0].body;
    assert_eq!(body.len(), 4);

    let Statement::Assign(assign) = &body[1] else {
        panic!("expected assign statement");
    };

    let AssignTarget::Index(target) = &assign.target else {
        panic!("expected indexed assign target");
    };

    assert_eq!(target.root.text, "matrix");
    assert_eq!(target.indices.len(), 2);
    assert!(matches!(
        target.indices[0],
        Expression::Integer(ref literal) if literal.lexeme == "2"
    ));
    assert!(matches!(
        target.indices[1],
        Expression::Integer(ref literal) if literal.lexeme == "1"
    ));
}
