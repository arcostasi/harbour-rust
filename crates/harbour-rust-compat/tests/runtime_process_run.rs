use std::fs;

mod support;
use support::{read_upstream_or_skip, workspace_fixture};

use harbour_rust_parser::parse;
use harbour_rust_runtime::{RuntimeError, Value, hb_processrun};

fn runtime_process_run_baseline() -> String {
    let mut out = String::new();
    out.push_str(&format!(
        "hb_processRun(\"exit 7\") => {}\n",
        result_text(hb_processrun(Some(&Value::from("exit 7"))))
    ));
    out.push_str(&format!(
        "hb_processRun(10) => {}\n",
        result_text(hb_processrun(Some(&Value::from(10_i64))))
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
fn process_run_fixture_parses_without_errors() {
    let source = fs::read_to_string(workspace_fixture(
        "tests/fixtures/compat/process_run_runtime.prg",
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
fn process_run_runtime_matches_the_documented_phase16_oracle_slice() {
    let Some(upstream_rtl) =
        read_upstream_or_skip("harbour-core/src/rtl/hbprocfn.c", "upstream rtl")
    else {
        return;
    };
    let Some(upstream_hbx) =
        read_upstream_or_skip("harbour-core/include/harbour.hbx", "upstream hbx")
    else {
        return;
    };
    let expected = fs::read_to_string(workspace_fixture(
        "tests/fixtures/compat/process_run_runtime.out",
    ))
    .expect("fixture snapshot");

    assert!(upstream_rtl.contains("hb_processRun( <cCommand>"));
    assert!(upstream_rtl.contains("HB_FUNC( HB_PROCESSRUN )"));
    assert!(upstream_hbx.contains("DYNAMIC hb_processRun"));

    assert_eq!(runtime_process_run_baseline(), expected);
}
