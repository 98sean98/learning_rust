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

fn main() {
    // instead of writing this
    let config_max = Some(10);
    match config_max {
        Some(max) => println!("the maximum is configured to be {}", max),
        _ => ()
    }

    // we can write this using the if-let syntax
    let config_max = Some(20);
    if let Some(max) = config_max { // if-let syntax takes a pattern (on the left) and an expression (on the right) separated by an `=` sign
        println!("the maximum is configured to be {}", max);
    }
    // but this conciseness loses the advantage of exhaustive checking of match expressions
    // better to think of if-let syntax as a sugar to match that runs code when the value matches 1 pattern, and do nothing for the other values


    // another use case for if-let
    let c = Coin::Quarter(UsState::Alabama);
    let mut counter = 0;
    match c {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => counter += 1
    }

    // can write like this
    if let Coin::Quarter(state) = c {
        println!("State quarter from {:?}!", state);
    } else {
        counter += 1;
    }

}