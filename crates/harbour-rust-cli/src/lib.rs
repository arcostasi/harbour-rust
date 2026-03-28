use std::{
    fmt, fs,
    path::{Path, PathBuf},
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
                "wrote C output to {}",
                result.output_path.display()
            ))
        }
        _ => Err(CliError::usage("unsupported command")),
    }
}

pub fn build_to_c(options: &BuildOptions) -> Result<BuildResult, CliError> {
    let source = fs::read_to_string(&options.input_path).map_err(|error| CliError {
        message: format!("failed to read {}: {}", options.input_path.display(), error),
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

    let output_path = options
        .output_path
        .clone()
        .unwrap_or_else(|| default_output_path(&options.input_path));

    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent).map_err(|error| CliError {
            message: format!("failed to create {}: {}", parent.display(), error),
        })?;
    }

    fs::write(&output_path, emitted.source).map_err(|error| CliError {
        message: format!("failed to write {}: {}", output_path.display(), error),
    })?;

    Ok(BuildResult { output_path })
}

pub fn usage() -> &'static str {
    "Usage:\n  harbour-rust-cli build <input.prg> [--out <output.c>]"
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

fn default_output_path(input_path: &Path) -> PathBuf {
    input_path.with_extension("c")
}

fn render_stage_errors(stage: &str, errors: Vec<String>) -> String {
    let mut message = format!("{} failed", stage);
    for error in errors {
        message.push_str("\n- ");
        message.push_str(&error);
    }
    message
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::{BuildOptions, default_output_path, parse_build_options, run_cli};

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
}
