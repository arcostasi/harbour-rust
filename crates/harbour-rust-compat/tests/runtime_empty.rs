use std::{fs, path::PathBuf};

use harbour_rust_parser::parse;
use harbour_rust_runtime::{RuntimeError, Value, empty};

fn workspace_fixture(path: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join(path)
}

fn runtime_empty_baseline() -> String {
    let mut out = String::new();
    out.push_str(&format!("Empty() => {}\n", result_text(empty(None))));
    out.push_str(&format!(
        "Empty(NIL) => {}\n",
        result_text(empty(Some(&Value::Nil)))
    ));
    out.push_str(&format!(
        "Empty(.F.) => {}\n",
        result_text(empty(Some(&Value::from(false))))
    ));
    out.push_str(&format!(
        "Empty(.T.) => {}\n",
        result_text(empty(Some(&Value::from(true))))
    ));
    out.push_str(&format!(
        "Empty(0) => {}\n",
        result_text(empty(Some(&Value::from(0_i64))))
    ));
    out.push_str(&format!(
        "Empty(10) => {}\n",
        result_text(empty(Some(&Value::from(10_i64))))
    ));
    out.push_str(&format!(
        "Empty(\"\") => {}\n",
        result_text(empty(Some(&Value::from(""))))
    ));
    out.push_str(&format!(
        "Empty(\"  \") => {}\n",
        result_text(empty(Some(&Value::from("  "))))
    ));
    out.push_str(&format!(
        "Empty(\" \\\\r\\\\t\") => {}\n",
        result_text(empty(Some(&Value::from(" \r\t"))))
    ));
    out.push_str(&format!(
        "Empty(\" \\\\0\") => {}\n",
        result_text(empty(Some(&Value::from(String::from(" \u{0000}")))))
    ));
    out.push_str(&format!(
        "Empty(\"A\") => {}\n",
        result_text(empty(Some(&Value::from("A"))))
    ));
    out.push_str(&format!(
        "Empty({{}}) => {}\n",
        result_text(empty(Some(&Value::empty_array())))
    ));
    out.push_str(&format!(
        "Empty({{0}}) => {}\n",
        result_text(empty(Some(&Value::array(vec![Value::from(0_i64)]))))
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
fn empty_fixture_parses_without_errors() {
    let source = fs::read_to_string(workspace_fixture("tests/fixtures/compat/empty_runtime.prg"))
        .expect("fixture source");
    let parsed = parse(&source);

    assert!(
        parsed.errors.is_empty(),
        "expected parse success, got {:?}",
        parsed.errors
    );
}

#[test]
fn empty_runtime_matches_upstream_oracle_snapshot() {
    let upstream = fs::read_to_string(workspace_fixture("harbour-core/utils/hbtest/rt_hvma.prg"))
        .expect("upstream hbtest");
    let upstream_rtl = fs::read_to_string(workspace_fixture("harbour-core/src/rtl/empty.c"))
        .expect("upstream rtl");
    let expected = fs::read_to_string(workspace_fixture("tests/fixtures/compat/empty_runtime.out"))
        .expect("fixture snapshot");

    assert!(upstream.contains("HBTEST Empty( \"Hallo\""));
    assert!(upstream.contains("HBTEST Empty( \"\""));
    assert!(upstream.contains("HBTEST Empty( \"  \""));
    assert!(upstream.contains("HBTEST Empty( \" \" + Chr( 13 ) + Chr( 9 )"));
    assert!(upstream.contains("HBTEST Empty( NIL"));
    assert!(upstream.contains("HBTEST Empty( {}"));
    assert!(upstream_rtl.contains("hb_strEmpty"));
    assert!(upstream_rtl.contains("case HB_IT_ARRAY:"));
    assert!(upstream_rtl.contains("default:"));

    assert_eq!(runtime_empty_baseline(), expected);
}
