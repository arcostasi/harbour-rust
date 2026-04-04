use std::{
    fs,
    path::{Path, PathBuf},
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

#[cfg(test)]
mod tests {
    use super::{read_workspace_text, run_fixture, workspace_path};

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
}
