use std::{fs, path::PathBuf};

use harbour_rust_hir::{StorageClass, lower_program};
use harbour_rust_parser::parse;

fn workspace_fixture(path: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join(path)
}

fn lower_fixture(path: &str) -> harbour_rust_hir::LoweringOutput {
    let source = fs::read_to_string(workspace_fixture(path)).expect("fixture source");
    let parsed = parse(&source);
    assert!(
        parsed.errors.is_empty(),
        "unexpected parse errors: {:?}",
        parsed.errors
    );
    lower_program(&parsed.program)
}

#[test]
fn lowers_hello_fixture_without_errors() {
    let lowered = lower_fixture("tests/fixtures/parser/hello.prg");
    assert!(
        lowered.errors.is_empty(),
        "unexpected lowering errors: {:?}",
        lowered.errors
    );
    assert_eq!(lowered.program.routines.len(), 1);
    assert_eq!(lowered.program.routines[0].name.text, "Main");
}

#[test]
fn lowers_while_fixture_without_errors() {
    let lowered = lower_fixture("tests/fixtures/parser/while.prg");
    assert!(
        lowered.errors.is_empty(),
        "unexpected lowering errors: {:?}",
        lowered.errors
    );
    assert_eq!(lowered.program.routines.len(), 1);
    assert_eq!(lowered.program.routines[0].body.len(), 3);
}

#[test]
fn lowers_static_fixture_with_static_storage_placeholder() {
    let lowered = lower_fixture("tests/fixtures/parser/static.prg");
    assert!(
        lowered.errors.is_empty(),
        "unexpected lowering errors: {:?}",
        lowered.errors
    );

    let declaration = match &lowered.program.routines[0].body[0] {
        harbour_rust_hir::Statement::Local(statement) => statement,
        statement => panic!("expected local-like declaration placeholder, found {statement:?}"),
    };

    assert_eq!(declaration.storage_class, StorageClass::Static);
    assert_eq!(declaration.bindings.len(), 2);
    assert_eq!(declaration.bindings[0].name.text, "cache");
    assert_eq!(declaration.bindings[1].name.text, "hits");
}

#[test]
fn lowers_arrays_fixture_without_hir_errors() {
    let lowered = lower_fixture("tests/fixtures/parser/arrays.prg");
    assert!(
        lowered.errors.is_empty(),
        "unexpected lowering errors: {:?}",
        lowered.errors
    );

    let return_value = match &lowered.program.routines[0].body[0] {
        harbour_rust_hir::Statement::Return(statement) => statement.value.as_ref(),
        statement => panic!("expected return statement, found {statement:?}"),
    };

    let Some(harbour_rust_hir::Expression::Array(array)) = return_value else {
        panic!("expected array literal in lowered HIR");
    };

    assert_eq!(array.elements.len(), 2);
    assert!(matches!(
        array.elements[0],
        harbour_rust_hir::Expression::Array(ref nested) if nested.elements.is_empty()
    ));
    assert!(matches!(
        array.elements[1],
        harbour_rust_hir::Expression::Array(ref nested) if nested.elements.len() == 3
    ));
}

