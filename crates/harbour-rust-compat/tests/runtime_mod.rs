use std::{fs, path::PathBuf};

use harbour_rust_parser::parse;
use harbour_rust_runtime::{RuntimeContext, RuntimeError, Value, call_builtin, mod_value};

fn workspace_fixture(path: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join(path)
}

fn runtime_mod_baseline() -> String {
    let mut out = String::new();
    out.push_str(&format!(
        "Mod(NIL, NIL) => {}\n",
        result_text(mod_value(Some(&Value::Nil), Some(&Value::Nil)))
    ));
    out.push_str(&format!(
        "Mod(100, 60, \"A\") => {}\n",
        builtin_result_text(&[Value::from(100_i64), Value::from(60_i64), Value::from("A"),])
    ));
    out.push_str(&format!(
        "Mod(1, 0) => {}\n",
        result_text(mod_value(
            Some(&Value::from(1_i64)),
            Some(&Value::from(0_i64))
        ))
    ));
    out.push_str(&format!(
        "Mod(2, 4) => {}\n",
        result_text(mod_value(
            Some(&Value::from(2_i64)),
            Some(&Value::from(4_i64))
        ))
    ));
    out.push_str(&format!(
        "Mod(-1, 3) => {}\n",
        result_text(mod_value(
            Some(&Value::from(-1_i64)),
            Some(&Value::from(3_i64))
        ))
    ));
    out.push_str(&format!(
        "Mod(1, -3) => {}\n",
        result_text(mod_value(
            Some(&Value::from(1_i64)),
            Some(&Value::from(-3_i64))
        ))
    ));
    out
}

fn builtin_result_text(arguments: &[Value]) -> String {
    let mut context = RuntimeContext::new();
    result_text(call_builtin("MOD", arguments, &mut context))
}

fn result_text(result: Result<Value, RuntimeError>) -> String {
    match result {
        Ok(value) => value.to_output_string(),
        Err(error) => error.message,
    }
}

#[test]
fn mod_fixture_parses_without_errors() {
    let source = fs::read_to_string(workspace_fixture("tests/fixtures/compat/mod_runtime.prg"))
        .expect("fixture source");
    let parsed = parse(&source);

    assert!(
        parsed.errors.is_empty(),
        "expected parse success, got {:?}",
        parsed.errors
    );
}

#[test]
fn mod_runtime_matches_upstream_oracle_snapshot() {
    let upstream_math =
        fs::read_to_string(workspace_fixture("harbour-core/utils/hbtest/rt_math.prg"))
            .expect("upstream rt_math");
    let expected = fs::read_to_string(workspace_fixture("tests/fixtures/compat/mod_runtime.out"))
        .expect("fixture snapshot");

    assert!(upstream_math.contains("HBTEST Mod()"));
    assert!(upstream_math.contains("HBTEST Mod( 100, 60, \"A\" )"));
    assert!(upstream_math.contains("HBTEST Str( Mod( -1,  3 ) )"));
    assert!(upstream_math.contains("BASE 1085 Argument error (%)"));
    assert!(upstream_math.contains("BASE 1341 Zero divisor (%)"));

    assert_eq!(runtime_mod_baseline(), expected);
}
