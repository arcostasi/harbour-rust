use std::fs;

mod support;
use support::{read_upstream_or_skip, workspace_fixture};

use harbour_rust_parser::parse;
use harbour_rust_runtime::{RuntimeError, Value, lower, upper};

fn runtime_upper_lower_baseline() -> String {
    let mut out = String::new();
    out.push_str(&format!(
        "Upper(\"aAZAZa\") => {}\n",
        result_text(upper(Some(&Value::from("aAZAZa"))))
    ));
    out.push_str(&format!(
        "Upper(\"\") => {}\n",
        result_text(upper(Some(&Value::from(""))))
    ));
    out.push_str(&format!(
        "Upper(100) => {}\n",
        result_text(upper(Some(&Value::from(100_i64))))
    ));
    out.push_str(&format!(
        "Lower(\"AazazA\") => {}\n",
        result_text(lower(Some(&Value::from("AazazA"))))
    ));
    out.push_str(&format!(
        "Lower(\"\") => {}\n",
        result_text(lower(Some(&Value::from(""))))
    ));
    out.push_str(&format!(
        "Lower(100) => {}\n",
        result_text(lower(Some(&Value::from(100_i64))))
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
fn upper_lower_fixture_parses_without_errors() {
    let source = fs::read_to_string(workspace_fixture(
        "tests/fixtures/compat/upper_lower_runtime.prg",
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
fn upper_lower_runtime_matches_upstream_oracle_snapshot() {
    let Some(upstream) =
        read_upstream_or_skip("harbour-core/utils/hbtest/rt_str.prg", "upstream hbtest")
    else {
        return;
    };
    let expected = fs::read_to_string(workspace_fixture(
        "tests/fixtures/compat/upper_lower_runtime.out",
    ))
    .expect("fixture snapshot");

    assert!(upstream.contains("HBTEST Upper( scString )"));
    assert!(upstream.contains("BASE 1102 Argument error (UPPER)"));
    assert!(upstream.contains("HBTEST Lower( scString )"));
    assert!(upstream.contains("BASE 1103 Argument error (LOWER)"));

    assert_eq!(runtime_upper_lower_baseline(), expected);
}
