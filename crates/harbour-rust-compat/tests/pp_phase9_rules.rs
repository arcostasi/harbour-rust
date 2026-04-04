use std::{fs, path::PathBuf};

use harbour_rust_pp::{Preprocessor, SourceFile};

fn workspace_fixture(path: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join(path)
}

#[test]
fn phase9_rule_fixture_matches_curated_upstream_subset() {
    let upstream_doc =
        fs::read_to_string(workspace_fixture("harbour-core/doc/pp.txt")).expect("upstream pp doc");
    let upstream_hbpp =
        fs::read_to_string(workspace_fixture("harbour-core/tests/hbpp/_pp_test.prg"))
            .expect("upstream hbpp test");
    let expected = fs::read_to_string(workspace_fixture("tests/fixtures/pp/rule_markers_root.out"))
        .expect("fixture snapshot");

    assert!(upstream_doc.contains("#command A <x> => ? <x>"));
    assert!(upstream_doc.contains("<idMarker,...> list match marker"));
    assert!(upstream_doc.contains("<idMarker:...> restricted match marker"));
    assert!(upstream_doc.contains("#<idMarker> Dumb stringify result marker"));
    assert!(upstream_hbpp.contains("#command ZZZ [<v>] => QOUT( [ <v>\\[1\\] ] )"));
    assert!(upstream_hbpp.contains("#command _DUMB_M(<z>) => dm( #<z> )"));
    assert!(upstream_hbpp.contains("#command MYCOMMAND [<mylist,...>] [MYCLAUSE <myval>] => ;"));

    let output = Preprocessor::default().preprocess(
        SourceFile::from_path(workspace_fixture("tests/fixtures/pp/rule_markers_root.prg"))
            .expect("fixture"),
    );

    assert!(
        output.errors.is_empty(),
        "unexpected errors: {:?}",
        output.errors
    );
    assert_eq!(output.text, expected);
}
