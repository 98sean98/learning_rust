// newtype pattern is useful for many things
// 1. statically enforce units of a value
// like Millimeter(u32) and Meter(u32) instead of just u32
// 2. abstract away some implementations of the inner type
// expose public api that is different from the api of the private inner type
// 3. hide internal implementation
// like People type to wrap a HashMap<i32, String>
// code using People can only interact with the public API we provide: adding a name
// the code won't know an i32 ID is assigned to each name


// type alias
// another name for an existing type

type Kilometers = i32;
// this is a synonym for i32, not a separate new type


// the never type that never returns
// special `!` known as the empty type, or preferably as the never type
// stands in the place of the return type for a function that never returns

fn bar() -> ! {
    panic!();
}
// the function `bar` returns never
// aka diverging function

// these are functions that return `!`
// `continue`
// `panic!()`
// `println!()`

// `!` can be coerced into any type based on the surrounding context


// dynamically sized types and the `Sized` trait
// rust needs to know how much space to allocate for a value of a particular type
// but dynamically sized types (DSTs or unsized types) allow code to use values whose size
// can only be known during runtime

// `str` is actually a DST
// not `&str` though

// the thing is, all values of a particular type take up the same amount of space each
// so a string that is dynamic in size, must be a DST cuz 2 strings can have different lengths

// DSTs use a pointer to store metadata: memory address of actual data, and the memory size

// `Sized` trait is to determine whether or not a type's size is known at compile time
// rust also implicitly adds `Sized` as a trait bound to every generic function
// meaning
fn generic1<T>(t: T) {}
// is the same as
fn generic2<T: Sized>(t: T) {}

// by default, generic functions only work types that have a known size at compile time
// but can use this special syntax to relax this restriction
fn generic3<T: ?Sized>(t: &T) {}
// `?Sized` means that `T` may or may not be `Sized`
// `?Trait` syntax is only available for the `Sized` trait
// also note that `t: &T`, using a reference because the size of t is unknown at compile time


fn main() {
    let x: i32 = 5;
    let y: Kilometers = 10;
    assert_eq!(x + y, 15);

}