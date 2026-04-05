use std::fs;

mod support;
use support::{read_upstream_or_skip, workspace_fixture};

use harbour_rust_parser::parse;
use harbour_rust_runtime::{RuntimeError, Value, type_value};

fn runtime_type_edges_baseline() -> String {
    let mut out = String::new();
    out.push_str(&format!(
        "Type(\"  NIL  \") => {}\n",
        result_text(type_value(Some(&Value::from("  NIL  "))))
    ));
    out.push_str(&format!(
        "Type(\"  .F.  \") => {}\n",
        result_text(type_value(Some(&Value::from("  .F.  "))))
    ));
    out.push_str(&format!(
        "Type(\"{{}}\") => {}\n",
        result_text(type_value(Some(&Value::from("{}"))))
    ));
    out.push_str(&format!(
        "Type(\" {{ }} \") => {}\n",
        result_text(type_value(Some(&Value::from(" { } "))))
    ));
    out.push_str(&format!(
        "Type(\"  mxNotHere  \") => {}\n",
        result_text(type_value(Some(&Value::from("  mxNotHere  "))))
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
fn type_edges_fixture_parses_without_errors() {
    let source = fs::read_to_string(workspace_fixture(
        "tests/fixtures/compat/type_edges_runtime.prg",
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
fn type_edges_runtime_matches_upstream_oracle_snapshot_for_current_slice() {
    let Some(upstream) =
        read_upstream_or_skip("harbour-core/utils/hbtest/rt_hvm.prg", "upstream hbtest")
    else {
        return;
    };
    let Some(upstream_rtl) = read_upstream_or_skip("harbour-core/src/rtl/type.c", "upstream rtl")
    else {
        return;
    };
    let expected = fs::read_to_string(workspace_fixture(
        "tests/fixtures/compat/type_edges_runtime.out",
    ))
    .expect("fixture snapshot");

    assert!(upstream.contains("HBTEST Type( \"mxNotHere\"  )            IS \"U\""));
    assert!(upstream.contains("HBTEST Type( \"muNIL\"     )             IS \"U\""));
    assert!(upstream.contains("HBTEST Type( \"mlFalse\"   )             IS \"L\""));
    assert!(upstream.contains("HBTEST Type( \"maArray\"   )             IS \"A\""));
    assert!(upstream_rtl.contains("hb_macroGetType( pItem )"));

    assert_eq!(runtime_type_edges_baseline(), expected);
}
