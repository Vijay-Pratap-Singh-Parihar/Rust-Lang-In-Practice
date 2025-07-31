// Zaroori traits aur types ko standard library se import kar rahe hain.
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll, Waker},
    thread,
    time::Duration,
};

// --- Part 1: Hamara Custom Future ---
// Yeh ek simple future hai jo 3 se 0 tak count down karega.
// Iske paas koi self-referential data nahi hai, isliye yeh `Unpin` hai.
struct Countdown {
    count: i32,
}

// Future trait implement kar rahe hain.
impl Future for Countdown {
    // Hamara future jab khatam hoga, toh ek String slice return karega.
    type Output = &'static str;

    // Yeh `poll` method async runtime ka "engine check" hai.
    // `Pin<&mut Self>` ka matlab hai ki yeh future memory mein move nahi ho sakta.
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.count > 0 {
            println!("Countdown: {}", self.count);
            self.count -= 1;

            // Waker ko clone karke, hum runtime ko kehte hain ki humein dobara poll kare.
            // Real-world mein, I/O event (jaise network data) ke aane par waker ko call kiya jaata hai.
            let waker = cx.waker().clone();
            thread::spawn(move || {
                thread::sleep(Duration::from_millis(200));
                // Hum runtime ko "wake up" call de rahe hain.
                waker.wake();
            });

            // "Kaam abhi baaki hai. Baad mein check karna."
            Poll::Pending
        } else {
            // "Kaam ho gaya! Yeh raha result."
            Poll::Ready("Countdown finished!")
        }
    }
}

// --- Part 2: Hamara Custom Stream ---
// Yeh stream ek-ek karke numbers emit karega.
struct NumberEmitter {
    current: u32,
    max: u32,
}

// Rust mein Stream trait abhi standard nahi hai, isliye hum ek conceptual version use kar rahe hain.
// `futures` crate ka Stream trait bilkul aisa hi dikhta hai.
trait Stream {
    type Item;
    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>>;
}

impl Stream for NumberEmitter {
    type Item = u32;

    // `poll_next` `poll` (Future) aur `next` (Iterator) ka combination hai.
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if self.current <= self.max {
            let val = self.current;
            self.current += 1;

            // Yahan bhi waker ka use karte hain.
            let waker = cx.waker().clone();
            thread::spawn(move || {
                thread::sleep(Duration::from_millis(150));
                waker.wake();
            });

            // "Ek naya item taiyaar hai."
            Poll::Ready(Some(val))
        } else {
            // "Assembly line band ho chuki hai, ab aur items nahi aayenge."
            Poll::Ready(None)
        }
    }
}


// --- Part 3: Sabko Ek Saath Use Karna ---
// Hum ek dummy Waker banayenge taaki hum manually poll kar sakein.
// Real runtime yeh sab automatically karta hai.
fn dummy_waker() -> Waker {
    use std::task::{RawWaker, RawWakerVTable};
    fn clone(_: *const ()) -> RawWaker { RawWaker::new(std::ptr::null(), &VTABLE) }
    fn wake(_: *const ()) {}
    fn wake_by_ref(_: *const ()) {}
    fn drop(_: *const ()) {}
    const VTABLE: RawWakerVTable = RawWakerVTable::new(clone, wake, wake_by_ref, drop);
    unsafe { Waker::from_raw(RawWaker::new(std::ptr::null(), &VTABLE)) }
}


fn main() {
    println!("--- Real-World Scenario Simulation ---");

    // === Scenario A: Running a single Future manually ===
    println!("\n[Scenario A] Manually polling a single countdown future...");
    let mut countdown_fut = Countdown { count: 3 };
    // `Box::pin` future ko memory mein "bolt" kar deta hai.
    let mut pinned_countdown = Box::pin(countdown_fut);
    
    let waker = dummy_waker();
    let mut context = Context::from_waker(&waker);
    
    loop {
        match pinned_countdown.as_mut().poll(&mut context) {
            Poll::Ready(result) => {
                println!("Result: {result}");
                break;
            }
            Poll::Pending => {
                // Real runtime yahan par doosre tasks par kaam karta.
                // Hum yahan simulate karne ke liye bas wait kar rahe hain.
                thread::sleep(Duration::from_millis(250));
            }
        }
    }

    // === Scenario B: Putting different Futures in a Vec ===
    println!("\n[Scenario B] Storing different futures in a Vec requires Pin<Box<dyn Future>>.");
    // Ek aur future, is baar `async` block se. Yeh ek alag, unique type hai.
    let another_fut = async {
        println!("Another async task ran!");
        "Task finished!"
    };

    // Alag-alag types ke futures ko ek Vec mein daalne ke liye hum `Pin<Box<dyn Future>>` use karte hain.
    // Yeh "generic ID badge" waala concept hai.
    let mut futures_vec: Vec<Pin<Box<dyn Future<Output = &str>>>> = vec![
        Box::pin(Countdown { count: 2 }), // Hamara custom `Countdown` future
        Box::pin(another_fut),            // Compiler-generated `async` block future
    ];
    // Ek real runtime (jaise Tokio) is vector par `join_all` chala kar sabko concurrently poora karta.
    println!("Vector created with 2 different future types. A real runtime would now execute them.");


    // === Scenario C: Consuming a Stream manually ===
    println!("\n[Scenario C] Manually polling a stream...");
    let mut number_stream = NumberEmitter { current: 1, max: 4 };
    let mut pinned_stream = Box::pin(number_stream);
    
    loop {
        match pinned_stream.as_mut().poll_next(&mut context) {
            Poll::Ready(Some(num)) => println!("Stream emitted: {num}"),
            Poll::Ready(None) => {
                println!("Stream finished.");
                break;
            }
            Poll::Pending => {
                thread::sleep(Duration::from_millis(200));
            }
        }
    }
}