use std::{fs, path::PathBuf};

use harbour_rust_parser::parse;
use harbour_rust_runtime::{RuntimeError, Value, log_value, str_value};

fn workspace_fixture(path: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join(path)
}

fn runtime_log_baseline() -> String {
    let mut out = String::new();
    out.push_str(&format!(
        "Log(\"A\") => {}\n",
        result_text(log_value(Some(&Value::from("A"))))
    ));
    out.push_str(&format!(
        "Str(Log(-1)) => {}\n",
        string_result_text(log_value(Some(&Value::from(-1_i64))), None, None)
    ));
    out.push_str(&format!(
        "Str(Log(1), 10, 2) => {}\n",
        string_result_text(
            log_value(Some(&Value::from(1_i64))),
            Some(Value::from(10_i64)),
            Some(Value::from(2_i64))
        )
    ));
    out.push_str(&format!(
        "Str(Log(12), 10, 2) => {}\n",
        string_result_text(
            log_value(Some(&Value::from(12_i64))),
            Some(Value::from(10_i64)),
            Some(Value::from(2_i64))
        )
    ));
    out.push_str(&format!(
        "Str(Log(10), 10, 2) => {}\n",
        string_result_text(
            log_value(Some(&Value::from(10_i64))),
            Some(Value::from(10_i64)),
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

#[test]
fn log_fixture_parses_without_errors() {
    let source = fs::read_to_string(workspace_fixture("tests/fixtures/compat/log_runtime.prg"))
        .expect("fixture source");
    let parsed = parse(&source);

    assert!(
        parsed.errors.is_empty(),
        "expected parse success, got {:?}",
        parsed.errors
    );
}

#[test]
fn log_runtime_matches_upstream_oracle_snapshot() {
    let upstream_math =
        fs::read_to_string(workspace_fixture("harbour-core/utils/hbtest/rt_math.prg"))
            .expect("upstream rt_math");
    let upstream_rtl =
        fs::read_to_string(workspace_fixture("harbour-core/src/rtl/math.c")).expect("upstream rtl");
    let expected = fs::read_to_string(workspace_fixture("tests/fixtures/compat/log_runtime.out"))
        .expect("fixture snapshot");

    assert!(upstream_math.contains("HBTEST Log( \"A\" )"));
    assert!(upstream_math.contains("HBTEST Str( Log( -1 ) )"));
    assert!(upstream_math.contains("HBTEST Str( Log( 1 ) )"));
    assert!(upstream_math.contains("HBTEST Str( Log( 12 ) )"));
    assert!(upstream_rtl.contains("HB_FUNC( LOG )"));
    assert!(upstream_rtl.contains("hb_errRT_BASE_SubstR( EG_ARG, 1095"));

    assert_eq!(runtime_log_baseline(), expected);
}
