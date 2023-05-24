fn main() {
    let v1: Vec<i32> = Vec::new(); // empty Vector stored in heap memory

    let v2 = vec![1,2,3]; // inferred Vector type <i32> using the vec! macro


    // updating a vector
    let mut v3 = Vec::new();
    v3.push(1); // infer that v3 stores i32
    // v3.push(2.32); // compile error! because v3 has been inferred to store i32 instead of f32


    // reading a vector
    let v4 = vec![1,2,3,4,5];

    let third: &i32 = &v4[2];
    println!("third is {}", third);
    // if &v4[i] indexing exceeds the vector,
    // then the program will panic on runtime
    // use [] to deliberately cause a crash when trying to access elements outside the range of the vector

    let fifth: Option<&i32> = v4.get(4); // using the `get` method
    match fifth {
        Some(k) => println!("third is {}", k),
        None => ()
    }


    // referencing
    let mut v5 = vec![5,6,7,8,9];

    let first = &v5[0]; // this is an immutable borrow of v5
    v5.push(10); // this is a mutable borrow
    // println!("first is {}", first); // immutable borrow later used here
    // this 3 lines of code cause compile error because mutable and immutable reference cannot happen at the same time in the same scope


    // iterating over the vector
    let v6 = vec![1,3,5];

    for i in &v6 { // immutable reference to each element
        println!("{i}");
    }

    let mut v7 = vec![4,6,8];

    for i in &mut v7 { // mutable reference to each element
        *i *= 2; // * is a dereference operator
    }

    // compile errors! because for loop holds the reference to the vector
    // preventing simultaneous modification to the whole vector
    // for i in &v7 { // `&v7` is an immutable borrow occurring and later used here
    //     v7.push(10); // mutable borrow occurs here
    // }

    // for i in &mut v7 { // `&mut v7` is a mutable borrow occurring and later used here
    //     v7.push(10); // second mutable borrow occurs here
    // }


    main2();

}


enum SpreadsheetCell {
    Int(i32),
    Float(f32),
    Text(String)
}

fn main2() {
    let row = vec![
        SpreadsheetCell::Int(1),
        SpreadsheetCell::Float(2.3),
        SpreadsheetCell::Text(String::from("hello world"))
    ];
    // using enums ensures that all the possible types of SpreadsheetCell are handled with a match expression

    {
        let v = vec![1,2,3];

        // do stuff with v
    } // v is dropped out of scope, all the contents also become invalid


    let y = some_function();
    println!("y is {}", y);

}

fn some_function() -> SpreadsheetCell {
    let v = vec![1,2,3];
    v[1] // return owned value, element must implement Copy trait to be moved out of vector
}