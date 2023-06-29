use std::{
    sync::{mpsc, Arc, Mutex},
    thread
};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

// a Job is a closure each thread needs to run
// so use a type alias for a trait object
type Job = Box<dyn FnOnce() + Send + 'static>;


impl ThreadPool {
    pub fn new(size: usize) -> Self {
        // make sure that size != 0 cuz it doesn't make sense
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        // the workers should share the same receiver

        let receiver = Arc::new(Mutex::new(receiver));
        // when the thread in a worker receives a Job,
        // it needs to mutate the receiver
        // but multiple workers cannot mutate the receiver freely
        // so use a Mutex to lock the receiver,
        // allowing only 1 worker thread to access and mutate at a time
        // then use Arc to allow workers to share the same receiver

        let mut workers = Vec::with_capacity(size);
        // vector with fixed size

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }


        Self { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        // send a job down the channel to be queued
        let job = Box::new(f);

        self.sender.send(job).unwrap();

    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>
    // expects each thread to run a closure that returns the unit type `()`
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();

            println!("Worker {id} got a job; executing...");

            job();


            // compared to this implementation which gives the wrong behaviour
            // mutex lock is held for longer than intended

            //  while let Ok(job) = receiver.lock().unwrap().recv().unwrap() {
            //      job();
            //  }

            // the ownership of the lock is based on the lifetime of the `MutexGuard<T>`
            // within the `LockResult<MutexGuard<T>>` that the `lock` method returns
            // which means for as long as `MutexGuard<T>` is still in scope
            // the lock is considered to be owned, and not yet unlocked

            // the expression `receiver.lock().unwrap().recv().unwrap();`
            // creates `LockResult<MutexGuard<T>>` as temporary value after `lock()` returns

            // ```let job = receiver.lock().unwrap().recv().unwrap();```
            // this works because at the end of `let` statement,
            // any temporary values is dropped out of scope

            // a `while let`, `if let` and `match` expression
            // does not drop the temporary values until
            // the end of the associated block
            // which means the lock is held until the `job()` finishes
            // so other workers cannot acquire the lock, and hence no new jobs
            // can be processed concurrently as one that is already running

        });

        Self { id, thread }
    }
}


// if the OS cannot create a thread due to limited system resources
// `thread::spawn` will panic
// to gracefully handle this, use `std::thread::Builder` and it's `spawn` method that returns a `Result` instead
