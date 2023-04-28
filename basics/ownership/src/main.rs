fn main() {
    let t1 = String::from("hi");
    let t2 = t1; // move t1 to t2, t1 becomes invalid

    // println!("t1 = {}", t1); // this does not compile

    let s1 = String::from("hello");
    let s2 = s1.clone(); // make s2 refer to the same heap memory referred by s1

    println!("s1 = {}, s2 = {}", s1, s2);

    // String type does not implement Copy trait
    // so it moves, if clone() is not called

    // stack only data supports Copy
    // like i32, f64, bool,
    // and even Tuple if it only contains types that also has Copy
    // for example (i32, i32) is Copy-able, but (i32, String) is not
    // this kinda data is easy to copy in stack memory, inexpensive operation


    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
    // but i32 is Copy, so it's okay to still
    // use x afterward

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
// special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
// memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.


// ------------------------------------------------------------

fn main2() {
    let s1 = gives_ownership();         // gives_ownership moves its return
    // value into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into
    // takes_and_gives_back, which also
    // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
// happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {             // gives_ownership will move its
    // return value into the function
    // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string                              // some_string is returned and
    // moves out to the calling
    // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
    // scope

    a_string  // a_string is returned and moves out to the calling function
}

fn main3 () {
    let mut s = String::from("hello");

    let mut r1 = &s;
    let mut r2 = &s;

    println!("{}, {}", r1, r2);
    // cannot borrow s as mutable more than once at a time
    // the first mutable borrow is in r1 and must last until it’s used in the println!,
    // but between the creation of that mutable reference and its usage,
    // we tried to create another mutable reference in r2 that borrows the same data as r1
}

fn main4() {
    let mut s = String::from("hello");

    { // new scope
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;
}

fn main5() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &mut s; // BIG PROBLEM

    println!("{}, {}, and {}", r1, r2, r3);

    // cannot have a mutable reference while having another immutable reference to the same value
    // users of an immutable reference don't expect the value to suddenly change from under them
    // but of course, multiple immutable references are fine
    // because no one who is just reading the data has the ability to affect the anyone else's reading of the data
}

fn main6() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);

    // compiler can tell that the reference is no longer being used at a point before the end of the scope
    // this is a pretty ridiculous feature of the rust compiler, IMO
}

fn main7() {
    let mut s = String::from("hello");

    let r1 = &mut s;

    println!("{}", r1);

    let r2 = &s;
    let r3 = &s;
    println!("{}, {}", r2, r3);
}

// the compiler guarantees that references will never be dangling references:
// if you have a reference to some data,
// the compiler will ensure that the data will not go out of scope before the reference to the data does
fn dangle() -> &String { // dangle returns a reference to a String

    let s = String::from("hello"); // s is a new String

    &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
// Danger!
