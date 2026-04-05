use std::fs;

mod support;
use support::{read_upstream_or_skip, workspace_fixture};

use harbour_rust_parser::parse;
use harbour_rust_runtime::{RuntimeError, Value, abs};

fn runtime_abs_edges_baseline() -> String {
    let mut out = String::new();
    out.push_str(&format!(
        "Abs(0.1) => {}\n",
        result_text(abs(Some(&Value::from(0.1_f64))))
    ));
    out.push_str(&format!(
        "Abs(-10.578) => {}\n",
        result_text(abs(Some(&Value::from(-10.578_f64))))
    ));
    out.push_str(&format!(
        "Abs(100000) => {}\n",
        result_text(abs(Some(&Value::from(100000_i64))))
    ));
    out.push_str(&format!(
        "Abs(-100000) => {}\n",
        result_text(abs(Some(&Value::from(-100000_i64))))
    ));
    out.push_str(&format!(
        "Abs(NIL) => {}\n",
        result_text(abs(Some(&Value::Nil)))
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
fn abs_edges_fixture_parses_without_errors() {
    let source = fs::read_to_string(workspace_fixture(
        "tests/fixtures/compat/abs_edges_runtime.prg",
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
fn abs_edges_runtime_matches_upstream_oracle_snapshot() {
    let Some(upstream_math) =
        read_upstream_or_skip("harbour-core/utils/hbtest/rt_math.prg", "upstream rt_math")
    else {
        return;
    };
    let expected = fs::read_to_string(workspace_fixture(
        "tests/fixtures/compat/abs_edges_runtime.out",
    ))
    .expect("fixture snapshot");

    assert!(upstream_math.contains("HBTEST Abs( 0.1 )"));
    assert!(upstream_math.contains("HBTEST Abs( -10.578 )"));
    assert!(upstream_math.contains("HBTEST Abs( 100000 )"));
    assert!(upstream_math.contains("HBTEST Abs( -100000 )"));
    assert!(upstream_math.contains("BASE 1089 Argument error (ABS)"));

    assert_eq!(runtime_abs_edges_baseline(), expected);
}
