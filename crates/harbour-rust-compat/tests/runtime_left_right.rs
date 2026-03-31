use std::{fs, path::PathBuf};

use harbour_rust_parser::parse;
use harbour_rust_runtime::{RuntimeError, Value, left, right};

fn workspace_fixture(path: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join(path)
}

fn runtime_left_right_baseline() -> String {
    let mut out = String::new();
    out.push_str(&format!(
        "Left(\"abcdef\", -2) => {}\n",
        result_text(left(
            Some(&Value::from("abcdef")),
            Some(&Value::from(-2_i64)),
        ))
    ));
    out.push_str(&format!(
        "Left(\"abcdef\", 2) => {}\n",
        result_text(left(
            Some(&Value::from("abcdef")),
            Some(&Value::from(2_i64)),
        ))
    ));
    out.push_str(&format!(
        "Left(\"abcdef\", 10) => {}\n",
        result_text(left(
            Some(&Value::from("abcdef")),
            Some(&Value::from(10_i64)),
        ))
    ));
    out.push_str(&format!(
        "Left(100, -10) => {}\n",
        result_text(left(
            Some(&Value::from(100_i64)),
            Some(&Value::from(-10_i64))
        ))
    ));
    out.push_str(&format!(
        "Left(\"abcdef\", \"A\") => {}\n",
        result_text(left(Some(&Value::from("abcdef")), Some(&Value::from("A"))))
    ));
    out.push_str(&format!(
        "Right(\"abcdef\", -2) => {}\n",
        result_text(right(
            Some(&Value::from("abcdef")),
            Some(&Value::from(-2_i64)),
        ))
    ));
    out.push_str(&format!(
        "Right(\"abcdef\", 2) => {}\n",
        result_text(right(
            Some(&Value::from("abcdef")),
            Some(&Value::from(2_i64)),
        ))
    ));
    out.push_str(&format!(
        "Right(\"abcdef\", 10) => {}\n",
        result_text(right(
            Some(&Value::from("abcdef")),
            Some(&Value::from(10_i64)),
        ))
    ));
    out.push_str(&format!(
        "Right(100, -10) => {}\n",
        result_text(right(
            Some(&Value::from(100_i64)),
            Some(&Value::from(-10_i64)),
        ))
    ));
    out.push_str(&format!(
        "Right(\"abcdef\", \"A\") => {}\n",
        result_text(right(Some(&Value::from("abcdef")), Some(&Value::from("A")),))
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
fn left_right_fixture_parses_without_errors() {
    let source = fs::read_to_string(workspace_fixture(
        "tests/fixtures/compat/left_right_runtime.prg",
    ))
    .expect("fixture source");
    let parsed = parse(&source);

    assert!(
        parsed.errors.is_empty(),
        "expected parse success, got {:?}",
        parsed.errors
    );
}

#[test]
fn left_right_runtime_matches_upstream_oracle_snapshot() {
    let upstream = fs::read_to_string(workspace_fixture("harbour-core/utils/hbtest/rt_str.prg"))
        .expect("upstream hbtest");
    let expected = fs::read_to_string(workspace_fixture(
        "tests/fixtures/compat/left_right_runtime.out",
    ))
    .expect("fixture snapshot");

    assert!(upstream.contains("HBTEST Left( \"abcdef\", -2 )"));
    assert!(upstream.contains("HBTEST Left( 100"));
    assert!(upstream.contains("BASE 1124 Argument error (LEFT)"));
    assert!(upstream.contains("HBTEST Right( \"abcdef\", -2 )"));
    assert!(upstream.contains("HBTEST Right( 100"));

    assert_eq!(runtime_left_right_baseline(), expected);
}
