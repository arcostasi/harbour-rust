use std::{fs, path::PathBuf};

use harbour_rust_parser::parse;
use harbour_rust_runtime::{RuntimeError, Value, abs};

fn workspace_fixture(path: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join(path)
}

fn runtime_abs_baseline() -> String {
    let mut out = String::new();
    out.push_str(&format!(
        "Abs(0) => {}\n",
        result_text(abs(Some(&Value::from(0_i64))))
    ));
    out.push_str(&format!(
        "Abs(-10) => {}\n",
        result_text(abs(Some(&Value::from(-10_i64))))
    ));
    out.push_str(&format!(
        "Abs(10.5) => {}\n",
        result_text(abs(Some(&Value::from(10.5_f64))))
    ));
    out.push_str(&format!(
        "Abs(-10.7) => {}\n",
        result_text(abs(Some(&Value::from(-10.7_f64))))
    ));
    out.push_str(&format!(
        "Abs(\"A\") => {}\n",
        result_text(abs(Some(&Value::from("A"))))
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
fn abs_fixture_parses_without_errors() {
    let source = fs::read_to_string(workspace_fixture("tests/fixtures/compat/abs_runtime.prg"))
        .expect("fixture source");
    let parsed = parse(&source);

    assert!(
        parsed.errors.is_empty(),
        "expected parse success, got {:?}",
        parsed.errors
    );
}

#[test]
fn abs_runtime_matches_upstream_oracle_snapshot() {
    let upstream_math =
        fs::read_to_string(workspace_fixture("harbour-core/utils/hbtest/rt_math.prg"))
            .expect("upstream rt_math");
    let expected = fs::read_to_string(workspace_fixture("tests/fixtures/compat/abs_runtime.out"))
        .expect("fixture snapshot");

    assert!(upstream_math.contains("HBTEST Abs( \"A\" )"));
    assert!(upstream_math.contains("HBTEST Abs( -10 )"));
    assert!(upstream_math.contains("HBTEST Abs( -10.7 )"));
    assert!(upstream_math.contains("BASE 1089 Argument error (ABS)"));

    assert_eq!(runtime_abs_baseline(), expected);
}
