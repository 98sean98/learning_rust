// rust language actually comes with very few concurrency features
// most of this stuff is implemented in the std library
// that means the options to perform concurrency are not limited to the language or the std implementation
// you could write your own concurrency features

// to do that, there are 2 concepts embedded in the language:
// `std::marker` traits `Sync` and `Send`


// `Send` trait allows transfer of ownership across threads
// almost all rust types implement `Send` but not `Rc<T>`
// because `Rc<T>` does not guarantee atomic update of reference count

// most primitive types are `Send`
// any type composed entirely of `Send` is automatically marked as `Send` as well


// `Sync` trait allows a type to be referenced from multiple threads
// any type `T` is `Sync` if `&T` (immutable reference) is `Send`
// meaning the reference can be sent safely to another thread

// most primitive types are `Sync`
// any type composed entirely of `Sync` is automatically marked as `Send` as well


// we don't usually need to implement `Send` and `Sync`
// because types made up of `Send` and `Sync` types are `Send` and `Sync`
// in fact, these traits don't have any methods to implement, they're marker traits
// used to enforce invariants related to concurrency

// implementing `Send` and `Sync` requires UNSAFE rust
