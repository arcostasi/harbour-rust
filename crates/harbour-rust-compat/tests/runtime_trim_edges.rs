use std::fs;

mod support;
use support::{read_upstream_or_skip, workspace_fixture};

use harbour_rust_parser::parse;
use harbour_rust_runtime::{RuntimeError, Value, ltrim, rtrim, trim};

fn runtime_trim_edges_baseline() -> String {
    let mut out = String::new();
    out.push_str(&format!(
        "Trim(\" \\0 UA  \") => {}\n",
        escaped_result(trim(Some(&Value::from(String::from(" \0 UA  ")))))
    ));
    out.push_str(&format!(
        "Trim(\" \\t UA  \") => {}\n",
        escaped_result(trim(Some(&Value::from(String::from(" \t UA  ")))))
    ));
    out.push_str(&format!(
        "LTrim(\" \\0 UA  \") => {}\n",
        escaped_result(ltrim(Some(&Value::from(String::from(" \0 UA  ")))))
    ));
    out.push_str(&format!(
        "LTrim(\" \\tU\\t\") => {}\n",
        escaped_result(ltrim(Some(&Value::from(String::from(" \tU\t")))))
    ));
    out.push_str(&format!(
        "LTrim(\"\\nU\\n\") => {}\n",
        escaped_result(ltrim(Some(&Value::from(String::from("\nU\n")))))
    ));
    out.push_str(&format!(
        "RTrim(\"A\\n\") => {}\n",
        escaped_result(rtrim(Some(&Value::from(String::from("A\n")))))
    ));
    out.push_str(&format!(
        "Trim(\"  \\0ABC\\0  \") => {}\n",
        escaped_result(trim(Some(&Value::from(String::from("  \0ABC\0  ")))))
    ));
    out
}

fn escaped_result(result: Result<Value, RuntimeError>) -> String {
    match result {
        Ok(Value::String(text)) => format!("\"{}\"", escape_string(&text)),
        Ok(value) => value.to_output_string(),
        Err(error) => error.message,
    }
}

fn escape_string(text: &str) -> String {
    let mut escaped = String::new();
    for ch in text.chars() {
        match ch {
            '\0' => escaped.push_str("\\0"),
            '\t' => escaped.push_str("\\t"),
            '\n' => escaped.push_str("\\n"),
            '\r' => escaped.push_str("\\r"),
            _ => escaped.push(ch),
        }
    }
    escaped
}

#[test]
fn trim_edges_fixture_parses_without_errors() {
    let source = fs::read_to_string(workspace_fixture(
        "tests/fixtures/compat/trim_edges_runtime.prg",
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
fn trim_edges_runtime_matches_upstream_oracle_snapshot() {
    let Some(upstream) =
        read_upstream_or_skip("harbour-core/utils/hbtest/rt_str.prg", "upstream hbtest")
    else {
        return;
    };
    let expected = fs::read_to_string(workspace_fixture(
        "tests/fixtures/compat/trim_edges_runtime.out",
    ))
    .expect("fixture snapshot");

    assert!(upstream.contains("HBTEST Trim( \" \" + Chr( 0 ) + \" UA  \" )"));
    assert!(upstream.contains("HBTEST Trim( \" \" + Chr( 9 ) + \" UA  \" )"));
    assert!(upstream.contains("HBTEST LTrim( \" \" + Chr( 0 ) + \" UA  \" )"));
    assert!(upstream.contains("HBTEST LTrim( \" \" + Chr( 9 ) + \"U\" + Chr( 9 ) )"));
    assert!(upstream.contains("HBTEST LTrim( Chr( 10 ) + \"U\" + Chr( 10 ) )"));
    assert!(upstream.contains("HBTEST RTrim( \"A\" + Chr( 10 ) )"));
    assert!(upstream.contains(
        "HBTEST Trim( \"  \" + Chr( 0 ) + \"ABC\" + Chr( 0 ) + \"  \" )"
    ));

    assert_eq!(runtime_trim_edges_baseline(), expected);
}
