use std::{fs, path::PathBuf};

use harbour_rust_parser::parse;
use harbour_rust_runtime::Value;

fn workspace_fixture(path: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join(path)
}

fn runtime_array_comparison_baseline() -> String {
    let same = Value::array(vec![Value::from(1_i64)]);
    let empty_left = Value::empty_array();
    let empty_right = Value::empty_array();

    let mut out = String::new();
    out.push_str(&format!(
        "same == same => {}\n",
        logical_text(same.exact_equals(&same))
    ));
    out.push_str(&format!(
        "{{}} == {{}} => {}\n",
        logical_text(empty_left.exact_equals(&empty_right))
    ));
    out.push_str(&format!(
        "same = same => {}\n",
        comparison_text(same.equals(&same))
    ));
    out.push_str(&format!(
        "same != same => {}\n",
        comparison_text(same.not_equals(&same))
    ));
    out.push_str(&format!(
        "same < same => {}\n",
        comparison_text(same.less_than(&same))
    ));
    out.push_str(&format!(
        "same <= same => {}\n",
        comparison_text(same.less_than_or_equal(&same))
    ));
    out.push_str(&format!(
        "same > same => {}\n",
        comparison_text(same.greater_than(&same))
    ));
    out.push_str(&format!(
        "same >= same => {}\n",
        comparison_text(same.greater_than_or_equal(&same))
    ));
    out
}

fn logical_text(result: Result<Value, harbour_rust_runtime::RuntimeError>) -> &'static str {
    match result {
        Ok(Value::Logical(true)) => ".T.",
        Ok(Value::Logical(false)) => ".F.",
        Ok(_) => panic!("expected logical result"),
        Err(_) => panic!("expected successful exact comparison"),
    }
}

fn comparison_text(result: Result<Value, harbour_rust_runtime::RuntimeError>) -> String {
    match result {
        Ok(value) => value.to_output_string(),
        Err(error) => error.message,
    }
}

#[test]
fn array_comparison_fixture_parses_without_errors() {
    let source = fs::read_to_string(workspace_fixture(
        "tests/fixtures/compat/array_comparison_runtime.prg",
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
fn array_comparison_runtime_matches_upstream_oracle_snapshot() {
    let upstream = fs::read_to_string(workspace_fixture("harbour-core/utils/hbtest/rt_hvm.prg"))
        .expect("upstream hbtest");
    let expected = fs::read_to_string(workspace_fixture(
        "tests/fixtures/compat/array_comparison_runtime.out",
    ))
    .expect("fixture snapshot");

    assert!(upstream.contains("HBTEST saArray == saArray              IS .T."));
    assert!(upstream.contains("HBTEST {} == {}                        IS .F."));
    assert!(upstream.contains("BASE 1071 Argument error (=)"));
    assert!(upstream.contains("BASE 1072 Argument error (<>)"));
    assert!(upstream.contains("BASE 1073 Argument error (<)"));
    assert!(upstream.contains("BASE 1074 Argument error (<=)"));
    assert!(upstream.contains("BASE 1075 Argument error (>)"));
    assert!(upstream.contains("BASE 1076 Argument error (>=)"));

    assert_eq!(runtime_array_comparison_baseline(), expected);
}
