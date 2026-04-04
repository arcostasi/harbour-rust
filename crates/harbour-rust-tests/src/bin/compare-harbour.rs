use std::{
    path::PathBuf,
    process,
};

use harbour_rust_tests::{
    compare_against_harbour, run_external_fixture, run_harbour_rust_fixture, workspace_path,
};

fn main() {
    match run() {
        Ok(message) => {
            println!("{message}");
        }
        Err(error) => {
            eprintln!("{error}");
            process::exit(1);
        }
    }
}

fn run() -> Result<String, String> {
    let options = parse_options(std::env::args().skip(1))?;
    let fixture = workspace_path(&options.fixture);
    let harbour_rust = run_harbour_rust_fixture(&options.fixture);
    let harbour = run_external_fixture(&options.harbour_runner, &fixture).map_err(|error| {
        format!(
            "failed to invoke harbour runner `{}`: {}",
            options.harbour_runner.display(),
            error
        )
    })?;

    compare_against_harbour(&fixture, harbour_rust, harbour, options.match_stderr)
        .map(|()| format!("matched harbour-rust and harbour for {}", fixture.display()))
        .map_err(|mismatch| mismatch.render(options.match_stderr))
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Options {
    fixture: String,
    harbour_runner: PathBuf,
    match_stderr: bool,
}

fn parse_options<I>(args: I) -> Result<Options, String>
where
    I: IntoIterator<Item = String>,
{
    let args = args.into_iter().collect::<Vec<_>>();
    if args.is_empty() {
        return Err(usage("expected arguments"));
    }
    if args.len() == 1 && matches!(args[0].as_str(), "-h" | "--help") {
        return Ok(Options {
            fixture: String::new(),
            harbour_runner: PathBuf::new(),
            match_stderr: false,
        });
    }

    let mut fixture = None;
    let mut harbour_runner = None;
    let mut match_stderr = false;
    let mut cursor = 0;

    while cursor < args.len() {
        match args[cursor].as_str() {
            "--fixture" => {
                cursor += 1;
                let Some(value) = args.get(cursor) else {
                    return Err(usage("expected a path after --fixture"));
                };
                fixture = Some(value.clone());
            }
            "--harbour-runner" => {
                cursor += 1;
                let Some(value) = args.get(cursor) else {
                    return Err(usage("expected a path after --harbour-runner"));
                };
                harbour_runner = Some(PathBuf::from(value));
            }
            "--match-stderr" => {
                match_stderr = true;
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

    let Some(fixture) = fixture else {
        return Err(usage("missing --fixture"));
    };
    let Some(harbour_runner) = harbour_runner else {
        return Err(usage("missing --harbour-runner"));
    };

    Ok(Options {
        fixture,
        harbour_runner,
        match_stderr,
    })
}

fn usage(message: &str) -> String {
    format!(
        "{message}\n\nUsage:\n  compare-harbour --fixture <path.prg> --harbour-runner <runner> [--match-stderr]"
    )
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::{Options, parse_options};

    #[test]
    fn parses_minimal_compare_options() {
        let options = parse_options([
            "--fixture".to_owned(),
            "examples/hello.prg".to_owned(),
            "--harbour-runner".to_owned(),
            "C:/tools/harbour-runner.exe".to_owned(),
        ])
        .expect("options");

        assert_eq!(
            options,
            Options {
                fixture: "examples/hello.prg".to_owned(),
                harbour_runner: PathBuf::from("C:/tools/harbour-runner.exe"),
                match_stderr: false,
            }
        );
    }
}
