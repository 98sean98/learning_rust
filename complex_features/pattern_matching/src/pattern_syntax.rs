fn main() {
    // matching named variayles
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {y}"), // new `y` in this `match` scope, that matches any value inside `Some`
        _ => println!("Default case: x = {:?}", x),
    }


    // multiple patterns using |
    let x = 1;

    match x {
        1 | 2 => println!("1 or 2"),
        3 => println!("3"),
        _ => println!("anything"),
    }


    // matching ranges of values using ..=
    let x = 5;

    match x {
        1..=5 => println!("1 through 5"), // more convenient than 1 | 2 | 3 | 4 | 5
        _ => println!("something else")
    }

    // ranges are only allowed for numyer and char values
    let x = 'c';

    match x {
        'a'..='k' => println!("early ASCII letter"),
        'l'..='z' => println!("late ASCII letter"),
        _ => println!("something else")
    }


    // destructure structs
    let point = Point {x: 10, y: 20};
    let Point {x: a, y: b} = point;
    println!("a = {a}, b = {b}");
    // or
    let Point { x, y } = point;
    println!("x = {x}, y = {y}");


    // destructure structs inside match
    let point = Point {x: -8, y: 0};

    match point {
        Point {x: 0, y} => println!("On the x-axis, y = {y}"),
        Point {x, y: 0} => println!("On the y-axis, x = {x}"),
        Point {x , y} => println!("On neither axis, x = {x}, y = {y}"),
    }


    // destructure enums
    let message = Message::ChangeColor(0, 65, 244);

    match message {
        Message::Quit => println!("The Quit variant has no data to destructure"),
        Message::Move {x, y} => println!("Move {x} in the x direction, and {y} in the y direction"),
        Message::Write(s) => println!("Text message: {s}"),
        Message::ChangeColor(r, g, b) => println!("Change the color to red {r}, green {g}, blue {b}"),
    }


    // destructure nested enums and structs
    let message2 = Message2::ChangeColor(Color::Hsv(30, 40, 55));

    match message2 {
        Message2::ChangeColor(Color::Hsv(h, s, v)) => println!("Change the color to hue {h}, saturation {s}, value {v}"),
        Message2::ChangeColor(Color::Rgb(r, g, b)) => println!("Change the color to red {r}, green {g}, blue {b}"),
        _ => (),
    }


    // ignoring a value with `_`
    foo(3,4);


    // ignoring parts of a value with `_`
    let mut setting_value = Some(10);
    let new_setting_value = Some(5);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => println!("Can't override setting"),
        _ => { setting_value = new_setting_value }
    }
    println!("setting is {:?}", setting_value);

    let numbers = (2,4,6,8,10);

    match numbers {
        (first, _, third, _, fifth) => println!("Some numbers: {first}, {third}, {fifth}")
    }


    // ignoring an unused variable by starting its name with `_`
    let _x = 5; // make the compiler stop complaining about unused variables, but still binds value to variable
    let y = 10;

    let s = Some(String::from("hello world"));

    if let Some(_s) = s {
        // `_s` takes ownership of the inner value in `Some` of `s`
        println!("found a string!");
    }
    // println!("trying to use s: {s} here causes compile errors");

    let s = Some(String::from("hello world"));
    if let Some(_) = s {
        println!("The _ doesn't bind to the inner value");
    }


    // ignoring remaining parts of a value with `..`
    let point = Point3D {x: 1, y: 2, z: 3};

    let Point3D {x, ..} = point;
    println!("Point3D's x = {x}");

    // `..` can expand to as many values as necessary
    let numbers = (1,3,5,7,9);

    match numbers {
        (first, .., last) => println!("Some numbers: first = {first}, last = {last}")
    }
    // however, the use of `..` must be unambiguous
    // like this `(.., second, ..)` will cause compile errors


    // extra conditions with match guards
    let num = Some(4);

    match num {
        Some(x) if x % 2 == 0 => println!("The number {} is even", x),
        Some(x) => println!("The number {} is odd", x),
        None => ()
    }
    // but need to be more careful with the logic
    // cuz compiler doesn't perform exhaustive checking when match guards are present

    // using match guards
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {n}"),
        _ => println!("Default case: x = {:?}", x)
    }

    // combine | with match guards
    let x = 4;
    let y = false;
    match x {
        4 | 5 | 6 if y => println!("yes"),
        // behaves like (4 | 5 | 6) if y
        // rather than 4 | 5 | (6 if y)
        _ => println!("no")
    }


    // `@` bindings
    // `@` creates a variable that holds a value at the same time as we're testing that value for a pattern match
    let msg = Message3 {id: 6};

    match msg {
        Message3 {id : renamed_variable @ 3..=7} => println!("Found an id in range: {}", renamed_variable),
        // `@` allows us to test a value and saved it to a variable
        Message3 {id : 8..=10} => println!("Found an id in another range"),
        // value in `id` is merely tested, not saved to a variable, so it is not available in the expression
        Message3 { id} => println!("Found some other id: {}", id)
        // shorthand struct syntax used to save the value to the same name as the field
    }

}


struct Point {
    x: i32,
    y: i32,
}


enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}


enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message2 {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(Color), // nested enum
}


fn foo(_: i32, y: i32) {
    // `_` matches any value but does not bind
    println!("foo function only uses y: {y}");
}


struct Point3D {
    x: i32, y: i32, z: i32
}


struct Message3 {
    id: i32
}