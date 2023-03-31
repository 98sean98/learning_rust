const GLOBAL_SCOPE: i32 = 123;

fn main() {
    let mut x = 1;
    println!("The value of x is :{x}");
    x = 2;
    println!("The value of x is :{x}");

    const FUNCTION_SCOPE: i32 = 234;
    println!("The value of global scope: {GLOBAL_SCOPE}");
    println!("The value of function scope: {FUNCTION_SCOPE}");

    let y = 3;
    let y = y + 2; // compute y + 2, then assign to a new variable of the same name
    // this new variable can be a different type
    {
        let y = y * 2;
        println!("The value of y in inner scope is: {y}");
    }
    println!("The value of y in outer scope is: {y}");

}
