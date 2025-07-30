// Message Passing
use std::{sync::mpsc, thread, time::Duration}; //mpsc => Multi producer, single consumer FIFO queue communication primitives


fn main() {
    let (tx, rx) = mpsc::channel();

    let tx2 = tx.clone();

    thread::spawn(move || {
        // let msg = String::from("Hello");
        // tx.send(msg).unwrap();
        let vals = vec![
            String::from("Hi"),
            String::from("from"),
            String::from("thread")
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        // let msg = String::from("Hello");
        // tx.send(msg).unwrap();
        let vals = vec![
            String::from("Another"),
            String::from("Messages from another end"),
            String::from("Like new thread 2")
        ];

        for val in vals {
            tx2.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // let recieved = rx.recv().unwrap();// recv this is used when we want to block main thread execution recev when there is no dependency of main thread, similarly we can use try_recv which will not block the main thread instead it will return directly when we have main thread working 
    // println!("Got: {}", recieved);
    for recieved in rx {
        println!("Got: {}", recieved);
    }
}
