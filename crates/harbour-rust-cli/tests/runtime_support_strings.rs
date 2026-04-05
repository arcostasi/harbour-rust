use std::{
    env,
    ffi::OsString,
    fs,
    path::{Path, PathBuf},
    process::Command,
    sync::atomic::{AtomicU64, Ordering},
    time::{SystemTime, UNIX_EPOCH},
};

static TEMP_DIR_COUNTER: AtomicU64 = AtomicU64::new(1);

#[test]
fn runtime_support_preserves_embedded_nul_bytes_in_string_helpers() {
    let compiler = detect_host_compiler().expect("host compiler");
    let temp_dir = unique_temp_dir("runtime-support-strings");
    fs::create_dir_all(&temp_dir).expect("temp dir");

    let harness_path = temp_dir.join("runtime_support_strings.c");
    let output_path = temp_dir.join(executable_name("runtime_support_strings"));
    let support_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("support");
    let support_c_path = support_dir.join("runtime_support.c");

    fs::write(&harness_path, runtime_support_harness_source()).expect("write harness");

    let compile_output = compiler_command(
        &compiler,
        &support_dir,
        &harness_path,
        &support_c_path,
        &output_path,
    )
    .output()
    .expect("invoke compiler");

    assert!(
        compile_output.status.success(),
        "expected successful host C compilation\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&compile_output.stdout),
        String::from_utf8_lossy(&compile_output.stderr)
    );

    let run_output = Command::new(&output_path).output().expect("run harness");
    assert!(
        run_output.status.success(),
        "expected successful harness status\nstderr:\n{}",
        String::from_utf8_lossy(&run_output.stderr)
    );

    let stdout = String::from_utf8(run_output.stdout)
        .expect("stdout utf8")
        .replace("\r\n", "\n");
    assert_eq!(
        stdout,
        concat!(
            "len:7\n",
            "substr:1:0\n",
            "right:4:0,100,101,102\n",
            "at:4\n",
            "add:11:97,98,99,0,100,101,102,88,89,0,90\n"
        )
    );

    fs::remove_dir_all(&temp_dir).expect("cleanup temp dir");
}

#[test]
fn runtime_support_prints_large_rounded_floats_without_scientific_notation() {
    let compiler = detect_host_compiler().expect("host compiler");
    let temp_dir = unique_temp_dir("runtime-support-floats");
    fs::create_dir_all(&temp_dir).expect("temp dir");

    let harness_path = temp_dir.join("runtime_support_floats.c");
    let output_path = temp_dir.join(executable_name("runtime_support_floats"));
    let support_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("support");
    let support_c_path = support_dir.join("runtime_support.c");

    fs::write(&harness_path, runtime_support_float_harness_source()).expect("write harness");

    let compile_output = compiler_command(
        &compiler,
        &support_dir,
        &harness_path,
        &support_c_path,
        &output_path,
    )
    .output()
    .expect("invoke compiler");

    assert!(
        compile_output.status.success(),
        "expected successful host C compilation\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&compile_output.stdout),
        String::from_utf8_lossy(&compile_output.stderr)
    );

    let run_output = Command::new(&output_path).output().expect("run harness");
    assert!(
        run_output.status.success(),
        "expected successful harness status\nstderr:\n{}",
        String::from_utf8_lossy(&run_output.stderr)
    );

    let stdout = String::from_utf8(run_output.stdout)
        .expect("stdout utf8")
        .replace("\r\n", "\n");
    assert_eq!(stdout, "5000000000.13\n0\n-0.6\n");

    fs::remove_dir_all(&temp_dir).expect("cleanup temp dir");
}

fn runtime_support_harness_source() -> &'static str {
    r#"#include <stdio.h>
#include "runtime_support.h"

static void print_bytes(const char *label, struct harbour_runtime_Value value) {
    size_t index;

    printf("%s:%zu:", label, value.as.string.length);
    for (index = 0; index < value.as.string.length; ++index) {
        printf("%u", (unsigned char) value.as.string.data[index]);
        if (index + 1 < value.as.string.length) {
            printf(",");
        }
    }
    printf("\n");
}

