// macros:
// 1. declarative macros with `macro_rules!`
// 2. 3 kinds of procedural macros
//      - custom `#[derive]` macros that specify code added with the `derive` attribute used on structs and enums
//      - attribute-like macros that define custom attributes usable on any item
//      - function-like macros that look like function calls but operate on tokens specified as their argument


// why macros?
// macros are a way to write code that writes more code
// aka metaprogramming
// reduce the amount of code you have to write and maintain

// function signature must define a fixed number of arguments and their types
// unlike macros such as `println!("hello")` and `println!("hi, {}", name)`
// with a variable number of arguments

// macros are expanded during compile time, before the compiler interprets the meaning of code
// which means they can implement a trait for a given type
// a function can't do that, cuz they only run during runtime

// macro definitions are more complex than function definitions
// cuz this is rust code writing more rust code


// declarative macros (`macro_rules!`)
// takes some rust code and apply to something similar to a `match` expression
// find the right pattern of code, and replace with more rust code

#[macro_export] // make the macro available to whenever the crate in which the macro is defined is brought into scope
macro_rules! vec {
    ( $( $x:expr ), *) => {
    // `( )` to contain the whole pattern
    // `$` to declare macro variable that contains rust code, matching the pattern
    // `$x:expr` to match any rust expression and gives the expression the name `$x`

        {
            let mut temp_vec = Vec::new();

            $(
                temp_vec.push($x);
            )*
            // for each `$x:expr` in `$( )`, replace with expression inside `$( )*`

            temp_vec
        }
    }
}

// calling `vec![1,2,3]`
// `$x` pattern matches 3 times with the 3 expressions `1`, `2`, `3`
// so after the replacement, it looks like this
// {
//      let mut temp_vec = Vec::new();
//      temp_vec.push(1);
//      temp_vec.push(2);
//      temp_vec.push(3);
//      temp_vec
// }

// therefore the `vec!` macro can take any number of arguments


// procedural macros accept some code as input, operate on that code, and produce some code as output
// 3 kinds: custom derive, attribute-like, function-like
// definitions must be in their own crate with a special crate type

use proc_macro;

#[some_attribute]
pub fn some_name(input: TokenStream) -> TokenStream {
    // `TokenStream` is defined in `proc_macro` crate
    // represents a sequence of tokens

}


// attribute-like procedural macros
// can be applied to all sorts of items including functions
// not just #derive which only applies to structs and enums

#[route(GET, "/")]
fn index() {}

// signature of macro definition
#[proc_macro_attribute]
pub fn route(attr: TokenStream, item: TokenSteam) -> TokenSteam {}


// function-like macros
// look like function calls
// can take an unknown number of arguments

let sql = sql!(SELECT * FROM posts WHERE id=1);

// can be defined like this
#[proc_macro]
pub fn sql(input: TokenStream) -> TokenSteam {};
