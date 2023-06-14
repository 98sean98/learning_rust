#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
        // || self.most_stocked() is a closure

        // .unwrap_or_else is implemented like this
        //  impl<T> Option<T> {
        //      pub fn unwrap_or_else<F>(self, f: F) -> T
        //      where
        //          f: FnOnce() -> T
        //      {
        //          match self {
        //              Some(x) => x,
        //              None => f(),
        //      }
        //  }
        // that means generic type F has a trait bound of FnOnce() -> T
        // which means F must be callable once, takes no arguments, and return a T
        // FnOnce is implemented by all closures,
        // and enables a closure to move a captured value out of the closure body
        // therefore, .unwrap_or_else() can accept the most different kinds of closures
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }

        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue]
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );


    // capturing references and moving ownership in closures
    // a function has 3 ways to take a parameter:
    // 1. immutable reference
    // 2. mutable reference
    // 3. taking ownership
    // closures also have 3 ways to capture values from their environment
    // that map to those 3 ways for functions

    // immutable references only
    let list = vec![1,2,3];
    println!("Before defining closure: {:?}", list);

    let only_borrows = || println!("Inside closure: {:?}", list);
    // closure captures immutable reference to list
    // that's why reading the list is still valid even after defining the closure
    // multiple immutable references to list at the same time

    println!("Before calling closure: {:?}", list);
    only_borrows();
    println!("After calling closure: {:?}", list);


    // mutable references
    let mut list = vec![1,2,3];

    let mut borrows_mutably = || list.push(10);
    // when this closure is defined,
    // it captures a mutable reference to list

    // println!("Before calling closure: {:?}", list); // compile error! immutable borrow is not allowed because mutable borrow occurs in the closure definition
    borrows_mutably(); // mutable borrow ends after the closure call
    println!("After calling closure: {:?}", list); // immutable borrow to list is allowed again


    // moving ownership
    let list = vec![1,2,3];
    println!("Before defining closure: {:?}", list);

    std::thread::spawn(move || println!("From thread: {:?}", list))
        .join().unwrap();
    // the closure body only needs an immutable reference to be able to perform println!
    // but there is no guarantee between whether the main thread or the spawn thread would finish first
    // the list is dropped from the scope belonging to the thread that owns the list
    // so if the main thread owns the list, and finishes first, the list is dropped
    // and the reference used in the thread becomes dangling, which is invalid behaviour
    // so the compiler requires the list to be moved into the closure
    // println!("From main: {:?}", list);


    // 3 `Fn` traits

    // 1. `FnOnce`: closures that can be called once, i.e. all closures
    // but a closure that moves a captured value out of the body
    // only implements `FnOnce` but not another traits
    // and can only be called once

    // 2. `FnMut`: closures that don't move captured values out of the body
    // but might mutate the captured values
    // these closures can be called more than once

    // 3. `Fn`: closures that don't move captured values out of the body,
    // and don't mutate captured values
    // can be called more than once without mutating the environment
    // important for situations where a closure is called multiple times concurrently

    example3();
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn example1() {
    let mut list = vec![
        Rectangle {width: 10, height: 1},
        Rectangle {width: 5, height: 2},
        Rectangle {width: 8, height: 3},
    ];

    list.sort_by_key(|r| r.width); // `r` is a reference to the current item in the `list` slice
    // sort_by_key expects a closure that is bounded by the `FnMut` trait
    // `|r| r.width` is called multiple times
    // it does not capture, mutate, or move anything out of the body
    // so it meets the `FnMut` requirement, in fact it meets the even stricter `Fn` trait bound

    println!("Sorted list: {:?}", list);
}

fn example2() {
    let mut list = vec![
        Rectangle {width: 10, height: 1},
        Rectangle {width: 5, height: 2},
        Rectangle {width: 8, height: 3},
    ];

    let mut sort_operations:Vec<String> = vec![];
    let value = String::from("by key called");

    list.sort_by_key(|r| {
        // sort_operations.push(value);
        // `value` is first captured by the closure
        // it is then pushed into the `sort_operations` vector which is also captured as a mutable reference
        // this transfers ownership of `value` to `sort_operations`
        // effectively this moves the `value` out of the closure body

        // this closure can be called once, but not more than that
        // because `value` would no longer be in the environment
        // to be pushed into `sort_operations` again

        // therefore this closure only implements `FnOnce`

        r.width
    });

}

fn example3() {
    let mut list = vec![
        Rectangle {width: 10, height: 1},
        Rectangle {width: 5, height: 2},
        Rectangle {width: 8, height: 3},
    ];

    let mut num_sort_operations = 0;

    list.sort_by_key(|r| {
        num_sort_operations += 1;
        // captures `num_sort_operations` as a mutable reference
        // performs mutation on `num_sort_operations`
        // does not move anything out of the body
        // so it meets the `FnMut` trait bound

        r.width
    });

    println!("Sorted list: {:?}", list);
    println!("Number of operations: {}", num_sort_operations);
}