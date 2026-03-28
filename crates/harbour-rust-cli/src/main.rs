fn main() {
    match harbour_rust_cli::run_cli(std::env::args().skip(1)) {
        Ok(message) => {
            if message.ends_with('\n') {
                print!("{message}");
            } else {
                println!("{message}");
            }
        }
        Err(error) => {
            eprintln!("{error}");
            std::process::exit(1);
        }
    }
}
