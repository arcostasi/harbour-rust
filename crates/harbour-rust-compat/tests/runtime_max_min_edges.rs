use std::fs;

mod support;
use support::{read_upstream_or_skip, workspace_fixture};

use harbour_rust_parser::parse;
use harbour_rust_runtime::{RuntimeError, Value, max_value, min_value};

fn runtime_max_min_edges_baseline() -> String {
    let mut out = String::new();
    out.push_str(&format!(
        "Max(10, 10) => {}\n",
        result_text(max_value(
            Some(&Value::from(10_i64)),
            Some(&Value::from(10_i64))
        ))
    ));
    out.push_str(&format!(
        "Min(10, 10) => {}\n",
        result_text(min_value(
            Some(&Value::from(10_i64)),
            Some(&Value::from(10_i64))
        ))
    ));
    out.push_str(&format!(
        "Max(-10, -5) => {}\n",
        result_text(max_value(
            Some(&Value::from(-10_i64)),
            Some(&Value::from(-5_i64))
        ))
    ));
    out.push_str(&format!(
        "Min(-10, -5) => {}\n",
        result_text(min_value(
            Some(&Value::from(-10_i64)),
            Some(&Value::from(-5_i64))
        ))
    ));
    out.push_str(&format!(
        "Max(.T., .T.) => {}\n",
        result_text(max_value(
            Some(&Value::from(true)),
            Some(&Value::from(true))
        ))
    ));
    out.push_str(&format!(
        "Min(.F., .F.) => {}\n",
        result_text(min_value(
            Some(&Value::from(false)),
            Some(&Value::from(false))
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
fn max_min_edges_fixture_parses_without_errors() {
    let source = fs::read_to_string(workspace_fixture(
        "tests/fixtures/compat/max_min_edges_runtime.prg",
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
fn max_min_edges_runtime_matches_upstream_oracle_snapshot() {
    let Some(upstream_math) =
        read_upstream_or_skip("harbour-core/utils/hbtest/rt_math.prg", "upstream rt_math")
    else {
        return;
    };
    let expected = fs::read_to_string(workspace_fixture(
        "tests/fixtures/compat/max_min_edges_runtime.out",
    ))
    .expect("fixture snapshot");

    assert!(
        upstream_math.contains("HBTEST Max( hb_SToD( \"19800101\" ), hb_SToD( \"19800101\" ) )")
    );
    assert!(upstream_math.contains("HBTEST Max( snIntP, snLongP )"));
    assert!(
        upstream_math.contains("HBTEST Min( hb_SToD( \"19800101\" ), hb_SToD( \"19800101\" ) )")
    );
    assert!(upstream_math.contains("HBTEST Min( snIntP, snLongP )"));

    assert_eq!(runtime_max_min_edges_baseline(), expected);
}
