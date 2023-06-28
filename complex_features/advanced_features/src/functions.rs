fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    // `fn` in the argument type for `f` is a function pointer
    f(arg) + f(arg)
}
// however, unlike closures, `fn` is a type not a trait
// if using closures, a generic type parameter is used with one of the `Fn` traits as trait bound

// function pointers implement all of the closure traits (`Fn`, `FnMut`, `FnOnce`)
// so function pointers can always be passed to functions that expect closures
// therefore, best to write functions with a generic type parameter and one of the closure traits
// so that both function pointers and functions can be accepted

// exception: only want to accept `fn` not closures is when interfacing with external code that doesn't have closures
// like C


// closures are represented by traits
// so you can't return a closure directly
// not allowed to use a function pointer as a return type either
// cuz rust doesn't know how much space is required to store the closure

// but can use a trait object (dynamic dispatch)
fn returns_closure() -> Box<dyn FnMut(&i32) -> i32> {
    Box::new(|&x| x + 1)
}


enum Status {
    Value(u32), // can become initializer function
    Stop,
}

fn main() {
    let answer = do_twice(add_one, 4);

    assert_eq!(answer, 10);

    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(|x| x.to_string()).collect(); // using a closure
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(ToString::to_string).collect(); // using a function

    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();


    let c = returns_closure();
    let updated_list_of_numbers: Vec<i32> = list_of_numbers.iter().map(c).collect();
    println!("{:?}", updated_list_of_numbers);
}
