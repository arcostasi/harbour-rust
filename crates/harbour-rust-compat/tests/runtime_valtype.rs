use std::fs;

mod support;
use support::{read_upstream_or_skip, workspace_fixture};

use harbour_rust_parser::parse;
use harbour_rust_runtime::{RuntimeError, Value, valtype};

fn runtime_valtype_baseline() -> String {
    let mut out = String::new();
    out.push_str(&format!("ValType() => {}\n", result_text(valtype(None))));
    out.push_str(&format!(
        "ValType(NIL) => {}\n",
        result_text(valtype(Some(&Value::Nil)))
    ));
    out.push_str(&format!(
        "ValType(.T.) => {}\n",
        result_text(valtype(Some(&Value::from(true))))
    ));
    out.push_str(&format!(
        "ValType(10) => {}\n",
        result_text(valtype(Some(&Value::from(10_i64))))
    ));
    out.push_str(&format!(
        "ValType(10.5) => {}\n",
        result_text(valtype(Some(&Value::from(10.5_f64))))
    ));
    out.push_str(&format!(
        "ValType(\"abc\") => {}\n",
        result_text(valtype(Some(&Value::from("abc"))))
    ));
    out.push_str(&format!(
        "ValType({{1,2,3}}) => {}\n",
        result_text(valtype(Some(&Value::array(vec![
            Value::from(1_i64),
            Value::from(2_i64),
            Value::from(3_i64),
        ]))))
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
fn valtype_fixture_parses_without_errors() {
    let source = fs::read_to_string(workspace_fixture(
        "tests/fixtures/compat/valtype_runtime.prg",
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
fn valtype_runtime_matches_upstream_oracle_snapshot() {
    let Some(upstream) =
        read_upstream_or_skip("harbour-core/utils/hbtest/rt_hvm.prg", "upstream hbtest")
    else {
        return;
    };
    let Some(upstream_rtl) =
        read_upstream_or_skip("harbour-core/src/rtl/valtype.c", "upstream rtl")
    else {
        return;
    };
    let expected = fs::read_to_string(workspace_fixture(
        "tests/fixtures/compat/valtype_runtime.out",
    ))
    .expect("fixture snapshot");

    assert!(upstream.contains("HBTEST ValType(  scString  )           IS \"C\""));
    assert!(upstream.contains("HBTEST ValType(  slTrue    )           IS \"L\""));
    assert!(upstream.contains("HBTEST ValType(  suNIL     )           IS \"U\""));
    assert!(upstream.contains("HBTEST ValType( { 1, 2, 3 } )          IS \"A\""));
    assert!(upstream_rtl.contains("hb_itemTypeStr( hb_param( 1, HB_IT_ANY ) )"));

    assert_eq!(runtime_valtype_baseline(), expected);
}
