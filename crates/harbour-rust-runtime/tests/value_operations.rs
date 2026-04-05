use harbour_rust_runtime::{
    OutputBuffer, RuntimeContext, RuntimeError, Value, aadd, abs, aclone, adel, ains, ascan, asize,
    at, call_builtin, call_builtin_mut, cos_value, empty, exp_value, int, left, log_value, lower,
    ltrim, max_value, min_value, mod_value, qout, replicate, right, round_value, rtrim, sin_value,
    space, sqrt_value, str_value, substr, tan_value, trim, type_value, upper, val, valtype,
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
fn public_string_comparison_matches_exact_off_prefix_baseline() {
    assert_eq!(
        Value::from("12345").equals(&Value::from("123")),
        Ok(Value::from(true))
    );
    assert_eq!(
        Value::from("123").equals(&Value::from("12345")),
        Ok(Value::from(false))
    );
    assert_eq!(
        Value::from("123").equals(&Value::from("")),
        Ok(Value::from(true))
    );
    assert_eq!(
        Value::from("").equals(&Value::from("123")),
        Ok(Value::from(false))
    );
    assert_eq!(
        Value::from("AA").exact_equals(&Value::from("A")),
        Ok(Value::from(false))
    );
    assert_eq!(
        Value::from("AA").not_equals(&Value::from("A")),
        Ok(Value::from(false))
    );
    assert_eq!(
        Value::from("Z").not_equals(&Value::from("A")),
        Ok(Value::from(true))
    );
}

#[test]
fn public_abs_matches_the_current_numeric_runtime_baseline() {
    assert_eq!(abs(Some(&Value::from(0_i64))), Ok(Value::from(0_i64)));
    assert_eq!(abs(Some(&Value::from(10_i64))), Ok(Value::from(10_i64)));
    assert_eq!(abs(Some(&Value::from(-10_i64))), Ok(Value::from(10_i64)));
    assert_eq!(abs(Some(&Value::from(0.1_f64))), Ok(Value::from(0.1_f64)));
    assert_eq!(
        abs(Some(&Value::from(-10.7_f64))),
        Ok(Value::from(10.7_f64))
    );
}

#[test]
fn public_abs_reports_xbase_style_argument_errors() {
    assert_eq!(
        abs(Some(&Value::from("A"))),
        Err(RuntimeError {
            message: "BASE 1089 Argument error (ABS)".to_owned(),
            expected: None,
            actual: Some(harbour_rust_runtime::ValueKind::String),
        })
    );
    assert_eq!(
        abs(None),
        Err(RuntimeError {
            message: "BASE 1089 Argument error (ABS)".to_owned(),
            expected: None,
            actual: None,
        })
    );
}

#[test]
fn public_abs_dispatches_through_the_immutable_builtin_surface() {
    let mut context = RuntimeContext::new();

    assert_eq!(
        call_builtin("ABS", &[Value::from(-10_i64)], &mut context),
        Ok(Value::from(10_i64))
    );

    let mut mutable_arguments = [Value::from(-150.245_f64)];
    assert_eq!(
        call_builtin_mut("abs", &mut mutable_arguments, &mut context),
        Ok(Value::from(150.245_f64))
    );
    assert_eq!(mutable_arguments[0], Value::from(-150.245_f64));
}

#[test]
fn public_sqrt_matches_the_current_numeric_runtime_baseline() {
    assert_eq!(
        sqrt_value(Some(&Value::from(-1_i64))),
        Ok(Value::from(0.0_f64))
    );
    assert_eq!(
        sqrt_value(Some(&Value::from(0_i64))),
        Ok(Value::from(0.0_f64))
    );
    assert_eq!(
        sqrt_value(Some(&Value::from(4_i64))),
        Ok(Value::from(2.0_f64))
    );
    assert_eq!(
        sqrt_value(Some(&Value::from(10_i64))),
        Ok(Value::from(10_f64.sqrt()))
    );
    assert_eq!(
        sqrt_value(Some(&Value::from(3.0_f64))),
        Ok(Value::from(3.0_f64.sqrt()))
    );
}

#[test]
fn public_sqrt_reports_xbase_style_argument_errors() {
    assert_eq!(
        sqrt_value(Some(&Value::from("A"))),
        Err(RuntimeError {
            message: "BASE 1097 Argument error (SQRT)".to_owned(),
            expected: None,
            actual: Some(harbour_rust_runtime::ValueKind::String),
        })
    );
    assert_eq!(
        sqrt_value(None),
        Err(RuntimeError {
            message: "BASE 1097 Argument error (SQRT)".to_owned(),
            expected: None,
            actual: None,
        })
    );
}

#[test]
fn public_sqrt_dispatches_through_the_immutable_builtin_surface() {
    let mut context = RuntimeContext::new();

    assert_eq!(
        call_builtin("SQRT", &[Value::from(4_i64)], &mut context),
        Ok(Value::from(2.0_f64))
    );

    let mut mutable_arguments = [Value::from(10_i64)];
    assert_eq!(
        call_builtin_mut("sqrt", &mut mutable_arguments, &mut context),
        Ok(Value::from(10_f64.sqrt()))
    );
    assert_eq!(mutable_arguments[0], Value::from(10_i64));
}

#[test]
fn public_sin_and_cos_match_the_current_numeric_runtime_baseline() {
    assert_eq!(
        sin_value(Some(&Value::from(0_i64))),
        Ok(Value::from(0.0_f64))
    );
    assert_eq!(
        cos_value(Some(&Value::from(0_i64))),
        Ok(Value::from(1.0_f64))
    );
    assert_eq!(
        round_value(
            sin_value(Some(&Value::from(1_i64))).ok().as_ref(),
            Some(&Value::from(2_i64))
        ),
        Ok(Value::from(0.84_f64))
    );
    assert_eq!(
        round_value(
            cos_value(Some(&Value::from(1_i64))).ok().as_ref(),
            Some(&Value::from(2_i64))
        ),
        Ok(Value::from(0.54_f64))
    );
}

#[test]
fn public_sin_and_cos_report_xbase_style_argument_errors() {
    assert_eq!(
        sin_value(Some(&Value::from("A"))),
        Err(RuntimeError {
            message: "BASE 1091 Argument error (SIN)".to_owned(),
            expected: None,
            actual: Some(harbour_rust_runtime::ValueKind::String),
        })
    );
    assert_eq!(
        cos_value(None),
        Err(RuntimeError {
            message: "BASE 1091 Argument error (COS)".to_owned(),
            expected: None,
            actual: None,
        })
    );
}

#[test]
fn public_sin_and_cos_dispatch_through_the_immutable_builtin_surface() {
    let mut context = RuntimeContext::new();

    assert_eq!(
        call_builtin("SIN", &[Value::from(0_i64)], &mut context),
        Ok(Value::from(0.0_f64))
    );
    assert_eq!(
        call_builtin("COS", &[Value::from(0_i64)], &mut context),
        Ok(Value::from(1.0_f64))
    );

    let mut mutable_arguments = [Value::from(1_i64)];
    assert_eq!(
        call_builtin_mut("cos", &mut mutable_arguments, &mut context),
        Ok(Value::from(1_f64.cos()))
    );
    assert_eq!(mutable_arguments[0], Value::from(1_i64));
}

#[test]
fn public_tan_matches_the_current_numeric_runtime_baseline() {
    assert_eq!(
        tan_value(Some(&Value::from(0_i64))),
        Ok(Value::from(0.0_f64))
    );
    assert_eq!(
        round_value(
            tan_value(Some(&Value::from(1_i64))).ok().as_ref(),
            Some(&Value::from(4_i64))
        ),
        Ok(Value::from(1.5574_f64))
    );
}

#[test]
fn public_tan_reports_xbase_style_argument_errors() {
    assert_eq!(
        tan_value(Some(&Value::from("A"))),
        Err(RuntimeError {
            message: "BASE 1091 Argument error (TAN)".to_owned(),
            expected: None,
            actual: Some(harbour_rust_runtime::ValueKind::String),
        })
    );
    assert_eq!(
        tan_value(None),
        Err(RuntimeError {
            message: "BASE 1091 Argument error (TAN)".to_owned(),
            expected: None,
            actual: None,
        })
    );
}

#[test]
fn public_tan_dispatches_through_the_immutable_builtin_surface() {
    let mut context = RuntimeContext::new();

    assert_eq!(
        call_builtin("TAN", &[Value::from(0_i64)], &mut context),
        Ok(Value::from(0.0_f64))
    );

    let mut mutable_arguments = [Value::from(1_i64)];
    assert_eq!(
        call_builtin_mut("tan", &mut mutable_arguments, &mut context),
        Ok(Value::from(1_f64.tan()))
    );
    assert_eq!(mutable_arguments[0], Value::from(1_i64));
}

#[test]
fn public_exp_matches_the_current_numeric_runtime_baseline() {
    assert_eq!(
        exp_value(Some(&Value::from(0_i64))),
        Ok(Value::from(1.0_f64))
    );
    assert_eq!(
        exp_value(Some(&Value::from(1_i64))),
        Ok(Value::from(1_f64.exp()))
    );
    assert_eq!(
        exp_value(Some(&Value::from(15_i64))),
        Ok(Value::from(15_f64.exp()))
    );
    assert_eq!(
        round_value(
            exp_value(Some(&Value::from(1_i64))).ok().as_ref(),
            Some(&Value::from(2_i64))
        ),
        Ok(Value::from(2.72_f64))
    );
    assert_eq!(
        str_value(
            exp_value(Some(&Value::from(1_i64))).ok().as_ref(),
            Some(&Value::from(20_i64)),
            Some(&Value::from(10_i64))
        ),
        Ok(Value::from("        2.7182818285"))
    );
}

#[test]
fn public_exp_reports_xbase_style_argument_errors() {
    assert_eq!(
        exp_value(Some(&Value::from("A"))),
        Err(RuntimeError {
            message: "BASE 1096 Argument error (EXP)".to_owned(),
            expected: None,
            actual: Some(harbour_rust_runtime::ValueKind::String),
        })
    );
    assert_eq!(
        exp_value(None),
        Err(RuntimeError {
            message: "BASE 1096 Argument error (EXP)".to_owned(),
            expected: None,
            actual: None,
        })
    );
}

#[test]
fn public_exp_dispatches_through_the_immutable_builtin_surface() {
    let mut context = RuntimeContext::new();

    assert_eq!(
        call_builtin("EXP", &[Value::from(0_i64)], &mut context),
        Ok(Value::from(1.0_f64))
    );

    let mut mutable_arguments = [Value::from(1_i64)];
    assert_eq!(
        call_builtin_mut("exp", &mut mutable_arguments, &mut context),
        Ok(Value::from(1_f64.exp()))
    );
    assert_eq!(mutable_arguments[0], Value::from(1_i64));
}

#[test]
fn public_log_matches_the_current_numeric_runtime_baseline() {
    assert_eq!(
        log_value(Some(&Value::from(-1_i64))),
        Ok(Value::from(f64::NEG_INFINITY))
    );
    assert_eq!(
        log_value(Some(&Value::from(1_i64))),
        Ok(Value::from(0.0_f64))
    );
    assert_eq!(
        log_value(Some(&Value::from(12_i64))),
        Ok(Value::from(12_f64.ln()))
    );
    assert_eq!(
        str_value(
            log_value(Some(&Value::from(-1_i64))).ok().as_ref(),
            None,
            None
        ),
        Ok(Value::from("***********************"))
    );
    assert_eq!(
        str_value(
            log_value(Some(&Value::from(10_i64))).ok().as_ref(),
            Some(&Value::from(10_i64)),
            Some(&Value::from(2_i64))
        ),
        Ok(Value::from("      2.30"))
    );
}

#[test]
fn public_log_reports_xbase_style_argument_errors() {
    assert_eq!(
        log_value(Some(&Value::from("A"))),
        Err(RuntimeError {
            message: "BASE 1095 Argument error (LOG)".to_owned(),
            expected: None,
            actual: Some(harbour_rust_runtime::ValueKind::String),
        })
    );
    assert_eq!(
        log_value(None),
        Err(RuntimeError {
            message: "BASE 1095 Argument error (LOG)".to_owned(),
            expected: None,
            actual: None,
        })
    );
}

#[test]
fn public_log_dispatches_through_the_immutable_builtin_surface() {
    let mut context = RuntimeContext::new();

    assert_eq!(
        call_builtin("LOG", &[Value::from(1_i64)], &mut context),
        Ok(Value::from(0.0_f64))
    );

    let mut mutable_arguments = [Value::from(10_i64)];
    assert_eq!(
        call_builtin_mut("log", &mut mutable_arguments, &mut context),
        Ok(Value::from(10_f64.ln()))
    );
    assert_eq!(mutable_arguments[0], Value::from(10_i64));
}

#[test]
fn public_int_matches_the_current_numeric_runtime_baseline() {
    assert_eq!(int(Some(&Value::from(0_i64))), Ok(Value::from(0_i64)));
    assert_eq!(int(Some(&Value::from(10_i64))), Ok(Value::from(10_i64)));
    assert_eq!(int(Some(&Value::from(-10_i64))), Ok(Value::from(-10_i64)));
    assert_eq!(int(Some(&Value::from(10.5_f64))), Ok(Value::from(10_i64)));
    assert_eq!(int(Some(&Value::from(-10.5_f64))), Ok(Value::from(-10_i64)));
    assert_eq!(
        int(Some(&Value::from(5_000_000_000.9_f64))),
        Ok(Value::from(5_000_000_000_i64))
    );
    assert_eq!(
        int(Some(&Value::from(-5_000_000_000.9_f64))),
        Ok(Value::from(-5_000_000_000_i64))
    );
}

#[test]
fn public_int_reports_xbase_style_argument_errors() {
    assert_eq!(
        int(Some(&Value::from("A"))),
        Err(RuntimeError {
            message: "BASE 1090 Argument error (INT)".to_owned(),
            expected: None,
            actual: Some(harbour_rust_runtime::ValueKind::String),
        })
    );
    assert_eq!(
        int(None),
        Err(RuntimeError {
            message: "BASE 1090 Argument error (INT)".to_owned(),
            expected: None,
            actual: None,
        })
    );
}

#[test]
fn public_int_dispatches_through_the_immutable_builtin_surface() {
    let mut context = RuntimeContext::new();

    assert_eq!(
        call_builtin("INT", &[Value::from(10.7_f64)], &mut context),
        Ok(Value::from(10_i64))
    );

    let mut mutable_arguments = [Value::from(-150.245_f64)];
    assert_eq!(
        call_builtin_mut("int", &mut mutable_arguments, &mut context),
        Ok(Value::from(-150_i64))
    );
    assert_eq!(mutable_arguments[0], Value::from(-150.245_f64));
}

#[test]
fn public_round_matches_the_current_numeric_runtime_baseline() {
    assert_eq!(
        round_value(Some(&Value::from(0.5_f64)), Some(&Value::from(0_i64))),
        Ok(Value::from(1_i64))
    );
    assert_eq!(
        round_value(Some(&Value::from(0.55_f64)), Some(&Value::from(1_i64))),
        Ok(Value::from(0.6_f64))
    );
    assert_eq!(
        round_value(Some(&Value::from(0.557_f64)), Some(&Value::from(2_i64))),
        Ok(Value::from(0.56_f64))
    );
    assert_eq!(
        round_value(Some(&Value::from(50_i64)), Some(&Value::from(-2_i64))),
        Ok(Value::from(100_i64))
    );
    assert_eq!(
        round_value(Some(&Value::from(-0.55_f64)), Some(&Value::from(1_i64))),
        Ok(Value::from(-0.6_f64))
    );
}

#[test]
fn public_round_reports_xbase_style_argument_errors() {
    assert_eq!(
        round_value(Some(&Value::Nil), Some(&Value::from(0_i64))),
        Err(RuntimeError {
            message: "BASE 1094 Argument error (ROUND)".to_owned(),
            expected: None,
            actual: Some(harbour_rust_runtime::ValueKind::Nil),
        })
    );
    assert_eq!(
        round_value(Some(&Value::from(0_i64)), Some(&Value::Nil)),
        Err(RuntimeError {
            message: "BASE 1094 Argument error (ROUND)".to_owned(),
            expected: None,
            actual: Some(harbour_rust_runtime::ValueKind::Nil),
        })
    );
}

#[test]
fn public_round_dispatches_through_the_immutable_builtin_surface() {
    let mut context = RuntimeContext::new();

    assert_eq!(
        call_builtin(
            "ROUND",
            &[Value::from(0.557_f64), Value::from(2_i64)],
            &mut context
        ),
        Ok(Value::from(0.56_f64))
    );

    let mut mutable_arguments = [Value::from(-0.55_f64), Value::from(1_i64)];
    assert_eq!(
        call_builtin_mut("round", &mut mutable_arguments, &mut context),
        Ok(Value::from(-0.6_f64))
    );
    assert_eq!(mutable_arguments[0], Value::from(-0.55_f64));
}

#[test]
fn public_mod_matches_the_current_numeric_runtime_baseline() {
    assert_eq!(
        mod_value(Some(&Value::from(100_i64)), Some(&Value::from(60_i64))),
        Ok(Value::from(40.0_f64))
    );
    assert_eq!(
        mod_value(Some(&Value::from(2_i64)), Some(&Value::from(4_i64))),
        Ok(Value::from(2.0_f64))
    );
    assert_eq!(
        mod_value(Some(&Value::from(-1_i64)), Some(&Value::from(3_i64))),
        Ok(Value::from(2.0_f64))
    );
    assert_eq!(
        mod_value(Some(&Value::from(1_i64)), Some(&Value::from(-3_i64))),
        Ok(Value::from(-2.0_f64))
    );
}

#[test]
fn public_mod_reports_xbase_style_argument_and_zero_divisor_errors() {
    assert_eq!(
        mod_value(Some(&Value::Nil), Some(&Value::Nil)),
        Err(RuntimeError {
            message: "BASE 1085 Argument error (%)".to_owned(),
            expected: None,
            actual: Some(harbour_rust_runtime::ValueKind::Nil),
        })
    );
    assert_eq!(
        mod_value(Some(&Value::from(1_i64)), Some(&Value::from(0_i64))),
        Err(RuntimeError {
            message: "BASE 1341 Zero divisor (%)".to_owned(),
            expected: None,
            actual: None,
        })
    );
}

#[test]
fn public_mod_dispatches_through_the_immutable_builtin_surface() {
    let mut context = RuntimeContext::new();

    assert_eq!(
        call_builtin(
            "MOD",
            &[
                Value::from(100_i64),
                Value::from(60_i64),
                Value::from("ignored"),
            ],
            &mut context
        ),
        Ok(Value::from(40.0_f64))
    );

    let mut mutable_arguments = [Value::from(-2_i64), Value::from(-3_i64)];
    assert_eq!(
        call_builtin_mut("mod", &mut mutable_arguments, &mut context),
        Ok(Value::from(-2.0_f64))
    );
    assert_eq!(mutable_arguments[0], Value::from(-2_i64));
}

#[test]
fn public_max_and_min_match_the_current_runtime_baseline() {
    assert_eq!(
        max_value(Some(&Value::from(10_i64)), Some(&Value::from(5_i64))),
        Ok(Value::from(10_i64))
    );
    assert_eq!(
        max_value(Some(&Value::from(10_i64)), Some(&Value::from(10.5_f64))),
        Ok(Value::from(10.5_f64))
    );
    assert_eq!(
        max_value(Some(&Value::from(false)), Some(&Value::from(true))),
        Ok(Value::from(true))
    );
    assert_eq!(
        max_value(Some(&Value::from(10_i64)), Some(&Value::from(10.0_f64))),
        Ok(Value::from(10_i64))
    );
    assert_eq!(
        min_value(Some(&Value::from(10_i64)), Some(&Value::from(5_i64))),
        Ok(Value::from(5_i64))
    );
    assert_eq!(
        min_value(Some(&Value::from(10_i64)), Some(&Value::from(10.5_f64))),
        Ok(Value::from(10_i64))
    );
    assert_eq!(
        min_value(Some(&Value::from(false)), Some(&Value::from(true))),
        Ok(Value::from(false))
    );
    assert_eq!(
        min_value(Some(&Value::from(10.0_f64)), Some(&Value::from(10_i64))),
        Ok(Value::from(10.0_f64))
    );
}

#[test]
fn public_max_and_min_report_xbase_style_argument_errors() {
    assert_eq!(
        max_value(Some(&Value::Nil), Some(&Value::Nil)),
        Err(RuntimeError {
            message: "BASE 1093 Argument error (MAX)".to_owned(),
            expected: None,
            actual: Some(harbour_rust_runtime::ValueKind::Nil),
        })
    );
    assert_eq!(
        min_value(Some(&Value::from(10_i64)), Some(&Value::Nil)),
        Err(RuntimeError {
            message: "BASE 1092 Argument error (MIN)".to_owned(),
            expected: None,
            actual: Some(harbour_rust_runtime::ValueKind::Nil),
        })
    );
    assert_eq!(
        max_value(None, Some(&Value::from(10_i64))),
        Err(RuntimeError {
            message: "BASE 1093 Argument error (MAX)".to_owned(),
            expected: None,
            actual: None,
        })
    );
    assert_eq!(
        min_value(Some(&Value::from("A")), Some(&Value::from(1_i64))),
        Err(RuntimeError {
            message: "BASE 1092 Argument error (MIN)".to_owned(),
            expected: None,
            actual: Some(harbour_rust_runtime::ValueKind::String),
        })
    );
}

#[test]
fn public_max_and_min_dispatch_through_the_immutable_builtin_surface() {
    let mut context = RuntimeContext::new();

    assert_eq!(
        call_builtin(
            "MAX",
            &[Value::from(10_i64), Value::from(100_i64)],
            &mut context
        ),
        Ok(Value::from(100_i64))
    );
    assert_eq!(
        call_builtin(
            "MIN",
            &[Value::from(true), Value::from(false)],
            &mut context
        ),
        Ok(Value::from(false))
    );

    let mut mutable_arguments = [Value::from(2.5_f64), Value::from(2_i64)];
    assert_eq!(
        call_builtin_mut("max", &mut mutable_arguments, &mut context),
        Ok(Value::from(2.5_f64))
    );
    assert_eq!(
        call_builtin_mut("min", &mut mutable_arguments, &mut context),
        Ok(Value::from(2_i64))
    );
    assert_eq!(mutable_arguments[0], Value::from(2.5_f64));
    assert_eq!(mutable_arguments[1], Value::from(2_i64));
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
            Some(&Value::from(-10_i64)),
            Some(&Value::from(15_i64)),
        ),
        Ok(Value::from("abcdef"))
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
            Some(&Value::from(0_i64)),
            None
        ),
        Ok(Value::from("abcdef"))
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
fn public_left_matches_the_current_runtime_baseline() {
    assert_eq!(
        left(Some(&Value::from("abcdef")), Some(&Value::from(-2_i64))),
        Ok(Value::from(""))
    );
    assert_eq!(
        left(Some(&Value::from("abcdef")), Some(&Value::from(2_i64))),
        Ok(Value::from("ab"))
    );
    assert_eq!(
        left(Some(&Value::from("abcdef")), Some(&Value::from(10_i64))),
        Ok(Value::from("abcdef"))
    );
    assert_eq!(
        left(Some(&Value::from("abcdef")), Some(&Value::from(0_i64))),
        Ok(Value::from(""))
    );
}

