use std::ops::Deref;

struct MyBox<T>(T); // tuple struct with 1 element of generic type T

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// treat `MyBox` like a reference by implementing the `Deref` trait
impl<T> Deref for MyBox<T> {
    type Target = T; // associated type for the trait to use

    fn deref(&self) -> &Self::Target {
        &self.0
    }

}


fn main() {
    // using immutable reference
    let x = 5;
    let y = &x; // reference to the value in x

    assert_eq!(5, x);
    assert_eq!(5, *y); // `*` dereference operator to get the value referenced by y


    // using Box
    let x = 5;
    let y = Box::new(x); // actually copies the value of x into the Box, that data is stored on the heap, pointed by the Box

    assert_eq!(5, *y); // `*` dereference Box which implements the Deref trait


    // using MyBox
    let x = 5;
    let y = MyBox::new(x); // actually copies the value of x into the MyBox, that data is stored on the stack

    assert_eq!(5, *y); // `*` dereference Box which implements the Deref trait
    assert_eq!(5, *(y.deref())); // the actual operation that runs when calling `*y`
    // `deref` method returns a reference to the value for which `*` can dereference to obtain the actual value
    // the `*` syntax works on a smart pointer that implements Deref which means
    // we can write code that can work for both ordinary references, and smart pointers


    main2();
}


// implicit deref coercions with functions and methods
// "Deref coercion" converts a reference to type A to a reference to type B
// such that a reference to type B is a function / method argument
// this is a pretty useful convenience that can be performed on types that implement the `Deref` trait
// in fact, `String`'s `deref` method returns a `&str` type (reference to a string slice)
// allows programmers to write functions and methods without writing a lot of `&` and `*`

fn main2() {
    let r = MyBox::new(String::from("world"));

    hello(&r);
    // `&r` is `&MyBox<String>`
    // deref is called to convert to `&String`
    // deref is called again to convert to `&str`
    // which is a reference to a string slice as required by the `hello` function argument

    // instead of having to write this
    hello(&(*r)[..]);
    // *r dereference MyBox to String
    // & and [..] take a string slice that is equal to the whole string
    // this matches the signature of the hello function
}

fn hello(s: &str) {
    println!("hello {s}!");
}