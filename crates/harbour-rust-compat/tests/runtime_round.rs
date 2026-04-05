use std::fs;

mod support;
use support::{read_upstream_or_skip, workspace_fixture};

use harbour_rust_parser::parse;
use harbour_rust_runtime::{RuntimeError, Value, round_value};

fn runtime_round_baseline() -> String {
    let mut out = String::new();
    out.push_str(&format!(
        "Round(NIL, 0) => {}\n",
        result_text(round_value(Some(&Value::Nil), Some(&Value::from(0_i64))))
    ));
    out.push_str(&format!(
        "Round(0, NIL) => {}\n",
        result_text(round_value(Some(&Value::from(0_i64)), Some(&Value::Nil)))
    ));
    out.push_str(&format!(
        "Round(0.5, 0) => {}\n",
        result_text(round_value(
            Some(&Value::from(0.5_f64)),
            Some(&Value::from(0_i64))
        ))
    ));
    out.push_str(&format!(
        "Round(0.55, 1) => {}\n",
        result_text(round_value(
            Some(&Value::from(0.55_f64)),
            Some(&Value::from(1_i64))
        ))
    ));
    out.push_str(&format!(
        "Round(0.557, 2) => {}\n",
        result_text(round_value(
            Some(&Value::from(0.557_f64)),
            Some(&Value::from(2_i64))
        ))
    ));
    out.push_str(&format!(
        "Round(50, -2) => {}\n",
        result_text(round_value(
            Some(&Value::from(50_i64)),
            Some(&Value::from(-2_i64))
        ))
    ));
    out.push_str(&format!(
        "Round(10.5, -1) => {}\n",
        result_text(round_value(
            Some(&Value::from(10.5_f64)),
            Some(&Value::from(-1_i64))
        ))
    ));
    out.push_str(&format!(
        "Round(-0.55, 1) => {}\n",
        result_text(round_value(
            Some(&Value::from(-0.55_f64)),
            Some(&Value::from(1_i64))
        ))
    ));
    out.push_str(&format!(
        "Round(-10.5, 0) => {}\n",
        result_text(round_value(
            Some(&Value::from(-10.5_f64)),
            Some(&Value::from(0_i64))
        ))
    ));
    out.push_str(&format!(
        "Round(-50, -2) => {}\n",
        result_text(round_value(
            Some(&Value::from(-50_i64)),
            Some(&Value::from(-2_i64))
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
fn round_fixture_parses_without_errors() {
    let source = fs::read_to_string(workspace_fixture("tests/fixtures/compat/round_runtime.prg"))
        .expect("fixture source");
    let parsed = parse(&source);

    assert!(
        parsed.errors.is_empty(),
        "expected parse success, got {:?}",
        parsed.errors
    );
}

#[test]
fn round_runtime_matches_upstream_oracle_snapshot() {
    let Some(upstream_math) =
        read_upstream_or_skip("harbour-core/utils/hbtest/rt_math.prg", "upstream rt_math")
    else {
        return;
    };
    let expected = fs::read_to_string(workspace_fixture("tests/fixtures/compat/round_runtime.out"))
        .expect("fixture snapshot");

    assert!(upstream_math.contains("HBTEST Round( NIL, 0 )"));
    assert!(upstream_math.contains("HBTEST Round( 0.55, 1 )"));
    assert!(upstream_math.contains("HBTEST Round( 50, -2 )"));
    assert!(upstream_math.contains("HBTEST Round( 10.50, -1 )"));
    assert!(upstream_math.contains("HBTEST Round( -10.50, 0 )"));
    assert!(upstream_math.contains("HBTEST Round( -50, -2 )"));
    assert!(upstream_math.contains("BASE 1094 Argument error (ROUND)"));

    assert_eq!(runtime_round_baseline(), expected);
}
