use std::{fs, path::PathBuf};

use harbour_rust_parser::parse;
use harbour_rust_runtime::{RuntimeError, Value, sqrt_value};

fn workspace_fixture(path: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join(path)
}

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
    let upstream_math =
        fs::read_to_string(workspace_fixture("harbour-core/utils/hbtest/rt_math.prg"))
            .expect("upstream rt_math");
    let upstream_rtl =
        fs::read_to_string(workspace_fixture("harbour-core/src/rtl/math.c")).expect("upstream rtl");
    let expected = fs::read_to_string(workspace_fixture("tests/fixtures/compat/sqrt_runtime.out"))
        .expect("fixture snapshot");

    assert!(upstream_math.contains("HBTEST Sqrt( \"A\" )"));
    assert!(upstream_math.contains("HBTEST Sqrt( -1 )"));
    assert!(upstream_math.contains("HBTEST Sqrt( 4 )"));
    assert!(upstream_rtl.contains("hb_errRT_BASE_SubstR( EG_ARG, 1097"));
    assert!(upstream_rtl.contains("dArg <= 0"));

    assert_eq!(runtime_sqrt_baseline(), expected);
}
