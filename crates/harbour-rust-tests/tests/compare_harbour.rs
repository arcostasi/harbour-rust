use std::{
    env, fs,
    path::{Path, PathBuf},
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

use harbour_rust_tests::workspace_path;

fn unique_temp_dir(label: &str) -> PathBuf {
    let suffix = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time")
        .as_nanos();
    env::temp_dir().join(format!("harbour-rust-tests-{label}-{suffix}"))
}

fn write_runner_script(
    directory: &Path,
    name: &str,
    stdout: &str,
    stderr: &str,
    exit_code: i32,
) -> PathBuf {
    if cfg!(windows) {
        let path = directory.join(format!("{name}.cmd"));
        let mut script = String::from("@echo off\r\n");
        if !stdout.is_empty() {
            for line in stdout.lines() {
                script.push_str("echo ");
                script.push_str(line);
                script.push_str("\r\n");
            }
        }
        if !stderr.is_empty() {
            for line in stderr.lines() {
                script.push_str(">&2 echo ");
                script.push_str(line);
                script.push_str("\r\n");
            }
        }
        script.push_str(&format!("exit /b {exit_code}\r\n"));
        fs::write(&path, script).expect("runner script");
        path
    } else {
        let path = directory.join(name);
        let escaped_stdout = stdout.replace('\'', "'\"'\"'");
        let escaped_stderr = stderr.replace('\'', "'\"'\"'");
        let script = format!(
            "#!/bin/sh\nprintf '%s' '{escaped_stdout}'\nprintf '%s' '{escaped_stderr}' >&2\nexit {exit_code}\n"
        );
        fs::write(&path, script).expect("runner script");
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut permissions = fs::metadata(&path).expect("metadata").permissions();
            permissions.set_mode(0o755);
            fs::set_permissions(&path, permissions).expect("chmod");
        }
        path
    }
}

#[test]
fn compare_harbour_reports_match_for_identical_output() {
    let temp_dir = unique_temp_dir("compare-match");
    fs::create_dir_all(&temp_dir).expect("temp dir");
    let runner = write_runner_script(&temp_dir, "harbour_runner", "Hello, world!\n", "", 0);

    let output = Command::new(env!("CARGO_BIN_EXE_compare-harbour"))
        .arg("--fixture")
        .arg("examples/hello.prg")
        .arg("--harbour-runner")
        .arg(&runner)
        .output()
        .expect("run compare-harbour");

    assert_eq!(output.status.code(), Some(0));
    let stdout = String::from_utf8(output.stdout).expect("stdout utf8");
    assert!(stdout.contains("matched harbour-rust and harbour"));

    fs::remove_dir_all(&temp_dir).expect("cleanup temp dir");
}

#[test]
fn compare_harbour_reports_stdout_mismatch() {
    let temp_dir = unique_temp_dir("compare-mismatch");
    fs::create_dir_all(&temp_dir).expect("temp dir");
    let runner = write_runner_script(&temp_dir, "harbour_runner", "OI\n", "", 0);

    let output = Command::new(env!("CARGO_BIN_EXE_compare-harbour"))
        .arg("--fixture")
        .arg("examples/hello.prg")
        .arg("--harbour-runner")
        .arg(&runner)
        .output()
        .expect("run compare-harbour");

    assert_eq!(output.status.code(), Some(1));
    let stderr = String::from_utf8(output.stderr).expect("stderr utf8");
    assert!(stderr.contains("comparison failed for"));
    assert!(stderr.contains("stdout mismatch"));
    assert!(stderr.contains(&workspace_path("examples/hello.prg").display().to_string()));

    fs::remove_dir_all(&temp_dir).expect("cleanup temp dir");
}
