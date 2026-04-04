use std::fs;

mod support;
use support::{read_upstream_or_skip, workspace_fixture};

use harbour_rust_parser::parse;
use harbour_rust_runtime::{RuntimeError, Value, exp_value, round_value, str_value};

fn runtime_exp_baseline() -> String {
    let mut out = String::new();
    out.push_str(&format!(
        "Exp(\"A\") => {}\n",
        result_text(exp_value(Some(&Value::from("A"))))
    ));
    out.push_str(&format!(
        "Exp(0) => {}\n",
        result_text(exp_value(Some(&Value::from(0_i64))))
    ));
    out.push_str(&format!(
        "Str(Exp(15)) => {}\n",
        string_result_text(exp_value(Some(&Value::from(15_i64))), None, None)
    ));
    out.push_str(&format!(
        "Round(Exp(1), 2) => {}\n",
        round_result_text(
            exp_value(Some(&Value::from(1_i64))),
            Some(Value::from(2_i64))
        )
    ));
    out.push_str(&format!(
        "Str(Exp(1), 20, 10) => {}\n",
        string_result_text(
            exp_value(Some(&Value::from(1_i64))),
            Some(Value::from(20_i64)),
            Some(Value::from(10_i64))
        )
    ));
    out.push_str(&format!(
        "Round(Exp(10), 2) => {}\n",
        round_result_text(
            exp_value(Some(&Value::from(10_i64))),
            Some(Value::from(2_i64))
        )
    ));
    out
}

fn result_text(result: Result<Value, RuntimeError>) -> String {
    match result {
        Ok(value) => value.to_output_string(),
        Err(error) => error.message,
    }
}

fn string_result_text(
    result: Result<Value, RuntimeError>,
    width: Option<Value>,
    decimals: Option<Value>,
) -> String {
    match result {
        Ok(value) => match str_value(Some(&value), width.as_ref(), decimals.as_ref()) {
            Ok(value) => value.to_output_string(),
            Err(error) => error.message,
        },
        Err(error) => error.message,
    }
}

fn round_result_text(result: Result<Value, RuntimeError>, decimals: Option<Value>) -> String {
    match result {
        Ok(value) => match round_value(Some(&value), decimals.as_ref()) {
            Ok(value) => value.to_output_string(),
            Err(error) => error.message,
        },
        Err(error) => error.message,
    }
}

#[test]
fn exp_fixture_parses_without_errors() {
    let source = fs::read_to_string(workspace_fixture("tests/fixtures/compat/exp_runtime.prg"))
        .expect("fixture source");
    let parsed = parse(&source);

    assert!(
        parsed.errors.is_empty(),
        "expected parse success, got {:?}",
        parsed.errors
    );
}

#[test]
fn exp_runtime_matches_upstream_oracle_snapshot() {
    let Some(upstream_math) =
        read_upstream_or_skip("harbour-core/utils/hbtest/rt_math.prg", "upstream rt_math")
    else {
        return;
    };
    let Some(upstream_rtl) = read_upstream_or_skip("harbour-core/src/rtl/math.c", "upstream rtl")
    else {
        return;
    };
    let expected = fs::read_to_string(workspace_fixture("tests/fixtures/compat/exp_runtime.out"))
        .expect("fixture snapshot");

    assert!(upstream_math.contains("HBTEST Exp( \"A\" )"));
    assert!(upstream_math.contains("HBTEST Exp( 0 )"));
    assert!(upstream_math.contains("HBTEST Str( Exp( 15 ) )"));
    assert!(upstream_math.contains("HBTEST Round( Exp( 1 ), 2 )"));
    assert!(upstream_rtl.contains("HB_FUNC( EXP )"));
    assert!(upstream_rtl.contains("hb_errRT_BASE_SubstR( EG_ARG, 1096"));

    assert_eq!(runtime_exp_baseline(), expected);
}
