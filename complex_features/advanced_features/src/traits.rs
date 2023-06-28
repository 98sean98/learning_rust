// specifying placeholder types in trait definitions with associated types

trait Iterator {
    type Item; // associated type

    fn next(&mut self) -> Option<Self::Item>; // `Item` used in method definition
}

// implementors of `Iterator` trait will specify a concrete type for `Item`

struct Counter {
    count: u32,
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
// only allows 1 implementation of `Iterator` for `Counter`

// as opposed to using generics like this

trait Iterator2<T> {
    fn next(&mut self) -> Option<T>;
}

impl Iterator2<String> for Counter {
    fn next(&mut self) -> Option<String> {
        todo!()
    }
}

// allows multiple implementation of `Iterator2` for the same `Counter` struct
impl Iterator2<u32> for Counter {
    fn next(&mut self) -> Option<u32> {
        todo!()
    }
}
// will have to annotate types explicitly to be able to identify which implementation should be used


// default generic type parameters and operator overloading
use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32, y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

// definition of `Add` trait in std::op
trait Add2<Rhs=Self> { // using `=` to define `Self` as default type parameter of generic `Rhs`
    type Output;

    fn add(self, rhs: Rhs) -> Self::Output;
}

// applying to 2 different structs
struct Millimeters(u32); // "newtype" pattern
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + other.0 * 1000)
    }
}

// default type parameters is useful for
// 1. to extend a type without breaking existing code
// possible to add type parameter to existing trait by giving a default so that existing code still works
// 2. to allow customization in specific cases most user's wont need
// don't have to specify type parameter most of the type


// fully qualified syntax for disambiguation
// calling methods with the same name

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("this is your captain speaking");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("waving arms furiously");
    }
}

// what about traits with associative functions (without `self`)
trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}


// supertraits to require one trait's functionality within another trait
// write a trait that can use the associated items of another trait
// the other trait is called a supertrait of the first trait

use std::fmt;

trait OutlinePrint: fmt::Display {
    // require the Display trait

    fn outline_print(&self) {
        let output = self.to_string(); // can call .to_string() on a self that already implements Display
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

struct Point2 {
    x: i32, y: i32,
}

impl OutlinePrint for Point2 {}

impl fmt::Display for Point2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}


// use newtype pattern to implement external traits on external types
// orphan rule: we're only allowed to implement a trait on a type
// if either the trait or the type are local to our crate
// which means that external traits cannot be implemented for external types
// ordinarily, that is

// with newtype pattern, it's possible
// by creating a new type in a tuple struct
// the tuple struct is a thin wrapper around the external type we want to implement an external trait for
// the wrapper type is still local to our crate
// and we can implement the trait on the wrapper

// no runtime performance penalty
// wrapper is elided during compile time

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "🚀 [{}]", self.0.join(", "))
        // use self.0 to access the inner Vec<T>
    }
}

// downside: wrapper is a new type, so it doesn't have the methods of the value it's holding
// would have to implement all of the methods of Vec<T> directly on wrapper
// such that methods delegate to self.0
// can implement Deref trait on the wrapper to return the inner type


fn main() {
    let p1 = Point {x: 3, y: 4};
    let p2 = Point {x: 5, y: 6};
    assert_eq!(p1 + p2, Point {x: 8, y: 10});


    let person = Human;
    // Pilot::fly() is the fully qualified syntax
    Pilot::fly(&person); // to call method in `Pilot`
    Wizard::fly(&person); // to call method in `Wizard`
    person.fly(); // to call method impl for `Human`

    // if there are 2 types implementing 1 trait
    // rust can figure out which method to call based on the type of `self`

    // what about traits with associative functions (without `self`)
    // need to disambiguate!
    assert_eq!(Dog::baby_name(), String::from("Spot")); // use implementation defined on Dog directly
    // Animal::baby_name() // compile error!
    // baby_name() doesn't have a self parameter, and there could be other types implementing Animal,
    // so rust doesn't know which one to call
    assert_eq!(<Dog as Animal>::baby_name(), String::from("puppy")); // use implementation of Animal for Dog
    // treat Dog as an Animal

    // in general, fully qualified syntax is as follows
    // <Type as Trait>::function(receiver_if_method, next_arg, ...);


    let point = Point2 {x: 8, y: -10};
    point.outline_print();


    let wrapped_v = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("wrapped_v: {}", wrapped_v);
}