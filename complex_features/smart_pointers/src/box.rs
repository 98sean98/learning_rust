// `Box` stores data on the heap

// 1. when you have a type whose size can't be known at compile time
// and you want to use a value of that type in a context that requires and exact size
// 2. when you have a large amount of data and you want to transfer ownership
// but ensure that data won't be moved when you do so
// 3. when you want to own a value and you care only that it's a type that
// implements a particular trait rather than being of a specific type

// Boxes only provide the indirection and heap allocation
// without any other special capabilities
// they also don't have any performance overhead

fn main() {
    let b = Box::new(5);
    // allocate an i32 value on the heap
    // and put the pointer on the stack as `b`

    println!("b = {}", b);
    // access the data in the box similar to how it would be done if the data is on the stack

    // when `b` goes out of scope at the end of `main`
    // the data on the heap is de-allocated as well

    main2();

}


// recursive data type
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil
}

use List::*;

fn main2() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    println!("{:?}", list);

    let list = list.insert_head(10);

    println!("{:?}", list);

    println!("{:?}", list.head());
    println!("{:?}", list.tail());

    let list = list.insert_tail(4);

    println!("{:?}", list);

    let list = list.insert_tail(5).insert_tail(6).insert_tail(7);

    println!("{:?}", list);
}

impl List {
    fn insert_head(self, head: i32) -> Self {
       Cons(head, Box::new(self))
    }

    fn insert_tail(self, tail: i32) -> Self {
        match self {
            Self::Cons(v, next) => Self::Cons(v, Box::new(next.insert_tail(tail))),
            Self::Nil => Self::Cons(tail, Box::new(Nil))
        }
    }

    fn head(&self) -> i32 {
        match self {
            Self::Cons(v, _) => *v,
            Self::Nil => panic!("No item in List")
        }
    }

    fn tail(&self) -> i32 {
        match self {
            Self::Cons(value, next) => {
                if let Self::Nil = **next { // `**next` to deref the reference to the Box first, then deref the Box to get the value on the heap
                    return *value;
                }
                next.tail()
            },
            Self::Nil => panic!("No item in List"),
        }
    }
}