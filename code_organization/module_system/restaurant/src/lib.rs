mod front_of_house {
    // inline module definition
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}

// module tree
//
// crate
//  └── front_of_house
//      ├── hosting
//      │   ├── add_to_waitlist
//      │   └── seat_at_table
//      └── serving
//          ├── take_order
//          ├── serve_order
//          └── take_payment


// this function is part of this library's public API
pub fn eat_at_restaurant() {
    // absolute path starting with the name of the crate
    crate::front_of_house::hosting::add_to_waitlist();
    // for code from the current crate, it starts with the literal `crate`


    // relative path
    front_of_house::hosting::add_to_waitlist();

}


fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
        // `super` to reference an item in the parent module
    }

    fn cook_order() {}

    pub struct Breakfast { // this is a public struct
        pub toast: String, // this is a public field
        seasonal_fruit: String // this remains private
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast{
                toast: String::from(toast),
                seasonal_fruit: String::from("Peaches")
            }
        }
    }

    pub enum Appetizer { // all traits are public in a public enum
        Soup,
        Salad
    }
}

pub fn eat_at_restaurant_2() {
    // order a breakfast in the summer with Rye toast
    let mut breakfast = back_of_house::Breakfast::summer("Rye");

    // change our mind about what toast we'd like
    breakfast.toast = String::from("Wheat");

    println!("I'd like {} toast please", breakfast.toast);

    // the next line causes compile error! we are not allowed to read or modify the seasonal fruit
    // breakfast.seasonal_fruit = String::from("Oranges");
    // because seasonal_fruit is a private field


    let order1 = back_of_house::Appetizer::Salad;
    let order2 = back_of_house::Appetizer::Soup;
}

// re-exporting
pub use front_of_house::hosting;
// so that other users of the `restaurant` package can just call `restaurant::hosting::add_to_waitlist()`
// instead of `restaurant::front_of_house::hosting::add_to_wait_list()` because other users of the library
// should not have to care about `front_of_house`


use std::{cmp::Ordering, io};
// is the same as
use std::cmp::Ordering;
use std::io;

use std::io::{self, Write};
// is the same as
use std::io;
use std::Write;


// glob operator
use std::collections::*;
// brings all public items defined in the path to the scope
