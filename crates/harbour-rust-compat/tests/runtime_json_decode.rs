use std::fs;

mod support;
use support::{read_upstream_or_skip, workspace_fixture};

use harbour_rust_parser::parse;
use harbour_rust_runtime::{RuntimeError, Value, hb_jsondecode, len, valtype};

fn runtime_json_decode_baseline() -> String {
    let decoded = hb_jsondecode(Some(&Value::from("{\"ok\":true,\"items\":[1,null,\"x\"]}")))
        .expect("json decode");

    let mut out = String::new();
    out.push_str(&format!(
        "hb_JsonDecode(\"null\") => {}\n",
        result_text(hb_jsondecode(Some(&Value::from("null"))))
    ));
    out.push_str(&format!(
        "hb_JsonDecode(\"true\") => {}\n",
        result_text(hb_jsondecode(Some(&Value::from("true"))))
    ));
    out.push_str(&format!(
        "ValType(decoded) => {}\n",
        result_text(valtype(Some(&decoded)))
    ));
    out.push_str(&format!(
        "Len(decoded) => {}\n",
        result_text(len(Some(&decoded)))
    ));
    out.push_str(&format!(
        "decoded[1][1] => {}\n",
        value_text(decoded.array_get_path(&[Value::from(1_i64), Value::from(1_i64)]))
    ));
    out.push_str(&format!(
        "decoded[1][2] => {}\n",
        value_text(decoded.array_get_path(&[Value::from(1_i64), Value::from(2_i64)]))
    ));
    out.push_str(&format!(
        "decoded[2][1] => {}\n",
        value_text(decoded.array_get_path(&[Value::from(2_i64), Value::from(1_i64)]))
    ));
    out.push_str(&format!(
        "decoded[2][2][1] => {}\n",
        value_text(decoded.array_get_path(&[
            Value::from(2_i64),
            Value::from(2_i64),
            Value::from(1_i64),
        ]))
    ));
    out.push_str(&format!(
        "ValType(decoded[2][2][2]) => {}\n",
        result_text(valtype(Some(
            decoded
                .array_get_path(&[Value::from(2_i64), Value::from(2_i64), Value::from(2_i64)])
                .expect("decoded[2][2][2]"),
        )))
    ));
    out.push_str(&format!(
        "decoded[2][2][3] => {}\n",
        value_text(decoded.array_get_path(&[
            Value::from(2_i64),
            Value::from(2_i64),
            Value::from(3_i64),
        ]))
    ));
    out.push_str(&format!(
        "ValType(hb_JsonDecode(\"{{\")) => {}\n",
        result_text(valtype(Some(
            &hb_jsondecode(Some(&Value::from("{"))).expect("invalid json"),
        )))
    ));
    out
}

fn result_text(result: Result<Value, RuntimeError>) -> String {
    match result {
        Ok(value) => value.to_output_string(),
        Err(error) => error.message,
    }
}

fn value_text(result: Result<&Value, RuntimeError>) -> String {
    match result {
        Ok(value) => value.to_output_string(),
        Err(error) => error.message,
    }
}

#[test]
fn json_decode_fixture_parses_without_errors() {
    let source = fs::read_to_string(workspace_fixture(
        "tests/fixtures/compat/json_decode_runtime.prg",
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
fn json_decode_runtime_matches_the_documented_phase16_oracle_slice() {
    let Some(upstream_rtl) = read_upstream_or_skip("harbour-core/src/rtl/hbjson.c", "upstream rtl")
    else {
        return;
    };
    let Some(upstream_hbx) =
        read_upstream_or_skip("harbour-core/include/harbour.hbx", "upstream hbx")
    else {
        return;
    };
    let expected = fs::read_to_string(workspace_fixture(
        "tests/fixtures/compat/json_decode_runtime.out",
    ))
    .expect("fixture snapshot");

    assert!(upstream_rtl.contains("hb_jsonDecode( cJSON ) --> xValue"));
    assert!(upstream_rtl.contains("hb_jsonDecode( cJSON, @xValue ) --> nLengthDecoded"));
    assert!(upstream_rtl.contains("HB_FUNC( HB_JSONDECODE )"));
    assert!(upstream_hbx.contains("DYNAMIC hb_jsonDecode"));

    assert_eq!(runtime_json_decode_baseline(), expected);
}
