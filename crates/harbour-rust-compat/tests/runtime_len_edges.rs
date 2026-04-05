use std::fs;

mod support;
use support::{read_upstream_or_skip, workspace_fixture};

use harbour_rust_parser::parse;
use harbour_rust_runtime::{RuntimeError, Value, len};

fn runtime_len_edges_baseline() -> String {
    let mut out = String::new();
    out.push_str(&format!(
        "Len(\"\") => {}\n",
        result_text(len(Some(&Value::from(""))))
    ));
    out.push_str(&format!(
        "Len(\"123\\\\0 456\") => {}\n",
        result_text(len(Some(&Value::from(String::from("123\u{0000}456 ")))))
    ));
    out.push_str(&format!(
        "Len({{}}) => {}\n",
        result_text(len(Some(&Value::empty_array())))
    ));
    out.push_str(&format!(
        "Len({{1}}) => {}\n",
        result_text(len(Some(&Value::array(vec![Value::from(1_i64)]))))
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
fn len_edges_fixture_parses_without_errors() {
    let source = fs::read_to_string(workspace_fixture(
        "tests/fixtures/compat/len_edges_runtime.prg",
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
fn len_edges_runtime_matches_upstream_oracle_snapshot() {
    let Some(upstream_hvm) =
        read_upstream_or_skip("harbour-core/utils/hbtest/rt_hvm.prg", "upstream hbtest")
    else {
        return;
    };
    let Some(upstream_hvma) =
        read_upstream_or_skip("harbour-core/utils/hbtest/rt_hvma.prg", "upstream hbtest")
    else {
        return;
    };
    let expected = fs::read_to_string(workspace_fixture(
        "tests/fixtures/compat/len_edges_runtime.out",
    ))
    .expect("fixture snapshot");

    assert!(upstream_hvma.contains("HBTEST Len( \"\" )"));
    assert!(upstream_hvma.contains("HBTEST Len( \"123\" + Chr( 0 ) + \"456 \" )"));
    assert!(upstream_hvma.contains("HBTEST Len( saArray )"));
    assert!(upstream_hvm.contains("HBTEST Type( \"maArray\"   )             IS \"A\""));

    assert_eq!(runtime_len_edges_baseline(), expected);
}
