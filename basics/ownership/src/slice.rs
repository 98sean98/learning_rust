fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // word is an immutable reference to s

    s.clear(); // this empties the String, making it equal to ""

    // word still has the value 5 here, but there's no string left
    // to meaningfully associate it with

    println!("the first word is: {}", word);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
           return i;
        }
    }

    s.len()
}

fn first_word2(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i]; // return a slice referring to the first word in s
        }
    }

    &s[..] // return a slice referring to the complete length of s
}

fn main2() {
    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole
    let word = first_word3(&my_string[0..6]);
    let word = first_word3(&my_string[..]);
    // `first_word3` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let word = first_word3(&my_string);

    let my_string_literal = "hello world";

    // `first_word3` works on slices of string literals, whether partial or whole
    let word = first_word3(&my_string_literal[0..6]);
    let word = first_word3(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word3(my_string_literal);
}

// this function works on both String and str slices
fn first_word3(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}