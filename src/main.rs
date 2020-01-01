extern crate clap;

use clap::{value_t, App, Arg};
use std::io::{self, Write};
use std::path::PathBuf;
use std::process;
use std::time::Instant;

const NAME: &'static str = env!("CARGO_PKG_NAME");
const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const AUTHOR: &'static str = env!("CARGO_PKG_AUTHORS");
const DESCRIPTION: &'static str = env!("CARGO_PKG_DESCRIPTION");

fn main() {
    let now = Instant::now();
    let matches = App::new(NAME)
        .version(VERSION)
        .author(AUTHOR)
        .about(DESCRIPTION)
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
    let reporter = value_t!(matches, "reporter", nix_test_runner::Reporter).unwrap();
    let test_file_path = PathBuf::from(matches.value_of("TEST").unwrap());
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
