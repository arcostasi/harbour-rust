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
