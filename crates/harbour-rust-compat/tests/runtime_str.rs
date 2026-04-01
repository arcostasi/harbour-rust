use std::{fs, path::PathBuf};

use harbour_rust_parser::parse;
use harbour_rust_runtime::{RuntimeError, Value, str_value};

fn workspace_fixture(path: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join(path)
}

fn runtime_str_baseline() -> String {
    let mut out = String::new();
    out.push_str(&format!(
        "Str(10) => {}\n",
        result_text(str_value(Some(&Value::from(10_i64)), None, None))
    ));
    out.push_str(&format!(
        "Str(0) => {}\n",
        result_text(str_value(Some(&Value::from(0_i64)), None, None))
    ));
    out.push_str(&format!(
        "Str(10.5) => {}\n",
        result_text(str_value(Some(&Value::from(10.5_f64)), None, None))
    ));
    out.push_str(&format!(
        "Str(10, 5) => {}\n",
        result_text(str_value(
            Some(&Value::from(10_i64)),
            Some(&Value::from(5_i64)),
            None,
        ))
    ));
    out.push_str(&format!(
        "Str(10.6, 5) => {}\n",
        result_text(str_value(
            Some(&Value::from(10.6_f64)),
            Some(&Value::from(5_i64)),
            None,
        ))
    ));
    out.push_str(&format!(
        "Str(2, 5, 2) => {}\n",
        result_text(str_value(
            Some(&Value::from(2_i64)),
            Some(&Value::from(5_i64)),
            Some(&Value::from(2_i64)),
        ))
    ));
    out.push_str(&format!(
        "Str(3.125, 8, 2) => {}\n",
        result_text(str_value(
            Some(&Value::from(3.125_f64)),
            Some(&Value::from(8_i64)),
            Some(&Value::from(2_i64)),
        ))
    ));
    out.push_str(&format!(
        "Str(100000, 5) => {}\n",
        result_text(str_value(
            Some(&Value::from(100000_i64)),
            Some(&Value::from(5_i64)),
            None,
        ))
    ));
    out.push_str(&format!(
        "Str(NIL) => {}\n",
        result_text(str_value(Some(&Value::Nil), None, None))
    ));
    out.push_str(&format!(
        "Str(\"A\", 10, 2) => {}\n",
        result_text(str_value(
            Some(&Value::from("A")),
            Some(&Value::from(10_i64)),
            Some(&Value::from(2_i64)),
        ))
    ));
    out.push_str(&format!(
        "Str(100, 10, \"A\") => {}\n",
        result_text(str_value(
            Some(&Value::from(100_i64)),
            Some(&Value::from(10_i64)),
            Some(&Value::from("A")),
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
fn str_fixture_parses_without_errors() {
    let source = fs::read_to_string(workspace_fixture("tests/fixtures/compat/str_runtime.prg"))
        .expect("fixture source");
    let parsed = parse(&source);

    assert!(
        parsed.errors.is_empty(),
        "expected parse success, got {:?}",
        parsed.errors
    );
}

#[test]
fn str_runtime_matches_upstream_oracle_snapshot() {
    let upstream_hvma =
        fs::read_to_string(workspace_fixture("harbour-core/utils/hbtest/rt_hvma.prg"))
            .expect("upstream rt_hvma");
    let upstream_math =
        fs::read_to_string(workspace_fixture("harbour-core/utils/hbtest/rt_math.prg"))
            .expect("upstream rt_math");
    let upstream_stra =
        fs::read_to_string(workspace_fixture("harbour-core/utils/hbtest/rt_stra.prg"))
            .expect("upstream rt_stra");
    let expected = fs::read_to_string(workspace_fixture("tests/fixtures/compat/str_runtime.out"))
        .expect("fixture snapshot");

    assert!(upstream_stra.contains("HBTEST Str( NIL )"));
    assert!(upstream_stra.contains("HBTEST Str( 100000, 5 )"));
    assert!(upstream_stra.contains("BASE 1099 Argument error (STR)"));
    assert!(upstream_hvma.contains("HBTEST Str( 2 + 0.5 )"));
    assert!(upstream_math.contains("HBTEST Str( Sqrt( 4 ), 21, 18 )"));

    assert_eq!(runtime_str_baseline(), expected);
}
