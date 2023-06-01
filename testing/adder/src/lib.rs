pub fn add(left: usize, right: usize) -> usize {
    left + right
}


#[derive(PartialEq, Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }
}


fn greeting(name: &str) -> String {
    format!("Hello! {}", name)
}


pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100 inclusive, got {}", value);
        }
        Guess { value }
    }

    pub fn new_v2(value: i32) -> Guess {
        if value < 1 {
            panic!("Guess value must be greater than or equal to 1, got {}", value);
        } else if value > 100 {
            panic!("Guess value must be less than or equal to 100, got {}", value);
        }
        Guess { value }
    }
}



// these are all unit tests
// and `cargo build` doesn't compile this `tests` module
// since this module is a child module of the library (parent) module,
// it has access to the private functions
// note the lack of `pub` keywords in the library code above
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
        // the values being compared in `left` and `right` must implement the `PartialEq` and `Debug` traits
    }

    #[test]
    #[should_panic]
    fn should_panic() {
        panic!("oh no!");
        // every test runs in a separate thread
        // so if a test function panics, the main thread sees it, and marks it as Failed
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle{width: 10, height: 20};
        let smaller = Rectangle{width: 5, height: 8};

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle{width: 10, height: 20};
        let smaller = Rectangle{width: 5, height: 8};

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn equal_rectangles() {
        let r1 = Rectangle{width: 1, height: 1};
        let r2 = Rectangle{width: 1, height: 1};

        assert_eq!(r1, r2);
    }

    #[test]
    fn greeting_contains_name() {
        let name = "Carol";
        let result = greeting(&name);

        assert!(result.contains(name), "Greeting did not contain name, value was `{}`", result);
        // after the required arguments are passed in the `assert!` macro
        // debug messages can be written to help understand why a test fails
    }

    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(101);
    }
    // but a `should_panic` test can be imprecise
    // a panic can happen not for the reason we want to test for

    #[test]
    #[should_panic(expected = "less than or equal to 100")]
    // expect panic! message to contain a substring of "less than or equal to 100"
    fn greater_than_100_v2() {
        Guess::new_v2(102);
    }

    // using Result<T, E>
    #[test]
    fn it_works_v2() -> Result<(), String> {
        if add(2, 2) == 4 {
            Ok(())
        } else {
            Err(String::from("2 + 2 does not equal 4"))
        }
    }

    #[test]
    fn it_works_v3() -> Result<(), std::io::Error> {
        std::fs::File::open("i_exist.txt")?;
        Ok(())
    }

    #[test]
    fn it_should_error() {
        let result = std::fs::File::open("i_dont_exist.txt");

        assert!(result.is_err());
        // to check if a Result is an Err variant
    }

    #[test]
    #[ignore] // ignore this test during `cargo test` unless specified
    fn expensive_test() {
        assert_eq!("funny things", "funny things");
    }

}
