use std::{
    env, fs,
    path::PathBuf,
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

fn workspace_path(path: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join(path)
}

fn unique_temp_dir(label: &str) -> PathBuf {
    let suffix = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time")
        .as_nanos();
    env::temp_dir().join(format!("harbour-rust-cli-{label}-{suffix}"))
}

#[test]
fn help_command_prints_top_level_usage() {
    let output = Command::new(env!("CARGO_BIN_EXE_harbour-rust-cli"))
        .arg("help")
        .output()
        .expect("run cli");

    assert!(output.status.success(), "expected successful help status");

    let stdout = String::from_utf8(output.stdout).expect("stdout utf8");
    assert!(stdout.contains("harbour-rust-cli check"));
    assert!(stdout.contains("harbour-rust-cli transpile --to c"));
}

#[test]
fn check_command_succeeds_for_hello_example() {
    let output = Command::new(env!("CARGO_BIN_EXE_harbour-rust-cli"))
        .arg("check")
        .arg(workspace_path("examples/hello.prg"))
        .output()
        .expect("run cli");

    assert!(output.status.success(), "expected successful check status");

    let stdout = String::from_utf8(output.stdout).expect("stdout utf8");
    assert!(stdout.contains("check succeeded for"));
}

#[test]
fn check_command_reports_preprocess_error_with_frontend_exit_code() {
    let output = Command::new(env!("CARGO_BIN_EXE_harbour-rust-cli"))
        .arg("check")
        .arg(workspace_path("tests/fixtures/pp/phase9_preprocess_error.prg"))
        .output()
        .expect("run cli");

    assert_eq!(output.status.code(), Some(1));

    let stderr = String::from_utf8(output.stderr).expect("stderr utf8");
    assert!(stderr.contains("preprocess failed"));
    assert!(stderr.contains("unterminated rule marker"));
}

#[test]
fn transpile_command_writes_c_output_for_hello_example() {
    let temp_dir = unique_temp_dir("transpile");
    fs::create_dir_all(&temp_dir).expect("temp dir");
    let output_path = temp_dir.join("hello.c");

    let output = Command::new(env!("CARGO_BIN_EXE_harbour-rust-cli"))
        .arg("transpile")
        .arg("--to")
        .arg("c")
        .arg(workspace_path("examples/hello.prg"))
        .arg("--out")
        .arg(&output_path)
        .output()
        .expect("run cli");

    assert!(output.status.success(), "expected successful transpile status");
    assert!(output_path.exists(), "expected generated C output");

    let generated = fs::read_to_string(&output_path).expect("generated c output");
    assert!(generated.contains("harbour_builtin_qout("));

    fs::remove_dir_all(&temp_dir).expect("cleanup temp dir");
}

#[test]
fn transpile_command_requires_explicit_c_target() {
    let output = Command::new(env!("CARGO_BIN_EXE_harbour-rust-cli"))
        .arg("transpile")
        .arg(workspace_path("examples/hello.prg"))
        .output()
        .expect("run cli");

    assert_eq!(output.status.code(), Some(1));

    let stderr = String::from_utf8(output.stderr).expect("stderr utf8");
    assert!(stderr.contains("transpile requires --to c"));
}

#[test]
fn build_command_reports_codegen_failure_with_distinct_exit_code() {
    let output = Command::new(env!("CARGO_BIN_EXE_harbour-rust-cli"))
        .arg("build")
        .arg(workspace_path("tests/fixtures/parser/compound_assign_mod.prg"))
        .output()
        .expect("run cli");

    assert_eq!(output.status.code(), Some(2));

    let stderr = String::from_utf8(output.stderr).expect("stderr utf8");
    assert!(stderr.contains("codegen-c failed"));
}
