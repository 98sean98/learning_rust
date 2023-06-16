// Rc<T> which stands for reference counting
// for multiple ownership of a value
// keeps track of number of references to a value
// to determine whether or not the value is still in use
// if there are 0 references to a value, it can be cleaned up

// use case
// allocate some data on the heap for different parts of program to use
// can't determine, during compile time, which part will finish using the data
// if we knew which part will use it last, we can just simply make that as the owner of the data

// Rc<T> is only for single-threaded scenarios
// later chapters will cover how to do reference counting for multi-threaded programs


#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil
}

use self::List::{Cons,Nil};
use std::rc::Rc;
use std::borrow::Borrow;

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    // a new `Rc<List>`

    println!("count after creating a = {}", Rc::strong_count(&a));

    let b = Rc::new(Cons(3, Rc::clone(&a)));
    // call `Rc::clone` and pass a reference of a `Rc<List>` which is `a`

    println!("count after creating b = {}", Rc::strong_count(&a));

    let c = Rc::new(Cons(4, Rc::clone(&a)));

    println!("count after creating c = {}", Rc::strong_count(&a));


    // immutable references allows Rc<T> to share data between multiple parts of the program
    // for reading only

    // if there are multiple mutable references to Rc<T>
    // then might violate borrowing rules
    // cuz multiple borrows to the same place can cause data inconsistencies and race conditions
}