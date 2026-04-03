use std::{fs, path::PathBuf};

use harbour_rust_parser::parse;
use harbour_rust_runtime::{RuntimeError, Value, round_value, tan_value};

fn workspace_fixture(path: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join(path)
}

fn runtime_tan_baseline() -> String {
    let mut out = String::new();
    out.push_str(&format!(
        "Tan(\"A\") => {}\n",
        result_text(tan_value(Some(&Value::from("A"))))
    ));
    out.push_str(&format!("Tan() => {}\n", result_text(tan_value(None))));
    out.push_str(&format!(
        "Tan(0) => {}\n",
        result_text(tan_value(Some(&Value::from(0_i64))))
    ));
    out.push_str(&format!(
        "Round(Tan(1), 4) => {}\n",
        result_text(round_value(
            tan_value(Some(&Value::from(1_i64))).ok().as_ref(),
            Some(&Value::from(4_i64))
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
fn tan_fixture_parses_without_errors() {
    let source = fs::read_to_string(workspace_fixture("tests/fixtures/compat/tan_runtime.prg"))
        .expect("fixture source");
    let parsed = parse(&source);

    assert!(
        parsed.errors.is_empty(),
        "expected parse success, got {:?}",
        parsed.errors
    );
}

#[test]
fn tan_runtime_matches_project_baseline_snapshot() {
    let upstream_test = fs::read_to_string(workspace_fixture(
        "harbour-core/contrib/hbct/tests/test.prg",
    ))
    .expect("upstream hbct test");
    let upstream_doc = fs::read_to_string(workspace_fixture(
        "harbour-core/contrib/hbct/doc/en/trig.txt",
    ))
    .expect("upstream hbct docs");
    let expected = fs::read_to_string(workspace_fixture("tests/fixtures/compat/tan_runtime.out"))
        .expect("fixture snapshot");

    assert!(upstream_test.contains("HBTEST Tan( 0.0 ) IS 0.0"));
    assert!(upstream_doc.contains("? Tan( 1.0 ) // --> 1.5574..."));

    assert_eq!(runtime_tan_baseline(), expected);
}
