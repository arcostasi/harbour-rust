use harbour_rust_runtime::Value;

#[test]
fn public_value_conversions_cover_core_variants() {
    assert_eq!(bool::try_from(&Value::from(true)), Ok(true));
    assert_eq!(i64::try_from(&Value::from(7_i64)), Ok(7));
    assert_eq!(f64::try_from(&Value::from(7_i64)), Ok(7.0));
    assert_eq!(f64::try_from(&Value::from(3.5_f64)), Ok(3.5));
    assert_eq!(String::try_from(&Value::from("abc")), Ok("abc".to_owned()));
    assert_eq!(
        Vec::<Value>::try_from(&Value::from(vec![Value::from(1_i64), Value::from("x")])),
        Ok(vec![Value::from(1_i64), Value::from("x")])
    );
}

#[test]
fn public_value_output_string_matches_runtime_baseline() {
    assert_eq!(Value::Nil.to_output_string(), "NIL");
    assert_eq!(Value::from(true).to_output_string(), ".T.");
    assert_eq!(Value::from(false).to_output_string(), ".F.");
    assert_eq!(Value::from(9_i64).to_output_string(), "9");
    assert_eq!(Value::from("text").to_output_string(), "text");
    assert_eq!(Value::array_with_len(2).to_output_string(), "{ Array(2) }");
}

#[test]
fn public_array_constructors_cover_empty_sized_and_explicit_values() {
    assert_eq!(Value::empty_array(), Value::from(Vec::<Value>::new()));
    assert_eq!(
        Value::array_with_len(3),
        Value::from(vec![Value::Nil, Value::Nil, Value::Nil])
    );
    assert_eq!(
        Value::array(vec![Value::from(1_i64), Value::from(true)]),
        Value::from(vec![Value::from(1_i64), Value::from(true)])
    );
}
