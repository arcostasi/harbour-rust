#![no_main]

use harbour_rust_lexer::lex;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if let Ok(source) = std::str::from_utf8(data) {
        let _ = lex(source);
    }
});
