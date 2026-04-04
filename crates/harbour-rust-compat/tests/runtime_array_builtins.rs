use std::fs;

mod support;
use support::{read_upstream_or_skip, workspace_fixture};

use harbour_rust_parser::parse;
use harbour_rust_runtime::{Value, adel, ains, ascan, valtype};

fn runtime_array_builtins_baseline() -> String {
    let mut values = Value::array(vec![
        Value::from(10_i64),
        Value::from(20_i64),
        Value::from(30_i64),
    ]);
    let words = Value::array(vec![
        Value::from("HELLO"),
        Value::from(""),
        Value::from("WORLD"),
    ]);

    let mut out = String::new();
    out.push_str(&format!(
        "AScan({{10,20,30}}, 20) => {}\n",
        value_text(ascan(Some(&values), Some(&Value::from(20_i64)), None, None))
    ));
    out.push_str(&format!(
        "AScan({{10,20,30}}, 0) => {}\n",
        value_text(ascan(Some(&values), Some(&Value::from(0_i64)), None, None))
    ));
    out.push_str(&format!(
        "AScan({{\"HELLO\",\"\",\"WORLD\"}}, \"HELL\") => {}\n",
        value_text(ascan(Some(&words), Some(&Value::from("HELL")), None, None))
    ));
    out.push_str(&format!(
        "AIns(values, 2) => {}\n",
        value_text(ains(&mut values, Some(&Value::from(2_i64))))
    ));
    out.push_str(&format!(
        "values[1] => {}\n",
        values
            .array_get_owned(&Value::from(1_i64))
            .unwrap()
            .to_output_string()
    ));
    out.push_str(&format!(
        "ValType(values[2]) => {}\n",
        value_text(valtype(Some(
            values.array_get(&Value::from(2_i64)).unwrap()
        )))
    ));
    out.push_str(&format!(
        "values[3] => {}\n",
        values
            .array_get_owned(&Value::from(3_i64))
            .unwrap()
            .to_output_string()
    ));
    out.push_str(&format!(
        "ADel(values, 1) => {}\n",
        value_text(adel(&mut values, Some(&Value::from(1_i64))))
    ));
    out.push_str(&format!(
        "ValType(values[1]) => {}\n",
        value_text(valtype(Some(
            values.array_get(&Value::from(1_i64)).unwrap()
        )))
    ));
    out.push_str(&format!(
        "values[2] => {}\n",
        values
            .array_get_owned(&Value::from(2_i64))
            .unwrap()
            .to_output_string()
    ));
    out.push_str(&format!(
        "ValType(values[3]) => {}\n",
        value_text(valtype(Some(
            values.array_get(&Value::from(3_i64)).unwrap()
        )))
    ));
    out
}

fn value_text(result: Result<Value, harbour_rust_runtime::RuntimeError>) -> String {
    match result {
        Ok(value) => value.to_output_string(),
        Err(error) => error.message,
    }
}

#[test]
fn array_builtins_fixture_parses_without_errors() {
    let source = fs::read_to_string(workspace_fixture(
        "tests/fixtures/compat/array_builtins_runtime.prg",
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
fn array_builtins_runtime_matches_upstream_oracle_snapshot() {
    let Some(upstream) =
        read_upstream_or_skip("harbour-core/utils/hbtest/rt_array.prg", "upstream hbtest")
    else {
        return;
    };
    let expected = fs::read_to_string(workspace_fixture(
        "tests/fixtures/compat/array_builtins_runtime.out",
    ))
    .expect("fixture snapshot");

    assert!(upstream.contains("HBTEST AScan( saAllTypes, scString )   IS 1"));
    assert!(upstream.contains("HBTEST AScan( saAllTypes, scStringE  ) IS 1"));
    assert!(upstream.contains("HBTEST ADel( { 1 }, 1 )                IS \"{.[1].}\""));
    assert!(upstream.contains("HBTEST AIns( { 1 }, 1 )                IS \"{.[1].}\""));

    assert_eq!(runtime_array_builtins_baseline(), expected);
}
