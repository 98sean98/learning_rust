fn main() {
    print_measurements(123, 'm');
}

fn print_measurements(x: i32, unit: char) {
    println!("Measurement of x is {}{}", x, unit);

    let f = five();
    println!("f: {f}");

    let x = 3;
    let z: i8 = {
        let y: i64 = 9;
        (x + y).try_into().unwrap() // try to wrap into data type of z variable; if this fails, the program panics
    };

    println!("x: {x}, z: {z}");

    let fib_n = fib(20);
    println!("fib_n: {fib_n}");
}

fn five() -> usize {
    5 // this is an expression, which is returned by the function
}

fn fib(n: u8) -> u128 {
    if n < 2 {
        1
    } else {
        fib(n - 1) + fib(n - 2)
    }
}
