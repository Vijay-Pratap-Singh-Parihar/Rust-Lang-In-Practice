use std::time::Duration;
use trpl::{Either, Html};

fn main() {
    trpl::run(async {
        // Task 1: Ek naya, lightweight async task spawn kiya.
        let handle = trpl::spawn_task(async {
            for i in 1..10 {
                println!("hi number {i} from the first task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        });

        // Task 2: Main async block mein chalta hai.
        for i in 1..5 {
            println!("hi number {i} from the second task!");
            trpl::sleep(Duration::from_millis(500)).await;
        }

        // handle.await non-blocking tareeke se wait karta hai.
        handle.await.unwrap();
    });
}

// Using join
// Agar aapko sirf kuch futures ko ek saath chalana hai, toh aapko alag se task spawn karne ki bhi zaroorat nahi hai. Aap trpl::join use kar sakte hain.

// Rust

// fn main() {
//     trpl::run(async {
//         let fut1 = async { /* ... loop 1 ... */ };
//         let fut2 = async { /* ... loop 2 ... */ };

//         // `join` dono futures ko concurrently chalata hai jab tak dono khatam na ho jayein.
//         trpl::join(fut1, fut2).await;
//     });
// }
// Yeh dono async blocks ko Future mein badal deta hai aur runtime unhe concurrently manage karta hai, ek ke baad ek progress karate hue.
