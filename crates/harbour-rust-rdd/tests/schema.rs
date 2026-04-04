use std::path::{Path, PathBuf};

use harbour_rust_rdd::{DbfSchema, DbfTable, FieldType};

fn workspace_path(path: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join(path)
}

fn fixture(path: &str) -> PathBuf {
    workspace_path(path)
}

#[test]
fn reads_schema_from_upstream_users_dbf() {
    let schema =
        DbfSchema::read_from_path(&fixture("harbour-core/contrib/hbhttpd/tests/users.dbf"))
            .expect("users schema");

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
    let schema =
        DbfSchema::read_from_path(&fixture("harbour-core/contrib/hbhttpd/tests/carts.dbf"))
            .expect("carts schema");

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
    let table = DbfTable::open(Path::new(
        &fixture("harbour-core/contrib/hbhttpd/tests/items.dbf"),
    ))
    .expect("items table");

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
