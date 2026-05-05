use std::fs;

mod support;
use support::{read_upstream_or_skip, workspace_fixture};

use harbour_rust_parser::parse;
use harbour_rust_runtime::{
    RuntimeError, Value, hb_gzcompress, hb_gzcompress_with_nresult, hb_gzcompress_with_options,
    len, valtype,
};

fn runtime_gz_compress_baseline() -> String {
    let compressed = hb_gzcompress(Some(&Value::from("abc"))).expect("gzip");

    let mut out = String::new();
    out.push_str(&format!(
        "ValType(hb_gzCompress(\"abc\")) => {}\n",
        result_text(valtype(Some(&compressed)))
    ));
    out.push_str(&format!(
        "Len(hb_gzCompress(\"abc\")) => {}\n",
        result_text(len(Some(&compressed)))
    ));
    out.push_str(&format!(
        "hb_gzCompress(\"abc\") prefix => {}\n",
        gzip_prefix_hex(&compressed)
    ));
    out.push_str(&format!(
        "Len(hb_gzCompress(\"\")) => {}\n",
        result_text(len(Some(
            &hb_gzcompress(Some(&Value::from(""))).expect("empty gzip"),
        )))
    ));
    out.push_str(&format!(
        "hb_gzCompress(10) => {}\n",
        result_text(hb_gzcompress(Some(&Value::from(10_i64))))
    ));
    out
}

fn runtime_gz_compress_nresult_baseline() -> String {
    let mut out = String::new();

    let mut arguments = [Value::from("abc"), Value::Nil, Value::from(-1_i64)];
    let compressed = hb_gzcompress_with_nresult(&mut arguments).expect("gzip with result");
    out.push_str(&format!(
        "ValType(hb_gzCompress(\"abc\", NIL, @nResult)) => {}\n",
        result_text(valtype(Some(&compressed)))
    ));
    out.push_str(&format!(
        "Len(hb_gzCompress(\"abc\", NIL, @nResult)) => {}\n",
        result_text(len(Some(&compressed)))
    ));
    out.push_str(&format!(
        "nResult after hb_gzCompress(\"abc\", NIL, @nResult) => {}\n",
        arguments[2].to_output_string()
    ));

    let mut empty_arguments = [Value::from(""), Value::Nil, Value::from(-1_i64)];
    let empty = hb_gzcompress_with_nresult(&mut empty_arguments).expect("empty gzip with result");
    out.push_str(&format!(
        "Len(hb_gzCompress(\"\", NIL, @nResult)) => {}\n",
        result_text(len(Some(&empty)))
    ));
    out.push_str(&format!(
        "nResult after hb_gzCompress(\"\", NIL, @nResult) => {}\n",
        empty_arguments[2].to_output_string()
    ));

    let mut sized_arguments = [Value::from("abc"), Value::from(26_i64), Value::from(-1_i64)];
    let sized = hb_gzcompress_with_nresult(&mut sized_arguments).expect("gzip sized result");
    out.push_str(&format!(
        "ValType(hb_gzCompress(\"abc\", 26, @nResult)) => {}\n",
        result_text(valtype(Some(&sized)))
    ));
    out.push_str(&format!(
        "nResult after hb_gzCompress(\"abc\", 26, @nResult) => {}\n",
        sized_arguments[2].to_output_string()
    ));

    let mut small_arguments = [Value::from("abc"), Value::from(25_i64), Value::from(-1_i64)];
    let small = hb_gzcompress_with_nresult(&mut small_arguments).expect("small gzip result");
    out.push_str(&format!(
        "ValType(hb_gzCompress(\"abc\", 25, @nResult)) => {}\n",
        result_text(valtype(Some(&small)))
    ));
    out.push_str(&format!(
        "nResult after hb_gzCompress(\"abc\", 25, @nResult) => {}\n",
        small_arguments[2].to_output_string()
    ));

    let immutable_small_args = [Value::from("abc"), Value::from(25_i64)];
    let immutable_small_arg_refs = immutable_small_args.iter().collect::<Vec<_>>();
    let immutable_small =
        hb_gzcompress_with_options(immutable_small_arg_refs.as_slice()).expect("small gzip");
    out.push_str(&format!(
        "ValType(hb_gzCompress(\"abc\", 25)) => {}\n",
        result_text(valtype(Some(&immutable_small)))
    ));

    let mut invalid_arguments = [Value::from("abc"), Value::from(true), Value::from(-1_i64)];
    out.push_str(&format!(
        "hb_gzCompress(\"abc\", .T., @nResult) => {}\n",
        result_text(hb_gzcompress_with_nresult(&mut invalid_arguments))
    ));

    out
}

fn result_text(result: Result<Value, RuntimeError>) -> String {
    match result {
        Ok(value) => value.to_output_string(),
        Err(error) => error.message,
    }
}

fn gzip_prefix_hex(value: &Value) -> String {
    let Value::String(bytes) = value else {
        panic!("expected string result");
    };
    bytes.as_bytes()[..3]
        .iter()
        .map(|byte| format!("{byte:02X}"))
        .collect::<String>()
}

#[test]
fn gz_compress_fixture_parses_without_errors() {
    let source = fs::read_to_string(workspace_fixture(
        "tests/fixtures/compat/gz_compress_runtime.prg",
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
fn gz_compress_nresult_fixture_parses_without_errors() {
    let source = fs::read_to_string(workspace_fixture(
        "tests/fixtures/compat/gz_compress_nresult_runtime.prg",
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
fn gz_compress_runtime_matches_the_documented_phase16_oracle_slice() {
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
        "tests/fixtures/compat/gz_compress_runtime.out",
    ))
    .expect("fixture snapshot");

    assert!(upstream_rtl.contains("hb_gzCompress( <cData>, [<nDstBufLen>|<@cBuffer>]"));
    assert!(upstream_rtl.contains("HB_FUNC( HB_GZCOMPRESS )"));
    assert!(upstream_hbx.contains("DYNAMIC hb_gzCompress"));

    assert_eq!(runtime_gz_compress_baseline(), expected);
}

#[test]
fn gz_compress_nresult_runtime_matches_the_documented_phase16_oracle_slice() {
    let Some(upstream_rtl) = read_upstream_or_skip("harbour-core/src/rtl/hbzlib.c", "upstream rtl")
    else {
        return;
    };
    let expected = fs::read_to_string(workspace_fixture(
        "tests/fixtures/compat/gz_compress_nresult_runtime.out",
    ))
    .expect("fixture snapshot");

    assert!(upstream_rtl.contains("hb_storni( iResult, 3 );"));
    assert!(upstream_rtl.contains("hb_gzCompress( <cData>, [<nDstBufLen>|<@cBuffer>]"));

    assert_eq!(runtime_gz_compress_nresult_baseline(), expected);
}
