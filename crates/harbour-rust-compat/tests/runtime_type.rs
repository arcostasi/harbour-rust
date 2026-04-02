use std::{fs, path::PathBuf};

use harbour_rust_parser::parse;
use harbour_rust_runtime::{RuntimeError, Value, type_value};

fn workspace_fixture(path: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join(path)
}

fn runtime_type_baseline() -> String {
    let mut out = String::new();
    out.push_str(&format!(
        "Type(\"NIL\") => {}\n",
        result_text(type_value(Some(&Value::from("NIL"))))
    ));
    out.push_str(&format!(
        "Type(\".T.\") => {}\n",
        result_text(type_value(Some(&Value::from(".T."))))
    ));
    out.push_str(&format!(
        "Type(\"10.5\") => {}\n",
        result_text(type_value(Some(&Value::from("10.5"))))
    ));
    out.push_str(&format!(
        "Type(\"{{1,2}}\") => {}\n",
        result_text(type_value(Some(&Value::from("{ 1, 2 }"))))
    ));
    out.push_str(&format!(
        "Type(\"'abc'\") => {}\n",
        result_text(type_value(Some(&Value::from("'abc'"))))
    ));
    out.push_str(&format!(
        "Type(\"mxNotHere\") => {}\n",
        result_text(type_value(Some(&Value::from("mxNotHere"))))
    ));
    out.push_str(&format!("Type() => {}\n", result_text(type_value(None))));
    out.push_str(&format!(
        "Type(100) => {}\n",
        result_text(type_value(Some(&Value::from(100_i64))))
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
fn type_fixture_parses_without_errors() {
    let source = fs::read_to_string(workspace_fixture("tests/fixtures/compat/type_runtime.prg"))
        .expect("fixture source");
    let parsed = parse(&source);

    assert!(
        parsed.errors.is_empty(),
        "expected parse success, got {:?}",
        parsed.errors
    );
}

#[test]
fn type_runtime_matches_upstream_oracle_snapshot_for_current_slice() {
    let upstream = fs::read_to_string(workspace_fixture("harbour-core/utils/hbtest/rt_hvm.prg"))
        .expect("upstream hbtest");
    let upstream_rtl =
        fs::read_to_string(workspace_fixture("harbour-core/src/rtl/type.c")).expect("upstream rtl");
    let expected = fs::read_to_string(workspace_fixture("tests/fixtures/compat/type_runtime.out"))
        .expect("fixture snapshot");

    assert!(upstream.contains("HBTEST Type( NIL )"));
    assert!(upstream.contains("HBTEST Type( 100 )"));
    assert!(upstream.contains("HBTEST Type( \"mxNotHere\"  )            IS \"U\""));
    assert!(upstream.contains("HBTEST Type( \"maArray\"   )             IS \"A\""));
    assert!(upstream_rtl.contains("hb_errRT_BASE_SubstR( EG_ARG, 1121, NULL, HB_ERR_FUNCNAME"));
    assert!(upstream_rtl.contains("hb_macroGetType( pItem )"));

    assert_eq!(runtime_type_baseline(), expected);
}
