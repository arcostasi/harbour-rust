use std::{fs, path::PathBuf};

use harbour_rust_parser::parse;
use harbour_rust_runtime::{RuntimeError, Value, val};

fn workspace_fixture(path: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join(path)
}

fn runtime_val_baseline() -> String {
    let mut out = String::new();
    out.push_str(&format!(
        "Val(\"\") => {}\n",
        result_text(val(Some(&Value::from(""))))
    ));
    out.push_str(&format!(
        "Val(\"A\") => {}\n",
        result_text(val(Some(&Value::from("A"))))
    ));
    out.push_str(&format!(
        "Val(\"10\") => {}\n",
        result_text(val(Some(&Value::from("10"))))
    ));
    out.push_str(&format!(
        "Val(\"  -12\") => {}\n",
        result_text(val(Some(&Value::from("  -12"))))
    ));
    out.push_str(&format!(
        "Val(\"15.001 \") => {}\n",
        result_text(val(Some(&Value::from("15.001 "))))
    ));
    out.push_str(&format!(
        "Val(\"1HELLO.\") => {}\n",
        result_text(val(Some(&Value::from("1HELLO."))))
    ));
    out.push_str(&format!(
        "Val(\"0x10\") => {}\n",
        result_text(val(Some(&Value::from("0x10"))))
    ));
    out.push_str(&format!(
        "Val(NIL) => {}\n",
        result_text(val(Some(&Value::Nil)))
    ));
    out.push_str(&format!(
        "Val(10) => {}\n",
        result_text(val(Some(&Value::from(10_i64))))
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
fn val_fixture_parses_without_errors() {
    let source = fs::read_to_string(workspace_fixture("tests/fixtures/compat/val_runtime.prg"))
        .expect("fixture source");
    let parsed = parse(&source);

    assert!(
        parsed.errors.is_empty(),
        "expected parse success, got {:?}",
        parsed.errors
    );
}

#[test]
fn val_runtime_matches_upstream_oracle_snapshot() {
    let upstream_str =
        fs::read_to_string(workspace_fixture("harbour-core/utils/hbtest/rt_str.prg"))
            .expect("upstream rt_str");
    let upstream_math =
        fs::read_to_string(workspace_fixture("harbour-core/utils/hbtest/rt_math.prg"))
            .expect("upstream rt_math");
    let upstream_val =
        fs::read_to_string(workspace_fixture("harbour-core/src/rtl/val.c")).expect("upstream val");
    let expected = fs::read_to_string(workspace_fixture("tests/fixtures/compat/val_runtime.out"))
        .expect("fixture snapshot");

    assert!(upstream_str.contains("HBTEST Val( NIL )"));
    assert!(upstream_str.contains("HBTEST Str( Val( \"  -12\" ) )"));
    assert!(upstream_str.contains("HBTEST Str( Val( \"1HELLO.\" ) )"));
    assert!(upstream_math.contains("HBTEST Str( Val( \"0x10\" )"));
    assert!(upstream_val.contains("hb_val( HB_FALSE )"));

    assert_eq!(runtime_val_baseline(), expected);
}
