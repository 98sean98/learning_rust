// rust handles 2 kinds of errors
// - recoverable error
// - unrecoverable error

// recoverable errors are errors thrown that allow a program to recover from
// it uses the Result<T, E> structure, and can be handled without exiting the program

// unrecoverable errors are errors thrown by a panic! macro call
// these errors cause a program to exit completely, cleaning up its memory stack, etc.


fn main() {
    // panic macro
    // panic!("crash and burn!");

    let v = vec![1,2,3];

    v[99]; // causes panic!
}
