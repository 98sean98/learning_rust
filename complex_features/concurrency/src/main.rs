use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("{i} from the spawned thread");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("{i} from the main thread");
        thread::sleep(Duration::from_millis(1));
    }

    // when main thread completes, all spawned threads are shut down
    // whether or not they have finished running


    handle.join().unwrap();
    // block the main thread until the handle representing the spawned thread finishes


    let v = vec![1,2,3];

    let handle = thread::spawn(move || {
        println!("vector: {:?}", v);
        // `println!` needs only a reference to `v`
        // but there's nothing to reference if the current function finishes before the thread finishes
        // so `move` is required to move the ownership of `v` into this closure
    });

    // `v` is no longer valid in the main function scope

    handle.join().unwrap();

}
