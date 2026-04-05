use std::fs;

mod support;
use support::{read_upstream_or_skip, workspace_fixture};

use harbour_rust_parser::parse;
use harbour_rust_runtime::{RuntimeError, Value, left, right};

fn runtime_left_right_edges_baseline() -> String {
    let mut out = String::new();
    out.push_str(&format!(
        "Left(\"abcdef\", 0) => {}\n",
        escaped_result(left(
            Some(&Value::from("abcdef")),
            Some(&Value::from(0_i64)),
        ))
    ));
    out.push_str(&format!(
        "Left(\"ab\\0def\", 5) => {}\n",
        escaped_result(left(
            Some(&Value::from(String::from("ab\0def"))),
            Some(&Value::from(5_i64)),
        ))
    ));
    out.push_str(&format!(
        "Right(\"abcdef\", 0) => {}\n",
        escaped_result(right(
            Some(&Value::from("abcdef")),
            Some(&Value::from(0_i64)),
        ))
    ));
    out.push_str(&format!(
        "Right(\"ab\\0def\", 5) => {}\n",
        escaped_result(right(
            Some(&Value::from(String::from("ab\0def"))),
            Some(&Value::from(5_i64)),
        ))
    ));
    out.push_str(&format!(
        "Right(\"abcdef\", -10) => {}\n",
        escaped_result(right(
            Some(&Value::from("abcdef")),
            Some(&Value::from(-10_i64)),
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
fn left_right_edges_fixture_parses_without_errors() {
    let source = fs::read_to_string(workspace_fixture(
        "tests/fixtures/compat/left_right_edges_runtime.prg",
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
fn left_right_edges_runtime_matches_upstream_oracle_snapshot() {
    let Some(upstream) =
        read_upstream_or_skip("harbour-core/utils/hbtest/rt_str.prg", "upstream hbtest")
    else {
        return;
    };
    let expected = fs::read_to_string(workspace_fixture(
        "tests/fixtures/compat/left_right_edges_runtime.out",
    ))
    .expect("fixture snapshot");

    assert!(upstream.contains("HBTEST Left( \"abcdef\", 0 )"));
    assert!(upstream.contains("HBTEST Left( \"ab\" + Chr( 0 ) + \"def\", 5 )"));
    assert!(upstream.contains("HBTEST Right( \"abcdef\", 0 )"));
    assert!(upstream.contains("HBTEST Right( \"ab\" + Chr( 0 ) + \"def\", 5 )"));
    assert!(upstream.contains("HBTEST Right( \"abcdef\", -10 )"));

    assert_eq!(runtime_left_right_edges_baseline(), expected);
}
