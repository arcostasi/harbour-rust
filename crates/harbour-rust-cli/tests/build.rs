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
fn build_command_writes_c_output_for_hello_example() {
    let temp_dir = unique_temp_dir("hello");
    fs::create_dir_all(&temp_dir).expect("temp dir");
    let output_path = temp_dir.join("hello.c");

    let status = Command::new(env!("CARGO_BIN_EXE_harbour-rust-cli"))
        .arg("build")
        .arg(workspace_path("examples/hello.prg"))
        .arg("--out")
        .arg(&output_path)
        .status()
        .expect("run cli");

    assert!(status.success(), "expected successful build status");

    let generated = fs::read_to_string(&output_path).expect("generated c output");
    assert!(generated.contains("int main(void)"));
    assert!(generated.contains("harbour_builtin_qout("));
    assert!(generated.contains("return harbour_value_nil();"));

    fs::remove_dir_all(&temp_dir).expect("cleanup temp dir");
}

#[test]
fn build_command_writes_c_output_for_while_fixture() {
    let temp_dir = unique_temp_dir("while");
    fs::create_dir_all(&temp_dir).expect("temp dir");
    let output_path = temp_dir.join("while.c");

    let status = Command::new(env!("CARGO_BIN_EXE_harbour-rust-cli"))
        .arg("build")
        .arg(workspace_path("tests/fixtures/parser/while.prg"))
        .arg("--out")
        .arg(&output_path)
        .status()
        .expect("run cli");

    assert!(status.success(), "expected successful build status");

    let generated = fs::read_to_string(&output_path).expect("generated c output");
    assert!(generated.contains("while (harbour_value_is_true("));
    assert!(generated.contains("harbour_value_postfix_increment(&x)"));
    assert!(generated.contains("harbour_value_less_than("));

    fs::remove_dir_all(&temp_dir).expect("cleanup temp dir");
}

#[test]
fn build_command_writes_c_output_for_for_sum_fixture() {
    let temp_dir = unique_temp_dir("for-sum");
    fs::create_dir_all(&temp_dir).expect("temp dir");
    let output_path = temp_dir.join("for_sum.c");

    let status = Command::new(env!("CARGO_BIN_EXE_harbour-rust-cli"))
        .arg("build")
        .arg(workspace_path("tests/fixtures/parser/for_sum.prg"))
        .arg("--out")
        .arg(&output_path)
        .status()
        .expect("run cli");

    assert!(status.success(), "expected successful build status");

    let generated = fs::read_to_string(&output_path).expect("generated c output");
    assert!(generated.contains("harbour_value_less_than_or_equal("));
    assert!(generated.contains("sum = harbour_value_add(sum, n);"));
    assert!(generated.contains("n = harbour_value_add(n, harbour_value_from_integer(1LL));"));

    fs::remove_dir_all(&temp_dir).expect("cleanup temp dir");
}

#[test]
fn run_command_executes_hello_example_with_host_compiler() {
    let output = Command::new(env!("CARGO_BIN_EXE_harbour-rust-cli"))
        .arg("run")
        .arg(workspace_path("examples/hello.prg"))
        .output()
        .expect("run cli");

    assert!(output.status.success(), "expected successful run status");

    let stdout = String::from_utf8(output.stdout).expect("stdout utf8");
    assert_eq!(stdout, "Hello, world!\n");
}

#[test]
fn run_command_executes_while_fixture_with_expected_output() {
    let output = Command::new(env!("CARGO_BIN_EXE_harbour-rust-cli"))
        .arg("run")
        .arg(workspace_path("tests/fixtures/parser/while.prg"))
        .output()
        .expect("run cli");

    assert!(output.status.success(), "expected successful run status");

    let stdout = String::from_utf8(output.stdout).expect("stdout utf8");
    assert!(stdout.starts_with("1\n2\n3\n"));
    assert!(stdout.ends_with("998\n999\n1000\n"));
    assert_eq!(stdout.lines().count(), 1000);
}

#[test]
fn run_command_executes_for_sum_fixture_with_expected_output() {
    let output = Command::new(env!("CARGO_BIN_EXE_harbour-rust-cli"))
        .arg("run")
        .arg(workspace_path("tests/fixtures/parser/for_sum.prg"))
        .output()
        .expect("run cli");

    assert!(output.status.success(), "expected successful run status");

    let stdout = String::from_utf8(output.stdout).expect("stdout utf8");
    assert_eq!(stdout, "15\n");
}
