fn main() {
    let v1 = vec![1,2,3];

    for value in &v1 { // implicitly create an iterator, and consume it
        println!("{value}");
    }


    let v1 = vec![4,5,6];
    let v1_iter = v1.iter(); // explicitly create an iterator

    for value in v1_iter { // consume the iterator
        println!("{value}");
    }


    // .iter() produces an iterator over immutable references
    // .into_iter() takes ownership of the iterable and returns owned values
    // .iter_mut() produces an iterator over mutable references


    // in rust, iterators are lazy
    // iterator adaptors are methods defined in the Iterator trait
    // that don't consume the iterator
    // but they produce different iterators by changing some aspect of the original iterator

    let v1 = vec![7,8,9];

    v1.iter().map(|x| x + 1);
    // iterators are lazy, they do nothing unless consumed

    let v1_mapped = v1.iter().map(|x| x + 1);
    for value in v1_mapped {
        println!("{value}");
    }

    // or use the .collect() method
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect(); // requires type annotation
    // consumes the iterator and collects the resulting values into a collection data type




}


// std library trait for Iterator
pub trait Iterator {
    type Item; // this is an associated type

    fn next(&mut self) -> Option<Self::Item>;
    // method to override for structs that implement this trait
    // return 1 item at a time inside `Some`
    // return `None` when out of items

}


#[cfg(test)]
mod tests {

    #[test]
    fn iterator_demonstration() {
        let v1 = vec![1,2,3];

        let mut v1_iter = v1.iter();
        // need to make v1_iter mutable because
        // calling the .next() method changes the internal state that the iterator uses
        // to keep track of where it is in the sequence
        // each call to .next() consumes 1 item from the iterator
        // using a `for` loop doesn't require the iterator to be mutable
        // because `for` takes ownership of the iterator, and makes it mutable behind the scenes


        assert_eq!(v1_iter.next(), Some(&1)); // .next() returns an immutable reference to the element
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);

    }

    #[test]
    fn iterator_sum() {
        let v1 = vec![1,2,3];

        let v1_iter = v1.iter();

        let total:i32 = v1_iter.sum(); // requires explicit type annotation
        // takes ownership of iterator
        // calls .next() method to sum over all items

        assert_eq!(total, 6);
    }

}


#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes
        .into_iter() // take ownership of `shoes` vector
        .filter( // pass a closure that returns a boolean
            |s| s.size == shoe_size // closure captures immutable reference of `shoe_size`
        )
        .collect() // gathers the values returned by the adapted iterator into a vector
}

#[cfg(test)]
mod tests2 {
    use super::*;

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {size: 10, style: String::from("sneakers")},
            Shoe {size: 12, style: String::from("sandals")},
            Shoe {size: 10, style: String::from("boots")},
        ];

        let in_size = shoes_in_size(shoes, 10);

        assert_eq!(
            in_size,
            vec![
                Shoe {size: 10, style: String::from("sneakers")},
                Shoe {size: 10, style: String::from("boots")},
            ]
        );
    }
}