#[test]
fn public_left_reports_xbase_style_argument_errors() {
    assert_eq!(
        left(Some(&Value::from(100_i64)), Some(&Value::from(-10_i64))),
        Err(RuntimeError {
            message: "BASE 1124 Argument error (LEFT)".to_owned(),
            expected: None,
            actual: Some(harbour_rust_runtime::ValueKind::Integer),
        })
    );
    assert_eq!(
        left(Some(&Value::from("abcdef")), Some(&Value::from("A"))),
        Err(RuntimeError {
            message: "BASE 1124 Argument error (LEFT)".to_owned(),
            expected: None,
            actual: Some(harbour_rust_runtime::ValueKind::String),
        })
    );
}

#[test]
fn public_right_matches_the_current_lenient_runtime_baseline() {
    assert_eq!(
        right(Some(&Value::from(100_i64)), Some(&Value::from(-10_i64))),
        Ok(Value::from(""))
    );
    assert_eq!(
        right(Some(&Value::from("abcdef")), Some(&Value::from("A"))),
        Ok(Value::from(""))
    );
    assert_eq!(
        right(Some(&Value::from("abcdef")), Some(&Value::from(-2_i64))),
        Ok(Value::from(""))
    );
    assert_eq!(
        right(Some(&Value::from("abcdef")), Some(&Value::from(2_i64))),
        Ok(Value::from("ef"))
    );
    assert_eq!(
        right(Some(&Value::from("abcdef")), Some(&Value::from(10_i64))),
        Ok(Value::from("abcdef"))
    );
    assert_eq!(
        right(Some(&Value::from("abcdef")), Some(&Value::from(0_i64))),
        Ok(Value::from(""))
    );
}

