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

fn assert_fixture_errors(source_path: &str, expected_path: &str) {
    let analysis = analyze_fixture(source_path);
    let expected = fs::read_to_string(workspace_fixture(expected_path)).expect("fixture snapshot");
    assert_eq!(harbour_rust_sema::render_errors(&analysis), expected);
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

#[test]
fn reports_unresolved_locals_across_control_flow_fixture() {
    assert_fixture_errors(
        "tests/fixtures/sema/control_flow_missing_locals.prg",
        "tests/fixtures/sema/control_flow_missing_locals.errors",
    );
}

#[test]
fn reports_unresolved_callables_across_control_flow_fixture() {
    assert_fixture_errors(
        "tests/fixtures/sema/control_flow_missing_callables.prg",
        "tests/fixtures/sema/control_flow_missing_callables.errors",
    );
}

#[test]
fn analyzes_static_fixture_with_static_bindings() {
    let analysis = analyze_fixture("tests/fixtures/parser/static.prg");
    assert!(
        analysis.errors.is_empty(),
        "unexpected semantic errors: {:?}",
        analysis.errors
    );
    assert_eq!(analysis.routines.len(), 1);
    assert_eq!(analysis.routines[0].locals.len(), 2);
    assert_eq!(analysis.routines[0].locals[0].kind, LocalSymbolKind::Static);
    assert_eq!(analysis.routines[0].locals[0].name, "cache");
    assert_eq!(analysis.routines[0].locals[1].kind, LocalSymbolKind::Static);
    assert_eq!(analysis.routines[0].locals[1].name, "hits");
    assert_eq!(analysis.routines[0].resolutions.len(), 1);
    assert_eq!(
        analysis.routines[0].resolutions[0].binding,
        Binding::Local(0)
    );
}

#[test]
fn analyzes_indexed_assignment_fixture_without_semantic_errors() {
    let analysis = analyze_fixture("tests/fixtures/parser/indexed_assign.prg");
    assert!(
        analysis.errors.is_empty(),
        "unexpected semantic errors: {:?}",
        analysis.errors
    );
}

#[test]
fn analyzes_aclones_fixture_without_semantic_errors() {
    let analysis = analyze_fixture("tests/fixtures/parser/aclone.prg");
    assert!(
        analysis.errors.is_empty(),
        "unexpected semantic errors: {:?}",
        analysis.errors
    );
}

#[test]
fn analyzes_array_builtins_fixture_without_semantic_errors() {
    let analysis = analyze_fixture("tests/fixtures/parser/array_builtins.prg");
    assert!(
        analysis.errors.is_empty(),
        "unexpected semantic errors: {:?}",
        analysis.errors
    );
}

#[test]
fn analyzes_module_static_fixture_with_shared_module_bindings() {
    let analysis = analyze_fixture("tests/fixtures/parser/static_module.prg");
    assert!(
        analysis.errors.is_empty(),
        "unexpected semantic errors: {:?}",
        analysis.errors
    );
    assert_eq!(analysis.module_static_symbols.len(), 2);
    assert_eq!(analysis.module_static_symbols[0].name, "s_count");
    assert_eq!(analysis.module_static_symbols[1].name, "s_cache");
    assert!(
        analysis
            .routines
            .iter()
            .flat_map(|routine| &routine.resolutions)
            .any(|resolution| {
                matches!(resolution.binding, Binding::ModuleStatic(0))
                    && resolution.name == "s_count"
            })
    );
    assert!(
        analysis
            .routines
            .iter()
            .flat_map(|routine| &routine.resolutions)
            .any(|resolution| {
                matches!(resolution.binding, Binding::ModuleStatic(1))
                    && resolution.name == "s_cache"
            })
    );
}

#[test]
fn analyzes_memvar_fixture_with_explicit_memvar_symbols() {
    let analysis = analyze_fixture("tests/fixtures/parser/memvars.prg");
    assert!(
        analysis.errors.is_empty(),
        "unexpected semantic errors: {:?}",
        analysis.errors
    );

    assert_eq!(analysis.routines.len(), 1);
    assert_eq!(analysis.routines[0].memvars.len(), 4);
    assert_eq!(
        analysis.routines[0].memvars[0].kind,
        harbour_rust_sema::MemvarSymbolKind::Private
    );
    assert_eq!(
        analysis.routines[0].memvars[2].kind,
        harbour_rust_sema::MemvarSymbolKind::Public
    );
}

#[test]
fn analyzes_codeblock_fixture_with_block_parameter_resolutions() {
    let analysis = analyze_fixture("tests/fixtures/parser/codeblock.prg");
    assert!(
        analysis.errors.is_empty(),
        "unexpected semantic errors: {:?}",
        analysis.errors
    );

    assert!(
        analysis.routines[0]
            .locals
            .iter()
            .any(|symbol| symbol.kind == LocalSymbolKind::BlockParameter && symbol.name == "x")
    );
    assert!(
        analysis.routines[0]
            .locals
            .iter()
            .any(|symbol| symbol.kind == LocalSymbolKind::BlockParameter && symbol.name == "y")
    );
}

#[test]
fn analyzes_private_dynamic_fixture_without_unresolved_memvar_errors() {
    let analysis = analyze_fixture("tests/fixtures/parser/private_dynamic.prg");
    assert!(
        analysis.errors.is_empty(),
        "unexpected semantic errors: {:?}",
        analysis.errors
    );

    assert!(
        analysis.routines[1]
            .resolutions
            .iter()
            .any(|resolution| resolution.binding == Binding::DynamicMemvar)
    );
}