int main(void) {
    static const char source_bytes[] = { 'a', 'b', 'c', '\0', 'd', 'e', 'f' };
    static const char suffix_bytes[] = { 'X', 'Y', '\0', 'Z' };
    static const char needle_bytes[] = { '\0', 'd' };
    struct harbour_runtime_Value source = harbour_value_from_string_parts(source_bytes, sizeof(source_bytes));
    struct harbour_runtime_Value suffix = harbour_value_from_string_parts(suffix_bytes, sizeof(suffix_bytes));
    struct harbour_runtime_Value needle = harbour_value_from_string_parts(needle_bytes, sizeof(needle_bytes));
    struct harbour_runtime_Value substring = harbour_builtin_substr(
        (struct harbour_runtime_Value[]) {
            source,
            harbour_value_from_integer(4),
            harbour_value_from_integer(1)
        },
        3
    );
    struct harbour_runtime_Value right = harbour_builtin_right(
        (struct harbour_runtime_Value[]) {
            source,
            harbour_value_from_integer(4)
        },
        2
    );
    struct harbour_runtime_Value at = harbour_builtin_at(
        (struct harbour_runtime_Value[]) {
            needle,
            source
        },
        2
    );
    struct harbour_runtime_Value added = harbour_value_add(source, suffix);
    struct harbour_runtime_Value length = harbour_builtin_len(
        (struct harbour_runtime_Value[]) { source },
        1
    );

    printf("len:%lld\n", length.as.integer);
    print_bytes("substr", substring);
    print_bytes("right", right);
    printf("at:%lld\n", at.as.integer);
    print_bytes("add", added);
    return 0;
}
"#
}

fn runtime_support_float_harness_source() -> &'static str {
    r##"#include <stdio.h>
#include "runtime_support.h"

int main(void) {
    harbour_builtin_qout(
        (struct harbour_runtime_Value[]) {
            harbour_builtin_round(
                (struct harbour_runtime_Value[]) {
                    harbour_value_from_float(5000000000.129),
                    harbour_value_from_integer(2)
                },
                2
            )
        },
        1
    );
    harbour_builtin_qout(
        (struct harbour_runtime_Value[]) {
            harbour_builtin_round(
                (struct harbour_runtime_Value[]) {
                    harbour_value_from_float(0.5),
                    harbour_value_from_integer(-1)
                },
                2
            )
        },
        1
    );
    harbour_builtin_qout(
        (struct harbour_runtime_Value[]) {
            harbour_builtin_round(
                (struct harbour_runtime_Value[]) {
                    harbour_value_from_float(-0.55),
                    harbour_value_from_integer(1)
                },
                2
            )
        },
        1
    );
    return 0;
}
"##
}

fn detect_host_compiler() -> Option<String> {
    ["gcc", "cc", "clang"]
        .into_iter()
        .find(|candidate| compiler_available(candidate))
        .map(str::to_owned)
}

fn compiler_available(executable: &str) -> bool {
    Command::new(executable)
        .arg("--version")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

fn compiler_command(
    compiler: &str,
    support_dir: &Path,
    harness_path: &Path,
    support_c_path: &Path,
    output_path: &Path,
) -> Command {
    let mut command = Command::new(compiler);
    command.arg("-std=c11");
    command.arg("-Wall");
    command.arg("-Wextra");
    command.arg("-I").arg(support_dir);
    command.arg(harness_path);
    command.arg(support_c_path);
    if cfg!(any(target_os = "linux", target_os = "android")) {
        command.arg("-lm");
    }
    command.arg("-o").arg(output_path);
    command
}

fn unique_temp_dir(label: &str) -> PathBuf {
    let suffix = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time")
        .as_nanos();
    let process_id = std::process::id();
    let counter = TEMP_DIR_COUNTER.fetch_add(1, Ordering::Relaxed);
    env::temp_dir().join(OsString::from(format!(
        "harbour-rust-cli-{label}-{process_id}-{counter}-{suffix}"
    )))
}

fn executable_name(stem: &str) -> String {
    if cfg!(windows) {
        format!("{stem}.exe")
    } else {
        stem.to_owned()
    }
}