#[test]
fn public_left_and_right_dispatch_through_the_immutable_builtin_surface() {
    let mut context = RuntimeContext::new();

    assert_eq!(
        call_builtin(
            "LEFT",
            &[Value::from("abcdef"), Value::from(2_i64)],
            &mut context,
        ),
        Ok(Value::from("ab"))
    );
    assert_eq!(
        call_builtin(
            "right",
            &[Value::from("abcdef"), Value::from(2_i64)],
            &mut context,
        ),
        Ok(Value::from("ef"))
    );

    let mut mutable_arguments = [Value::from("abcdef"), Value::from(10_i64)];
    assert_eq!(
        call_builtin_mut("RIGHT", &mut mutable_arguments, &mut context),
        Ok(Value::from("abcdef"))
    );
    assert_eq!(mutable_arguments[0], Value::from("abcdef"));
}

#[test]
fn public_upper_and_lower_match_the_current_ascii_runtime_baseline() {
    assert_eq!(
        upper(Some(&Value::from("aAZAZa"))),
        Ok(Value::from("AAZAZA"))
    );
    assert_eq!(upper(Some(&Value::from("2"))), Ok(Value::from("2")));
    assert_eq!(
        lower(Some(&Value::from("AazazA"))),
        Ok(Value::from("aazaza"))
    );
    assert_eq!(lower(Some(&Value::from("{"))), Ok(Value::from("{")));
}

