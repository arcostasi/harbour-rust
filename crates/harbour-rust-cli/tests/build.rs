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
fn build_command_writes_c_output_for_if_else_fixture() {
    let temp_dir = unique_temp_dir("if-else");
    fs::create_dir_all(&temp_dir).expect("temp dir");
    let output_path = temp_dir.join("if_else.c");

    let status = Command::new(env!("CARGO_BIN_EXE_harbour-rust-cli"))
        .arg("build")
        .arg(workspace_path("tests/fixtures/parser/if_else.prg"))
        .arg("--out")
        .arg(&output_path)
        .status()
        .expect("run cli");

    assert!(status.success(), "expected successful build status");

    let generated = fs::read_to_string(&output_path).expect("generated c output");
    assert!(generated.contains("if (harbour_value_is_true("));
    assert!(generated.contains("harbour_value_greater_than("));
    assert!(generated.contains("else {"));

    fs::remove_dir_all(&temp_dir).expect("cleanup temp dir");
}

#[test]
fn build_command_writes_c_output_for_compound_assign_run_fixture() {
    let temp_dir = unique_temp_dir("compound-assign-run");
    fs::create_dir_all(&temp_dir).expect("temp dir");
    let output_path = temp_dir.join("compound_assign_run.c");

    let status = Command::new(env!("CARGO_BIN_EXE_harbour-rust-cli"))
        .arg("build")
        .arg(workspace_path(
            "tests/fixtures/parser/compound_assign_run.prg",
        ))
        .arg("--out")
        .arg(&output_path)
        .status()
        .expect("run cli");

    assert!(status.success(), "expected successful build status");

    let generated = fs::read_to_string(&output_path).expect("generated c output");
    assert!(generated.contains("harbour_value_add("));
    assert!(generated.contains("harbour_value_subtract("));
    assert!(generated.contains("harbour_value_multiply("));
    assert!(generated.contains("harbour_value_divide("));

    fs::remove_dir_all(&temp_dir).expect("cleanup temp dir");
}

#[test]
fn build_command_writes_c_output_for_len_builtin_fixture() {
    let temp_dir = unique_temp_dir("len-builtin");
    fs::create_dir_all(&temp_dir).expect("temp dir");
    let output_path = temp_dir.join("len_builtin.c");

    let status = Command::new(env!("CARGO_BIN_EXE_harbour-rust-cli"))
        .arg("build")
        .arg(workspace_path("tests/fixtures/parser/len_builtin.prg"))
        .arg("--out")
        .arg(&output_path)
        .status()
        .expect("run cli");

    assert!(status.success(), "expected successful build status");

    let generated = fs::read_to_string(&output_path).expect("generated c output");
    assert!(generated.contains("harbour_builtin_len("));

    fs::remove_dir_all(&temp_dir).expect("cleanup temp dir");
}

#[test]
fn build_command_writes_c_output_for_aclones_fixture() {
    let temp_dir = unique_temp_dir("aclone");
    fs::create_dir_all(&temp_dir).expect("temp dir");
    let output_path = temp_dir.join("aclone.c");

    let status = Command::new(env!("CARGO_BIN_EXE_harbour-rust-cli"))
        .arg("build")
        .arg(workspace_path("tests/fixtures/parser/aclone.prg"))
        .arg("--out")
        .arg(&output_path)
        .status()
        .expect("run cli");

    assert!(status.success(), "expected successful build status");

    let generated = fs::read_to_string(&output_path).expect("generated c output");
    assert!(generated.contains("harbour_builtin_aclone("));
    assert!(generated.contains("harbour_builtin_qout("));

    fs::remove_dir_all(&temp_dir).expect("cleanup temp dir");
}

#[test]
fn build_command_writes_c_output_for_mutable_builtins_fixture() {
    let temp_dir = unique_temp_dir("mutable-builtins");
    fs::create_dir_all(&temp_dir).expect("temp dir");
    let output_path = temp_dir.join("mutable_builtins.c");

    let status = Command::new(env!("CARGO_BIN_EXE_harbour-rust-cli"))
        .arg("build")
        .arg(workspace_path("tests/fixtures/parser/mutable_builtins.prg"))
        .arg("--out")
        .arg(&output_path)
        .status()
        .expect("run cli");

    assert!(status.success(), "expected successful build status");

    let generated = fs::read_to_string(&output_path).expect("generated c output");
    assert!(generated.contains("harbour_builtin_aadd("));
    assert!(generated.contains("harbour_builtin_asize("));

    fs::remove_dir_all(&temp_dir).expect("cleanup temp dir");
}

