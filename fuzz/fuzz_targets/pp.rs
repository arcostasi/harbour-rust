#![no_main]

use std::path::PathBuf;

use harbour_rust_pp::{Preprocessor, SourceFile};
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if let Ok(source) = std::str::from_utf8(data) {
        let file = SourceFile::new(PathBuf::from("fuzz.prg"), source);
        let _ = Preprocessor::default().preprocess(file);
    }
});
