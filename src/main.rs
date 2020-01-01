extern crate clap;

use clap::{value_t, App, Arg};
use std::io::{self, Write};
use std::path::PathBuf;
use std::process;
use std::time::Instant;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() {
    let now = Instant::now();
    let matches = App::new("nix-test-runner")
        .version(VERSION)
        .author("Christoph H. <schtoeffel@gmail.com>")
        .about("Run nix expression tests.")
        .arg(
            Arg::with_name("TEST")
                .required(true)
                .index(1)
                .help("A nix expression containing testcases."),
        )
        .arg(
            Arg::with_name("reporter")
                .required(false)
                .short("r")
                .long("reporter")
                .default_value("human")
                .possible_values(&nix_test_runner::Reporter::variants())
                .case_insensitive(true),
        )
        .get_matches();
    let test_file = matches.value_of("TEST").unwrap();
    let reporter =
        value_t!(matches, "reporter", nix_test_runner::Reporter).unwrap_or_else(|e| e.exit());
    let test_file_path = PathBuf::from(test_file);
    assert!(
        test_file_path.exists(),
        "You need to provide an existing file."
    );
    match nix_test_runner::run(test_file_path) {
        Ok(result) => {
            let formatted = result.format(now.elapsed(), reporter);
            io::stdout().write_all(formatted.as_bytes()).unwrap();
            process::exit(if result.successful() { 0 } else { 1 })
        }
        Err(err) => {
            io::stderr().write_all(err.as_bytes()).unwrap();
            process::exit(1)
        }
    }
}
