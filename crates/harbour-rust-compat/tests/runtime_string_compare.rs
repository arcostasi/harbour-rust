use std::fs;

mod support;
use support::{read_upstream_or_skip, workspace_fixture};

use harbour_rust_parser::parse;
use harbour_rust_runtime::Value;

fn runtime_string_compare_baseline() -> String {
    let mut out = String::new();
    out.push_str(&format!(
        "\"12345\" = \"123\" => {}\n",
        logical_text(Value::from("12345").equals(&Value::from("123")))
    ));
    out.push_str(&format!(
        "\"123\" = \"12345\" => {}\n",
        logical_text(Value::from("123").equals(&Value::from("12345")))
    ));
    out.push_str(&format!(
        "\"123\" = \"\" => {}\n",
        logical_text(Value::from("123").equals(&Value::from("")))
    ));
    out.push_str(&format!(
        "\"\" = \"123\" => {}\n",
        logical_text(Value::from("").equals(&Value::from("123")))
    ));
    out.push_str(&format!(
        "\"A\" == \"A\" => {}\n",
        logical_text(Value::from("A").exact_equals(&Value::from("A")))
    ));
    out.push_str(&format!(
        "\"AA\" == \"A\" => {}\n",
        logical_text(Value::from("AA").exact_equals(&Value::from("A")))
    ));
    out.push_str(&format!(
        "\"AA\" != \"A\" => {}\n",
        logical_text(Value::from("AA").not_equals(&Value::from("A")))
    ));
    out.push_str(&format!(
        "\"Z\" != \"A\" => {}\n",
        logical_text(Value::from("Z").not_equals(&Value::from("A")))
    ));
    out
}

fn logical_text(result: Result<Value, harbour_rust_runtime::RuntimeError>) -> &'static str {
    match result {
        Ok(Value::Logical(true)) => ".T.",
        Ok(Value::Logical(false)) => ".F.",
        Ok(_) => panic!("expected logical result"),
        Err(error) => panic!("expected successful comparison, got {error:?}"),
    }
}

#[test]
fn string_compare_fixture_parses_without_errors() {
    let source = fs::read_to_string(workspace_fixture(
        "tests/fixtures/compat/string_compare_runtime.prg",
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
fn string_compare_runtime_matches_upstream_oracle_snapshot() {
    let Some(upstream) =
        read_upstream_or_skip("harbour-core/utils/hbtest/rt_hvm.prg", "upstream hbtest")
    else {
        return;
    };
    let expected = fs::read_to_string(workspace_fixture(
        "tests/fixtures/compat/string_compare_runtime.out",
    ))
    .expect("fixture snapshot");

    assert!(upstream.contains("Set( _SET_EXACT, .F. )"));
    assert!(upstream.contains("HBTEST \"12345\" = \"123\"                 IS .T."));
    assert!(upstream.contains("HBTEST \"123\" = \"12345\"                 IS .F."));
    assert!(upstream.contains("HBTEST \"123\" = \"\"                      IS .T."));
    assert!(upstream.contains("HBTEST \"\" = \"123\"                      IS .F."));
    assert!(upstream.contains("HBTEST \"A\" == \"A\"                      IS .T."));
    assert!(upstream.contains("HBTEST \"AA\" == \"A\"                     IS .F."));
    assert!(upstream.contains("HBTEST \"AA\" != \"A\"                     IS .F."));
    assert!(upstream.contains("HBTEST \"Z\" != \"A\"                      IS .T."));

    assert_eq!(runtime_string_compare_baseline(), expected);
}
