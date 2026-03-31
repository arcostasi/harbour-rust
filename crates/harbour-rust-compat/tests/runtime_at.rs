use std::{fs, path::PathBuf};

use harbour_rust_parser::parse;
use harbour_rust_runtime::{RuntimeError, Value, at};

fn workspace_fixture(path: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join(path)
}

fn runtime_at_baseline() -> String {
    let mut out = String::new();
    out.push_str(&format!(
        "At(\"\", \"\") => {}\n",
        result_text(at(Some(&Value::from("")), Some(&Value::from(""))))
    ));
    out.push_str(&format!(
        "At(\"\", \"ABCDEF\") => {}\n",
        result_text(at(Some(&Value::from("")), Some(&Value::from("ABCDEF"))))
    ));
    out.push_str(&format!(
        "At(\"ABCDEF\", \"\") => {}\n",
        result_text(at(Some(&Value::from("ABCDEF")), Some(&Value::from(""))))
    ));
    out.push_str(&format!(
        "At(\"AB\", \"AB\") => {}\n",
        result_text(at(Some(&Value::from("AB")), Some(&Value::from("AB"))))
    ));
    out.push_str(&format!(
        "At(\"AB\", \"AAB\") => {}\n",
        result_text(at(Some(&Value::from("AB")), Some(&Value::from("AAB"))))
    ));
    out.push_str(&format!(
        "At(\"X\", \"ABCDEF\") => {}\n",
        result_text(at(Some(&Value::from("X")), Some(&Value::from("ABCDEF"))))
    ));
    out.push_str(&format!(
        "At(90, 100) => {}\n",
        result_text(at(Some(&Value::from(90_i64)), Some(&Value::from(100_i64))))
    ));
    out.push_str(&format!(
        "At(\"\", 100) => {}\n",
        result_text(at(Some(&Value::from("")), Some(&Value::from(100_i64))))
    ));
    out
}

fn result_text(result: Result<Value, RuntimeError>) -> String {
    match result {
        Ok(value) => value.to_output_string(),
        Err(error) => error.message,
    }
}

#[test]
fn at_fixture_parses_without_errors() {
    let source = fs::read_to_string(workspace_fixture("tests/fixtures/compat/at_runtime.prg"))
        .expect("fixture source");
    let parsed = parse(&source);

    assert!(
        parsed.errors.is_empty(),
        "expected parse success, got {:?}",
        parsed.errors
    );
}

#[test]
fn at_runtime_matches_upstream_oracle_snapshot() {
    let upstream = fs::read_to_string(workspace_fixture("harbour-core/utils/hbtest/rt_str.prg"))
        .expect("upstream hbtest");
    let expected = fs::read_to_string(workspace_fixture("tests/fixtures/compat/at_runtime.out"))
        .expect("fixture snapshot");

    assert!(upstream.contains("HBTEST At( 90, 100 )"));
    assert!(upstream.contains("HBTEST At( \"\", \"ABCDEF\" )"));
    assert!(upstream.contains("HBTEST At( \"AB\", \"AAB\" )"));
    assert!(upstream.contains("BASE 1108 Argument error (AT)"));

    assert_eq!(runtime_at_baseline(), expected);
}
