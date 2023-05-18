// structs are similar to tuples
// both hold multiple related values
// but values in a struct can be accessed without knowing ordering
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let mut user = User {
        active: true,
        username: String::from("some user"),
        email: String::from("whoami@linux.org"),
        sign_in_count: 40,
    };

    println!("{}", user.email);

    user.email = String::from("newemail@linux.org");

    println!("{}", user.email);

    let user2 = User {
        username: String::from("user2"),
        ..user // struct update syntax
    }; // this is an assignment from `user` to `user2`
    // which means `user` is no longer usable
    // because the String in the `email` field of `user` has been moved into `user2`
    // if `user2` was given new String values for both `username` and `email`, then `user` would still be valid
    // both `active` and `sign_in_count` are types that implement the Copy trait so the behaviour in "Stack-only data copy" would apply

    // println!("{}", user.email); // compile error! user.email has been moved to user2.email
    println!("{}", user.username); // no compile error! user.username was not moved to user2.username which was given a new String
    println!("{}", user.active); // no compile error! user.active (bool) is Copy-able
    // but not good to continue using user as it is no longer valid overall

    println!("{}, {}", user2.username, user2.email);
}

fn build_user(username: String, email: String) -> User {
    User {
        active: true,
        username, // field init shorthand syntax
        email, // because the function parameter names and the struct field names are the same
        sign_in_count: 1
    }
}

// tuple structs
// useful when you want to give the whole tuple a name and make the tuple a different type from other tuples
// when naming each field as in a regular struct would be verbose or redundant
struct Color(i32, i32, i32); // rgb values
struct Point(i32, i32, i32); // xyz values
// both have different semantic meanings
// a function that takes a parameter of Color type cannot take a Point as an argument
// even though both types are made up three i32 values

fn main2() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

// unit-like structs without any fields
// behave similarly to ()
// useful when you need to implement a trait on some type but don't want to have any data to store
struct AlwaysEqual;

fn main3() {
    let subject = AlwaysEqual;
}