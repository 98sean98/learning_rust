#[derive(Debug)] // explicitly opt in for Debug trait
// Debug trait enables us to print our struct in a way that is useful for developers to see its value while debugging
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect = Rectangle {
        width: 10,
        height: 20
    };

    let a = area(&rect); // borrow the struct so that `main` retains ownership
    println!("a: {}", a);

    println!("rect: {:?}", rect); // able to print
    println!("rect: {:#?}", rect); // different style of printing, more useful for larger structs

    dbg!(rect); // macro that takes ownership of an expression (as opposed to println! which takes a reference)
    // prints the file and line number of where this macro call occurs along with the resultant value of the expression and returns ownership of that value

    // println!("rect: {:?}", rect); // compile error! rect was given to dbg! macro call earlier, and not returned to `main` scope

    let rect2 = Rectangle {
        width: dbg!(30 / 2), // debugging the expression 30 / 2 and its resultant value is returned
        height: 3,
    };

    dbg!(&rect2);
}

fn area(r: &Rectangle) -> u64 {
    (r.width * r.height).into()
}

// there are many traits to use with the `derive` attribute that can add useful behaviour to custom types
