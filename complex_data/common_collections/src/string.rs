// strings are hard for 3 reasons:
// rust's propensity for exposing possible errors
// strings being a more complicated data structure than most people give credit for
// utf-8

fn main() {
    // rust core language only has 1 string type which is the `str` slice
    // which often appears in its borrowed form `&str`

    // the `String` type is in the standard library
    // that is growable, mutable, owned, UTF-8 encoded string type

    // both `String` type and `str` slice are utf-8 encoded

    // creating a new `String` is the same as a `Vec<T>` which is a generic vector
    // `String` is actually a vector of bytes, with extra guarantees, restrictions and capabilities

    let mut s = String::new(); // create empty `String`

    let data = "this is a string literal"; // data is `&str` slice

    let s = data.to_string(); // s is a `String` type

    // this method also works on a string literal directly
    let s = "this is a string literal".to_string();

    // and obviously
    let s = String::from("this is a string literal");

    // UTF-8 encoding
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");


    // updating a string
    let mut s = String::from("foo");
    s.push_str("bar"); // s is now "foobar"
    // push_str does not take ownership of the &str slice passed as the function argument
    let s2 = "baz";
    s.push_str(s2);
    // so s2 is still valid after the push_str() call
    println!("s2 is {}", s2);

    s.push('s'); // push() takes a single character


    // concatenation
    let mut s1 = String::from("foo");
    let s3 = s1 + &s2; // s1 has been moved here and can no longer be used
    // + operator actually implements an add method with this function signature
    // fn add(self, s: &str) -> String {}
    // First, s2 has an &, meaning that we’re adding a reference of the second string to the first string.
    // This is because of the s parameter in the add function:
    // we can only add a &str to a String; we can’t add two String values together.
    // But wait—the type of &s2 is &String, not &str, as specified in the second parameter to add.
    //
    // The reason we’re able to use &s2 in the call to add is that the compiler can coerce the &String argument into a &str.
    // When we call the add method, Rust uses a deref coercion, which here turns &s2 into &s2[..].
    // Because add does not take ownership of the s parameter,
    // s2 will still be a valid String after this operation.
    //
    // Second, we can see in the signature that add takes ownership of self,
    // because self does not have an &.
    // s1 will be moved into the add call and will no longer be valid after that.
    // So although let s3 = s1 + &s2; looks like it will copy both strings and create a new one,
    // this statement actually takes ownership of s1,
    // appends a copy of the contents of s2,
    // and then returns ownership of the result.
    // In other words, it looks like it’s making a lot of copies but isn’t;
    // the implementation is more efficient than copying.

    // formatting with format! macro
    let s1 = "foo";
    let s2 = "bar";
    let s3 = "baz";
    let s4 = format!("{s1}-{s2}-{s3}"); // similar to println! macro, it doesn't take ownership, only borrows


    // string indexing
    // is not a thing in rust because strings are actually complicated to index because of UTF-8
    // many characters actually take up more than 1 byte of space
    // so a string with 4 characters might take up 8 bytes

    // so then slicing is allowed
    let hello = "Здравствуйте";
    let s = &hello[0..4]; // `&str` slice with the first 4 bytes in the `hello` string literal

    // but if we try to index only 1 byte in `hello`
    // let s = &hello[0..1];
    // it will panic in runtime
    // 'byte index 1 not a char boundary


    // iterate over strings
    // the best way to operate on pieces of strings is to be explicit about
    // whether you want characters or bytes

    for char in "Зд".chars() {
        println!("{char}");
    }

    for byte in "Зд".bytes() {
        println!("{byte}");
    }


    // strings are complicated
    // rust programmers must put more thought into handling utf-8 data early in the development life cycle
    // rust exposes more of the complexity of strings than other languages

    // the standard library has a lot of built-in functionality for `String` and `&str` types
    // to help handle complex situations correctly
    // checkout methods like `contains` for searching in a string and `replace` for substituting parts of a string with another string


}