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
