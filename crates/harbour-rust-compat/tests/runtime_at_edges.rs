use std::fs;

mod support;
use support::{read_upstream_or_skip, workspace_fixture};

use harbour_rust_parser::parse;
use harbour_rust_runtime::{RuntimeError, Value, at};

fn runtime_at_edges_baseline() -> String {
    let mut out = String::new();
    out.push_str(&format!(
        "At(100, \"\") => {}\n",
        result_text(at(Some(&Value::from(100_i64)), Some(&Value::from(""))))
    ));
    out.push_str(&format!(
        "At(\"F\", \"ABCDEF\") => {}\n",
        result_text(at(Some(&Value::from("F")), Some(&Value::from("ABCDEF"))))
    ));
    out.push_str(&format!(
        "At(\"BCDEF\", \"ABCDEF\") => {}\n",
        result_text(at(
            Some(&Value::from("BCDEF")),
            Some(&Value::from("ABCDEF")),
        ))
    ));
    out.push_str(&format!(
        "At(\"ABCDEFG\", \"ABCDEF\") => {}\n",
        result_text(at(
            Some(&Value::from("ABCDEFG")),
            Some(&Value::from("ABCDEF")),
        ))
    ));
    out.push_str(&format!(
        "At(\"FI\", \"ABCDEF\") => {}\n",
        result_text(at(Some(&Value::from("FI")), Some(&Value::from("ABCDEF"))))
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
fn at_edges_fixture_parses_without_errors() {
    let source = fs::read_to_string(workspace_fixture(
        "tests/fixtures/compat/at_edges_runtime.prg",
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
fn at_edges_runtime_matches_upstream_oracle_snapshot() {
    let Some(upstream) =
        read_upstream_or_skip("harbour-core/utils/hbtest/rt_str.prg", "upstream hbtest")
    else {
        return;
    };
    let expected = fs::read_to_string(workspace_fixture(
        "tests/fixtures/compat/at_edges_runtime.out",
    ))
    .expect("fixture snapshot");

    assert!(upstream.contains("HBTEST At( 100, \"\" )"));
    assert!(upstream.contains("HBTEST At( \"F\", \"ABCDEF\" )"));
    assert!(upstream.contains("HBTEST At( \"BCDEF\", \"ABCDEF\" )"));
    assert!(upstream.contains("HBTEST At( \"ABCDEFG\", \"ABCDEF\" )"));
    assert!(upstream.contains("HBTEST At( \"FI\", \"ABCDEF\" )"));
    assert!(upstream.contains("BASE 1108 Argument error (AT)"));

    assert_eq!(runtime_at_edges_baseline(), expected);
}
