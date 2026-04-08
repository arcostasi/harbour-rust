use std::{fs, path::PathBuf};

use harbour_rust_compat::render_lexed;

fn normalize_newlines(text: &str) -> String {
    text.replace("\r\n", "\n").replace('\r', "\n")
}

fn workspace_fixture(path: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join(path)
}

fn assert_fixture_tokens(source_path: &str, expected_path: &str) {
    let source = normalize_newlines(
        &fs::read_to_string(workspace_fixture(source_path)).expect("fixture source"),
    );
    let expected = normalize_newlines(
        &fs::read_to_string(workspace_fixture(expected_path)).expect("fixture snapshot"),
    );
    assert_eq!(render_lexed(&source), expected);
}

#[test]
fn hello_fixture_matches_snapshot() {
    assert_fixture_tokens(
        "tests/fixtures/lexer/hello.prg",
        "tests/fixtures/lexer/hello.tokens",
    );
}

#[test]
fn while_fixture_matches_snapshot() {
    assert_fixture_tokens(
        "tests/fixtures/lexer/while.prg",
        "tests/fixtures/lexer/while.tokens",
    );
}