#[test]
fn build_command_writes_c_output_for_compare_ops_fixture() {
    let temp_dir = unique_temp_dir("compare-ops");
    fs::create_dir_all(&temp_dir).expect("temp dir");
    let output_path = temp_dir.join("compare_ops.c");

    let status = Command::new(env!("CARGO_BIN_EXE_harbour-rust-cli"))
        .arg("build")
        .arg(workspace_path("tests/fixtures/parser/compare_ops.prg"))
        .arg("--out")
        .arg(&output_path)
        .status()
        .expect("run cli");

    assert!(status.success(), "expected successful build status");

    let generated = fs::read_to_string(&output_path).expect("generated c output");
    assert!(generated.contains("harbour_value_exact_equals("));
    assert!(generated.contains("harbour_value_equals("));
    assert!(generated.contains("harbour_value_not_equals("));
    assert!(generated.contains("harbour_value_greater_than("));
    assert!(generated.contains("harbour_value_greater_than_or_equal("));

    fs::remove_dir_all(&temp_dir).expect("cleanup temp dir");
}

#[test]
fn build_command_writes_c_output_for_compare_ops_lt_fixture() {
    let temp_dir = unique_temp_dir("compare-ops-lt");
    fs::create_dir_all(&temp_dir).expect("temp dir");
    let output_path = temp_dir.join("compare_ops_lt.c");

    let status = Command::new(env!("CARGO_BIN_EXE_harbour-rust-cli"))
        .arg("build")
        .arg(workspace_path("tests/fixtures/parser/compare_ops_lt.prg"))
        .arg("--out")
        .arg(&output_path)
        .status()
        .expect("run cli");

    assert!(status.success(), "expected successful build status");

    let generated = fs::read_to_string(&output_path).expect("generated c output");
    assert!(generated.contains("harbour_value_less_than("));
    assert!(generated.contains("harbour_value_less_than_or_equal("));

    fs::remove_dir_all(&temp_dir).expect("cleanup temp dir");
}

#[test]
fn build_command_writes_c_output_for_static_counter_fixture() {
    let temp_dir = unique_temp_dir("static-counter");
    fs::create_dir_all(&temp_dir).expect("temp dir");
    let output_path = temp_dir.join("static_counter.c");

    let status = Command::new(env!("CARGO_BIN_EXE_harbour-rust-cli"))
        .arg("build")
        .arg(workspace_path("tests/fixtures/parser/static_counter.prg"))
        .arg("--out")
        .arg(&output_path)
        .status()
        .expect("run cli");

    assert!(status.success(), "expected successful build status");

    let generated = fs::read_to_string(&output_path).expect("generated c output");
    assert!(generated.contains("static harbour_runtime_Value harbour_static_bump_count;"));
    assert!(generated.contains("if (!harbour_static_bump_count__initialized) {"));
    assert!(generated.contains(
        "harbour_static_bump_count = harbour_value_add(harbour_static_bump_count, harbour_value_from_integer(1LL));"
    ));
    assert!(generated.contains(
        "harbour_builtin_qout((harbour_runtime_Value[]) { harbour_routine_bump() }, 1);"
    ));

    fs::remove_dir_all(&temp_dir).expect("cleanup temp dir");
}

#[test]
fn build_command_uses_configured_include_directory_for_preprocess_handoff() {
    let temp_dir = unique_temp_dir("pp-include");
    fs::create_dir_all(&temp_dir).expect("temp dir");
    let output_path = temp_dir.join("angle_search.c");

    let status = Command::new(env!("CARGO_BIN_EXE_harbour-rust-cli"))
        .arg("build")
        .arg(workspace_path(
            "tests/fixtures/pp/angle_search_path_root.prg",
        ))
        .arg("--include-dir")
        .arg(workspace_path("tests/fixtures/pp/include-path"))
        .arg("--out")
        .arg(&output_path)
        .status()
        .expect("run cli");

    assert!(status.success(), "expected successful build status");

    let generated = fs::read_to_string(&output_path).expect("generated c output");
    assert!(generated.contains("harbour_value_from_string_literal(\"angle search path\")"));

    fs::remove_dir_all(&temp_dir).expect("cleanup temp dir");
}