#[test]
fn public_upper_and_lower_report_xbase_style_argument_errors() {
    assert_eq!(
        upper(Some(&Value::from(100_i64))),
        Err(RuntimeError {
            message: "BASE 1102 Argument error (UPPER)".to_owned(),
            expected: None,
            actual: Some(harbour_rust_runtime::ValueKind::Integer),
        })
    );
    assert_eq!(
        lower(Some(&Value::from(100_i64))),
        Err(RuntimeError {
            message: "BASE 1103 Argument error (LOWER)".to_owned(),
            expected: None,
            actual: Some(harbour_rust_runtime::ValueKind::Integer),
        })
    );
}

#[test]
fn public_upper_and_lower_dispatch_through_the_immutable_builtin_surface() {
    let mut context = RuntimeContext::new();

    assert_eq!(
        call_builtin("UPPER", &[Value::from("harbour")], &mut context),
        Ok(Value::from("HARBOUR"))
    );
    assert_eq!(
        call_builtin("lower", &[Value::from("HARBOUR")], &mut context),
        Ok(Value::from("harbour"))
    );

    let mut mutable_arguments = [Value::from("MiXeD")];
    assert_eq!(
        call_builtin_mut("UPPER", &mut mutable_arguments, &mut context),
        Ok(Value::from("MIXED"))
    );
    assert_eq!(mutable_arguments[0], Value::from("MiXeD"));
}

