use std::{fs, path::PathBuf};

use harbour_rust_pp::{FileSystemIncludeResolver, Preprocessor, SourceFile};

fn fixture_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("tests")
        .join("fixtures")
        .join("pp")
        .join(name)
}

#[test]
fn preprocesses_include_fixture_with_filesystem_resolver() {
    let root = fixture_path("include_root.prg");
    let expected = fs::read_to_string(fixture_path("include_root.out")).unwrap();

    let output = Preprocessor::default().preprocess(SourceFile::from_path(&root).unwrap());

    assert!(
        output.errors.is_empty(),
        "unexpected errors: {:?}",
        output.errors
    );
    assert_eq!(output.text, expected);
    assert_eq!(
        output
            .defines
            .iter()
            .map(|define| define.name.as_str())
            .collect::<Vec<_>>(),
        vec!["APP_NAME", "GREETING"]
    );
    assert_eq!(
        output
            .line_origins
            .iter()
            .map(|origin| {
                (
                    origin
                        .source_path
                        .file_name()
                        .unwrap()
                        .to_string_lossy()
                        .into_owned(),
                    origin.source_line,
                )
            })
            .collect::<Vec<_>>(),
        vec![
            ("shared.ch".to_owned(), 2),
            ("include_root.prg".to_owned(), 3),
            ("include_root.prg".to_owned(), 4),
            ("include_root.prg".to_owned(), 5),
        ]
    );
}

#[test]
fn preprocesses_object_like_define_fixture() {
    let root = fixture_path("define_root.prg");
    let expected = fs::read_to_string(fixture_path("define_root.out")).unwrap();

    let output = Preprocessor::default().preprocess(SourceFile::from_path(&root).unwrap());

    assert!(
        output.errors.is_empty(),
        "unexpected errors: {:?}",
        output.errors
    );
    assert_eq!(output.text, expected);
    assert_eq!(
        output
            .defines
            .iter()
            .map(|define| define.name.as_str())
            .collect::<Vec<_>>(),
        vec!["APP_NAME", "GREETING"]
    );
}

#[test]
fn preprocesses_recursive_object_like_define_fixture() {
    let root = fixture_path("recursive_define_root.prg");
    let expected = fs::read_to_string(fixture_path("recursive_define_root.out")).unwrap();

    let output = Preprocessor::default().preprocess(SourceFile::from_path(&root).unwrap());

    assert!(
        output.errors.is_empty(),
        "unexpected errors: {:?}",
        output.errors
    );
    assert_eq!(output.text, expected);
    assert_eq!(
        output
            .defines
            .iter()
            .map(|define| define.name.as_str())
            .collect::<Vec<_>>(),
        vec!["APP_NAME", "GREETING"]
    );
}

#[test]
fn reports_cyclic_object_like_define_fixture() {
    let root = fixture_path("cyclic_define_root.prg");

    let output = Preprocessor::default().preprocess(SourceFile::from_path(&root).unwrap());

    assert_eq!(output.text, "PROCEDURE Main()\n   ? A\nRETURN\n");
    assert_eq!(output.errors.len(), 1);
    assert_eq!(
        output.errors[0].message,
        "cyclic define expansion detected: A -> B -> A"
    );
}

#[test]
fn resolves_quoted_include_through_search_paths() {
    let root = fixture_path("quoted_search_path_root.prg");
    let expected = fs::read_to_string(fixture_path("quoted_search_path_root.out")).unwrap();
    let resolver = FileSystemIncludeResolver::new().with_search_path(fixture_path("include-path"));

    let output = Preprocessor::new(resolver).preprocess(SourceFile::from_path(&root).unwrap());

    assert!(
        output.errors.is_empty(),
        "unexpected errors: {:?}",
        output.errors
    );
    assert_eq!(output.text, expected);
}

