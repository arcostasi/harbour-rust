use std::fs;

mod support;
use support::{read_upstream_or_skip, workspace_fixture};

use harbour_rust_parser::parse;
use harbour_rust_runtime::{RuntimeError, Value, sqrt_value};

fn runtime_sqrt_baseline() -> String {
    let mut out = String::new();
    out.push_str(&format!(
        "Sqrt(\"A\") => {}\n",
        result_text(sqrt_value(Some(&Value::from("A"))))
    ));
    out.push_str(&format!(
        "Sqrt(-1) => {}\n",
        result_text(sqrt_value(Some(&Value::from(-1_i64))))
    ));
    out.push_str(&format!(
        "Sqrt(0) => {}\n",
        result_text(sqrt_value(Some(&Value::from(0_i64))))
    ));
    out.push_str(&format!(
        "Sqrt(4) => {}\n",
        result_text(sqrt_value(Some(&Value::from(4_i64))))
    ));
    out.push_str(&format!(
        "Sqrt(10) => {}\n",
        result_text(sqrt_value(Some(&Value::from(10_i64))))
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
fn sqrt_fixture_parses_without_errors() {
    let source = fs::read_to_string(workspace_fixture("tests/fixtures/compat/sqrt_runtime.prg"))
        .expect("fixture source");
    let parsed = parse(&source);

    assert!(
        parsed.errors.is_empty(),
        "expected parse success, got {:?}",
        parsed.errors
    );
}

#[test]
fn sqrt_runtime_matches_upstream_oracle_snapshot() {
    let Some(upstream_math) =
        read_upstream_or_skip("harbour-core/utils/hbtest/rt_math.prg", "upstream rt_math")
    else {
        return;
    };
    let Some(upstream_rtl) = read_upstream_or_skip("harbour-core/src/rtl/math.c", "upstream rtl")
    else {
        return;
    };
    let expected = fs::read_to_string(workspace_fixture("tests/fixtures/compat/sqrt_runtime.out"))
        .expect("fixture snapshot");

    assert!(upstream_math.contains("HBTEST Sqrt( \"A\" )"));
    assert!(upstream_math.contains("HBTEST Sqrt( -1 )"));
    assert!(upstream_math.contains("HBTEST Sqrt( 4 )"));
    assert!(upstream_rtl.contains("hb_errRT_BASE_SubstR( EG_ARG, 1097"));
    assert!(upstream_rtl.contains("dArg <= 0"));

    assert_eq!(runtime_sqrt_baseline(), expected);
}