#[test]
fn public_at_matches_the_current_harbour_runtime_baseline() {
    assert_eq!(
        at(Some(&Value::from("")), Some(&Value::from(""))),
        Ok(Value::from(0_i64))
    );
    assert_eq!(
        at(Some(&Value::from("")), Some(&Value::from("ABCDEF"))),
        Ok(Value::from(0_i64))
    );
    assert_eq!(
        at(Some(&Value::from("ABCDEF")), Some(&Value::from(""))),
        Ok(Value::from(0_i64))
    );
    assert_eq!(
        at(Some(&Value::from("AB")), Some(&Value::from("AB"))),
        Ok(Value::from(1_i64))
    );
    assert_eq!(
        at(Some(&Value::from("AB")), Some(&Value::from("AAB"))),
        Ok(Value::from(2_i64))
    );
    assert_eq!(
        at(Some(&Value::from("X")), Some(&Value::from("ABCDEF"))),
        Ok(Value::from(0_i64))
    );
}

#[test]
fn public_at_reports_xbase_style_argument_errors() {
    assert_eq!(
        at(Some(&Value::from(90_i64)), Some(&Value::from(100_i64))),
        Err(RuntimeError {
            message: "BASE 1108 Argument error (AT)".to_owned(),
            expected: None,
            actual: Some(harbour_rust_runtime::ValueKind::Integer),
        })
    );
    assert_eq!(
        at(Some(&Value::from("")), Some(&Value::from(100_i64))),
        Err(RuntimeError {
            message: "BASE 1108 Argument error (AT)".to_owned(),
            expected: None,
            actual: Some(harbour_rust_runtime::ValueKind::Integer),
        })
    );
}

#[test]
fn public_at_dispatches_through_the_immutable_builtin_surface() {
    let mut context = RuntimeContext::new();

    assert_eq!(
        call_builtin("AT", &[Value::from("AB"), Value::from("AAB")], &mut context,),
        Ok(Value::from(2_i64))
    );

    let mut mutable_arguments = [Value::from("X"), Value::from("ABCDEF")];
    assert_eq!(
        call_builtin_mut("at", &mut mutable_arguments, &mut context),
        Ok(Value::from(0_i64))
    );
    assert_eq!(mutable_arguments[0], Value::from("X"));
}

#[test]
fn public_replicate_and_space_match_the_current_runtime_baseline() {
    assert_eq!(
        replicate(Some(&Value::from("")), Some(&Value::from(10_i64))),
        Ok(Value::from(""))
    );
    assert_eq!(
        replicate(Some(&Value::from("A")), Some(&Value::from(2_i64))),
        Ok(Value::from("AA"))
    );
    assert_eq!(
        replicate(Some(&Value::from("HE")), Some(&Value::from(3.7_f64))),
        Ok(Value::from("HEHEHE"))
    );
    assert_eq!(
        replicate(Some(&Value::from("HE")), Some(&Value::from(-3_i64))),
        Ok(Value::from(""))
    );

    assert_eq!(space(Some(&Value::from(0_i64))), Ok(Value::from("")));
    assert_eq!(space(Some(&Value::from(-10_i64))), Ok(Value::from("")));
    assert_eq!(space(Some(&Value::from(3_i64))), Ok(Value::from("   ")));
    assert_eq!(space(Some(&Value::from(3.7_f64))), Ok(Value::from("   ")));
}

#[test]
fn public_replicate_and_space_report_xbase_style_argument_errors() {
    assert_eq!(
        replicate(Some(&Value::from(200_i64)), Some(&Value::from(0_i64))),
        Err(RuntimeError {
            message: "BASE 1106 Argument error (REPLICATE)".to_owned(),
            expected: None,
            actual: Some(harbour_rust_runtime::ValueKind::Integer),
        })
    );
    assert_eq!(
        replicate(Some(&Value::from("A")), Some(&Value::from("B"))),
        Err(RuntimeError {
            message: "BASE 1106 Argument error (REPLICATE)".to_owned(),
            expected: None,
            actual: Some(harbour_rust_runtime::ValueKind::String),
        })
    );
    assert_eq!(
        replicate(Some(&Value::from("XXX")), Some(&Value::from(30_000_i64))),
        Err(RuntimeError {
            message: "BASE 1234 String overflow (REPLICATE)".to_owned(),
            expected: None,
            actual: None,
        })
    );
    assert_eq!(
        space(Some(&Value::from("A"))),
        Err(RuntimeError {
            message: "BASE 1105 Argument error (SPACE)".to_owned(),
            expected: None,
            actual: Some(harbour_rust_runtime::ValueKind::String),
        })
    );
}

#[test]
fn public_replicate_and_space_dispatch_through_the_immutable_builtin_surface() {
    let mut context = RuntimeContext::new();

    assert_eq!(
        call_builtin(
            "REPLICATE",
            &[Value::from("HE"), Value::from(3.1_f64)],
            &mut context,
        ),
        Ok(Value::from("HEHEHE"))
    );
    assert_eq!(
        call_builtin("space", &[Value::from(4_i64)], &mut context),
        Ok(Value::from("    "))
    );

    let mut mutable_arguments = [Value::from("A"), Value::from(1_i64)];
    assert_eq!(
        call_builtin_mut("REPLICATE", &mut mutable_arguments, &mut context),
        Ok(Value::from("A"))
    );
    assert_eq!(mutable_arguments[0], Value::from("A"));
}

