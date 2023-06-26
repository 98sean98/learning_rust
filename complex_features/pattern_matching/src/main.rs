fn main() {
    let favourite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favourite_color {
        println!("Using your favourite color, {color}, as your background");
    } else if is_tuesday {
        println!("Tuesday is a green day");
    } else if let Ok(age) = age {
        // `age` in `Ok(age)` is a shadow that is only valid inside the if block scope
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }
    // if-let syntax doesn't check for exhaustiveness
    // won't alert us to handle missing cases


    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        // while-let runs for as long as the pattern matches
        println!("{}", top);
    }

    
    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        // `(index, value)` to destructure returned result of `.enumerate()`
        println!("{} at index {}", value, index);
    }


    // destructure a tuple, and bind to variables
    let (x, y, z) = (1, 2, 3);
    println!("x = {x}, y = {y}, z = {z}");

    // ignore the rest
    let (x, y, ..) = (1, 2, 3, 4);
    println!("x = {x}, y = {y}");


    let point = (4, -3);
    print_coordinates(&point);

}


// pattern matching with function arguments
fn print_coordinates(&(x, y): &(i32, i32))  {
    println!("Current location: ({}, {})", x, y);
}