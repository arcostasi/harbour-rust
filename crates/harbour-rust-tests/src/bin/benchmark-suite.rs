use std::{
    path::PathBuf,
    time::{Duration, Instant},
};

use harbour_rust_cli::{
    BuildOptions, CheckOptions, RunOptions, build_to_c, check_source, run_with_host_compiler,
};
use harbour_rust_tests::workspace_path;

fn main() {
    match run() {
        Ok(report) => {
            print!("{report}");
        }
        Err(error) => {
            eprintln!("{error}");
            std::process::exit(1);
        }
    }
}

fn run() -> Result<String, String> {
    let options = parse_options(std::env::args().skip(1))?;
    let fixtures = if options.fixtures.is_empty() {
        vec!["examples/hello.prg".to_owned()]
    } else {
        options.fixtures
    };

    let mut results = Vec::new();
    for fixture in fixtures {
        results.push(benchmark_fixture(&fixture, options.iterations)?);
    }

    Ok(render_markdown(&results, options.iterations))
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Options {
    fixtures: Vec<String>,
    iterations: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct BenchmarkResult {
    fixture: String,
    check_ms: u128,
    transpile_ms: u128,
    run_ms: u128,
}

fn parse_options<I>(args: I) -> Result<Options, String>
where
    I: IntoIterator<Item = String>,
{
    let args = args.into_iter().collect::<Vec<_>>();
    let mut fixtures = Vec::new();
    let mut iterations = 3;
    let mut cursor = 0;

    while cursor < args.len() {
        match args[cursor].as_str() {
            "--fixture" => {
                cursor += 1;
                let Some(value) = args.get(cursor) else {
                    return Err(usage("expected a path after --fixture"));
                };
                fixtures.push(value.clone());
            }
            "--iterations" => {
                cursor += 1;
                let Some(value) = args.get(cursor) else {
                    return Err(usage("expected a value after --iterations"));
                };
                iterations = value
                    .parse::<u32>()
                    .map_err(|_| usage("expected an integer after --iterations"))?;
                if iterations == 0 {
                    return Err(usage("--iterations must be greater than zero"));
                }
            }
            "-h" | "--help" => {
                return Err(usage("help requested"));
            }
            flag => {
                return Err(usage(&format!("unsupported option `{flag}`")));
            }
        }
        cursor += 1;
    }

    Ok(Options {
        fixtures,
        iterations,
    })
}

fn usage(message: &str) -> String {
    format!("{message}\n\nUsage:\n  benchmark-suite [--fixture <path.prg>]... [--iterations <n>]")
}

fn benchmark_fixture(fixture: &str, iterations: u32) -> Result<BenchmarkResult, String> {
    let input_path = workspace_path(fixture);
    let check_ms = average_duration_ms(iterations, || {
        check_source(&CheckOptions {
            input_path: input_path.clone(),
            include_directories: Vec::new(),
        })
        .map(|_| ())
        .map_err(|error| error.to_string())
    })?;

    let transpile_ms = average_duration_ms(iterations, || {
        let output_path = temp_output_path("bench-transpile", "c");
        build_to_c(&BuildOptions {
            input_path: input_path.clone(),
            output_path: Some(output_path.clone()),
            include_directories: Vec::new(),
        })
        .map(|_| ())
        .map_err(|error| error.to_string())?;
        std::fs::remove_file(&output_path).map_err(|error| error.to_string())?;
        Ok(())
    })?;

    let run_ms = average_duration_ms(iterations, || {
        run_with_host_compiler(&RunOptions {
            input_path: input_path.clone(),
            include_directories: Vec::new(),
        })
        .map(|_| ())
        .map_err(|error| error.to_string())
    })?;

    Ok(BenchmarkResult {
        fixture: fixture.to_owned(),
        check_ms,
        transpile_ms,
        run_ms,
    })
}

fn average_duration_ms<F>(iterations: u32, mut run: F) -> Result<u128, String>
where
    F: FnMut() -> Result<(), String>,
{
    let mut total = Duration::ZERO;
    for _ in 0..iterations {
        let start = Instant::now();
        run()?;
        total += start.elapsed();
    }
    Ok(total.as_millis() / u128::from(iterations))
}

fn temp_output_path(label: &str, extension: &str) -> PathBuf {
    let suffix = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("system time")
        .as_nanos();
    std::env::temp_dir().join(format!("harbour-rust-{label}-{suffix}.{extension}"))
}

fn render_markdown(results: &[BenchmarkResult], iterations: u32) -> String {
    let mut out = format!(
        "# harbour-rust benchmark suite\n\nIterations per fixture: {iterations}\n\n| Fixture | check (ms) | transpile (ms) | run (ms) |\n| --- | ---: | ---: | ---: |\n"
    );

    for result in results {
        out.push_str(&format!(
            "| {} | {} | {} | {} |\n",
            result.fixture, result.check_ms, result.transpile_ms, result.run_ms
        ));
    }

    out
}

#[cfg(test)]
mod tests {
    use super::{Options, parse_options, render_markdown};

    #[test]
    fn parses_benchmark_options() {
        let options = parse_options([
            "--fixture".to_owned(),
            "examples/hello.prg".to_owned(),
            "--fixture".to_owned(),
            "tests/fixtures/parser/phase7_acceptance.prg".to_owned(),
            "--iterations".to_owned(),
            "2".to_owned(),
        ])
        .expect("options");

        assert_eq!(
            options,
            Options {
                fixtures: vec![
                    "examples/hello.prg".to_owned(),
                    "tests/fixtures/parser/phase7_acceptance.prg".to_owned(),
                ],
                iterations: 2,
            }
        );
    }

    #[test]
    fn renders_markdown_table() {
        let report = render_markdown(
            &[super::BenchmarkResult {
                fixture: "examples/hello.prg".to_owned(),
                check_ms: 1,
                transpile_ms: 2,
                run_ms: 3,
            }],
            1,
        );

        assert!(report.contains("| examples/hello.prg | 1 | 2 | 3 |"));
    }
}
