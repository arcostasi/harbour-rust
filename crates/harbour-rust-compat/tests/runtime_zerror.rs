use std::fs;

mod support;
use support::{read_upstream_or_skip, workspace_fixture};

use harbour_rust_parser::parse;
use harbour_rust_runtime::{RuntimeError, Value, hb_zerror};

fn runtime_zerror_baseline() -> String {
    let mut out = String::new();
    out.push_str(&format!(
        "hb_ZError(0) => {}\n",
        result_text(hb_zerror(Some(&Value::from(0_i64))))
    ));
    out.push_str(&format!(
        "hb_ZError(-5) => {}\n",
        result_text(hb_zerror(Some(&Value::from(-5_i64))))
    ));
    out.push_str(&format!(
        "hb_ZError(-6) => {}\n",
        result_text(hb_zerror(Some(&Value::from(-6_i64))))
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
fn zerror_fixture_parses_without_errors() {
    let source = fs::read_to_string(workspace_fixture(
        "tests/fixtures/compat/zerror_runtime.prg",
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
fn zerror_runtime_matches_the_documented_phase16_oracle_slice() {
    let Some(upstream_rtl) = read_upstream_or_skip("harbour-core/src/rtl/hbzlib.c", "upstream rtl")
    else {
        return;
    };
    let Some(upstream_hbx) =
        read_upstream_or_skip("harbour-core/include/harbour.hbx", "upstream hbx")
    else {
        return;
    };
    let expected = fs::read_to_string(workspace_fixture(
        "tests/fixtures/compat/zerror_runtime.out",
    ))
    .expect("fixture snapshot");

    assert!(upstream_rtl.contains("HB_FUNC( HB_ZERROR )"));
    assert!(upstream_hbx.contains("DYNAMIC hb_ZError"));

    assert_eq!(runtime_zerror_baseline(), expected);
}
