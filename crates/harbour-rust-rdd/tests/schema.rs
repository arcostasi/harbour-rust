use std::{
    env, fs,
    io::ErrorKind,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

use harbour_rust_rdd::{DbfSchema, DbfTable, FieldType, Rdd};
use harbour_rust_runtime::Value;

fn workspace_path(path: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join(path)
}

fn fixture(path: &str) -> PathBuf {
    workspace_path(path)
}

fn upstream_fixture_or_skip(path: &str, label: &str) -> Option<PathBuf> {
    let full_path = fixture(path);
    match fs::metadata(&full_path) {
        Ok(_) => Some(full_path),
        Err(error) if error.kind() == ErrorKind::NotFound => {
            eprintln!(
                "skipping rdd oracle `{label}` because {} is not available",
                full_path.display()
            );
            None
        }
        Err(error) => panic!("{label}: {error}"),
    }
}

fn unique_temp_dir(label: &str) -> PathBuf {
    let suffix = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time")
        .as_nanos();
    env::temp_dir().join(format!("harbour-rust-rdd-{label}-{suffix}"))
}

#[test]
fn reads_schema_from_upstream_users_dbf() {
    let Some(users_dbf) = upstream_fixture_or_skip(
        "harbour-core/contrib/hbhttpd/tests/users.dbf",
        "users dbf fixture",
    ) else {
        return;
    };
    let schema = DbfSchema::read_from_path(&users_dbf).expect("users schema");

    assert_eq!(schema.header.version, 0x03);
    assert_eq!(schema.header.record_count, 0);
    assert_eq!(schema.header.header_length, 130);
    assert_eq!(schema.header.record_length, 83);
    assert_eq!(schema.fields.len(), 3);

    assert_eq!(schema.fields[0].name, "USER");
    assert_eq!(schema.fields[0].field_type, FieldType::Character);
    assert_eq!(schema.fields[0].length, 16);
    assert_eq!(schema.fields[0].offset, 1);

    assert_eq!(schema.fields[1].name, "PASSWORD");
    assert_eq!(schema.fields[1].field_type, FieldType::Character);
    assert_eq!(schema.fields[1].length, 16);
    assert_eq!(schema.fields[1].offset, 17);

    assert_eq!(schema.fields[2].name, "NAME");
    assert_eq!(schema.fields[2].field_type, FieldType::Character);
    assert_eq!(schema.fields[2].length, 50);
    assert_eq!(schema.fields[2].offset, 33);
}

#[test]
fn reads_schema_with_numeric_field_decimals_from_upstream_carts_dbf() {
    let Some(carts_dbf) = upstream_fixture_or_skip(
        "harbour-core/contrib/hbhttpd/tests/carts.dbf",
        "carts dbf fixture",
    ) else {
        return;
    };
    let schema = DbfSchema::read_from_path(&carts_dbf).expect("carts schema");

    assert_eq!(schema.header.record_count, 0);
    assert_eq!(schema.header.record_length, 48);
    assert_eq!(schema.fields.len(), 4);

    let amount = schema.field("amount").expect("amount field");
    assert_eq!(amount.field_type, FieldType::Numeric);
    assert_eq!(amount.length, 6);
    assert_eq!(amount.decimals, 0);

    let total = schema.field("TOTAL").expect("total field");
    assert_eq!(total.field_type, FieldType::Numeric);
    assert_eq!(total.length, 9);
    assert_eq!(total.decimals, 2);
}

#[test]
fn opens_upstream_items_dbf_table_with_expected_header() {
    let Some(items_dbf) = upstream_fixture_or_skip(
        "harbour-core/contrib/hbhttpd/tests/items.dbf",
        "items dbf fixture",
    ) else {
        return;
    };
    let table = DbfTable::open(Path::new(&items_dbf)).expect("items table");

    assert_eq!(table.schema().header.record_count, 29);
    assert_eq!(table.schema().fields.len(), 3);
    assert_eq!(table.schema().field("TITLE").expect("TITLE").length, 80);
    assert_eq!(table.schema().field("PRICE").expect("PRICE").decimals, 2);
    assert!(table.bof());
    assert!(!table.eof());
    assert_eq!(table.recno(), 0);
    assert!(!table.is_closed());
    assert!(!table.raw_bytes().is_empty());
}

#[test]
fn navigates_items_records_and_reads_fields() {
    let Some(items_dbf) = upstream_fixture_or_skip(
        "harbour-core/contrib/hbhttpd/tests/items.dbf",
        "items dbf fixture",
    ) else {
        return;
    };
    let mut table = DbfTable::open(Path::new(&items_dbf)).expect("items table");

    table.go_to(1).expect("goto first");
    assert_eq!(table.recno(), 1);
    assert_eq!(table.field_get("code").expect("code"), Value::from("0001"));
    assert_eq!(
        table.field_get("TITLE").expect("title"),
        Value::from("Linux in a Nutshell")
    );
    assert_eq!(
        table.field_get("PRICE").expect("price"),
        Value::Float(26.67)
    );
    assert!(!table.deleted().expect("deleted flag"));

    table.skip(1).expect("skip forward");
    assert_eq!(table.recno(), 2);
    assert_eq!(table.field_get("code").expect("code"), Value::from("0002"));
    assert_eq!(
        table.field_get("TITLE").expect("title"),
        Value::from("Python in a Nutshell")
    );
    assert_eq!(
        table.field_get("PRICE").expect("price"),
        Value::Float(26.39)
    );
}

#[test]
fn skip_tracks_bof_and_eof_boundaries() {
    let Some(items_dbf) = upstream_fixture_or_skip(
        "harbour-core/contrib/hbhttpd/tests/items.dbf",
        "items dbf fixture",
    ) else {
        return;
    };
    let mut table = DbfTable::open(Path::new(&items_dbf)).expect("items table");

    table.go_to(1).expect("goto first");
    table.skip(-1).expect("skip before first");
    assert!(table.bof());
    assert!(!table.eof());
    assert_eq!(table.recno(), 0);
    assert!(matches!(
        table.field_get("CODE"),
        Err(harbour_rust_rdd::RddError::NotPositioned)
    ));

    table.go_to(table.rec_count()).expect("goto last");
    table.skip(1).expect("skip after last");
    assert!(!table.bof());
    assert!(table.eof());
    assert_eq!(table.recno(), 0);
}

#[test]
fn appends_blank_and_persists_character_fields() {
    let Some(users_dbf) = upstream_fixture_or_skip(
        "harbour-core/contrib/hbhttpd/tests/users.dbf",
        "users dbf fixture",
    ) else {
        return;
    };
    let temp_dir = unique_temp_dir("users-write");
    fs::create_dir_all(&temp_dir).expect("temp dir");
    let temp_path = temp_dir.join("users.dbf");
    fs::copy(users_dbf, &temp_path).expect("copy users fixture");

    let mut table = DbfTable::open(&temp_path).expect("open temp users");
    table.append_blank().expect("append blank");
    table
        .field_put("USER", Value::from("admin"))
        .expect("write user");
    table
        .field_put("PASSWORD", Value::from("secret"))
        .expect("write password");
    table
        .field_put("NAME", Value::from("Administrator"))
        .expect("write name");
    table.close().expect("close table");

    let mut reopened = DbfTable::open(&temp_path).expect("reopen temp users");
    assert_eq!(reopened.rec_count(), 1);
    reopened.go_to(1).expect("goto first");
    assert_eq!(
        reopened.field_get("USER").expect("user"),
        Value::from("admin")
    );
    assert_eq!(
        reopened.field_get("PASSWORD").expect("password"),
        Value::from("secret")
    );
    assert_eq!(
        reopened.field_get("NAME").expect("name"),
        Value::from("Administrator")
    );

    fs::remove_dir_all(&temp_dir).expect("cleanup temp dir");
}

#[test]
fn appends_blank_and_persists_numeric_fields() {
    let Some(carts_dbf) = upstream_fixture_or_skip(
        "harbour-core/contrib/hbhttpd/tests/carts.dbf",
        "carts dbf fixture",
    ) else {
        return;
    };
    let temp_dir = unique_temp_dir("carts-write");
    fs::create_dir_all(&temp_dir).expect("temp dir");
    let temp_path = temp_dir.join("carts.dbf");
    fs::copy(carts_dbf, &temp_path).expect("copy carts fixture");

    let mut table = DbfTable::open(&temp_path).expect("open temp carts");
    table.append_blank().expect("append blank");
    table
        .field_put("USER", Value::from("alice"))
        .expect("write user");
    table
        .field_put("CODE", Value::from("0001"))
        .expect("write code");
    table
        .field_put("AMOUNT", Value::Integer(3))
        .expect("write amount");
    table
        .field_put("TOTAL", Value::Float(80.01))
        .expect("write total");

    let mut reopened = DbfTable::open(&temp_path).expect("reopen temp carts");
    assert_eq!(reopened.rec_count(), 1);
    reopened.go_to(1).expect("goto first");
    assert_eq!(
        reopened.field_get("USER").expect("user"),
        Value::from("alice")
    );
    assert_eq!(
        reopened.field_get("CODE").expect("code"),
        Value::from("0001")
    );
    assert_eq!(
        reopened.field_get("AMOUNT").expect("amount"),
        Value::Integer(3)
    );
    assert_eq!(
        reopened.field_get("TOTAL").expect("total"),
        Value::Float(80.01)
    );

    fs::remove_dir_all(&temp_dir).expect("cleanup temp dir");
}

#[test]
fn reads_logical_and_date_fields_from_upstream_test_dbf() {
    let Some(test_dbf) =
        upstream_fixture_or_skip("harbour-core/tests/test.dbf", "test dbf fixture")
    else {
        return;
    };
    let mut table = DbfTable::open(Path::new(&test_dbf)).expect("open test.dbf");

    table.go_to(1).expect("goto first");
    assert_eq!(
        table.field_get("FIRST").expect("first"),
        Value::from("Homer")
    );
    assert_eq!(
        table.field_get("LAST").expect("last"),
        Value::from("Simpson")
    );
    assert_eq!(
        table.field_get("HIREDATE").expect("hiredate"),
        Value::from("19920918")
    );
    assert_eq!(
        table.field_get("MARRIED").expect("married"),
        Value::Logical(true)
    );
}

#[test]
fn replaces_logical_and_date_fields_on_temp_test_dbf() {
    let Some(test_dbf) =
        upstream_fixture_or_skip("harbour-core/tests/test.dbf", "test dbf fixture")
    else {
        return;
    };
    let temp_dir = unique_temp_dir("test-write");
    fs::create_dir_all(&temp_dir).expect("temp dir");
    let temp_path = temp_dir.join("test.dbf");
    fs::copy(test_dbf, &temp_path).expect("copy test fixture");

    let mut table = DbfTable::open(&temp_path).expect("open temp test");
    table.go_to(1).expect("goto first");
    table
        .field_put("HIREDATE", Value::from("20260403"))
        .expect("write date");
    table
        .field_put("MARRIED", Value::Logical(false))
        .expect("write logical");

    let mut reopened = DbfTable::open(&temp_path).expect("reopen temp test");
    reopened.go_to(1).expect("goto first");
    assert_eq!(
        reopened.field_get("HIREDATE").expect("hiredate"),
        Value::from("20260403")
    );
    assert_eq!(
        reopened.field_get("MARRIED").expect("married"),
        Value::Logical(false)
    );

    fs::remove_dir_all(&temp_dir).expect("cleanup temp dir");
}

#[test]
fn delete_and_recall_persist_record_flag() {
    let Some(users_dbf) = upstream_fixture_or_skip(
        "harbour-core/contrib/hbhttpd/tests/users.dbf",
        "users dbf fixture",
    ) else {
        return;
    };
    let temp_dir = unique_temp_dir("users-delete");
    fs::create_dir_all(&temp_dir).expect("temp dir");
    let temp_path = temp_dir.join("users.dbf");
    fs::copy(users_dbf, &temp_path).expect("copy users fixture");

    let mut table = DbfTable::open(&temp_path).expect("open temp users");
    table.append_blank().expect("append blank");
    table
        .field_put("USER", Value::from("admin"))
        .expect("write user");
    table.delete().expect("delete record");

    let mut reopened = DbfTable::open(&temp_path).expect("reopen deleted users");
    reopened.go_to(1).expect("goto first");
    assert!(reopened.deleted().expect("deleted flag"));

    reopened.recall().expect("recall record");
    let mut recalled = DbfTable::open(&temp_path).expect("reopen recalled users");
    recalled.go_to(1).expect("goto first");
    assert!(!recalled.deleted().expect("deleted flag"));
    assert_eq!(
        recalled.field_get("USER").expect("user"),
        Value::from("admin")
    );

    fs::remove_dir_all(&temp_dir).expect("cleanup temp dir");
}
