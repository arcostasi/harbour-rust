use harbour_rust_runtime::{OutputBuffer, RuntimeContext, RuntimeError, Value, call_builtin, qout};

#[test]
fn public_arithmetic_operations_cover_core_runtime_baseline() {
    assert_eq!(
        Value::from(5_i64).add(&Value::from(4_i64)),
        Ok(Value::from(9_i64))
    );
    assert_eq!(
        Value::from(5_i64).subtract(&Value::from(1.5_f64)),
        Ok(Value::from(3.5_f64))
    );
    assert_eq!(
        Value::from(3_i64).multiply(&Value::from(2_i64)),
        Ok(Value::from(6_i64))
    );
    assert_eq!(
        Value::from(7_i64).divide(&Value::from(2_i64)),
        Ok(Value::from(3.5_f64))
    );
    assert_eq!(
        Value::from("har").add(&Value::from("bour")),
        Ok(Value::from("harbour"))
    );
}

#[test]
fn public_comparison_operations_cover_numeric_and_string_values() {
    assert_eq!(
        Value::from(2_i64).equals(&Value::from(2.0_f64)),
        Ok(Value::from(true))
    );
    assert_eq!(
        Value::from(2_i64).not_equals(&Value::from(3_i64)),
        Ok(Value::from(true))
    );
    assert_eq!(
        Value::from(2_i64).less_than_or_equal(&Value::from(3_i64)),
        Ok(Value::from(true))
    );
    assert_eq!(
        Value::from("abc").greater_than(&Value::from("abb")),
        Ok(Value::from(true))
    );
}

#[test]
fn public_qout_builtin_writes_expected_output_and_returns_nil() {
    let mut output = OutputBuffer::new();

    assert_eq!(
        qout(
            &[Value::from("sum"), Value::from(3_i64), Value::from(4.5_f64)],
            &mut output,
        ),
        Ok(Value::Nil)
    );
    assert_eq!(output.into_string(), "sum 3 4.5\n");
}

#[test]
fn public_builtin_dispatch_routes_print_calls_through_runtime_context() {
    let mut context = RuntimeContext::new();

    assert_eq!(
        call_builtin(
            "QOUT",
            &[Value::from("sum"), Value::from(3_i64), Value::from(4.5_f64)],
            &mut context,
        ),
        Ok(Value::Nil)
    );
    assert_eq!(context.into_output().into_string(), "sum 3 4.5\n");
}

#[test]
fn public_builtin_dispatch_reports_unknown_builtin() {
    let mut context = RuntimeContext::new();

    assert_eq!(
        call_builtin("Nope", &[], &mut context),
        Err(RuntimeError {
            message: "unknown builtin Nope".to_owned(),
            expected: None,
            actual: None,
        })
    );
}
