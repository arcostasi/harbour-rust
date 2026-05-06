use std::fs;

mod support;
use support::{read_upstream_or_skip, workspace_fixture};

use harbour_rust_parser::parse;
use harbour_rust_runtime::{RuntimeError, Value, hb_zcompressbound};

fn runtime_z_compress_bound_baseline() -> String {
    let mut out = String::new();
    out.push_str(&format!(
        "hb_ZCompressBound(\"abc\") => {}\n",
        result_text(hb_zcompressbound(Some(&Value::from("abc"))))
    ));
    out.push_str(&format!(
        "hb_ZCompressBound(10) => {}\n",
        result_text(hb_zcompressbound(Some(&Value::from(10_i64))))
    ));
    out.push_str(&format!(
        "hb_ZCompressBound(0) => {}\n",
        result_text(hb_zcompressbound(Some(&Value::from(0_i64))))
    ));
    out.push_str(&format!(
        "hb_ZCompressBound(.T.) => {}\n",
        result_text(hb_zcompressbound(Some(&Value::from(true))))
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
fn z_compress_bound_fixture_parses_without_errors() {
    let source = fs::read_to_string(workspace_fixture(
        "tests/fixtures/compat/z_compress_bound_runtime.prg",
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
fn z_compress_bound_runtime_matches_the_documented_phase16_oracle_slice() {
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
        "tests/fixtures/compat/z_compress_bound_runtime.out",
    ))
    .expect("fixture snapshot");

    assert!(upstream_rtl.contains("HB_FUNC( HB_ZCOMPRESSBOUND )"));
    assert!(upstream_rtl.contains("s_zlibCompressBound"));
    assert!(upstream_hbx.contains("DYNAMIC hb_ZCompressBound"));

    assert_eq!(runtime_z_compress_bound_baseline(), expected);
}
