use std::{fs, path::PathBuf};

use harbour_rust_parser::parse;
use harbour_rust_runtime::{RuntimeError, Value, replicate, space};

fn workspace_fixture(path: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join(path)
}

fn runtime_replicate_space_baseline() -> String {
    let mut out = String::new();
    out.push_str(&format!(
        "Replicate(200, 0) => {}\n",
        result_text(replicate(
            Some(&Value::from(200_i64)),
            Some(&Value::from(0_i64)),
        ))
    ));
    out.push_str(&format!(
        "Replicate(\"\", 10) => {}\n",
        result_text(replicate(
            Some(&Value::from("")),
            Some(&Value::from(10_i64)),
        ))
    ));
    out.push_str(&format!(
        "Replicate(\"A\", \"B\") => {}\n",
        result_text(replicate(Some(&Value::from("A")), Some(&Value::from("B")),))
    ));
    out.push_str(&format!(
        "Replicate(\"A\", 2) => {}\n",
        result_text(replicate(
            Some(&Value::from("A")),
            Some(&Value::from(2_i64)),
        ))
    ));
    out.push_str(&format!(
        "Replicate(\"HE\", 3.7) => {}\n",
        result_text(replicate(
            Some(&Value::from("HE")),
            Some(&Value::from(3.7_f64)),
        ))
    ));
    out.push_str(&format!(
        "Replicate(\"HE\", -3) => {}\n",
        result_text(replicate(
            Some(&Value::from("HE")),
            Some(&Value::from(-3_i64)),
        ))
    ));
    out.push_str(&format!(
        "Space(\"A\") => {}\n",
        result_text(space(Some(&Value::from("A"))))
    ));
    out.push_str(&format!(
        "Space(0) => {}\n",
        result_text(space(Some(&Value::from(0_i64))))
    ));
    out.push_str(&format!(
        "Space(-10) => {}\n",
        result_text(space(Some(&Value::from(-10_i64))))
    ));
    out.push_str(&format!(
        "Space(3) => {}\n",
        result_text(space(Some(&Value::from(3_i64))))
    ));
    out.push_str(&format!(
        "Space(3.7) => {}\n",
        result_text(space(Some(&Value::from(3.7_f64))))
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
fn replicate_space_fixture_parses_without_errors() {
    let source = fs::read_to_string(workspace_fixture(
        "tests/fixtures/compat/replicate_space_runtime.prg",
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
fn replicate_space_runtime_matches_upstream_oracle_snapshot() {
    let upstream = fs::read_to_string(workspace_fixture("harbour-core/utils/hbtest/rt_str.prg"))
        .expect("upstream hbtest");
    let expected = fs::read_to_string(workspace_fixture(
        "tests/fixtures/compat/replicate_space_runtime.out",
    ))
    .expect("fixture snapshot");

    assert!(upstream.contains("HBTEST Replicate( 200  , 0 )"));
    assert!(upstream.contains("HBTEST Replicate( \"HE\", 3.7 )"));
    assert!(upstream.contains("BASE 1106 Argument error (REPLICATE)"));
    assert!(upstream.contains("HBTEST Space( \"A\" )"));
    assert!(upstream.contains("HBTEST Space( 10.7 )"));
    assert!(upstream.contains("BASE 1105 Argument error (SPACE)"));

    assert_eq!(runtime_replicate_space_baseline(), expected);
}
