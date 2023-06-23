use std::sync::Mutex;
use std::thread;

fn main() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        // block this thread until the lock is obtained
        // .lock() returns Result<T, E>
        // if another thread that currently holds the lock panics
        // .lock() returns Err variant for which this thread can choose how to handle
        // in this case, it panics also through .unwrap()

        *num += 1;

        // more precisely, .lock() returns Result<MutexGuard, E>
        // .unwrap() unwraps the result to give MutexGuard
        // MutexGuard is a smart pointer that implements the Deref trait
        // so it can be de-referenced to give the inner value through `*` operator

        // MutexGuard implements the Drop trait
        // that releases the lock automatically when it goes out of scope
        // therefore, there's no risk forgetting to release the lock and blocking the mutex from being used by other threads
    }

    println!("m = {:?}", m);


    // sharing mutex with multiple threads
    // using Arc<T> - "Atomically reference counted" type
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 1..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final counter: {}", *counter.lock().unwrap());

    // why Arc<T> and Rc<T> exist
    // Arc<T> comes with performance penalties to be able to implement thread safety
    // Rc<T> does not guarantee that the reference counter is incremented / decremented
    // when it is not known how the threads get cpu time
    // there might be memory leaks through incorrect counting
}

// Mutex<T> actually provides interior mutability (notice that counter is defined to be immutable)
// together with Arc<T> through Arc<Mutex<T>> definitions
// this is similar to Rc<RefCell<T>>

// however, rust's Mutex does not protect against all kinds of logic errors
// there's still risk of encountering deadlocks
