// Zaroori cheezein import kar rahe hain.
use std::future::Future;
use std::pin::Pin;
use std::time::Duration;

// Yeh main function hai jo async code ko chalaane ke liye runtime set up karta hai.
fn main() {
    trpl::run(async {
        println!("--- 1. The Type Mismatch Problem ---");
        // Har `async` block ek naya, unique, anonymous type banata hai.
        let fut1 = async { println!("Future 1 ran"); };
        let fut2 = async { println!("Future 2 ran"); };

        // YEH CODE COMPILE NAHI HOGA! ❌
        // let futures = vec![fut1, fut2];
        // Compiler error dega: "mismatched types".
        // Kyunki `fut1` aur `fut2` alag-alag types ke hain, bhale hi woh ek jaise dikhte hon.
        // Ek Vec mein sirf same type ke elements ho sakte hain.
        println!("We can't put different Future types in a Vec directly.\n");


        println!("--- 2. Solution: Trait Objects + Pinning ---");
        // Is problem ko solve karne ke liye hum do cheezein use karte hain:
        // 1. Trait Objects (`dyn Future`): Har future ko ek generic "Future" type mein badal deta hai.
        // 2. Pinning (`Pin`): Future ko memory mein move hone se rokta hai.

        // `Box::pin` dono kaam ek saath karta hai:
        // - Future ko heap par daalta hai (`Box`).
        // - Use memory mein "pin" kar deta hai.
        let fut3 = Box::pin(async {
            println!("Future 3 ran");
        });
        let fut4 = Box::pin(async {
            trpl::sleep(Duration::from_millis(10)).await;
            println!("Future 4 ran after a delay");
        });

        // Ab hum in "pinned boxes" ko ek Vec mein daal sakte hain.
        // Kyunki ab sabka type `Pin<Box<dyn Future>>` hai, jo same hai.
        let futures_vec: Vec<Pin<Box<dyn Future<Output = ()>>>> = vec![fut3, fut4];

        // `join_all` is Vec ke saare futures ko concurrently chalata hai.
        println!("Running a Vec of futures concurrently...");
        trpl::join_all(futures_vec).await;
        println!("All futures in the Vec have completed.\n");


        println!("--- 3. The Starvation Problem & Yielding Solution ---");
        println!("Running a 'starving' task that blocks the executor:");

        // Yeh task bina ruke 5 baar print karega, ismein koi `.await` nahi hai.
        let starving_task = async {
            for i in 1..=5 {
                println!("Starving task is hogging the CPU: step {i}");
            }
        };

        let other_task = async {
            println!("==> Other task wants to run!");
        };

        // Hum in dono ko `join!` se concurrently chalate hain.
        // Lekin `starving_task` poora khatam ho jaayega, tabhi `other_task` ko chance milega.
        trpl::join!(starving_task, other_task);
        println!("Starving task finished. Notice how 'other_task' had to wait.\n");


        println!("Now, running a 'cooperative' task that yields:");
        // Yeh task har step ke baad `yield_now` call karta hai.
        let cooperative_task = async {
            for i in 1..=5 {
                println!("Cooperative task is working: step {i}");
                // `yield_now().await` runtime ko kehta hai, "Main thodi der rukta hoon,
                // kisi aur ready task ko chance do."
                trpl::yield_now().await;
            }
        };

        let another_task = async {
            println!("==> Another task wants to run!");
        };

        // Ab jab hum in dono ko chalayenge, toh output interleaved (mix) hoga.
        // Kyunki `cooperative_task` har step ke baad control chhod raha hai.
        trpl::join!(cooperative_task, another_task);
        println!("Cooperative task finished. Notice how both tasks made progress together.");
    });
}