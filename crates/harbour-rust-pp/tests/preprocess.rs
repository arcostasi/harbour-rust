use std::{fs, path::PathBuf};

use harbour_rust_pp::{Preprocessor, SourceFile};

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