#[test]
fn resolves_angle_include_through_search_paths() {
    let root = fixture_path("angle_search_path_root.prg");
    let expected = fs::read_to_string(fixture_path("angle_search_path_root.out")).unwrap();
    let resolver = FileSystemIncludeResolver::new().with_search_path(fixture_path("include-path"));

    let output = Preprocessor::new(resolver).preprocess(SourceFile::from_path(&root).unwrap());

    assert!(
        output.errors.is_empty(),
        "unexpected errors: {:?}",
        output.errors
    );
    assert_eq!(output.text, expected);
}

#[test]
fn preprocesses_command_and_translate_fixture() {
    let root = fixture_path("command_translate_root.prg");
    let expected = fs::read_to_string(fixture_path("command_translate_root.out")).unwrap();

    let output = Preprocessor::default().preprocess(SourceFile::from_path(&root).unwrap());

    assert!(
        output.errors.is_empty(),
        "unexpected errors: {:?}",
        output.errors
    );
    assert_eq!(output.text, expected);
    assert_eq!(output.rules.len(), 4);
}

#[test]
fn preprocesses_optional_list_and_restricted_rule_fixture() {
    let root = fixture_path("rule_markers_root.prg");
    let expected = fs::read_to_string(fixture_path("rule_markers_root.out")).unwrap();

    let output = Preprocessor::default().preprocess(SourceFile::from_path(&root).unwrap());

    assert!(
        output.errors.is_empty(),
        "unexpected errors: {:?}",
        output.errors
    );
    assert_eq!(output.text, expected);
    assert_eq!(output.rules.len(), 2);
}

#[test]
fn preprocesses_optional_stringify_fixture() {
    let root = fixture_path("optional_stringify_root.prg");
    let expected = fs::read_to_string(fixture_path("optional_stringify_root.out")).unwrap();

    let output = Preprocessor::default().preprocess(SourceFile::from_path(&root).unwrap());

    assert!(
        output.errors.is_empty(),
        "unexpected errors: {:?}",
        output.errors
    );
    assert_eq!(output.text, expected);
    assert_eq!(output.rules.len(), 1);
}

#[test]
fn preprocesses_logical_result_marker_fixture() {
    let root = fixture_path("logical_marker_root.prg");
    let expected = fs::read_to_string(fixture_path("logical_marker_root.out")).unwrap();

    let output = Preprocessor::default().preprocess(SourceFile::from_path(&root).unwrap());

    assert!(
        output.errors.is_empty(),
        "unexpected errors: {:?}",
        output.errors
    );
    assert_eq!(output.text, expected);
    assert_eq!(output.rules.len(), 1);
}

#[test]
fn preprocesses_quoted_result_marker_fixture() {
    let root = fixture_path("quoted_marker_root.prg");
    let expected = fs::read_to_string(fixture_path("quoted_marker_root.out")).unwrap();

    let output = Preprocessor::default().preprocess(SourceFile::from_path(&root).unwrap());

    assert!(
        output.errors.is_empty(),
        "unexpected errors: {:?}",
        output.errors
    );
    assert_eq!(output.text, expected);
    assert_eq!(output.rules.len(), 1);
}

#[test]
fn preprocesses_quoted_result_marker_macro_fixture() {
    let root = fixture_path("quoted_macro_marker_root.prg");
    let expected = fs::read_to_string(fixture_path("quoted_macro_marker_root.out")).unwrap();

    let output = Preprocessor::default().preprocess(SourceFile::from_path(&root).unwrap());

    assert!(
        output.errors.is_empty(),
        "unexpected errors: {:?}",
        output.errors
    );
    assert_eq!(output.text, expected);
    assert_eq!(output.rules.len(), 1);
}

#[test]
fn preprocesses_smart_result_marker_fixture() {
    let root = fixture_path("smart_marker_root.prg");
    let expected = fs::read_to_string(fixture_path("smart_marker_root.out")).unwrap();

    let output = Preprocessor::default().preprocess(SourceFile::from_path(&root).unwrap());

    assert!(
        output.errors.is_empty(),
        "unexpected errors: {:?}",
        output.errors
    );
    assert_eq!(output.text, expected);
    assert_eq!(output.rules.len(), 1);
}

