use crate::garden::vegetables::Asparagus;
// `use` keyword to create shortcuts

pub mod garden;
// tell compiler to include code in src/garden.rs

fn main() {
    let a = Asparagus{};

    println!("This is an asparagus: {:?}", a);
}