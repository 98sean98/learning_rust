#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}

fn value_in_cents(c: Coin) -> u8 {
    // overall match is an expression itself
    let n = match coin {
        Coin::Penny => { // => expects an expression
            println!("lucky penny!");
            1 // return this value
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => { // extract data associated with the Quarter variant
            println!("State quarter from {:?!}", state)
        }
    };

    n
}

fn main() {
    value_in_cents(Coin::Quarter(UsState::Alabama));


    // match expressions are exhaustive: must exhaust every last possibility to match the values of an expression

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (), // catch-all arm without binding to a value, and return the unit value (empty tuple type)
    }

    let dice_roll = 10;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => do_other_things(other), // catch-all arm that binds value to "other" variable
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn do_other_things(x: i32) {}