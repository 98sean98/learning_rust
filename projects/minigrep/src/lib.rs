use std::fs;
use std::error::Error;
use std::env;


pub struct Config {
    query: String,
    filepath: String,
    ignore_case: bool,
}

impl Config {
    // constructor method for a `Config` instance
    pub fn new(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        // take ownership of `args` which is any type that implements the `Iterator` trait over `String` items

        args.next(); // first argument is the name of the program, so skip over it

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filepath = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a filepath"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();
        // .var returns a Result, so just check if it's an Ok variant
        // Ok variant indicates the IGNORE_CASE variable has been set

        Ok(Config {query, filepath, ignore_case})
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {

    let contents = fs::read_to_string(config.filepath)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())

}

// the return value is a vector of string slices that reference the slices of the argument `contents`
// rather than the argument `query`
// this explicit lifetimes annotation is required
// because rust doesn't know how the lifetimes of the `query`, `contents` and the return value are related
fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();

    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
    // avoids using a mutable `results` vector
    // this makes code cleaner
    // it even makes future implementation of parallel searching possible
    // because there's no need to maintain mutually exclusive access to `results` vector when pushing to it

    // this kinda high level abstraction makes it easy to understand what is happening to the `contents` str
    // because it's not necessary to think about the mutable `results` state
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        // the `\` right after the opening double-quotes tells rust not to put a newline character in the very beginning
        // so `contents` only has 3 lines

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn two_results() {
        let query = "e";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive.", "Pick three."], search(query, contents));
    }

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }

}