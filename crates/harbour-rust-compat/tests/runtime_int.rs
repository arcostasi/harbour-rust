use std::{fs, path::PathBuf};

use harbour_rust_parser::parse;
use harbour_rust_runtime::{RuntimeError, Value, int};

fn workspace_fixture(path: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join(path)
}

fn runtime_int_baseline() -> String {
    let mut out = String::new();
    out.push_str(&format!(
        "Int(NIL) => {}\n",
        result_text(int(Some(&Value::Nil)))
    ));
    out.push_str(&format!(
        "Int(\"A\") => {}\n",
        result_text(int(Some(&Value::from("A"))))
    ));
    out.push_str(&format!(
        "Int({{}}) => {}\n",
        result_text(int(Some(&Value::empty_array())))
    ));
    out.push_str(&format!(
        "Int(0) => {}\n",
        result_text(int(Some(&Value::from(0_i64))))
    ));
    out.push_str(&format!(
        "Int(0.0) => {}\n",
        result_text(int(Some(&Value::from(0.0_f64))))
    ));
    out.push_str(&format!(
        "Int(10) => {}\n",
        result_text(int(Some(&Value::from(10_i64))))
    ));
    out.push_str(&format!(
        "Int(-10) => {}\n",
        result_text(int(Some(&Value::from(-10_i64))))
    ));
    out.push_str(&format!(
        "Int(10.5) => {}\n",
        result_text(int(Some(&Value::from(10.5_f64))))
    ));
    out.push_str(&format!(
        "Int(-10.5) => {}\n",
        result_text(int(Some(&Value::from(-10.5_f64))))
    ));
    out.push_str(&format!(
        "Int(5000000000.9) => {}\n",
        result_text(int(Some(&Value::from(5_000_000_000.9_f64))))
    ));
    out.push_str(&format!(
        "Int(-5000000000.9) => {}\n",
        result_text(int(Some(&Value::from(-5_000_000_000.9_f64))))
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
fn int_fixture_parses_without_errors() {
    let source = fs::read_to_string(workspace_fixture("tests/fixtures/compat/int_runtime.prg"))
        .expect("fixture source");
    let parsed = parse(&source);

    assert!(
        parsed.errors.is_empty(),
        "expected parse success, got {:?}",
        parsed.errors
    );
}

#[test]
fn int_runtime_matches_upstream_oracle_snapshot() {
    let upstream_math =
        fs::read_to_string(workspace_fixture("harbour-core/utils/hbtest/rt_math.prg"))
            .expect("upstream rt_math");
    let expected = fs::read_to_string(workspace_fixture("tests/fixtures/compat/int_runtime.out"))
        .expect("fixture snapshot");

    assert!(upstream_math.contains("HBTEST Int( NIL )"));
    assert!(upstream_math.contains("HBTEST Int( 10.5 )"));
    assert!(upstream_math.contains("HBTEST Int( -10.5 )"));
    assert!(upstream_math.contains("BASE 1090 Argument error (INT)"));

    assert_eq!(runtime_int_baseline(), expected);
}
