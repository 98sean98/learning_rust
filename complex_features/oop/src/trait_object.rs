// a trait object points to an instance of a type implementing the trait
// and a table used to lookup trait methods on that type at runtime
// create a trait object by specifying some sort of reference like `&` or `Box<T>`
// and the `dyn` keyword, and the trait

// we can use trait objects in place of generic or concrete type
// when we use a trait object, rust ensures that any value that used in that context implements the trait object's trait, during compile time
// therefore we don't need to know all the possible types at compile time

// in rust, structs and enums are not considered equivalent as objects from other languages
// because the data in structs and enums are separated from methods implemented in `impl`
// but other languages put data and methods together in a single unit – object

// trait objects are somewhat closer to these traditional objects
// since they combine data and behavior
// but we can't add data to a trait object
// so trait objects aren't generally as useful as traditional objects
// but they focus on create abstractions over some common behavior

pub trait Draw {
    fn draw(&self);
}

struct Screen {
    components: Vec<Box<dyn Draw>>
    // specify `Box<T>` smart pointer, `dyn` keyword, `Draw` trait
    // `Box<dyn Draw>` is a trait object, which is a stand-in for any type inside `Box` that implements `Draw` trait
    // it replaces the `T` generic expected by `Vec<T>`
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

// this is different from using generic type parameters and trait bounds
struct Screen2<T: Draw> {
    components: Vec<T>
}

impl<T> Screen2<T>
where T: Draw
{
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

// if using generic type parameters, then only 1 concrete type, i.e. `Button` can be put inside of `Screen2.components` vector
// meaning homogenous collections – `Vec<Button>` or `Vec<TextField>`
// whereas any type that implements `Draw` trait can be put inside `Screen.components` vector
// meaning non-homogenous collections –  `Vec` can contain `Box<Button>` and `Box<TextField>` together


// implementing the trait
#[derive(Debug)]
pub struct Button {
    width: u32,
    height: u32,
    label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("{:?}", self);
    }
}

#[derive(Debug)]
pub struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("{:?}", self);
    }
}


// using the library

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(Button {width: 10, height: 4, label: String::from("Press me!")}),
            Box::new(SelectBox {width: 20, height: 5, options: vec![String::from("abc"), String::from("def")]}),
        ]
    };

    screen.run();
}


// generics and trait bounds allow monomorphization
// compiler generates non-generic implementations of functions and methods for each concrete type
// that is used in-place of a generic type parameter
// results in "static dispatch" – the compiler knows which method to call at compile time

// trait objects requires "dynamic dispatch"
// the compiler can't tell which method to call at compile time
// so it emits code that will figure out which method to call at runtime
// at runtime, rust uses pointers inside the trait object to know which method to call
// this lookup incurs a runtime cost that doesn't occur with "static dispatch"
// "dynamic dispatch" also prevents the compiler from in-lining a method's code, which in turn prevents some optimization
