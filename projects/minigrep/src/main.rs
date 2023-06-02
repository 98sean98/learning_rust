use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // main function in a binary should only have these responsibilities
    //   - calling the command line parsing logic with the argument values
    //   - setting up any other configuration
    //   - calling a `run` function in `lib.rs`
    //   - handling the error if `run` returns one

    let args: Vec<String> = env::args().collect();
    // .args() returns an iterator
    // .collect() turns the iterator into a vector containing all the values produced by the iterator
    // .collect creates many kinds of collections, that's why explicit type annotation on `args` is necessary

    let config = Config::new(&args).unwrap_or_else(|err| {
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
