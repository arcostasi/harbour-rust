use std::{fs, path::PathBuf};

use harbour_rust_parser::parse;
use harbour_rust_runtime::{RuntimeError, Value, max_value, min_value};

fn workspace_fixture(path: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join(path)
}

fn runtime_max_min_baseline() -> String {
    let mut out = String::new();
    out.push_str(&format!(
        "Max(NIL, NIL) => {}\n",
        result_text(max_value(Some(&Value::Nil), Some(&Value::Nil)))
    ));
    out.push_str(&format!(
        "Max(10, NIL) => {}\n",
        result_text(max_value(Some(&Value::from(10_i64)), Some(&Value::Nil)))
    ));
    out.push_str(&format!(
        "Max(10, 5) => {}\n",
        result_text(max_value(
            Some(&Value::from(10_i64)),
            Some(&Value::from(5_i64))
        ))
    ));
    out.push_str(&format!(
        "Max(10, 10.5) => {}\n",
        result_text(max_value(
            Some(&Value::from(10_i64)),
            Some(&Value::from(10.5_f64))
        ))
    ));
    out.push_str(&format!(
        "Max(.F., .T.) => {}\n",
        result_text(max_value(
            Some(&Value::from(false)),
            Some(&Value::from(true))
        ))
    ));
    out.push_str(&format!(
        "Min(NIL, NIL) => {}\n",
        result_text(min_value(Some(&Value::Nil), Some(&Value::Nil)))
    ));
    out.push_str(&format!(
        "Min(10, NIL) => {}\n",
        result_text(min_value(Some(&Value::from(10_i64)), Some(&Value::Nil)))
    ));
    out.push_str(&format!(
        "Min(10, 5) => {}\n",
        result_text(min_value(
            Some(&Value::from(10_i64)),
            Some(&Value::from(5_i64))
        ))
    ));
    out.push_str(&format!(
        "Min(10, 10.5) => {}\n",
        result_text(min_value(
            Some(&Value::from(10_i64)),
            Some(&Value::from(10.5_f64))
        ))
    ));
    out.push_str(&format!(
        "Min(.F., .T.) => {}\n",
        result_text(min_value(
            Some(&Value::from(false)),
            Some(&Value::from(true))
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
fn max_min_fixture_parses_without_errors() {
    let source = fs::read_to_string(workspace_fixture(
        "tests/fixtures/compat/max_min_runtime.prg",
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
fn max_min_runtime_matches_upstream_oracle_snapshot() {
    let upstream_math =
        fs::read_to_string(workspace_fixture("harbour-core/utils/hbtest/rt_math.prg"))
            .expect("upstream rt_math");
    let upstream_rtl = fs::read_to_string(workspace_fixture("harbour-core/src/rtl/minmax.c"))
        .expect("upstream rtl");
    let expected = fs::read_to_string(workspace_fixture(
        "tests/fixtures/compat/max_min_runtime.out",
    ))
    .expect("fixture snapshot");

    assert!(upstream_math.contains("HBTEST Max( NIL, NIL )"));
    assert!(upstream_math.contains("HBTEST Max( 10, NIL )"));
    assert!(upstream_math.contains("HBTEST Max( snIntP, snLongP )"));
    assert!(upstream_math.contains("HBTEST Min( NIL, NIL )"));
    assert!(upstream_math.contains("HBTEST Min( 10, NIL )"));
    assert!(upstream_math.contains("HBTEST Min( snIntP, snLongP )"));
    assert!(upstream_rtl.contains("hb_errRT_BASE_SubstR( EG_ARG, 1093"));
    assert!(upstream_rtl.contains("hb_errRT_BASE_SubstR( EG_ARG, 1092"));

    assert_eq!(runtime_max_min_baseline(), expected);
}
