// Mutex - Mutex is a abbreviation of mutual exclusion it means that you have some pieces of data only one thread can access that piece of the data 
//          at any given time to achieve this mutexes use a locking
use std::sync::{Arc, Mutex}; // Arc refers to Atomic reference counting
use std::thread;
use std::time::Duration;

fn main() {
    // let m = Mutex::new(5);

    // {
    //     let mut num = m.lock().unwrap();
    //     *num = 6;
    // }

    // println!("m = {:?}", m);

    let counter = Arc::new(Mutex::new(0));// Rc pointer doesn't allow us to use in thread safely instead of that Arc is similar to rc but it is thread safe has the atomic reference counting smart pointer which is exactly what we want, atomics are like primitive types except that they could
    let mut handles = vec![];
    // RefCell comes with risk of creating circular dependencies and the mutex smart pointer comes with the risk of creating deadlocks 

    for lint in 0..20 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            println!("New Thread {}", lint);
            thread::sleep(Duration::from_secs(4));
            println!("After 4 seconds");
            *num += 1;
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