#[test]
fn public_str_matches_the_current_numeric_runtime_baseline() {
    assert_eq!(
        str_value(Some(&Value::from(10_i64)), None, None),
        Ok(Value::from("        10"))
    );
    assert_eq!(
        str_value(Some(&Value::from(0_i64)), None, None),
        Ok(Value::from("         0"))
    );
    assert_eq!(
        str_value(Some(&Value::from(10.5_f64)), None, None),
        Ok(Value::from("      10.5"))
    );
    assert_eq!(
        str_value(Some(&Value::from(10_i64)), Some(&Value::from(5_i64)), None),
        Ok(Value::from("   10"))
    );
    assert_eq!(
        str_value(Some(&Value::from(10_i64)), Some(&Value::from(-5_i64)), None),
        Ok(Value::from("        10"))
    );
    assert_eq!(
        str_value(
            Some(&Value::from(10.6_f64)),
            Some(&Value::from(5_i64)),
            None,
        ),
        Ok(Value::from("   11"))
    );
    assert_eq!(
        str_value(
            Some(&Value::from(10.5_f64)),
            Some(&Value::from(-5_i64)),
            None,
        ),
        Ok(Value::from("        10"))
    );
    assert_eq!(
        str_value(
            Some(&Value::from(-10_i64)),
            Some(&Value::from(-5_i64)),
            None,
        ),
        Ok(Value::from("       -10"))
    );
    assert_eq!(
        str_value(
            Some(&Value::from(2_i64)),
            Some(&Value::from(5_i64)),
            Some(&Value::from(2_i64)),
        ),
        Ok(Value::from(" 2.00"))
    );
    assert_eq!(
        str_value(
            Some(&Value::from(100000_i64)),
            Some(&Value::from(-8_i64)),
            None,
        ),
        Ok(Value::from("    100000"))
    );
    assert_eq!(
        str_value(
            Some(&Value::from(100000_i64)),
            Some(&Value::from(5_i64)),
            None,
        ),
        Ok(Value::from("*****"))
    );
}

#[test]
fn public_str_reports_xbase_style_argument_errors() {
    assert_eq!(
        str_value(Some(&Value::Nil), None, None),
        Err(RuntimeError {
            message: "BASE 1099 Argument error (STR)".to_owned(),
            expected: None,
            actual: Some(harbour_rust_runtime::ValueKind::Nil),
        })
    );
    assert_eq!(
        str_value(
            Some(&Value::from("A")),
            Some(&Value::from(10_i64)),
            Some(&Value::from(2_i64)),
        ),
        Err(RuntimeError {
            message: "BASE 1099 Argument error (STR)".to_owned(),
            expected: None,
            actual: Some(harbour_rust_runtime::ValueKind::String),
        })
    );
    assert_eq!(
        str_value(
            Some(&Value::from(100_i64)),
            Some(&Value::from(10_i64)),
            Some(&Value::from("A")),
        ),
        Err(RuntimeError {
            message: "BASE 1099 Argument error (STR)".to_owned(),
            expected: None,
            actual: Some(harbour_rust_runtime::ValueKind::String),
        })
    );
}

#[test]
fn public_str_dispatches_through_the_immutable_builtin_surface() {
    let mut context = RuntimeContext::new();

    assert_eq!(
        call_builtin("STR", &[Value::from(10_i64)], &mut context),
        Ok(Value::from("        10"))
    );
    assert_eq!(
        call_builtin(
            "str",
            &[Value::from(2_i64), Value::from(5_i64), Value::from(2_i64)],
            &mut context,
        ),
        Ok(Value::from(" 2.00"))
    );

    let mut mutable_arguments = [Value::from(10.6_f64), Value::from(5_i64)];
    assert_eq!(
        call_builtin_mut("STR", &mut mutable_arguments, &mut context),
        Ok(Value::from("   11"))
    );
    assert_eq!(mutable_arguments[0], Value::from(10.6_f64));
}

#[test]
fn public_val_matches_the_current_string_to_numeric_runtime_baseline() {
    assert_eq!(val(Some(&Value::from(""))), Ok(Value::from(0_i64)));
    assert_eq!(val(Some(&Value::from("A"))), Ok(Value::from(0_i64)));
    assert_eq!(val(Some(&Value::from("10"))), Ok(Value::from(10_i64)));
    assert_eq!(val(Some(&Value::from("  -12"))), Ok(Value::from(-12_i64)));
    assert_eq!(
        val(Some(&Value::from("15.001 "))),
        Ok(Value::from(15.001_f64))
    );
    assert_eq!(val(Some(&Value::from("1HELLO."))), Ok(Value::from(1_i64)));
    assert_eq!(val(Some(&Value::from("0x10"))), Ok(Value::from(0_i64)));
    assert_eq!(val(Some(&Value::from(".1"))), Ok(Value::from(0.1_f64)));
    assert_eq!(val(Some(&Value::from("-.1"))), Ok(Value::from(-0.1_f64)));
}

#[test]
fn public_val_reports_xbase_style_argument_errors() {
    assert_eq!(
        val(Some(&Value::Nil)),
        Err(RuntimeError {
            message: "BASE 1098 Argument error (VAL)".to_owned(),
            expected: None,
            actual: Some(harbour_rust_runtime::ValueKind::Nil),
        })
    );
    assert_eq!(
        val(Some(&Value::from(10_i64))),
        Err(RuntimeError {
            message: "BASE 1098 Argument error (VAL)".to_owned(),
            expected: None,
            actual: Some(harbour_rust_runtime::ValueKind::Integer),
        })
    );
}

#[test]
fn public_val_dispatches_through_the_immutable_builtin_surface() {
    let mut context = RuntimeContext::new();

    assert_eq!(
        call_builtin("VAL", &[Value::from("15.001 ")], &mut context),
        Ok(Value::from(15.001_f64))
    );

    let mut mutable_arguments = [Value::from("1HELLO.")];
    assert_eq!(
        call_builtin_mut("val", &mut mutable_arguments, &mut context),
        Ok(Value::from(1_i64))
    );
    assert_eq!(mutable_arguments[0], Value::from("1HELLO."));
}

#[test]
fn public_valtype_matches_the_current_runtime_baseline() {
    assert_eq!(valtype(None), Ok(Value::from("U")));
    assert_eq!(valtype(Some(&Value::Nil)), Ok(Value::from("U")));
    assert_eq!(valtype(Some(&Value::from(true))), Ok(Value::from("L")));
    assert_eq!(valtype(Some(&Value::from(10_i64))), Ok(Value::from("N")));
    assert_eq!(valtype(Some(&Value::from(10.5_f64))), Ok(Value::from("N")));
    assert_eq!(valtype(Some(&Value::from("abc"))), Ok(Value::from("C")));
    assert_eq!(
        valtype(Some(&Value::array(vec![
            Value::from(1_i64),
            Value::from(2_i64)
        ]))),
        Ok(Value::from("A"))
    );
    assert_eq!(
        valtype(Some(&Value::codeblock(
            "{|| NIL}",
            |_arguments, _context| Ok(Value::Nil)
        ))),
        Ok(Value::from("B"))
    );
}

#[test]
fn public_valtype_dispatches_through_builtin_surfaces() {
    let mut context = RuntimeContext::new();

    assert_eq!(
        call_builtin("VALTYPE", &[], &mut context),
        Ok(Value::from("U"))
    );
    assert_eq!(
        call_builtin("valtype", &[Value::from("abc")], &mut context),
        Ok(Value::from("C"))
    );

    let mut mutable_arguments = [Value::array(vec![Value::from(1_i64)])];
    assert_eq!(
        call_builtin_mut("VALTYPE", &mut mutable_arguments, &mut context),
        Ok(Value::from("A"))
    );
    assert_eq!(mutable_arguments[0], Value::array(vec![Value::from(1_i64)]));
}

