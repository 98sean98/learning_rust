# Crates and Packages

Crate: smallest amount of code the rust compiler considers at a time

Crate can either be
- binary crate
- library crate

A binary crate is an executable, like a command line or a server.
A binary crate must contain a `main` function.

A library crate contains code that is sharable with other source code that can use the functions and implementations in the crate.
For example, the `rand` library crate contains functions to generate random numbers.
A library crate doesn't contain a `main` function.

A package bundles 1 or more crates that provide a set of functionalities.
It contains a `Cargo.toml` file that describes how to build those crates.
A package can have 0 or more binary crates, but only at most 1 library crate.

The `src/main.rs` file refers to the crate root of the binary crate in a package.
The `src/lib.rs` file refers to the crate root of the library crate.
These file names are standard conventions, and the crates share the same name as the package.
A package can have more binary crates by placing files in the `src/bin` directory.

A crate root is the root source file that the rust compiler starts from and makes up the root module of the crate.
A crate contains many modules, and the modules can be defined in other files that get compiled with the crate.
