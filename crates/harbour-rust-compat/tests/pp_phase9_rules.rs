use std::fs;

mod support;
use support::{read_upstream_or_skip, workspace_fixture};

use harbour_rust_pp::{Preprocessor, SourceFile};

#[test]
fn phase9_rule_fixture_matches_curated_upstream_subset() {
    let Some(upstream_doc) = read_upstream_or_skip("harbour-core/doc/pp.txt", "upstream pp doc")
    else {
        return;
    };
    let Some(upstream_hbpp) =
        read_upstream_or_skip("harbour-core/tests/hbpp/_pp_test.prg", "upstream hbpp test")
    else {
        return;
    };
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
