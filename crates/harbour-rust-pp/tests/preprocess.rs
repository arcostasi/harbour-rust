use std::{fs, path::PathBuf};

use harbour_rust_pp::{FileSystemIncludeResolver, Preprocessor, SourceFile};

fn normalize_newlines(text: &str) -> String {
    text.replace("\r\n", "\n").replace('\r', "\n")
}

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

    assert_eq!(
        normalize_newlines(&output.text),
        "PROCEDURE Main()\n   ? A\nRETURN\n"
    );
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
    assert_eq!(output.rules.len(), 2);
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
    assert_eq!(output.rules.len(), 3);
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
    assert_eq!(output.rules.len(), 3);
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
    assert_eq!(output.rules.len(), 2);
}

#[test]
fn preprocesses_nested_optional_match_fixture() {
    let root = fixture_path("nested_optional_match_root.prg");
    let expected = fs::read_to_string(fixture_path("nested_optional_match_root.out")).unwrap();

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
fn preprocesses_insert_rule_fixture() {
    let root = fixture_path("insert_rule_root.prg");
    let expected = fs::read_to_string(fixture_path("insert_rule_root.out")).unwrap();

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
fn preprocesses_multiline_result_rule_fixture() {
    let root = fixture_path("multiline_result_rule_root.prg");
    let expected = fs::read_to_string(fixture_path("multiline_result_rule_root.out")).unwrap();

    let output = Preprocessor::default().preprocess(SourceFile::from_path(&root).unwrap());

    assert!(
        output.errors.is_empty(),
        "unexpected errors: {:?}",
        output.errors
    );
    assert_eq!(output.text, expected);
    assert_eq!(output.rules.len(), 3);
}

#[test]
fn preprocesses_multiline_pattern_rule_fixture() {
    let root = fixture_path("multiline_pattern_rule_root.prg");
    let expected = fs::read_to_string(fixture_path("multiline_pattern_rule_root.out")).unwrap();

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
fn preprocesses_xtrans_match_fixture() {
    let root = fixture_path("xtrans_match_root.prg");
    let expected = fs::read_to_string(fixture_path("xtrans_match_root.out")).unwrap();

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
fn preprocesses_xtrans_macro_chain_fixture() {
    let root = fixture_path("xtrans_macro_chain_root.prg");
    let expected = fs::read_to_string(fixture_path("xtrans_macro_chain_root.out")).unwrap();

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
fn preprocesses_xtrans_full_fixture() {
    let root = fixture_path("xtrans_full_root.prg");
    let expected = fs::read_to_string(fixture_path("xtrans_full_root.out")).unwrap();

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
fn preprocesses_macro_call_fixture() {
    let root = fixture_path("macro_call_root.prg");
    let expected = fs::read_to_string(fixture_path("macro_call_root.out")).unwrap();

    let output = Preprocessor::default().preprocess(SourceFile::from_path(&root).unwrap());

    assert!(
        output.errors.is_empty(),
        "unexpected errors: {:?}",
        output.errors
    );
    assert_eq!(output.text, expected);
    assert_eq!(output.rules.len(), 3);
}

#[test]
fn preprocesses_macro_pair_fixture() {
    let root = fixture_path("macro_pair_root.prg");
    let expected = fs::read_to_string(fixture_path("macro_pair_root.out")).unwrap();

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
fn preprocesses_mxcall_post_fixture() {
    let root = fixture_path("mxcall_post_root.prg");
    let expected = fs::read_to_string(fixture_path("mxcall_post_root.out")).unwrap();

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
fn preprocesses_macro_command_operator_fixture() {
    let root = fixture_path("macro_command_operator_root.prg");
    let expected = fs::read_to_string(fixture_path("macro_command_operator_root.out")).unwrap();

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
fn preprocesses_define_window_fixture() {
    let root = fixture_path("define_window_root.prg");
    let expected = fs::read_to_string(fixture_path("define_window_root.out")).unwrap();

    let output = Preprocessor::default().preprocess(SourceFile::from_path(&root).unwrap());

    assert!(
        output.errors.is_empty(),
        "unexpected errors: {:?}",
        output.errors
    );
    assert_eq!(output.text, expected);
    assert_eq!(output.rules.len(), 3);
}

#[test]
fn preprocesses_property_translate_fixture() {
    let root = fixture_path("property_translate_root.prg");
    let expected = fs::read_to_string(fixture_path("property_translate_root.out")).unwrap();

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
fn preprocesses_constructor_translate_fixture() {
    let root = fixture_path("constructor_translate_root.prg");
    let expected = fs::read_to_string(fixture_path("constructor_translate_root.out")).unwrap();

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
fn preprocesses_constructor_identifier_translate_fixture() {
    let root = fixture_path("constructor_identifier_translate_root.prg");
    let expected =
        fs::read_to_string(fixture_path("constructor_identifier_translate_root.out")).unwrap();

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
fn preprocesses_regular_marker_compound_fixture() {
    let root = fixture_path("regular_marker_compound_root.prg");
    let expected = fs::read_to_string(fixture_path("regular_marker_compound_root.out")).unwrap();

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
fn preprocesses_normal_marker_compound_fixture() {
    let root = fixture_path("normal_marker_compound_root.prg");
    let expected = fs::read_to_string(fixture_path("normal_marker_compound_root.out")).unwrap();

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
fn preprocesses_smart_marker_compound_fixture() {
    let root = fixture_path("smart_marker_compound_root.prg");
    let expected = fs::read_to_string(fixture_path("smart_marker_compound_root.out")).unwrap();

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
fn preprocesses_dumb_marker_compound_fixture() {
    let root = fixture_path("dumb_marker_compound_root.prg");
    let expected = fs::read_to_string(fixture_path("dumb_marker_compound_root.out")).unwrap();

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
fn preprocesses_regular_list_compound_fixture() {
    let root = fixture_path("regular_list_compound_root.prg");
    let expected = fs::read_to_string(fixture_path("regular_list_compound_root.out")).unwrap();

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
fn preprocesses_normal_list_compound_fixture() {
    let root = fixture_path("normal_list_compound_root.prg");
    let expected = fs::read_to_string(fixture_path("normal_list_compound_root.out")).unwrap();

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
fn preprocesses_smart_list_compound_fixture() {
    let root = fixture_path("smart_list_compound_root.prg");
    let expected = fs::read_to_string(fixture_path("smart_list_compound_root.out")).unwrap();

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
fn preprocesses_dumb_list_compound_fixture() {
    let root = fixture_path("dumb_list_compound_root.prg");
    let expected = fs::read_to_string(fixture_path("dumb_list_compound_root.out")).unwrap();

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
fn preprocesses_index_preserve_spaces_fixture() {
    let root = fixture_path("index_preserve_spaces_root.prg");
    let expected = fs::read_to_string(fixture_path("index_preserve_spaces_root.out")).unwrap();

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
fn preprocesses_function_like_define_case_fixture() {
    let root = fixture_path("function_like_define_case_root.prg");
    let expected = fs::read_to_string(fixture_path("function_like_define_case_root.out")).unwrap();

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
        vec!["F1", "F3"]
    );
}

#[test]
fn preprocesses_nested_function_like_define_fixture() {
    let root = fixture_path("nested_function_like_define_root.prg");
    let expected =
        fs::read_to_string(fixture_path("nested_function_like_define_root.out")).unwrap();

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
        vec!["DATENEW", "DATEOLD", "datediff"]
    );
}

#[test]
fn preprocesses_constructor_wrapper_function_like_define_fixture() {
    let root = fixture_path("constructor_wrapper_function_like_define_root.prg");
    let expected = fs::read_to_string(fixture_path(
        "constructor_wrapper_function_like_define_root.out",
    ))
    .unwrap();

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
        vec!["clas"]
    );
}

#[test]
fn preprocesses_tooltip_command_fixture() {
    let root = fixture_path("tooltip_command_root.prg");
    let expected = fs::read_to_string(fixture_path("tooltip_command_root.out")).unwrap();

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
fn preprocesses_zzz_escape_fixture() {
    let root = fixture_path("zzz_escape_root.prg");
    let expected = fs::read_to_string(fixture_path("zzz_escape_root.out")).unwrap();

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
fn preprocesses_hmg_escape_translate_fixture() {
    let root = fixture_path("hmg_escape_translate_root.prg");
    let expected = fs::read_to_string(fixture_path("hmg_escape_translate_root.out")).unwrap();

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
fn preprocesses_set_filter_macro_fixture() {
    let root = fixture_path("set_filter_macro_root.prg");
    let expected = fs::read_to_string(fixture_path("set_filter_macro_root.out")).unwrap();

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
fn preprocesses_copy_structure_extended_fixture() {
    let root = fixture_path("copy_structure_extended_root.prg");
    let expected = fs::read_to_string(fixture_path("copy_structure_extended_root.out")).unwrap();

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
fn preprocesses_get_command_base_fixture() {
    let root = fixture_path("get_command_base_root.prg");
    let expected = fs::read_to_string(fixture_path("get_command_base_root.out")).unwrap();

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
fn preprocesses_get_command_picture_fixture() {
    let root = fixture_path("get_command_picture_root.prg");
    let expected = fs::read_to_string(fixture_path("get_command_picture_root.out")).unwrap();

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
fn preprocesses_get_command_valid_fixture() {
    let root = fixture_path("get_command_valid_root.prg");
    let expected = fs::read_to_string(fixture_path("get_command_valid_root.out")).unwrap();

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
fn preprocesses_get_command_when_fixture() {
    let root = fixture_path("get_command_when_root.prg");
    let expected = fs::read_to_string(fixture_path("get_command_when_root.out")).unwrap();

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
fn preprocesses_get_command_caption_fixture() {
    let root = fixture_path("get_command_caption_root.prg");
    let expected = fs::read_to_string(fixture_path("get_command_caption_root.out")).unwrap();

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
fn preprocesses_get_command_message_fixture() {
    let root = fixture_path("get_command_message_root.prg");
    let expected = fs::read_to_string(fixture_path("get_command_message_root.out")).unwrap();

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
fn preprocesses_get_command_send_fixture() {
    let root = fixture_path("get_command_send_root.prg");
    let expected = fs::read_to_string(fixture_path("get_command_send_root.out")).unwrap();

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
fn preprocesses_get_command_range_fixture() {
    let root = fixture_path("get_command_range_root.prg");
    let expected = fs::read_to_string(fixture_path("get_command_range_root.out")).unwrap();

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
fn preprocesses_get_command_picture_range_fixture() {
    let root = fixture_path("get_command_picture_range_root.prg");
    let expected = fs::read_to_string(fixture_path("get_command_picture_range_root.out")).unwrap();

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
fn preprocesses_get_command_range_picture_reordered_fixture() {
    let root = fixture_path("get_command_range_picture_reordered_root.prg");
    let expected =
        fs::read_to_string(fixture_path("get_command_range_picture_reordered_root.out")).unwrap();

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
fn preprocesses_get_command_valid_range_fixture() {
    let root = fixture_path("get_command_valid_range_root.prg");
    let expected = fs::read_to_string(fixture_path("get_command_valid_range_root.out")).unwrap();

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
fn preprocesses_get_command_when_range_fixture() {
    let root = fixture_path("get_command_when_range_root.prg");
    let expected = fs::read_to_string(fixture_path("get_command_when_range_root.out")).unwrap();

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
fn preprocesses_get_command_picture_range_when_reordered_fixture() {
    let root = fixture_path("get_command_picture_range_when_reordered_root.prg");
    let expected = fs::read_to_string(fixture_path(
        "get_command_picture_range_when_reordered_root.out",
    ))
    .unwrap();

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
fn preprocesses_get_command_picture_range_when_caption_reordered_fixture() {
    let root = fixture_path("get_command_picture_range_when_caption_reordered_root.prg");
    let expected = fs::read_to_string(fixture_path(
        "get_command_picture_range_when_caption_reordered_root.out",
    ))
    .unwrap();

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
fn preprocesses_get_command_picture_range_when_caption_message_reordered_fixture() {
    let root = fixture_path("get_command_picture_range_when_caption_message_reordered_root.prg");
    let expected = fs::read_to_string(fixture_path(
        "get_command_picture_range_when_caption_message_reordered_root.out",
    ))
    .unwrap();

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
fn preprocesses_get_command_picture_range_when_caption_message_send_reordered_fixture() {
    let root =
        fixture_path("get_command_picture_range_when_caption_message_send_reordered_root.prg");
    let expected = fs::read_to_string(fixture_path(
        "get_command_picture_range_when_caption_message_send_reordered_root.out",
    ))
    .unwrap();

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
fn preprocesses_get_command_pushbutton_base_fixture() {
    let root = fixture_path("get_command_pushbutton_base_root.prg");
    let expected =
        fs::read_to_string(fixture_path("get_command_pushbutton_base_root.out")).unwrap();

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
fn preprocesses_get_command_pushbutton_valid_fixture() {
    let root = fixture_path("get_command_pushbutton_valid_root.prg");
    let expected =
        fs::read_to_string(fixture_path("get_command_pushbutton_valid_root.out")).unwrap();

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
fn preprocesses_get_command_pushbutton_when_fixture() {
    let root = fixture_path("get_command_pushbutton_when_root.prg");
    let expected =
        fs::read_to_string(fixture_path("get_command_pushbutton_when_root.out")).unwrap();

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
fn preprocesses_get_command_pushbutton_caption_fixture() {
    let root = fixture_path("get_command_pushbutton_caption_root.prg");
    let expected =
        fs::read_to_string(fixture_path("get_command_pushbutton_caption_root.out")).unwrap();

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
fn preprocesses_get_command_pushbutton_message_fixture() {
    let root = fixture_path("get_command_pushbutton_message_root.prg");
    let expected =
        fs::read_to_string(fixture_path("get_command_pushbutton_message_root.out")).unwrap();

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
fn preprocesses_get_command_pushbutton_color_fixture() {
    let root = fixture_path("get_command_pushbutton_color_root.prg");
    let expected =
        fs::read_to_string(fixture_path("get_command_pushbutton_color_root.out")).unwrap();

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
fn preprocesses_get_command_pushbutton_focus_fixture() {
    let root = fixture_path("get_command_pushbutton_focus_root.prg");
    let expected =
        fs::read_to_string(fixture_path("get_command_pushbutton_focus_root.out")).unwrap();

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
fn preprocesses_get_command_pushbutton_state_fixture() {
    let root = fixture_path("get_command_pushbutton_state_root.prg");
    let expected =
        fs::read_to_string(fixture_path("get_command_pushbutton_state_root.out")).unwrap();

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
fn preprocesses_get_command_pushbutton_style_fixture() {
    let root = fixture_path("get_command_pushbutton_style_root.prg");
    let expected =
        fs::read_to_string(fixture_path("get_command_pushbutton_style_root.out")).unwrap();

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
fn preprocesses_get_command_pushbutton_send_fixture() {
    let root = fixture_path("get_command_pushbutton_send_root.prg");
    let expected =
        fs::read_to_string(fixture_path("get_command_pushbutton_send_root.out")).unwrap();

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
fn preprocesses_get_command_pushbutton_guisend_fixture() {
    let root = fixture_path("get_command_pushbutton_guisend_root.prg");
    let expected =
        fs::read_to_string(fixture_path("get_command_pushbutton_guisend_root.out")).unwrap();

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
fn preprocesses_get_command_pushbutton_size_fixture() {
    let root = fixture_path("get_command_pushbutton_size_root.prg");
    let expected =
        fs::read_to_string(fixture_path("get_command_pushbutton_size_root.out")).unwrap();

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
fn preprocesses_get_command_pushbutton_capoff_fixture() {
    let root = fixture_path("get_command_pushbutton_capoff_root.prg");
    let expected =
        fs::read_to_string(fixture_path("get_command_pushbutton_capoff_root.out")).unwrap();

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
fn preprocesses_get_command_pushbutton_bitmap_fixture() {
    let root = fixture_path("get_command_pushbutton_bitmap_root.prg");
    let expected =
        fs::read_to_string(fixture_path("get_command_pushbutton_bitmap_root.out")).unwrap();

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
fn preprocesses_get_command_pushbutton_bmpoff_fixture() {
    let root = fixture_path("get_command_pushbutton_bmpoff_root.prg");
    let expected =
        fs::read_to_string(fixture_path("get_command_pushbutton_bmpoff_root.out")).unwrap();

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
fn preprocesses_get_command_pushbutton_color_only_fixture() {
    let root = fixture_path("get_command_pushbutton_color_only_root.prg");
    let expected =
        fs::read_to_string(fixture_path("get_command_pushbutton_color_only_root.out")).unwrap();

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
fn preprocesses_get_command_pushbutton_reordered_sparse_fixture() {
    let root = fixture_path("get_command_pushbutton_reordered_sparse_root.prg");
    let expected = fs::read_to_string(fixture_path(
        "get_command_pushbutton_reordered_sparse_root.out",
    ))
    .unwrap();

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
fn preprocesses_get_command_pushbutton_reordered_sparse_color_tail_fixture() {
    let root = fixture_path("get_command_pushbutton_reordered_sparse_color_tail_root.prg");
    let expected = fs::read_to_string(fixture_path(
        "get_command_pushbutton_reordered_sparse_color_tail_root.out",
    ))
    .unwrap();

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
fn preprocesses_get_command_pushbutton_reordered_full_fixture() {
    let root = fixture_path("get_command_pushbutton_reordered_full_root.prg");
    let expected = fs::read_to_string(fixture_path(
        "get_command_pushbutton_reordered_full_root.out",
    ))
    .unwrap();

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
fn preprocesses_define_clipboard_fixture() {
    let root = fixture_path("define_clipboard_root.prg");
    let expected = fs::read_to_string(fixture_path("define_clipboard_root.out")).unwrap();

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
fn preprocesses_define_clipboard_oemtext_fixture() {
    let root = fixture_path("define_clipboard_oemtext_root.prg");
    let expected = fs::read_to_string(fixture_path("define_clipboard_oemtext_root.out")).unwrap();

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
fn preprocesses_release_all_fixture() {
    let root = fixture_path("release_all_root.prg");
    let expected = fs::read_to_string(fixture_path("release_all_root.out")).unwrap();

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
fn preprocesses_release_all_like_fixture() {
    let root = fixture_path("release_all_like_root.prg");
    let expected = fs::read_to_string(fixture_path("release_all_like_root.out")).unwrap();

    let output = Preprocessor::default().preprocess(SourceFile::from_path(&root).unwrap());

    assert!(
        output.errors.is_empty(),
        "unexpected errors: {:?}",
        output.errors
    );
    assert_eq!(output.text, expected);
    assert_eq!(output.rules.len(), 3);
}

#[test]
fn preprocesses_release_all_except_fixture() {
    let root = fixture_path("release_all_except_root.prg");
    let expected = fs::read_to_string(fixture_path("release_all_except_root.out")).unwrap();

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
fn preprocesses_save_all_like_fixture() {
    let root = fixture_path("save_all_like_root.prg");
    let expected = fs::read_to_string(fixture_path("save_all_like_root.out")).unwrap();

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
fn preprocesses_save_to_all_like_fixture() {
    let root = fixture_path("save_to_all_like_root.prg");
    let expected = fs::read_to_string(fixture_path("save_to_all_like_root.out")).unwrap();

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
fn preprocesses_save_to_all_fixture() {
    let root = fixture_path("save_to_all_root.prg");
    let expected = fs::read_to_string(fixture_path("save_to_all_root.out")).unwrap();

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
fn preprocesses_save_all_except_fixture() {
    let root = fixture_path("save_all_except_root.prg");
    let expected = fs::read_to_string(fixture_path("save_all_except_root.out")).unwrap();

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
fn preprocesses_save_to_all_except_fixture() {
    let root = fixture_path("save_to_all_except_root.prg");
    let expected = fs::read_to_string(fixture_path("save_to_all_except_root.out")).unwrap();

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
fn preprocesses_list_base_fixture() {
    let root = fixture_path("list_base_root.prg");
    let expected = fs::read_to_string(fixture_path("list_base_root.out")).unwrap();

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
fn preprocesses_list_destination_fixture() {
    let root = fixture_path("list_destination_root.prg");
    let expected = fs::read_to_string(fixture_path("list_destination_root.out")).unwrap();

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
fn preprocesses_list_off_fixture() {
    let root = fixture_path("list_off_root.prg");
    let expected = fs::read_to_string(fixture_path("list_off_root.out")).unwrap();

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
fn preprocesses_list_fields_fixture() {
    let root = fixture_path("list_fields_root.prg");
    let expected = fs::read_to_string(fixture_path("list_fields_root.out")).unwrap();

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
fn preprocesses_list_fields_destination_fixture() {
    let root = fixture_path("list_fields_destination_root.prg");
    let expected = fs::read_to_string(fixture_path("list_fields_destination_root.out")).unwrap();

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
fn preprocesses_list_fields_off_fixture() {
    let root = fixture_path("list_fields_off_root.prg");
    let expected = fs::read_to_string(fixture_path("list_fields_off_root.out")).unwrap();

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
fn preprocesses_get_command_caption_range_fixture() {
    let root = fixture_path("get_command_caption_range_root.prg");
    let expected = fs::read_to_string(fixture_path("get_command_caption_range_root.out")).unwrap();

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
fn preprocesses_get_command_message_range_fixture() {
    let root = fixture_path("get_command_message_range_root.prg");
    let expected = fs::read_to_string(fixture_path("get_command_message_range_root.out")).unwrap();

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
fn preprocesses_get_command_send_range_fixture() {
    let root = fixture_path("get_command_send_range_root.prg");
    let expected = fs::read_to_string(fixture_path("get_command_send_range_root.out")).unwrap();

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
fn preprocesses_multiline_nested_optional_list_fixture() {
    let root = fixture_path("multiline_nested_optional_list_root.prg");
    let expected =
        fs::read_to_string(fixture_path("multiline_nested_optional_list_root.out")).unwrap();

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
fn preprocesses_optional_reorder_fixture() {
    let root = fixture_path("optional_reorder_root.prg");
    let expected = fs::read_to_string(fixture_path("optional_reorder_root.out")).unwrap();

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

    assert_eq!(
        normalize_newlines(&output.text),
        "PROCEDURE Main()\n   BAD 1\nRETURN\n"
    );
    assert_eq!(output.errors.len(), 1);
    assert_eq!(
        output.errors[0].message,
        "unterminated rule marker in pattern"
    );
}
