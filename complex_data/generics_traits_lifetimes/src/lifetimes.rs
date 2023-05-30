
// lifetimes are another kind of generic
// that ensure references are valid as long as we need them to be

// every reference has a lifetime
// which is the scope for which the reference is valid
// most of the time lifetimes are inferred
// just like how most types are inferred unless we need to annotate them

// we need to annotate the generic relationships using lifetime parameters
// to ensure that actual references used at runtime will be valid
// this means providing a bit more information to the compiler


// borrow checker
// compares scopes to determine whether all borrows are valid during compile time

fn main() {

}

fn f<'a>() {
    let x: &i32;        // a reference
    let y: &'a i32;     // a reference with an explicit lifetime
    let z: &'a mut i32; // a mutable reference with an explicit lifetime
}

// lifetime annotations don't change how long any of the references live
// rather, they describe the relationships of the lifetimes of multiple references to each other
// without affecting the lifetimes


fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
// tells borrow checker that for some lifetime 'a
// the function takes 2 parameters that live at least as long as lifetime 'a
// and the str slice returned by the function also live as long as lifetime 'a
// in other words, the lifetime of the returned reference is the same as the lifetime
// of the smaller lifetimes of the x and y
// this is the relationship we explicitly tell the compiler to use for code analysis

// again, we're not changing the lifetimes of any values passed or returned
// but informing the borrow checker to reject any values that don't adhere to this constraint

// lifetime annotations are part of the contract of the function
// they are in the function signature, not in the function body


// structs that use references in their fields
struct ImportantExcerpt<'a> {
    part: &'a str
}
// an instance of this struct should only be valid as long as its `part` reference remains valid

fn t() {
    let novel = String::from("This is a new world. It is beautiful.");
    let first = novel.split('.').next().expect("No '.' character found"); // `first` is a reference to a str slice in `novel`
    let i = ImportantExcerpt {
        part: first // reference to the same str slice in `novel`
    };
    // so if `novel` goes out of scope, `i` will also go out of scope
}


// lifetime elision rules
// these rules are used by the compiler to check functions that use references in their input or output
// 1. the compiler assigns a lifetime parameter to each parameter that’s a reference.
// In other words, a function with one parameter gets one lifetime parameter:
// fn foo<'a>(x: &'a i32);
// a function with two parameters gets two separate lifetime parameters:
// fn foo<'a, 'b>(x: &'a i32, y: &'b i32);
// and so on.
//
// 2. if there is exactly one input lifetime parameter,
// that lifetime is assigned to all output lifetime parameters:
// fn foo<'a>(x: &'a i32) -> &'a i32.
//
// 3. if there are multiple input lifetime parameters,
// but one of them is &self or &mut self because this is a method,
// the lifetime of self is assigned to all output lifetime parameters.
// This rule makes methods much nicer to read and write because fewer symbols are necessary.


// 'static is a special lifetime annotation
// it denotes that a reference can live for the entire duration of the program
// all string literals have 'static lifetimes
// they are stored directly in the binary
// sometimes, error messages might suggest to use 'static
// but it usually means there's a dangling reference or a mismatch of the available lifetimes
// so always try to fix those bugs first, before considering to use 'static
// 'static should only be used for variables that really need to live for as long as the program is running


// now, let's use everything together
fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T
) -> &str
where
    T: Display
{
    println!("Announcement: {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}