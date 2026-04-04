use std::{
    fs, io,
    path::{Path, PathBuf},
    process::Command,
};

use harbour_rust_cli::{BuildOptions, RunOptions, build_to_c, run_with_host_compiler};

pub fn workspace_path(path: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join(path)
}

pub fn read_workspace_text(path: &str) -> String {
    fs::read_to_string(workspace_path(path)).expect("workspace text")
}

pub fn read_path_text(path: &Path) -> String {
    fs::read_to_string(path).expect("path text")
}

pub fn run_fixture(path: &str) -> String {
    run_with_host_compiler(&RunOptions {
        input_path: workspace_path(path),
        include_directories: Vec::new(),
    })
    .expect("run fixture")
    .stdout
}

pub fn build_fixture_to_temp(path: &str, output_path: &Path) -> String {
    build_to_c(&BuildOptions {
        input_path: workspace_path(path),
        output_path: Some(output_path.to_path_buf()),
        include_directories: Vec::new(),
    })
    .expect("build fixture");
    read_path_text(output_path)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunOutcome {
    pub exit_code: i32,
    pub stdout: String,
    pub stderr: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComparisonMismatch {
    pub fixture: PathBuf,
    pub harbour_rust: RunOutcome,
    pub harbour: RunOutcome,
}

impl ComparisonMismatch {
    pub fn render(&self, match_stderr: bool) -> String {
        let mut out = format!("comparison failed for {}\n", self.fixture.display());

        if self.harbour_rust.exit_code != self.harbour.exit_code {
            out.push_str(&format!(
                "- exit code mismatch: harbour-rust={} harbour={}\n",
                self.harbour_rust.exit_code, self.harbour.exit_code
            ));
        }
        if self.harbour_rust.stdout != self.harbour.stdout {
            out.push_str("- stdout mismatch\n");
            out.push_str("  harbour-rust:\n");
            out.push_str(&indent_block(&self.harbour_rust.stdout));
            out.push_str("  harbour:\n");
            out.push_str(&indent_block(&self.harbour.stdout));
        }
        if match_stderr && self.harbour_rust.stderr != self.harbour.stderr {
            out.push_str("- stderr mismatch\n");
            out.push_str("  harbour-rust:\n");
            out.push_str(&indent_block(&self.harbour_rust.stderr));
            out.push_str("  harbour:\n");
            out.push_str(&indent_block(&self.harbour.stderr));
        }

        out
    }
}

pub fn run_harbour_rust_fixture(path: &str) -> RunOutcome {
    match run_with_host_compiler(&RunOptions {
        input_path: workspace_path(path),
        include_directories: Vec::new(),
    }) {
        Ok(result) => RunOutcome {
            exit_code: 0,
            stdout: normalize_line_endings(&result.stdout),
            stderr: String::new(),
        },
        Err(error) => RunOutcome {
            exit_code: error.exit_code(),
            stdout: String::new(),
            stderr: normalize_line_endings(&format!("{error}\n")),
        },
    }
}

pub fn run_external_fixture(runner: &Path, fixture: &Path) -> io::Result<RunOutcome> {
    let output = Command::new(runner).arg(fixture).output()?;
    Ok(RunOutcome {
        exit_code: output.status.code().unwrap_or(1),
        stdout: normalize_line_endings(&String::from_utf8_lossy(&output.stdout)),
        stderr: normalize_line_endings(&String::from_utf8_lossy(&output.stderr)),
    })
}

pub fn compare_against_harbour(
    fixture: &Path,
    harbour_rust: RunOutcome,
    harbour: RunOutcome,
    match_stderr: bool,
) -> Result<(), Box<ComparisonMismatch>> {
    let same = harbour_rust.exit_code == harbour.exit_code
        && harbour_rust.stdout == harbour.stdout
        && (!match_stderr || harbour_rust.stderr == harbour.stderr);

    if same {
        Ok(())
    } else {
        Err(Box::new(ComparisonMismatch {
            fixture: fixture.to_path_buf(),
            harbour_rust,
            harbour,
        }))
    }
}

fn normalize_line_endings(text: &str) -> String {
    text.replace("\r\n", "\n")
}

fn indent_block(text: &str) -> String {
    if text.is_empty() {
        return "    <empty>\n".to_owned();
    }

    let mut out = String::new();
    for line in text.lines() {
        out.push_str("    ");
        out.push_str(line);
        out.push('\n');
    }
    if text.ends_with('\n') && text.lines().next_back().is_some_and(|line| line.is_empty()) {
        out.push_str("    \n");
    }
    out
}

#[cfg(test)]
mod tests {
    use super::{
        ComparisonMismatch, RunOutcome, compare_against_harbour, read_workspace_text, run_fixture,
        workspace_path,
    };

    #[test]
    fn workspace_paths_resolve_from_repo_root() {
        let path = workspace_path("examples/hello.prg");
        assert!(path.ends_with("examples/hello.prg"));
        assert!(path.exists(), "expected hello fixture to exist");
    }

    #[test]
    fn run_fixture_executes_hello_program() {
        assert_eq!(run_fixture("examples/hello.prg"), "Hello, world!\n");
    }

    #[test]
    fn reads_workspace_text_from_repo_root() {
        let source = read_workspace_text("examples/hello.prg");
        assert!(source.contains("PROCEDURE Main"));
    }

    #[test]
    fn compare_detects_stdout_mismatch() {
        let mismatch = compare_against_harbour(
            &workspace_path("examples/hello.prg"),
            RunOutcome {
                exit_code: 0,
                stdout: "Hello, world!\n".to_owned(),
                stderr: String::new(),
            },
            RunOutcome {
                exit_code: 0,
                stdout: "Oi\n".to_owned(),
                stderr: String::new(),
            },
            false,
        )
        .expect_err("expected mismatch");

        assert_eq!(
            mismatch,
            Box::new(ComparisonMismatch {
                fixture: workspace_path("examples/hello.prg"),
                harbour_rust: RunOutcome {
                    exit_code: 0,
                    stdout: "Hello, world!\n".to_owned(),
                    stderr: String::new(),
                },
                harbour: RunOutcome {
                    exit_code: 0,
                    stdout: "Oi\n".to_owned(),
                    stderr: String::new(),
                },
            })
        );
    }
}
