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
fn phase15_nested_function_like_define_fixture_matches_curated_upstream_subset() {
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
        "tests/fixtures/pp/nested_function_like_define_root.out",
    ))
    .expect("fixture snapshot");

    assert!(upstream_pp_test.contains("#define DATENEW   1"));
    assert!(upstream_pp_test.contains("#define DATEOLD(x)   x"));
    assert!(upstream_pp_test.contains("#define datediff(x,y) ( DATEOLD(x) - DATENEW )"));
    assert!(upstream_hbpptest.contains("x := datediff( x, y )"));
    assert!(upstream_hbpptest.contains("pre := \"x := (x - 1 )\""));

    let output = Preprocessor::default().preprocess(
        SourceFile::from_path(workspace_fixture(
            "tests/fixtures/pp/nested_function_like_define_root.prg",
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
fn phase15_constructor_wrapper_function_like_define_fixture_matches_curated_upstream_subset() {
    let Some(upstream_pp_test) =
        read_upstream_or_skip("harbour-core/tests/hbpp/_pp_test.prg", "upstream hbpp test")
    else {
        return;
    };
    let expected = fs::read_to_string(workspace_fixture(
        "tests/fixtures/pp/constructor_wrapper_function_like_define_root.out",
    ))
    .expect("fixture snapshot");

    assert!(upstream_pp_test.contains("#define clas( x )   (x)"));
    assert!(upstream_pp_test.contains("#xtranslate ( <name>{ [<p,...>] } => (<name>():New(<p>)"));
    assert!(upstream_pp_test.contains("a :=clas( TesT{ 1,2,3} )"));
    assert!(upstream_pp_test.contains("a :=clas( a+3{ 11,2,3} )"));
    assert!(upstream_pp_test.contains("a :=clas( a(){ 11,2,3} )"));

    let output = Preprocessor::default().preprocess(
        SourceFile::from_path(workspace_fixture(
            "tests/fixtures/pp/constructor_wrapper_function_like_define_root.prg",
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
fn phase15_tooltip_command_fixture_matches_curated_upstream_subset() {
    let Some(upstream_hbpptest) = read_upstream_or_skip(
        "harbour-core/tests/hbpp/hbpptest.prg",
        "upstream hbpp runtime test",
    ) else {
        return;
    };
    let expected = fs::read_to_string(workspace_fixture(
        "tests/fixtures/pp/tooltip_command_root.out",
    ))
    .expect("fixture snapshot");

    assert!(upstream_hbpptest.contains("#xcommand SET TOOLTIP TO <color> OF <form> => SM("));
    assert!(
        upstream_hbpptest
            .contains("RGB(<color>\\[1], <color>\\[2\\], <color>[, <color>\\[ 3 \\] ]), 0)")
    );
    assert!(upstream_hbpptest.contains("SET TOOLTIP TO RED OF form1"));
    assert!(upstream_hbpptest.contains(
        "SM(TTH (\"form1\"),1,RGB({255,0,0}[1],{255,0,0}[2],{255,0,0},{255,0,0}[ 3 ] ),0)"
    ));

    let output = Preprocessor::default().preprocess(
        SourceFile::from_path(workspace_fixture(
            "tests/fixtures/pp/tooltip_command_root.prg",
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
fn phase15_zzz_escape_fixture_matches_curated_upstream_subset() {
    let Some(upstream_hbpptest) = read_upstream_or_skip(
        "harbour-core/tests/hbpp/hbpptest.prg",
        "upstream hbpp runtime test",
    ) else {
        return;
    };
    let expected = fs::read_to_string(workspace_fixture("tests/fixtures/pp/zzz_escape_root.out"))
        .expect("fixture snapshot");

    assert!(upstream_hbpptest.contains("#command ZZZ [<v>] => QOUT([<v>\\[1\\]])"));
    assert!(upstream_hbpptest.contains("pre :=\"QOUT(a[1] )\""));
    assert!(upstream_hbpptest.contains("pre :=\"QOUT()\""));
    assert!(upstream_hbpptest.contains("pre := \"QOUT(a[1]+2[1] )\""));

    let output = Preprocessor::default().preprocess(
        SourceFile::from_path(workspace_fixture("tests/fixtures/pp/zzz_escape_root.prg"))
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
fn phase15_hmg_escape_translate_fixture_matches_curated_upstream_subset() {
    let Some(upstream_hbpptest) = read_upstream_or_skip(
        "harbour-core/tests/hbpp/hbpptest.prg",
        "upstream hbpp runtime test",
    ) else {
        return;
    };
    let expected = fs::read_to_string(workspace_fixture(
        "tests/fixtures/pp/hmg_escape_translate_root.out",
    ))
    .expect("fixture snapshot");

    assert!(upstream_hbpptest.contains("#xtranslate _HMG_a  =>  _HMG\\[137\\]"));
    assert!(upstream_hbpptest.contains("v:= _bro[ a( _HMG_a [i] ) ]"));
    assert!(upstream_hbpptest.contains("pre :=\"v:= _bro[ a( _HMG[137] [i] ) ]\""));

    let output = Preprocessor::default().preprocess(
        SourceFile::from_path(workspace_fixture(
            "tests/fixtures/pp/hmg_escape_translate_root.prg",
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
fn phase15_set_filter_macro_fixture_matches_curated_upstream_subset() {
    let Some(upstream_hbpptest) = read_upstream_or_skip(
        "harbour-core/tests/hbpp/hbpptest.prg",
        "upstream hbpp runtime test",
    ) else {
        return;
    };
    let Some(upstream_std) =
        read_upstream_or_skip("harbour-core/include/std.ch", "upstream std.ch")
    else {
        return;
    };
    let expected = fs::read_to_string(workspace_fixture(
        "tests/fixtures/pp/set_filter_macro_root.out",
    ))
    .expect("fixture snapshot");

    assert!(
        upstream_std
            .contains("#command SET FILTER TO <exp>     => dbSetFilter( <{exp}>, <\"exp\"> )")
    );
    assert!(upstream_std.contains(
        "#command SET FILTER TO <x:&>     => if ( Empty( <(x)> ) ) ; dbClearFilter() ;;"
    ));
    assert!(upstream_hbpptest.contains("in := \"SET FILTER TO &cVar.\""));
    assert!(upstream_hbpptest.contains("dbSetFilter({|| &cVar.},cVar)"));
    assert!(upstream_hbpptest.contains("in := \"SET FILTER TO &(cVar .AND. &cVar)\""));
    assert!(upstream_hbpptest.contains("dbSetFilter({|| &(cVar .AND. &cVar)},(cVar .AND. &cVar))"));
    assert!(upstream_hbpptest.contains("in := \"SET FILTER TO &cVar. .AND. cVar\""));
    assert!(
        upstream_hbpptest
            .contains("pre := 'dbSetFilter( {|| &cVar. .AND. cVar}, \"&cVar. .AND. cVar\" )'")
    );

    let output = Preprocessor::default().preprocess(
        SourceFile::from_path(workspace_fixture(
            "tests/fixtures/pp/set_filter_macro_root.prg",
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
fn phase15_copy_structure_extended_fixture_matches_curated_upstream_subset() {
    let Some(upstream_hbpptest) = read_upstream_or_skip(
        "harbour-core/tests/hbpp/hbpptest.prg",
        "upstream hbpp runtime test",
    ) else {
        return;
    };
    let Some(upstream_std) =
        read_upstream_or_skip("harbour-core/include/std.ch", "upstream std.ch")
    else {
        return;
    };
    let expected = fs::read_to_string(workspace_fixture(
        "tests/fixtures/pp/copy_structure_extended_root.out",
    ))
    .expect("fixture snapshot");

    assert!(
        upstream_std.contains(
            "#command COPY [STRUCTURE] [EXTENDED] [TO <(f)>] => __dbCopyXStruct( <(f)> )"
        )
    );
    assert!(upstream_hbpptest.contains("in := \"COPY STRUCTURE EXTENDED TO teststru\""));
    assert!(upstream_hbpptest.contains("pre := '__dbCopyXStruct( \"teststru\" )'"));

    let output = Preprocessor::default().preprocess(
        SourceFile::from_path(workspace_fixture(
            "tests/fixtures/pp/copy_structure_extended_root.prg",
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
fn phase15_get_command_base_fixture_matches_curated_upstream_subset() {
    let Some(upstream_hbpptest) = read_upstream_or_skip(
        "harbour-core/tests/hbpp/hbpptest.prg",
        "upstream hbpp runtime test",
    ) else {
        return;
    };
    let expected = fs::read_to_string(workspace_fixture(
        "tests/fixtures/pp/get_command_base_root.out",
    ))
    .expect("fixture snapshot");

    assert!(upstream_hbpptest.contains("#command @ <row>, <col> GET <var>"));
    assert!(upstream_hbpptest.contains("[PICTURE <pic>]"));
    assert!(upstream_hbpptest.contains("=> SetPos( <row>, <col> )"));
    assert!(upstream_hbpptest.contains("in := '@ 0,1 GET a'"));
    assert!(upstream_hbpptest.contains(
        "pre := 'SetPos(0,1 ) ; AAdd(GetList,_GET_(a,\"a\",,, ) )     ; ATail(GetList):Display()'"
    ));

    let output = Preprocessor::default().preprocess(
        SourceFile::from_path(workspace_fixture(
            "tests/fixtures/pp/get_command_base_root.prg",
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
fn phase15_get_command_picture_fixture_matches_curated_upstream_subset() {
    let Some(upstream_hbpptest) = read_upstream_or_skip(
        "harbour-core/tests/hbpp/hbpptest.prg",
        "upstream hbpp runtime test",
    ) else {
        return;
    };
    let expected = fs::read_to_string(workspace_fixture(
        "tests/fixtures/pp/get_command_picture_root.out",
    ))
    .expect("fixture snapshot");

    assert!(upstream_hbpptest.contains("in := '@ 0,2 GET a PICTURE \"X\"'"));
    assert!(
        upstream_hbpptest.contains(
            "pre := 'SetPos(0,2 ) ; AAdd(GetList,_GET_(a,\"a\",\"X\",, ) )     ; ATail(GetList):Display()'"
        )
    );

    let output = Preprocessor::default().preprocess(
        SourceFile::from_path(workspace_fixture(
            "tests/fixtures/pp/get_command_picture_root.prg",
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
fn phase15_get_command_valid_fixture_matches_curated_upstream_subset() {
    let Some(upstream_hbpptest) = read_upstream_or_skip(
        "harbour-core/tests/hbpp/hbpptest.prg",
        "upstream hbpp runtime test",
    ) else {
        return;
    };
    let expected = fs::read_to_string(workspace_fixture(
        "tests/fixtures/pp/get_command_valid_root.out",
    ))
    .expect("fixture snapshot");

    assert!(upstream_hbpptest.contains("in := '@ 0,3 GET a PICTURE \"X\" VALID .T.'"));
    assert!(
        upstream_hbpptest.contains(
            "pre := 'SetPos(0,3 ) ; AAdd(GetList,_GET_(a,\"a\",\"X\",{|| .T.}, ) )     ; ATail(GetList):Display()'"
        )
    );

    let output = Preprocessor::default().preprocess(
        SourceFile::from_path(workspace_fixture(
            "tests/fixtures/pp/get_command_valid_root.prg",
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
fn phase15_get_command_when_fixture_matches_curated_upstream_subset() {
    let Some(upstream_hbpptest) = read_upstream_or_skip(
        "harbour-core/tests/hbpp/hbpptest.prg",
        "upstream hbpp runtime test",
    ) else {
        return;
    };
    let expected = fs::read_to_string(workspace_fixture(
        "tests/fixtures/pp/get_command_when_root.out",
    ))
    .expect("fixture snapshot");

    assert!(upstream_hbpptest.contains("in := '@ 0,4 GET a PICTURE \"X\" VALID .T. WHEN .T.'"));
    assert!(
        upstream_hbpptest.contains(
            "pre := 'SetPos(0,4 ) ; AAdd(GetList,_GET_(a,\"a\",\"X\",{|| .T.},{|| .T.} ) )     ; ATail(GetList):Display()'"
        )
    );

    let output = Preprocessor::default().preprocess(
        SourceFile::from_path(workspace_fixture(
            "tests/fixtures/pp/get_command_when_root.prg",
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
fn phase15_get_command_caption_fixture_matches_curated_upstream_subset() {
    let Some(upstream_hbpptest) = read_upstream_or_skip(
        "harbour-core/tests/hbpp/hbpptest.prg",
        "upstream hbpp runtime test",
    ) else {
        return;
    };
    let expected = fs::read_to_string(workspace_fixture(
        "tests/fixtures/pp/get_command_caption_root.out",
    ))
    .expect("fixture snapshot");

    assert!(
        upstream_hbpptest
            .contains("in := '@ 0,5 GET a PICTURE \"X\" VALID .T. WHEN .T. CAPTION \"myget\"'")
    );
    assert!(upstream_hbpptest.contains(
        "pre := 'SetPos(0,5 ) ; AAdd(GetList,_GET_(a,\"a\",\"X\",{|| .T.},{|| .T.} ) ) ; ATail(GetList):Caption := \"myget\"  ; ATail(GetList):CapRow := ATail(Getlist):row ; ATail(GetList):CapCol := ATail(Getlist):col - __CapLength(\"myget\") - 1    ; ATail(GetList):Display()'"
    ));

    let output = Preprocessor::default().preprocess(
        SourceFile::from_path(workspace_fixture(
            "tests/fixtures/pp/get_command_caption_root.prg",
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
fn phase15_get_command_message_fixture_matches_curated_upstream_subset() {
    let Some(upstream_hbpptest) = read_upstream_or_skip(
        "harbour-core/tests/hbpp/hbpptest.prg",
        "upstream hbpp runtime test",
    ) else {
        return;
    };
    let expected = fs::read_to_string(workspace_fixture(
        "tests/fixtures/pp/get_command_message_root.out",
    ))
    .expect("fixture snapshot");

    assert!(upstream_hbpptest.contains(
        "in := '@ 0,6 GET a PICTURE \"X\" VALID .T. WHEN .T. CAPTION \"myget\" MESSAGE \"mymess\"'"
    ));
    assert!(upstream_hbpptest.contains(
        "pre := 'SetPos(0,6 ) ; AAdd(GetList,_GET_(a,\"a\",\"X\",{|| .T.},{|| .T.} ) ) ; ATail(GetList):Caption := \"myget\"  ; ATail(GetList):CapRow := ATail(Getlist):row ; ATail(GetList):CapCol := ATail(Getlist):col - __CapLength(\"myget\") - 1  ; ATail(GetList):message := \"mymess\"   ; ATail(GetList):Display()'"
    ));

    let output = Preprocessor::default().preprocess(
        SourceFile::from_path(workspace_fixture(
            "tests/fixtures/pp/get_command_message_root.prg",
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
fn phase15_get_command_send_fixture_matches_curated_upstream_subset() {
    let Some(upstream_hbpptest) = read_upstream_or_skip(
        "harbour-core/tests/hbpp/hbpptest.prg",
        "upstream hbpp runtime test",
    ) else {
        return;
    };
    let expected = fs::read_to_string(workspace_fixture(
        "tests/fixtures/pp/get_command_send_root.out",
    ))
    .expect("fixture snapshot");

    assert!(upstream_hbpptest.contains(
        "in := '@ 0,7 GET a PICTURE \"X\" VALID .T. WHEN .T. CAPTION \"myget\" MESSAGE \"mymess\" SEND send()'"
    ));
    assert!(upstream_hbpptest.contains(
        "pre := 'SetPos(0,7 ) ; AAdd(GetList,_GET_(a,\"a\",\"X\",{|| .T.},{|| .T.} ) ) ; ATail(GetList):Caption := \"myget\"  ; ATail(GetList):CapRow := ATail(Getlist):row ; ATail(GetList):CapCol := ATail(Getlist):col - __CapLength(\"myget\") - 1  ; ATail(GetList):message := \"mymess\"  ; ATail(GetList):send()  ; ATail(GetList):Display()'"
    ));

    let output = Preprocessor::default().preprocess(
        SourceFile::from_path(workspace_fixture(
            "tests/fixtures/pp/get_command_send_root.prg",
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
fn phase15_get_command_range_fixture_matches_curated_upstream_subset() {
    let Some(upstream_hbpptest) = read_upstream_or_skip(
        "harbour-core/tests/hbpp/hbpptest.prg",
        "upstream hbpp runtime test",
    ) else {
        return;
    };
    let expected = fs::read_to_string(workspace_fixture(
        "tests/fixtures/pp/get_command_range_root.out",
    ))
    .expect("fixture snapshot");

    assert!(upstream_hbpptest.contains("in :='@ 1,1 GET a RANGE 0,100'"));
    assert!(upstream_hbpptest.contains(
        "pre := 'SetPos(1,1 ) ; AAdd(GetList,_GET_(a,\"a\",,{|_1| RangeCheck(_1,, 0, 100)}, ) )     ; ATail(GetList):Display()'"
    ));

    let output = Preprocessor::default().preprocess(
        SourceFile::from_path(workspace_fixture(
            "tests/fixtures/pp/get_command_range_root.prg",
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
fn phase15_get_command_picture_range_fixture_matches_curated_upstream_subset() {
    let Some(upstream_hbpptest) = read_upstream_or_skip(
        "harbour-core/tests/hbpp/hbpptest.prg",
        "upstream hbpp runtime test",
    ) else {
        return;
    };
    let expected = fs::read_to_string(workspace_fixture(
        "tests/fixtures/pp/get_command_picture_range_root.out",
    ))
    .expect("fixture snapshot");

    assert!(upstream_hbpptest.contains("in :='@ 1,2 GET a PICTURE \"X\" RANGE 0,100'"));
    assert!(upstream_hbpptest.contains(
        "pre := 'SetPos(1,2 ) ; AAdd(GetList,_GET_(a,\"a\",\"X\",{|_1| RangeCheck(_1,, 0, 100)}, ) )     ; ATail(GetList):Display()'"
    ));

    let output = Preprocessor::default().preprocess(
        SourceFile::from_path(workspace_fixture(
            "tests/fixtures/pp/get_command_picture_range_root.prg",
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
fn phase15_get_command_range_picture_reordered_fixture_matches_curated_upstream_subset() {
    let Some(upstream_hbpptest) = read_upstream_or_skip(
        "harbour-core/tests/hbpp/hbpptest.prg",
        "upstream hbpp runtime test",
    ) else {
        return;
    };
    let expected = fs::read_to_string(workspace_fixture(
        "tests/fixtures/pp/get_command_range_picture_reordered_root.out",
    ))
    .expect("fixture snapshot");

    assert!(upstream_hbpptest.contains("in :='@ 2,2 GET a RANGE 0,100 PICTURE \"X\"'"));
    assert!(upstream_hbpptest.contains(
        "pre := 'SetPos(2,2 ) ; AAdd(GetList,_GET_(a,\"a\",\"X\",{|_1| RangeCheck(_1,, 0, 100)}, ) )     ; ATail(GetList):Display()'"
    ));

    let output = Preprocessor::default().preprocess(
        SourceFile::from_path(workspace_fixture(
            "tests/fixtures/pp/get_command_range_picture_reordered_root.prg",
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
fn phase15_get_command_valid_range_fixture_matches_curated_upstream_subset() {
    let Some(upstream_hbpptest) = read_upstream_or_skip(
        "harbour-core/tests/hbpp/hbpptest.prg",
        "upstream hbpp runtime test",
    ) else {
        return;
    };
    let expected = fs::read_to_string(workspace_fixture(
        "tests/fixtures/pp/get_command_valid_range_root.out",
    ))
    .expect("fixture snapshot");

    assert!(upstream_hbpptest.contains("in :='@ 1,3 GET a PICTURE \"X\" VALID .T. RANGE 0,100'"));
    assert!(upstream_hbpptest.contains(
        "pre := 'SetPos(1,3 ) ; AAdd(GetList,_GET_(a,\"a\",\"X\",{|| .T.}, ) )     ; ATail(GetList):Display()'"
    ));

    let output = Preprocessor::default().preprocess(
        SourceFile::from_path(workspace_fixture(
            "tests/fixtures/pp/get_command_valid_range_root.prg",
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
fn phase15_get_command_when_range_fixture_matches_curated_upstream_subset() {
    let Some(upstream_hbpptest) = read_upstream_or_skip(
        "harbour-core/tests/hbpp/hbpptest.prg",
        "upstream hbpp runtime test",
    ) else {
        return;
    };
    let expected = fs::read_to_string(workspace_fixture(
        "tests/fixtures/pp/get_command_when_range_root.out",
    ))
    .expect("fixture snapshot");

    assert!(upstream_hbpptest.contains("in :='@ 1,4 GET a PICTURE \"X\" WHEN .T. RANGE 0,100'"));
    assert!(upstream_hbpptest.contains(
        "pre := 'SetPos(1,4 ) ; AAdd(GetList,_GET_(a,\"a\",\"X\",{|_1| RangeCheck(_1,, 0, 100)},{|| .T.} ) )     ; ATail(GetList):Display()'"
    ));

    let output = Preprocessor::default().preprocess(
        SourceFile::from_path(workspace_fixture(
            "tests/fixtures/pp/get_command_when_range_root.prg",
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
fn phase15_get_command_picture_range_when_reordered_fixture_matches_curated_upstream_subset() {
    let Some(upstream_hbpptest) = read_upstream_or_skip(
        "harbour-core/tests/hbpp/hbpptest.prg",
        "upstream hbpp runtime test",
    ) else {
        return;
    };
    let expected = fs::read_to_string(workspace_fixture(
        "tests/fixtures/pp/get_command_picture_range_when_reordered_root.out",
    ))
    .expect("fixture snapshot");

    assert!(upstream_hbpptest.contains("in :='@ 2,4 GET a PICTURE \"X\" RANGE 0,100 WHEN .T.'"));
    assert!(upstream_hbpptest.contains(
        "pre := 'SetPos(2,4 ) ; AAdd(GetList,_GET_(a,\"a\",\"X\",{|_1| RangeCheck(_1,, 0, 100)},{|| .T.} ) )     ; ATail(GetList):Display()'"
    ));

    let output = Preprocessor::default().preprocess(
        SourceFile::from_path(workspace_fixture(
            "tests/fixtures/pp/get_command_picture_range_when_reordered_root.prg",
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
fn phase15_get_command_picture_range_when_caption_reordered_fixture_matches_curated_upstream_subset()
 {
    let Some(upstream_hbpptest) = read_upstream_or_skip(
        "harbour-core/tests/hbpp/hbpptest.prg",
        "upstream hbpp runtime test",
    ) else {
        return;
    };
    let expected = fs::read_to_string(workspace_fixture(
        "tests/fixtures/pp/get_command_picture_range_when_caption_reordered_root.out",
    ))
    .expect("fixture snapshot");

    assert!(
        upstream_hbpptest
            .contains("in :='@ 2,5 GET a PICTURE \"X\" RANGE 0,100 WHEN .T. CAPTION \"myget\"'")
    );
    assert!(upstream_hbpptest.contains(
        "pre := 'SetPos(2,5 ) ; AAdd(GetList,_GET_(a,\"a\",\"X\",{|_1| RangeCheck(_1,, 0, 100)},{|| .T.} ) ) ; ATail(GetList):Caption := \"myget\"  ; ATail(GetList):CapRow := ATail(Getlist):row ; ATail(GetList):CapCol := ATail(Getlist):col - __CapLength(\"myget\") - 1    ; ATail(GetList):Display()'"
    ));

    let output = Preprocessor::default().preprocess(
        SourceFile::from_path(workspace_fixture(
            "tests/fixtures/pp/get_command_picture_range_when_caption_reordered_root.prg",
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
fn phase15_get_command_picture_range_when_caption_message_reordered_fixture_matches_curated_upstream_subset()
 {
    let Some(upstream_hbpptest) = read_upstream_or_skip(
        "harbour-core/tests/hbpp/hbpptest.prg",
        "upstream hbpp runtime test",
    ) else {
        return;
    };
    let expected = fs::read_to_string(workspace_fixture(
        "tests/fixtures/pp/get_command_picture_range_when_caption_message_reordered_root.out",
    ))
    .expect("fixture snapshot");

    assert!(upstream_hbpptest.contains(
        "in :='@ 2,6 GET a PICTURE \"X\" RANGE 0,100 WHEN .T. CAPTION \"myget\" MESSAGE \"mymess\"'"
    ));
    assert!(upstream_hbpptest.contains(
        "pre := 'SetPos(2,6 ) ; AAdd(GetList,_GET_(a,\"a\",\"X\",{|_1| RangeCheck(_1,, 0, 100)},{|| .T.} ) ) ; ATail(GetList):Caption := \"myget\"  ; ATail(GetList):CapRow := ATail(Getlist):row ; ATail(GetList):CapCol := ATail(Getlist):col - __CapLength(\"myget\") - 1  ; ATail(GetList):message := \"mymess\"   ; ATail(GetList):Display()'"
    ));

    let output = Preprocessor::default().preprocess(
        SourceFile::from_path(workspace_fixture(
            "tests/fixtures/pp/get_command_picture_range_when_caption_message_reordered_root.prg",
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
fn phase15_get_command_picture_range_when_caption_message_send_reordered_fixture_matches_curated_upstream_subset()
 {
    let Some(upstream_hbpptest) = read_upstream_or_skip(
        "harbour-core/tests/hbpp/hbpptest.prg",
        "upstream hbpp runtime test",
    ) else {
        return;
    };
    let expected = fs::read_to_string(workspace_fixture(
        "tests/fixtures/pp/get_command_picture_range_when_caption_message_send_reordered_root.out",
    ))
    .expect("fixture snapshot");

    assert!(upstream_hbpptest.contains(
        "in :='@ 2,7 GET a PICTURE \"X\" RANGE 0,100 WHEN .T. CAPTION \"myget\" MESSAGE \"mymess\" SEND send()'"
    ));
    assert!(upstream_hbpptest.contains(
        "pre := 'SetPos(2,7 ) ; AAdd(GetList,_GET_(a,\"a\",\"X\",{|_1| RangeCheck(_1,, 0, 100)},{|| .T.} ) ) ; ATail(GetList):Caption := \"myget\"  ; ATail(GetList):CapRow := ATail(Getlist):row ; ATail(GetList):CapCol := ATail(Getlist):col - __CapLength(\"myget\") - 1  ; ATail(GetList):message := \"mymess\"  ; ATail(GetList):send()  ; ATail(GetList):Display()'"
    ));

    let output = Preprocessor::default().preprocess(
        SourceFile::from_path(workspace_fixture(
            "tests/fixtures/pp/get_command_picture_range_when_caption_message_send_reordered_root.prg",
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
fn phase15_get_command_pushbutton_base_fixture_matches_curated_upstream_subset() {
    let Some(upstream_hbpptest) = read_upstream_or_skip(
        "harbour-core/tests/hbpp/hbpptest.prg",
        "upstream hbpp runtime test",
    ) else {
        return;
    };
    let expected = fs::read_to_string(workspace_fixture(
        "tests/fixtures/pp/get_command_pushbutton_base_root.out",
    ))
    .expect("fixture snapshot");

    assert!(upstream_hbpptest.contains("in :='@ 4,1 GET a PUSHBUTTON'"));
    assert!(upstream_hbpptest.contains(
        "pre := 'SetPos(4,1 ) ; AAdd(GetList,_GET_(a,\"a\",NIL,, ) ) ; ATail(GetList):Control := _PushButt_(,,,,,,,,,,,, ) ; ATail(GetList):reader := { | a,b,c,d | GuiReader(a,b,c,d ) }   ; ATail(GetList):Control:Display()'"
    ));

    let output = Preprocessor::default().preprocess(
        SourceFile::from_path(workspace_fixture(
            "tests/fixtures/pp/get_command_pushbutton_base_root.prg",
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
fn phase15_get_command_pushbutton_valid_fixture_matches_curated_upstream_subset() {
    let Some(upstream_hbpptest) = read_upstream_or_skip(
        "harbour-core/tests/hbpp/hbpptest.prg",
        "upstream hbpp runtime test",
    ) else {
        return;
    };
    let expected = fs::read_to_string(workspace_fixture(
        "tests/fixtures/pp/get_command_pushbutton_valid_root.out",
    ))
    .expect("fixture snapshot");

    assert!(upstream_hbpptest.contains("in :='@ 4,1 GET a PUSHBUTTON VALID valid()'"));
    assert!(upstream_hbpptest.contains(
        "pre := 'SetPos(4,1 ) ; AAdd(GetList,_GET_(a,\"a\",NIL,{|| valid()}, ) ) ; ATail(GetList):Control := _PushButt_(,,,,,,,,,,,, ) ; ATail(GetList):reader := { | a,b,c,d | GuiReader(a,b,c,d ) }   ; ATail(GetList):Control:Display()'"
    ));

    let output = Preprocessor::default().preprocess(
        SourceFile::from_path(workspace_fixture(
            "tests/fixtures/pp/get_command_pushbutton_valid_root.prg",
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
fn phase15_get_command_pushbutton_when_fixture_matches_curated_upstream_subset() {
    let Some(upstream_hbpptest) = read_upstream_or_skip(
        "harbour-core/tests/hbpp/hbpptest.prg",
        "upstream hbpp runtime test",
    ) else {
        return;
    };
    let expected = fs::read_to_string(workspace_fixture(
        "tests/fixtures/pp/get_command_pushbutton_when_root.out",
    ))
    .expect("fixture snapshot");

    assert!(upstream_hbpptest.contains("in :='@ 4,1 GET a PUSHBUTTON VALID valid() WHEN when()'"));
    assert!(upstream_hbpptest.contains(
        "pre := 'SetPos(4,1 ) ; AAdd(GetList,_GET_(a,\"a\",NIL,{|| valid()},{|| when()} ) ) ; ATail(GetList):Control := _PushButt_(,,,,,,,,,,,, ) ; ATail(GetList):reader := { | a,b,c,d | GuiReader(a,b,c,d ) }   ; ATail(GetList):Control:Display()'"
    ));

    let output = Preprocessor::default().preprocess(
        SourceFile::from_path(workspace_fixture(
            "tests/fixtures/pp/get_command_pushbutton_when_root.prg",
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
fn phase15_get_command_pushbutton_caption_fixture_matches_curated_upstream_subset() {
    let Some(upstream_hbpptest) = read_upstream_or_skip(
        "harbour-core/tests/hbpp/hbpptest.prg",
        "upstream hbpp runtime test",
    ) else {
        return;
    };
    let expected = fs::read_to_string(workspace_fixture(
        "tests/fixtures/pp/get_command_pushbutton_caption_root.out",
    ))
    .expect("fixture snapshot");

    assert!(
        upstream_hbpptest
            .contains("in :='@ 4,1 GET a PUSHBUTTON VALID valid() WHEN when() CAPTION \"cap\"'")
    );
    assert!(upstream_hbpptest.contains(
        "pre := 'SetPos(4,1 ) ; AAdd(GetList,_GET_(a,\"a\",NIL,{|| valid()},{|| when()} ) ) ; ATail(GetList):Control := _PushButt_(\"cap\",,,,,,,,,,,, ) ; ATail(GetList):reader := { | a,b,c,d | GuiReader(a,b,c,d ) }   ; ATail(GetList):Control:Display()'"
    ));

    let output = Preprocessor::default().preprocess(
        SourceFile::from_path(workspace_fixture(
            "tests/fixtures/pp/get_command_pushbutton_caption_root.prg",
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
fn phase15_get_command_pushbutton_message_fixture_matches_curated_upstream_subset() {
    let Some(upstream_hbpptest) = read_upstream_or_skip(
        "harbour-core/tests/hbpp/hbpptest.prg",
        "upstream hbpp runtime test",
    ) else {
        return;
    };
    let expected = fs::read_to_string(workspace_fixture(
        "tests/fixtures/pp/get_command_pushbutton_message_root.out",
    ))
    .expect("fixture snapshot");

    assert!(upstream_hbpptest.contains(
        "in :='@ 4,1 GET a PUSHBUTTON VALID valid() WHEN when() CAPTION \"cap\" MESSAGE \"mes\"'"
    ));
    assert!(upstream_hbpptest.contains(
        "pre := 'SetPos(4,1 ) ; AAdd(GetList,_GET_(a,\"a\",NIL,{|| valid()},{|| when()} ) ) ; ATail(GetList):Control := _PushButt_(\"cap\",\"mes\",,,,,,,,,,, ) ; ATail(GetList):reader := { | a,b,c,d | GuiReader(a,b,c,d ) }   ; ATail(GetList):Control:Display()'"
    ));

    let output = Preprocessor::default().preprocess(
        SourceFile::from_path(workspace_fixture(
            "tests/fixtures/pp/get_command_pushbutton_message_root.prg",
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
fn phase15_get_command_pushbutton_color_fixture_matches_curated_upstream_subset() {
    let Some(upstream_hbpptest) = read_upstream_or_skip(
        "harbour-core/tests/hbpp/hbpptest.prg",
        "upstream hbpp runtime test",
    ) else {
        return;
    };
    let expected = fs::read_to_string(workspace_fixture(
        "tests/fixtures/pp/get_command_pushbutton_color_root.out",
    ))
    .expect("fixture snapshot");

    assert!(upstream_hbpptest.contains(
        "in :='@ 4,1 GET a PUSHBUTTON VALID valid() WHEN when() CAPTION \"cap\" MESSAGE \"mes\" COLOR color()'"
    ));
    assert!(upstream_hbpptest.contains(
        "pre := 'SetPos(4,1 ) ; AAdd(GetList,_GET_(a,\"a\",NIL,{|| valid()},{|| when()} ) ) ; ATail(GetList):Control := _PushButt_(\"cap\",\"mes\",color(),,,,,,,,,, ) ; ATail(GetList):reader := { | a,b,c,d | GuiReader(a,b,c,d ) }   ; ATail(GetList):Control:Display()'"
    ));

    let output = Preprocessor::default().preprocess(
        SourceFile::from_path(workspace_fixture(
            "tests/fixtures/pp/get_command_pushbutton_color_root.prg",
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
fn phase15_get_command_pushbutton_focus_fixture_matches_curated_upstream_subset() {
    let Some(upstream_hbpptest) = read_upstream_or_skip(
        "harbour-core/tests/hbpp/hbpptest.prg",
        "upstream hbpp runtime test",
    ) else {
        return;
    };
    let expected = fs::read_to_string(workspace_fixture(
        "tests/fixtures/pp/get_command_pushbutton_focus_root.out",
    ))
    .expect("fixture snapshot");

    assert!(upstream_hbpptest.contains(
        "in :='@ 4,1 GET a PUSHBUTTON VALID valid() WHEN when() CAPTION \"cap\" MESSAGE \"mes\" COLOR color() FOCUS focus()'"
    ));
    assert!(upstream_hbpptest.contains(
        "pre := 'SetPos(4,1 ) ; AAdd(GetList,_GET_(a,\"a\",NIL,{|| valid()},{|| when()} ) ) ; ATail(GetList):Control := _PushButt_(\"cap\",\"mes\",color(),{|| focus()},,,,,,,,, ) ; ATail(GetList):reader := { | a,b,c,d | GuiReader(a,b,c,d ) }   ; ATail(GetList):Control:Display()'"
    ));

    let output = Preprocessor::default().preprocess(
        SourceFile::from_path(workspace_fixture(
            "tests/fixtures/pp/get_command_pushbutton_focus_root.prg",
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
fn phase15_get_command_pushbutton_state_fixture_matches_curated_upstream_subset() {
    let Some(upstream_hbpptest) = read_upstream_or_skip(
        "harbour-core/tests/hbpp/hbpptest.prg",
        "upstream hbpp runtime test",
    ) else {
        return;
    };
    let expected = fs::read_to_string(workspace_fixture(
        "tests/fixtures/pp/get_command_pushbutton_state_root.out",
    ))
    .expect("fixture snapshot");

    assert!(upstream_hbpptest.contains(
        "in :='@ 4,1 GET a PUSHBUTTON VALID valid() WHEN when() CAPTION \"cap\" MESSAGE \"mes\" COLOR color() FOCUS focus() STATE state()'"
    ));
    assert!(upstream_hbpptest.contains(
        "pre := 'SetPos(4,1 ) ; AAdd(GetList,_GET_(a,\"a\",NIL,{|| valid()},{|| when()} ) ) ; ATail(GetList):Control := _PushButt_(\"cap\",\"mes\",color(),{|| focus()},{|| state()},,,,,,,, ) ; ATail(GetList):reader := { | a,b,c,d | GuiReader(a,b,c,d ) }   ; ATail(GetList):Control:Display()'"
    ));

    let output = Preprocessor::default().preprocess(
        SourceFile::from_path(workspace_fixture(
            "tests/fixtures/pp/get_command_pushbutton_state_root.prg",
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
fn phase15_get_command_pushbutton_style_fixture_matches_curated_upstream_subset() {
    let Some(upstream_hbpptest) = read_upstream_or_skip(
        "harbour-core/tests/hbpp/hbpptest.prg",
        "upstream hbpp runtime test",
    ) else {
        return;
    };
    let expected = fs::read_to_string(workspace_fixture(
        "tests/fixtures/pp/get_command_pushbutton_style_root.out",
    ))
    .expect("fixture snapshot");

    assert!(upstream_hbpptest.contains(
        "in :='@ 4,1 GET a PUSHBUTTON VALID valid() WHEN when() CAPTION \"cap\" MESSAGE \"mes\" COLOR color() FOCUS focus() STATE state() STYLE style()'"
    ));
    assert!(upstream_hbpptest.contains(
        "pre := 'SetPos(4,1 ) ; AAdd(GetList,_GET_(a,\"a\",NIL,{|| valid()},{|| when()} ) ) ; ATail(GetList):Control := _PushButt_(\"cap\",\"mes\",color(),{|| focus()},{|| state()},style(),,,,,,, ) ; ATail(GetList):reader := { | a,b,c,d | GuiReader(a,b,c,d ) }   ; ATail(GetList):Control:Display()'"
    ));

    let output = Preprocessor::default().preprocess(
        SourceFile::from_path(workspace_fixture(
            "tests/fixtures/pp/get_command_pushbutton_style_root.prg",
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
fn phase15_get_command_pushbutton_send_fixture_matches_curated_upstream_subset() {
    let Some(upstream_hbpptest) = read_upstream_or_skip(
        "harbour-core/tests/hbpp/hbpptest.prg",
        "upstream hbpp runtime test",
    ) else {
        return;
    };
    let expected = fs::read_to_string(workspace_fixture(
        "tests/fixtures/pp/get_command_pushbutton_send_root.out",
    ))
    .expect("fixture snapshot");

    assert!(upstream_hbpptest.contains(
        "in :='@ 4,1 GET a PUSHBUTTON VALID valid() WHEN when() CAPTION \"cap\" MESSAGE \"mes\" COLOR color() FOCUS focus() STATE state() STYLE style() SEND send()'"
    ));
    assert!(upstream_hbpptest.contains(
        "pre := 'SetPos(4,1 ) ; AAdd(GetList,_GET_(a,\"a\",NIL,{|| valid()},{|| when()} ) ) ; ATail(GetList):Control := _PushButt_(\"cap\",\"mes\",color(),{|| focus()},{|| state()},style(),,,,,,, ) ; ATail(GetList):reader := { | a,b,c,d | GuiReader(a,b,c,d ) } ; ATail(GetList):send()   ; ATail(GetList):Control:Display()'"
    ));

    let output = Preprocessor::default().preprocess(
        SourceFile::from_path(workspace_fixture(
            "tests/fixtures/pp/get_command_pushbutton_send_root.prg",
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
fn phase15_get_command_pushbutton_guisend_fixture_matches_curated_upstream_subset() {
    let Some(upstream_hbpptest) = read_upstream_or_skip(
        "harbour-core/tests/hbpp/hbpptest.prg",
        "upstream hbpp runtime test",
    ) else {
        return;
    };
    let expected = fs::read_to_string(workspace_fixture(
        "tests/fixtures/pp/get_command_pushbutton_guisend_root.out",
    ))
    .expect("fixture snapshot");

    assert!(upstream_hbpptest.contains(
        "in :='@ 4,1 GET a PUSHBUTTON VALID valid() WHEN when() CAPTION \"cap\" MESSAGE \"mes\" COLOR color() FOCUS focus() STATE state() STYLE style() SEND send() GUISEND guisend()'"
    ));
    assert!(upstream_hbpptest.contains(
        "pre := 'SetPos(4,1 ) ; AAdd(GetList,_GET_(a,\"a\",NIL,{|| valid()},{|| when()} ) ) ; ATail(GetList):Control := _PushButt_(\"cap\",\"mes\",color(),{|| focus()},{|| state()},style(),,,,,,, ) ; ATail(GetList):reader := { | a,b,c,d | GuiReader(a,b,c,d ) } ; ATail(GetList):send()  ; ATail(GetList):Control:guisend()  ; ATail(GetList):Control:Display()'"
    ));

    let output = Preprocessor::default().preprocess(
        SourceFile::from_path(workspace_fixture(
            "tests/fixtures/pp/get_command_pushbutton_guisend_root.prg",
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
fn phase15_get_command_pushbutton_size_fixture_matches_curated_upstream_subset() {
    let Some(upstream_hbpptest) = read_upstream_or_skip(
        "harbour-core/tests/hbpp/hbpptest.prg",
        "upstream hbpp runtime test",
    ) else {
        return;
    };
    let expected = fs::read_to_string(workspace_fixture(
        "tests/fixtures/pp/get_command_pushbutton_size_root.out",
    ))
    .expect("fixture snapshot");

    assert!(upstream_hbpptest.contains(
        "in :='@ 4,1 GET a PUSHBUTTON VALID valid() WHEN when() CAPTION \"cap\" MESSAGE \"mes\" COLOR color() FOCUS focus() STATE state() STYLE style() SEND send() GUISEND guisend() SIZE X 100 Y 100'"
    ));
    assert!(upstream_hbpptest.contains(
        "pre := 'SetPos(4,1 ) ; AAdd(GetList,_GET_(a,\"a\",NIL,{|| valid()},{|| when()} ) ) ; ATail(GetList):Control := _PushButt_(\"cap\",\"mes\",color(),{|| focus()},{|| state()},style(),100,100,,,,, ) ; ATail(GetList):reader := { | a,b,c,d | GuiReader(a,b,c,d ) } ; ATail(GetList):send()  ; ATail(GetList):Control:guisend()  ; ATail(GetList):Control:Display()'"
    ));

    let output = Preprocessor::default().preprocess(
        SourceFile::from_path(workspace_fixture(
            "tests/fixtures/pp/get_command_pushbutton_size_root.prg",
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
fn phase15_get_command_pushbutton_capoff_fixture_matches_curated_upstream_subset() {
    let Some(upstream_hbpptest) = read_upstream_or_skip(
        "harbour-core/tests/hbpp/hbpptest.prg",
        "upstream hbpp runtime test",
    ) else {
        return;
    };
    let expected = fs::read_to_string(workspace_fixture(
        "tests/fixtures/pp/get_command_pushbutton_capoff_root.out",
    ))
    .expect("fixture snapshot");

    assert!(upstream_hbpptest.contains(
        "in :='@ 4,1 GET a PUSHBUTTON VALID valid() WHEN when() CAPTION \"cap\" MESSAGE \"mes\" COLOR color() FOCUS focus() STATE state() STYLE style() SEND send() GUISEND guisend() SIZE X 100 Y 100 CAPOFF X 10 Y 10'"
    ));
    assert!(upstream_hbpptest.contains(
        "pre := 'SetPos(4,1 ) ; AAdd(GetList,_GET_(a,\"a\",NIL,{|| valid()},{|| when()} ) ) ; ATail(GetList):Control := _PushButt_(\"cap\",\"mes\",color(),{|| focus()},{|| state()},style(),100,100,10,10,,, ) ; ATail(GetList):reader := { | a,b,c,d | GuiReader(a,b,c,d ) } ; ATail(GetList):send()  ; ATail(GetList):Control:guisend()  ; ATail(GetList):Control:Display()'"
    ));

    let output = Preprocessor::default().preprocess(
        SourceFile::from_path(workspace_fixture(
            "tests/fixtures/pp/get_command_pushbutton_capoff_root.prg",
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
fn phase15_get_command_pushbutton_bitmap_fixture_matches_curated_upstream_subset() {
    let Some(upstream_hbpptest) = read_upstream_or_skip(
        "harbour-core/tests/hbpp/hbpptest.prg",
        "upstream hbpp runtime test",
    ) else {
        return;
    };
    let expected = fs::read_to_string(workspace_fixture(
        "tests/fixtures/pp/get_command_pushbutton_bitmap_root.out",
    ))
    .expect("fixture snapshot");

    assert!(upstream_hbpptest.contains(
        "in :='@ 4,1 GET a PUSHBUTTON VALID valid() WHEN when() CAPTION \"cap\" MESSAGE \"mes\" COLOR color() FOCUS focus() STATE state() STYLE style() SEND send() GUISEND guisend() SIZE X 100 Y 100 CAPOFF X 10 Y 10 BITMAP bitmap()'"
    ));
    assert!(upstream_hbpptest.contains(
        "pre := 'SetPos(4,1 ) ; AAdd(GetList,_GET_(a,\"a\",NIL,{|| valid()},{|| when()} ) ) ; ATail(GetList):Control := _PushButt_(\"cap\",\"mes\",color(),{|| focus()},{|| state()},style(),100,100,10,10,bitmap(),, ) ; ATail(GetList):reader := { | a,b,c,d | GuiReader(a,b,c,d ) } ; ATail(GetList):send()  ; ATail(GetList):Control:guisend()  ; ATail(GetList):Control:Display()'"
    ));

    let output = Preprocessor::default().preprocess(
        SourceFile::from_path(workspace_fixture(
            "tests/fixtures/pp/get_command_pushbutton_bitmap_root.prg",
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
fn phase15_get_command_pushbutton_bmpoff_fixture_matches_curated_upstream_subset() {
    let Some(upstream_hbpptest) = read_upstream_or_skip(
        "harbour-core/tests/hbpp/hbpptest.prg",
        "upstream hbpp runtime test",
    ) else {
        return;
    };
    let expected = fs::read_to_string(workspace_fixture(
        "tests/fixtures/pp/get_command_pushbutton_bmpoff_root.out",
    ))
    .expect("fixture snapshot");

    assert!(upstream_hbpptest.contains(
        "in :='@ 4,1 GET a PUSHBUTTON VALID valid() WHEN when() CAPTION \"cap\" MESSAGE \"mes\" COLOR color() FOCUS focus() STATE state() STYLE style() SEND send() GUISEND guisend() SIZE X 100 Y 100 CAPOFF X 10 Y 10 BITMAP bitmap() BMPOFF X 2 Y 2'"
    ));
    assert!(upstream_hbpptest.contains(
        "pre := 'SetPos(4,1 ) ; AAdd(GetList,_GET_(a,\"a\",NIL,{|| valid()},{|| when()} ) ) ; ATail(GetList):Control := _PushButt_(\"cap\",\"mes\",color(),{|| focus()},{|| state()},style(),100,100,10,10,bitmap(),2,2 ) ; ATail(GetList):reader := { | a,b,c,d | GuiReader(a,b,c,d ) } ; ATail(GetList):send()  ; ATail(GetList):Control:guisend()  ; ATail(GetList):Control:Display()'"
    ));

    let output = Preprocessor::default().preprocess(
        SourceFile::from_path(workspace_fixture(
            "tests/fixtures/pp/get_command_pushbutton_bmpoff_root.prg",
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
fn phase15_get_command_pushbutton_color_only_fixture_matches_curated_upstream_subset() {
    let Some(upstream_hbpptest) = read_upstream_or_skip(
        "harbour-core/tests/hbpp/hbpptest.prg",
        "upstream hbpp runtime test",
    ) else {
        return;
    };
    let expected = fs::read_to_string(workspace_fixture(
        "tests/fixtures/pp/get_command_pushbutton_color_only_root.out",
    ))
    .expect("fixture snapshot");

    assert!(upstream_hbpptest.contains("in :='@ 4,1 GET a PUSHBUTTON COLOR \"W/N\"'"));
    assert!(upstream_hbpptest.contains(
        "pre := 'SetPos(4,1 ) ; AAdd(GetList,_GET_(a,\"a\",NIL,, ) ) ; ATail(GetList):Control := _PushButt_(,,\"W/N\",,,,,,,,,, ) ; ATail(GetList):reader := { | a,b,c,d | GuiReader(a,b,c,d ) }   ; ATail(GetList):Control:Display()'"
    ));

    let output = Preprocessor::default().preprocess(
        SourceFile::from_path(workspace_fixture(
            "tests/fixtures/pp/get_command_pushbutton_color_only_root.prg",
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
fn phase15_get_command_pushbutton_reordered_sparse_fixture_matches_curated_upstream_subset() {
    let Some(upstream_hbpptest) = read_upstream_or_skip(
        "harbour-core/tests/hbpp/hbpptest.prg",
        "upstream hbpp runtime test",
    ) else {
        return;
    };
    let expected = fs::read_to_string(workspace_fixture(
        "tests/fixtures/pp/get_command_pushbutton_reordered_sparse_root.out",
    ))
    .expect("fixture snapshot");

    assert!(upstream_hbpptest.contains(
        "in :='@ 4,1 GET a PUSHBUTTON COLOR \"W/N\" SIZE X 100 Y 100 BMPOFF X 2 Y 2 VALID valid() GUISEND guisend() WHEN when() MESSAGE \"mes\"'"
    ));
    assert!(upstream_hbpptest.contains(
        "pre := 'SetPos(4,1 ) ; AAdd(GetList,_GET_(a,\"a\",NIL,{|| valid()},{|| when()} ) ) ; ATail(GetList):Control := _PushButt_(,\"mes\",\"W/N\",,,,100,100,,,,2,2 ) ; ATail(GetList):reader := { | a,b,c,d | GuiReader(a,b,c,d ) }  ; ATail(GetList):Control:guisend()  ; ATail(GetList):Control:Display()'"
    ));

    let output = Preprocessor::default().preprocess(
        SourceFile::from_path(workspace_fixture(
            "tests/fixtures/pp/get_command_pushbutton_reordered_sparse_root.prg",
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
fn phase15_get_command_pushbutton_reordered_sparse_color_tail_fixture_matches_curated_upstream_subset()
 {
    let Some(upstream_hbpptest) = read_upstream_or_skip(
        "harbour-core/tests/hbpp/hbpptest.prg",
        "upstream hbpp runtime test",
    ) else {
        return;
    };
    let expected = fs::read_to_string(workspace_fixture(
        "tests/fixtures/pp/get_command_pushbutton_reordered_sparse_color_tail_root.out",
    ))
    .expect("fixture snapshot");

    assert!(upstream_hbpptest.contains(
        "in :='@ 4,1 GET a PUSHBUTTON SIZE X 100 Y 100 BMPOFF X 2 Y 2 VALID valid() GUISEND guisend() WHEN when() MESSAGE \"mes\" COLOR \"W/N\"'"
    ));
    assert!(upstream_hbpptest.contains(
        "pre := 'SetPos(4,1 ) ; AAdd(GetList,_GET_(a,\"a\",NIL,{|| valid()},{|| when()} ) ) ; ATail(GetList):Control := _PushButt_(,\"mes\",\"W/N\",,,,100,100,,,,2,2 ) ; ATail(GetList):reader := { | a,b,c,d | GuiReader(a,b,c,d ) }  ; ATail(GetList):Control:guisend()  ; ATail(GetList):Control:Display()'"
    ));

    let output = Preprocessor::default().preprocess(
        SourceFile::from_path(workspace_fixture(
            "tests/fixtures/pp/get_command_pushbutton_reordered_sparse_color_tail_root.prg",
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
fn phase15_get_command_pushbutton_reordered_full_fixture_matches_curated_upstream_subset() {
    let Some(upstream_hbpptest) = read_upstream_or_skip(
        "harbour-core/tests/hbpp/hbpptest.prg",
        "upstream hbpp runtime test",
    ) else {
        return;
    };
    let expected = fs::read_to_string(workspace_fixture(
        "tests/fixtures/pp/get_command_pushbutton_reordered_full_root.out",
    ))
    .expect("fixture snapshot");

    assert!(upstream_hbpptest.contains(
        "in :='@ 4,1 GET a PUSHBUTTON SIZE X 100 Y 100 BMPOFF X 2 Y 2 VALID valid() GUISEND guisend() WHEN when() MESSAGE \"mes\" COLOR \"W/N\" CAPOFF X 10 Y 10 FOCUS focus() STATE state() STYLE style() SEND send() BITMAP bitmap() CAPTION \"cap\"'"
    ));
    assert!(upstream_hbpptest.contains(
        "pre := 'SetPos(4,1 ) ; AAdd(GetList,_GET_(a,\"a\",NIL,{|| valid()},{|| when()} ) ) ; ATail(GetList):Control := _PushButt_(\"cap\",\"mes\",\"W/N\",{|| focus()},{|| state()},style(),100,100,10,10,bitmap(),2,2 ) ; ATail(GetList):reader := { | a,b,c,d | GuiReader(a,b,c,d ) } ; ATail(GetList):send()  ; ATail(GetList):Control:guisend()  ; ATail(GetList):Control:Display()'"
    ));

    let output = Preprocessor::default().preprocess(
        SourceFile::from_path(workspace_fixture(
            "tests/fixtures/pp/get_command_pushbutton_reordered_full_root.prg",
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
fn phase15_define_clipboard_fixture_matches_curated_upstream_subset() {
    let Some(upstream_hbpptest) = read_upstream_or_skip(
        "harbour-core/tests/hbpp/hbpptest.prg",
        "upstream hbpp runtime test",
    ) else {
        return;
    };
    let expected = fs::read_to_string(workspace_fixture(
        "tests/fixtures/pp/define_clipboard_root.out",
    ))
    .expect("fixture snapshot");

    assert!(upstream_hbpptest.contains("#command DEFINE CLIPBOARD <oClp>"));
    assert!(upstream_hbpptest.contains("in:= \"DEFINE CLIPBOARD oC OF oD FORMAT TEXT\""));
    assert!(upstream_hbpptest.contains("pre :='oC := TClipboard():New(UPPER(\"TEXT\") ,oD )'"));

    let output = Preprocessor::default().preprocess(
        SourceFile::from_path(workspace_fixture(
            "tests/fixtures/pp/define_clipboard_root.prg",
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
fn phase15_define_clipboard_oemtext_fixture_matches_curated_rule_subset() {
    let Some(upstream_hbpptest) = read_upstream_or_skip(
        "harbour-core/tests/hbpp/hbpptest.prg",
        "upstream hbpp runtime test",
    ) else {
        return;
    };
    let expected = fs::read_to_string(workspace_fixture(
        "tests/fixtures/pp/define_clipboard_oemtext_root.out",
    ))
    .expect("fixture snapshot");

    assert!(upstream_hbpptest.contains("#command DEFINE CLIPBOARD <oClp>"));
    assert!(upstream_hbpptest.contains("FORMAT <format:TEXT,OEMTEXT,BITMAP,DIF>"));
    assert!(
        upstream_hbpptest.contains("<oClp> := TClipboard():New( [UPPER(<(format)>)], <oWnd> )")
    );

    let output = Preprocessor::default().preprocess(
        SourceFile::from_path(workspace_fixture(
            "tests/fixtures/pp/define_clipboard_oemtext_root.prg",
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
fn phase15_release_all_fixture_matches_curated_upstream_subset() {
    let Some(upstream_pp_test) = read_upstream_or_skip(
        "harbour-core/tests/hbpp/_pp_test.prg",
        "upstream _pp_test corpus",
    ) else {
        return;
    };
    let Some(upstream_std) =
        read_upstream_or_skip("harbour-core/include/std.ch", "upstream std.ch")
    else {
        return;
    };
    let expected = fs::read_to_string(workspace_fixture("tests/fixtures/pp/release_all_root.out"))
        .expect("fixture snapshot");

    assert!(
        upstream_std.contains("#command RELEASE <v,...>               => __mvXRelease( <\"v\"> )")
    );
    assert!(
        upstream_std
            .contains("#command RELEASE ALL                   => __mvRelease( \"*\", .t. )")
    );
    assert!(upstream_pp_test.contains("RELEASE ALL"));

    let output = Preprocessor::default().preprocess(
        SourceFile::from_path(workspace_fixture("tests/fixtures/pp/release_all_root.prg"))
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
fn phase15_release_all_like_fixture_matches_curated_upstream_subset() {
    let Some(upstream_pp_test) = read_upstream_or_skip(
        "harbour-core/tests/hbpp/_pp_test.prg",
        "upstream _pp_test corpus",
    ) else {
        return;
    };
    let Some(upstream_std) =
        read_upstream_or_skip("harbour-core/include/std.ch", "upstream std.ch")
    else {
        return;
    };
    let expected = fs::read_to_string(workspace_fixture(
        "tests/fixtures/pp/release_all_like_root.out",
    ))
    .expect("fixture snapshot");

    assert!(
        upstream_std.contains("#command RELEASE <v,...>               => __mvXRelease( <\"v\"> )")
    );
    assert!(
        upstream_std
            .contains("#command RELEASE ALL                   => __mvRelease( \"*\", .t. )")
    );
    assert!(
        upstream_std.contains("#command RELEASE ALL LIKE <p>          => __mvRelease( #<p>, .t. )")
    );
    assert!(upstream_pp_test.contains("RELEASE ALL LIKE A"));

    let output = Preprocessor::default().preprocess(
        SourceFile::from_path(workspace_fixture(
            "tests/fixtures/pp/release_all_like_root.prg",
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
fn phase15_release_all_except_fixture_matches_curated_upstream_subset() {
    let Some(upstream_pp_test) = read_upstream_or_skip(
        "harbour-core/tests/hbpp/_pp_test.prg",
        "upstream _pp_test corpus",
    ) else {
        return;
    };
    let Some(upstream_std) =
        read_upstream_or_skip("harbour-core/include/std.ch", "upstream std.ch")
    else {
        return;
    };
    let expected = fs::read_to_string(workspace_fixture(
        "tests/fixtures/pp/release_all_except_root.out",
    ))
    .expect("fixture snapshot");

    assert!(
        upstream_std.contains("#command RELEASE <v,...>               => __mvXRelease( <\"v\"> )")
    );
    assert!(
        upstream_std
            .contains("#command RELEASE ALL                   => __mvRelease( \"*\", .t. )")
    );
    assert!(
        upstream_std.contains("#command RELEASE ALL LIKE <p>          => __mvRelease( #<p>, .t. )")
    );
    assert!(
        upstream_std.contains("#command RELEASE ALL EXCEPT <p>        => __mvRelease( #<p>, .f. )")
    );
    assert!(upstream_pp_test.contains("RELEASE ALL EXCEPT A"));

    let output = Preprocessor::default().preprocess(
        SourceFile::from_path(workspace_fixture(
            "tests/fixtures/pp/release_all_except_root.prg",
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
fn phase15_save_all_like_fixture_matches_curated_upstream_subset() {
    let Some(upstream_pp_test) = read_upstream_or_skip(
        "harbour-core/tests/hbpp/_pp_test.prg",
        "upstream _pp_test corpus",
    ) else {
        return;
    };
    let Some(upstream_std) =
        read_upstream_or_skip("harbour-core/include/std.ch", "upstream std.ch")
    else {
        return;
    };
    let expected = fs::read_to_string(workspace_fixture(
        "tests/fixtures/pp/save_all_like_root.out",
    ))
    .expect("fixture snapshot");

    assert!(
        upstream_std
            .contains("#command SAVE TO <(f)> ALL LIKE <p>    => __mvSave( <(f)>, <(p)>, .t. )")
    );
    assert!(
        upstream_std
            .contains("#command SAVE ALL LIKE <p> TO <(f)>    => __mvSave( <(f)>, <(p)>, .t. )")
    );
    assert!(upstream_pp_test.contains("SAVE ALL LIKE A TO A"));

    let output = Preprocessor::default().preprocess(
        SourceFile::from_path(workspace_fixture(
            "tests/fixtures/pp/save_all_like_root.prg",
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
fn phase15_save_to_all_like_fixture_matches_curated_upstream_subset() {
    let Some(upstream_pp_test) = read_upstream_or_skip(
        "harbour-core/tests/hbpp/_pp_test.prg",
        "upstream _pp_test corpus",
    ) else {
        return;
    };
    let Some(upstream_std) =
        read_upstream_or_skip("harbour-core/include/std.ch", "upstream std.ch")
    else {
        return;
    };
    let expected = fs::read_to_string(workspace_fixture(
        "tests/fixtures/pp/save_to_all_like_root.out",
    ))
    .expect("fixture snapshot");

    assert!(
        upstream_std
            .contains("#command SAVE TO <(f)> ALL LIKE <p>    => __mvSave( <(f)>, <(p)>, .t. )")
    );
    assert!(
        upstream_std
            .contains("#command SAVE ALL LIKE <p> TO <(f)>    => __mvSave( <(f)>, <(p)>, .t. )")
    );
    assert!(upstream_pp_test.contains("SAVE TO A ALL LIKE A"));

    let output = Preprocessor::default().preprocess(
        SourceFile::from_path(workspace_fixture(
            "tests/fixtures/pp/save_to_all_like_root.prg",
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
fn phase15_save_to_all_fixture_matches_curated_upstream_subset() {
    let Some(upstream_pp_test) = read_upstream_or_skip(
        "harbour-core/tests/hbpp/_pp_test.prg",
        "upstream _pp_test corpus",
    ) else {
        return;
    };
    let Some(upstream_std) =
        read_upstream_or_skip("harbour-core/include/std.ch", "upstream std.ch")
    else {
        return;
    };
    let expected = fs::read_to_string(workspace_fixture("tests/fixtures/pp/save_to_all_root.out"))
        .expect("fixture snapshot");

    assert!(
        upstream_std
            .contains("#command SAVE [TO <(f)>] [ALL]         => __mvSave( <(f)>, \"*\", .t. )")
    );
    assert!(upstream_pp_test.contains("SAVE TO A ALL"));

    let output = Preprocessor::default().preprocess(
        SourceFile::from_path(workspace_fixture("tests/fixtures/pp/save_to_all_root.prg"))
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
fn phase15_save_all_except_fixture_matches_curated_upstream_subset() {
    let Some(upstream_pp_test) = read_upstream_or_skip(
        "harbour-core/tests/hbpp/_pp_test.prg",
        "upstream _pp_test corpus",
    ) else {
        return;
    };
    let Some(upstream_std) =
        read_upstream_or_skip("harbour-core/include/std.ch", "upstream std.ch")
    else {
        return;
    };
    let expected = fs::read_to_string(workspace_fixture(
        "tests/fixtures/pp/save_all_except_root.out",
    ))
    .expect("fixture snapshot");

    assert!(
        upstream_std
            .contains("#command SAVE ALL EXCEPT <p> TO <(f)>  => __mvSave( <(f)>, <(p)>, .f. )")
    );
    assert!(
        upstream_std
            .contains("#command SAVE TO <(f)> ALL EXCEPT <p>  => __mvSave( <(f)>, <(p)>, .f. )")
    );
    assert!(upstream_pp_test.contains("SAVE ALL EXCEPT A TO A"));

    let output = Preprocessor::default().preprocess(
        SourceFile::from_path(workspace_fixture(
            "tests/fixtures/pp/save_all_except_root.prg",
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
fn phase15_save_to_all_except_fixture_matches_curated_upstream_subset() {
    let Some(upstream_pp_test) = read_upstream_or_skip(
        "harbour-core/tests/hbpp/_pp_test.prg",
        "upstream _pp_test corpus",
    ) else {
        return;
    };
    let Some(upstream_std) =
        read_upstream_or_skip("harbour-core/include/std.ch", "upstream std.ch")
    else {
        return;
    };
    let expected = fs::read_to_string(workspace_fixture(
        "tests/fixtures/pp/save_to_all_except_root.out",
    ))
    .expect("fixture snapshot");

    assert!(
        upstream_std
            .contains("#command SAVE ALL EXCEPT <p> TO <(f)>  => __mvSave( <(f)>, <(p)>, .f. )")
    );
    assert!(
        upstream_std
            .contains("#command SAVE TO <(f)> ALL EXCEPT <p>  => __mvSave( <(f)>, <(p)>, .f. )")
    );
    assert!(upstream_pp_test.contains("SAVE TO A ALL EXCEPT A"));

    let output = Preprocessor::default().preprocess(
        SourceFile::from_path(workspace_fixture(
            "tests/fixtures/pp/save_to_all_except_root.prg",
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
fn phase15_get_command_caption_range_fixture_matches_curated_upstream_subset() {
    let Some(upstream_hbpptest) = read_upstream_or_skip(
        "harbour-core/tests/hbpp/hbpptest.prg",
        "upstream hbpp runtime test",
    ) else {
        return;
    };
    let expected = fs::read_to_string(workspace_fixture(
        "tests/fixtures/pp/get_command_caption_range_root.out",
    ))
    .expect("fixture snapshot");

    assert!(
        upstream_hbpptest
            .contains("in :='@ 1,5 GET a PICTURE \"X\" WHEN .T. CAPTION \"myget\" RANGE 0,100'")
    );
    assert!(upstream_hbpptest.contains(
        "pre := 'SetPos(1,5 ) ; AAdd(GetList,_GET_(a,\"a\",\"X\",{|_1| RangeCheck(_1,, 0, 100)},{|| .T.} ) ) ; ATail(GetList):Caption := \"myget\"  ; ATail(GetList):CapRow := ATail(Getlist):row ; ATail(GetList):CapCol := ATail(Getlist):col - __CapLength(\"myget\") - 1    ; ATail(GetList):Display()'"
    ));

    let output = Preprocessor::default().preprocess(
        SourceFile::from_path(workspace_fixture(
            "tests/fixtures/pp/get_command_caption_range_root.prg",
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
fn phase15_get_command_message_range_fixture_matches_curated_upstream_subset() {
    let Some(upstream_hbpptest) = read_upstream_or_skip(
        "harbour-core/tests/hbpp/hbpptest.prg",
        "upstream hbpp runtime test",
    ) else {
        return;
    };
    let expected = fs::read_to_string(workspace_fixture(
        "tests/fixtures/pp/get_command_message_range_root.out",
    ))
    .expect("fixture snapshot");

    assert!(upstream_hbpptest.contains(
        "in :='@ 1,6 GET a PICTURE \"X\" WHEN .T. CAPTION \"myget\" MESSAGE \"mymess\" RANGE 0,100'"
    ));
    assert!(upstream_hbpptest.contains(
        "pre := 'SetPos(1,6 ) ; AAdd(GetList,_GET_(a,\"a\",\"X\",{|_1| RangeCheck(_1,, 0, 100)},{|| .T.} ) ) ; ATail(GetList):Caption := \"myget\"  ; ATail(GetList):CapRow := ATail(Getlist):row ; ATail(GetList):CapCol := ATail(Getlist):col - __CapLength(\"myget\") - 1  ; ATail(GetList):message := \"mymess\"   ; ATail(GetList):Display()'"
    ));

    let output = Preprocessor::default().preprocess(
        SourceFile::from_path(workspace_fixture(
            "tests/fixtures/pp/get_command_message_range_root.prg",
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
fn phase15_get_command_send_range_fixture_matches_curated_upstream_subset() {
    let Some(upstream_hbpptest) = read_upstream_or_skip(
        "harbour-core/tests/hbpp/hbpptest.prg",
        "upstream hbpp runtime test",
    ) else {
        return;
    };
    let expected = fs::read_to_string(workspace_fixture(
        "tests/fixtures/pp/get_command_send_range_root.out",
    ))
    .expect("fixture snapshot");

    assert!(upstream_hbpptest.contains(
        "in :='@ 1,7 GET a PICTURE \"X\" WHEN .T. CAPTION \"myget\" MESSAGE \"mymess\" SEND send() RANGE 0,100'"
    ));
    assert!(upstream_hbpptest.contains(
        "pre := 'SetPos(1,7 ) ; AAdd(GetList,_GET_(a,\"a\",\"X\",{|_1| RangeCheck(_1,, 0, 100)},{|| .T.} ) ) ; ATail(GetList):Caption := \"myget\"  ; ATail(GetList):CapRow := ATail(Getlist):row ; ATail(GetList):CapCol := ATail(Getlist):col - __CapLength(\"myget\") - 1  ; ATail(GetList):message := \"mymess\"  ; ATail(GetList):send()  ; ATail(GetList):Display()'"
    ));

    let output = Preprocessor::default().preprocess(
        SourceFile::from_path(workspace_fixture(
            "tests/fixtures/pp/get_command_send_range_root.prg",
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
