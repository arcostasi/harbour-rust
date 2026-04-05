use std::fs;

mod support;
use support::{read_upstream_or_skip, workspace_fixture};

use harbour_rust_parser::parse;
use harbour_rust_runtime::{RuntimeError, Value, replicate};

fn runtime_replicate_edges_baseline() -> String {
    let mut out = String::new();
    out.push_str(&format!(
        "Replicate(\"\", 0) => {}\n",
        escaped_result(replicate(Some(&Value::from("")), Some(&Value::from(0_i64)),))
    ));
    out.push_str(&format!(
        "Replicate(\"HE\", 3.1) => {}\n",
        escaped_result(replicate(
            Some(&Value::from("HE")),
            Some(&Value::from(3.1_f64)),
        ))
    ));
    out.push_str(&format!(
        "Replicate(\"H{}\", 2) => {}\n",
        "\\0",
        escaped_result(replicate(
            Some(&Value::from(String::from("H\0"))),
            Some(&Value::from(2_i64)),
        ))
    ));
    out.push_str(&format!(
        "Replicate(\"XXX\", 30000) => {}\n",
        escaped_result(replicate(
            Some(&Value::from("XXX")),
            Some(&Value::from(30_000_i64)),
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
fn replicate_edges_fixture_parses_without_errors() {
    let source = fs::read_to_string(workspace_fixture(
        "tests/fixtures/compat/replicate_edges_runtime.prg",
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
fn replicate_edges_runtime_matches_upstream_oracle_snapshot() {
    let Some(upstream) =
        read_upstream_or_skip("harbour-core/utils/hbtest/rt_str.prg", "upstream hbtest")
    else {
        return;
    };
    let expected = fs::read_to_string(workspace_fixture(
        "tests/fixtures/compat/replicate_edges_runtime.out",
    ))
    .expect("fixture snapshot");

    assert!(upstream.contains("HBTEST Replicate( \"\"   , 0 )"));
    assert!(upstream.contains("HBTEST Replicate( \"HE\", 3.1 )"));
    assert!(upstream.contains("HBTEST Replicate( \"H\" + Chr( 0 ), 2 )"));
    assert!(upstream.contains("HBTEST Replicate( \"XXX\", 30000)"));
    assert!(upstream.contains("BASE 1234 String overflow (REPLICATE)"));

    assert_eq!(runtime_replicate_edges_baseline(), expected);
}
