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
fn public_exact_comparison_distinguishes_array_identity_from_value_equality() {
    let array = Value::array(vec![Value::from(1_i64), Value::from(2_i64)]);
    let clone = array.clone();

    assert_eq!(array.exact_equals(&array), Ok(Value::from(true)));
    assert_eq!(array.exact_equals(&clone), Ok(Value::from(false)));
    assert_eq!(array.exact_not_equals(&clone), Ok(Value::from(true)));
    assert_eq!(
        Value::from(2_i64).exact_equals(&Value::from(2.0_f64)),
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

#[test]
fn public_array_index_diagnostics_cover_type_and_bounds_errors() {
    let values = Value::array(vec![Value::from(10_i64), Value::from(20_i64)]);

    assert_eq!(
        values.array_get(&Value::from("1")),
        Err(RuntimeError {
            message: "array index must be Integer".to_owned(),
            expected: Some(harbour_rust_runtime::ValueKind::Integer),
            actual: Some(harbour_rust_runtime::ValueKind::String),
        })
    );
    assert_eq!(
        values.array_get(&Value::from(0_i64)),
        Err(RuntimeError {
            message: "array index 0 out of bounds for length 2".to_owned(),
            expected: None,
            actual: None,
        })
    );
}

#[test]
fn public_array_set_diagnostics_cover_empty_paths_and_non_array_nested_targets() {
    let mut values = Value::array(vec![Value::from(10_i64), Value::from(20_i64)]);

    assert_eq!(
        values.array_set_path(&[], Value::from(1_i64)),
        Err(RuntimeError {
            message: "array assignment path must not be empty".to_owned(),
            expected: None,
            actual: None,
        })
    );
    assert_eq!(
        values.array_set_path(&[Value::from(1_i64), Value::from(1_i64)], Value::Nil),
        Err(RuntimeError {
            message: "convert value to array".to_owned(),
            expected: None,
            actual: Some(harbour_rust_runtime::ValueKind::Integer),
        })
    );
}
