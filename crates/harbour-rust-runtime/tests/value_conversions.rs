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

#[test]
fn public_array_index_helpers_follow_one_based_semantics() {
    let values = Value::array(vec![Value::from(10_i64), Value::from(20_i64)]);

    assert_eq!(values.array_len(), Ok(2));
    assert_eq!(
        values.array_get(&Value::from(1_i64)),
        Ok(&Value::from(10_i64))
    );
    assert_eq!(
        values.array_get_owned(&Value::from(2_i64)),
        Ok(Value::from(20_i64))
    );
    assert_eq!(
        values.array_get_path(&[Value::from(2_i64)]),
        Ok(&Value::from(20_i64))
    );
}

#[test]
fn public_array_set_helpers_follow_one_based_semantics() {
    let mut values = Value::array(vec![
        Value::array(vec![Value::from(10_i64), Value::from(20_i64)]),
        Value::array(vec![Value::from(30_i64), Value::from(40_i64)]),
    ]);

    assert_eq!(
        values.array_set(&Value::from(1_i64), Value::array(vec![Value::from(99_i64)])),
        Ok(Value::array(vec![Value::from(99_i64)]))
    );
    assert_eq!(
        values.array_set_path(&[Value::from(2_i64), Value::from(2_i64)], Value::from("ok")),
        Ok(Value::from("ok"))
    );
    assert_eq!(
        values.array_get_path(&[Value::from(2_i64), Value::from(2_i64)]),
        Ok(&Value::from("ok"))
    );
}

#[test]
fn public_array_collection_helpers_cover_resize_push_and_clone() {
    let mut values = Value::array(vec![Value::from(1_i64)]);

    assert_eq!(
        values.array_push(Value::from("tail")),
        Ok(Value::from("tail"))
    );
    assert_eq!(values.array_resize(4), Ok(()));
    assert_eq!(
        values,
        Value::array(vec![
            Value::from(1_i64),
            Value::from("tail"),
            Value::Nil,
            Value::Nil,
        ])
    );

    let cloned = values.array_clone();
    assert_eq!(cloned, Ok(values.clone()));

    assert_eq!(values.array_resize(1), Ok(()));
    assert_eq!(values, Value::array(vec![Value::from(1_i64)]));
    assert_eq!(
        cloned,
        Ok(Value::array(vec![
            Value::from(1_i64),
            Value::from("tail"),
            Value::Nil,
            Value::Nil,
        ]))
    );
}
