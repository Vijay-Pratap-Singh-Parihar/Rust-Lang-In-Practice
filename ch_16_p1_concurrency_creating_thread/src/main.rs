use std::{thread, time::Duration};

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // when we write handle join here it first execute above thread then main thread
    handle.join().unwrap();

    // This code will be executed until a main thread operations are finished
    // like below for loop is part of main thread and above we have created thread::spawn
    // Which creates a new thread and executes until main thread is active like after last iteration
    // New thread will also stop
    for i in 1..5 {
        println!("Hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    // We will be creating handle join for making our new created thread to execute everything after main thread stop executing too
    // here we will be using join variant for continuing the executiion of the thread which is created
    // handle.join().unwrap();

    using_move_closures_with_thread();
}

fn using_move_closures_with_thread() {
    let v = vec![1, 2, 3];

    //main.rs(34, 43): `v` is borrowed here
    // main.rs(33, 18): function requires argument type to outlive `'static`
    // main.rs(33, 32): to force the closure to take ownership of `v` (and any other referenced variables), use the `move` keyword: `move
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}
