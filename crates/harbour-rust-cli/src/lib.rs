use std::{
    env, fmt, fs,
    path::{Path, PathBuf},
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

use harbour_rust_codegen_c::emit_program;
use harbour_rust_hir::lower_program as lower_hir_program;
use harbour_rust_ir::lower_program as lower_ir_program;
use harbour_rust_parser::parse;
use harbour_rust_sema::analyze_program;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BuildOptions {
    pub input_path: PathBuf,
    pub output_path: Option<PathBuf>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BuildResult {
    pub output_path: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunOptions {
    pub input_path: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunResult {
    pub stdout: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CliError {
    pub message: String,
}

impl CliError {
    fn usage(message: &str) -> Self {
        Self {
            message: format!("{message}\n\n{}", usage()),
        }
    }
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for CliError {}

const RUNTIME_SUPPORT_HEADER: &str = include_str!("../support/runtime_support.h");
const RUNTIME_SUPPORT_C: &str = include_str!("../support/runtime_support.c");

pub fn run_cli<I>(args: I) -> Result<String, CliError>
where
    I: IntoIterator<Item = String>,
{
    let args = args.into_iter().collect::<Vec<_>>();
    let Some(command) = args.first().map(String::as_str) else {
        return Err(CliError::usage("expected a command"));
    };

    match command {
        "build" => {
            let options = parse_build_options(&args[1..])?;
            let result = build_to_c(&options)?;
            Ok(format!(
                "wrote C output to {}\n",
                result.output_path.display()
            ))
        }
        "run" => {
            let options = parse_run_options(&args[1..])?;
            let result = run_with_host_compiler(&options)?;
            Ok(result.stdout)
        }
        _ => Err(CliError::usage("unsupported command")),
    }
}

pub fn build_to_c(options: &BuildOptions) -> Result<BuildResult, CliError> {
    let emitted = compile_to_c_source(&options.input_path)?;

    let output_path = options
        .output_path
        .clone()
        .unwrap_or_else(|| default_output_path(&options.input_path));

    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent).map_err(|error| CliError {
            message: format!("failed to create {}: {}", parent.display(), error),
        })?;
    }

    fs::write(&output_path, emitted).map_err(|error| CliError {
        message: format!("failed to write {}: {}", output_path.display(), error),
    })?;

    Ok(BuildResult { output_path })
}

pub fn run_with_host_compiler(options: &RunOptions) -> Result<RunResult, CliError> {
    let compilers = detect_host_compilers()?;
    let generated_c = compile_to_c_source(&options.input_path)?;
    let temp_dir = unique_temp_dir("run");
    fs::create_dir_all(&temp_dir).map_err(|error| CliError {
        message: format!("failed to create {}: {}", temp_dir.display(), error),
    })?;

    let c_path = temp_dir.join("program.c");
    let header_path = temp_dir.join("runtime_support.h");
    let runtime_c_path = temp_dir.join("runtime_support.c");
    let output_path = temp_dir.join(executable_name("harbour_program"));

    fs::write(&c_path, generated_c).map_err(|error| CliError {
        message: format!("failed to write {}: {}", c_path.display(), error),
    })?;
    fs::write(&header_path, RUNTIME_SUPPORT_HEADER).map_err(|error| CliError {
        message: format!("failed to write {}: {}", header_path.display(), error),
    })?;
    fs::write(&runtime_c_path, RUNTIME_SUPPORT_C).map_err(|error| CliError {
        message: format!("failed to write {}: {}", runtime_c_path.display(), error),
    })?;

    let mut last_error = None;

    for compiler in compilers {
        let compile_output = compiler
            .to_command(&c_path, &runtime_c_path, &header_path, &output_path)
            .output()
            .map_err(|error| CliError {
                message: format!(
                    "failed to invoke host compiler `{}`: {}",
                    compiler.executable, error
                ),
            })?;

        if !compile_output.status.success() {
            let stderr = String::from_utf8_lossy(&compile_output.stderr);
            let stdout = String::from_utf8_lossy(&compile_output.stdout);
            last_error = Some(format!(
                "host C compilation failed with `{}`\nstdout:\n{}\nstderr:\n{}",
                compiler.executable, stdout, stderr
            ));
            continue;
        }

        let run_output = Command::new(&output_path)
            .output()
            .map_err(|error| CliError {
                message: format!("failed to run {}: {}", output_path.display(), error),
            })?;

        let stdout = String::from_utf8(run_output.stdout).map_err(|error| CliError {
            message: format!("program stdout was not valid UTF-8: {}", error),
        })?;
        let stdout = stdout.replace("\r\n", "\n");
        let stderr = String::from_utf8_lossy(&run_output.stderr);
        let _ = fs::remove_dir_all(&temp_dir);

        if !run_output.status.success() {
            return Err(CliError {
                message: format!(
                    "generated program exited with status {}\nstderr:\n{}",
                    run_output.status, stderr
                ),
            });
        }

        return Ok(RunResult { stdout });
    }

    let _ = fs::remove_dir_all(&temp_dir);
    Err(CliError {
        message: last_error.unwrap_or_else(|| "host C compilation failed".to_owned()),
    })
}

pub fn usage() -> &'static str {
    "Usage:\n  harbour-rust-cli build <input.prg> [--out <output.c>]\n  harbour-rust-cli run <input.prg>"
}

fn parse_build_options(args: &[String]) -> Result<BuildOptions, CliError> {
    let Some(input) = args.first() else {
        return Err(CliError::usage("build requires an input .prg file"));
    };

    let mut output_path = None;
    let mut cursor = 1;

    while cursor < args.len() {
        match args[cursor].as_str() {
            "--out" => {
                cursor += 1;
                let Some(path) = args.get(cursor) else {
                    return Err(CliError::usage("expected a path after --out"));
                };
                output_path = Some(PathBuf::from(path));
            }
            flag => {
                return Err(CliError::usage(&format!(
                    "unsupported build option `{flag}`"
                )));
            }
        }
        cursor += 1;
    }

    Ok(BuildOptions {
        input_path: PathBuf::from(input),
        output_path,
    })
}

fn parse_run_options(args: &[String]) -> Result<RunOptions, CliError> {
    let Some(input) = args.first() else {
        return Err(CliError::usage("run requires an input .prg file"));
    };

    if args.len() > 1 {
        return Err(CliError::usage("run does not accept extra options yet"));
    }

    Ok(RunOptions {
        input_path: PathBuf::from(input),
    })
}

fn default_output_path(input_path: &Path) -> PathBuf {
    input_path.with_extension("c")
}

fn compile_to_c_source(input_path: &Path) -> Result<String, CliError> {
    let source = fs::read_to_string(input_path).map_err(|error| CliError {
        message: format!("failed to read {}: {}", input_path.display(), error),
    })?;

    let parsed = parse(&source);
    if !parsed.errors.is_empty() {
        return Err(CliError {
            message: render_stage_errors(
                "parse",
                parsed.errors.iter().map(ToString::to_string).collect(),
            ),
        });
    }

    let hir = lower_hir_program(&parsed.program);
    if !hir.errors.is_empty() {
        return Err(CliError {
            message: render_stage_errors(
                "hir lowering",
                hir.errors.iter().map(ToString::to_string).collect(),
            ),
        });
    }

    let sema = analyze_program(&hir.program);
    if !sema.errors.is_empty() {
        return Err(CliError {
            message: render_stage_errors(
                "semantic analysis",
                sema.errors.iter().map(ToString::to_string).collect(),
            ),
        });
    }

    let ir = lower_ir_program(&hir.program);
    if !ir.errors.is_empty() {
        return Err(CliError {
            message: render_stage_errors(
                "ir lowering",
                ir.errors.iter().map(ToString::to_string).collect(),
            ),
        });
    }

    let emitted = emit_program(&ir.program);
    if !emitted.errors.is_empty() {
        return Err(CliError {
            message: render_stage_errors(
                "codegen-c",
                emitted.errors.iter().map(ToString::to_string).collect(),
            ),
        });
    }

    Ok(emitted.source)
}

fn render_stage_errors(stage: &str, errors: Vec<String>) -> String {
    let mut message = format!("{} failed", stage);
    for error in errors {
        message.push_str("\n- ");
        message.push_str(&error);
    }
    message
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct HostCompiler {
    executable: String,
}

impl HostCompiler {
    fn to_command(
        &self,
        c_path: &Path,
        runtime_c_path: &Path,
        header_path: &Path,
        output_path: &Path,
    ) -> Command {
        let mut command = Command::new(&self.executable);
        command
            .arg("-std=c99")
            .arg("-include")
            .arg(header_path)
            .arg(c_path)
            .arg(runtime_c_path)
            .arg("-o")
            .arg(output_path);
        command
    }
}

fn detect_host_compilers() -> Result<Vec<HostCompiler>, CliError> {
    let mut compilers = Vec::new();

    for executable in ["gcc", "cc", "clang"] {
        if compiler_available(executable) {
            compilers.push(HostCompiler {
                executable: executable.to_owned(),
            });
        }
    }

    if compilers.is_empty() {
        Err(CliError {
            message: "no supported host C compiler found; expected one of: gcc, cc, clang"
                .to_owned(),
        })
    } else {
        Ok(compilers)
    }
}

fn compiler_available(executable: &str) -> bool {
    Command::new(executable)
        .arg("--version")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

fn unique_temp_dir(label: &str) -> PathBuf {
    let suffix = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time")
        .as_nanos();
    env::temp_dir().join(format!("harbour-rust-cli-{label}-{suffix}"))
}

fn executable_name(stem: &str) -> String {
    if cfg!(windows) {
        format!("{stem}.exe")
    } else {
        stem.to_owned()
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::{
        BuildOptions, default_output_path, executable_name, parse_build_options, parse_run_options,
        run_cli,
    };

    #[test]
    fn build_options_use_c_extension_by_default() {
        assert_eq!(
            default_output_path(PathBuf::from("examples/hello.prg").as_path()),
            PathBuf::from("examples/hello.c")
        );
    }

    #[test]
    fn parses_build_options_with_explicit_output() {
        let options = parse_build_options(&[
            "examples/hello.prg".to_owned(),
            "--out".to_owned(),
            "target/hello.c".to_owned(),
        ])
        .expect("build options");

        assert_eq!(
            options,
            BuildOptions {
                input_path: PathBuf::from("examples/hello.prg"),
                output_path: Some(PathBuf::from("target/hello.c")),
            }
        );
    }

    #[test]
    fn run_cli_reports_usage_for_missing_command() {
        let error = run_cli(Vec::<String>::new()).expect_err("missing command should fail");
        assert!(error.message.contains("expected a command"));
        assert!(error.message.contains("Usage:"));
    }

    #[test]
    fn parses_run_options() {
        let options = parse_run_options(&["examples/hello.prg".to_owned()]).expect("run options");
        assert_eq!(
            options,
            crate::RunOptions {
                input_path: PathBuf::from("examples/hello.prg"),
            }
        );
    }

    #[test]
    fn executable_name_adds_platform_suffix() {
        let executable = executable_name("harbour_program");
        if cfg!(windows) {
            assert_eq!(executable, "harbour_program.exe");
        } else {
            assert_eq!(executable, "harbour_program");
        }
    }
}
