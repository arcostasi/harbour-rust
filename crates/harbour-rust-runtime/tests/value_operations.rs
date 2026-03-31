use harbour_rust_runtime::{
    OutputBuffer, RuntimeContext, RuntimeError, Value, aadd, aclone, asize, call_builtin,
    call_builtin_mut, qout, substr,
};

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
fn public_array_comparison_operations_follow_xbase_baseline_errors() {
    let array = Value::array(vec![Value::from(1_i64)]);

    assert_eq!(
        array.equals(&array),
        Err(RuntimeError {
            message: "BASE 1071 Argument error (=)".to_owned(),
            expected: None,
            actual: None,
        })
    );
    assert_eq!(
        array.not_equals(&array),
        Err(RuntimeError {
            message: "BASE 1072 Argument error (<>)".to_owned(),
            expected: None,
            actual: None,
        })
    );
    assert_eq!(
        array.less_than(&array),
        Err(RuntimeError {
            message: "BASE 1073 Argument error (<)".to_owned(),
            expected: None,
            actual: None,
        })
    );
    assert_eq!(
        array.less_than_or_equal(&array),
        Err(RuntimeError {
            message: "BASE 1074 Argument error (<=)".to_owned(),
            expected: None,
            actual: None,
        })
    );
    assert_eq!(
        array.greater_than(&array),
        Err(RuntimeError {
            message: "BASE 1075 Argument error (>)".to_owned(),
            expected: None,
            actual: None,
        })
    );
    assert_eq!(
        array.greater_than_or_equal(&array),
        Err(RuntimeError {
            message: "BASE 1076 Argument error (>=)".to_owned(),
            expected: None,
            actual: None,
        })
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
fn public_substr_matches_the_current_ascii_runtime_baseline() {
    assert_eq!(
        substr(
            Some(&Value::from("abcdef")),
            Some(&Value::from(0_i64)),
            Some(&Value::from(1_i64)),
        ),
        Ok(Value::from("a"))
    );
    assert_eq!(
        substr(
            Some(&Value::from("abcdef")),
            Some(&Value::from(2_i64)),
            Some(&Value::from(7_i64)),
        ),
        Ok(Value::from("bcdef"))
    );
    assert_eq!(
        substr(
            Some(&Value::from("abcdef")),
            Some(&Value::from(-2_i64)),
            None,
        ),
        Ok(Value::from("ef"))
    );
    assert_eq!(
        substr(
            Some(&Value::from("abcdef")),
            Some(&Value::from(10_i64)),
            None,
        ),
        Ok(Value::from(""))
    );
    assert_eq!(
        substr(
            Some(&Value::from("abcdef")),
            Some(&Value::from(2_i64)),
            Some(&Value::from(-1_i64)),
        ),
        Ok(Value::from(""))
    );
}

#[test]
fn public_substr_reports_xbase_style_argument_errors() {
    assert_eq!(
        substr(
            Some(&Value::from(100_i64)),
            Some(&Value::from(0_i64)),
            Some(&Value::from(-1_i64)),
        ),
        Err(RuntimeError {
            message: "BASE 1110 Argument error (SUBSTR)".to_owned(),
            expected: None,
            actual: Some(harbour_rust_runtime::ValueKind::Integer),
        })
    );
    assert_eq!(
        substr(Some(&Value::from("abcdef")), None, None),
        Err(RuntimeError {
            message: "BASE 1110 Argument error (SUBSTR)".to_owned(),
            expected: None,
            actual: None,
        })
    );
    assert_eq!(
        substr(
            Some(&Value::from("abcdef")),
            Some(&Value::from("a")),
            Some(&Value::from(1_i64)),
        ),
        Err(RuntimeError {
            message: "BASE 1110 Argument error (SUBSTR)".to_owned(),
            expected: None,
            actual: Some(harbour_rust_runtime::ValueKind::String),
        })
    );
}

#[test]
fn public_substr_dispatches_through_the_immutable_builtin_surface() {
    let mut context = RuntimeContext::new();

    assert_eq!(
        call_builtin(
            "SUBSTR",
            &[
                Value::from("abcdef"),
                Value::from(2_i64),
                Value::from(3_i64),
            ],
            &mut context,
        ),
        Ok(Value::from("bcd"))
    );

    let mut mutable_arguments = [
        Value::from("abcdef"),
        Value::from(-2_i64),
        Value::from(7_i64),
    ];
    assert_eq!(
        call_builtin_mut("substr", &mut mutable_arguments, &mut context),
        Ok(Value::from("ef"))
    );
    assert_eq!(mutable_arguments[0], Value::from("abcdef"));
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
fn public_array_builtins_mutate_the_first_argument_through_mutable_dispatch() {
    let mut context = RuntimeContext::new();
    let mut add_arguments = [Value::empty_array(), Value::from("tail")];

    assert_eq!(
        call_builtin_mut("AADD", &mut add_arguments, &mut context),
        Ok(Value::from("tail"))
    );
    assert_eq!(add_arguments[0], Value::array(vec![Value::from("tail")]));

    let mut size_arguments = [add_arguments[0].clone(), Value::from(3_i64)];
    assert_eq!(
        call_builtin_mut("ASIZE", &mut size_arguments, &mut context),
        Ok(Value::array(vec![
            Value::from("tail"),
            Value::Nil,
            Value::Nil,
        ]))
    );
    assert_eq!(
        size_arguments[0],
        Value::array(vec![Value::from("tail"), Value::Nil, Value::Nil])
    );
}

#[test]
fn public_aclone_follows_the_current_lenient_runtime_baseline() {
    let values = Value::array(vec![
        Value::from(1_i64),
        Value::array(vec![Value::from("nested")]),
    ]);

    assert_eq!(aclone(None), Ok(Value::Nil));
    assert_eq!(aclone(Some(&Value::Nil)), Ok(Value::Nil));
    assert_eq!(aclone(Some(&Value::from("nope"))), Ok(Value::Nil));
    assert_eq!(aclone(Some(&values)), Ok(values.clone()));
}

#[test]
fn public_aadd_and_asize_follow_the_current_lenient_runtime_baseline() {
    let mut values = Value::empty_array();

    assert_eq!(aadd(&mut values, Value::Nil), Ok(Value::Nil));
    assert_eq!(values, Value::array(vec![Value::Nil]));

    assert_eq!(
        asize(&mut values, Some(&Value::from(-1_i64))),
        Ok(Value::empty_array())
    );
    assert_eq!(values, Value::empty_array());

    let mut not_array = Value::from("nope");
    assert_eq!(aadd(&mut not_array, Value::from(1_i64)), Ok(Value::Nil));
    assert_eq!(
        asize(&mut not_array, Some(&Value::from(3_i64))),
        Ok(Value::Nil)
    );
}

#[test]
fn public_mutating_array_builtins_report_when_called_through_immutable_dispatch() {
    let mut context = RuntimeContext::new();

    assert_eq!(
        call_builtin(
            "AADD",
            &[Value::empty_array(), Value::from(1_i64)],
            &mut context
        ),
        Err(RuntimeError {
            message: "builtin AADD requires mutable dispatch".to_owned(),
            expected: None,
            actual: None,
        })
    );
    assert_eq!(
        call_builtin(
            "ASIZE",
            &[Value::empty_array(), Value::from(2_i64)],
            &mut context
        ),
        Err(RuntimeError {
            message: "builtin ASIZE requires mutable dispatch".to_owned(),
            expected: None,
            actual: None,
        })
    );
}

#[test]
fn public_aclone_dispatches_through_the_immutable_builtin_surface() {
    let mut context = RuntimeContext::new();
    let values = Value::array(vec![
        Value::from(1_i64),
        Value::array(vec![Value::from("nested")]),
    ]);

    assert_eq!(
        call_builtin("ACLONE", std::slice::from_ref(&values), &mut context),
        Ok(values.clone())
    );
    assert_eq!(call_builtin("ACLONE", &[], &mut context), Ok(Value::Nil));

    let mut mutable_arguments = [values.clone()];
    assert_eq!(
        call_builtin_mut("ACLONE", &mut mutable_arguments, &mut context),
        Ok(values.clone())
    );
    assert_eq!(mutable_arguments[0], values);
}

#[test]
fn public_array_index_diagnostics_cover_type_and_bounds_errors() {
    let values = Value::array(vec![Value::from(10_i64), Value::from(20_i64)]);

    assert_eq!(
        values.array_get(&Value::from("1")),
        Err(RuntimeError {
            message: "BASE 1068 Argument error (array access)".to_owned(),
            expected: Some(harbour_rust_runtime::ValueKind::Integer),
            actual: Some(harbour_rust_runtime::ValueKind::String),
        })
    );
    assert_eq!(
        values.array_get(&Value::from(0_i64)),
        Err(RuntimeError {
            message: "BASE 1132 Bound error (array access)".to_owned(),
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
        values.array_set(&Value::from("1"), Value::Nil),
        Err(RuntimeError {
            message: "BASE 1069 Argument error (array assign)".to_owned(),
            expected: Some(harbour_rust_runtime::ValueKind::Integer),
            actual: Some(harbour_rust_runtime::ValueKind::String),
        })
    );
    assert_eq!(
        values.array_set(&Value::from(3_i64), Value::Nil),
        Err(RuntimeError {
            message: "BASE 1133 Bound error (array assign)".to_owned(),
            expected: None,
            actual: None,
        })
    );
    assert_eq!(
        values.array_set_path(&[Value::from(1_i64), Value::from(1_i64)], Value::Nil),
        Err(RuntimeError {
            message: "BASE 1069 Argument error (array assign)".to_owned(),
            expected: Some(harbour_rust_runtime::ValueKind::Integer),
            actual: Some(harbour_rust_runtime::ValueKind::Integer),
        })
    );
}
