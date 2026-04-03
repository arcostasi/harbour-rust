use std::{fs, path::PathBuf};

use harbour_rust_compat::render_parsed;

fn workspace_fixture(path: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join(path)
}

fn assert_fixture_ast(source_path: &str, expected_path: &str) {
    let source = fs::read_to_string(workspace_fixture(source_path)).expect("fixture source");
    let expected = fs::read_to_string(workspace_fixture(expected_path)).expect("fixture snapshot");
    assert_eq!(render_parsed(&source), expected);
}

#[test]
fn hello_fixture_matches_snapshot() {
    assert_fixture_ast(
        "tests/fixtures/parser/hello.prg",
        "tests/fixtures/parser/hello.ast",
    );
}

#[test]
fn while_fixture_matches_snapshot() {
    assert_fixture_ast(
        "tests/fixtures/parser/while.prg",
        "tests/fixtures/parser/while.ast",
    );
}

#[test]
fn static_fixture_matches_snapshot() {
    assert_fixture_ast(
        "tests/fixtures/parser/static.prg",
        "tests/fixtures/parser/static.ast",
    );
}

#[test]
fn global_static_fixture_matches_snapshot() {
    assert_fixture_ast(
        "tests/fixtures/parser/global_static.prg",
        "tests/fixtures/parser/global_static.ast",
    );
}

#[test]
fn arrays_fixture_matches_snapshot() {
    assert_fixture_ast(
        "tests/fixtures/parser/arrays.prg",
        "tests/fixtures/parser/arrays.ast",
    );
}

#[test]
fn compound_assign_fixture_matches_snapshot() {
    assert_fixture_ast(
        "tests/fixtures/parser/compound_assign.prg",
        "tests/fixtures/parser/compound_assign.ast",
    );
}

#[test]
fn indexing_fixture_matches_snapshot() {
    assert_fixture_ast(
        "tests/fixtures/parser/indexing.prg",
        "tests/fixtures/parser/indexing.ast",
    );
}

#[test]
fn memvars_fixture_matches_snapshot() {
    assert_fixture_ast(
        "tests/fixtures/parser/memvars.prg",
        "tests/fixtures/parser/memvars.ast",
    );
}

#[test]
fn codeblock_fixture_matches_snapshot() {
    assert_fixture_ast(
        "tests/fixtures/parser/codeblock.prg",
        "tests/fixtures/parser/codeblock.ast",
    );
}

#[test]
fn macro_read_fixture_matches_snapshot() {
    assert_fixture_ast(
        "tests/fixtures/parser/macro_read.prg",
        "tests/fixtures/parser/macro_read.ast",
    );
}