#[test]
fn build_command_reports_preprocess_error_for_missing_include_search_path() {
    let output = Command::new(env!("CARGO_BIN_EXE_harbour-rust-cli"))
        .arg("build")
        .arg(workspace_path(
            "tests/fixtures/pp/angle_search_path_root.prg",
        ))
        .output()
        .expect("run cli");

    assert!(!output.status.success(), "expected failing build status");

    let stderr = String::from_utf8(output.stderr).expect("stderr utf8");
    assert!(stderr.contains("preprocess failed"));
    assert!(stderr.contains("failed to resolve include"));
}

#[test]
fn build_command_reports_codegen_error_for_unsupported_compound_assign_operator() {
    let output = Command::new(env!("CARGO_BIN_EXE_harbour-rust-cli"))
        .arg("build")
        .arg(workspace_path(
            "tests/fixtures/parser/compound_assign_mod.prg",
        ))
        .output()
        .expect("run cli");

    assert!(!output.status.success(), "expected failing build status");

    let stderr = String::from_utf8(output.stderr).expect("stderr utf8");
    assert!(stderr.contains("codegen-c failed"));
    assert!(stderr.contains("C emission for this binary operator is not implemented yet"));
}

#[test]
fn run_command_executes_len_builtin_fixture_with_expected_output() {
    let output = Command::new(env!("CARGO_BIN_EXE_harbour-rust-cli"))
        .arg("run")
        .arg(workspace_path("tests/fixtures/parser/len_builtin.prg"))
        .output()
        .expect("run cli");

    assert!(output.status.success(), "expected successful run status");

    let stdout = String::from_utf8(output.stdout).expect("stdout utf8");
    assert_eq!(stdout, "4\n3\n");
}

