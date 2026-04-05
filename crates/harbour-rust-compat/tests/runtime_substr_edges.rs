use std::fs;

mod support;
use support::{read_upstream_or_skip, workspace_fixture};

use harbour_rust_parser::parse;
use harbour_rust_runtime::{RuntimeError, Value, substr};

fn runtime_substr_edges_baseline() -> String {
    let mut out = String::new();
    out.push_str(&format!(
        "SubStr(\"abcdef\", 0) => {}\n",
        escaped_result(substr(
            Some(&Value::from("abcdef")),
            Some(&Value::from(0_i64)),
            None,
        ))
    ));
    out.push_str(&format!(
        "SubStr(\"abcdef\", -10, 1) => {}\n",
        escaped_result(substr(
            Some(&Value::from("abcdef")),
            Some(&Value::from(-10_i64)),
            Some(&Value::from(1_i64)),
        ))
    ));
    out.push_str(&format!(
        "SubStr(\"abcdef\", -10, 15) => {}\n",
        escaped_result(substr(
            Some(&Value::from("abcdef")),
            Some(&Value::from(-10_i64)),
            Some(&Value::from(15_i64)),
        ))
    ));
    out.push_str(&format!(
        "SubStr(\"abcdef\", 2, 0) => {}\n",
        escaped_result(substr(
            Some(&Value::from("abcdef")),
            Some(&Value::from(2_i64)),
            Some(&Value::from(0_i64)),
        ))
    ));
    out.push_str(&format!(
        "SubStr(\"ab\\0def\", 2, 3) => {}\n",
        escaped_result(substr(
            Some(&Value::from(String::from("ab\0def"))),
            Some(&Value::from(2_i64)),
            Some(&Value::from(3_i64)),
        ))
    ));
    out.push_str(&format!(
        "SubStr(\"abc\\0def\", 4, 1) => {}\n",
        escaped_result(substr(
            Some(&Value::from(String::from("abc\0def"))),
            Some(&Value::from(4_i64)),
            Some(&Value::from(1_i64)),
        ))
    ));
    out.push_str(&format!(
        "SubStr(\"abcdef\", 1, \"a\") => {}\n",
        escaped_result(substr(
            Some(&Value::from("abcdef")),
            Some(&Value::from(1_i64)),
            Some(&Value::from("a")),
        ))
    ));
    out
}

fn escaped_result(result: Result<Value, RuntimeError>) -> String {
    match result {
        Ok(Value::String(text)) => format!("\"{}\"", escape_string(&text)),
        Ok(value) => value.to_output_string(),
        Err(error) => error.message,
    }
}

fn escape_string(text: &str) -> String {
    let mut escaped = String::new();
    for ch in text.chars() {
        match ch {
            '\0' => escaped.push_str("\\0"),
            '\t' => escaped.push_str("\\t"),
            '\n' => escaped.push_str("\\n"),
            '\r' => escaped.push_str("\\r"),
            _ => escaped.push(ch),
        }
    }
    escaped
}

#[test]
fn substr_edges_fixture_parses_without_errors() {
    let source = fs::read_to_string(workspace_fixture(
        "tests/fixtures/compat/substr_edges_runtime.prg",
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
fn substr_edges_runtime_matches_upstream_oracle_snapshot() {
    let Some(upstream) =
        read_upstream_or_skip("harbour-core/utils/hbtest/rt_str.prg", "upstream hbtest")
    else {
        return;
    };
    let expected = fs::read_to_string(workspace_fixture(
        "tests/fixtures/compat/substr_edges_runtime.out",
    ))
    .expect("fixture snapshot");

    assert!(upstream.contains("HBTEST SubStr( \"abcdef\", 0 )"));
    assert!(upstream.contains("HBTEST SubStr( \"abcdef\", -10, 1 )"));
    assert!(upstream.contains("HBTEST SubStr( \"abcdef\", -10, 15 )"));
    assert!(upstream.contains("HBTEST SubStr( \"abcdef\", 2, 0 )"));
    assert!(upstream.contains("HBTEST SubStr( \"ab\" + Chr( 0 ) + \"def\", 2, 3 )"));
    assert!(upstream.contains("HBTEST SubStr( \"abc\" + Chr( 0 ) + \"def\", 4, 1 )"));
    assert!(upstream.contains("HBTEST SubStr( \"abcdef\", 1, \"a\" )"));
    assert!(upstream.contains("BASE 1110 Argument error (SUBSTR)"));

    assert_eq!(runtime_substr_edges_baseline(), expected);
}
