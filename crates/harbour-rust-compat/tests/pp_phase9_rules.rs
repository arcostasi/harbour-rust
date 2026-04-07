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
fn phase14_quoted_result_marker_macro_fixture_matches_curated_upstream_subset() {
    let Some(upstream_doc) = read_upstream_or_skip("harbour-core/doc/pp.txt", "upstream pp doc")
    else {
        return;
    };
    let Some(upstream_hbpptest) = read_upstream_or_skip(
        "harbour-core/tests/hbpp/hbpptest.prg",
        "upstream hbpp runtime test",
    ) else {
        return;
    };
    let expected = fs::read_to_string(workspace_fixture(
        "tests/fixtures/pp/quoted_macro_marker_root.out",
    ))
    .expect("fixture snapshot");

    assert!(upstream_doc.contains("Normal stringify result marker"));
    assert!(upstream_doc.contains("macro tokens expressions starting with '&' followed by '('"));
    assert!(upstream_doc.contains("rest of tokens copied"));
    assert!(upstream_hbpptest.contains("#command MCOMMAND <x> => normal_c(<\"x\">)"));
    assert!(upstream_hbpptest.contains("pre :='normal_c(\"&cVar+1\")'"));
    assert!(upstream_hbpptest.contains("pre :='normal_c((cVar) +1)'"));

    let output = Preprocessor::default().preprocess(
        SourceFile::from_path(workspace_fixture(
            "tests/fixtures/pp/quoted_macro_marker_root.prg",
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

#[test]
fn phase14_smart_result_marker_macro_fixture_matches_curated_upstream_subset() {
    let Some(upstream_doc) = read_upstream_or_skip("harbour-core/doc/pp.txt", "upstream pp doc")
    else {
        return;
    };
    let Some(upstream_hbpptest) = read_upstream_or_skip(
        "harbour-core/tests/hbpp/hbpptest.prg",
        "upstream hbpp runtime test",
    ) else {
        return;
    };
    let expected = fs::read_to_string(workspace_fixture(
        "tests/fixtures/pp/smart_marker_macro_root.out",
    ))
    .expect("fixture snapshot");

    assert!(upstream_doc.contains("Smart stringify result marker"));
    assert!(upstream_doc.contains("rules as for normal stringify with the exception"));
    assert!(upstream_doc.contains("which start with string or '(' token"));
    assert!(upstream_hbpptest.contains("#translate MTRANSLATE <x:&>  => macro_t(<(x)>)"));
    assert!(upstream_hbpptest.contains("pre :='macro_t(cVar)'"));
    assert!(upstream_hbpptest.contains("pre :='macro_t(\"&cVar&cVar\")'"));

    let output = Preprocessor::default().preprocess(
        SourceFile::from_path(workspace_fixture(
            "tests/fixtures/pp/smart_marker_macro_root.prg",
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
fn phase14_macro_pattern_translate_fixture_matches_curated_upstream_subset() {
    let Some(upstream_doc) = read_upstream_or_skip("harbour-core/doc/pp.txt", "upstream pp doc")
    else {
        return;
    };
    let Some(upstream_hbpptest) = read_upstream_or_skip(
        "harbour-core/tests/hbpp/hbpptest.prg",
        "upstream hbpp runtime test",
    ) else {
        return;
    };
    let expected = fs::read_to_string(workspace_fixture(
        "tests/fixtures/pp/macro_pattern_translate_root.out",
    ))
    .expect("fixture snapshot");

    assert!(upstream_doc.contains("special meaning."));
    assert!(upstream_doc.contains("It will match any macro tokens"));
    assert!(upstream_hbpptest.contains("#translate MTRANSLATE <x:&>  => macro_t(<(x)>)"));
    assert!(upstream_hbpptest.contains("pre :='macro_t(cVar)+1'"));
    assert!(upstream_hbpptest.contains("pre :='macro_t(cVar)++'"));
    assert!(upstream_hbpptest.contains("pre :='macro_t(cVar)+=1'"));
    assert!(upstream_hbpptest.contains("pre :='macro_t(\"&cVar&cVar\")'"));
    assert!(upstream_hbpptest.contains("pre :='macro_t(\"&cVar.&cVar.\")'"));
    assert!(upstream_hbpptest.contains("pre :='macro_t(\"&cVar.&cVar.&cVar&cVar\")'"));
    assert!(upstream_hbpptest.contains("pre :='macro_t(\"&cVar.&\")(cVar)'"));

    let output = Preprocessor::default().preprocess(
        SourceFile::from_path(workspace_fixture(
            "tests/fixtures/pp/macro_pattern_translate_root.prg",
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
fn phase14_macro_pattern_command_fixture_matches_curated_upstream_subset() {
    let Some(upstream_hbpptest) = read_upstream_or_skip(
        "harbour-core/tests/hbpp/hbpptest.prg",
        "upstream hbpp runtime test",
    ) else {
        return;
    };
    let expected = fs::read_to_string(workspace_fixture(
        "tests/fixtures/pp/macro_pattern_command_root.out",
    ))
    .expect("fixture snapshot");

    assert!(upstream_hbpptest.contains("#command MCOMMAND <x> => normal_c(<\"x\">)"));
    assert!(upstream_hbpptest.contains("#command MCOMMAND <x:&>  => macro_c(<(x)>)"));
    assert!(upstream_hbpptest.contains("pre :='macro_c(cVar)'"));
    assert!(upstream_hbpptest.contains("pre :='macro_c(\"&cVar.&cVar.\")'"));
    assert!(upstream_hbpptest.contains("pre :='macro_c(\"&cVar.&cVar.&cVar&cVar2\")'"));
    assert!(upstream_hbpptest.contains("pre :='normal_c(\"&cVar++\")'"));
    assert!(upstream_hbpptest.contains("pre :='normal_c(\"&cVar+=1\")'"));
    assert!(upstream_hbpptest.contains("pre :='normal_c(\"&cVar+1\")'"));
    assert!(upstream_hbpptest.contains("pre :='normal_c(\"&cVar.&(cVar)\")'"));

    let output = Preprocessor::default().preprocess(
        SourceFile::from_path(workspace_fixture(
            "tests/fixtures/pp/macro_pattern_command_root.prg",
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
fn phase13_blockify_result_marker_fixture_matches_curated_upstream_subset() {
    let Some(upstream_doc) = read_upstream_or_skip("harbour-core/doc/pp.txt", "upstream pp doc")
    else {
        return;
    };
    let Some(upstream_hbpptest) = read_upstream_or_skip(
        "harbour-core/tests/hbpp/hbpptest.prg",
        "upstream hbpp runtime test",
    ) else {
        return;
    };
    let expected = fs::read_to_string(workspace_fixture(
        "tests/fixtures/pp/blockify_marker_root.out",
    ))
    .expect("fixture snapshot");

    assert!(upstream_doc.contains("<{idMarker}> Blockify result marker"));
    assert!(upstream_doc.contains("adding \"{||\" prefix and \"}\" suffix"));
    assert!(upstream_hbpptest.contains("_GET_( <var>, <\"var\">, <pic>, <{valid}>, <{when}> )"));
    assert!(upstream_hbpptest.contains("dbSetFilter({|| &cVar.},cVar)"));

    let output = Preprocessor::default().preprocess(
        SourceFile::from_path(workspace_fixture(
            "tests/fixtures/pp/blockify_marker_root.prg",
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
fn phase14_nested_optional_list_fixture_matches_curated_upstream_subset() {
    let Some(upstream_hbpptest) = read_upstream_or_skip(
        "harbour-core/tests/hbpp/hbpptest.prg",
        "upstream hbpp runtime test",
    ) else {
        return;
    };
    let expected = fs::read_to_string(workspace_fixture(
        "tests/fixtures/pp/nested_optional_list_root.out",
    ))
    .expect("fixture snapshot");

    assert!(upstream_hbpptest.contains("#xcommand SET <var1> [, <varN>] WITH <val> =>"));
    assert!(upstream_hbpptest.contains("pre := \"v1:=0; v2:=0 ; v3:=0 \""));
    assert!(upstream_hbpptest.contains("#command AVG <x1> [, <xn>] TO <v1> [, <vn>]  =>"));
    assert!(upstream_hbpptest.contains("pre := \"AVERAGE({||s1:=s1+f1}  )\""));
    assert!(upstream_hbpptest.contains("pre := \"AVERAGE({||s1:=s1+f1} ,{||s2:=s2+f2}   )\""));
    assert!(
        upstream_hbpptest
            .contains("pre := \"AVERAGE({||s1:=s1+f1} ,{||s2:=s2+f2}  ,{||s3:=s3+f3}   )\"")
    );

    let output = Preprocessor::default().preprocess(
        SourceFile::from_path(workspace_fixture(
            "tests/fixtures/pp/nested_optional_list_root.prg",
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
fn phase14_nested_optional_match_fixture_matches_curated_upstream_subset() {
    let Some(upstream_hbpptest) = read_upstream_or_skip(
        "harbour-core/tests/hbpp/hbpptest.prg",
        "upstream hbpp runtime test",
    ) else {
        return;
    };
    let expected = fs::read_to_string(workspace_fixture(
        "tests/fixtures/pp/nested_optional_match_root.out",
    ))
    .expect("fixture snapshot");

    assert!(upstream_hbpptest.contains("#xtranslate AAA [A <a> [B <b>] ] => Qout([<a>][, <b>])"));
    assert!(upstream_hbpptest.contains("pre :=\"Qout()\""));
    assert!(upstream_hbpptest.contains("pre :=\"Qout(a )\""));
    assert!(upstream_hbpptest.contains("pre :=\"Qout(a ,b )\""));

    let output = Preprocessor::default().preprocess(
        SourceFile::from_path(workspace_fixture(
            "tests/fixtures/pp/nested_optional_match_root.prg",
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
