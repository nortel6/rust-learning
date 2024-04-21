use std::{error::Error, fmt::{self, Display}, sync::{mpsc, Arc, Mutex}, thread::{self, JoinHandle}};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender : Option<mpsc::Sender<Job>>,
}

// A closure that fits thread
type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        // Multiple thread owns the receiver
        // Mutex to avoid race condition
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender: Some(sender) }
    }

    // pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {
    //     match size {
    //         0 => return Err(PoolCreationError),
    //         _ => Ok(ThreadPool)
    //     }
    // }

    pub fn execute<F>(&self, f: F)
    where
        // Execute closure only once
        // Transfer ownership for closure
        // Thread lives indefinitely
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        // Since the threads run indefinitely, we will need
        // a way to notify the threads it is time to stop running
        // and exit
        // We will use the sender as the signal, if sender is None,
        // then exit the loop
        drop(self.sender.take());

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            // Takes the ownership of the thread behind Worker
            // leaving a None
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

#[derive(Debug)]
pub struct PoolCreationError;

impl Error for PoolCreationError {}

impl Display for PoolCreationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Size cannot be 0")
    }
}

pub struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    pub fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            // This thread runs indefinitely
            // Runs if it gets a job
            // The mutex lock is consumed and unlocked here
            let message = receiver.lock().unwrap().recv();

            match message {
                Ok(job) => {
                    println!("Worker {id} got a job; executing.");

                    job();
                }
                // Receive will returns Err if Sender is closed
                Err(_) => {
                    println!("Worker {id} disconnected; shutting down.");
                    break;
                }
            }
        });

        Worker { id, thread: Some(thread) }
    }
}