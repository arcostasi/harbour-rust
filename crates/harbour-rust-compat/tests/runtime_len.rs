use std::{fs, path::PathBuf};

use harbour_rust_parser::parse;
use harbour_rust_runtime::{RuntimeError, Value, len};

fn workspace_fixture(path: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join(path)
}

fn runtime_len_baseline() -> String {
    let mut out = String::new();
    out.push_str(&format!(
        "Len(\"123\") => {}\n",
        result_text(len(Some(&Value::from("123"))))
    ));
    out.push_str(&format!(
        "Len({{1,2,3}}) => {}\n",
        result_text(len(Some(&Value::array(vec![
            Value::from(1_i64),
            Value::from(2_i64),
            Value::from(3_i64),
        ]))))
    ));
    out.push_str(&format!(
        "Len(NIL) => {}\n",
        result_text(len(Some(&Value::Nil)))
    ));
    out.push_str(&format!(
        "Len(123) => {}\n",
        result_text(len(Some(&Value::from(123_i64))))
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
fn len_fixture_parses_without_errors() {
    let source = fs::read_to_string(workspace_fixture("tests/fixtures/compat/len_runtime.prg"))
        .expect("fixture source");
    let parsed = parse(&source);

    assert!(
        parsed.errors.is_empty(),
        "expected parse success, got {:?}",
        parsed.errors
    );
}

#[test]
fn len_runtime_matches_upstream_oracle_snapshot() {
    let upstream = fs::read_to_string(workspace_fixture("harbour-core/utils/hbtest/rt_hvma.prg"))
        .expect("upstream hbtest");
    let expected = fs::read_to_string(workspace_fixture("tests/fixtures/compat/len_runtime.out"))
        .expect("fixture snapshot");

    assert!(upstream.contains("HBTEST Len( NIL )"));
    assert!(upstream.contains("BASE 1111 Argument error (LEN)"));
    assert!(upstream.contains("HBTEST Len( \"123\" )"));
    assert!(upstream.contains("HBTEST Len( saArray )"));

    assert_eq!(runtime_len_baseline(), expected);
}
