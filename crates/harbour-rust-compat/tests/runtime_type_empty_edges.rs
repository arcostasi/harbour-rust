use std::fs;

mod support;
use support::{read_upstream_or_skip, workspace_fixture};

use harbour_rust_parser::parse;
use harbour_rust_runtime::{RuntimeError, Value, empty, valtype};

fn runtime_type_empty_edges_baseline() -> String {
    let mut out = String::new();
    out.push_str(&format!(
        "ValType(.F.) => {}\n",
        result_text(valtype(Some(&Value::from(false))))
    ));
    out.push_str(&format!(
        "ValType(\"\") => {}\n",
        result_text(valtype(Some(&Value::from(""))))
    ));
    out.push_str(&format!(
        "ValType({{}}) => {}\n",
        result_text(valtype(Some(&Value::empty_array())))
    ));
    out.push_str(&format!(
        "Empty(\" \\\\r\\\\t\") => {}\n",
        result_text(empty(Some(&Value::from(" \r\t"))))
    ));
    out.push_str(&format!(
        "Empty(\"  A\") => {}\n",
        result_text(empty(Some(&Value::from("  A"))))
    ));
    out.push_str(&format!(
        "Empty(\" \\\\0\") => {}\n",
        result_text(empty(Some(&Value::from(String::from(" \u{0000}")))))
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
fn type_empty_edges_fixture_parses_without_errors() {
    let source = fs::read_to_string(workspace_fixture(
        "tests/fixtures/compat/type_empty_edges_runtime.prg",
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
fn type_empty_edges_runtime_matches_upstream_oracle_snapshot() {
    let Some(upstream_hvm) =
        read_upstream_or_skip("harbour-core/utils/hbtest/rt_hvm.prg", "upstream rt_hvm")
    else {
        return;
    };
    let Some(upstream_hvma) =
        read_upstream_or_skip("harbour-core/utils/hbtest/rt_hvma.prg", "upstream rt_hvma")
    else {
        return;
    };
    let expected = fs::read_to_string(workspace_fixture(
        "tests/fixtures/compat/type_empty_edges_runtime.out",
    ))
    .expect("fixture snapshot");

    assert!(upstream_hvm.contains("HBTEST ValType(  slFalse   )           IS \"L\""));
    assert!(upstream_hvm.contains("HBTEST ValType(  scStringE )           IS \"C\""));
    assert!(upstream_hvm.contains("HBTEST ValType( { 1, 2, 3 } )          IS \"A\""));
    assert!(upstream_hvma.contains("HBTEST Empty( \" \" + Chr( 13 ) + Chr( 9 )"));
    assert!(upstream_hvma.contains("HBTEST Empty( \"  A\""));
    assert!(upstream_hvma.contains("HBTEST Empty( \" \" + Chr( 0 )"));
    assert!(upstream_hvma.contains("HBTEST Empty( {}"));
    assert!(upstream_hvma.contains("HBTEST Empty( { 0 }"));

    assert_eq!(runtime_type_empty_edges_baseline(), expected);
}
