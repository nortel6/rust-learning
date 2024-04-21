use std::thread;
use std::time::Duration;
use std::sync::{mpsc, Arc, Mutex};

fn main() {
    // Using closure for the threads
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // Wait for the thread
    handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    let v = vec![1, 2, 3];

    // The move here force closure to take ownership of the value
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    // We can drop here, so v cannot be a reference when passing to threads
    // drop(v);

    handle.join().unwrap();


    // Channels, message-passing between threads
    // Transmitter and receiver
    // mpsc = multiple producer, single consumer
    let (tx, rx) = mpsc::channel();
    
    // This wouldn't work, because a value is used after sent/moved
    // It could have been droppped
    // thread::spawn(move || {
    //     let val = String::from("hi");
    //     tx.send(val).unwrap();
    //     println!("val is {}", val);
    // });

    // let received = rx.recv().unwrap();
    // println!("Got: {}", received);

    // Both unwrap is important
    // If receiver has been drop, transmitter couldn't work
    // Vice versa
    // If transmitter dropped, receiver can know it from the result
    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    // recv() takes the value, try_recv() check if a message exists
    let received = rx.recv().unwrap();
    println!("Got: {}", received);

    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // Wait for the messages to be sent
    // tx will be dropped after all the messages sent
    // thus stopping rx
    for received in rx {
        println!("Got: {}", received);
    }

    let (tx, rx) = mpsc::channel();

    // Second producer
    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }

    

    let m = Mutex::new(5);

    {
        // Returns a MutexGuard smart pointer
        let mut num = m.lock().unwrap();
        *num = 6;
        // When out of scoped, dropped and unlocked automatically
    }

    println!("m = {:?}", m);

    // Arc is atomic Rc
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            // Wait until lock available
            // Since multiple variables need to own the lock,
            // We need a reference counting here
            let mut num = counter.lock().unwrap();

            // Mutex provides interior mutability
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());

    // Additional Notes
    // You can implement your own concurrency library
    // Mainly, implement Sync and Send trait
    // Send means value can be transferred between threads
    // Sync means value can be referenced between threads
}