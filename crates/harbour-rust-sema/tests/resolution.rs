use std::{fs, path::PathBuf};

use harbour_rust_hir::lower_program;
use harbour_rust_parser::parse;
use harbour_rust_sema::{Binding, LocalSymbolKind, analyze_program};

fn workspace_fixture(path: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join(path)
}

fn analyze_fixture(path: &str) -> harbour_rust_sema::Analysis {
    let source = fs::read_to_string(workspace_fixture(path)).expect("fixture source");
    let parsed = parse(&source);
    assert!(
        parsed.errors.is_empty(),
        "unexpected parse errors: {:?}",
        parsed.errors
    );
    let lowered = lower_program(&parsed.program);
    assert!(
        lowered.errors.is_empty(),
        "unexpected lowering errors: {:?}",
        lowered.errors
    );
    analyze_program(&lowered.program)
}

#[test]
fn analyzes_hello_fixture_without_semantic_errors() {
    let analysis = analyze_fixture("tests/fixtures/parser/hello.prg");
    assert!(
        analysis.errors.is_empty(),
        "unexpected semantic errors: {:?}",
        analysis.errors
    );
    assert_eq!(analysis.routine_symbols.len(), 1);
    assert_eq!(analysis.routine_symbols[0].name, "Main");
    assert_eq!(analysis.routines[0].locals, Vec::new());
}

#[test]
fn analyzes_while_fixture_resolving_local_x() {
    let analysis = analyze_fixture("tests/fixtures/parser/while.prg");
    assert!(
        analysis.errors.is_empty(),
        "unexpected semantic errors: {:?}",
        analysis.errors
    );
    assert_eq!(analysis.routines.len(), 1);
    assert_eq!(analysis.routines[0].locals.len(), 1);
    assert_eq!(analysis.routines[0].locals[0].kind, LocalSymbolKind::Local);
    assert_eq!(analysis.routines[0].locals[0].name, "x");
    assert_eq!(analysis.routines[0].resolutions.len(), 2);
    assert_eq!(
        analysis.routines[0].resolutions[0].binding,
        Binding::Local(0)
    );
    assert_eq!(
        analysis.routines[0].resolutions[1].binding,
        Binding::Local(0)
    );
}
