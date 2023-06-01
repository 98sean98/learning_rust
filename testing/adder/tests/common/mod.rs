// old style of writing a module
// to avoid `cargo test` from trying to treat this file as an integration test file
// thus avoiding a useless test section because this has 0 tests anyways

pub fn setup() {
    // some useful common setup code for integrations tests

    println!("Hi, I'm gonna test you now!");
}