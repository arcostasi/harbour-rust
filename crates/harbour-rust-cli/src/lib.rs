use std::{
    env, fmt, fs,
    path::{Path, PathBuf},
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

use harbour_rust_codegen_c::emit_program;
use harbour_rust_hir::lower_program as lower_hir_program;
use harbour_rust_ir::lower_program as lower_ir_program;
use harbour_rust_parser::{ParseOutput, parse};
use harbour_rust_pp::{FileSystemIncludeResolver, PreprocessOutput, Preprocessor, SourceFile};
use harbour_rust_sema::analyze_program;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BuildOptions {
    pub input_path: PathBuf,
    pub output_path: Option<PathBuf>,
    pub include_directories: Vec<PathBuf>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BuildResult {
    pub output_path: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckOptions {
    pub input_path: PathBuf,
    pub include_directories: Vec<PathBuf>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckResult {
    pub input_path: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunOptions {
    pub input_path: PathBuf,
    pub include_directories: Vec<PathBuf>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunResult {
    pub stdout: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TranspileOptions {
    pub input_path: PathBuf,
    pub output_path: Option<PathBuf>,
    pub include_directories: Vec<PathBuf>,
    pub target: TranspileTarget,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TranspileTarget {
    C,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CliError {
    pub message: String,
    pub exit_code: i32,
}

impl CliError {
    fn usage(message: &str) -> Self {
        Self {
            message: format!("{message}\n\n{}", usage()),
            exit_code: 1,
        }
    }

    fn with_exit_code(message: impl Into<String>, exit_code: i32) -> Self {
        Self {
            message: message.into(),
            exit_code,
        }
    }

    fn input_error(path: &Path, error: impl fmt::Display) -> Self {
        Self::with_exit_code(format!("B001 failed to read {}: {}", path.display(), error), 1)
    }

    fn stage_failure(stage: &str, path: &Path, errors: Vec<String>) -> Self {
        let mut message = format!("{} failed for {}", stage, path.display());
        for error in errors {
            message.push_str("\n- ");
            message.push_str(&error);
        }
        Self::with_exit_code(message, 1)
    }

    fn codegen_failure(path: &Path, errors: Vec<String>) -> Self {
        let mut message = format!("codegen-c failed for {}", path.display());
        for error in errors {
            message.push_str("\n- ");
            message.push_str(&error);
        }
        Self::with_exit_code(message, 2)
    }

    fn output_error(path: &Path, error: impl fmt::Display) -> Self {
        Self::with_exit_code(format!("failed to write {}: {}", path.display(), error), 1)
    }

    pub fn exit_code(&self) -> i32 {
        self.exit_code
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

    if matches!(command, "help" | "-h" | "--help") {
        return render_help(args.get(1).map(String::as_str));
    }

    match command {
        "build" => {
            if args.get(1).is_some_and(|arg| is_help_flag(arg)) {
                return Ok(build_help().to_owned());
            }
            let options = parse_build_options(&args[1..])?;
            let result = build_to_c(&options)?;
            Ok(format!(
                "wrote C output to {}\n",
                result.output_path.display()
            ))
        }
        "check" => {
            if args.get(1).is_some_and(|arg| is_help_flag(arg)) {
                return Ok(check_help().to_owned());
            }
            let options = parse_check_options(&args[1..])?;
            let result = check_source(&options)?;
            Ok(format!(
                "check succeeded for {}\n",
                result.input_path.display()
            ))
        }
        "run" => {
            if args.get(1).is_some_and(|arg| is_help_flag(arg)) {
                return Ok(run_help().to_owned());
            }
            let options = parse_run_options(&args[1..])?;
            let result = run_with_host_compiler(&options)?;
            Ok(result.stdout)
        }
        "transpile" => {
            if args.get(1).is_some_and(|arg| is_help_flag(arg)) {
                return Ok(transpile_help().to_owned());
            }
            let options = parse_transpile_options(&args[1..])?;
            let result = transpile(&options)?;
            Ok(format!(
                "wrote C output to {}\n",
                result.output_path.display()
            ))
        }
        _ => Err(CliError::usage("unsupported command")),
    }
}

pub fn build_to_c(options: &BuildOptions) -> Result<BuildResult, CliError> {
    let emitted =
        compile_to_c_source_with_options(&options.input_path, &options.include_directories)?;

    let output_path = options
        .output_path
        .clone()
        .unwrap_or_else(|| default_output_path(&options.input_path));

    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent).map_err(|error| CliError::output_error(parent, error))?;
    }

    fs::write(&output_path, emitted).map_err(|error| CliError::output_error(&output_path, error))?;

    Ok(BuildResult { output_path })
}

pub fn check_source(options: &CheckOptions) -> Result<CheckResult, CliError> {
    let _ = analyze_source_with_options(&options.input_path, &options.include_directories)?;
    Ok(CheckResult {
        input_path: options.input_path.clone(),
    })
}

pub fn run_with_host_compiler(options: &RunOptions) -> Result<RunResult, CliError> {
    let compilers = detect_host_compilers()?;
    let generated_c =
        compile_to_c_source_with_options(&options.input_path, &options.include_directories)?;
    let temp_dir = unique_temp_dir("run");
    fs::create_dir_all(&temp_dir).map_err(|error| CliError::output_error(&temp_dir, error))?;

    let c_path = temp_dir.join("program.c");
    let header_path = temp_dir.join("runtime_support.h");
    let runtime_c_path = temp_dir.join("runtime_support.c");
    let output_path = temp_dir.join(executable_name("harbour_program"));

    fs::write(&c_path, generated_c).map_err(|error| CliError::output_error(&c_path, error))?;
    fs::write(&header_path, RUNTIME_SUPPORT_HEADER)
        .map_err(|error| CliError::output_error(&header_path, error))?;
    fs::write(&runtime_c_path, RUNTIME_SUPPORT_C)
        .map_err(|error| CliError::output_error(&runtime_c_path, error))?;

    let mut last_error = None;

    for compiler in compilers {
        let compile_output = compiler
            .to_command(&c_path, &runtime_c_path, &header_path, &output_path)
            .output()
            .map_err(|error| CliError::with_exit_code(
                format!(
                    "failed to invoke host compiler `{}`: {}",
                    compiler.executable, error
                ),
                3,
            ))?;

        if !compile_output.status.success() {
            let stderr = String::from_utf8_lossy(&compile_output.stderr);
            let stdout = String::from_utf8_lossy(&compile_output.stdout);
            last_error = Some(format!(
                "B003 host C compilation failed with `{}`\nstdout:\n{}\nstderr:\n{}",
                compiler.executable, stdout, stderr
            ));
            continue;
        }

        let run_output = Command::new(&output_path)
            .output()
            .map_err(|error| CliError::with_exit_code(
                format!("failed to run {}: {}", output_path.display(), error),
                3,
            ))?;

        let stdout = String::from_utf8(run_output.stdout)
            .map_err(|error| CliError::with_exit_code(
                format!("program stdout was not valid UTF-8: {}", error),
                3,
            ))?;
        let stdout = stdout.replace("\r\n", "\n");
        let stderr = String::from_utf8_lossy(&run_output.stderr);
        let _ = fs::remove_dir_all(&temp_dir);

        if !run_output.status.success() {
            return Err(CliError::with_exit_code(
                format!(
                    "generated program exited with status {}\nstderr:\n{}",
                    run_output.status, stderr
                ),
                run_output.status.code().unwrap_or(1),
            ));
        }

        return Ok(RunResult { stdout });
    }

    let _ = fs::remove_dir_all(&temp_dir);
    Err(CliError::with_exit_code(
        last_error.unwrap_or_else(|| "B003 host C compilation failed".to_owned()),
        3,
    ))
}

pub fn usage() -> &'static str {
    "Usage:\n  harbour-rust-cli help [command]\n  harbour-rust-cli build <input.prg> [--out <output.c>] [-I <dir> | --include-dir <dir>]...\n  harbour-rust-cli check <input.prg> [-I <dir> | --include-dir <dir>]...\n  harbour-rust-cli run <input.prg> [-I <dir> | --include-dir <dir>]...\n  harbour-rust-cli transpile --to c <input.prg> [--out <output.c>] [-I <dir> | --include-dir <dir>]..."
}

pub fn build_help() -> &'static str {
    "Usage:\n  harbour-rust-cli build <input.prg> [--out <output.c>] [-I <dir> | --include-dir <dir>]...\n\nBuilds a .prg file to generated C output."
}

pub fn check_help() -> &'static str {
    "Usage:\n  harbour-rust-cli check <input.prg> [-I <dir> | --include-dir <dir>]...\n\nRuns preprocess, parse, HIR lowering and semantic analysis without generating code."
}

pub fn run_help() -> &'static str {
    "Usage:\n  harbour-rust-cli run <input.prg> [-I <dir> | --include-dir <dir>]...\n\nBuilds, compiles with a host C compiler and executes the resulting program."
}

pub fn transpile_help() -> &'static str {
    "Usage:\n  harbour-rust-cli transpile --to c <input.prg> [--out <output.c>] [-I <dir> | --include-dir <dir>]...\n\nTranspiles a .prg file to C without invoking the host C compiler."
}

fn render_help(command: Option<&str>) -> Result<String, CliError> {
    match command {
        None => Ok(usage().to_owned()),
        Some("build") => Ok(build_help().to_owned()),
        Some("check") => Ok(check_help().to_owned()),
        Some("run") => Ok(run_help().to_owned()),
        Some("transpile") => Ok(transpile_help().to_owned()),
        Some(other) => Err(CliError::usage(&format!(
            "unsupported help topic `{other}`"
        ))),
    }
}

fn parse_build_options(args: &[String]) -> Result<BuildOptions, CliError> {
    let Some(input) = args.first() else {
        return Err(CliError::usage("build requires an input .prg file"));
    };

    let mut output_path = None;
    let mut include_directories = Vec::new();
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
            "-I" | "--include-dir" => {
                cursor += 1;
                let Some(path) = args.get(cursor) else {
                    return Err(CliError::usage(
                        "expected a path after include directory option",
                    ));
                };
                include_directories.push(PathBuf::from(path));
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
        include_directories,
    })
}

fn parse_check_options(args: &[String]) -> Result<CheckOptions, CliError> {
    let Some(input) = args.first() else {
        return Err(CliError::usage("check requires an input .prg file"));
    };

    let mut include_directories = Vec::new();
    let mut cursor = 1;
    while cursor < args.len() {
        match args[cursor].as_str() {
            "-I" | "--include-dir" => {
                cursor += 1;
                let Some(path) = args.get(cursor) else {
                    return Err(CliError::usage(
                        "expected a path after include directory option",
                    ));
                };
                include_directories.push(PathBuf::from(path));
            }
            flag => {
                return Err(CliError::usage(&format!("unsupported check option `{flag}`")));
            }
        }
        cursor += 1;
    }

    Ok(CheckOptions {
        input_path: PathBuf::from(input),
        include_directories,
    })
}

fn parse_run_options(args: &[String]) -> Result<RunOptions, CliError> {
    let Some(input) = args.first() else {
        return Err(CliError::usage("run requires an input .prg file"));
    };

    let mut include_directories = Vec::new();
    let mut cursor = 1;
    while cursor < args.len() {
        match args[cursor].as_str() {
            "-I" | "--include-dir" => {
                cursor += 1;
                let Some(path) = args.get(cursor) else {
                    return Err(CliError::usage(
                        "expected a path after include directory option",
                    ));
                };
                include_directories.push(PathBuf::from(path));
            }
            flag => {
                return Err(CliError::usage(&format!("unsupported run option `{flag}`")));
            }
        }
        cursor += 1;
    }

    Ok(RunOptions {
        input_path: PathBuf::from(input),
        include_directories,
    })
}

fn parse_transpile_options(args: &[String]) -> Result<TranspileOptions, CliError> {
    if args.is_empty() {
        return Err(CliError::usage("transpile requires --to c and an input .prg file"));
    }

    let mut output_path = None;
    let mut include_directories = Vec::new();
    let mut target = None;
    let mut input_path = None;
    let mut cursor = 0;

    while cursor < args.len() {
        match args[cursor].as_str() {
            "--to" => {
                cursor += 1;
                let Some(raw_target) = args.get(cursor) else {
                    return Err(CliError::usage("expected a target after --to"));
                };
                target = Some(parse_transpile_target(raw_target)?);
            }
            "--out" => {
                cursor += 1;
                let Some(path) = args.get(cursor) else {
                    return Err(CliError::usage("expected a path after --out"));
                };
                output_path = Some(PathBuf::from(path));
            }
            "-I" | "--include-dir" => {
                cursor += 1;
                let Some(path) = args.get(cursor) else {
                    return Err(CliError::usage(
                        "expected a path after include directory option",
                    ));
                };
                include_directories.push(PathBuf::from(path));
            }
            value if value.starts_with('-') => {
                return Err(CliError::usage(&format!(
                    "unsupported transpile option `{value}`"
                )));
            }
            value => {
                if input_path.is_some() {
                    return Err(CliError::usage(
                        "transpile accepts a single input .prg file",
                    ));
                }
                input_path = Some(PathBuf::from(value));
            }
        }
        cursor += 1;
    }

    let Some(target) = target else {
        return Err(CliError::usage("transpile requires --to c"));
    };
    let Some(input_path) = input_path else {
        return Err(CliError::usage("transpile requires an input .prg file"));
    };

    Ok(TranspileOptions {
        input_path,
        output_path,
        include_directories,
        target,
    })
}

fn parse_transpile_target(raw_target: &str) -> Result<TranspileTarget, CliError> {
    match raw_target {
        "c" => Ok(TranspileTarget::C),
        other => Err(CliError::usage(&format!(
            "unsupported transpile target `{other}`"
        ))),
    }
}

fn default_output_path(input_path: &Path) -> PathBuf {
    input_path.with_extension("c")
}

pub fn transpile(options: &TranspileOptions) -> Result<BuildResult, CliError> {
    match options.target {
        TranspileTarget::C => build_to_c(&BuildOptions {
            input_path: options.input_path.clone(),
            output_path: options.output_path.clone(),
            include_directories: options.include_directories.clone(),
        }),
    }
}

fn compile_to_c_source_with_options(
    input_path: &Path,
    include_directories: &[PathBuf],
) -> Result<String, CliError> {
    let analysis = analyze_source_with_options(input_path, include_directories)?;

    let ir = lower_ir_program(&analysis.hir.program);
    if !ir.errors.is_empty() {
        return Err(CliError::stage_failure(
            "ir lowering",
            input_path,
            ir.errors.iter().map(ToString::to_string).collect(),
        ));
    }

    let emitted = emit_program(&ir.program);
    if !emitted.errors.is_empty() {
        return Err(CliError::codegen_failure(
            input_path,
            emitted.errors.iter().map(ToString::to_string).collect(),
        ));
    }

    Ok(emitted.source)
}

fn analyze_source_with_options(
    input_path: &Path,
    include_directories: &[PathBuf],
) -> Result<AnalyzedProgram, CliError> {
    let handoff = preprocess_and_parse(input_path, include_directories)?;
    if !handoff.parsed.errors.is_empty() {
        return Err(CliError::stage_failure(
            "parse",
            input_path,
            handoff.parsed.errors.iter().map(ToString::to_string).collect(),
        ));
    }

    let hir = lower_hir_program(&handoff.parsed.program);
    if !hir.errors.is_empty() {
        return Err(CliError::stage_failure(
            "hir lowering",
            input_path,
            hir.errors.iter().map(ToString::to_string).collect(),
        ));
    }

    let sema = analyze_program(&hir.program);
    if !sema.errors.is_empty() {
        return Err(CliError::stage_failure(
            "semantic analysis",
            input_path,
            sema.errors.iter().map(ToString::to_string).collect(),
        ));
    }

    Ok(AnalyzedProgram { handoff, hir })
}

fn preprocess_and_parse(
    input_path: &Path,
    include_directories: &[PathBuf],
) -> Result<PreprocessHandoff, CliError> {
    let source = SourceFile::from_path(input_path).map_err(|error| CliError::input_error(input_path, error))?;

    let resolver = include_directories.iter().fold(
        FileSystemIncludeResolver::new(),
        |resolver, include_directory| resolver.with_search_path(include_directory.clone()),
    );
    let preprocessed = Preprocessor::new(resolver).preprocess(source);
    if !preprocessed.errors.is_empty() {
        return Err(CliError::stage_failure(
            "preprocess",
            input_path,
            preprocessed
                .errors
                .iter()
                .map(ToString::to_string)
                .collect(),
        ));
    }

    let parsed = parse(&preprocessed.text);
    Ok(PreprocessHandoff {
        preprocessed,
        parsed,
    })
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct PreprocessHandoff {
    preprocessed: PreprocessOutput,
    parsed: ParseOutput,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct AnalyzedProgram {
    handoff: PreprocessHandoff,
    hir: harbour_rust_hir::LoweringOutput,
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
            message: "B002 no supported host C compiler found; expected one of: gcc, cc, clang"
                .to_owned(),
            exit_code: 3,
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

fn is_help_flag(argument: &str) -> bool {
    matches!(argument, "-h" | "--help")
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
        BuildOptions, CheckOptions, TranspileOptions, TranspileTarget, build_help, check_help,
        default_output_path, executable_name, parse_build_options, parse_check_options,
        parse_run_options, parse_transpile_options, run_cli, run_help, transpile_help, usage,
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
                include_directories: Vec::new(),
            }
        );
    }

    #[test]
    fn parses_build_options_with_include_directories() {
        let options = parse_build_options(&[
            "examples/hello.prg".to_owned(),
            "-I".to_owned(),
            "tests/fixtures/pp/include-path".to_owned(),
            "--include-dir".to_owned(),
            "tests/includes".to_owned(),
        ])
        .expect("build options");

        assert_eq!(
            options,
            BuildOptions {
                input_path: PathBuf::from("examples/hello.prg"),
                output_path: None,
                include_directories: vec![
                    PathBuf::from("tests/fixtures/pp/include-path"),
                    PathBuf::from("tests/includes"),
                ],
            }
        );
    }

    #[test]
    fn run_cli_reports_usage_for_missing_command() {
        let error = run_cli(Vec::<String>::new()).expect_err("missing command should fail");
        assert!(error.message.contains("expected a command"));
        assert!(error.message.contains("Usage:"));
        assert_eq!(error.exit_code(), 1);
    }

    #[test]
    fn parses_run_options() {
        let options = parse_run_options(&["examples/hello.prg".to_owned()]).expect("run options");
        assert_eq!(
            options,
            crate::RunOptions {
                input_path: PathBuf::from("examples/hello.prg"),
                include_directories: Vec::new(),
            }
        );
    }

    #[test]
    fn parses_run_options_with_include_directories() {
        let options = parse_run_options(&[
            "examples/hello.prg".to_owned(),
            "--include-dir".to_owned(),
            "tests/fixtures/pp/include-path".to_owned(),
        ])
        .expect("run options");

        assert_eq!(
            options,
            crate::RunOptions {
                input_path: PathBuf::from("examples/hello.prg"),
                include_directories: vec![PathBuf::from("tests/fixtures/pp/include-path")],
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

    #[test]
    fn parses_check_options_with_include_directories() {
        let options = parse_check_options(&[
            "examples/hello.prg".to_owned(),
            "-I".to_owned(),
            "tests/fixtures/pp/include-path".to_owned(),
        ])
        .expect("check options");

        assert_eq!(
            options,
            CheckOptions {
                input_path: PathBuf::from("examples/hello.prg"),
                include_directories: vec![PathBuf::from("tests/fixtures/pp/include-path")],
            }
        );
    }

    #[test]
    fn parses_transpile_options_for_c_target() {
        let options = parse_transpile_options(&[
            "--to".to_owned(),
            "c".to_owned(),
            "examples/hello.prg".to_owned(),
            "--out".to_owned(),
            "target/hello.c".to_owned(),
        ])
        .expect("transpile options");

        assert_eq!(
            options,
            TranspileOptions {
                input_path: PathBuf::from("examples/hello.prg"),
                output_path: Some(PathBuf::from("target/hello.c")),
                include_directories: Vec::new(),
                target: TranspileTarget::C,
            }
        );
    }

    #[test]
    fn help_commands_render_expected_usage() {
        assert!(usage().contains("harbour-rust-cli check"));
        assert!(build_help().contains("build <input.prg>"));
        assert!(check_help().contains("check <input.prg>"));
        assert!(run_help().contains("run <input.prg>"));
        assert!(transpile_help().contains("transpile --to c"));
        assert_eq!(
            run_cli(["help".to_owned(), "check".to_owned()]).expect("help check"),
            check_help()
        );
        assert_eq!(
            run_cli(["--help".to_owned()]).expect("top-level help"),
            usage()
        );
    }
}
