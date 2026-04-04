use std::fs;

mod support;
use support::{read_upstream_or_skip, workspace_fixture};

use harbour_rust_parser::parse;
use harbour_rust_runtime::{RuntimeError, Value, ltrim, rtrim, trim};

fn runtime_trim_baseline() -> String {
    let mut out = String::new();
    out.push_str(&format!(
        "Trim(\"UA   \") => {}\n",
        result_text(trim(Some(&Value::from("UA   "))))
    ));
    out.push_str(&format!(
        "Trim(\"   UA  \") => {}\n",
        result_text(trim(Some(&Value::from("   UA  "))))
    ));
    out.push_str(&format!(
        "Trim(100) => {}\n",
        result_text(trim(Some(&Value::from(100_i64))))
    ));
    out.push_str(&format!(
        "RTrim(\"   UA  \") => {}\n",
        result_text(rtrim(Some(&Value::from("   UA  "))))
    ));
    out.push_str(&format!(
        "RTrim(NIL) => {}\n",
        result_text(rtrim(Some(&Value::Nil)))
    ));
    out.push_str(&format!(
        "LTrim(\"   UA  \") => {}\n",
        result_text(ltrim(Some(&Value::from("   UA  "))))
    ));
    out.push_str(&format!(
        "LTrim(\" \\tUA  \") => {}\n",
        result_text(ltrim(Some(&Value::from(" \tUA  "))))
    ));
    out.push_str(&format!(
        "LTrim(100) => {}\n",
        result_text(ltrim(Some(&Value::from(100_i64))))
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
fn trim_fixture_parses_without_errors() {
    let source = fs::read_to_string(workspace_fixture("tests/fixtures/compat/trim_runtime.prg"))
        .expect("fixture source");
    let parsed = parse(&source);

    assert!(
        parsed.errors.is_empty(),
        "expected parse success, got {:?}",
        parsed.errors
    );
}

#[test]
fn trim_runtime_matches_upstream_oracle_snapshot() {
    let Some(upstream) =
        read_upstream_or_skip("harbour-core/utils/hbtest/rt_str.prg", "upstream hbtest")
    else {
        return;
    };
    let expected = fs::read_to_string(workspace_fixture("tests/fixtures/compat/trim_runtime.out"))
        .expect("fixture snapshot");

    assert!(upstream.contains("HBTEST Trim( scString )"));
    assert!(upstream.contains("BASE 1100 Argument error (TRIM)"));
    assert!(upstream.contains("HBTEST RTrim( scString )"));
    assert!(upstream.contains("HBTEST LTrim( scString )"));
    assert!(upstream.contains("BASE 1101 Argument error (LTRIM)"));

    assert_eq!(runtime_trim_baseline(), expected);
}
