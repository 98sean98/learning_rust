struct Human {
    name: String
}

fn main() {
    let mut humans: [Human; 3] = [
        Human{name: String::from("George")},
        Human{name: String::from("Mary")},
        Human{name: String::from("Ivan")},
    ];


    for human in &humans { // `&humans` is a slice of the `humans` array to avoid moving `humans` into the for loop
        println!("Name is: {}", human.name);
    }

    humans[1].name = String::from("Abby");
    println!("New name is: {}", humans[1].name);
}