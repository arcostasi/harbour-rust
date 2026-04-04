use std::{fs, io::ErrorKind, path::PathBuf};

pub fn workspace_fixture(path: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join(path)
}

pub fn read_upstream_or_skip(path: &str, label: &str) -> Option<String> {
    let full_path = workspace_fixture(path);
    match fs::read_to_string(&full_path) {
        Ok(content) => Some(content),
        Err(error) if error.kind() == ErrorKind::NotFound => {
            eprintln!(
                "skipping compatibility oracle `{label}` because {} is not available",
                full_path.display()
            );
            None
        }
        Err(error) => panic!("{label}: {error}"),
    }
}
