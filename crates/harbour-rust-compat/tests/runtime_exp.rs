use std::{fs, path::PathBuf};

use harbour_rust_parser::parse;
use harbour_rust_runtime::{RuntimeError, Value, exp_value, round_value, str_value};

fn workspace_fixture(path: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join(path)
}

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
    let upstream_math =
        fs::read_to_string(workspace_fixture("harbour-core/utils/hbtest/rt_math.prg"))
            .expect("upstream rt_math");
    let upstream_rtl =
        fs::read_to_string(workspace_fixture("harbour-core/src/rtl/math.c")).expect("upstream rtl");
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
