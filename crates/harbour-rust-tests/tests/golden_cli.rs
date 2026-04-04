use harbour_rust_tests::{read_workspace_text, run_fixture};

struct GoldenCase {
    fixture: &'static str,
    snapshot: &'static str,
}

#[test]
fn curated_cli_run_fixtures_match_golden_snapshots() {
    let cases = [
        GoldenCase {
            fixture: "examples/hello.prg",
            snapshot: "tests/golden/cli-run/hello.stdout",
        },
        GoldenCase {
            fixture: "tests/fixtures/parser/phase7_acceptance.prg",
            snapshot: "tests/golden/cli-run/phase7_acceptance.stdout",
        },
        GoldenCase {
            fixture: "tests/fixtures/parser/phase8_acceptance.prg",
            snapshot: "tests/golden/cli-run/phase8_acceptance.stdout",
        },
        GoldenCase {
            fixture: "tests/fixtures/pp/phase9_acceptance.prg",
            snapshot: "tests/golden/cli-run/phase9_acceptance.stdout",
        },
    ];

    for case in cases {
        let expected = read_workspace_text(case.snapshot);
        let actual = run_fixture(case.fixture);
        assert_eq!(actual, expected, "golden mismatch for {}", case.fixture);
    }
}