#[test]
fn public_type_matches_the_current_textual_runtime_baseline() {
    assert_eq!(type_value(Some(&Value::from("NIL"))), Ok(Value::from("U")));
    assert_eq!(type_value(Some(&Value::from(".T."))), Ok(Value::from("L")));
    assert_eq!(type_value(Some(&Value::from("10.5"))), Ok(Value::from("N")));
    assert_eq!(
        type_value(Some(&Value::from("{ 1, 2 }"))),
        Ok(Value::from("A"))
    );
    assert_eq!(
        type_value(Some(&Value::from("'abc'"))),
        Ok(Value::from("C"))
    );
    assert_eq!(
        type_value(Some(&Value::from("missingVar"))),
        Ok(Value::from("U"))
    );
}

#[test]
fn public_type_reports_xbase_style_argument_errors() {
    assert_eq!(
        type_value(None),
        Err(RuntimeError {
            message: "BASE 1121 Argument error (TYPE)".to_owned(),
            expected: None,
            actual: None,
        })
    );
    assert_eq!(
        type_value(Some(&Value::from(10_i64))),
        Err(RuntimeError {
            message: "BASE 1121 Argument error (TYPE)".to_owned(),
            expected: None,
            actual: Some(harbour_rust_runtime::ValueKind::Integer),
        })
    );
}

#[test]
fn public_type_dispatches_through_builtin_surfaces() {
    let mut context = RuntimeContext::new();

    assert_eq!(
        call_builtin("TYPE", &[Value::from(".F.")], &mut context),
        Ok(Value::from("L"))
    );

    let mut mutable_arguments = [Value::from("{ 1 }")];
    assert_eq!(
        call_builtin_mut("type", &mut mutable_arguments, &mut context),
        Ok(Value::from("A"))
    );
    assert_eq!(mutable_arguments[0], Value::from("{ 1 }"));
}

#[test]
fn public_empty_matches_the_current_runtime_baseline() {
    assert_eq!(empty(None), Ok(Value::from(true)));
    assert_eq!(empty(Some(&Value::Nil)), Ok(Value::from(true)));
    assert_eq!(empty(Some(&Value::from(false))), Ok(Value::from(true)));
    assert_eq!(empty(Some(&Value::from(true))), Ok(Value::from(false)));
    assert_eq!(empty(Some(&Value::from(0_i64))), Ok(Value::from(true)));
    assert_eq!(empty(Some(&Value::from(10_i64))), Ok(Value::from(false)));
    assert_eq!(empty(Some(&Value::from(0.0_f64))), Ok(Value::from(true)));
    assert_eq!(empty(Some(&Value::from("  \r\t"))), Ok(Value::from(true)));
    assert_eq!(
        empty(Some(&Value::from(String::from(" \u{0000}")))),
        Ok(Value::from(false))
    );
    assert_eq!(empty(Some(&Value::empty_array())), Ok(Value::from(true)));
    assert_eq!(
        empty(Some(&Value::array(vec![Value::from(0_i64)]))),
        Ok(Value::from(false))
    );
    assert_eq!(
        empty(Some(&Value::codeblock(
            "{|| NIL}",
            |_arguments, _context| Ok(Value::Nil)
        ))),
        Ok(Value::from(false))
    );
}

#[test]
fn public_eval_dispatches_codeblocks_with_arguments() {
    let mut context = RuntimeContext::new();
    let block = Value::codeblock("{|p1, p2| p1 + p2 }", |arguments, _context| {
        let left = arguments.first().cloned().unwrap_or(Value::Nil);
        let right = arguments.get(1).cloned().unwrap_or(Value::Nil);
        left.add(&right)
    });

    assert_eq!(
        call_builtin(
            "EVAL",
            &[block.clone(), Value::from("A"), Value::from("B")],
            &mut context
        ),
        Ok(Value::from("AB"))
    );

    let mut mutable_arguments = [block, Value::from(10_i64), Value::from(5_i64)];
    assert_eq!(
        call_builtin_mut("eval", &mut mutable_arguments, &mut context),
        Ok(Value::from(15_i64))
    );
}

#[test]
fn public_eval_reports_xbase_style_argument_errors_for_non_codeblocks() {
    let mut context = RuntimeContext::new();

    assert_eq!(
        call_builtin("EVAL", &[], &mut context),
        Err(RuntimeError {
            message: "BASE 1004 Argument error (EVAL)".to_owned(),
            expected: Some(harbour_rust_runtime::ValueKind::Codeblock),
            actual: None,
        })
    );
    assert_eq!(
        call_builtin("EVAL", &[Value::from("not-a-block")], &mut context),
        Err(RuntimeError {
            message: "BASE 1004 Argument error (EVAL)".to_owned(),
            expected: Some(harbour_rust_runtime::ValueKind::Codeblock),
            actual: Some(harbour_rust_runtime::ValueKind::String),
        })
    );
}

#[test]
fn public_memvar_context_prefers_private_over_public_and_updates_existing_bindings() {
    let mut context = RuntimeContext::new();

    assert_eq!(
        context.define_public("counter", Value::from(10_i64)),
        Value::from(10_i64)
    );
    assert_eq!(context.read_memvar("counter"), Value::from(10_i64));

    context.push_private_frame();
    assert_eq!(
        context.define_private("counter", Value::from(1_i64)),
        Value::from(1_i64)
    );
    assert_eq!(context.read_memvar("counter"), Value::from(1_i64));

    assert_eq!(
        context.assign_memvar("counter", Value::from(2_i64)),
        Value::from(2_i64)
    );
    assert_eq!(context.read_memvar("counter"), Value::from(2_i64));

    context.pop_private_frame();
    assert_eq!(context.read_memvar("counter"), Value::from(10_i64));
}

#[test]
fn public_memvar_context_assigns_to_the_current_dynamic_scope_when_missing() {
    let mut context = RuntimeContext::new();

    context.push_private_frame();
    assert_eq!(
        context.assign_memvar("shadow", Value::from("private")),
        Value::from("private")
    );
    assert_eq!(context.read_memvar("shadow"), Value::from("private"));

    context.pop_private_frame();
    assert_eq!(context.read_memvar("shadow"), Value::Nil);

    assert_eq!(
        context.assign_memvar("global", Value::from(7_i64)),
        Value::from(7_i64)
    );
    assert_eq!(context.read_memvar("global"), Value::from(7_i64));
}

#[test]
fn public_empty_dispatches_through_builtin_surfaces() {
    let mut context = RuntimeContext::new();

    assert_eq!(
        call_builtin("EMPTY", &[], &mut context),
        Ok(Value::from(true))
    );
    assert_eq!(
        call_builtin("empty", &[Value::from("A")], &mut context),
        Ok(Value::from(false))
    );

    let mut mutable_arguments = [Value::from(false)];
    assert_eq!(
        call_builtin_mut("EMPTY", &mut mutable_arguments, &mut context),
        Ok(Value::from(true))
    );
    assert_eq!(mutable_arguments[0], Value::from(false));
}

