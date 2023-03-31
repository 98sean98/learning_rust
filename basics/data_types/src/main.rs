fn main() {
    let tup = (1, 2.5, 'c');

    // destructuring tuple
    let (a,b,c) = tup;

    println!("Tuple index 1: {}, index 2: {}", tup.1, tup.2);

    let mut array: [f32; 3] = [1.0, 2.0, 2.3];
    println!("{}", array[0]);

    array[0] = -10.2;
    println!("{}", array[0]);

    const X: usize = 1;

    let mut f: [f32; X + 1] = [0.0; X + 1];

    println!("f at index 0: {}", f[0]);

    f[0] = -12.3;
    println!("f at index 0: {}", f[0]);

    // arrays and tuples are placed on the stack, not the heap
    // trying to access a tuple with out-of-bounds index causes compile error
    // trying to access an array with out-of-bounds index causes runtime errors
}
