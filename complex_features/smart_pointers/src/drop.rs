// `Drop` trait
// run a piece of code when a value goes out of scope


struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping custom smart pointer with data: `{}`!", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer{data: String::from("my stuff")};
    let d = CustomSmartPointer{data: String::from("other stuff")};

    println!("Custom pointers created");

    // rust automatically calls `drop` when the instances go out of scope
    // variables are dropped in the reverse order of their creation


    // dropping a value early using std::mem::drop

    let c = CustomSmartPointer{data: String::from("hello world")};
    println!("Custom smart pointer created");
    drop(c); // method in prelude
    // to force a value to be cleaned up early
    // actually moves `c` into the `drop` method
    println!("Custom smart pointer dropped before end of main");

}