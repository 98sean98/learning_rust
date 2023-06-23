use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();
    // mpsc - multiple producer single consumer
    // multiple sending ends, but only a single receiving end
    // tx - transmitter, rx - receiver

    thread::spawn(move || {
        // move the `tx` into the closure
        // `tx` needs to be owned in order to be able to send messages
        let val = String::from("hi");
        tx.send(val).unwrap();
        // .send() returns a Result<T, E>
        // it takes ownership of the value to be sent
        // if the receiver has already been dropped, then an Err variant is returned
        // .unwrap to panic when Err
    });

    let received = rx.recv().unwrap();
    // .recv() to block main thread to receive a message
    // `received` variable takes ownership of transported value
    // if the transmitter is closed, .recv() returns an Err variant

    // .try_recv() does not block the main thread
    // but instantly returns a Result<T, E> immediately
    // useful if the main thread has other things to do while waiting for messages

    println!("Got : {}", received);


    // sending multiple values, blocking the receiver until received
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let values = vec![
            String::from("hello"),
            String::from("from"),
            String::from("this"),
            String::from("beautiful"),
            String::from("world"),
        ];

        for value in values {
            tx.send(value).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        // treating `rx` as an iterator
        // blocks until channel is closed by transmitter
        // once channel is closed, iteration ends
        println!("Got: {}", received);
    }


    // creating multiple producers by cloning
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let values = vec![
            String::from("hi"),
            String::from("from"),
            String::from("this"),
            String::from("thread"),
        ];

        for value in values {
            tx1.send(value).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    thread::spawn(move || {
        let values = vec![
            String::from("hi"),
            String::from("from"),
            String::from("this"),
            String::from("thread"),
        ];

        for value in values {
            tx.send(value).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}
