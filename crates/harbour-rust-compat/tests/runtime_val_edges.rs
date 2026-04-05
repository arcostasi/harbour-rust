use std::fs;

mod support;
use support::{read_upstream_or_skip, workspace_fixture};

use harbour_rust_parser::parse;
use harbour_rust_runtime::{RuntimeError, Value, str_value, val};

fn runtime_val_edges_baseline() -> String {
    let mut out = String::new();
    out.push_str(&format!(
        "Str(Val(\"1.\")) => {}\n",
        result_text(str_value_from_val("1."))
    ));
    out.push_str(&format!(
        "Str(Val(\"1..\")) => {}\n",
        result_text(str_value_from_val("1.."))
    ));
    out.push_str(&format!(
        "Str(Val(\"-.1\")) => {}\n",
        result_text(str_value_from_val("-.1"))
    ));
    out.push_str(&format!(
        "Str(Val(\" --12\")) => {}\n",
        result_text(str_value_from_val(" --12"))
    ));
    out.push_str(&format!(
        "Str(Val(\" 13.1.9\")) => {}\n",
        result_text(str_value_from_val(" 13.1.9"))
    ));
    out.push_str(&format!(
        "Str(Val(\"1E2\")) => {}\n",
        result_text(str_value_from_val("1E2"))
    ));
    out
}

fn str_value_from_val(source: &str) -> Result<Value, RuntimeError> {
    let parsed = val(Some(&Value::from(source)))?;
    str_value(Some(&parsed), None, None)
}

fn result_text(result: Result<Value, RuntimeError>) -> String {
    match result {
        Ok(value) => value.to_output_string(),
        Err(error) => error.message,
    }
}

#[test]
fn val_edges_fixture_parses_without_errors() {
    let source =
        fs::read_to_string(workspace_fixture("tests/fixtures/compat/val_edges_runtime.prg"))
            .expect("fixture source");
    let parsed = parse(&source);

    assert!(
        parsed.errors.is_empty(),
        "expected parse success, got {:?}",
        parsed.errors
    );
}

#[test]
fn val_edges_runtime_matches_upstream_oracle_snapshot() {
    let Some(upstream_str) =
        read_upstream_or_skip("harbour-core/utils/hbtest/rt_str.prg", "upstream rt_str")
    else {
        return;
    };
    let expected =
        fs::read_to_string(workspace_fixture("tests/fixtures/compat/val_edges_runtime.out"))
            .expect("fixture snapshot");

    assert!(upstream_str.contains("HBTEST Str( Val( \"1.\" ) )"));
    assert!(upstream_str.contains("HBTEST Str( Val( \"1..\" ) )"));
    assert!(upstream_str.contains("HBTEST Str( Val( \"-.1\" ) )"));
    assert!(upstream_str.contains("HBTEST Str( Val( \" --12\" ) )"));
    assert!(upstream_str.contains("HBTEST Str( Val( \" 13.1.9\" ) )"));
    assert!(upstream_str.contains("HBTEST Str( Val( \"1E2\" ) )"));

    assert_eq!(runtime_val_edges_baseline(), expected);
}
