use std::process::Command;

#[test]
fn benchmark_suite_smoke_renders_markdown_report() {
    let output = Command::new(env!("CARGO_BIN_EXE_benchmark-suite"))
        .arg("--fixture")
        .arg("examples/hello.prg")
        .arg("--iterations")
        .arg("1")
        .output()
        .expect("run benchmark-suite");

    assert_eq!(output.status.code(), Some(0));

    let stdout = String::from_utf8(output.stdout).expect("stdout utf8");
    assert!(stdout.contains("# harbour-rust benchmark suite"));
    assert!(stdout.contains("| examples/hello.prg |"));
}
