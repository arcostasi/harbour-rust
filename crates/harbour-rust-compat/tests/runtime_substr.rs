use std::fs;

mod support;
use support::{read_upstream_or_skip, workspace_fixture};

use harbour_rust_parser::parse;
use harbour_rust_runtime::{RuntimeError, Value, substr};

fn runtime_substr_baseline() -> String {
    let mut out = String::new();
    out.push_str(&format!(
        "SubStr(\"abcdef\", 0, 1) => {}\n",
        result_text(substr(
            Some(&Value::from("abcdef")),
            Some(&Value::from(0_i64)),
            Some(&Value::from(1_i64)),
        ))
    ));
    out.push_str(&format!(
        "SubStr(\"abcdef\", 2, 7) => {}\n",
        result_text(substr(
            Some(&Value::from("abcdef")),
            Some(&Value::from(2_i64)),
            Some(&Value::from(7_i64)),
        ))
    ));
    out.push_str(&format!(
        "SubStr(\"abcdef\", -2) => {}\n",
        result_text(substr(
            Some(&Value::from("abcdef")),
            Some(&Value::from(-2_i64)),
            None,
        ))
    ));
    out.push_str(&format!(
        "SubStr(\"abcdef\", 10) => {}\n",
        result_text(substr(
            Some(&Value::from("abcdef")),
            Some(&Value::from(10_i64)),
            None,
        ))
    ));
    out.push_str(&format!(
        "SubStr(\"abcdef\", 2, -1) => {}\n",
        result_text(substr(
            Some(&Value::from("abcdef")),
            Some(&Value::from(2_i64)),
            Some(&Value::from(-1_i64)),
        ))
    ));
    out.push_str(&format!(
        "SubStr(100, 0, -1) => {}\n",
        result_text(substr(
            Some(&Value::from(100_i64)),
            Some(&Value::from(0_i64)),
            Some(&Value::from(-1_i64)),
        ))
    ));
    out.push_str(&format!(
        "SubStr(\"abcdef\", \"a\") => {}\n",
        result_text(substr(
            Some(&Value::from("abcdef")),
            Some(&Value::from("a")),
            None,
        ))
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
fn substr_fixture_parses_without_errors() {
    let source = fs::read_to_string(workspace_fixture(
        "tests/fixtures/compat/substr_runtime.prg",
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
fn substr_runtime_matches_upstream_oracle_snapshot() {
    let Some(upstream) =
        read_upstream_or_skip("harbour-core/utils/hbtest/rt_str.prg", "upstream hbtest")
    else {
        return;
    };
    let expected = fs::read_to_string(workspace_fixture(
        "tests/fixtures/compat/substr_runtime.out",
    ))
    .expect("fixture snapshot");

    assert!(upstream.contains("HBTEST SubStr( \"abcdef\", 0, 1 )"));
    assert!(upstream.contains("HBTEST SubStr( \"abcdef\", -2 )"));
    assert!(upstream.contains("HBTEST SubStr( \"abcdef\", 10 )"));
    assert!(upstream.contains("BASE 1110 Argument error (SUBSTR)"));

    assert_eq!(runtime_substr_baseline(), expected);
}