#[test]
fn public_trim_variants_match_the_current_runtime_baseline() {
    assert_eq!(trim(Some(&Value::from("UA   "))), Ok(Value::from("UA")));
    assert_eq!(
        trim(Some(&Value::from("   UA  "))),
        Ok(Value::from("   UA"))
    );
    assert_eq!(
        rtrim(Some(&Value::from("   UA  "))),
        Ok(Value::from("   UA"))
    );
    assert_eq!(
        ltrim(Some(&Value::from("   UA  "))),
        Ok(Value::from("UA  "))
    );
    assert_eq!(ltrim(Some(&Value::from(" \tU\t"))), Ok(Value::from("U\t")));
}

#[test]
fn public_trim_variants_report_xbase_style_argument_errors() {
    assert_eq!(
        trim(Some(&Value::from(100_i64))),
        Err(RuntimeError {
            message: "BASE 1100 Argument error (TRIM)".to_owned(),
            expected: None,
            actual: Some(harbour_rust_runtime::ValueKind::Integer),
        })
    );
    assert_eq!(
        rtrim(Some(&Value::Nil)),
        Err(RuntimeError {
            message: "BASE 1100 Argument error (TRIM)".to_owned(),
            expected: None,
            actual: Some(harbour_rust_runtime::ValueKind::Nil),
        })
    );
    assert_eq!(
        ltrim(Some(&Value::from(100_i64))),
        Err(RuntimeError {
            message: "BASE 1101 Argument error (LTRIM)".to_owned(),
            expected: None,
            actual: Some(harbour_rust_runtime::ValueKind::Integer),
        })
    );
}

#[test]
fn public_trim_variants_dispatch_through_the_immutable_builtin_surface() {
    let mut context = RuntimeContext::new();

    assert_eq!(
        call_builtin("TRIM", &[Value::from("UA   ")], &mut context),
        Ok(Value::from("UA"))
    );
    assert_eq!(
        call_builtin("ltrim", &[Value::from("   UA  ")], &mut context),
        Ok(Value::from("UA  "))
    );
    assert_eq!(
        call_builtin("RTRIM", &[Value::from("   UA  ")], &mut context),
        Ok(Value::from("   UA"))
    );

    let mut mutable_arguments = [Value::from("  X  ")];
    assert_eq!(
        call_builtin_mut("TRIM", &mut mutable_arguments, &mut context),
        Ok(Value::from("  X"))
    );
    assert_eq!(mutable_arguments[0], Value::from("  X  "));
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
fn public_adel_and_ains_follow_the_current_lenient_runtime_baseline() {
    let mut values = Value::array(vec![
        Value::from(10_i64),
        Value::from(20_i64),
        Value::from(30_i64),
    ]);

    assert_eq!(
        ains(&mut values, Some(&Value::from(2_i64))),
        Ok(Value::array(vec![
            Value::from(10_i64),
            Value::Nil,
            Value::from(20_i64),
        ]))
    );
    assert_eq!(
        values,
        Value::array(vec![Value::from(10_i64), Value::Nil, Value::from(20_i64),])
    );

    assert_eq!(
        adel(&mut values, Some(&Value::from(1_i64))),
        Ok(Value::array(vec![
            Value::Nil,
            Value::from(20_i64),
            Value::Nil,
        ]))
    );
    assert_eq!(
        values,
        Value::array(vec![Value::Nil, Value::from(20_i64), Value::Nil,])
    );

    let mut untouched = Value::array(vec![Value::from(1_i64)]);
    assert_eq!(
        adel(&mut untouched, None),
        Ok(Value::array(vec![Value::from(1_i64)]))
    );
    assert_eq!(
        ains(&mut untouched, Some(&Value::from(100_i64))),
        Ok(Value::array(vec![Value::from(1_i64)]))
    );

    let mut not_array = Value::from("nope");
    assert_eq!(
        adel(&mut not_array, Some(&Value::from(1_i64))),
        Ok(Value::Nil)
    );
    assert_eq!(
        ains(&mut not_array, Some(&Value::from(1_i64))),
        Ok(Value::Nil)
    );
}

#[test]
fn public_ascan_follows_the_current_lenient_runtime_baseline() {
    let values = Value::array(vec![
        Value::from("HELLO"),
        Value::from(""),
        Value::from(10_i64),
        Value::from(true),
    ]);

    assert_eq!(
        ascan(Some(&values), Some(&Value::from("HELL")), None, None),
        Ok(Value::from(1_i64))
    );
    assert_eq!(
        ascan(
            Some(&values),
            Some(&Value::from("")),
            Some(&Value::from(2_i64)),
            None
        ),
        Ok(Value::from(2_i64))
    );
    assert_eq!(
        ascan(
            Some(&values),
            Some(&Value::from(10.0_f64)),
            Some(&Value::from(2_i64)),
            Some(&Value::from(2_i64))
        ),
        Ok(Value::from(3_i64))
    );
    assert_eq!(
        ascan(Some(&values), Some(&Value::from(false)), None, None),
        Ok(Value::from(0_i64))
    );
    assert_eq!(
        ascan(None, Some(&Value::from(1_i64)), None, None),
        Ok(Value::from(0_i64))
    );
    assert_eq!(
        ascan(
            Some(&Value::from("nope")),
            Some(&Value::from(1_i64)),
            None,
            None
        ),
        Ok(Value::from(0_i64))
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
    assert_eq!(
        call_builtin(
            "ADEL",
            &[Value::empty_array(), Value::from(1_i64)],
            &mut context
        ),
        Err(RuntimeError {
            message: "builtin ADEL requires mutable dispatch".to_owned(),
            expected: None,
            actual: None,
        })
    );
    assert_eq!(
        call_builtin(
            "AINS",
            &[Value::empty_array(), Value::from(1_i64)],
            &mut context
        ),
        Err(RuntimeError {
            message: "builtin AINS requires mutable dispatch".to_owned(),
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
fn public_array_builtin_dispatch_covers_ascan_adel_and_ains() {
    let mut context = RuntimeContext::new();
    let mut arguments = [
        Value::array(vec![
            Value::from(10_i64),
            Value::from(20_i64),
            Value::from(30_i64),
        ]),
        Value::from(2_i64),
    ];

    assert_eq!(
        call_builtin(
            "ASCAN",
            &[arguments[0].clone(), Value::from(20_i64)],
            &mut context
        ),
        Ok(Value::from(2_i64))
    );
    assert_eq!(
        call_builtin_mut("AINS", &mut arguments, &mut context),
        Ok(Value::array(vec![
            Value::from(10_i64),
            Value::Nil,
            Value::from(20_i64),
        ]))
    );
    assert_eq!(
        call_builtin_mut("ADEL", &mut arguments, &mut context),
        Ok(Value::array(vec![
            Value::from(10_i64),
            Value::from(20_i64),
            Value::Nil,
        ]))
    );
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