#[test]
fn run_command_executes_len_builtin_invalid_fixture_with_xbase_error_output() {
    let output = Command::new(env!("CARGO_BIN_EXE_harbour-rust-cli"))
        .arg("run")
        .arg(workspace_path(
            "tests/fixtures/parser/len_builtin_invalid.prg",
        ))
        .output()
        .expect("run cli");

    assert!(output.status.success(), "expected successful run status");

    let stdout = String::from_utf8(output.stdout).expect("stdout utf8");
    assert_eq!(stdout, "BASE 1111 Argument error (LEN)\n");
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

#[test]
fn run_command_executes_if_else_fixture_with_expected_output() {
    let output = Command::new(env!("CARGO_BIN_EXE_harbour-rust-cli"))
        .arg("run")
        .arg(workspace_path("tests/fixtures/parser/if_else.prg"))
        .output()
        .expect("run cli");

    assert!(output.status.success(), "expected successful run status");

    let stdout = String::from_utf8(output.stdout).expect("stdout utf8");
    assert_eq!(stdout, "maior\nmenor ou igual\n");
}

#[test]
fn run_command_executes_compound_assign_run_fixture_with_expected_output() {
    let output = Command::new(env!("CARGO_BIN_EXE_harbour-rust-cli"))
        .arg("run")
        .arg(workspace_path(
            "tests/fixtures/parser/compound_assign_run.prg",
        ))
        .output()
        .expect("run cli");

    assert!(output.status.success(), "expected successful run status");

    let stdout = String::from_utf8(output.stdout).expect("stdout utf8");
    assert_eq!(stdout, "15\n12\n24\n6\n");
}

#[test]
fn run_command_executes_indexed_assignment_fixture_with_expected_output() {
    let output = Command::new(env!("CARGO_BIN_EXE_harbour-rust-cli"))
        .arg("run")
        .arg(workspace_path("tests/fixtures/parser/indexed_assign.prg"))
        .output()
        .expect("run cli");

    assert!(output.status.success(), "expected successful run status");

    let stdout = String::from_utf8(output.stdout).expect("stdout utf8");
    assert_eq!(stdout, "99\n");
}

#[test]
fn run_command_executes_aclones_fixture_with_expected_output() {
    let output = Command::new(env!("CARGO_BIN_EXE_harbour-rust-cli"))
        .arg("run")
        .arg(workspace_path("tests/fixtures/parser/aclone.prg"))
        .output()
        .expect("run cli");

    assert!(output.status.success(), "expected successful run status");

    let stdout = String::from_utf8(output.stdout).expect("stdout utf8");
    assert_eq!(stdout, "{ Array(2) }\n");
}

#[test]
fn run_command_executes_mutable_builtins_fixture_with_expected_output() {
    let output = Command::new(env!("CARGO_BIN_EXE_harbour-rust-cli"))
        .arg("run")
        .arg(workspace_path("tests/fixtures/parser/mutable_builtins.prg"))
        .output()
        .expect("run cli");

    assert!(output.status.success(), "expected successful run status");

    let stdout = String::from_utf8(output.stdout).expect("stdout utf8");
    assert_eq!(stdout, "7\n7\n{ Array(3) }\n{ Array(3) }\n");
}

#[test]
fn run_command_executes_array_exact_compare_fixture_with_expected_output() {
    let output = Command::new(env!("CARGO_BIN_EXE_harbour-rust-cli"))
        .arg("run")
        .arg(workspace_path(
            "tests/fixtures/parser/array_exact_compare.prg",
        ))
        .output()
        .expect("run cli");

    assert!(output.status.success(), "expected successful run status");

    let stdout = String::from_utf8(output.stdout).expect("stdout utf8");
    assert_eq!(stdout, ".T.\n.T.\n.F.\n.F.\n");
}

#[test]
fn run_command_executes_compare_ops_fixture_with_xbase_array_diagnostics() {
    let output = Command::new(env!("CARGO_BIN_EXE_harbour-rust-cli"))
        .arg("run")
        .arg(workspace_path("tests/fixtures/parser/compare_ops.prg"))
        .output()
        .expect("run cli");

    assert!(output.status.success(), "expected successful run status");

    let stdout = String::from_utf8(output.stdout).expect("stdout utf8");
    assert_eq!(
        stdout,
        ".T.\nBASE 1071 Argument error (=)\nBASE 1072 Argument error (<>)\nBASE 1075 Argument error (>)\nBASE 1076 Argument error (>=)\n"
    );
}

#[test]
fn run_command_executes_compare_ops_lt_fixture_with_xbase_array_diagnostics() {
    let output = Command::new(env!("CARGO_BIN_EXE_harbour-rust-cli"))
        .arg("run")
        .arg(workspace_path("tests/fixtures/parser/compare_ops_lt.prg"))
        .output()
        .expect("run cli");

    assert!(output.status.success(), "expected successful run status");

    let stdout = String::from_utf8(output.stdout).expect("stdout utf8");
    assert_eq!(
        stdout,
        "BASE 1073 Argument error (<)\nBASE 1074 Argument error (<=)\n"
    );
}

#[test]
fn run_command_executes_static_counter_fixture_with_persistent_output() {
    let output = Command::new(env!("CARGO_BIN_EXE_harbour-rust-cli"))
        .arg("run")
        .arg(workspace_path("tests/fixtures/parser/static_counter.prg"))
        .output()
        .expect("run cli");

    assert!(output.status.success(), "expected successful run status");

    let stdout = String::from_utf8(output.stdout).expect("stdout utf8");
    assert_eq!(stdout, "1\n2\n3\n");
}

#[test]
fn run_command_uses_configured_include_directory_for_preprocess_handoff() {
    let output = Command::new(env!("CARGO_BIN_EXE_harbour-rust-cli"))
        .arg("run")
        .arg(workspace_path(
            "tests/fixtures/pp/angle_search_path_root.prg",
        ))
        .arg("-I")
        .arg(workspace_path("tests/fixtures/pp/include-path"))
        .output()
        .expect("run cli");

    assert!(output.status.success(), "expected successful run status");

    let stdout = String::from_utf8(output.stdout).expect("stdout utf8");
    assert_eq!(stdout, "angle search path\n");
}
