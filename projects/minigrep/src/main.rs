use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // main function in a binary should only have these responsibilities
    //   - calling the command line parsing logic with the argument values
    //   - setting up any other configuration
    //   - calling a `run` function in `lib.rs`
    //   - handling the error if `run` returns one

    let args = env::args();
    // .args() returns an instance of std::env::Arg
    // which implements the Iterator trait over `String` items

    let config = Config::new(args).unwrap_or_else(|err| {
        // this is a closure
        // that runs if Result from new() is Err variant

        // print to stderr
        eprintln!("Problem parsing arguments: {err}");

        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }

}
