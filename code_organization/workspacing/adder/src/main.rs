use add_one;
use add_two;

fn main() {
    let arg = 5;

    println!("arg: {arg}");

    let result1 = add_one::add_one(arg);
    let result2 = add_two::add_two(arg);

    println!("result1: {result1}, result2: {result2}");
}
