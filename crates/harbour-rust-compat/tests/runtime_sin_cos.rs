use std::{fs, path::PathBuf};

use harbour_rust_parser::parse;
use harbour_rust_runtime::{RuntimeError, Value, cos_value, round_value, sin_value};

fn workspace_fixture(path: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join(path)
}

fn runtime_sin_cos_baseline() -> String {
    let mut out = String::new();
    out.push_str(&format!(
        "Sin(\"A\") => {}\n",
        result_text(sin_value(Some(&Value::from("A"))))
    ));
    out.push_str(&format!("Cos() => {}\n", result_text(cos_value(None))));
    out.push_str(&format!(
        "Sin(0) => {}\n",
        result_text(sin_value(Some(&Value::from(0_i64))))
    ));
    out.push_str(&format!(
        "Cos(0) => {}\n",
        result_text(cos_value(Some(&Value::from(0_i64))))
    ));
    out.push_str(&format!(
        "Round(Sin(1), 2) => {}\n",
        result_text(round_value(
            sin_value(Some(&Value::from(1_i64))).ok().as_ref(),
            Some(&Value::from(2_i64))
        ))
    ));
    out.push_str(&format!(
        "Round(Cos(1), 2) => {}\n",
        result_text(round_value(
            cos_value(Some(&Value::from(1_i64))).ok().as_ref(),
            Some(&Value::from(2_i64))
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
fn sin_cos_fixture_parses_without_errors() {
    let source = fs::read_to_string(workspace_fixture(
        "tests/fixtures/compat/sin_cos_runtime.prg",
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
fn sin_cos_runtime_matches_project_baseline_snapshot() {
    let upstream_c_std = fs::read_to_string(workspace_fixture("harbour-core/doc/c_std.txt"))
        .expect("upstream c_std");
    let expected = fs::read_to_string(workspace_fixture(
        "tests/fixtures/compat/sin_cos_runtime.out",
    ))
    .expect("fixture snapshot");

    assert!(upstream_c_std.contains("sin()"));
    assert!(upstream_c_std.contains("cos()"));

    assert_eq!(runtime_sin_cos_baseline(), expected);
}
