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

#[test]
fn phase13_optional_and_stringify_fixture_matches_curated_upstream_subset() {
    let Some(upstream_doc) = read_upstream_or_skip("harbour-core/doc/pp.txt", "upstream pp doc")
    else {
        return;
    };
    let Some(upstream_hbpp) =
        read_upstream_or_skip("harbour-core/tests/hbpp/_pp_test.prg", "upstream hbpp test")
    else {
        return;
    };
    let expected = fs::read_to_string(workspace_fixture(
        "tests/fixtures/pp/optional_stringify_root.out",
    ))
    .expect("fixture snapshot");

    assert!(upstream_doc.contains("#<idMarker> Dumb stringify result marker"));
    assert!(upstream_doc.contains("optional clause in match pattern"));
    assert!(upstream_hbpp.contains("#command ZZZ [<v>] => QOUT( [ <v>\\[1\\] ] )"));
    assert!(upstream_hbpp.contains("#command _DUMB_M(<z>) => dm( #<z> )"));
    assert!(upstream_hbpp.contains("#command MYCOMMAND [<mylist,...>] [MYCLAUSE <myval>] => ;"));

    let output = Preprocessor::default().preprocess(
        SourceFile::from_path(workspace_fixture(
            "tests/fixtures/pp/optional_stringify_root.prg",
        ))
        .expect("fixture"),
    );

    assert!(
        output.errors.is_empty(),
        "unexpected errors: {:?}",
        output.errors
    );
    assert_eq!(output.text, expected);
}

#[test]
fn phase13_logical_result_marker_fixture_matches_curated_upstream_subset() {
    let Some(upstream_hbpptest) = read_upstream_or_skip(
        "harbour-core/tests/hbpp/hbpptest.prg",
        "upstream hbpp runtime test",
    ) else {
        return;
    };
    let expected = fs::read_to_string(workspace_fixture(
        "tests/fixtures/pp/logical_marker_root.out",
    ))
    .expect("fixture snapshot");

    assert!(
        upstream_hbpptest
            .contains("#command MYCOMMAND3 [<mylist,...>] [MYCLAUSE <myval>] [<all:ALL>] =>")
    );
    assert!(upstream_hbpptest.contains("MyFunction( {<mylist>} [, <myval>] [,<.all.>] )"));
    assert!(upstream_hbpptest.contains("pre := 'MyFunction({\"HELLO\"} ,321  ,.T.  )'"));
    assert!(upstream_hbpptest.contains("pre := 'MyFunction({\"HELLO\"} ,321   )'"));

    let output = Preprocessor::default().preprocess(
        SourceFile::from_path(workspace_fixture(
            "tests/fixtures/pp/logical_marker_root.prg",
        ))
        .expect("fixture"),
    );

    assert!(
        output.errors.is_empty(),
        "unexpected errors: {:?}",
        output.errors
    );
    assert_eq!(output.text, expected);
}

#[test]
fn phase13_quoted_result_marker_fixture_matches_curated_upstream_subset() {
    let Some(upstream_hbpptest) = read_upstream_or_skip(
        "harbour-core/tests/hbpp/hbpptest.prg",
        "upstream hbpp runtime test",
    ) else {
        return;
    };
    let expected = fs::read_to_string(workspace_fixture(
        "tests/fixtures/pp/quoted_marker_root.out",
    ))
    .expect("fixture snapshot");

    assert!(upstream_hbpptest.contains("#command _NORMAL_M(<z>) => nm( <\"z\"> )"));
    assert!(upstream_hbpptest.contains("pre :='nm(\"a\" )'"));
    assert!(upstream_hbpptest.contains("pre :='nm(\"a[1]\" )'"));

    let output = Preprocessor::default().preprocess(
        SourceFile::from_path(workspace_fixture(
            "tests/fixtures/pp/quoted_marker_root.prg",
        ))
        .expect("fixture"),
    );

    assert!(
        output.errors.is_empty(),
        "unexpected errors: {:?}",
        output.errors
    );
    assert_eq!(output.text, expected);
}

#[test]
fn phase13_smart_result_marker_fixture_matches_curated_upstream_subset() {
    let Some(upstream_hbpptest) = read_upstream_or_skip(
        "harbour-core/tests/hbpp/hbpptest.prg",
        "upstream hbpp runtime test",
    ) else {
        return;
    };
    let expected = fs::read_to_string(workspace_fixture("tests/fixtures/pp/smart_marker_root.out"))
        .expect("fixture snapshot");

    assert!(upstream_hbpptest.contains("#command _SMART_M(<z>) => sm( <(z)> )"));
    assert!(upstream_hbpptest.contains("pre :='sm(\"a\" )'"));
    assert!(upstream_hbpptest.contains("pre :='sm(\"a[1]\" )'"));

    let output = Preprocessor::default().preprocess(
        SourceFile::from_path(workspace_fixture("tests/fixtures/pp/smart_marker_root.prg"))
            .expect("fixture"),
    );

    assert!(
        output.errors.is_empty(),
        "unexpected errors: {:?}",
        output.errors
    );
    assert_eq!(output.text, expected);
}
