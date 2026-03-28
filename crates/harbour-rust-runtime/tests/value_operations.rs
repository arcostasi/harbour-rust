use harbour_rust_runtime::Value;

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
