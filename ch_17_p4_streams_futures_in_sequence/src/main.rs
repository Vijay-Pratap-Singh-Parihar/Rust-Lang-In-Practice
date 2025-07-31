use trpl::{Stream, StreamExt, ReceiverStream}; // Extension trait zaroori hai `.next()` ke liye

fn main() {
    simple_iterator();
    iterator_with_adapters();
    // spawn_task_asynchronously();
}

fn simple_iterator() {
    trpl::run(async {
        // Ek simple iterator se stream banate hain
        let mut stream = trpl::stream_from_iter(1..=3);

        // Jab tak stream `Some(value)` de raha hai, loop chalta rahega
        while let Some(value) = stream.next().await {
            println!("Got value: {value}");
        }
    });
}

// use trpl::StreamExt; // `.next()`, `.filter()`, `.map()` jaise tools ke liye
use std::time::Duration;

fn iterator_with_adapters() {
    // adapters --> chef
    // trpl::run ek async runtime shuru karta hai
    trpl::run(async {
        println!("--- Example 1: Basic Stream (The Sushi Belt) ---");

        // Ek simple iterator (1 se 10 tak numbers) se ek stream banate hain.
        // Yeh hamara basic sushi conveyor belt hai.
        let number_stream = trpl::stream_from_iter(1..=10);

        // Ab hum is conveyor belt mein do "specialist chefs" (adapters) add karte hain:
        // 1. Pehla Chef (`.filter`): Sirf even numbers waali plates ko aage jaane dega.
        // 2. Doosra Chef (`.map`): Har plate par jo number hai, usse double kar dega.
        // Yeh sab lazy hai, abhi tak koi kaam nahi hua hai.
        let mut processed_stream = number_stream
            .filter(|num| num % 2 == 0) // Sirf even numbers (2, 4, 6, 8, 10) pass honge
            .map(|num| num * 2);      // Pass hue numbers double ho jaayenge (4, 8, 12, 16, 20)

        // Ab hum conveyor belt ke aakhir mein baith kar plates ka intezaar karte hain.
        // `while let` loop tab tak chalta hai jab tak plates aa rahi hain.
        while let Some(value) = processed_stream.next().await {
            // `.next().await` non-blocking tareeke se agle item ka intezaar karta hai.
            println!("Processed value: {value}");
        }
    });
}

// The core idea: A Stream is an asynchronous version of an Iterator. It's a sequence of values that become available over time, without blocking.
