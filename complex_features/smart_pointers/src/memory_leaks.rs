use self::List::*;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    // ability to modify the List value of the Cons variant is pointing to
    Nil
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
        // this creates a reference cycle
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // these println! statements cause stack overflow
    // println!("a = {:?}", a);
    // println!("a next item = {:?}", a.tail());


    // at the end of main()
    // b is dropped first, causing the reference count of b's `Rc<List>` instance to decrease from 2 to 1
    // then a is dropped, causing the reference count of a's `Rc<List>` instance to decrease from 2 to 1
    // then the memory on the heap is still not cleared because there's a cyclic reference between the 2 lists


    // rust makes creating memory that is not cleaned up difficult but not impossible
    // preventing memory leaks is not one of rust's guarantees
    // Rc<T> inside of RefCell<T> or similar nested combinations of types
    // with interior mutability and reference counting makes memory leaks possible
    // through reference cycles
    // so have to be very careful
    // to catch these kinds of logic bugs through automated tests, code reviews

    main2();
}


// preventing reference cycles
// turn Rc<T> into Weak<T>

// Rc<T> uses strong references to enable multiple owners for a value
// only when strong reference count decreases to 0, the value is dropped
// can also create a weak reference by calling Rc::downgrade
// weak references don't express ownership relationship
// weak count doesn't affect when a value is cleaned up
// any cycle with weak references get broken once the strong reference count of values involved is 0

// because the value that Weak<T> references might be dropped
// to do anything with that value requires calling `upgrade` method on the Weak<T> instance
// to return an `Option<Rc<T>>`
// gets `Some<Rc<T>>` if the value has not dropped yet; and `None` if the value has been dropped
// therefore, there wont' be an invalid pointer

use std::rc::Weak;

#[derive(Debug)]
struct Node {
    value: i32,

    children: RefCell<Vec<Rc<Node>>>,
    // vector of Nodes to be shared with many variables – able to access each Node directly
    // able to modify which nodes are children of this node
    // a parent node owns its children nodes
    // if a parent node is deleted, the children nodes should be deleted as well

    parent: RefCell<Weak<Node>>,
    // a child node is related to its parent node
    // but does not own the parent
    // this relationship calls for Weak references
}
// this is essentially a doubly-linked tree structure

fn main2() {
    let leaf = Rc::new(
        Node{value: 3, children: RefCell::new(vec![]), parent: RefCell::new(Weak::new())}
    );

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    let branch = Rc::new(
        Node{value: 5, children: RefCell::new(vec![Rc::clone(&leaf)]), parent: RefCell::new(Weak::new())}
    );

    // properly attach the branch node as the `parent` of the leaf node
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    // note that `Weak<Node` references are printed as `(Weak)`


    // now what are the strong count and weak count?
    let leaf = Rc::new(
        Node{value: 3, children: RefCell::new(vec![]), parent: RefCell::new(Weak::new())}
    );

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );

    // create an inner scope
    {
        let branch = Rc::new(
            Node{value: 5, children: RefCell::new(vec![Rc::clone(&leaf)]), parent: RefCell::new(Weak::new())}
        );

        // properly attach the branch node as the `parent` of the leaf node
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch)
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf)
        );

        // `branch` is dropped from scope
    }

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );
}