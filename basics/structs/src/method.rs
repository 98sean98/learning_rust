#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // `&self` is actually a shorthand for `self: &Self` for which Self is an alias for the type that the `impl` block is for
    // borrow the Self instance
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // methods can
    // 1. take ownership of `self`: area(self) or area(self: Self)
    // 2. borrow `self` immutably: area(&self) or area(self: &Self)
    // 3. borrow `self` mutably: area(&mut self) or area(self: &mut Self)

    // can name a method with the same name as a field in the struct
    fn width(&self) -> bool {
        self.width > 0
    }

    // check if self can hold another Rectangle inside without rotation
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl Rectangle {
    // can define associated functions that don't have `self` as their first parameter
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 10,
        height: 20,
    };

    println!("rect1 area: {}", rect1.area());

    if rect1.width() {
        println!("rect1 has valid width");
    }

    let rect2 = Rectangle {
        width: 2,
        height: 5,
    };

    println!("rect1 can hold rect2: {}", rect1.can_hold(&rect2));

    println!("rect2 can hold rect1: {}", rect2.can_hold(&rect1));

    let square = Rectangle::square(32);

    println!("square area: {}", square.area());

}
