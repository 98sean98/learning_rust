use std::fmt::Display;
use std::cmp::PartialOrd;
use std::fmt::Debug;

// a trait defines functionality a particular type has and can share with other types
// used to define shared behaviour in an abstract way
// can use trait bounds to specify a generic type can be any type that has certain behaviour

// a `Summary` trait that has the behaviour provided by the `summarize` method
trait Summary {
    fn summarize(&self) -> String;
    // only declaring the method signature, without implementation

    fn summarize_with_default(&self) -> String {
        String::from("(Read more...)")
    }
    // types that simply use this default implementation as their method behavior
    // or implement their own behavior by overriding

    fn summarize_author(&self) -> String;

    fn summarize_awesome(&self) -> String {
        // call other methods (even if their default implementation is absent) in the same trait
        format!("(Read more from {})", self.summarize_author())
    }
}
// each type implementing this trait must provide its own implementation of the `summarize` method


struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }

    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }
}

struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }

    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}


// can implement traits from other crates onto types in the local crate
// can implement traits in local crate onto types from other crates
// but cannot implement traits from other crates onto types from other crates
// this is the "orphan rule"
// ensures that other people cannot break your code, and vice versa
// without the rule, two crates can implement the same trait for the same type
// and rust wouldn't know which one to use



// traits as parameters of functions
fn notify(item: &impl Summary) {
    // item is any type that implements the Summary trait
    // and can call summarize method which is a shared behaviour of types that have the trait
    println!("Breaking news! {}", item.summarize());
}
// equivalent syntax using "trait bounds"
fn notify2<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// however,
fn notify3(item1: &impl Summary, item2: &impl Summary) {
    // item1 and item2 can be different concrete types
    // as long as they both implement the Summary trait
}
// whereas,
fn notify4<T: Summary>(item1: &T, item2: &T) {
    // ensures that item1 and item2 have the same concrete type
    // also both must implement the Summary trait
}

// using multiple traits with the `+` syntax
fn notify5(item: &(impl Summary + Display)) {
    // item must implement both Summary and Display traits
}
// trait bound syntax
fn notify6<T: Summary + Display>(item: T) {
}

// clearer trait bounds with `where` clause
// when there are many generic types with their own trait bounds, instead of writing this
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
    1
}
// write with `where` clause
fn some_function2<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug
{
   1
}
// this function signature is much less cluttered


// return a type that implements a trait
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("98sean98"),
        content: String::from("I love rust!"),
        reply: false,
        retweet: false
    }
}
// however, this function can only return a particular concrete type
// not different types only known in runtime
// like this is not allowed
// fn returns_summarizable(switch: bool) -> impl Summary {
//      if switch {
//          NewsArticle {...}
//      } else {
//          Tweet {...}
//      }
// }
// this is due to restrictions around how `impl Trait` syntax is implemented in the compiler
// will see how to write a function for this kind of behaviour in a later chapter


// use trait bounds to conditionally implement methods
struct Pair<T> { // T is a generic without any restrictions
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self {x, y}
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
    // this function requires x and y to be comparable, and also printable
    // hence T must implement the Display and PartialOrd traits
}


// conditionally implement a trait for any type that implements another trait
// implementations of a trait for any type that satisfy some trait bounds are called
// "blanket implementations"
trait SomeTrait {
    fn abc() {}
}
impl<T: Display> SomeTrait for T {
    // SomeTrait is a trait, and its implementation for T is in this block of code
    // T is a generic that must implement the Display trait (i.e. it satisfies the Display trait bound)

    fn abc() {
        println!("wut?");
    }
}
// in other words, any type that satisfy Display trait bound,
// can now also call methods implemented in the SomeTrait trait


fn main() {
    let tweet = Tweet {
        username: String::from("98sean98"),
        content: String::from("I love rust!"),
        reply: false,
        retweet: false
    };

    println!("1 new tweet: {}", tweet.summarize());

    println!("{}", tweet.summarize_awesome());

}
