// generic function that uses a type parameter T
// takes a slice of T, and returns a T reference
// to enable comparisons, we need to use the `PartialOrd` trait in the std library
// `PartialOrd` is implemented for both i32 and char concrete types in the library
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {

    let mut largest = &list[0];

    for n in list {
        if n > largest {
            largest = n;
        }
    }

    largest
}


fn main() {
    let list = vec![4,2,5,7,3,5,7,4,6,8,1];

    let largest_num = largest(&list);

    println!("Largest number is {}", largest_num);


    let list = vec!['h', 'e', 'l', 'l', 'o', ' ', 'w', 'o', 'r', 'l', 'd'];

    let largest_char = largest(&list);

    println!("Largest char is {}", largest_char);


    let integer_point = Point {x: 1, y: 2};
    let float_point = Point {x: 1.32, y: 5.31};

    // let wont_work = Point {x: 1, y: 2.45}; // x and y are not the same T type


    println!("x: {}", float_point.x());
    println!("y: {}", float_point.y());


    // println!("distance from origin: {}", integer_point.distance_from_origin()); // compile error! no method found for struct `Point<{integer}>`
    println!("distance from origin: {}", float_point.distance_from_origin());


    let p1 = Point2 {x: 1, y: 2.45};
    let p2 = Point2 {x: "Hello", y: 'c'};
    let p3 = p1.mixup(p2);

    println!("p3.x: {}, p3.y: {}", p3.x, p3.y);
}


struct Point<T> {
    x: T,
    y: T
}

impl<T> Point<T> {
    // declare `T` just after `impl` so that we can use T to implement methods on the type `Point<T>`
    // by declaring `T` as a generic type after `impl`, rust can identify the type in the `<>` of `Point` is a generic type rather than a concrete type

    fn x(&self) -> &T {
        &self.x
    }
}

impl<P> Point<P> {
    fn y(&self) -> &P {
        &self.y
    }
}

// implement a method only on Point<f32> instances rather than on any Point<T> instances
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct Point2<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point2<X1, Y1> {
    // `X1` and `Y1` declared after `impl` to go with the struct definition of `Point2<X1, Y1>`
    fn mixup<X2, Y2>(self, other: Point2<X2, Y2>) -> Point2<X1, Y2> {
        // `X2` and `Y2` declared in method definition because they're only relevant in the method

        // take x of self, and y of other to create a new point
        Point2 {x: self.x, y: other.y}
    }

    // note the `self` that is moved into the `mixup` function which takes ownership
    // and the same happens to `other`
    // which means after calling `mixup` which creates a new Point2 instance, the 2 original Point2 instances would go out of scope
}