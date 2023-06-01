// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }


use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let result = File::open("hello.txt");

    let file = match result {
        Ok(file) => file,
        Err(error) => {
            match error.kind() {
                ErrorKind::NotFound => match File::create("hello.txt") {
                    Ok(file) => file,
                    Err(error) => panic!("Problem creating the file: {:?}", error)
                },
                other_error => panic!("Problem opening the file: {:?}", other_error)
            }
        }
    };

    // without using match expressions, because they are quite primitive
    // use the `unwrap_or_else` method, and some closures || {}
    let another_file = File::open("world.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("world.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            }) // return the file if successfully created
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });


    let file = File::open("hello.txt").unwrap();
    // if the Result is the Err variant, it will call panic! for us


    let file = File::open("hello.txt").expect("oof file not found");
    // if the Result is the Err variant, it will call panic! with this error message

    // most people choose `expect`rather than `ùnwrap`
    // because it gives more context about why the operation is expected to always succeed


    if let Ok(username) = read_username_from_file() {
        println!("username: {}", username);
    } else {
        panic!("argh error!");
    }

}

use std::io::{self, Read};

// propagating recoverable errors
fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e)
    }
}

fn read_username_from_file_shorter() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    // if the Result is Ok, the value inside is returned to the expression
    // otherwise, the Err is returned by the whole function, thereby propagating it

    // the `?` operator calls the `from` function on the error values
    // `from` function implements the `From` trait, which is used to convert values from one type to another
    // `?` converts the error from the type it received to the type defined in the whole function
    // this is useful when a function returns one error type to represent all the ways a function might fail,
    // even if parts might fail for many different reasons

    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)

    // if we want to use our own error like `OurError` that we define
    // we can also define
    // impl From<io::Error> for OurError
    // to construct an instance of `OurError` from an `io::Error`

    // the `?` operator eliminates a lot of boilerplate
    // and makes this function's implementation simpler
}

fn read_username_from_file_even_shorter() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}

fn read_username_from_file_now_use_library() -> Result<String, io::Error> {
    std::fs::read_to_string("hello.txt")
}

// the `?` operator implements an early return for a Result type for the function
// so if the function signature is incompatible, then we can't use `?`
// technically, the `?` operator expects to return a `Result`, `Option` or `FromResidual`

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
    // early return `None` for the `Option` returned by `next()` function call
}
