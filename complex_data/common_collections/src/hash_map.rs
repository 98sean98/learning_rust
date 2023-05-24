use std::collections::HashMap;
// unlike String, and Vector,
// HashMap is not already included in the Prelude
// so need to brought into scope


fn main() {
    let mut scores = HashMap::new(); // stored in heap memory

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // HashMap is homogenous:
    // all of the keys must be the same type
    // all of the values must be the same type


    let team = String::from("Blue");
    let score = scores
        .get(&team) // returns an Option<&V>, if there's no value for the key, get() returns None
        .copied() // get an Option<i32> instead of Option<&i32>, i32 is Copy-able
        .unwrap_or(0); // unwrap set `score` to 0 if scores doesn't have an entry for the key
    println!("score is {score}");

    for score in &scores { // arbitrary order of `score`s
        let (k, v) = score; // score is a tuple
        println!("{k}: {v}");
    }


    // ownership
    let field_name = String::from("Favourite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value); // both `field_name` and `field_value` ownership taken by the hashmap

    // however, if we insert references to values into the hashmap, the values themselves won't be moved into the hashmap
    // the values that the references point to must remain valid for as long as the hashmap is valid


    // overwriting a value
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 20); // value of the entry with "Blue" key is overwritten
    println!("{:?}", scores);


    // adding an entry if the key isn't present
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores
        .entry(String::from("Blue")) // returns an Entry that represents a value that might or might not exist
        .or_insert(50); // return a mutable reference to the value for the corresponding Entry key if exists, otherwise a mutable reference to the new value used to create a new Entry in the HashMap
    scores.entry(String::from("Yellow")).or_insert(20);
    println!("{:?}", scores);


    // updating a value based on the old value
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0); // if the word doesn't already exist in the map, add an entry with the value 0, return a mutable reference to the value
        // println!("{:?}", map); // compile error! immutable borrow occurs here when there is a mutable borrow in the next line of code
        *count += 1; // +1 to the referenced value
        println!("{:?}", map); // no compile error! immutable borrow occurs after the mutable borrow stopped being used
    }
    println!("{:?}", map);


    // HashMap, by default, uses SipHash as its hashing function
    // this provides resistance against DoS attacks
    // can replace the hashing function with other implementations that have the BuildHasher trait



}