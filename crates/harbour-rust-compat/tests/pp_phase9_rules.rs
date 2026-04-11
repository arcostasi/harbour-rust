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

#[test]
fn phase15_insert_fixture_matches_curated_upstream_subset() {
    let Some(upstream_hbpp) =
        read_upstream_or_skip("harbour-core/tests/hbpp/_pp_test.prg", "upstream hbpp test")
    else {
        return;
    };
    let expected = fs::read_to_string(workspace_fixture("tests/fixtures/pp/insert_rule_root.out"))
        .expect("fixture snapshot");

    assert!(upstream_hbpp.contains("#xcommand INSERT INTO <table> ( <uField1> [, <uFieldN> ] ) VALUES ( <uVal1> [, <uValN> ] ) => ;"));
    assert!(upstream_hbpp.contains("replace <table>-><uField1> with <uVal1> ;;"));
    assert!(upstream_hbpp.contains("#xcommand INSERT2 INTO <table> ( <uField1> [, <uFieldN> ] ) VALUES ( <uVal1> [, <uValN> ] ) => ;"));
    assert!(upstream_hbpp.contains("insert2 into test ( FIRST, LAST, STREET ) ;"));
    assert!(upstream_hbpp.contains("values ( \"first\", \"last\", \"street\" )"));

    let output = Preprocessor::default().preprocess(
        SourceFile::from_path(workspace_fixture("tests/fixtures/pp/insert_rule_root.prg"))
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
fn phase15_multiline_result_fixture_matches_curated_upstream_subset() {
    let Some(upstream_hbpptest) = read_upstream_or_skip(
        "harbour-core/tests/hbpp/hbpptest.prg",
        "upstream hbpp runtime test",
    ) else {
        return;
    };
    let expected = fs::read_to_string(workspace_fixture(
        "tests/fixtures/pp/multiline_result_rule_root.out",
    ))
    .expect("fixture snapshot");

    assert!(upstream_hbpptest.contains("#xcommand INSERT2 INTO <table> ( <uField1> [, <uFieldN> ] ) VALUES ( <uVal1> [, <uValN> ] ) =>"));
    assert!(
        upstream_hbpptest
            .contains("#command MYCOMMAND2 [<mylist,...>] [MYCLAUSE <myval>] [ALL] =>")
    );
    assert!(
        upstream_hbpptest
            .contains("#command MYCOMMAND3 [<mylist,...>] [MYCLAUSE <myval>] [<all:ALL>] =>")
    );
    assert!(upstream_hbpptest.contains("in := 'MYCOMMAND2 MYCLAUSE 321 ALL \"HELLO\"'"));
    assert!(upstream_hbpptest.contains("pre := 'MyFunction({\"HELLO\"} ,321  )'"));
    assert!(upstream_hbpptest.contains("pre := 'MyFunction({\"HELLO\",\"WORLD\"} ,321  ,.T.  )'"));

    let output = Preprocessor::default().preprocess(
        SourceFile::from_path(workspace_fixture(
            "tests/fixtures/pp/multiline_result_rule_root.prg",
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
fn phase15_multiline_pattern_fixture_matches_curated_upstream_subset() {
    let Some(upstream_hbpptest) = read_upstream_or_skip(
        "harbour-core/tests/hbpp/hbpptest.prg",
        "upstream hbpp runtime test",
    ) else {
        return;
    };
    let expected = fs::read_to_string(workspace_fixture(
        "tests/fixtures/pp/multiline_pattern_rule_root.out",
    ))
    .expect("fixture snapshot");

    assert!(upstream_hbpptest.contains("#command MYCOMMAND2 [<myList,...>]"));
    assert!(upstream_hbpptest.contains(
        "[MYCLAUSE <myVal>] [MYOTHER <myOther>] => MyFunction( {<myList>}, <myVal>, <myOther> )"
    ));

    let output = Preprocessor::default().preprocess(
        SourceFile::from_path(workspace_fixture(
            "tests/fixtures/pp/multiline_pattern_rule_root.prg",
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
fn phase15_xtrans_match_fixture_matches_curated_upstream_subset() {
    let Some(upstream_hbpp) =
        read_upstream_or_skip("harbour-core/tests/hbpp/_pp_test.prg", "upstream hbpp test")
    else {
        return;
    };
    let Some(upstream_hbpptest) = read_upstream_or_skip(
        "harbour-core/tests/hbpp/hbpptest.prg",
        "upstream hbpp runtime test",
    ) else {
        return;
    };
    let expected = fs::read_to_string(workspace_fixture("tests/fixtures/pp/xtrans_match_root.out"))
        .expect("fixture snapshot");

    assert!(upstream_hbpp.contains("#xtranslate XTRANS(<x>( => normal_match( <(x)> )"));
    assert!(upstream_hbpp.contains("#xtranslate XTRANS(<x:&>( => macro_match( <(x)> )"));
    assert!(upstream_hbpptest.contains("#xtranslate XTRANS(<x>( => normal( <(x)> )"));
    assert!(upstream_hbpptest.contains("#xtranslate XTRANS(<x:&>( => macro( <(x)> )"));
    assert!(upstream_hbpptest.contains("pre := 'normal(\"cVar\" )'"));
    assert!(upstream_hbpptest.contains("pre := 'macro(cVar )'"));
    assert!(upstream_hbpptest.contains("pre := 'normal(\"&cVar+1\" )'"));
    assert!(upstream_hbpptest.contains("pre := 'macro(cVar )'"));
    assert!(upstream_hbpptest.contains("pre := 'XTRANS( (&cVar.) ('"));
    assert!(upstream_hbpptest.contains("pre := 'macro((cVar) )'"));
    assert!(upstream_hbpptest.contains("pre := 'normal(\"&cVar[3]\" )'"));
    assert!(upstream_hbpptest.contains("pre := 'normal(\"&cVar.  [3]\" )'"));
    assert!(upstream_hbpptest.contains("pre := 'macro((cVar  [3],&cvar) )'"));
    assert!(upstream_hbpptest.contains("pre := 'XTRANS( (&cVar.  [3],&cvar) ('"));
    assert!(upstream_hbpptest.contains("pre := 'normal(\"&cVar.1+5\" )'"));
    assert!(upstream_hbpptest.contains("pre := 'normal(\"&cVar .AND. cVar\" )'"));
    assert!(upstream_hbpptest.contains("pre := 'normal(\"&cVar. .AND. cVar\" )'"));

    let output = Preprocessor::default().preprocess(
        SourceFile::from_path(workspace_fixture("tests/fixtures/pp/xtrans_match_root.prg"))
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
fn phase15_xtrans_macro_chain_fixture_matches_curated_upstream_subset() {
    let Some(upstream_hbpp) =
        read_upstream_or_skip("harbour-core/tests/hbpp/_pp_test.prg", "upstream hbpp test")
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
        "tests/fixtures/pp/xtrans_macro_chain_root.out",
    ))
    .expect("fixture snapshot");

    assert!(upstream_hbpp.contains("XTRANS( &cVar&cVar ("));
    assert!(upstream_hbpp.contains("XTRANS( &cVar.&cVar ("));
    assert!(upstream_hbpp.contains("XTRANS( &cVar.&cVar. ("));
    assert!(upstream_hbpptest.contains("pre :='macro_t(\"&cVar&cVar\")'"));
    assert!(upstream_hbpptest.contains("pre :='macro_t(\"&cVar.&cVar.\")'"));

    let output = Preprocessor::default().preprocess(
        SourceFile::from_path(workspace_fixture(
            "tests/fixtures/pp/xtrans_macro_chain_root.prg",
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
fn phase15_xtrans_full_fixture_matches_upstream_pp_test_block() {
    let Some(upstream_hbpp) =
        read_upstream_or_skip("harbour-core/tests/hbpp/_pp_test.prg", "upstream hbpp test")
    else {
        return;
    };
    let expected = fs::read_to_string(workspace_fixture("tests/fixtures/pp/xtrans_full_root.out"))
        .expect("fixture snapshot");

    assert!(upstream_hbpp.contains("XTRANS( cVar ("));
    assert!(upstream_hbpp.contains("XTRANS( &cVar ("));
    assert!(upstream_hbpp.contains("XTRANS( &cVar+1 ("));
    assert!(upstream_hbpp.contains("XTRANS( &cVar. ("));
    assert!(upstream_hbpp.contains("XTRANS( &cVar&cVar ("));
    assert!(upstream_hbpp.contains("XTRANS( &cVar.&cVar ("));
    assert!(upstream_hbpp.contains("XTRANS( &cVar.&cVar. ("));
    assert!(upstream_hbpp.contains("XTRANS( (&cVar.) ("));
    assert!(upstream_hbpp.contains("XTRANS( &(cVar) ("));
    assert!(upstream_hbpp.contains("XTRANS( &cVar[3] ("));
    assert!(upstream_hbpp.contains("XTRANS( &cVar.  [3] ("));
    assert!(upstream_hbpp.contains("XTRANS( &(cVar  [3],&cvar) ("));
    assert!(upstream_hbpp.contains("XTRANS( (&cVar.  [3],&cvar) ("));
    assert!(upstream_hbpp.contains("XTRANS( &cVar.1+5 ("));
    assert!(upstream_hbpp.contains("XTRANS( &cVar .AND. cVar ("));
    assert!(upstream_hbpp.contains("XTRANS( &cVar. .AND. cVar ("));

    let output = Preprocessor::default().preprocess(
        SourceFile::from_path(workspace_fixture("tests/fixtures/pp/xtrans_full_root.prg"))
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
fn phase15_macro_call_fixture_matches_curated_upstream_subset() {
    let Some(upstream_hbpptest) = read_upstream_or_skip(
        "harbour-core/tests/hbpp/hbpptest.prg",
        "upstream hbpp runtime test",
    ) else {
        return;
    };
    let expected = fs::read_to_string(workspace_fixture("tests/fixtures/pp/macro_call_root.out"))
        .expect("fixture snapshot");

    assert!(upstream_hbpptest.contains("in := \"MXCALL &cVar\""));
    assert!(upstream_hbpptest.contains("pre := '(&cVar)'"));
    assert!(upstream_hbpptest.contains("in := \"MXCALL &cVar++\""));
    assert!(upstream_hbpptest.contains("pre := '(&cVar)++'"));
    assert!(upstream_hbpptest.contains("in := \"MYCALL &cVar &cVar\""));
    assert!(upstream_hbpptest.contains("pre := '&cVar(&cVar,\"mycall\" )'"));
    assert!(upstream_hbpptest.contains("in := \"MYCALL &cVar+1 &cVar\""));
    assert!(upstream_hbpptest.contains("pre := '&cVar(+1,\"mycall\" ) &cVar'"));
    assert!(upstream_hbpptest.contains("in := \"MZCALL &cVar ++cVar\""));
    assert!(upstream_hbpptest.contains("pre := '&cVar ++(cVar,\"mzcall\" )'"));
    assert!(upstream_hbpptest.contains("in := \"MZCALL &cVar+1 &cVar\""));
    assert!(upstream_hbpptest.contains("pre := '&cVar+1(&cVar,\"mzcall\" )'"));

    let output = Preprocessor::default().preprocess(
        SourceFile::from_path(workspace_fixture("tests/fixtures/pp/macro_call_root.prg"))
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
fn phase15_macro_pair_fixture_matches_curated_upstream_subset() {
    let Some(upstream_hbpptest) = read_upstream_or_skip(
        "harbour-core/tests/hbpp/hbpptest.prg",
        "upstream hbpp runtime test",
    ) else {
        return;
    };
    let expected = fs::read_to_string(workspace_fixture("tests/fixtures/pp/macro_pair_root.out"))
        .expect("fixture snapshot");

    assert!(upstream_hbpptest.contains("in := \"FOO &cVar FOO &var.\""));
    assert!(upstream_hbpptest.contains("pre := 'cVar+var'"));
    assert!(upstream_hbpptest.contains("in := \"BAR &cVar BAR &var.\""));
    assert!(upstream_hbpptest.contains("in := \"FOO &cVar FOO &var.+1\""));
    assert!(upstream_hbpptest.contains("pre := 'FOO &cVar FOO &var.+1'"));
    assert!(upstream_hbpptest.contains("in := \"BAR &cVar BAR &var.+1\""));
    assert!(upstream_hbpptest.contains("pre := 'cVar+var+1'"));

    let output = Preprocessor::default().preprocess(
        SourceFile::from_path(workspace_fixture("tests/fixtures/pp/macro_pair_root.prg"))
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
fn phase15_mxcall_post_fixture_matches_curated_upstream_subset() {
    let Some(upstream_hbpptest) = read_upstream_or_skip(
        "harbour-core/tests/hbpp/hbpptest.prg",
        "upstream hbpp runtime test",
    ) else {
        return;
    };
    let expected = fs::read_to_string(workspace_fixture("tests/fixtures/pp/mxcall_post_root.out"))
        .expect("fixture snapshot");

    assert!(upstream_hbpptest.contains("in := \"MXCALL &cVar()\""));
    assert!(upstream_hbpptest.contains("pre := '(&cVar)()'"));
    assert!(upstream_hbpptest.contains("in := \"MXCALL &cVar++\""));
    assert!(upstream_hbpptest.contains("pre := '(&cVar)++'"));
    assert!(upstream_hbpptest.contains("in := \"(MXCALL &cVar)++\""));
    assert!(upstream_hbpptest.contains("pre := '((&cVar))++'"));
    assert!(upstream_hbpptest.contains("in := \"MXCALL &cVar.()\""));
    assert!(upstream_hbpptest.contains("pre := '(&cVar.)()'"));
    assert!(upstream_hbpptest.contains("in := \"MXCALL &cVar.++\""));
    assert!(upstream_hbpptest.contains("pre := '(&cVar.)++'"));
    assert!(upstream_hbpptest.contains("in := \"(MXCALL &cVar.)++\""));
    assert!(upstream_hbpptest.contains("pre := '((&cVar.))++'"));
    assert!(upstream_hbpptest.contains("in := \"MXCALL &cVar.1 ()\""));
    assert!(upstream_hbpptest.contains("pre := '(&cVar.1) ()'"));
    assert!(upstream_hbpptest.contains("in := \"MXCALL &cVar.1 ++\""));
    assert!(upstream_hbpptest.contains("pre := '(&cVar.1) ++'"));
    assert!(upstream_hbpptest.contains("in := \"(MXCALL &cVar.1) ++\""));
    assert!(upstream_hbpptest.contains("pre := '((&cVar.1)) ++'"));

    let output = Preprocessor::default().preprocess(
        SourceFile::from_path(workspace_fixture("tests/fixtures/pp/mxcall_post_root.prg"))
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
fn phase15_macro_command_operator_fixture_matches_curated_upstream_subset() {
    let Some(upstream_hbpptest) = read_upstream_or_skip(
        "harbour-core/tests/hbpp/hbpptest.prg",
        "upstream hbpp runtime test",
    ) else {
        return;
    };
    let expected = fs::read_to_string(workspace_fixture(
        "tests/fixtures/pp/macro_command_operator_root.out",
    ))
    .expect("fixture snapshot");

    assert!(upstream_hbpptest.contains("in := \"MCOMMAND &cVar.+1\""));
    assert!(upstream_hbpptest.contains("pre :='normal_c(\"&cVar.+1\")'"));
    assert!(upstream_hbpptest.contains("in := \"MCOMMAND &cVar. .AND.  .T.\""));
    assert!(upstream_hbpptest.contains("pre :='normal_c(\"&cVar. .AND.  .T.\")'"));
    assert!(upstream_hbpptest.contains("in := \"MCOMMAND &cVar.++\""));
    assert!(upstream_hbpptest.contains("pre :='normal_c(\"&cVar.++\")'"));
    assert!(upstream_hbpptest.contains("in := \"MCOMMAND &cVar.-=2\""));
    assert!(upstream_hbpptest.contains("pre :='normal_c(\"&cVar.-=2\")'"));
    assert!(upstream_hbpptest.contains("in := \"MCOMMAND &cVar .AND.  .T.\""));
    assert!(upstream_hbpptest.contains("pre :='normal_c(\"&cVar .AND.  .T.\")'"));
    assert!(upstream_hbpptest.contains("in := \"MCOMMAND & (cVar) +1\""));
    assert!(upstream_hbpptest.contains("pre :='normal_c( (cVar) +1)'"));

    let output = Preprocessor::default().preprocess(
        SourceFile::from_path(workspace_fixture(
            "tests/fixtures/pp/macro_command_operator_root.prg",
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
fn phase15_define_window_fixture_matches_curated_upstream_subset() {
    let Some(upstream_pp_test) =
        read_upstream_or_skip("harbour-core/tests/hbpp/_pp_test.prg", "upstream hbpp test")
    else {
        return;
    };
    let expected = fs::read_to_string(workspace_fixture(
        "tests/fixtures/pp/define_window_root.out",
    ))
    .expect("fixture snapshot");

    assert!(upstream_pp_test.contains("#xcommand DECLARE WINDOW <w> ;"));
    assert!(upstream_pp_test.contains(
        "#xtranslate <w> . <p:Name,Title,f1,f2,f3,f4,f5,f6,f7,f8,f9> := <n> => SProp( <\"w\">, <\"p\"> , <n> )"
    ));
    assert!(upstream_pp_test.contains("#xcommand DEFINE WINDOW <w> [ON INIT <IProc>] =>;"));
    assert!(upstream_pp_test.contains("DEFINE WINDOW &oW"));
    assert!(upstream_pp_test.contains("DEFINE WINDOW &oW ON INIT &oW.Title:= \"My title\""));
    assert!(upstream_pp_test.contains("&oW.Title := \"title\""));
    assert!(upstream_pp_test.contains("&oW.f9 := 9"));

    let output = Preprocessor::default().preprocess(
        SourceFile::from_path(workspace_fixture(
            "tests/fixtures/pp/define_window_root.prg",
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
fn phase15_property_translate_fixture_matches_curated_rule_subset() {
    let Some(upstream_pp_test) =
        read_upstream_or_skip("harbour-core/tests/hbpp/_pp_test.prg", "upstream hbpp test")
    else {
        return;
    };
    let expected = fs::read_to_string(workspace_fixture(
        "tests/fixtures/pp/property_translate_root.out",
    ))
    .expect("fixture snapshot");

    assert!(upstream_pp_test.contains(
        "#xtranslate <w> . <p:Name,Title,f1,f2,f3,f4,f5,f6,f7,f8,f9> := <n> => SProp( <\"w\">, <\"p\"> , <n> )"
    ));

    let output = Preprocessor::default().preprocess(
        SourceFile::from_path(workspace_fixture(
            "tests/fixtures/pp/property_translate_root.prg",
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
fn phase15_constructor_translate_fixture_matches_curated_upstream_subset() {
    let Some(upstream_pp_test) =
        read_upstream_or_skip("harbour-core/tests/hbpp/_pp_test.prg", "upstream hbpp test")
    else {
        return;
    };
    let expected = fs::read_to_string(workspace_fixture(
        "tests/fixtures/pp/constructor_translate_root.out",
    ))
    .expect("fixture snapshot");

    assert!(upstream_pp_test.contains("#xtranslate ( <name>{ [<p,...>] } => (<name>():New(<p>)"));
    assert!(upstream_pp_test.contains("a :=clas( TEST{ 1,2,3} )"));
    assert!(upstream_pp_test.contains("a :=clas( a+3{ 11,2,3} )"));
    assert!(upstream_pp_test.contains("a :=clas( a(){ 11,2,3} )"));

    let output = Preprocessor::default().preprocess(
        SourceFile::from_path(workspace_fixture(
            "tests/fixtures/pp/constructor_translate_root.prg",
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
fn phase15_constructor_identifier_translate_fixture_matches_curated_upstream_subset() {
    let Some(upstream_pp_test) =
        read_upstream_or_skip("harbour-core/tests/hbpp/_pp_test.prg", "upstream hbpp test")
    else {
        return;
    };
    let expected = fs::read_to_string(workspace_fixture(
        "tests/fixtures/pp/constructor_identifier_translate_root.out",
    ))
    .expect("fixture snapshot");

    assert!(upstream_pp_test.contains("#xtranslate ( <!name!>{ [<p,...>] } => (<name>():New(<p>)"));
    assert!(upstream_pp_test.contains("a :=clas( TesT{ 1,2,3} )"));
    assert!(upstream_pp_test.contains("a :=clas( a+3{ 11,2,3} )"));
    assert!(upstream_pp_test.contains("a :=clas( a(){ 11,2,3} )"));

    let output = Preprocessor::default().preprocess(
        SourceFile::from_path(workspace_fixture(
            "tests/fixtures/pp/constructor_identifier_translate_root.prg",
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
fn phase15_regular_marker_compound_fixture_matches_curated_upstream_subset() {
    let Some(upstream_pp_test) =
        read_upstream_or_skip("harbour-core/tests/hbpp/_pp_test.prg", "upstream hbpp test")
    else {
        return;
    };
    let expected = fs::read_to_string(workspace_fixture(
        "tests/fixtures/pp/regular_marker_compound_root.out",
    ))
    .expect("fixture snapshot");

    assert!(upstream_pp_test.contains("#command _REGULAR_(<z>) => rm( <z> )"));
    assert!(upstream_pp_test.contains("_REGULAR_(a)"));
    assert!(upstream_pp_test.contains("_REGULAR_(\"a\")"));
    assert!(upstream_pp_test.contains("_REGULAR_(&a.1)"));
    assert!(upstream_pp_test.contains("_REGULAR_(&a)"));
    assert!(upstream_pp_test.contains("_REGULAR_(a[1])"));

    let output = Preprocessor::default().preprocess(
        SourceFile::from_path(workspace_fixture(
            "tests/fixtures/pp/regular_marker_compound_root.prg",
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
fn phase15_normal_marker_compound_fixture_matches_curated_upstream_subset() {
    let Some(upstream_pp_test) =
        read_upstream_or_skip("harbour-core/tests/hbpp/_pp_test.prg", "upstream hbpp test")
    else {
        return;
    };
    let expected = fs::read_to_string(workspace_fixture(
        "tests/fixtures/pp/normal_marker_compound_root.out",
    ))
    .expect("fixture snapshot");

    assert!(upstream_pp_test.contains("#command _NORMAL_M(<z>) => nm( <\"z\"> )"));
    assert!(upstream_pp_test.contains("_NORMAL_M(a)"));
    assert!(upstream_pp_test.contains("_NORMAL_M(\"a\")"));
    assert!(upstream_pp_test.contains("_NORMAL_M('a')"));
    assert!(upstream_pp_test.contains("_NORMAL_M([\"'a'\"])"));
    assert!(upstream_pp_test.contains("_NORMAL_M(&a.1)"));
    assert!(upstream_pp_test.contains("_NORMAL_M(&a)"));
    assert!(upstream_pp_test.contains("_NORMAL_M(&a.)"));
    assert!(upstream_pp_test.contains("_NORMAL_M(&(a))"));
    assert!(upstream_pp_test.contains("_NORMAL_M(&a[1])"));
    assert!(upstream_pp_test.contains("_NORMAL_M(a[1])"));
    assert!(upstream_pp_test.contains("_NORMAL_M(\"['']\")"));

    let output = Preprocessor::default().preprocess(
        SourceFile::from_path(workspace_fixture(
            "tests/fixtures/pp/normal_marker_compound_root.prg",
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
fn phase15_smart_marker_compound_fixture_matches_curated_upstream_subset() {
    let Some(upstream_pp_test) =
        read_upstream_or_skip("harbour-core/tests/hbpp/_pp_test.prg", "upstream hbpp test")
    else {
        return;
    };
    let expected = fs::read_to_string(workspace_fixture(
        "tests/fixtures/pp/smart_marker_compound_root.out",
    ))
    .expect("fixture snapshot");

    assert!(upstream_pp_test.contains("#command _SMART_M(<z>) => sm( <(z)> )"));
    assert!(upstream_pp_test.contains("_SMART_M(a)"));
    assert!(upstream_pp_test.contains("_SMART_M(\"a\")"));
    assert!(upstream_pp_test.contains("_SMART_M('a')"));
    assert!(upstream_pp_test.contains("_SMART_M([\"'a'\"])"));
    assert!(upstream_pp_test.contains("_SMART_M(&a.1)"));
    assert!(upstream_pp_test.contains("_SMART_M(&a)"));
    assert!(upstream_pp_test.contains("_SMART_M(&a.)"));
    assert!(upstream_pp_test.contains("_SMART_M(&(a))"));
    assert!(upstream_pp_test.contains("_SMART_M(&a[1])"));
    assert!(upstream_pp_test.contains("_SMART_M(a[1])"));
    assert!(upstream_pp_test.contains("_SMART_M(\"['']\")"));

    let output = Preprocessor::default().preprocess(
        SourceFile::from_path(workspace_fixture(
            "tests/fixtures/pp/smart_marker_compound_root.prg",
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
fn phase15_dumb_marker_compound_fixture_matches_curated_upstream_subset() {
    let Some(upstream_pp_test) =
        read_upstream_or_skip("harbour-core/tests/hbpp/_pp_test.prg", "upstream hbpp test")
    else {
        return;
    };
    let expected = fs::read_to_string(workspace_fixture(
        "tests/fixtures/pp/dumb_marker_compound_root.out",
    ))
    .expect("fixture snapshot");

    assert!(upstream_pp_test.contains("#command _DUMB_M(<z>) => dm( #<z> )"));
    assert!(upstream_pp_test.contains("_DUMB_M(a)"));
    assert!(upstream_pp_test.contains("_DUMB_M(\"a\")"));
    assert!(upstream_pp_test.contains("_DUMB_M('a')"));
    assert!(upstream_pp_test.contains("_DUMB_M([\"'a'\"])"));
    assert!(upstream_pp_test.contains("_DUMB_M(&a.1)"));
    assert!(upstream_pp_test.contains("_DUMB_M(&a)"));
    assert!(upstream_pp_test.contains("_DUMB_M(&a.)"));
    assert!(upstream_pp_test.contains("_DUMB_M(&(a))"));
    assert!(upstream_pp_test.contains("_DUMB_M(&a[1])"));
    assert!(upstream_pp_test.contains("_DUMB_M(a[1])"));
    assert!(upstream_pp_test.contains("_DUMB_M(\"['']\")"));

    let output = Preprocessor::default().preprocess(
        SourceFile::from_path(workspace_fixture(
            "tests/fixtures/pp/dumb_marker_compound_root.prg",
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
fn phase15_regular_list_compound_fixture_matches_curated_upstream_subset() {
    let Some(upstream_pp_test) =
        read_upstream_or_skip("harbour-core/tests/hbpp/_pp_test.prg", "upstream hbpp test")
    else {
        return;
    };
    let expected = fs::read_to_string(workspace_fixture(
        "tests/fixtures/pp/regular_list_compound_root.out",
    ))
    .expect("fixture snapshot");

    assert!(upstream_pp_test.contains("#command _REGULAR_L(<z,...>) => rl( <z> )"));
    assert!(upstream_pp_test.contains(
        "_REGULAR_L(a,\"a\",'a',[\"'a'\"],\"['a']\",'[\"a\"]',&a.1,&a,&a.,&a.  ,&(a),&a[1],&a.[1],&a.  [2],&a&a, &a.a,  a, a)"
    ));

    let output = Preprocessor::default().preprocess(
        SourceFile::from_path(workspace_fixture(
            "tests/fixtures/pp/regular_list_compound_root.prg",
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
fn phase15_normal_list_compound_fixture_matches_curated_upstream_subset() {
    let Some(upstream_pp_test) =
        read_upstream_or_skip("harbour-core/tests/hbpp/_pp_test.prg", "upstream hbpp test")
    else {
        return;
    };
    let expected = fs::read_to_string(workspace_fixture(
        "tests/fixtures/pp/normal_list_compound_root.out",
    ))
    .expect("fixture snapshot");

    assert!(upstream_pp_test.contains("#command _NORMAL_L(<z,...>) => nl( <\"z\"> )"));
    assert!(upstream_pp_test.contains(
        "_NORMAL_L(n,\"n\",'a',[\"'a'\"],\"['a']\",'[\"a\"]',&a.1,&a,&a.,&a.  ,&(a),&a[1],&a.[1],&a.  [2],&a&a, &.a, &a.a,  a, a)"
    ));

    let output = Preprocessor::default().preprocess(
        SourceFile::from_path(workspace_fixture(
            "tests/fixtures/pp/normal_list_compound_root.prg",
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
fn phase15_smart_list_compound_fixture_matches_curated_upstream_subset() {
    let Some(upstream_pp_test) =
        read_upstream_or_skip("harbour-core/tests/hbpp/_pp_test.prg", "upstream hbpp test")
    else {
        return;
    };
    let expected = fs::read_to_string(workspace_fixture(
        "tests/fixtures/pp/smart_list_compound_root.out",
    ))
    .expect("fixture snapshot");

    assert!(upstream_pp_test.contains("#command _SMART_L(<z,...>) => sl( <(z)> )"));
    assert!(upstream_pp_test.contains(
        "_SMART_L(a,\"a\",'a',[\"'a'\"],\"['a']\",'[\"a\"]',&a.1,&a,&a.,&a.  ,&(a),&a[1],&a.[1],&a.  [2],&a&a, &.a, &a.a,  a, a)"
    ));

    let output = Preprocessor::default().preprocess(
        SourceFile::from_path(workspace_fixture(
            "tests/fixtures/pp/smart_list_compound_root.prg",
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
fn phase15_dumb_list_compound_fixture_matches_curated_upstream_subset() {
    let Some(upstream_pp_test) =
        read_upstream_or_skip("harbour-core/tests/hbpp/_pp_test.prg", "upstream hbpp test")
    else {
        return;
    };
    let expected = fs::read_to_string(workspace_fixture(
        "tests/fixtures/pp/dumb_list_compound_root.out",
    ))
    .expect("fixture snapshot");

    assert!(upstream_pp_test.contains("#command _DUMB_L(<z,...>) => dl( #<z> )"));
    assert!(upstream_pp_test.contains(
        "_DUMB_L(a,\"a\",'a',[\"'a'\"],\"['a']\",'[\"a\"]',&a.1,&a,&a.,&a.  ,&(a),&a[1],&a.[1],&a.  [2],&a&a, &.a, &a.a,  a, a)"
    ));

    let output = Preprocessor::default().preprocess(
        SourceFile::from_path(workspace_fixture(
            "tests/fixtures/pp/dumb_list_compound_root.prg",
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
fn phase15_index_preserve_spaces_fixture_matches_curated_upstream_subset() {
    let Some(upstream_pp_test) =
        read_upstream_or_skip("harbour-core/tests/hbpp/_pp_test.prg", "upstream hbpp test")
    else {
        return;
    };
    let Some(upstream_std_ch) =
        read_upstream_or_skip("harbour-core/include/std.ch", "upstream std.ch")
    else {
        return;
    };
    let expected = fs::read_to_string(workspace_fixture(
        "tests/fixtures/pp/index_preserve_spaces_root.out",
    ))
    .expect("fixture snapshot");

    assert!(upstream_pp_test.contains("index on LEFT(   f1  ,  10   )      to _tst"));
    assert!(upstream_std_ch.contains("#command INDEX ON <key> TO <(file)> [<u: UNIQUE>] => ;"));
    assert!(
        upstream_std_ch
            .contains("dbCreateIndex( <(file)>, <\"key\">, <{key}>, iif( <.u.>, .t., NIL ) )")
    );

    let output = Preprocessor::default().preprocess(
        SourceFile::from_path(workspace_fixture(
            "tests/fixtures/pp/index_preserve_spaces_root.prg",
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
fn phase15_function_like_define_case_fixture_matches_curated_upstream_subset() {
    let Some(upstream_pp_test) =
        read_upstream_or_skip("harbour-core/tests/hbpp/_pp_test.prg", "upstream hbpp test")
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
        "tests/fixtures/pp/function_like_define_case_root.out",
    ))
    .expect("fixture snapshot");

    assert!(upstream_pp_test.contains("#define F1( n ) F2( n, N )"));
    assert!(upstream_pp_test.contains("#define F3( nN, Nn ) F2( nN, Nn, NN, nn, N, n )"));
    assert!(upstream_hbpptest.contains("pre := \"F2(1 ,N )\""));
    assert!(upstream_hbpptest.contains("pre := \"F2(1,2 ,NN,nn,N,n )\""));

    let output = Preprocessor::default().preprocess(
        SourceFile::from_path(workspace_fixture(
            "tests/fixtures/pp/function_like_define_case_root.prg",
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
fn phase15_multiline_nested_optional_list_fixture_matches_curated_upstream_subset() {
    let Some(upstream_hbpptest) = read_upstream_or_skip(
        "harbour-core/tests/hbpp/hbpptest.prg",
        "upstream hbpp runtime test",
    ) else {
        return;
    };
    let expected = fs::read_to_string(workspace_fixture(
        "tests/fixtures/pp/multiline_nested_optional_list_root.out",
    ))
    .expect("fixture snapshot");

    assert!(upstream_hbpptest.contains("#xcommand SET <var1> [, <varN>] WITH <val> =>"));
    assert!(upstream_hbpptest.contains("pre := \"v1:=0 ; v2:=0 ; v3:=0 ; v4:=0 \""));
    assert!(upstream_hbpptest.contains("#command AVG <x1> [, <xn>] TO <v1> [, <vn>]  =>"));
    assert!(
        upstream_hbpptest
            .contains("pre := \"AVERAGE({||s1:=s1+f1} ,{||s2:=s2+f2}  ,{||s3:=s3+f3}   )\"")
    );

    let output = Preprocessor::default().preprocess(
        SourceFile::from_path(workspace_fixture(
            "tests/fixtures/pp/multiline_nested_optional_list_root.prg",
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
fn phase15_optional_reorder_fixture_matches_curated_upstream_subset() {
    let Some(upstream_hbpp) =
        read_upstream_or_skip("harbour-core/tests/hbpp/_pp_test.prg", "upstream hbpp test")
    else {
        return;
    };
    let expected = fs::read_to_string(workspace_fixture(
        "tests/fixtures/pp/optional_reorder_root.out",
    ))
    .expect("fixture snapshot");

    assert!(upstream_hbpp.contains("#xcommand MYCOMMAND3 [<myList,...>] ;"));
    assert!(upstream_hbpp.contains(
        "[MYCLAUSE <myVal>] [MYOTHER <myOther>] => MyFunction3( {<myList>}, <myVal>, <myOther> )"
    ));
    assert!(upstream_hbpp.contains("MYCOMMAND3 MYCLAUSE 322 \"Hello\" MYOTHER 1"));
    assert!(upstream_hbpp.contains("MYCOMMAND3 MYOTHER 1 MYCLAUSE 322 \"Hello\""));
    assert!(upstream_hbpp.contains("MYCOMMAND3 \"Hello\" MYOTHER 1 MYCLAUSE 322"));
    assert!(upstream_hbpp.contains("MYCOMMAND3 MYOTHER 1 \"Hello\" MYCLAUSE 322"));

    let output = Preprocessor::default().preprocess(
        SourceFile::from_path(workspace_fixture(
            "tests/fixtures/pp/optional_reorder_root.prg",
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