#[test]
fn lowers_compound_assignment_fixture_to_assign_and_binary_nodes() {
    let lowered = lower_fixture("tests/fixtures/parser/compound_assign.prg");
    assert!(
        lowered.errors.is_empty(),
        "unexpected lowering errors: {:?}",
        lowered.errors
    );

    let body = &lowered.program.routines[0].body;
    assert_eq!(body.len(), 5);

    let static_declaration = match &body[1] {
        harbour_rust_hir::Statement::Local(statement) => statement,
        statement => panic!("expected static-like declaration, found {statement:?}"),
    };
    assert_eq!(static_declaration.storage_class, StorageClass::Static);
    assert_eq!(static_declaration.bindings[0].name.text, "factor");

    let first_assignment = match &body[2] {
        harbour_rust_hir::Statement::Evaluate(statement) => &statement.expression,
        statement => panic!("expected evaluation statement, found {statement:?}"),
    };
    let harbour_rust_hir::Expression::Assign(assign) = first_assignment else {
        panic!("expected lowered assign expression, found {first_assignment:?}");
    };
    assert!(matches!(
        assign.target,
        harbour_rust_hir::AssignTarget::Symbol(ref symbol) if symbol.text == "total"
    ));

    let harbour_rust_hir::Expression::Binary(binary) = assign.value.as_ref() else {
        panic!("expected binary expression in compound assignment");
    };
    assert_eq!(binary.operator, harbour_rust_hir::BinaryOperator::Add);
    assert!(matches!(
        binary.left.as_ref(),
        harbour_rust_hir::Expression::Symbol(symbol) if symbol.text == "total"
    ));
    assert!(matches!(
        binary.right.as_ref(),
        harbour_rust_hir::Expression::Integer(literal) if literal.lexeme == "3"
    ));

    let second_assignment = match &body[3] {
        harbour_rust_hir::Statement::Evaluate(statement) => &statement.expression,
        statement => panic!("expected evaluation statement, found {statement:?}"),
    };
    let harbour_rust_hir::Expression::Assign(assign) = second_assignment else {
        panic!("expected lowered assign expression, found {second_assignment:?}");
    };
    assert!(matches!(
        assign.target,
        harbour_rust_hir::AssignTarget::Symbol(ref symbol) if symbol.text == "factor"
    ));

    let harbour_rust_hir::Expression::Binary(binary) = assign.value.as_ref() else {
        panic!("expected binary expression in compound assignment");
    };
    assert_eq!(binary.operator, harbour_rust_hir::BinaryOperator::Multiply);
    assert!(matches!(
        binary.left.as_ref(),
        harbour_rust_hir::Expression::Symbol(symbol) if symbol.text == "factor"
    ));
    assert!(matches!(
        binary.right.as_ref(),
        harbour_rust_hir::Expression::Symbol(symbol) if symbol.text == "total"
    ));
}

#[test]
fn lowers_indexing_fixture_to_explicit_hir_index_nodes() {
    let lowered = lower_fixture("tests/fixtures/parser/indexing.prg");
    assert!(
        lowered.errors.is_empty(),
        "unexpected lowering errors: {:?}",
        lowered.errors
    );

    let body = &lowered.program.routines[0].body;
    assert_eq!(body.len(), 2);

    let return_value = match &body[1] {
        harbour_rust_hir::Statement::Return(statement) => statement.value.as_ref(),
        statement => panic!("expected return statement, found {statement:?}"),
    };

    let Some(harbour_rust_hir::Expression::Index(outer_index)) = return_value else {
        panic!("expected outer index expression");
    };
    assert_eq!(outer_index.indices.len(), 1);
    assert!(matches!(
        outer_index.indices[0],
        harbour_rust_hir::Expression::Binary(ref binary)
            if binary.operator == harbour_rust_hir::BinaryOperator::Add
    ));

    let harbour_rust_hir::Expression::Index(inner_index) = outer_index.target.as_ref() else {
        panic!("expected nested index expression");
    };
    assert_eq!(inner_index.indices.len(), 1);
    assert!(matches!(
        inner_index.indices[0],
        harbour_rust_hir::Expression::Symbol(ref symbol) if symbol.text == "row"
    ));
    assert!(matches!(
        inner_index.target.as_ref(),
        harbour_rust_hir::Expression::Symbol(symbol) if symbol.text == "matrix"
    ));
}

#[test]
fn lowers_indexed_assignment_fixture_to_flat_assign_target() {
    let lowered = lower_fixture("tests/fixtures/parser/indexed_assign.prg");
    assert!(
        lowered.errors.is_empty(),
        "unexpected lowering errors: {:?}",
        lowered.errors
    );

    let body = &lowered.program.routines[0].body;
    assert_eq!(body.len(), 4);

    let assignment = match &body[1] {
        harbour_rust_hir::Statement::Evaluate(statement) => &statement.expression,
        statement => panic!("expected evaluation statement, found {statement:?}"),
    };

    let harbour_rust_hir::Expression::Assign(assign) = assignment else {
        panic!("expected assign expression, found {assignment:?}");
    };

    let harbour_rust_hir::AssignTarget::Index(target) = &assign.target else {
        panic!("expected indexed assign target");
    };

    assert_eq!(target.root.text, "matrix");
    assert_eq!(target.indices.len(), 2);
    assert!(matches!(
        target.indices[0],
        harbour_rust_hir::Expression::Integer(ref literal) if literal.lexeme == "2"
    ));
    assert!(matches!(
        target.indices[1],
        harbour_rust_hir::Expression::Integer(ref literal) if literal.lexeme == "1"
    ));
}