#[test]
fn preprocesses_smart_result_marker_macro_fixture() {
    let root = fixture_path("smart_marker_macro_root.prg");
    let expected = fs::read_to_string(fixture_path("smart_marker_macro_root.out")).unwrap();

    let output = Preprocessor::default().preprocess(SourceFile::from_path(&root).unwrap());

    assert!(
        output.errors.is_empty(),
        "unexpected errors: {:?}",
        output.errors
    );
    assert_eq!(output.text, expected);
    assert_eq!(output.rules.len(), 1);
}

#[test]
fn preprocesses_blockify_result_marker_fixture() {
    let root = fixture_path("blockify_marker_root.prg");
    let expected = fs::read_to_string(fixture_path("blockify_marker_root.out")).unwrap();

    let output = Preprocessor::default().preprocess(SourceFile::from_path(&root).unwrap());

    assert!(
        output.errors.is_empty(),
        "unexpected errors: {:?}",
        output.errors
    );
    assert_eq!(output.text, expected);
    assert_eq!(output.rules.len(), 1);
}

#[test]
fn preprocesses_macro_pattern_translate_fixture() {
    let root = fixture_path("macro_pattern_translate_root.prg");
    let expected = fs::read_to_string(fixture_path("macro_pattern_translate_root.out")).unwrap();

    let output = Preprocessor::default().preprocess(SourceFile::from_path(&root).unwrap());

    assert!(
        output.errors.is_empty(),
        "unexpected errors: {:?}",
        output.errors
    );
    assert_eq!(output.text, expected);
    assert_eq!(output.rules.len(), 1);
}

#[test]
fn preprocesses_macro_pattern_command_fixture() {
    let root = fixture_path("macro_pattern_command_root.prg");
    let expected = fs::read_to_string(fixture_path("macro_pattern_command_root.out")).unwrap();

    let output = Preprocessor::default().preprocess(SourceFile::from_path(&root).unwrap());

    assert!(
        output.errors.is_empty(),
        "unexpected errors: {:?}",
        output.errors
    );
    assert_eq!(output.text, expected);
    assert_eq!(output.rules.len(), 2);
}

#[test]
fn preprocesses_nested_optional_list_fixture() {
    let root = fixture_path("nested_optional_list_root.prg");
    let expected = fs::read_to_string(fixture_path("nested_optional_list_root.out")).unwrap();

    let output = Preprocessor::default().preprocess(SourceFile::from_path(&root).unwrap());

    assert!(
        output.errors.is_empty(),
        "unexpected errors: {:?}",
        output.errors
    );
    assert_eq!(output.text, expected);
    assert_eq!(output.rules.len(), 1);
}

#[test]
fn preprocesses_multiline_command_fixture() {
    let root = fixture_path("multiline_command_root.prg");
    let expected = fs::read_to_string(fixture_path("multiline_command_root.out")).unwrap();

    let output = Preprocessor::default().preprocess(SourceFile::from_path(&root).unwrap());

    assert!(
        output.errors.is_empty(),
        "unexpected errors: {:?}",
        output.errors
    );
    assert_eq!(output.text, expected);
    assert_eq!(output.rules.len(), 1);
}

#[test]
fn reports_malformed_rule_fixture() {
    let root = fixture_path("malformed_rule_root.prg");

    let output = Preprocessor::default().preprocess(SourceFile::from_path(&root).unwrap());

    assert_eq!(output.text, "PROCEDURE Main()\n   BAD 1\nRETURN\n");
    assert_eq!(output.errors.len(), 1);
    assert_eq!(
        output.errors[0].message,
        "unterminated rule marker in pattern"
    );
}
