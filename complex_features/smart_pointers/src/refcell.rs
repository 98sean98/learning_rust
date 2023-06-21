// RefCell<T>
// for single-threaded scenarios
// allows immutable borrows and mutable borrows checked at runtime
// mutable the value inside RefCell<T> even when the RefCell<T> is immutable
// this is the interior mutability pattern
// borrowing rules are still checked, and if violated during runtime, program will panic


pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger.send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger.send("Warning: You've used up over 75% of your quota!");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger{
                sent_messages: RefCell::new(vec![])
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, msg: &str) {
            self.sent_messages
                .borrow_mut()
                // get a mutable reference to the value inside RefCell
                // which is the Vector<String>
                // actually returns a RefMut<T> type that implements Deref trait
                .push(String::from(msg));
                // call push method on mutable reference
        }
    }

    impl MockMessenger {
        fn bad_send(&self, msg: &str) {
            let mut one_borrow = self.sent_messages.borrow_mut();
            let mut two_borrow = self.sent_messages.borrow_mut();

            one_borrow.push(String::from(msg));
            two_borrow.push(String::from(msg));

            // this code can compile
            // but calling this function will cause a panic
            // 2 mutable references exist in the same scope
            // which violates borrowing rules, in runtime
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
        // .borrow() to get immutable reference to the value inside RefCell
        // actually returns a Ref<T> type that implements Deref trait
        limit_tracker.set_value(90);

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 2);
    }

    #[test]
    #[should_panic]
    fn it_panics() {
        let mock_messenger = MockMessenger::new();

        mock_messenger.bad_send("panic!");
    }
}


#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use self::List::*;
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    println!("value = {:?}", value);
    println!("a = {:?}", a);
    println!("b = {:?}", b);
    println!("c = {:?}", c);

    *value.borrow_mut() += 10;
    // .borrow_mut() to create mutable reference
    // * to dereference interior value to apply += operator

    println!("value = {:?}", value);
    println!("a = {:?}", a);
    println!("b = {:?}", b);
    println!("c = {:?}", c);
}