use std::fs;

mod support;
use support::{read_upstream_or_skip, workspace_fixture};

use harbour_rust_parser::parse;
use harbour_rust_runtime::{RuntimeContext, RuntimeError, Value, call_builtin, mod_value};

fn runtime_mod_edges_baseline() -> String {
    let mut out = String::new();
    out.push_str(&format!(
        "Mod(\"A\", 100) => {}\n",
        result_text(mod_value(
            Some(&Value::from("A")),
            Some(&Value::from(100_i64))
        ))
    ));
    out.push_str(&format!(
        "Mod(100, \"B\") => {}\n",
        result_text(mod_value(
            Some(&Value::from(100_i64)),
            Some(&Value::from("B"))
        ))
    ));
    out.push_str(&format!(
        "Mod(1, NIL) => {}\n",
        result_text(mod_value(Some(&Value::from(1_i64)), Some(&Value::Nil)))
    ));
    out.push_str(&format!(
        "Mod(-1, -3) => {}\n",
        result_text(mod_value(
            Some(&Value::from(-1_i64)),
            Some(&Value::from(-3_i64)),
        ))
    ));
    out.push_str(&format!(
        "Mod(100, 60, \"A\") => {}\n",
        builtin_result_text(&[Value::from(100_i64), Value::from(60_i64), Value::from("A")])
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
fn mod_edges_fixture_parses_without_errors() {
    let source = fs::read_to_string(workspace_fixture(
        "tests/fixtures/compat/mod_edges_runtime.prg",
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
fn mod_edges_runtime_matches_upstream_oracle_snapshot() {
    let Some(upstream_math) =
        read_upstream_or_skip("harbour-core/utils/hbtest/rt_math.prg", "upstream rt_math")
    else {
        return;
    };
    let expected = fs::read_to_string(workspace_fixture(
        "tests/fixtures/compat/mod_edges_runtime.out",
    ))
    .expect("fixture snapshot");

    assert!(upstream_math.contains("HBTEST Mod( \"A\", 100 )"));
    assert!(upstream_math.contains("HBTEST Mod( 100, \"B\" )"));
    assert!(upstream_math.contains("HBTEST Mod( 1, NIL )"));
    assert!(upstream_math.contains("HBTEST Str( Mod( -1, -3 ) )"));
    assert!(upstream_math.contains("HBTEST Mod( 100, 60, \"A\" )"));

    assert_eq!(runtime_mod_edges_baseline(), expected);
}
