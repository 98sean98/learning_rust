fn main() {
    // raw pointers
    // 1. are allowed to ignore the borrowing rules by having both immutable and mutable pointers
    // or multiple mutable pointers to the same location
    // 2. aren't guaranteed to point a valid memory
    // 3. are allowed to be null
    // 4. don't implement any automatic cleanup

    // give up guaranteed safety, in exchange for greater performance
    // or the ability to interface with another language or hardware
    // where rust's guarantees don't apply

    let mut num = 5;

    // immutable raw pointer
    let r1 = &num as *const i32;

    // mutable raw pointer
    let r2 = &mut num as *mut i32;

    // can create raw pointers in safe code
    // but need to be in `unsafe` block to be able to dereference

    // arbitrary memory address
    let address = 0x012345usize;
    let _r = address as *const i32;
    // no real good reason to do so, but it's allowed

    // dereference in unsafe
    unsafe {
        assert_eq!(*r1, 5);
        assert_eq!(*r2, 5);
    }


    // unsafe functions
    // can only be called inside an unsafe block
    unsafe {
        dangerous();
        // asserting to rust that we have read the documentation of the unsafe function
        // and we understand how to use it properly
        // we've also verified that we fulfill the contract of the function
    }


    // safe abstraction over unsafe code
    let mut values = vec![1,2,3,4,5,6];
    let (a, b) = split_at_mut(&mut values, 3);
    assert_eq!(a, vec![1,2,3]);
    assert_eq!(b, vec![4,5,6]);


    // calling external functions
    // through `extern` which creates and use a Foreign Function Interface (ffi)
    // functions declared with `extern` blocks are always unsafe to call
    // because they don't conform to rust's guarantees, and rust can't even check them
    // so programmers have responsibility to check correctness manually

    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }

    // accessing or modifying a mutable static variable
    // static variables in rust are considered to be global variables
    // but accessing static variables can be problematic due to the borrowing rules
    // i.e. 2 threads accessing the same mutable global variable can cause data race

    add_to_count(3);

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }

    // should try to avoid using mutable static variables
    // especially in a multithreaded program
    // should try to use concurrency techniques and thread-safe smart pointers


    // accessing fields of a union
    // a union is similar to a struct
    // but only 1 declared field is used in a particular instance at one time
    // primarily used to interface with C code
    // unsafe because rust cannot guarantee the type of data currently being stored in the union instance
    // find more info here: https://doc.rust-lang.org/reference/items/unions.html


    // tips to use unsafe code
    // if you need any of the superpowers explained here, you should use unsafe rust!
    // but make sure to understand what is going on to ensure that there are no logical bugs in program
    // also, try to minimize the size of unsafe block
    // it'll make it easier to trace memory bugs related to unsafe rust code

}



unsafe fn dangerous() {
    // doing stuff that need to be manually guaranteed
}


// safe abstraction over unsafe code
fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len); // make sure that mid <= len, otherwise panic!

    unsafe {
        (
            std::slice::from_raw_parts_mut(ptr, mid),
            std::slice::from_raw_parts_mut(ptr.add(mid), len - mid)
        )
        // std::slice::from_raw_parts_mut ptr.add() are unsafe functions
        // from_raw_parts() takes a raw pointer that is trusted to be valid
        // .add() takes an offset location that is trusted to be valid too
    }

    // since it has been asserted tht mid <= len
    // we can guarantee that the unsafe code is, in fact, memory safe
    // therefore this is an acceptable and appropriate use of unsafe rust
}


// `extern` to call external code
extern "C" {
    // list names and signatures of external functions from another language

    fn abs(input: i32) -> i32;
}
// the "C" part defines the application binary interface (ABI) the external function uses
// "C" ABI is the most common, and follows the C language's ABI


// this is a global static variable
// variable naming convention is SCREAMING_SNAKE_CASE
static HELLO_WORLD: &str = "Hello world!";
// this is a mutable static variable
static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}


// unsafe trait
// a trait is unsafe when at least 1 of its methods has some invariant that the compiler cannot verify
unsafe trait Foo {
    // method definitions
}

unsafe impl Foo for i32 {
    // method implementations
}