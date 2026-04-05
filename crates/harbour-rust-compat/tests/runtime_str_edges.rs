use std::fs;

mod support;
use support::{read_upstream_or_skip, workspace_fixture};

use harbour_rust_parser::parse;
use harbour_rust_runtime::{RuntimeError, Value, str_value};

fn runtime_str_edges_baseline() -> String {
    let mut out = String::new();
    out.push_str(&format!(
        "Str(10, -5) => {}\n",
        result_text(str_value(
            Some(&Value::from(10_i64)),
            Some(&Value::from(-5_i64)),
            None,
        ))
    ));
    out.push_str(&format!(
        "Str(10.5, -5) => {}\n",
        result_text(str_value(
            Some(&Value::from(10.5_f64)),
            Some(&Value::from(-5_i64)),
            None,
        ))
    ));
    out.push_str(&format!(
        "Str(-10, -5) => {}\n",
        result_text(str_value(
            Some(&Value::from(-10_i64)),
            Some(&Value::from(-5_i64)),
            None,
        ))
    ));
    out.push_str(&format!(
        "Str(100000, -8) => {}\n",
        result_text(str_value(
            Some(&Value::from(100000_i64)),
            Some(&Value::from(-8_i64)),
            None,
        ))
    ));
    out.push_str(&format!(
        "Str(100, 10, NIL) => {}\n",
        result_text(str_value(
            Some(&Value::from(100_i64)),
            Some(&Value::from(10_i64)),
            Some(&Value::Nil),
        ))
    ));
    out.push_str(&format!(
        "Str(100, NIL, NIL) => {}\n",
        result_text(str_value(
            Some(&Value::from(100_i64)),
            Some(&Value::Nil),
            Some(&Value::Nil),
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
fn str_edges_fixture_parses_without_errors() {
    let source = fs::read_to_string(workspace_fixture(
        "tests/fixtures/compat/str_edges_runtime.prg",
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
fn str_edges_runtime_matches_upstream_oracle_snapshot() {
    let Some(upstream_stra) =
        read_upstream_or_skip("harbour-core/utils/hbtest/rt_stra.prg", "upstream rt_stra")
    else {
        return;
    };
    let expected = fs::read_to_string(workspace_fixture(
        "tests/fixtures/compat/str_edges_runtime.out",
    ))
    .expect("fixture snapshot");

    assert!(upstream_stra.contains("HBTEST Str( 10, -5 )"));
    assert!(upstream_stra.contains("HBTEST Str( 10.50, -5 )"));
    assert!(upstream_stra.contains("HBTEST Str( -10, -5 )"));
    assert!(upstream_stra.contains("HBTEST Str( 100000, -8 )"));
    assert!(upstream_stra.contains("HBTEST Str( 100, 10, NIL )"));
    assert!(upstream_stra.contains("HBTEST Str( 100, NIL, NIL )"));

    assert_eq!(runtime_str_edges_baseline(), expected);
}
